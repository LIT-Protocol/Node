use crate::error::{config_err, unexpected_err, Result};
use crate::resolver::rpc::config::{RpcConfig, RpcEntry};
use arc_swap::ArcSwap;
use futures::stream::FuturesUnordered;
use futures::Future;
use isahc::prelude::*;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::{json, Value};
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tracing::trace;
use url::Url;

use ethers::prelude::*;
use ethers::providers::Provider;
use futures::FutureExt;
use im::HashMap;
use lit_core::error::Unexpected;

pub mod config;

pub static RPC_RESOLVER: Lazy<ArcSwap<RpcResolver>> = Lazy::new(|| {
    ArcSwap::from(Arc::new(RpcResolver::load().expect("failed to load RPC resolver")))
});

static HEALTH_REQUEST_ID: AtomicUsize = AtomicUsize::new(0);

pub static ENDPOINT_MANAGER: Lazy<StandardRpcHealthcheckPoller> =
    Lazy::new(|| StandardRpcHealthcheckPoller::new(&RPC_RESOLVER, &HEALTH_REQUEST_ID));

pub struct StandardRpcHealthcheckPoller<'a> {
    latencies: ArcSwap<im::hashmap::HashMap<RpcEntry, Latency>>,
    rpc_resolver: &'a Lazy<ArcSwap<RpcResolver>>,
    health_request_id: &'a AtomicUsize,
}

impl<'a> StandardRpcHealthcheckPoller<'a> {
    pub fn new(
        rpc_resolver: &'a Lazy<ArcSwap<RpcResolver>>, health_request_id: &'a AtomicUsize,
    ) -> Self {
        StandardRpcHealthcheckPoller {
            rpc_resolver,
            health_request_id,
            latencies: Self::load_latencies_from_rpc_resolver(rpc_resolver),
        }
    }
}

impl<'a> RpcHealthcheckPoller for StandardRpcHealthcheckPoller<'a> {
    fn get_latencies(&self) -> &ArcSwap<HashMap<RpcEntry, Latency>> {
        &self.latencies
    }
    fn get_rpc_resolver(&self) -> &Lazy<ArcSwap<RpcResolver>> {
        self.rpc_resolver
    }
    fn get_health_request_id(&self) -> &AtomicUsize {
        self.health_request_id
    }
    fn healthcheck(
        &self, host: &str, request_body: &Value,
    ) -> impl Future<Output = Result<Duration>> + Send {
        async move {
            let request = isahc::Request::post(host)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(request_body).map_err(|e| unexpected_err(e, None))?)
                .map_err(|e| unexpected_err(e, None))?;

            let start = SystemTime::now();
            let mut response = request.send_async().await.map_err(|e| unexpected_err(e, None))?;
            let finish = SystemTime::now();

            if !response.status().is_success() {
                return Err(unexpected_err(
                    format!("http response from {} was not 200 OK", host),
                    None,
                ));
            }
            #[derive(Deserialize)]
            struct Response {
                result: Option<String>,
            }
            let bytes = response.bytes().await.map_err(|e| unexpected_err(e, None))?;
            let parsed_response =
                serde_json::from_slice::<Response>(&bytes).map_err(|e| unexpected_err(e, None))?;
            parsed_response.result.expect_or_err(String::from(
                "jsonrpc response from eth_blockNumber did not contain a result",
            ))?;
            finish.duration_since(start).map_err(|e| unexpected_err(e, None))
        }
    }
}

pub trait RpcHealthcheckPoller: Sync {
    fn get_latencies(&self) -> &ArcSwap<im::hashmap::HashMap<RpcEntry, Latency>>;

    fn get_rpc_resolver(&self) -> &Lazy<ArcSwap<RpcResolver>>;

    fn get_health_request_id(&self) -> &AtomicUsize;

    fn healthcheck(
        &self, host: &str, req_body: &serde_json::Value,
    ) -> impl Future<Output = Result<Duration>> + Send;

    fn measure_latency_of<F>(
        thing: F,
    ) -> impl Future<Output = (<F as Future>::Output, Result<Duration>)>
    where
        F: Future,
    {
        let a = SystemTime::now();
        thing.map(move |ret| {
            let b = SystemTime::now();
            (ret, b.duration_since(a).map_err(|e| unexpected_err(e, None)))
        })
    }

