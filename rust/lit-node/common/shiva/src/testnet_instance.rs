use std::process::{Child, Command};

use anyhow::anyhow;
use ethers::types::U256;
use tracing::info;

use test_common::validator::ValidatorCollection;

use crate::models::{ContractAbis, ContractAddresses, TestNetCreateParams};

use test_common::testnet::contracts::StakingContractConfig;
use test_common::testnet::node_config::CustomNodeRuntimeConfig;
use test_common::testnet::Testnet;
use test_common::testnet::{actions, TestnetContracts};

// Custom impl to avoid `From<T>` trait as it requires borrowing which we do not want as we cannot brrow from the runtime context
impl ContractAbis {
    pub fn new(contracts: &TestnetContracts) -> Result<Self, anyhow::Error> {
        let lit_token = serde_json::to_string(contracts.contracts().lit_token.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let erc20 = serde_json::to_string(contracts.contracts().erc20.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let backup_recovery =
            serde_json::to_string(contracts.contracts().backup_recovery.abi()).unwrap();
        let staking = serde_json::to_string(contracts.contracts().staking.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let staking_balances = serde_json::to_string(contracts.contracts().staking_balances.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let rate_limit_nft = serde_json::to_string(contracts.contracts().rate_limit_nft.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let pkpnft = serde_json::to_string(contracts.contracts().pkpnft.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let pubkey_router = serde_json::to_string(contracts.contracts().pubkey_router.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let pkp_helper = serde_json::to_string(contracts.contracts().pkp_helper.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let pkp_permissions = serde_json::to_string(contracts.contracts().pkp_permissions.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;

        let contract_resolver =
            serde_json::to_string(contracts.contracts().contract_resolver.abi())
                .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let payment_delegation =
            serde_json::to_string(contracts.contracts().payment_delegation.abi())
                .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;

        Ok(Self {
            lit_token,
            erc20,
            backup_recovery,
            staking,
            staking_balances,
            rate_limit_nft,
            pkpnft,
            pkp_helper,
            pkp_permissions,
            pubkey_router,
            contract_resolver,
            payment_delegation,
        })
    }
}

impl ContractAddresses {
    pub fn new(addresses: &test_common::testnet::contracts::ContractAddresses) -> Self {
        Self {
            lit_token: format!("{:#x}", addresses.lit_token),
            backup_recovery: format!("{:#x}", addresses.backup_recovery),
            staking: format!("{:#x}", addresses.staking),
            staking_balances: format!("{:#x}", addresses.staking_balances),
            rate_limit_nft: format!("{:#x}", addresses.rate_limit_nft),
            pkpnft: format!("{:#x}", addresses.pkpnft),
            pubkey_router: format!("{:#x}", addresses.pubkey_router),
            pkp_permissions: format!("{:#x}", addresses.pkp_permissions),
            pkp_helper: format!("{:#x}", addresses.pkp_helper),
            contract_resolver: format!("{:#x}", addresses.contract_resolver),
            key_deriver: format!("{:#x}", addresses.key_deriver),
            payment_delegation: format!("{:#x}", addresses.payment_delegation),
        }
    }
}

pub struct TestnetInsance {
    pub node_count: usize,
    pub polling_interval: String,
    pub epoch_length: i32,
    pub exisiting_config_path: Option<String>,
    pub ecdsa_round_timeout: Option<String>,
    pub enable_rate_limiting: Option<String>,
    pub action_server: Option<Child>,
    pub actions: actions::Actions,
    pub test_net: Testnet,
    pub contracts: test_common::testnet::TestnetContracts,
    pub validators: test_common::validator::ValidatorCollection,
}

impl TestnetInsance {
    pub async fn init(params: TestNetCreateParams) -> Result<Self, anyhow::Error> {
        let mut lit_action_process: Option<Child> = None;
        if params.custom_build_path.is_some()
            && params.lit_action_server_custom_build_path.is_some()
        {
            let lit_action_path = match params.clone().lit_action_server_custom_build_path {
                Some(path) => path,
                None => {
                    // TODO add way of signaling to parent we have experienced a startup error and we should not continue with setup.
                    return Err(anyhow::anyhow!("Can not specify one prebuilt binary, must provide both lit action and lit node"));
                }
            };

            info!("Spawning Lit Action server at path: {}", lit_action_path);

            let lit_action_server = Command::new(lit_action_path)
                .spawn()
                .map_err(|e| anyhow::anyhow!("Error while spawning lit action server: {}", e))?;
            lit_action_process = Some(lit_action_server);
        }

        let mut testnet = Testnet::builder()
            .num_staked_and_joined_validators(params.node_count.clone())
            .custom_node_runtime_config(
                CustomNodeRuntimeConfig::builder()
                    .chain_polling_interval(params.polling_interval.clone())
                    .build(),
            )
            .build()
            .await;
        let testnet_contracts = Testnet::setup_contracts(
            &mut testnet,
            Some(
                StakingContractConfig::builder()
                    .epoch_length(U256::from(params.epoch_length.clone()))
                    .build(),
            ),
        )
        .await
        .map_err(|e| anyhow::anyhow!("Error while spawning testnet contracts: {}", e))?;

        let actions = testnet.actions(testnet_contracts.contracts());
        let validator_collection = ValidatorCollection::builder()
            .num_staked_nodes(params.node_count.clone())
            .custom_binary_path(params.custom_build_path)
            .build(&testnet, &actions)
            .await
            .map_err(|e| anyhow::anyhow!("Error while spawning validators: {}", e))?;

        Ok(TestnetInsance {
            node_count: params.node_count.clone(),
            polling_interval: params.polling_interval.clone(),
            epoch_length: params.epoch_length.clone(),
            exisiting_config_path: params.existing_config_path.clone(),
            ecdsa_round_timeout: params.ecdsa_round_timeout.clone(),
            enable_rate_limiting: params.enable_rate_limiting.clone(),
            action_server: lit_action_process,
            actions: actions,
            test_net: testnet,
            contracts: testnet_contracts,
            validators: validator_collection,
        })
    }

    pub async fn stop_random_node(&mut self) -> Result<(), anyhow::Error> {
        info!("Stopping random node");
        let current_epoch = self.validators.actions().get_current_epoch().await;
        let res = self.validators.stop_random_node().await;
        if res.is_ok() {
            info!("Stopped random node");
            self.validators
                .actions()
                .wait_for_epoch(current_epoch + 1)
                .await;
            return Ok(());
        }

        Err(anyhow::anyhow!("error while kicking random validator"))
    }

    pub async fn wait_for_epoch(&mut self) -> () {
        info!("Stopping random node");
        let current_epoch = self.validators.actions().get_current_epoch().await;
        let _res = self
            .validators
            .actions()
            .wait_for_epoch(current_epoch + 1)
            .await;

        ()
    }

    pub async fn stop_node(&mut self, address: String) -> Result<(), anyhow::Error> {
        let mut idx: usize = 999;
        for i in 0..self.validators.size() {
            if self
                .validators
                .get_validator_by_idx(i)
                .account()
                .node_address
                .to_string()
                == address
            {
                idx = i;
            }
        }

        if idx != 999 {
            let res = self.validators.stop_node(idx).await;
            return Ok(());
        }

        Err(anyhow::anyhow!("error while kicking random validator"))
    }
}

impl Drop for TestnetInsance {
    fn drop(&mut self) {
        info!("Stopping processes");
        if let Some(la) = self.action_server.as_mut() {
            info!("Killing lit action server with pid: {}", la.id());
            let _ = la.kill();
        }
    }
}
