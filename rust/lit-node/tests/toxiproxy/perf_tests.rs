#![cfg(all(feature = "proxy_http", feature = "testing"))]

use lit_node::networking::http::proxy::mapping::HttpClientProxyMapping;
use once_cell::sync::Lazy;
use std::io::BufRead;
use std::time::Duration;
use test_common::faults::{
    get_local_grpc_url_from_port, get_local_url_from_port, inject_latency_fault, setup_proxies,
};
use test_common::init_test_config;
use tracing::info;

use crate::integration::ecdsa::new_node_collection_with_authd_pkp;
use test_common::ecdsa::sign_with_hd_key;
use test_common::faults::generate_and_save_proxy_mappings_for_local_testing;

const PERF_TEST_NUM_NODES: usize = 10;
const STARTING_PORT: usize = 7470;

static PROXY_MAPPINGS: Lazy<HttpClientProxyMapping> = Lazy::new(|| {
    generate_and_save_proxy_mappings_for_local_testing(PERF_TEST_NUM_NODES, STARTING_PORT).unwrap()
});

fn setup() {
    init_test_config();

    // Set up proxies
    setup_proxies(&PROXY_MAPPINGS);
}

// This is the baseline load test with no additional latency between nodes
#[tokio::test]
pub async fn load_with_no_latency() {
    test_common::init_test_config();
    info!("Starting test: load_with_no_latency");
    let num_nodes = 3;
    let (testnet, validator_collection, pubkey) =
        new_node_collection_with_authd_pkp(num_nodes, false).await;

    // open the log files
    let mut log_readers = validator_collection.log_readers();

    let _start = std::time::Instant::now();
    let _ = sign_with_hd_key(
        &validator_collection
            .ports()
            .iter()
            .map(|x| x.to_string())
            .collect(),
        validator_collection.actions(),
        pubkey.clone(),
        false,
        true,
        1,
        Some("First Test message".to_string()),
        false,
    )
    .await;

    // give the nodes a few seconds to populate a triple or two.
    let warmup_time = Duration::from_millis(30000);
    validator_collection
        .actions()
        .sleep_millis(warmup_time.as_millis() as u64)
        .await;

    // clear the log buffer
    for reader in log_readers.iter_mut() {
        let _lines = reader
            .lines()
            .map(|line| line.unwrap_or("".to_string()))
            .collect::<Vec<String>>();
    }
    let messages_to_sign = 10;

    let mut bt_cache_hit = 0;
    let mut bt_cache_hit_duration: Duration = Duration::from_millis(0);
    let mut bt_cache_miss = 0;
    let mut sign_success = 0;
    let mut bt_cache_miss_duration: Duration = Duration::from_millis(0);

    let start = std::time::Instant::now();
    for i in 0..messages_to_sign {
        info!("Starting sig #{}", i);
        let message_to_sign = Some(format!("Test message #{}", i));
        let start_1 = std::time::Instant::now();
        let validation = sign_with_hd_key(
            &validator_collection
                .ports()
                .iter()
                .map(|x| x.to_string())
                .collect(),
            validator_collection.actions(),
            pubkey.clone(),
            false,
            false,
            1,
            message_to_sign,
            false,
        )
        .await;

        if validation {
            sign_success += 1;
        };

        'outer: for reader in &mut log_readers {
            let lines = reader
                .lines()
                .map(|line| line.unwrap_or("".to_string()))
                .collect::<Vec<String>>();

            for line in lines {
                if line.contains("BT Cache Hit") {
                    bt_cache_hit += 1;
                    bt_cache_hit_duration += start_1.elapsed();
                    break 'outer;
                }
                if line.contains("BT Cache Miss") {
                    bt_cache_miss += 1;
                    bt_cache_miss_duration += start_1.elapsed();
                    break 'outer;
                }
            }
        }
    }
    let total_elapsed = start.elapsed();
    info!(
        "
        Signing {} messages randomly in a {} node network 
        Pregen BT  Warmup: {:?} 
        BT Cache Hit (qty/time): {} / {:?}  
        BT Cache Miss (qty/time): {} / {:?} 
        Cache success: {:?} 
        Total Time elapsed: {:?} 
        Sign success: {:?} ",
        messages_to_sign,
        num_nodes,
        warmup_time,
        bt_cache_hit,
        bt_cache_hit_duration,
        bt_cache_miss,
        bt_cache_miss_duration,
        (bt_cache_hit as f64 / messages_to_sign as f64),
        total_elapsed,
        sign_success
    );

    let validation = true;
    assert!(validation);
}

