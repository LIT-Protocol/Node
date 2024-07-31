pub mod assertions;
pub mod auth_sig;
pub mod config;
pub mod faults;
pub mod interpolation;
pub mod validator;

#[cfg(feature = "lit-actions")]
pub mod lit_actions;
pub mod models;
pub mod networking;
pub mod node_collection;
pub mod pkp;
pub mod rand;
pub mod recovery_party;
pub mod session_sigs;
pub mod testnet;
pub mod version;

use lit_api_core::logging::TracingPlugin;
use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_logging::plugin::log_service::LogServicePlugin;
use lit_node::config::load_cfg;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use tracing::info;

use self::testnet::contracts::StakingContractConfig;
use self::testnet::node_config::CustomNodeRuntimeConfig;
use self::testnet::Testnet;
use self::validator::ValidatorCollection;

pub async fn new_node_collection(
    num_staked_and_joined_validators: usize,
    is_fault_test: bool,
) -> (Testnet, ValidatorCollection) {
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(num_staked_and_joined_validators)
        .is_fault_test(is_fault_test)
        .build()
        .await;

    let testnet_contracts = Testnet::setup_contracts(&mut testnet, None)
        .await
        .expect("Failed to setup contracts");

    let actions = testnet.actions(testnet_contracts.contracts());

    let validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(num_staked_and_joined_validators)
        .build(&testnet, &actions)
        .await
        .expect("Failed to build validator collection");

    (testnet, validator_collection)
}

pub async fn new_node_collection_with_custom_config(
    num_staked_and_joined_validators: usize,
    num_staked_only_validators: usize,
    is_fault_test: bool,
    custom_config: CustomNodeRuntimeConfig,
) -> (Testnet, ValidatorCollection) {
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(num_staked_and_joined_validators)
        .num_staked_only_validators(num_staked_only_validators)
        .is_fault_test(is_fault_test)
        .custom_node_runtime_config(custom_config)
        .build()
        .await;

    let testnet_contracts = Testnet::setup_contracts(&mut testnet, None)
        .await
        .expect("Failed to setup contracts");

    let actions = testnet.actions(testnet_contracts.contracts());

    let validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(num_staked_and_joined_validators)
        .build(&testnet, &actions)
        .await
        .expect("Failed to build validator collection");

    (testnet, validator_collection)
}

pub async fn new_node_collection_with_contract_config(
    num_staked_and_joined_validators: usize,
    is_fault_test: bool,
    staking_contract_initial_config: Option<StakingContractConfig>,
) -> (Testnet, ValidatorCollection) {
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(num_staked_and_joined_validators)
        .is_fault_test(is_fault_test)
        .build()
        .await;

    let testnet_contracts = Testnet::setup_contracts(&mut testnet, staking_contract_initial_config)
        .await
        .expect("Failed to setup contracts");

    let actions = testnet.actions(testnet_contracts.contracts());

    let validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(num_staked_and_joined_validators)
        .build(&testnet, &actions)
        .await
        .expect("Failed to build validator collection");

    (testnet, validator_collection)
}

static LOGGING_SETUP: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
#[doc = "Setup configuration as node #0 and logging for tests"]
pub fn setup_logging() {
    if let Ok(mut lock) = LOGGING_SETUP.lock() {
        if *lock {
            return;
        }
        *lock = true;

        info!("Setting up logging for tests");
        std::env::set_var(ENV_LIT_CONFIG_FILE, "./tests/lit_logging_config.toml");
        let cfg = load_cfg().expect("failed to load LitConfig");

        // special prefix for testing
        lit_logging::builder(cfg.load().as_ref(), "TEST")
            //lit_logging::builder(&cfg, "TEST")
            .plugin(TracingPlugin::new())
            .plugin(LogServicePlugin::new())
            // .add_field("TEST", json!(""))
            .init()
            .expect("failed to init logging");
    }
}

// load environment vars
#[doc = "Setup configuration as node #0 and logging for tests"]
pub fn init_test_config() {
    setup_logging();

    let clean_folders = false;
    if clean_folders {
        remove_test_dir("./node_keys/");
        remove_test_dir("./config/test/component/");
    }
}

pub fn remove_test_dir(testdir: &str) {
    let path = format!("{}", testdir);
    if let Err(e) = std::fs::remove_dir_all(path.clone()) {
        tracing::error!("Failed to remove directory: {} with error {}", path, e);
    }
}
