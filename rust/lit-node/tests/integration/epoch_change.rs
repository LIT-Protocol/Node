use ethers::types::U256;
use test_common::{
    new_node_collection_with_custom_config,
    testnet::{
        contracts_repo::fetch_node_config_file_from_node_account,
        node_config::CustomNodeRuntimeConfig, Testnet,
    },
    validator::{Node, ValidatorCollection},
};
use tracing::info;

#[tokio::test]
#[doc = "This test is used to test the DKG reshare process when a new node is added and the threshold stays the same."]
pub async fn add_one_node_and_keep_threshold() {
    test_common::init_test_config();

    let num_staked_and_joined_validators = 6;
    let num_staked_only_validators = 1;
    let changed_nodes = 1; // 1 node is added

    info!("TEST: add_one_node_and_keep_threshold- 2 Recovery DKGs");
    refresh_third_epoch_and_update_backup_members(
        num_staked_and_joined_validators,
        num_staked_only_validators,
        changed_nodes,
        true,
    )
    .await;

    info!("TEST: add_one_node_and_keep_threshold- 1 Recovery DKG");
    refresh_third_epoch_and_update_backup_members(
        num_staked_and_joined_validators,
        num_staked_only_validators,
        changed_nodes,
        false,
    )
    .await;
}

#[tokio::test]
#[doc = "This test is used to test the DKG reshare process when a new node is added and the threshold changes."]
pub async fn add_one_node_and_change_threshold() {
    test_common::init_test_config();

    let num_staked_and_joined_validators = 4;
    let num_staked_only_validators = 1;
    let changed_nodes = 1; // 1 node is added

    info!("TEST: add_one_node_and_change_threshold- 2 Recovery DKGs");
    refresh_third_epoch_and_update_backup_members(
        num_staked_and_joined_validators,
        num_staked_only_validators,
        changed_nodes,
        true,
    )
    .await;

    info!("TEST: add_one_node_and_change_threshold- 1 Recovery DKG");
    refresh_third_epoch_and_update_backup_members(
        num_staked_and_joined_validators,
        num_staked_only_validators,
        changed_nodes,
        false,
    )
    .await;
}

#[tokio::test]
#[doc = "This test is used to test the DKG reshare process when a new node is removed."]
pub async fn remove_one_node_and_change_threshold() {
    test_common::init_test_config();

    let num_staked_and_joined_validators = 5;
    let num_staked_only_validators = 0;
    let changed_nodes = -1; // 1 node is removed

    info!("TEST: remove_one_node_and_change_threshold- 2 Recovery DKGs");
    refresh_third_epoch_and_update_backup_members(
        num_staked_and_joined_validators,
        num_staked_only_validators,
        changed_nodes,
        true,
    )
    .await;

    info!("TEST: remove_one_node_and_change_threshold- 1 Recovery DKG");
    refresh_third_epoch_and_update_backup_members(
        num_staked_and_joined_validators,
        num_staked_only_validators,
        changed_nodes,
        false,
    )
    .await;
}

pub async fn refresh_third_epoch_and_update_backup_members(
    num_staked_and_joined_validators: i32,
    num_staked_only_validators: i32,
    changed_nodes: i32,
    register_recovery_party_again: bool,
) {
    let (testnet, validator_collection, current_epoch, second_epoch_count) =
        setup_and_second_epoch(
            num_staked_and_joined_validators,
            num_staked_only_validators,
            changed_nodes,
        )
        .await;

    // Wait for the new node to be active.
    validator_collection.actions().wait_for_active().await;

    // Re-register Backup Party
    if register_recovery_party_again {
        let party_size = num_staked_and_joined_validators.try_into().unwrap();
        let owner = &testnet.deploy_account;
        validator_collection
            .actions()
            .register_recovery_party(party_size, owner, None)
            .await;
    }

    //in test peers refresh every 1 second, so this allows time to refresh peer data after the new node is staked and active.
    // the real solution should be to either vote on locking, or not allow locking until every node has every other nodes' commkeys - though that coordination is tough, without gossiping.
    validator_collection.actions().sleep_millis(2000).await;

    // Fast forward the network by 300 seconds, and wait for the new node to be active - effectively waiting for the next epoch.
    validator_collection
        .actions()
        .increase_blockchain_timestamp(300)
        .await;

    // Wait for DKG to finish - nodes become active once more.
    validator_collection
        .actions()
        .wait_for_epoch(current_epoch + 1)
        .await;

    // Confirm that Recovery DKG has occured, and the nodes have voted for root keys
    validator_collection
        .actions()
        .wait_for_recovery_keys()
        .await;

    // Confirm that corresponding party members can download the recovery key shares
    let rpc_url = format!("http://{}", testnet.rpcurl);
    let chain_id = testnet.chain_id;
    validator_collection
        .actions()
        .download_recovery_key_shares(&rpc_url, chain_id)
        .await;

    // Confirm that Refresh DKG has occured, and the nodes have voted for root keys
    validator_collection.actions().wait_for_root_keys().await;

    // Display the final validator count.
    let final_count = validator_collection
        .actions()
        .get_current_validator_count()
        .await as u32;
    info!("Third epoch validators count: {}", final_count);
    info!("Second epoch validators count: {}", second_epoch_count);

    assert_eq!(final_count, second_epoch_count);
}

