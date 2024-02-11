use std::sync::Arc;

use crate::common::testnet::contracts_repo::contract_addresses_from_deployment;
use crate::common::testnet::node_config::CustomNodeRuntimeConfig;

use super::actions::Actions;
use super::contracts::Contracts;
use super::contracts::StakingContractInitialConfig;
use super::node_config::generate_custom_node_runtime_config;
use super::Testnet;
use super::TestnetBuilder;
use super::WhichTestnet;
use ethers::prelude::*;
use ethers::providers::Provider;
use tracing::info;

#[must_use]
pub struct ScenarioBuilder {
    which: WhichTestnet,
    num_staked: usize,
    num_nodes: usize,
    num_awake_initially: usize,
    initial_port: usize,
    initial_dkg_state: InitialDKGState,
    wait_for_dkg_to_complete: bool,
    is_fault_test: bool,
    staking_contract_initial_config: Option<StakingContractInitialConfig>,
    force_deploy_in_ci: bool,
    testnet_builder: Option<TestnetBuilder>,
    custom_node_runtime_config: Option<CustomNodeRuntimeConfig>,
}

impl Default for ScenarioBuilder {
    fn default() -> Self {
        Self {
            which: WhichTestnet::Anvil,
            num_staked: 10,
            num_nodes: 10,
            num_awake_initially: 10,
            initial_port: 7470,
            initial_dkg_state: InitialDKGState::ForceStart,
            wait_for_dkg_to_complete: true,
            is_fault_test: false,
            staking_contract_initial_config: None,
            force_deploy_in_ci: false,
            testnet_builder: None,
            custom_node_runtime_config: None,
        }
    }
}

impl ScenarioBuilder {
    pub fn which_testnet(self, which: WhichTestnet) -> Self {
        Self { which, ..self }
    }

    pub fn num_nodes(self, num_nodes: usize) -> Self {
        Self { num_nodes, ..self }
    }

    pub fn num_staked(self, num_staked: usize) -> Self {
        Self { num_staked, ..self }
    }

    pub fn num_awake_initially(self, num_awake_initially: usize) -> Self {
        Self {
            num_awake_initially,
            ..self
        }
    }

    pub fn initial_port(self, initial_port: usize) -> Self {
        Self {
            initial_port,
            ..self
        }
    }

    pub fn initial_dkg_state(self, initial_dkg_state: InitialDKGState) -> Self {
        Self {
            initial_dkg_state,
            ..self
        }
    }

    pub fn wait_for_dkg_to_complete(self, wait_for_dkg_to_complete: bool) -> Self {
        Self {
            wait_for_dkg_to_complete,
            ..self
        }
    }

    pub fn is_fault_test(self, is_fault_test: bool) -> Self {
        Self {
            is_fault_test,
            ..self
        }
    }

    pub fn staking_contract_initial_config(
        self,
        staking_contract_initial_config: StakingContractInitialConfig,
    ) -> Self {
        Self {
            staking_contract_initial_config: Some(staking_contract_initial_config),
            ..self
        }
    }

    pub fn force_deploy_in_ci(self, force_deploy_in_ci: bool) -> Self {
        Self {
            force_deploy_in_ci,
            ..self
        }
    }

    pub fn testnet_builder(self, testnet_builder: TestnetBuilder) -> Self {
        Self {
            testnet_builder: Some(testnet_builder),
            ..self
        }
    }

    pub fn custom_node_runtime_config(
        self,
        custom_node_runtime_config: CustomNodeRuntimeConfig,
    ) -> Self {
        Self {
            custom_node_runtime_config: Some(custom_node_runtime_config),
            ..self
        }
    }

    /// NOTE: This is usually used for creating virtual node collections for component tests.
    pub async fn build_blank(self) -> Scenario {
        let epoch_length: U256 = U256::from(160);

        let testnet = match self.testnet_builder {
            Some(testnet_builder) => testnet_builder.build().await,
            None => {
                Testnet::builder()
                    .which_testnet(WhichTestnet::NoChain)
                    .num_nodes(self.num_nodes)
                    .num_staked(self.num_staked)
                    .build()
                    .await
            }
        };
        let client = testnet.deploy_account.signing_provider.clone();
        let provider = testnet.provider.clone();

        let contracts = Contracts::new_blank(client)
            .await
            .expect("Failed to deploy/resolve contracts");

        let actions = Actions::new(
            contracts.clone(),
            provider.clone(),
            WhichTestnet::NoChain,
            Address::zero(),
        );

        Scenario {
            testnet,
            provider,
            num_staked: self.num_staked,
            num_nodes: self.num_nodes,
            num_awake_initially: self.num_awake_initially,
            initial_port: self.initial_port,
            epoch_length,
            initial_dkg_state: InitialDKGState::ForceStart,
            wait_for_dkg_to_complete: false,
            is_fault_test: self.is_fault_test,
            contracts,
            actions,
        }
    }

    pub async fn build(self) -> Scenario {
        // Generate the custom node runtime config
        generate_custom_node_runtime_config(
            self.is_fault_test,
            self.which.clone(),
            self.custom_node_runtime_config.unwrap_or_default(),
        );
        tracing::info!("Generated custom node runtime config.");
        // Initialize the Testnet
        let mut testnet = match self.testnet_builder {
            Some(testnet_builder) => testnet_builder.build().await,
            None => {
                Testnet::builder()
                    .which_testnet(self.which)
                    .num_nodes(self.num_nodes)
                    .num_staked(self.num_staked)
                    .force_deploy_in_ci(self.force_deploy_in_ci)
                    .build()
                    .await
            }
        };

        let epoch_length: U256 = U256::from(160);

        let provider = testnet.deploy_account.signing_provider.clone();

        let ca = match testnet.existing_config_path.clone() {
            Some(_path) => {
                Contracts::contract_addresses_from_resolver(
                    _path,
                    provider.clone(),
                    &testnet,
                    self.initial_dkg_state,
                )
                .await
            }
            None => contract_addresses_from_deployment(&testnet).await,
        };

        info!("contract addresses: {:?}", &ca);
        let contracts = Contracts::new(
            &ca,
            &mut testnet,
            provider,
            epoch_length,
            self.staking_contract_initial_config,
        )
        .await
        .expect("Failed to deploy/resolve contracts");
        let provider = testnet.provider.clone();
        let actions = Actions::new(
            contracts.clone(),
            provider.clone(),
            testnet.which.clone(),
            testnet.deploy_address,
        );

        Scenario {
            testnet,
            contracts,
            actions,
            provider,
            num_staked: self.num_staked,
            num_nodes: self.num_nodes,
            num_awake_initially: self.num_awake_initially,
            initial_port: self.initial_port,
            epoch_length,
            initial_dkg_state: self.initial_dkg_state,
            wait_for_dkg_to_complete: self.wait_for_dkg_to_complete,
            is_fault_test: self.is_fault_test,
        }
    }
}

pub struct Scenario {
    pub testnet: Testnet,
    pub contracts: Contracts,
    pub actions: Actions,
    pub provider: Arc<Provider<Http>>,
    pub num_staked: usize,
    pub num_nodes: usize,
    pub num_awake_initially: usize,
    pub initial_port: usize,
    pub epoch_length: U256,
    pub initial_dkg_state: InitialDKGState,
    pub wait_for_dkg_to_complete: bool,
    pub is_fault_test: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InitialDKGState {
    ReUseOrStart,
    ForceStart,
    TestIsStakingAndDKG,
    TestIsOnlyStaking,
}

impl Scenario {
    pub fn builder() -> ScenarioBuilder {
        ScenarioBuilder::default()
    }
}
