use crate::error::{config_err, Result};
use crate::resolver::rpc::config::{RpcConfig, RpcEntry};
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

use ethers::prelude::*;
use ethers::providers::Provider;

pub mod config;

pub static RPC_RESOLVER: Lazy<ArcSwap<RpcResolver>> = Lazy::new(|| {
    ArcSwap::from(Arc::new(RpcResolver::load().expect("failed to load RPC resolver")))
});

pub struct RpcResolver {
    config: RpcConfig,
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
        RPC_RESOLVER.store(Arc::new(RpcResolver::new(config)));

        Ok(())
    }
}

pub fn get_provider<C>(chain_name: C, index: usize) -> Result<Provider<Http>>
where
    C: AsRef<str>,
{
    let entry = rpc_entry(chain_name.as_ref(), index)
        .map_err(|e| config_err(e, Some("Could not get RPC entry".into())))?;

    create_provider(entry)
}

fn create_provider(rpc_entry: RpcEntry) -> Result<Provider<Http>> {
    let mut header_map = reqwest::header::HeaderMap::new();

    if let Some(headers) = rpc_entry.headers() {
        header_map = headers
            .try_into()
            .map_err(|e| config_err(e, Some("Not a valid header value".into())))?;
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

pub fn rpc_entry<C>(chain_name: C, index: usize) -> Result<RpcEntry>
where
    C: AsRef<str>,
{
    Ok(RPC_RESOLVER.load().resolve_entry(chain_name.as_ref(), index)?.clone())
}

pub fn rpc_url<C>(chain_name: C, index: usize) -> Result<String>
where
    C: AsRef<str>,
{
    let rpc = rpc_entry(chain_name.as_ref(), index)?;
    Ok(rpc.url().to_string())
}