    fn poll_rpcs_for_latency(&self) -> impl std::future::Future<Output = ()> + Send {
        async move {
            trace!("polling RPCs for latency");
            let rpc_resolver_struct = self.get_rpc_resolver();
            let rpc_resolver = rpc_resolver_struct.load_full();
            let chains = rpc_resolver.config.chains().clone();
            let mut tasks = FuturesUnordered::new();
            for (_chain, entries) in chains {
                // trace!("polling chain {} for latency with {} rpcs", chain, entries.len());
                if entries.len() < 2 {
                    // trace!("skipping chain {} with less than 2 RPCs", chain);
                    continue;
                }
                for rpc_entry in entries {
                    let rpc_entry_clone = rpc_entry.clone();
                    let u = rpc_entry.url().clone();
                    let fut = async move {
                        let id = self.get_health_request_id().fetch_add(1, Ordering::SeqCst);
                        let request_body = json!({
                            "jsonrpc": "2.0",
                            "method": "eth_blockNumber",
                            "params": [],
                            "id": id
                        });
                        let healthcheck = self.healthcheck(u.as_str(), &request_body).await;
                        // let (h, latency) = Self::measure_latency_of(healthcheck).await;
                        let latency = match healthcheck {
                            Ok(latency) => {
                                // trace!("RPC Health check for {} returned as healthy", u);
                                Latency::Healthy(latency)
                            }
                            Err(e) => {
                                trace!("RPC Health check for {} returned as unhealthy {}", u, e);
                                Latency::Unhealthy
                            }
                        };

                        Ok((rpc_entry_clone, latency))
                    };
                    tasks.push(fut);
                }
            }

            let mut results: Vec<Result<(RpcEntry, Latency)>> = vec![];
            while let Some(result) = tasks.next().await {
                results.push(result);
            }

            let mut latencies = self.get_latencies().load_full().deref().clone();
            for result in results {
                let (rpc_entry, latency) = match result {
                    Ok((u, l)) => (u, l),
                    Err(e) => {
                        trace!("Unexpected error polling RPC for latency: {:?}", e);
                        continue;
                    }
                };

                trace!("RPC Health check for {} returned as {:?}", rpc_entry.url(), latency);

                if let Some(existing_latency_entry) = latencies.get_mut(&rpc_entry) {
                    *existing_latency_entry = latency;
                } else {
                    latencies.insert(rpc_entry.clone(), latency);
                }
            }
            self.get_latencies().store(latencies.into());
        }
    }
    fn load_latencies_from_rpc_resolver(
        rpc_resolver: &Lazy<ArcSwap<RpcResolver>>,
    ) -> ArcSwap<HashMap<RpcEntry, Latency>> {
        ArcSwap::from(Arc::new({
            let resolver = rpc_resolver.load();
            let chains = resolver.config.chains();
            let key_values = chains
                .values()
                .map(|v| v.into_iter().rev())
                .flatten()
                .zip((0..).map(|t| Duration::MAX.saturating_sub(Duration::from_secs(t))))
                .map(|(k, v)| (k.clone(), Latency::Healthy(v)));
            let mut m = im::hashmap::HashMap::new();
            m.extend(key_values);
            m
        }))
    }
    fn rpc_entry<C>(&self, chain_name: C) -> Result<RpcEntry>
    where
        C: AsRef<str>,
    {
        let latencies = self.get_latencies().load();
        let resolver = self.get_rpc_resolver().load();
        resolver
            .resolve(chain_name.as_ref())?
            .iter()
            .min_by_key(|entry| latencies.get(entry))
            .ok_or(config_err(
                format!("No RPC entry exists for chain id: {}", chain_name.as_ref()),
                None,
            ))
            .cloned()
    }

    fn get_provider<C>(&self, chain_name: C) -> Result<Provider<Http>>
    where
        C: AsRef<str>,
    {
        let entry = self.rpc_entry(chain_name)?;
        create_provider(&entry)
    }

    fn rpc_url<C>(&self, chain_name: C) -> Result<String>
    where
        C: AsRef<str>,
    {
        let entry = self.rpc_entry(chain_name)?;
        Ok(entry.url().to_string())
    }
}

pub struct RpcResolver {
    config: RpcConfig,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Latency {
    Healthy(Duration),
    Unhealthy,
}

impl RpcResolver {
    fn new(config: RpcConfig) -> Self {
        Self { config }
    }

