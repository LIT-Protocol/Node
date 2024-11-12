use ethers::types::H160;
use lit_node::{peers::peer_reviewer::Issue, utils::consensus::get_threshold_count};
use test_common::{
    assertions::NetworkIntegrityChecker,
    init_test_config,
    node_collection::get_node_versions,
    testnet::{
        contracts::StakingContractConfig,
        contracts_repo::{
            self, alias_node_configs_path, get_alias_manifest_template, latest_wallet_manifest,
            save_alias_manifest, WalletManifestItem,
        },
        NodeAccount, Testnet,
    },
    validator::ValidatorCollection,
    version::{get_crate_version, update_node_crate_version},
};

// use anyhow::{Ok, Result};
use ethers::types::U256;
use lit_blockchain::{
    contracts::staking::ComplaintConfig,
    resolver::rpc::{RpcHealthcheckPoller, ENDPOINT_MANAGER},
};
use lit_core::utils::binary::bytes_to_hex;
use rand::seq::SliceRandom;
use std::{fs, process::Command, time::Duration};
use test_case::test_case;
use tracing::info;

fn setup() {
    init_test_config();
}

/// Tests when an inactive validator that comes online with an invalid version, and then the staker requests to join,
/// that the node should eventually be kicked for non-participation.
#[tokio::test]
async fn node_boot_invalid_version() {
    setup();

    info!("TEST: node_boot_invalid_version");

    // Set up a network with 6 nodes.
    let num_nodes = 6;
    // set epoch length to 30 mins so it never elapses unless we advance the clock
    let epoch_length = 1800;
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(num_nodes)
        .num_staked_only_validators(1)
        .build()
        .await;

    let testnet_contracts = Testnet::setup_contracts(
        &mut testnet,
        Some(
            StakingContractConfig::builder()
                .epoch_length(U256::from(epoch_length))
                .max_triple_count(U256::from(0))
                .min_triple_count(U256::from(0))
                .build(),
        ),
    )
    .await
    .expect("Failed to setup contracts");

    let actions = testnet.actions(testnet_contracts.contracts());

    let mut validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(num_nodes)
        .build(&testnet, &actions)
        .await
        .expect("Failed to build validator collection");

    let network_checker = NetworkIntegrityChecker::new(&actions).await;

    // Upgrade the node crate to a new version
    let _crate_version_handle = update_node_crate_version("2.9999.9999".to_string());

    // Update version requirements by setting a max version requirement, rendering the new node version invalid.
    let max_version = "2.9999.9998";
    actions
        .set_staking_max_version(max_version)
        .await
        .expect("Failed to set max version");

    // Lower the configured threshold for non-participation complaints.
    info!("Lowering the configured threshold for non-participation complaints");
    actions
        .set_complaint_reason_config(
            U256::from(Issue::NonParticipation.value()),
            ComplaintConfig {
                tolerance: U256::from(2),
                interval_secs: U256::from(120),
                kick_penalty_percent: U256::from(10),
            },
        )
        .await
        .expect("Failed to set complaint config");

    // Spin up a new node with the new node version
    info!("Spinning up a new node with the new node version");
    let validator_to_kick = validator_collection
        .add_one(false, Some(test_common::validator::BuildMode::UseNewBuild))
        .await
        .expect("Failed to add new node");
    let staker_address_to_kick = validator_to_kick.account().staker_address;

    // Fast forward time to allow the network to attempt to deal in the new node with the new node version
    // before voting to kick it out due to non-participation.
    info!("Fast forwarding time to allow the network to attempt to deal in the new node with the new node version");
    actions.increase_blockchain_timestamp(epoch_length).await;

    // Wait for kick
    let voting_status = validator_collection
        .actions()
        .wait_for_voting_status_to_kick_validator(
            U256::from(2),
            staker_address_to_kick,
            H160::random(), // For simplicity, we only care about asserting the number of votes.
            get_threshold_count(num_nodes),
        )
        .await;
    assert!(voting_status.is_ok());

    // Wait for new epoch
    info!("Waiting for epoch 3");
    actions.wait_for_epoch(U256::from(3)).await;

    // Run network checks
    info!("Checking network state");
    assert_eq!(
        actions.get_current_validator_count().await as usize,
        num_nodes
    );
    network_checker.check(&validator_collection).await;
}

