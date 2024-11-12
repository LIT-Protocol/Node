#![cfg(all(feature = "proxy_http", feature = "testing"))]

use anyhow::Result;
use chrono::Duration;
use reqwest::Url;
use std::collections::BTreeMap;

use crate::networking::HttpClientProxyConfiguration;

use lit_node::networking::http::proxy::mapping::HttpClientProxyMapping;
use lit_node::tasks::batch_transmissions::INTERNAL_CHATTER_PORT_OFFSET;
use proxy::*;
use rand::Rng;
use std::thread;
use toxiproxy_rust::*;
use tracing::{debug, info, trace};

pub const FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS: u64 = 30;

/// Given a number of nodes and a starting port, generate and save proxy mappings for local testing.
pub fn generate_and_save_proxy_mappings_for_local_testing(
    num_nodes: usize,
    initial_port: usize,
) -> Result<HttpClientProxyMapping> {
    debug!("Generating proxy URLs for local testing");

    let mut proxy_mappings: BTreeMap<Url, BTreeMap<Url, Url>> = BTreeMap::new();

    for i in 0..num_nodes {
        let our_url = Url::parse(&format!("http://127.0.0.1:{}", initial_port + i))?;
        assert!(proxy_mappings
            .insert(our_url.clone(), BTreeMap::new())
            .is_none());

        let grpc_our_url = Url::parse(&format!(
            "http://127.0.0.1:{}",
            initial_port + i + INTERNAL_CHATTER_PORT_OFFSET as usize
        ))?;
        assert!(proxy_mappings
            .insert(grpc_our_url.clone(), BTreeMap::new())
            .is_none());

        for j in 0..num_nodes {
            let proxy_url = HttpClientProxyConfiguration::determine_local_proxy(
                initial_port + i,
                initial_port + j,
            );
            trace!(
                "From: {:?} To: {:?} - Proxy URL: {:?}",
                initial_port + i,
                initial_port + j,
                proxy_url.port().unwrap()
            );

            let dest_url = Url::parse(&format!("http://127.0.0.1:{}", initial_port + j))?;

            assert!(proxy_mappings
                .get_mut(&our_url)
                .unwrap()
                .insert(dest_url, proxy_url)
                .is_none());

            // do the same for grpc
            let proxy_grpc_url = HttpClientProxyConfiguration::determine_local_proxy_grpc(
                initial_port + i,
                initial_port + j,
            );
            trace!(
                "From: {:?} To: {:?} - Proxy URL: {:?}",
                initial_port + i,
                initial_port + j,
                proxy_grpc_url.port().unwrap()
            );

            let dest_grpc_url = Url::parse(&format!(
                "http://127.0.0.1:{}",
                initial_port + j + INTERNAL_CHATTER_PORT_OFFSET as usize
            ))?;

            assert!(proxy_mappings
                .get_mut(&grpc_our_url)
                .unwrap()
                .insert(dest_grpc_url, proxy_grpc_url)
                .is_none());
        }
    }

    let http_client_proxy_mapping = HttpClientProxyMapping::new_with_mappings(&proxy_mappings);

    // Save proxy mappings to file
    assert!(http_client_proxy_mapping.write_file_local().is_ok());

    Ok(http_client_proxy_mapping)
}

/// Given proxy mappings, setup fresh proxy for each mapping with no faults.
///
/// Spawns a thread internally to avoid nested runtimes within the same thread.
/// See:
/// - https://stackoverflow.com/questions/65426683/why-does-tokio-return-the-error-cannot-drop-a-runtime-in-a-context-where-blocki
/// - https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
pub fn setup_proxies(proxy_mappings: &'static HttpClientProxyMapping) {
    thread::spawn(|| {
        // First check if TOXIPROXY is running
        assert!(TOXIPROXY.is_running());

        // Reset all proxies
        let existing_proxies = TOXIPROXY.all().unwrap();
        for proxy in existing_proxies.values() {
            assert!(proxy.delete().is_ok());
        }

        // Iterate through top level of proxy mappings
        for (our_url, dest_urls) in proxy_mappings.proxy_mappings().iter() {
            let mut proxies_to_populate = Vec::new();

            // Iterate through each destination URL
            for (dest_url, proxy_url) in dest_urls.iter() {
                // Determine name of proxy
                let proxy_name = get_proxy_name(our_url, dest_url);

                // Add to list of proxies to populate
                proxies_to_populate.push(ProxyPack::new(
                    proxy_name,
                    sanitize_url_for_proxy_pack(proxy_url),
                    sanitize_url_for_proxy_pack(dest_url),
                ));
            }

            // Get proxy names
            let proxy_names: Vec<String> = proxies_to_populate
                .iter()
                .map(|pp| pp.name.clone())
                .collect();

            // Populate proxies
            assert!(TOXIPROXY.populate(proxies_to_populate).is_ok());

            // Reset each proxy and make sure no faults
            for proxy_name in proxy_names.iter() {
                let proxy_result = TOXIPROXY.find_and_reset_proxy(proxy_name);
                assert!(proxy_result.is_ok());

                let proxy_toxics = proxy_result.as_ref().unwrap().toxics();
                assert!(proxy_toxics.is_ok());
                assert_eq!(0, proxy_toxics.as_ref().unwrap().len());
            }
        }
    })
    .join()
    .expect("Failed to set up proxies");
}