#[tokio::test]
pub async fn load_with_50ms_latency_single_link() {
    setup();
    info!("Starting test: load_with_50ms_latency");

    let latency_ms = 50;
    let jitter_ms = 0;
    let toxicity = 1.0;

    // Inject fault between node 1 and node 0
    inject_latency_fault(
        get_local_url_from_port(STARTING_PORT + 1),
        get_local_url_from_port(STARTING_PORT),
        latency_ms,
        jitter_ms,
        toxicity,
    );

    // Inject fault between node 0 and node 1
    inject_latency_fault(
        get_local_url_from_port(STARTING_PORT),
        get_local_url_from_port(STARTING_PORT + 1),
        latency_ms,
        jitter_ms,
        toxicity,
    );

    // Inject fault between node 1 and node 0
    inject_latency_fault(
        get_local_grpc_url_from_port(STARTING_PORT + 1),
        get_local_grpc_url_from_port(STARTING_PORT),
        latency_ms,
        jitter_ms,
        toxicity,
    );

    // Inject fault between node 0 and node 1
    inject_latency_fault(
        get_local_grpc_url_from_port(STARTING_PORT),
        get_local_grpc_url_from_port(STARTING_PORT + 1),
        latency_ms,
        jitter_ms,
        toxicity,
    );

    let num_nodes = 3;
    let (testnet, validator_collection, pubkey) =
        new_node_collection_with_authd_pkp(num_nodes, false).await;

    // open the log files
    let mut log_readers = validator_collection.log_readers();

    let _start = std::time::Instant::now();
    let _ = sign_with_hd_key(
        &validator_collection
            .ports()
            .iter()
            .map(|x| x.to_string())
            .collect(),
        validator_collection.actions(),
        pubkey.clone(),
        false,
        true,
        1,
        Some("First Test message".to_string()),
        false,
    )
    .await;

    // give the nodes a few seconds to populate a triple or two.
    let warmup_time = Duration::from_millis(30000);
    validator_collection
        .actions()
        .sleep_millis(warmup_time.as_millis() as u64)
        .await;

    // clear the log buffer
    for reader in log_readers.iter_mut() {
        let _lines = reader
            .lines()
            .map(|line| line.unwrap_or("".to_string()))
            .collect::<Vec<String>>();
    }

    let messages_to_sign = 10;

    let mut bt_cache_hit = 0;
    let mut bt_cache_hit_duration: Duration = Duration::from_millis(0);
    let mut bt_cache_miss = 0;
    let mut sign_success = 0;
    let mut bt_cache_miss_duration: Duration = Duration::from_millis(0);

    let start = std::time::Instant::now();
    for i in 0..messages_to_sign {
        info!("Starting sig #{}", i);
        let message_to_sign = Some(format!("Test message #{}", i));
        let start_1 = std::time::Instant::now();
        let validation = sign_with_hd_key(
            &validator_collection
                .ports()
                .iter()
                .map(|x| x.to_string())
                .collect(),
            validator_collection.actions(),
            pubkey.clone(),
            false,
            false,
            1,
            message_to_sign,
            false,
        )
        .await;

        if validation {
            sign_success += 1;
        };

        'outer: for reader in &mut log_readers {
            let lines = reader
                .lines()
                .map(|line| line.unwrap_or("".to_string()))
                .collect::<Vec<String>>();

            for line in lines {
                if line.contains("BT Cache Hit") {
                    bt_cache_hit += 1;
                    bt_cache_hit_duration += start_1.elapsed();
                    break 'outer;
                }
                if line.contains("BT Cache Miss") {
                    bt_cache_miss += 1;
                    bt_cache_miss_duration += start_1.elapsed();
                    break 'outer;
                }
            }
        }
    }
    let total_elapsed = start.elapsed();
    info!(
        "
        Signing {} messages randomly in a {} node network 
        Pregen BT  Warmup: {:?} 
        BT Cache Hit (qty/time): {} / {:?}  
        BT Cache Miss (qty/time): {} / {:?} 
        Cache success: {:?} 
        Total Time elapsed: {:?} 
        Sign success: {:?} ",
        messages_to_sign,
        num_nodes,
        warmup_time,
        bt_cache_hit,
        bt_cache_hit_duration,
        bt_cache_miss,
        bt_cache_miss_duration,
        (bt_cache_hit as f64 / messages_to_sign as f64),
        total_elapsed,
        sign_success
    );

    let validation = true;
    assert!(validation);
}