/// Tests the version requirement change such that an active validator is running a node version that is incompatible,
/// so it should request to leave.
#[tokio::test]
async fn active_validator_invalid_version() {
    setup();

    info!("TEST: active_validator_invalid_version");

    // Set up a network with 6 nodes.
    let num_nodes = 6;
    // set epoch length to 30 mins so it never elapses unless we advance the clock
    let epoch_length = 1800;
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(num_nodes)
        .num_staked_only_validators(1)
        .build()
        .await;

    let testnet_contracts = Testnet::setup_contracts(
        &mut testnet,
        Some(
            StakingContractConfig::builder()
                .epoch_length(U256::from(epoch_length))
                .max_triple_count(U256::from(0))
                .min_triple_count(U256::from(0))
                .build(),
        ),
    )
    .await
    .expect("Failed to setup contracts");

    let actions = testnet.actions(testnet_contracts.contracts());

    let mut validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(num_nodes)
        .build(&testnet, &actions)
        .await
        .expect("Failed to build validator collection");

    let network_checker = NetworkIntegrityChecker::new(&actions).await;

    // Upgrade the node crate to a new version
    let _crate_version_handle = update_node_crate_version("2.9999.9999".to_string());

    // Spin up a new node with the new node version
    info!("Spinning up a new node with the new node version");
    let new_validator = validator_collection
        .add_one(false, Some(test_common::validator::BuildMode::UseNewBuild))
        .await
        .expect("Failed to add new node");
    let new_validator_staker_address = new_validator.account().staker_address;

    // Fast forward time to allow the network to deal in the new node with the new node version
    info!("Fast forwarding time to allow the network to deal in the new node with the new node version");
    actions.increase_blockchain_timestamp(epoch_length).await;

    // Wait for the new epoch
    info!("Waiting for epoch 3");
    actions.wait_for_epoch(U256::from(3)).await;

    // Run network checks
    info!("Checking network state");
    assert_eq!(
        actions.get_current_validator_count().await as usize,
        num_nodes + 1
    );
    network_checker.check(&validator_collection).await;

    // Update version requirements by setting a max version requirement, rendering the new node version invalid.
    let max_version = "2.9999.9998";
    actions
        .set_staking_max_version(max_version)
        .await
        .expect("Failed to set max version");

    // After some time, fast forward to allow the network to deal out the new node with the new node version.
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    info!("Fast forwarding time to allow the network to deal out the new node with the new node version");
    actions.increase_blockchain_timestamp(epoch_length).await;

    // Wait for the new epoch
    info!("Waiting for epoch 4");
    actions.wait_for_epoch(U256::from(4)).await;

    // Run network checks
    info!("Checking network state");
    assert_eq!(
        actions.get_current_validator_count().await as usize,
        num_nodes
    );
    network_checker.check(&validator_collection).await;

    // Check that the new node is no longer a validator.
    let active_validators = actions.get_current_validators().await;
    assert!(!active_validators.contains(&new_validator_staker_address));
}

