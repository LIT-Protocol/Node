#![cfg(all(feature = "proxy_http", feature = "testing"))]

use crate::common::assertions::NetworkIntegrityChecker;
use crate::common::testnet::contracts::StakingContractInitialConfig;
use crate::common::{self, new_node_collection, new_node_collection_with_contract_config};

use chrono::Duration;
use common::faults::{get_local_url_from_port, inject_latency_fault, setup_proxies, FaultType};
use common::init_test_config;
use ethers::types::U256;
use lit_node::networking::http::proxy::mapping::HttpClientProxyMapping;
use once_cell::sync::Lazy;
use test_case::test_case;
use tracing::{debug, info};

use crate::common::faults::{
    generate_and_save_proxy_mappings_for_local_testing,
    inject_fault_between_all_sources_to_random_target, FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS,
};

const FAULT_TEST_NUM_NODES: usize = 10;
const STARTING_PORT: usize = 7470;

static PROXY_MAPPINGS: Lazy<HttpClientProxyMapping> = Lazy::new(|| {
    generate_and_save_proxy_mappings_for_local_testing(FAULT_TEST_NUM_NODES, STARTING_PORT).unwrap()
});

fn setup() {
    init_test_config();

    // Set up proxies
    setup_proxies(&PROXY_MAPPINGS);
}

/// This tests that when the link between node 0 and node 1 has a transient fault in one direction, it results in
/// node 1 voting to kick node 0 and vice versa.
#[tokio::test]
pub async fn single_link_fault_transient_oneway() {
    setup();

    info!("Starting single_link_fault_transient_oneway test");

    // Inject fault between node 1 and node 0
    // This is intentionally placed right a little above the timeout duration with a 3s jitter up/down
    // so that we can simulate a transient fault.
    inject_latency_fault(
        get_local_url_from_port(7470 + 1),
        get_local_url_from_port(7470),
        Duration::seconds(i64::try_from(FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS + 2).unwrap())
            .num_milliseconds()
            .try_into()
            .unwrap(),
        Duration::seconds(3).num_milliseconds().try_into().unwrap(),
        1.0,
    );

    // Start staking and DKG
    let (nc, s) = new_node_collection_with_contract_config(
        FAULT_TEST_NUM_NODES,
        FAULT_TEST_NUM_NODES,
        None,
        false,
        true,
        Some(
            StakingContractInitialConfig::builder()
                .complaint_tolerance(U256::from(1))
                .build(),
        ),
    )
    .await;

    // Get staker address of the validator to be kicked (node 0)
    let node_0_staker_address = s.testnet.node_accounts[0].staker_address;

    // Get staker address of the validator voting to kick (node 1)
    let node_1_staker_address = s.testnet.node_accounts[1].staker_address;

    // Assert that, eventually, there is 1 vote from node 1 voting to kick node 0.
    let voting_status = s
        .actions
        .wait_for_voting_status_to_kick_validator(
            U256::from(1),
            node_0_staker_address,
            node_1_staker_address,
            1,
        )
        .await;
    assert!(voting_status.is_ok());

    // Drop node collection properly
    drop(nc);

    let voting_status = voting_status.unwrap();
    assert_eq!(voting_status.votes.as_usize(), 1);
    assert!(voting_status.did_voter_kick_validator);
}

/// This tests that the nodes are able to DKG and sign when a single node is semi-faulty.
#[tokio::test]
async fn single_node_semi_faulty() {
    setup();

    info!("TEST: single_node_semi_faulty");

    // Inject faults
    inject_fault_between_all_sources_to_random_target(
        FaultType::LatencyAroundClientTimeout,
        STARTING_PORT,
        FAULT_TEST_NUM_NODES,
    );

    // Start a new node collection
    let (nc, s) =
        new_node_collection(FAULT_TEST_NUM_NODES, FAULT_TEST_NUM_NODES, None, true, true).await;

    s.actions.wait_for_active().await; // wait for state to become active again after DKG(s)

    // Assert that the node didn't get kicked
    let current_validators = s.actions.get_current_validators().await;
    assert_eq!(current_validators.len(), FAULT_TEST_NUM_NODES);

    // Assert that the network can still perform operations
    let network_checker = NetworkIntegrityChecker::new(&s).await;
    network_checker.check(&nc, &s).await;
}

