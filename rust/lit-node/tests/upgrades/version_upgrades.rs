use crate::common::{
    assertions::NetworkIntegrityChecker,
    init_test_config,
    node_collection::{get_node_versions, NodeCollection},
    testnet::{
        contracts::StakingContractInitialConfig,
        contracts_repo::{
            self, alias_node_configs_path, get_alias_manifest_template, latest_wallet_manifest,
            save_alias_manifest, WalletManifestItem,
        },
        scenario::Scenario,
    },
};
use ethers::types::U256;
use lit_core::utils::binary::bytes_to_hex;
use rand::seq::SliceRandom;
use std::{fs, process::Command};
use tracing::info;

fn setup() {
    init_test_config();
}

#[tokio::test]
async fn test_version_upgrade() {
    setup();

    info!("TEST: test_version_upgrade");

    // set epoch length to 30 mins so it never elapses unless we advance the clock
    let epoch_length = 1800;

    // Start a new node collection and wait for the DKG to complete
    // and root keys to be voted for.
    let initial_port = 7470;
    let num_nodes = 10;
    let s = Scenario::builder()
        .num_staked(num_nodes)
        .num_nodes(num_nodes)
        .num_awake_initially(num_nodes)
        .initial_port(initial_port)
        .staking_contract_initial_config(
            StakingContractInitialConfig::builder()
                .epoch_length(U256::from(epoch_length))
                .build(),
        )
        .force_deploy_in_ci(true)
        .build()
        .await;
    let (mut node_collection, scenario) = NodeCollection::from_scenario(s).await;

    let starting_epoch = scenario.actions.get_current_epoch().await;
    let epoch = scenario.contracts.staking.epoch().await.unwrap();
    assert!(
        epoch.epoch_length == U256::from(epoch_length),
        "We set the epoch length to {} but it's coming back from the chain as {}",
        epoch_length,
        epoch.epoch_length
    );

    // Keep track of the node versions.
    let initial_node_versions = get_node_versions(&scenario).await;
    info!("Initial node versions: {:?}", initial_node_versions);
    // Assert all node versions are the same.
    assert!(initial_node_versions
        .iter()
        .all(|v| v == &initial_node_versions[0]));

    let network_checker = NetworkIntegrityChecker::new(&scenario).await;

    // Prepare manifest and run script to generate and add new alias wallet.
    let alias_node_port = node_collection
        .portnames
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        + 1;
    let existing_wallet_with_alias = generate_wallet_and_add_as_alias(alias_node_port).await;

    // Upgrade the node crate.
    let crate_version_handle = update_node_crate_version("2.9999.9999".to_string());
    // Assert that it is same as returned from handshake endpoint above.
    assert_eq!(crate_version_handle.0, initial_node_versions[0]);

    // Spin up a new node with the new version and the alias wallet.
    let alias_node_config_path = format!("{}/alias_lit_config0.toml", alias_node_configs_path());
    node_collection
        .extend(&scenario, "7480".into(), alias_node_config_path, 10, true)
        .await;

    // Now that the new node with the alias wallet is up, we should request to join the network
    // using the alias wallet.
    request_to_join_with_alias(&scenario, alias_node_port);

    // Fast forward time to allow nodes to start a DKG to advance to the next epoch.
    scenario
        .actions
        .increase_blockchain_timestamp(epoch_length)
        .await;

    // After next epoch arrives, run interpolation and decryption tests.
    scenario.actions.wait_for_epoch(starting_epoch + 1).await;
    network_checker.check(&node_collection, &scenario).await;

    // Assert node versions via the handshake endpoint.
    // First num_nodes should be the same as initial_node_versions, and the last one should be equal to 2.9999.9999.
    let node_versions = get_node_versions(&scenario).await;
    info!(
        "node versions ({:?}) {:?} and initial node versions {:?}",
        node_versions.len(),
        node_versions,
        initial_node_versions
    );
    assert_eq!(node_versions.len(), num_nodes + 1);
    for (i, version) in node_versions.iter().enumerate() {
        if i < num_nodes {
            assert_eq!(version, &initial_node_versions[0]);
        } else {
            assert_eq!(version, "2.9999.9999");
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
            bytes_to_hex(scenario.contracts.staking.address().as_bytes())
        ),
    );

    // Fast forward time to allow nodes to start a DKG to advance to the next epoch.
    scenario
        .actions
        .increase_blockchain_timestamp(epoch_length)
        .await;

    // After next epoch arrives, kill node with old version and run network tests.
    scenario.actions.wait_for_epoch(starting_epoch + 2).await;
    network_checker.check(&node_collection, &scenario).await;

    // Kill the node with the old staker wallet.
    node_collection.shutdown_node(existing_wallet_with_alias.idx);

    network_checker.check(&node_collection, &scenario).await;
}