/// This test assumes that you have the lit_node builds for the target branches.
/// During local development, there are two ways to get the builds:
/// 1. Run the `build_target_branches` script in the `scripts` directory. (x86 and arm64 builds)
/// 2. Run the `download_builds` script in the `scripts` directory. (x86 builds only)
/// The test will fail if the builds are not found.
#[test_case("origin/release-habanero-*"; "Upgrade against the latest Habanero release branch")]
#[test_case("origin/release-manzano-*"; "Upgrade against the latest Manzano release branch")]
#[test_case("origin/release-cayenne-*"; "Upgrade against the latest Cayenne release branch")]
#[tokio::test]
async fn test_version_upgrade_against_old_version(target_branch: &str) {
    setup();

    info!(
        "TEST: test_version_upgrade_against_old_version against {}",
        target_branch
    );

    // Get the commit hash that we want the build for.
    let old_build_commit_hash =
        utils::get_target_branch_commit_hash(target_branch).expect("Failed to get commit hash");

    // First check if we have the build.
    let old_build_path = format!("./target/debug/lit_node_{}", old_build_commit_hash);
    assert!(
        fs::metadata(&old_build_path).is_ok(),
        "Build does not exist at {}",
        old_build_path
    );

    // Set up a network of nodes running the old build.

    // set epoch length to 30 mins so it never elapses unless we advance the clock
    let epoch_length = 1800;

    // Start a new node collection and wait for the DKG to complete
    // and root keys to be voted for.
    let num_nodes = 3;
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(num_nodes)
        .force_deploy_in_ci(true)
        .build()
        .await;

    let testnet_contracts = Testnet::setup_contracts(
        &mut testnet,
        Some(
            StakingContractConfig::builder()
                .epoch_length(U256::from(epoch_length))
                .min_triple_count(U256::from(0))
                .max_triple_count(U256::from(0))
                .max_triple_concurrency(U256::from(0))
                .build(),
        ),
    )
    .await
    .expect("Failed to setup contracts");

    let actions = testnet.actions(testnet_contracts.contracts());

    let mut validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(num_nodes)
        .custom_binary_path(Some(old_build_path))
        .build(&testnet, &actions)
        .await
        .expect("Failed to build validator collection");

    let starting_epoch = validator_collection.actions().get_current_epoch().await;
    let mut next_epoch = starting_epoch + 1;

    // Keep track of the node versions.
    let initial_node_versions = get_node_versions(validator_collection.actions()).await;
    info!("Initial node versions: {:?}", initial_node_versions);
    // Assert all node versions are the same.
    assert!(initial_node_versions
        .iter()
        .all(|v| v == &initial_node_versions[0]));

    let network_checker = NetworkIntegrityChecker::new(validator_collection.actions()).await;
    network_checker
        .check_with_drained_beaver_triples(&validator_collection)
        .await;

    // First, we shuffle the order of the original staker wallets that we will be gradually adding aliases for.
    let mut wallet_manifest_wallets = latest_wallet_manifest(false);
    wallet_manifest_wallets.shuffle(&mut rand::thread_rng());

    // Keep dealing in new node versions and dealing out old node versions until the entire network is upgraded.
    for upgrade_round in 0..num_nodes {
        info!("Upgrading node {} to the new build", upgrade_round);

        // Prepare manifest and run script to generate and add new alias wallet.
        let alias_node_port = validator_collection.max_port() + 1;
        let existing_wallet_to_add_alias_for = wallet_manifest_wallets[upgrade_round].to_owned();
        generate_wallet_and_add_as_alias(&existing_wallet_to_add_alias_for, alias_node_port).await;
        let existing_wallet_with_alias = existing_wallet_to_add_alias_for;

        // Spin up a new node with the new version and the alias wallet.
        let alias_node_config_path =
            format!("{}/alias_lit_config0.toml", alias_node_configs_path());
        assert!(validator_collection
            .add_one_custom(
                false,
                alias_node_config_path,
                &get_latest_alias_node_account(0, &testnet),
                Some(test_common::validator::BuildMode::UseNewBuild)
            )
            .await
            .is_ok());

        // Fast forward time to allow nodes to start a DKG to advance to the next epoch.
        validator_collection
            .actions()
            .increase_blockchain_timestamp(epoch_length)
            .await;

        // After next epoch arrives, run interpolation and decryption tests.
        validator_collection
            .actions()
            .wait_for_epoch(next_epoch)
            .await;
        next_epoch += U256::from(1);

        validator_collection.actions().sleep_millis(2000).await; // FIXME : let the nodes all acknowledge the epoch, then  run the tests.   This should be removed once signing across epochs works.
        network_checker
            .check_with_drained_beaver_triples(&validator_collection)
            .await;

        // Assert node versions.
        let mut node_versions = get_node_versions(validator_collection.actions()).await;
        // Sort the node versions to make it easier to compare.
        node_versions.sort();
        info!(
            "node versions ({:?}) {:?} and initial node versions {:?}",
            node_versions.len(),
            node_versions,
            initial_node_versions
        );
        assert_eq!(node_versions.len(), num_nodes + 1);

        // Get current crate version.
        let current_crate_version = get_crate_version();
        for (i, version) in node_versions.iter().enumerate() {
            if i < (num_nodes - upgrade_round) {
                assert_eq!(version, &initial_node_versions[0]);
            } else {
                assert_eq!(version.to_owned(), current_crate_version);
            }
        }

        // The old staker wallet request to leave the network.
        info!(
            "Requesting to leave the network for staker {:?}",
            existing_wallet_with_alias.staker.address
        );
        contracts_repo::request_to_leave(
            &existing_wallet_with_alias.staker.private_key,
            &format!(
                "0x{}",
                bytes_to_hex(
                    validator_collection
                        .actions()
                        .contracts()
                        .staking
                        .address()
                        .as_bytes()
                )
            ),
        );

        // Fast forward time to allow nodes to start a DKG to advance to the next epoch.
        validator_collection
            .actions()
            .increase_blockchain_timestamp(epoch_length)
            .await;

        // After next epoch arrives, kill node with old version and run network tests.
        validator_collection
            .actions()
            .wait_for_epoch(next_epoch)
            .await;
        next_epoch += U256::from(1);
        network_checker
            .check_with_drained_beaver_triples(&validator_collection)
            .await;

        // Kill the node with the old staker wallet.
        assert!(validator_collection
            .stop_node(existing_wallet_with_alias.idx)
            .await
            .is_ok());

        network_checker
            .check_with_drained_beaver_triples(&validator_collection)
            .await;
    }
}