/// This tests that the nodes are able to DKG and sign when a single node is faulty
/// during the second DKG.
#[test_case(FaultType::LatencyBelowClientTimeout, false; "with latency below client timeout")]
#[test_case(FaultType::SlowTcpClosing, false; "with slow tcp closing")]
#[test_case(FaultType::TimeoutAboveClientTimeout, true; "with timeout above client timeout")]
#[test_case(FaultType::Slicer, false; "with slicer")]
#[tokio::test]
pub async fn single_node_fault_after_first_dkg_during_second_dkg(
    fault_type: FaultType,
    is_faulty_node_kicked: bool,
) {
    setup();

    info!("Starting test with fault type: {:?}", fault_type);

    // Start staking and Initial DKG
    let (nc, s) = new_node_collection_with_contract_config(
        FAULT_TEST_NUM_NODES,
        FAULT_TEST_NUM_NODES,
        None,
        true,
        true,
        Some(
            StakingContractInitialConfig::builder()
                .complaint_tolerance(U256::from(1))
                .build(),
        ),
    )
    .await;

    s.actions.wait_for_active().await; // wait for state to become active again after DKG(s)
    let starting_epoch = s.actions.get_current_epoch().await;
    debug!("Starting epoch: {}", starting_epoch);
    assert!(
        starting_epoch == U256::from(2),
        "Starting epoch should be 2"
    );

    let network_checker = NetworkIntegrityChecker::new(&s).await;

    // Inject faults before the next DKG starts for all connections upstream to a randomly
    // selected node.
    let random_faulty_node_port = inject_fault_between_all_sources_to_random_target(
        fault_type,
        STARTING_PORT,
        FAULT_TEST_NUM_NODES,
    );

    // fast forward timestamp so that nodes can lock and advance.
    s.actions.increase_blockchain_timestamp(300).await;

    s.actions.wait_for_lock().await; // wait for lock, triggering DKG
    let epoch_number = s.actions.get_current_epoch().await;
    info!(
        "Validator set locked for Epoch 3 - we are in Epoch {}",
        epoch_number
    );
    assert!(
        epoch_number == U256::from(2),
        "Epoch number should still be 2"
    );
    if is_faulty_node_kicked {
        info!(
            "Waiting for faulty node with port {} to be kicked",
            random_faulty_node_port
        );
        // Assert that, eventually, there are a threshold number (7) of votes from nodes voting to kick the randomly chosen node with faults.
        let random_faulty_node_idx = random_faulty_node_port - STARTING_PORT;
        let first_non_faulty_node_idx = {
            let mut first_non_faulty_node_idx = 99usize;
            for i in 0..FAULT_TEST_NUM_NODES {
                if i != random_faulty_node_idx {
                    first_non_faulty_node_idx = i;
                    break;
                }
            }
            first_non_faulty_node_idx
        };
        let staker_address_to_kick = s.testnet.node_accounts[random_faulty_node_idx].staker_address;
        let staker_address_of_non_faulty_node =
            s.testnet.node_accounts[first_non_faulty_node_idx].staker_address;
        let get_voting_status_res = s
            .actions
            .wait_for_voting_status_to_kick_validator(
                U256::from(2),
                staker_address_to_kick,
                staker_address_of_non_faulty_node,
                6,
            )
            .await;
        assert!(get_voting_status_res.is_ok());
        info!(
            "Faulty node with port {} and staker_address {} is kicked",
            random_faulty_node_port, staker_address_to_kick
        );
    }

    // Wait for network to advance to new epoch after DKG
    s.actions.wait_for_epoch(starting_epoch + 1).await;

    // Run network checks.
    network_checker.check(&nc, &s).await;
}