/// Grabs the first wallet from the latest generated alias wallet manifest and uses that to send a
/// transaction to request to join the validator network.
fn request_to_join_with_alias(scenario: &Scenario, alias_node_port: usize) {
    let alias_wallet = &latest_wallet_manifest(true)[0];

    contracts_repo::request_to_join(
        alias_wallet.staker.private_key.clone(),
        format!(
            "0x{}",
            bytes_to_hex(scenario.contracts.staking.address().as_bytes())
        ),
        format!(
            "0x{}",
            bytes_to_hex(scenario.contracts.staking_balances.address().as_bytes())
        ),
        "127.0.0.1".into(),
        alias_node_port.to_string(),
        alias_wallet.node.address.clone(),
        alias_wallet.node.coms_keys_sender.public_key.clone(),
        alias_wallet.node.coms_keys_receiver.public_key.clone(),
    );
}

/// Returns the wallet manifest item that we had added an alias for.
async fn generate_wallet_and_add_as_alias(alias_node_port: usize) -> WalletManifestItem {
    let wallet_manifest_wallets = latest_wallet_manifest(false);

    // Choose a random item in the wallet manifest.
    let random_item = wallet_manifest_wallets
        .choose(&mut rand::thread_rng())
        .unwrap();
    info!(
        "Using random wallet from manifest to add an alias for: {:?}",
        random_item
    );

    // Generate a new alias manifest by copying from the template and adjusting the values.
    let mut parsed_alias_manifest_template = get_alias_manifest_template();
    info!("Using {:?} as the alias node port", alias_node_port);
    parsed_alias_manifest_template.alias_port = alias_node_port;
    parsed_alias_manifest_template.existing_staker_wallet_private_key =
        random_item.staker.private_key.clone();
    parsed_alias_manifest_template.node_custom_runtime_config_path =
        Some("../../rust/lit-node/config/test/custom_node_runtime_config.toml".into());

    // Write to file.
    save_alias_manifest(&parsed_alias_manifest_template);

    // Now that we have the alias manifest ready, we can run the script.
    contracts_repo::generate_wallet_and_add_as_alias();

    random_item.to_owned()
}

fn update_node_crate_version(new_crate_version: String) -> CrateVersionHandle {
    // First get the current crate version. Do this by running the command `cargo pkgid` and parsing
    // the characters at the end after the `@`.
    let cmd = Command::new("cargo")
        .args(["pkgid"])
        .output()
        .expect("Failed to get current crate version");
    assert!(cmd.status.success());
    let current_crate_version = String::from_utf8(cmd.stdout)
        .unwrap()
        .split('@')
        .last()
        .unwrap()
        .trim()
        .to_string();
    info!(
        "Current crate version before updating: {:?}",
        current_crate_version
    );

    // Update the crate version.
    let cargo_toml = fs::read_to_string("./Cargo.toml").expect("Failed to read Cargo.toml");
    let mut doc = cargo_toml
        .parse::<toml_edit::Document>()
        .expect("Failed to parse Cargo.toml");
    doc["package"]["version"] = toml_edit::value(new_crate_version.clone());
    fs::write("./Cargo.toml", doc.to_string()).expect("Failed to write Cargo.toml");
    info!("Updated node crate version to {}", new_crate_version);

    CrateVersionHandle(current_crate_version)
}

/// A simple struct that stores a version, and updates the crate to this version when this
/// struct is dropped.
struct CrateVersionHandle(String);

impl Drop for CrateVersionHandle {
    fn drop(&mut self) {
        info!("Reverting crate version back to {:?}", self.0);

        // Update the crate version.
        let cargo_toml = fs::read_to_string("./Cargo.toml").expect("Failed to read Cargo.toml");
        let mut doc = cargo_toml
            .parse::<toml_edit::Document>()
            .expect("Failed to parse Cargo.toml");
        doc["package"]["version"] = toml_edit::value(self.0.clone());
        fs::write("./Cargo.toml", doc.to_string()).expect("Failed to write Cargo.toml");
        info!("Updated node crate version to {}", self.0);
    }
}
