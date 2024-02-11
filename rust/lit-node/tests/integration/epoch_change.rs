use crate::common::{
    new_node_collection_with_custom_config,
    node_collection::NodeCollection,
    testnet::{
        node_config::CustomNodeRuntimeConfig,
        scenario::{InitialDKGState, Scenario},
    },
};
use ethers::types::{Address, U256};
use tracing::info;

#[tokio::test]
#[doc = "This test is used to test the DKG reshare process when a new node is added and the threshold stays the same."]
pub async fn add_one_node_and_keep_threshold() {
    crate::common::init_test_config();

    let num_nodes = 7;
    let num_staked = 6;
    let changed_nodes = 1; // 1 node is added

    info!("TEST: add_one_node_and_keep_threshold- 2 Recovery DKGs");
    refresh_third_epoch_and_update_backup_members(num_nodes, num_staked, changed_nodes, true).await;

    info!("TEST: add_one_node_and_keep_threshold- 1 Recovery DKG");
    refresh_third_epoch_and_update_backup_members(num_nodes, num_staked, changed_nodes, false)
        .await;
}

#[tokio::test]
#[doc = "This test is used to test the DKG reshare process when a new node is added and the threshold changes."]
pub async fn add_one_node_and_change_threshold() {
    crate::common::init_test_config();

    let num_nodes = 5;
    let num_staked = 4;
    let changed_nodes = 1; // 1 node is added

    info!("TEST: add_one_node_and_change_threshold- 2 Recovery DKGs");
    refresh_third_epoch_and_update_backup_members(num_nodes, num_staked, changed_nodes, true).await;

    info!("TEST: add_one_node_and_change_threshold- 1 Recovery DKG");
    refresh_third_epoch_and_update_backup_members(num_nodes, num_staked, changed_nodes, false)
        .await;
}

#[tokio::test]
#[doc = "This test is used to test the DKG reshare process when a new node is removed."]
pub async fn remove_one_node_and_change_threshold() {
    crate::common::init_test_config();

    let num_nodes = 5;
    let num_staked = 5;
    let changed_nodes = -1; // 1 node is removed

    info!("TEST: remove_one_node_and_change_threshold- 2 Recovery DKGs");
    refresh_third_epoch_and_update_backup_members(num_nodes, num_staked, changed_nodes, true).await;

    info!("TEST: remove_one_node_and_change_threshold- 1 Recovery DKG");
    refresh_third_epoch_and_update_backup_members(num_nodes, num_staked, changed_nodes, false)
        .await;
}

pub async fn refresh_third_epoch_and_update_backup_members(
    num_nodes: i32,
    num_staked: i32,
    changed_nodes: i32,
    register_backup_parties_again: bool,
) {
    let (_node_collection, scenario, current_epoch, second_epoch_count) =
        setup_and_second_epoch(num_nodes, num_staked, changed_nodes).await;

    // Wait for the new node to be active.
    scenario.actions.wait_for_active().await;

    // Re-register Backup Party
    if register_backup_parties_again {
        let party_members: Vec<Address> = scenario.testnet.node_accounts
            [0..num_staked as usize - 1] // Also updating the number of backup members
            .iter()
            .map(|na| na.staker_address)
            .collect();
        info!("New party_members- {:?}", party_members);
        let owner = &scenario.testnet.deploy_account;
        scenario
            .actions
            .register_backup_parties(party_members, owner)
            .await;
    }

    //in test peers refresh every 1 second, so this allows time to refresh peer data after the new node is staked and active.
    // the real solution should be to either vote on locking, or not allow locking until every node has every other nodes' commkeys - though that coordination is tough, without gossiping.
    scenario.actions.sleep_millis(2000).await;

    // Fast forward the network by 300 seconds, and wait for the new node to be active - effectively waiting for the next epoch.
    scenario.actions.increase_blockchain_timestamp(300).await;

    // Wait for DKG to finish - nodes become active once more.
    scenario.actions.wait_for_epoch(current_epoch + 1).await;

    // Confirm that Recovery DKG has occured, and the nodes have voted for root keys
    scenario.actions.wait_for_recovery_keys().await;

    // Confirm that Refresh DKG has occured, and the nodes have voted for root keys
    scenario.actions.wait_for_root_keys(&scenario).await;

    // Display the final validator count.
    let final_count = scenario.actions.get_current_validator_count().await as u32;
    info!("Third epoch validators count: {}", final_count);
    info!("Second epoch validators count: {}", second_epoch_count);

    let result = final_count == second_epoch_count;
    assert!(result);
}