#[derive(Clone, Debug)]
pub enum FaultType {
    /// Injects a latency fault that is never more than the client timeout.
    LatencyBelowClientTimeout,
    /// Injects a latency fault that has enough jitter to simulate semi-faulty nodes.
    LatencyAroundClientTimeout,
    /// Injects a latency fault that is always more than the client timeout.
    LatencyAboveClientTimeout,
    /// Injects a slow TCP closing fault.
    SlowTcpClosing,
    /// Injects a timeout fault that is never more than the client timeout.
    TimeoutBelowClientTimeout,
    /// Injects a timeout fault that is always more than the client timeout.
    TimeoutAboveClientTimeout,
    /// Injects a slicer fault.
    Slicer,
}

/// Injects a fault between two URLs.
pub fn inject_fault_between_urls(fault_type: FaultType, url_1: &Url, url_2: &Url) {
    info!(
        "Injecting {:?} fault from {:?} to {:?}",
        fault_type, url_1, url_2
    );
    inject_fault(&fault_type, url_1, url_2);

    info!(
        "Injecting {:?} fault from {:?} to {:?}",
        fault_type, url_2, url_1
    );
    inject_fault(&fault_type, url_2, url_1);
}

/// Injects a fault between all sources and a random target.
pub fn inject_fault_between_all_sources_to_random_target(
    fault_type: FaultType,
    starting_port_number: usize,
    num_nodes: usize,
) -> usize {
    let random_faulty_node_port =
        get_random_faulty_node_port(starting_port_number, starting_port_number + num_nodes);
    let random_fault_node = get_local_url_from_port(random_faulty_node_port);
    info!(
        "Injecting {:?} fault between all sources to random target: {:?}",
        fault_type, random_fault_node
    );
    for i in 0..num_nodes {
        let source_port = starting_port_number + i;
        if source_port != random_faulty_node_port {
            inject_fault_between_urls(
                fault_type.clone(),
                &get_local_url_from_port(source_port),
                &random_fault_node,
            );
        }
    }

    random_faulty_node_port
}