#[doc = "This test is used to test the DKG reshare process when the network peer set changes."]
pub async fn setup_and_second_epoch(
    num_staked_and_joined_validators: i32,
    num_staked_only_validators: i32,
    changed_nodes: i32,
) -> (Testnet, ValidatorCollection, U256, u32) {
    let (testnet, mut validator_collection) = new_node_collection_with_custom_config(
        num_staked_and_joined_validators as usize,
        num_staked_only_validators as usize,
        false,
        CustomNodeRuntimeConfig::default(),
    )
    .await;

    // nodes are already staked but we can double check
    for i in 0..num_staked_and_joined_validators as usize {
        let node_account = testnet.node_accounts[i].clone();

        // Fetch the node config to get the node port and network address
        let node_config_path = fetch_node_config_file_from_node_account(&node_account)
            .expect("Failed to fetch node config file");
        let node_port = Node::get_node_port_from_config_file(&node_config_path)
            .expect("Failed to get node port from config file");
        let node_addr = Node::get_node_network_addr_from_config_file(&node_config_path)
            .expect("Failed to get node network address from config file");

        info!("Node Account Ensured: {:?}", &node_account.staker_address);
        let result = validator_collection
            .actions()
            .ensure_node_staked_and_joined(&node_account, &node_addr, node_port)
            .await;
        assert!(result.is_ok());
    }

    // Register Backup Party
    let party_size = num_staked_and_joined_validators.try_into().unwrap();
    let owner = &testnet.deploy_account;
    validator_collection
        .actions()
        .register_recovery_party(party_size, owner, None)
        .await;

    // Wait for the initial epoch to end
    validator_collection
        .actions()
        .wait_for_initial_epoch()
        .await;

    // Confirm that DKG has occured, and the nodes have voted for root keys
    validator_collection.actions().wait_for_root_keys().await;

    // Display our current validator count.
    let initial_count = validator_collection
        .actions()
        .get_current_validator_count()
        .await as i32;
    info!("Initial validators count: {}", initial_count);
    let current_epoch = validator_collection.actions().get_current_epoch().await;

    let total_num_validators = num_staked_and_joined_validators + num_staked_only_validators;
    let last_staked_validator_idx = total_num_validators - 1;
    if changed_nodes > 0 {
        // Add a new node to the network.
        info!("Increasing network size");
        info!("Node accounts: {:?}", testnet.node_accounts);

        let node_account = testnet.node_accounts[last_staked_validator_idx as usize].clone();

        // Fetch the node config to get the node port and network address
        let node_config_path = fetch_node_config_file_from_node_account(&node_account)
            .expect("Failed to fetch node config file");
        let node_port = Node::get_node_port_from_config_file(&node_config_path)
            .expect("Failed to get node port from config file");
        let node_addr = Node::get_node_network_addr_from_config_file(&node_config_path)
            .expect("Failed to get node network address from config file");

        info!(
            "Node Account Ensured: {:?}",
            &node_account.clone().staker_address
        );
        let result = validator_collection
            .actions()
            .ensure_node_staked_and_joined(&node_account, &node_addr, node_port)
            .await;
        if let Err(e) = result {
            info!("Error: {:?}", e);
            panic!("{}", e);
        }

        // Instantly try to start the new node process.
        assert!(validator_collection.add_one(false, None).await.is_ok());
    } else if changed_nodes < 0 {
        // Remove a node from the network.
        info!("Decreasing network size");
        let node_account = testnet.node_accounts[(last_staked_validator_idx - 1) as usize].clone();
        info!("Node Account Ensured: {:?}", &node_account.staker_address);
        let result = validator_collection
            .actions()
            .ensure_node_unstaked(node_account)
            .await;
        if let Err(e) = result {
            info!("Error: {:?}", e);
            panic!("{}", e);
        }
    }

    // Wait for the new node to be active.
    validator_collection.actions().wait_for_active().await;

    //in test peers refresh every 1 second, so this allows time to refresh peer data after the new node is staked and active.
    // the real solution should be to either vote on locking, or not allow locking until every node has every other nodes' commkeys - though that coordination is tough, without gossiping.
    validator_collection.actions().sleep_millis(2000).await;

    // Fast forward the network by 300 seconds, and wait for the new node to be active - effectively waiting for the next epoch.
    validator_collection
        .actions()
        .increase_blockchain_timestamp(300)
        .await;

    // Lock the validators for the next epoch;
    // validator_collection.actions().lock_validators_for_next_epoch().await;

    // Wait for DKG to finish - nodes become active once more.
    validator_collection
        .actions()
        .wait_for_epoch(current_epoch + 1)
        .await;

    // Confirm that Recovery DKG has occured, and the nodes have voted for root keys
    validator_collection
        .actions()
        .wait_for_recovery_keys()
        .await;

    // Confirm that corresponding party members can download the recovery key shares
    let rpc_url = format!("http://{}", testnet.rpcurl);
    let chain_id = testnet.chain_id;
    validator_collection
        .actions()
        .download_recovery_key_shares(&rpc_url, chain_id)
        .await;

    // Confirm that Reshare DKG has occured, and the nodes have voted for root keys
    validator_collection.actions().wait_for_root_keys().await;

    // Display the final validator count.
    let final_count = validator_collection
        .actions()
        .get_current_validator_count()
        .await as u32;
    let second_epoch_count = (initial_count + changed_nodes) as u32;

    info!("Second validators count: {}", final_count);
    info!(
        "Initial validators count: {} + changed nodes count {} = {}",
        initial_count, changed_nodes, second_epoch_count
    );

    assert_eq!(final_count, second_epoch_count);

    (
        testnet,
        validator_collection,
        current_epoch + 1,
        final_count,
    )
}
