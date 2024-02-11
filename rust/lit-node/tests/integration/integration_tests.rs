use std::{sync::Arc, time::Duration};

use crate::common::{
    self,
    assertions::NetworkIntegrityChecker,
    init_test_config,
    node_collection::{choose_random_indices, NodeCollection},
    testnet::{
        contracts::{Contracts, StakingContractInitialConfig},
        contracts_repo::default_staker_ip_addresses,
        scenario::Scenario,
        NodeAccount, Testnet,
    },
};
use common::testnet::scenario::InitialDKGState;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::types::{H160, U256};
use futures::future::{join_all, BoxFuture};
use lit_blockchain::{
    contracts::staking::{Staking, StakingEvents, Validator},
    resolver::rpc::get_provider,
};
use lit_node::peers::peer_state::models::NetworkState;
use rand::Rng;
use test_case::test_case;
use tracing::info;

fn setup() {
    init_test_config();
}

/// Tests a node being kicked after locking and signaling ready but before the epoch advances.
/// The nodes will re-lock and retry the DKG / reshare with the new locked set
#[tokio::test]
async fn retry_after_signaling_ready_test() {
    setup();

    info!("TEST: retry_after_signaling_ready_test");

    // Start a new node collection and wait for the DKG to complete
    // and root keys to be voted for.
    let initial_port = 7470;
    let num_nodes = 6;
    let s = Scenario::builder()
        .num_staked(num_nodes)
        .num_nodes(num_nodes)
        .num_awake_initially(num_nodes)
        .initial_port(initial_port)
        .staking_contract_initial_config(
            StakingContractInitialConfig::builder()
                .epoch_length(U256::from(300))
                .build(),
        )
        .build()
        .await;
    let (nc, scenario) = NodeCollection::from_scenario(s).await;

    // Assert that the network works
    let network_checker = NetworkIntegrityChecker::new(&scenario).await;
    network_checker.check(&nc, &scenario).await;

    // as soon as any node has signalled ready, kick that node
    // let staking_clone = scenario.contracts.staking.clone();
    let mut provider_mut = get_provider(scenario.testnet.chain_name.clone(), 0)
        .expect("Error retrieving provider - check name and yaml.");
    let provider = provider_mut.set_interval(Duration::from_millis(10)).clone();
    let sk = SigningKey::from_bytes(
        scenario
            .testnet
            .deploy_account
            .staker_address_private_key
            .as_bytes()
            .into(),
    )
    .unwrap();
    let wallet = LocalWallet::from(sk).with_chain_id(scenario.testnet.chain_id);
    let staking_clone = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
        scenario.contracts.staking.address(),
        Arc::new(SignerMiddleware::new(provider, wallet)),
    );
    let handle = tokio::spawn(async move {
        // Subscribe to staking events
        let events = staking_clone.events();
        let mut stream = events.stream().await.unwrap();
        while let Some(Ok(event)) = stream.next().await {
            match event {
                StakingEvents::ReadyForNextEpochFilter(ready_event) => {
                    info!("Received ReadyForNextEpochFilter event: {:?}", ready_event);

                    let staker_address_to_kick = ready_event.staker;
                    // kick this validator

                    let func =
                        staking_clone.admin_kick_validator_in_next_epoch(staker_address_to_kick);

                    let tx = func.send().await.unwrap();
                    match tx.await {
                        Ok(receipt) => {
                            info!(
                                "Successfully kicked validator: {:?} - receipt: {:?}",
                                staker_address_to_kick, receipt
                            );
                        }
                        Err(e) => {
                            info!("Failed to kick validator: {:?}", e);
                        }
                    }

                    // confirm epoch state
                    let epoch_state =
                        NetworkState::from(staking_clone.state().call().await.unwrap());
                    info!("Epoch state after kicking: {:?}", epoch_state);
                    assert!(
                        epoch_state == NetworkState::Unlocked,
                        "Epoch state should be unlocked after kicking"
                    );

                    // confirm node is kicked
                    let kicked_validators = staking_clone.get_kicked_validators().await.unwrap();
                    assert!(
                        kicked_validators.contains(&staker_address_to_kick),
                        "Node {:?} is not in the set of kicked nodes: {:?}",
                        staker_address_to_kick,
                        kicked_validators
                    );
                    break;
                }
                _ => {
                    info!("Received event: {:?}", event);
                }
            }
        }
    });

    let actions_clone = scenario.actions.clone();
    let advance_handle = tokio::spawn(async move {
        // advance timestamp so that nodes can lock and advance
        actions_clone.increase_blockchain_timestamp(300).await;
        info!("Increased blockchain timestamp by 300 seconds");
    });

    join_all(vec![handle, advance_handle]).await;

    scenario.actions.wait_for_active().await;
    info!("Network is active again");
    // sleep for a few secs so the nodes get their peers again
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    // how many nodes are in the current set?
    let current_validators = scenario.actions.get_current_validator_structs().await;
    info!("Current validators length: {:?}", current_validators.len());
    assert!(
        current_validators.len() == num_nodes - 1,
        "There should be 1 less validator in the current node set since we kicked one"
    );
    let next_validators = scenario.actions.get_next_validators().await;
    info!("Next validators length: {:?}", next_validators.len());
    assert!(
        current_validators.len() == num_nodes - 1,
        "There should be 1 less validator in the next node set since we kicked one"
    );
    network_checker.check(&nc, &scenario).await;
}