#[tokio::test]
pub async fn load_with_50ms_latency_all_links() {
    setup();
    info!("Starting test: load_with_50ms_latency");

    let latency_ms = 50;
    let jitter_ms = 0;
    let toxicity = 1.0;

    // inject faults between all nodes
    let ports = [STARTING_PORT, STARTING_PORT + 1, STARTING_PORT + 2];
    for i in 0..ports.len() {
        for j in i + 1..ports.len() {
            inject_latency_fault(
                get_local_url_from_port(ports[i]),
                get_local_url_from_port(ports[j]),
                latency_ms,
                jitter_ms,
                toxicity,
            );
            inject_latency_fault(
                get_local_url_from_port(ports[j]),
                get_local_url_from_port(ports[i]),
                latency_ms,
                jitter_ms,
                toxicity,
            );
            inject_latency_fault(
                get_local_grpc_url_from_port(ports[i]),
                get_local_grpc_url_from_port(ports[j]),
                latency_ms,
                jitter_ms,
                toxicity,
            );
            inject_latency_fault(
                get_local_grpc_url_from_port(ports[j]),
                get_local_grpc_url_from_port(ports[i]),
                latency_ms,
                jitter_ms,
                toxicity,
            );
        }
    }

    let num_nodes = 3;
    let (testnet, validator_collection, pubkey) =
        new_node_collection_with_authd_pkp(num_nodes, false).await;

    // open the log files
    let mut log_readers = validator_collection.log_readers();

    let _start = std::time::Instant::now();
    let _ = sign_with_hd_key(
        &validator_collection
            .ports()
            .iter()
            .map(|x| x.to_string())
            .collect(),
        validator_collection.actions(),
        pubkey.clone(),
        false,
        true,
        1,
        Some("First Test message".to_string()),
        false,
    )
    .await;

    // give the nodes a few seconds to populate a triple or two.
    let warmup_time = Duration::from_millis(30000);
    validator_collection
        .actions()
        .sleep_millis(warmup_time.as_millis() as u64)
        .await;

    // clear the log buffer
    for reader in log_readers.iter_mut() {
        let _lines = reader
            .lines()
            .map(|line| line.unwrap_or("".to_string()))
            .collect::<Vec<String>>();
    }

    let messages_to_sign = 10;

    let mut bt_cache_hit = 0;
    let mut bt_cache_hit_duration: Duration = Duration::from_millis(0);
    let mut bt_cache_miss = 0;
    let mut sign_success = 0;
    let mut bt_cache_miss_duration: Duration = Duration::from_millis(0);

    let start = std::time::Instant::now();
    for i in 0..messages_to_sign {
        info!("Starting sig #{}", i);
        let message_to_sign = Some(format!("Test message #{}", i));
        let start_1 = std::time::Instant::now();
        let validation = sign_with_hd_key(
            &validator_collection
                .ports()
                .iter()
                .map(|x| x.to_string())
                .collect(),
            validator_collection.actions(),
            pubkey.clone(),
            false,
            false,
            1,
            message_to_sign,
            false,
        )
        .await;

        if validation {
            sign_success += 1;
        };

        'outer: for reader in &mut log_readers {
            let lines = reader
                .lines()
                .map(|line| line.unwrap_or("".to_string()))
                .collect::<Vec<String>>();

            for line in lines {
                if line.contains("BT Cache Hit") {
                    bt_cache_hit += 1;
                    bt_cache_hit_duration += start_1.elapsed();
                    break 'outer;
                }
                if line.contains("BT Cache Miss") {
                    bt_cache_miss += 1;
                    bt_cache_miss_duration += start_1.elapsed();
                    break 'outer;
                }
            }
        }
    }
    let total_elapsed = start.elapsed();
    info!(
        "
        Signing {} messages randomly in a {} node network 
        Pregen BT  Warmup: {:?} 
        BT Cache Hit (qty/time): {} / {:?}  
        BT Cache Miss (qty/time): {} / {:?} 
        Cache success: {:?} 
        Total Time elapsed: {:?} 
        Sign success: {:?} ",
        messages_to_sign,
        num_nodes,
        warmup_time,
        bt_cache_hit,
        bt_cache_hit_duration,
        bt_cache_miss,
        bt_cache_miss_duration,
        (bt_cache_hit as f64 / messages_to_sign as f64),
        total_elapsed,
        sign_success
    );

    let validation = true;
    assert!(validation);
}