fn get_latest_alias_node_account(idx: usize, testnet: &Testnet) -> NodeAccount {
    let latest_alias_wallet_manifest = latest_wallet_manifest(true);
    let mut provider = ENDPOINT_MANAGER
        .get_provider(testnet.chain_name.clone())
        .expect("Failed to get provider");
    provider.set_interval(Duration::new(0, 10));
    latest_alias_wallet_manifest[idx].map_to_node_account(provider, testnet.chain_id)
}

/// Returns the wallet manifest item that we had added an alias for.
async fn generate_wallet_and_add_as_alias(
    existing_wallet_manifest_item: &WalletManifestItem,
    alias_node_port: usize,
) {
    info!(
        "Using random wallet from manifest to add an alias for: {:?}",
        existing_wallet_manifest_item
    );

    // Generate a new alias manifest by copying from the template and adjusting the values.
    let mut parsed_alias_manifest_template = get_alias_manifest_template();
    info!("Using {:?} as the alias node port", alias_node_port);
    parsed_alias_manifest_template.alias_port = alias_node_port;
    parsed_alias_manifest_template.existing_staker_wallet_private_key =
        existing_wallet_manifest_item.staker.private_key.clone();
    parsed_alias_manifest_template.node_config_ipfs_api_key = std::env::var("IPFS_API_KEY")
        .expect("IPFS_API_KEY not set")
        .to_owned();

    // Write to file.
    save_alias_manifest(&parsed_alias_manifest_template);

    // Now that we have the alias manifest ready, we can run the script.
    contracts_repo::generate_wallet_and_add_as_alias();
}