/// This test is used to test when a staked node never wakes up, which results in it getting kicked
/// before the rest of the network successfully advances to the new epoch.
#[tokio::test]
async fn one_node_never_wakes() {
    setup();

    info!("TEST: one_node_never_wakes");

    // Start the node collection
    let initial_port = 7470;
    let num_nodes = 6;
    let s = Scenario::builder()
        .num_staked(num_nodes)
        .num_nodes(num_nodes)
        .num_awake_initially(num_nodes - 1)
        .initial_port(initial_port)
        .initial_dkg_state(InitialDKGState::TestIsOnlyStaking)
        .wait_for_dkg_to_complete(false)
        .staking_contract_initial_config(
            StakingContractInitialConfig::builder()
                .complaint_tolerance(U256::from(5))
                .build(),
        )
        .build()
        .await;
    let (mut nc, s) = NodeCollection::from_scenario(s).await;

    info!("Waiting for sleeping node to get kicked");
    let node_idx_asleep = *nc
        .asleep_initially()
        .iter()
        .next()
        .expect("No node is asleep");
    let staker_address_to_kick = s.testnet.node_accounts[node_idx_asleep].staker_address;
    let voting_status = s
        .actions
        .wait_for_voting_status_to_kick_validator(
            U256::from(1),
            staker_address_to_kick,
            H160::random(), // For simplicity, we only care about asserting the number of votes.
            1, // In the genesis epoch, the number of votes required to kick a node is 1.
        )
        .await;
    assert!(voting_status.is_ok());

    // Assert that the node that was asleep got kicked
    let voting_status = voting_status.unwrap();
    info!("Voting status: {:?}", voting_status);
    assert!(voting_status.votes.as_usize() >= 1);

    // After the node is kicked, wait for the DKG to complete
    s.actions.wait_for_initial_epoch().await;

    // Assert that the current validator set is 1 less.
    let current_validators = s.actions.get_current_validators().await;
    info!("Current validators: {:?}", current_validators);

    assert_eq!(current_validators.len(), num_nodes - 1);
}

/// This tests that a node goes offline and online again and can resume operation with the rest of the network of nodes.
#[tokio::test]
async fn node_restarts() {
    setup();

    info!("TEST: node_restarts");

    // Start a new node collection and wait for the DKG to complete
    // and root keys to be voted for.
    let initial_port = 7470;
    let num_nodes = 6;
    let s = Scenario::builder()
        .num_staked(num_nodes)
        .num_nodes(num_nodes)
        .num_awake_initially(num_nodes)
        .initial_port(initial_port)
        .staking_contract_initial_config(
            StakingContractInitialConfig::builder()
                .epoch_length(U256::from(300))
                .build(),
        )
        .build()
        .await;
    let (mut node_collection, scenario) = NodeCollection::from_scenario(s).await;

    // Run network checks
    let network_checker = NetworkIntegrityChecker::new(&scenario).await;
    network_checker.check(&node_collection, &scenario).await;

    // Get random idx within the node collection
    let mut rng = rand::thread_rng();
    let random_node_idx_to_shutdown = rng.gen_range(0..node_collection.size());
    node_collection.shutdown_node(random_node_idx_to_shutdown);

    // Wait for 5 seconds before starting the node back up.  the node should not be kicked, since this is shorter than the complaint tolerance.
    info!(
        "Waiting for 5 seconds before re-starting the offline node at idx {:?}",
        random_node_idx_to_shutdown
    );
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Start the node back up
    info!("Restarting the shutdown node");
    node_collection
        .start_node(random_node_idx_to_shutdown)
        .await;
    info!("Node restarted");

    // give the node like, 5 seconds to come back online
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Run network checks
    network_checker.check(&node_collection, &scenario).await;
    info!("Network integrity check passed after node came back online");

    // Fast forward time to allow nodes to start a DKG to advance to the next epoch.
    scenario.actions.increase_blockchain_timestamp(300).await;

    // After next epoch arrives, run interpolation and decryption tests.
    scenario.actions.wait_for_lock().await;
    scenario.actions.wait_for_active().await;
    network_checker.check(&node_collection, &scenario).await;
}