#[doc = "This test is used to test the DKG reshare process when the network peer set changes."]
pub async fn setup_and_second_epoch(
    num_nodes: i32,
    num_staked: i32,
    changed_nodes: i32,
) -> (NodeCollection, Scenario, U256, u32) {
    let (node_collection, scenario) = new_node_collection_with_custom_config(
        num_staked as usize,
        num_nodes as usize,
        Some(InitialDKGState::TestIsStakingAndDKG),
        true,
        false,
        CustomNodeRuntimeConfig::default(),
    )
    .await;

    // nodes are already staked but we can double check
    for i in 0..num_staked as usize {
        let node_account = scenario.testnet.node_accounts[i].clone();
        info!("Node Account Ensured: {:?}", &node_account.staker_address);
        let result = scenario.actions.ensure_node_staked(node_account).await;
        assert!(result.is_ok());
    }

    // Register Backup Party
    let party_members: Vec<Address> = scenario.testnet.node_accounts[0..num_staked as usize]
        .iter()
        .map(|na| na.staker_address)
        .collect();
    info!("party_members- {:?}", party_members);
    let owner = &scenario.testnet.deploy_account;
    scenario
        .actions
        .register_backup_parties(party_members, owner)
        .await;

    // Wait for the initial epoch to end
    scenario.actions.wait_for_initial_epoch().await;

    // Confirm that DKG has occured, and the nodes have voted for root keys
    scenario.actions.wait_for_root_keys(&scenario).await;

    // Display our current validator count.
    let initial_count = scenario.actions.get_current_validator_count().await as i32;
    info!("Initial validators count: {}", initial_count);
    let current_epoch = scenario.actions.get_current_epoch().await;

    if changed_nodes > 0 {
        // Add a new node to the network.
        info!("Increasing network size");
        let node_account = scenario.testnet.node_accounts[num_staked as usize].clone();
        info!(
            "Node Account Ensured: {:?}",
            &node_account.clone().staker_address
        );
        let result = scenario
            .actions
            .ensure_node_staked(node_account.clone())
            .await;
        if let Err(e) = result {
            info!("Error: {:?}", e);
            panic!("{}", e);
        }
    } else if changed_nodes < 0 {
        // Remove a node from the network.
        info!("Decreasing network size");
        let node_account = scenario.testnet.node_accounts[num_staked as usize - 1].clone();
        info!("Node Account Ensured: {:?}", &node_account.staker_address);
        let result = scenario.actions.ensure_node_unstaked(node_account).await;
        if let Err(e) = result {
            info!("Error: {:?}", e);
            panic!("{}", e);
        }
    }

    // Wait for the new node to be active.
    scenario.actions.wait_for_active().await;

    //in test peers refresh every 1 second, so this allows time to refresh peer data after the new node is staked and active.
    // the real solution should be to either vote on locking, or not allow locking until every node has every other nodes' commkeys - though that coordination is tough, without gossiping.
    scenario.actions.sleep_millis(2000).await;

    // Fast forward the network by 300 seconds, and wait for the new node to be active - effectively waiting for the next epoch.
    scenario.actions.increase_blockchain_timestamp(300).await;

    // Lock the validators for the next epoch;
    // scenario.actions.lock_validators_for_next_epoch().await;

    // Wait for DKG to finish - nodes become active once more.
    scenario.actions.wait_for_epoch(current_epoch + 1).await;

    // Confirm that Recovery DKG has occured, and the nodes have voted for root keys
    scenario.actions.wait_for_recovery_keys().await;

    // Confirm that Reshare DKG has occured, and the nodes have voted for root keys
    scenario.actions.wait_for_root_keys(&scenario).await;

    // Display the final validator count.
    let final_count = scenario.actions.get_current_validator_count().await as u32;
    let second_epoch_count = (initial_count + changed_nodes) as u32;

    info!("Second validators count: {}", final_count);
    info!(
        "Initial validators count: {} + changed nodes count {} = {}",
        initial_count, changed_nodes, second_epoch_count
    );

    let result = final_count == second_epoch_count;
    assert!(result);

    (node_collection, scenario, current_epoch + 1, final_count)
}