    fn load() -> Result<Self> {
        let config = RpcConfig::load()?;
        config.verify()?;

        Ok(Self::new(config))
    }

    // Resolve
    pub fn resolve<C>(&self, chain_name: C) -> Result<&Vec<RpcEntry>>
    where
        C: AsRef<str>,
    {
        self.config.chains().get(chain_name.as_ref()).ok_or_else(|| {
            config_err(format!("unable to resolve RPC for chain id: {}", chain_name.as_ref()), None)
        })
    }

    pub fn resolve_entry<C>(&self, chain_name: C, index: usize) -> Result<&RpcEntry>
    where
        C: AsRef<str>,
    {
        let entries = self.resolve(chain_name.as_ref())?;
        match entries.get(index) {
            None => Err(config_err(
                format!("no RPC index in entries for chain id: {}, {}", chain_name.as_ref(), index),
                None,
            )),
            Some(value) => Ok(value),
        }
    }

    // Reload
    pub fn reload() -> Result<()> {
        let config = RpcConfig::load()?;
        config.verify()?;
        let mut latencies = ENDPOINT_MANAGER.get_latencies().load_full().deref().clone();
        let rpc_entries: std::collections::HashSet<&RpcEntry> =
            config.chains().values().flat_map(|v| v.into_iter()).collect();

        latencies.retain(|e, _| rpc_entries.contains(e));
        for (d, rpc_entry) in config.chains().values().flat_map(|v| {
            v.into_iter().enumerate().rev().map(|(i, v)| {
                (Duration::MAX.saturating_sub(Duration::from_secs(164 + i as u64)), v)
            })
        }) {
            if !latencies.contains_key(rpc_entry) {
                latencies.insert(rpc_entry.clone(), Latency::Healthy(d));
            }
        }

        RPC_RESOLVER.store(Arc::new(RpcResolver::new(config)));
        ENDPOINT_MANAGER.get_latencies().store(latencies.into());

        Ok(())
    }
}

fn create_provider(rpc_entry: &RpcEntry) -> Result<Provider<Http>> {
    let mut header_map = reqwest::header::HeaderMap::new();

    if let Some(headers) = rpc_entry.headers() {
        let h: Result<Vec<_>> = headers
            .into_iter()
            .map(|(k, v)| {
                let k = reqwest::header::HeaderName::try_from(k)
                    .map_err(|e| config_err(e, Some("Not a valid header key".into())))?;
                let v = reqwest::header::HeaderValue::try_from(v)
                    .map_err(|e| config_err(e, Some("Not a valid header value".into())))?;
                Ok((k, v))
            })
            .collect();
        header_map = reqwest::header::HeaderMap::from_iter(h?.into_iter());
    }

    if let Some(apikey) = rpc_entry.apikey() {
        // Consider marking security-sensitive headers with `set_sensitive`.
        let mut auth_value = reqwest::header::HeaderValue::from_str(apikey.as_str())
            .map_err(|e| config_err(e, Some("Not a valid api key value".into())))?;
        auth_value.set_sensitive(true);
        header_map.insert(reqwest::header::AUTHORIZATION, auth_value);
    }

    // get a client builder
    let client = reqwest::Client::builder()
        .default_headers(header_map)
        .build()
        .map_err(|e| config_err(e, Some("Could not create provider client".into())))?;

    let url = Url::parse(rpc_entry.url())
        .map_err(|e| config_err(e, Some("Could not get RPC URL".into())))?;

    let mut provider = Provider::new(Http::new_with_client(url, client));
    provider.set_interval(Duration::from_secs(1));

    Ok(provider.clone())
}

// #[cfg(test)]
// mod tests {
//     use crate::resolver::rpc::config::{RpcConfig, RpcEntry};
//     use crate::resolver::rpc::{Latency, RpcHealthcheckPoller, RpcResolver};
//     use anyhow::bail;
//     use arc_swap::ArcSwap;
//     use futures::FutureExt;
//     use futures::StreamExt;
//     use futures::TryStreamExt;
//     use maplit::btreemap;
//     use maplit::btreeset;
//     use once_cell::sync::Lazy;
//     use pretty_assertions::assert_eq;
//     use std::collections::BTreeSet;
//     use std::future::Future;
//     use std::ops::Add;
//     use std::sync::atomic::AtomicUsize;
//     use std::sync::Arc;
//     use std::time::{Duration, Instant};

//     #[tokio::test]
//     async fn test_rpc_healthcheck_poll() -> anyhow::Result<()> {
//         static RPC_RESOLVER: Lazy<ArcSwap<RpcResolver>> = Lazy::new(|| {
//             ArcSwap::from(Arc::new(RpcResolver::new(
//                 toml::from_str(
//                     r#"
//                         [[chains.Foo]]
//                         url = "url.com"
//                         [[chains.Foo]]
//                         url = "url2.com"
//                     "#,
//                 )
//                 .unwrap(),
//             )))
//         });
//         static LATENCIES: Lazy<ArcSwap<im::hashmap::HashMap<RpcEntry, Latency>>> =
//             Lazy::new(|| TestHealthChecker::load_latencies_from_rpc_resolver(&RPC_RESOLVER));
//         static HEALTH_REQUEST_ID: AtomicUsize = AtomicUsize::new(0);

//         mod preordained_for_testing_purposes {
//             use std::sync::atomic::AtomicUsize;
//             use std::time::Duration;
//             pub(crate) static LATENCIES: [Duration; 2] =
//                 [Duration::from_secs(1), Duration::from_secs(2)];
//             pub(crate) static CASE: AtomicUsize = AtomicUsize::new(0);
//         }

//         struct TestHealthChecker;
//         impl RpcHealthcheckPoller for TestHealthChecker {
//             fn get_latencies(&self) -> &ArcSwap<im::hashmap::HashMap<RpcEntry, Latency>> {
//                 &LATENCIES
//             }
//             fn get_rpc_resolver(&self) -> &Lazy<ArcSwap<RpcResolver>> {
//                 &RPC_RESOLVER
//             }
//             fn get_health_request_id(&self) -> &AtomicUsize {
//                 &HEALTH_REQUEST_ID
//             }
//             async fn healthcheck(
//                 &self, host: &str, req_body: &serde_json::Value,
//             ) -> lit_core::error::Result<()> {
//                 Ok(())
//             }
//             fn measure_latency_of<F>(
//                 thing: F,
//             ) -> impl Future<Output = (<F as Future>::Output, lit_core::error::Result<Duration>)>
//             where
//                 F: Future,
//             {
//                 thing.map(|ret| {
//                     (
//                         ret,
//                         Ok(preordained_for_testing_purposes::LATENCIES
//                             [preordained_for_testing_purposes::CASE
//                                 .fetch_add(1, std::sync::atomic::Ordering::SeqCst)]),
//                     )
//                 })
//             }
//         }

//         let latency_to_rpc = btreemap! {
//             Duration::MAX.saturating_sub(Duration::from_secs(0)) => String::from("url2.com"),
//             Duration::MAX.saturating_sub(Duration::from_secs(1)) => String::from("url.com")
//         };

//         let expected_rpc = latency_to_rpc.first_key_value().map(|kv| kv.1);
//         let received_rpc = TestHealthChecker.rpc_entry("Foo")?;
//         let received_rpc = received_rpc.url();

//         assert_eq!(
//             expected_rpc,
//             Some(received_rpc),
//             "before polling, the first RPC should be used by default"
//         );

//         TestHealthChecker.poll_rpcs_for_latency().await;
//         let latencies = TestHealthChecker.get_latencies().load();

//         assert_eq!(
//             btreeset! {
//                 latencies.get("url.com").unwrap(),
//                 latencies.get("url2.com").unwrap()
//             },
//             BTreeSet::from_iter(preordained_for_testing_purposes::LATENCIES.iter())
//         );

//         assert_eq!(
//             btreeset! {
//                 latencies.get("url.com").unwrap().clone(),
//                 latencies.get("url2.com").unwrap().clone()
//             },
//             btreeset! {
//                 String::from("url.com"),
//                 String::from("url2.com")
//             }
//         );

//         let latency_to_rpc = btreemap! {
//             l1 => s1,
//             l2 => s2
//         };

//         let expected_rpc = latency_to_rpc.first_key_value().map(|kv| kv.1);
//         let received_rpc = TestHealthChecker.rpc_entry("Foo")?;
//         let received_rpc = received_rpc.url();

//         assert_eq!(
//             expected_rpc,
//             Some(&received_rpc),
//             "after polling, the minimum latency RPC should be used"
//         );

//         Ok(())
//     }
// }