/// This tests that a node registers an invalid port and gets kicked.
#[test_case(1; "with invalid port")]
/// This tests that a node registers the same IP and port as another node and gets kicked.
#[test_case(2; "with same IP and port")]
#[tokio::test]
async fn one_node_conflicting_networking_info(test_case: usize) {
    setup();

    info!("TEST: one_node_conflicting_networking_info");

    let num_nodes = 6;
    let (random_node_idx_to_be_kicked, new_staked_port_of_node_to_be_kicked) = match test_case {
        1 => {
            // Choose a random node index to stake with an invalid port.
            let random_node_idx = choose_random_indices(num_nodes, 1)
                .iter()
                .cloned()
                .collect::<Vec<usize>>()[0];

            (random_node_idx, "55555".to_owned())
        }
        2 => {
            // Randomly choose an impersonator and a victim.
            let random_node_indices: Vec<usize> = choose_random_indices(num_nodes, 2)
                .iter()
                .cloned()
                .collect();
            let random_node_idx_impersonater = random_node_indices[0];
            let random_node_idx_impersonated = random_node_indices[1];
            let default_ip_addresses = default_staker_ip_addresses(7470, num_nodes);
            let new_port_of_impersonator = default_ip_addresses[random_node_idx_impersonated]
                .split(':')
                .collect::<Vec<&str>>()[1];

            (
                random_node_idx_impersonater,
                new_port_of_impersonator.to_owned(),
            )
        }
        _ => panic!("Invalid test case"),
    };

    // Start the node collection
    let initial_port = 7470;
    let testnet_builder = Testnet::builder()
        .which_testnet(common::testnet::WhichTestnet::Anvil)
        .num_nodes(num_nodes)
        .num_staked(num_nodes)
        .staker_account_setup_mapper(Box::new(move |args: (usize, NodeAccount, Contracts)| {
            let random_node_idx_to_be_kicked_clone = random_node_idx_to_be_kicked;
            let new_staked_port_of_node_to_be_kicked_clone =
                new_staked_port_of_node_to_be_kicked.clone();

            Box::pin(async move {
                if args.0 == random_node_idx_to_be_kicked_clone {
                    // Send a TX to chain to update the staker information with an invalid port.
                    let staker_provider = args.1.signing_provider;
                    let staking =
                        Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
                            args.2.staking.address(),
                            staker_provider.clone(),
                        );

                    let validator: Validator = staking
                        .validators(args.1.staker_address)
                        .call()
                        .await
                        .expect("Failed to get staker config");
                    let new_staked_port = new_staked_port_of_node_to_be_kicked_clone
                        .parse::<u32>()
                        .expect("Failed to parse port");
                    let update_cc = staking.set_ip_port_node_address_and_communication_pub_keys(
                        validator.ip,
                        validator.ipv_6,
                        new_staked_port,
                        validator.node_address,
                        validator.sender_pub_key,
                        validator.receiver_pub_key,
                    );

                    Contracts::process_contract_call(
                        update_cc,
                        "setting staker port to invalid port",
                    )
                    .await;

                    info!(
                        "Successfully updated staker ({}) {:?}: port {:?} -> {:?}",
                        args.0, args.1.staker_address, validator.port, new_staked_port
                    );
                }

                Ok(())
            }) as BoxFuture<'static, Result<(), anyhow::Error>>
        }));
    let s = Scenario::builder()
        .num_staked(num_nodes)
        .num_nodes(num_nodes)
        .num_awake_initially(num_nodes)
        .initial_port(initial_port)
        .initial_dkg_state(InitialDKGState::TestIsOnlyStaking)
        .wait_for_dkg_to_complete(false)
        .testnet_builder(testnet_builder)
        .build()
        .await;
    let (nc, s) = NodeCollection::from_scenario(s).await;

    info!(
        "Waiting for node {} to be kicked",
        random_node_idx_to_be_kicked
    );
    let staker_address_to_kick =
        s.testnet.node_accounts[random_node_idx_to_be_kicked].staker_address;
    let voting_status = s
        .actions
        .wait_for_voting_status_to_kick_validator(
            U256::from(1),
            staker_address_to_kick,
            H160::random(), // For simplicity, we only care about asserting the number of votes.
            1, // In the genesis epoch, the number of votes required to kick a node is 1.
        )
        .await;
    assert!(voting_status.is_ok());

    // After the node is kicked, wait for the DKG to complete
    s.actions.wait_for_epoch(U256::from(2)).await;

    // Assert that the current validator set is 1 less.
    let current_validators = s.actions.get_current_validators().await;
    info!("Current validators: {:?}", current_validators);
    assert_eq!(current_validators.len(), num_nodes - 1);

    // Run network checks
    let network_checker = NetworkIntegrityChecker::new(&s).await;
    network_checker.check(&nc, &s).await;
}
