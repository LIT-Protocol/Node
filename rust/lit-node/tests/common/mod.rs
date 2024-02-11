pub mod assertions;
pub mod auth_sig;
pub mod config;
pub mod faults;
pub mod interpolation;

#[cfg(feature = "lit-actions")]
pub mod lit_actions;
pub mod models;
pub mod networking;
pub mod node_collection;
pub mod pkp;
pub mod testnet;

use lit_api_core::logging::TracingPlugin;
use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_logging::plugin::log_service::LogServicePlugin;
use lit_node::config::load_cfg;
use node_collection::NodeCollection;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use testnet::scenario::{InitialDKGState, Scenario};

use tracing::info;

use self::testnet::contracts::StakingContractInitialConfig;
use self::testnet::node_config::CustomNodeRuntimeConfig;

pub async fn new_node_collection(
    num_staked: usize,
    num_nodes: usize,
    initial_dkg_state: Option<InitialDKGState>,
    wait_for_dkg_to_complete: bool,
    is_fault_test: bool,
) -> (NodeCollection, Scenario) {
    let initial_port = 7470;

    let s = Scenario::builder()
        .num_staked(num_staked)
        .num_nodes(num_nodes)
        .num_awake_initially(num_nodes)
        .initial_port(initial_port)
        .initial_dkg_state(initial_dkg_state.unwrap_or(InitialDKGState::ForceStart))
        .wait_for_dkg_to_complete(wait_for_dkg_to_complete)
        .is_fault_test(is_fault_test)
        .build()
        .await;
    NodeCollection::from_scenario(s).await
}

pub async fn new_node_collection_with_custom_config(
    num_staked: usize,
    num_nodes: usize,
    initial_dkg_state: Option<InitialDKGState>,
    wait_for_dkg_to_complete: bool,
    is_fault_test: bool,
    custom_config: CustomNodeRuntimeConfig,
) -> (NodeCollection, Scenario) {
    let initial_port = 7470;

    let s = Scenario::builder()
        .num_staked(num_staked)
        .num_nodes(num_nodes)
        .num_awake_initially(num_nodes)
        .initial_port(initial_port)
        .initial_dkg_state(initial_dkg_state.unwrap_or(InitialDKGState::ForceStart))
        .wait_for_dkg_to_complete(wait_for_dkg_to_complete)
        .is_fault_test(is_fault_test)
        .custom_node_runtime_config(custom_config)
        .build()
        .await;
    NodeCollection::from_scenario(s).await
}

pub async fn new_node_collection_with_contract_config(
    num_staked: usize,
    num_nodes: usize,
    initial_dkg_state: Option<InitialDKGState>,
    wait_for_dkg_to_complete: bool,
    is_fault_test: bool,
    staking_contract_initial_config: Option<StakingContractInitialConfig>,
) -> (NodeCollection, Scenario) {
    let initial_port = 7470;

    let mut scenario_builder = Scenario::builder()
        .num_staked(num_staked)
        .num_nodes(num_nodes)
        .num_awake_initially(num_nodes)
        .initial_port(initial_port)
        .initial_dkg_state(initial_dkg_state.unwrap_or(InitialDKGState::ForceStart))
        .wait_for_dkg_to_complete(wait_for_dkg_to_complete)
        .is_fault_test(is_fault_test);

    if let Some(staking_contract_initial_config) = staking_contract_initial_config {
        scenario_builder =
            scenario_builder.staking_contract_initial_config(staking_contract_initial_config);
    }

    let s = scenario_builder.build().await;
    NodeCollection::from_scenario(s).await
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
}