/// Injects a fault between the source and target on the upstream.
pub fn inject_fault(fault_type: &FaultType, source_url: &Url, target_url: &Url) {
    match fault_type {
        FaultType::LatencyBelowClientTimeout => {
            inject_latency_fault(
                source_url.clone(),
                target_url.clone(),
                // 2s because 10 requests sent serially in node_share_direct must not exceed 30s ecdsa round timeout
                Duration::seconds(i64::try_from(2).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                Duration::seconds(1).num_milliseconds().try_into().unwrap(),
                1.0,
            );

            let (source_url_grpc, target_url_grpc) =
                get_grpc_urls_from_http_urls(source_url.clone(), target_url.clone());
            inject_latency_fault(
                source_url_grpc.clone(),
                target_url_grpc.clone(),
                // 2s because 10 requests sent serially in node_share_direct must not exceed 30s ecdsa round timeout
                Duration::seconds(i64::try_from(2).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                Duration::seconds(1).num_milliseconds().try_into().unwrap(),
                1.0,
            );
        }
        FaultType::LatencyAroundClientTimeout => {
            inject_latency_fault(
                source_url.clone(),
                target_url.clone(),
                // 3s for each of the 10 requests with 3s jitter to simulate semi-faulty behavior
                Duration::seconds(i64::try_from(3).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                Duration::seconds(3).num_milliseconds().try_into().unwrap(),
                1.0,
            );
            let (source_url_grpc, target_url_grpc) =
                get_grpc_urls_from_http_urls(source_url.clone(), target_url.clone());
            inject_latency_fault(
                source_url_grpc.clone(),
                target_url_grpc.clone(),
                // 3s for each of the 10 requests with 3s jitter to simulate semi-faulty behavior
                Duration::seconds(i64::try_from(3).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                Duration::seconds(3).num_milliseconds().try_into().unwrap(),
                1.0,
            );
        }
        FaultType::LatencyAboveClientTimeout => {
            inject_latency_fault(
                source_url.clone(),
                target_url.clone(),
                Duration::seconds(i64::try_from(FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS + 5).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                Duration::seconds(3).num_milliseconds().try_into().unwrap(),
                1.0,
            );
            let (source_url_grpc, target_url_grpc) =
                get_grpc_urls_from_http_urls(source_url.clone(), target_url.clone());
            inject_latency_fault(
                source_url_grpc.clone(),
                target_url_grpc.clone(),
                Duration::seconds(i64::try_from(FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS + 5).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                Duration::seconds(3).num_milliseconds().try_into().unwrap(),
                1.0,
            );
        }
        FaultType::SlowTcpClosing => {
            inject_slow_tcp_close_fault(
                source_url.clone(),
                target_url.clone(),
                Duration::seconds(i64::try_from(FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS + 5).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                1.0,
            );
            let (source_url_grpc, target_url_grpc) =
                get_grpc_urls_from_http_urls(source_url.clone(), target_url.clone());
            inject_slow_tcp_close_fault(
                source_url_grpc.clone(),
                target_url_grpc.clone(),
                Duration::seconds(i64::try_from(FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS + 5).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                1.0,
            );
        }
        FaultType::TimeoutBelowClientTimeout => {
            inject_timeout_fault(
                source_url.clone(),
                target_url.clone(),
                // 2s because 10 requests sent serially in node_share_direct must not exceed 30s ecdsa round timeout
                Duration::seconds(i64::try_from(2).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                1.0,
            );
            let (source_url_grpc, target_url_grpc) =
                get_grpc_urls_from_http_urls(source_url.clone(), target_url.clone());
            inject_timeout_fault(
                source_url_grpc,
                target_url_grpc,
                // 2s because 10 requests sent serially in node_share_direct must not exceed 30s ecdsa round timeout
                Duration::seconds(i64::try_from(2).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                1.0,
            );
        }
        FaultType::TimeoutAboveClientTimeout => {
            inject_timeout_fault(
                source_url.clone(),
                target_url.clone(),
                Duration::seconds(i64::try_from(FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS + 5).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                1.0,
            );
            let (source_url_grpc, target_url_grpc) =
                get_grpc_urls_from_http_urls(source_url.clone(), target_url.clone());
            inject_timeout_fault(
                source_url_grpc.clone(),
                target_url_grpc.clone(),
                Duration::seconds(i64::try_from(FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS + 5).unwrap())
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
                1.0,
            );
        }
        FaultType::Slicer => {
            inject_slicer_fault(source_url.clone(), target_url.clone(), 1024, 128, 250, 1.0);
            let (source_url_grpc, target_url_grpc) =
                get_grpc_urls_from_http_urls(source_url.clone(), target_url.clone());
            inject_slicer_fault(
                source_url_grpc.clone(),
                target_url_grpc.clone(),
                1024,
                128,
                250,
                1.0,
            );
        }
    }
}

pub fn get_grpc_urls_from_http_urls(source_url: Url, target_url: Url) -> (Url, Url) {
    let source_url_grpc = Url::parse(&format!(
        "http://{}:{}",
        source_url.host_str().unwrap(),
        source_url.port().unwrap() + INTERNAL_CHATTER_PORT_OFFSET as u16
    ))
    .unwrap();
    let target_url_grpc = Url::parse(&format!(
        "http://{}:{}",
        target_url.host_str().unwrap(),
        target_url.port().unwrap() + INTERNAL_CHATTER_PORT_OFFSET as u16
    ))
    .unwrap();

    (source_url_grpc, target_url_grpc)
}

/// Inject latency to the target's response to the source.
///
/// Spawns a thread internally to avoid nested runtimes within the same thread.
pub fn inject_latency_fault(
    source_url: Url,
    target_url: Url,
    latency_ms: usize,
    jitter_ms: usize,
    toxicity: f32,
) {
    info!(
        "Injecting latency fault from {:?} to {:?} with latency {}ms, jitter {}ms, toxicity {}",
        source_url, target_url, latency_ms, jitter_ms, toxicity
    );

    thread::spawn(move || {
        // Get name of proxy
        let proxy_name = get_proxy_name(&source_url, &target_url);

        // Retrieve Proxy object
        let get_proxy_result = TOXIPROXY.find_proxy(proxy_name.as_str());
        assert!(get_proxy_result.is_ok());

        get_proxy_result.as_ref().unwrap().with_latency(
            "upstream".into(),
            latency_ms as u32,
            jitter_ms as u32,
            toxicity,
        );
    })
    .join()
    .expect("Failed to set up latency fault");
}

/// Inject timeout to the source's request to the target.
///
/// Spawns a thread internally to avoid nested runtimes within the same thread.
pub fn inject_timeout_fault(source_url: Url, target_url: Url, timeout_ms: usize, toxicity: f32) {
    info!(
        "Injecting timeout fault from {:?} to {:?} with timeout {}ms, toxicity {}",
        source_url, target_url, timeout_ms, toxicity
    );

    thread::spawn(move || {
        // Get name of proxy
        let proxy_name = get_proxy_name(&source_url, &target_url);

        // Retrieve Proxy object
        let get_proxy_result = TOXIPROXY.find_proxy(proxy_name.as_str());
        assert!(get_proxy_result.is_ok());

        get_proxy_result.as_ref().unwrap().with_timeout(
            "upstream".into(),
            timeout_ms as u32,
            toxicity,
        );
    })
    .join()
    .expect("Failed to set up timeout fault");
}

/// Inject slow TCP close to the source's request to the target.
///
/// Spawns a thread internally to avoid nested runtimes within the same thread.
pub fn inject_slow_tcp_close_fault(
    source_url: Url,
    target_url: Url,
    delay_ms: usize,
    toxicity: f32,
) {
    info!(
        "Injecting slow TCP close fault from {:?} to {:?} with delay {}ms, toxicity {}",
        source_url, target_url, delay_ms, toxicity
    );

    thread::spawn(move || {
        // Get name of proxy
        let proxy_name = get_proxy_name(&source_url, &target_url);

        // Retrieve Proxy object
        let get_proxy_result = TOXIPROXY.find_proxy(proxy_name.as_str());
        assert!(get_proxy_result.is_ok());

        get_proxy_result.as_ref().unwrap().with_slow_close(
            "upstream".into(),
            delay_ms as u32,
            toxicity,
        );
    })
    .join()
    .expect("Failed to set up slow TCP close fault");
}

/// Inject slicer to the source's request to the target.
///
/// Spawns a thread internally to avoid nested runtimes within the same thread.
pub fn inject_slicer_fault(
    source_url: Url,
    target_url: Url,
    average_size: usize,
    size_variation: usize,
    delay_ms: usize,
    toxicity: f32,
) {
    info!(
        "Injecting slicer fault from {:?} to {:?} with average size {}, size variation {}, delay {}ms, toxicity {}",
        source_url, target_url, average_size, size_variation, delay_ms, toxicity
    );

    thread::spawn(move || {
        // Get name of proxy
        let proxy_name = get_proxy_name(&source_url, &target_url);

        // Retrieve Proxy object
        let get_proxy_result = TOXIPROXY.find_proxy(proxy_name.as_str());
        assert!(get_proxy_result.is_ok());

        get_proxy_result.as_ref().unwrap().with_slicer(
            "upstream".into(),
            average_size as u32,
            size_variation as u32,
            delay_ms as u32,
            toxicity,
        );
    })
    .join()
    .expect("Failed to set up slicer fault");
}

pub fn get_local_url_from_port(port: usize) -> Url {
    Url::parse(format!("http://127.0.0.1:{}", port).as_str()).unwrap()
}

pub fn get_local_grpc_url_from_port(port: usize) -> Url {
    Url::parse(
        format!(
            "http://127.0.0.1:{}",
            port + INTERNAL_CHATTER_PORT_OFFSET as usize
        )
        .as_str(),
    )
    .unwrap()
}

fn get_proxy_name(source_url: &Url, target_url: &Url) -> String {
    format!(
        "{}-{}",
        source_url.port().unwrap(),
        target_url.port().unwrap()
    )
}

fn sanitize_url_for_proxy_pack(url: &Url) -> String {
    let url = strip_scheme_from_url(url);
    remove_trailing_slash(url)
}

fn remove_trailing_slash(str: String) -> String {
    if str.ends_with('/') {
        str[..str.len() - 1].to_string()
    } else {
        str
    }
}

fn strip_scheme_from_url(url: &Url) -> String {
    url.to_string()
        .replace("http://", "")
        .replace("https://", "")
}

pub fn get_random_faulty_node_port(
    starting_port_number: usize,
    ending_port_number: usize,
) -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(starting_port_number..ending_port_number)
}
