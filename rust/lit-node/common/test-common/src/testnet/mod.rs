pub mod actions;
pub mod chain;
pub mod contracts;
pub mod contracts_repo;
pub mod node_config;
pub mod rate_limit_nfts;

use crate::testnet::contracts_repo::{
    contract_addresses_from_deployment, remote_deployment_and_config_creation,
};

use self::actions::Actions;
use self::chain::ChainTrait;
use self::contracts::{ContractAddresses, Contracts, StakingContractConfig};
use self::contracts_repo::{check_and_load_chain_state_cache, save_to_chain_state_cache};
use self::node_config::{generate_custom_node_runtime_config, CustomNodeRuntimeConfig};
use command_group::GroupChild;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use ethers::types::Address;
use futures::future::BoxFuture;
use lit_blockchain::resolver::rpc::{RpcHealthcheckPoller, ENDPOINT_MANAGER};
use lit_core::utils::toml::SimpleToml;
use lit_node::models::coms_keys::ComsKeys;
use lit_node::utils::encoding::hex_to_bytes;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

pub fn signing_provider(
    wallet: Wallet<SigningKey>,
    chain: String,
) -> Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    let rpc = lit_node::access_control::rpc_url(chain).expect("Could not get RPC Provider");
    let provider = Provider::<Http>::try_from(rpc).expect("Could not create ethers provider");

    Arc::new(SignerMiddleware::new(provider, wallet))
}

#[derive(PartialEq, Clone, Default)]
pub enum WhichTestnet {
    Hardhat,
    NoChain,
    #[default]
    Anvil,
}

#[derive(Clone, Debug)]
pub struct NodeAccount {
    pub signing_provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub node_address: Address,
    pub node_address_private_key: ethers::types::H256,
    pub staker_address: Address,
    pub staker_address_private_key: ethers::types::H256,
    pub coms_keys: ComsKeys,
}

impl PartialEq for NodeAccount {
    fn eq(&self, other: &Self) -> bool {
        self.staker_address_private_key == other.staker_address_private_key
            && self.node_address_private_key == other.node_address_private_key
    }
}

#[must_use]
pub struct TestnetBuilder {
    which: WhichTestnet,
    num_staked_only_validators: usize,
    num_staked_and_joined_validators: usize,
    force_deploy_in_ci: bool,
    staker_account_setup_mapper: Option<
        Box<dyn StakerAccountSetupMapper<Future = BoxFuture<'static, Result<(), anyhow::Error>>>>,
    >,

    // FIXME: these parameters need to be refactor since conceptually don't belong to Testnet struct.
    custom_node_runtime_config: Option<CustomNodeRuntimeConfig>,
    is_fault_test: bool,
}

impl Default for TestnetBuilder {
    fn default() -> Self {
        Self {
            which: WhichTestnet::default(),
            num_staked_only_validators: 0,
            num_staked_and_joined_validators: 10,
            force_deploy_in_ci: false,
            staker_account_setup_mapper: None,
            custom_node_runtime_config: None,
            is_fault_test: false,
        }
    }
}

impl TestnetBuilder {
    pub fn which_testnet(self, which: WhichTestnet) -> Self {
        Self { which, ..self }
    }

    pub fn num_staked_only_validators(self, num_staked_only_validators: usize) -> Self {
        Self {
            num_staked_only_validators,
            ..self
        }
    }

    pub fn num_staked_and_joined_validators(self, num_staked_and_joined_validators: usize) -> Self {
        Self {
            num_staked_and_joined_validators,
            ..self
        }
    }

    pub fn total_num_validators(&self) -> usize {
        self.num_staked_only_validators + self.num_staked_and_joined_validators
    }

    pub fn force_deploy_in_ci(self, force_deploy_in_ci: bool) -> Self {
        Self {
            force_deploy_in_ci,
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

    pub fn is_fault_test(self, is_fault_test: bool) -> Self {
        Self {
            is_fault_test,
            ..self
        }
    }

    pub fn staker_account_setup_mapper(
        self,
        staker_account_setup_mapper: Box<
            dyn StakerAccountSetupMapper<Future = BoxFuture<'static, Result<(), anyhow::Error>>>,
        >,
    ) -> Self {
        Self {
            staker_account_setup_mapper: Some(staker_account_setup_mapper),
            ..self
        }
    }

    pub async fn build(self) -> Testnet {
        let chain = match self.which {
            WhichTestnet::Hardhat => {
                Box::new(chain::hardhat::Hardhat::new(self.total_num_validators()))
                    as Box<dyn ChainTrait>
            }
            WhichTestnet::Anvil => Box::new(chain::anvil::Anvil::new(self.total_num_validators()))
                as Box<dyn ChainTrait>,
            WhichTestnet::NoChain => {
                Box::new(chain::no_chain::NoChain::new(self.total_num_validators()))
                    as Box<dyn ChainTrait>
            }
        };

        let net_process = chain.start_chain().await;
        let mut provider_mut = ENDPOINT_MANAGER
            .get_provider(chain.chain_name())
            .expect("Error retrieving provider - check name and yaml.");

        let provider = Arc::new(provider_mut.set_interval(Duration::from_millis(10)).clone());

        // deploy the contracts via script first, so that we can read them when the testnet configuration is loaded.
        if self.which != WhichTestnet::NoChain {
            // First, determine whether we need to generate custom node runtime config.
            let need_custom_node_runtime_config =
                self.custom_node_runtime_config.is_some() || self.is_fault_test;
            if let Some(custom_node_runtime_config) = self.custom_node_runtime_config.as_ref() {
                generate_custom_node_runtime_config(
                    self.is_fault_test,
                    &self.which,
                    custom_node_runtime_config,
                );
            } else if self.is_fault_test {
                generate_custom_node_runtime_config(
                    self.is_fault_test,
                    &self.which,
                    &Default::default(),
                );
            }

            // if we're in CI, or if the LIT_SAVE_AND_RESTORE_CHAIN_STATE env var is set to 1, we save and restore the chain state
            let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string());
            let save_and_restore_chain_state =
                std::env::var("LIT_SAVE_AND_RESTORE_CHAIN_STATE").unwrap_or("0".to_string());
            if (in_github_ci == "1" || save_and_restore_chain_state == "1")
                && !self.force_deploy_in_ci
            {
                if !check_and_load_chain_state_cache(
                    provider.clone(),
                    self.num_staked_and_joined_validators,
                    self.num_staked_only_validators,
                )
                .await
                {
                    remote_deployment_and_config_creation(
                        self.num_staked_and_joined_validators,
                        self.num_staked_only_validators,
                        need_custom_node_runtime_config,
                    )
                    .await;
                    info!("Deployed contracts via script.");
                    if self.which == WhichTestnet::Anvil {
                        save_to_chain_state_cache(
                            provider.clone(),
                            self.num_staked_and_joined_validators,
                            self.num_staked_only_validators,
                        )
                        .await;
                    }
                }
            } else {
                // just deploy normally
                remote_deployment_and_config_creation(
                    self.num_staked_and_joined_validators,
                    self.num_staked_only_validators,
                    need_custom_node_runtime_config,
                )
                .await;
            }
        }

        let rpcurl = chain.rpc_url();
        let deploy_account = chain.deployer();
        let deploy_address = deploy_account.signing_provider.address();
        let existing_config_path = chain.reuse_config_path();

        Testnet {
            process: net_process,
            rpcurl,
            which: self.which,
            provider,
            deploy_address,
            chain_name: chain.chain_name().to_string(),
            chain_id: chain.chain_id(),
            node_accounts: chain.accounts(),
            deploy_account,
            existing_config_path,
            num_staked_only_validators: self.num_staked_only_validators,
            num_staked_and_joined_validators: self.num_staked_and_joined_validators,
            staker_account_setup_mapper: self.staker_account_setup_mapper,
        }
    }
}

pub struct TestnetContracts {
    contracts: Contracts,
    contract_addresses: ContractAddresses,
}

impl TestnetContracts {
    pub fn contracts(&self) -> &Contracts {
        &self.contracts
    }

    pub fn contract_addresses(&self) -> &ContractAddresses {
        &self.contract_addresses
    }
}

pub struct Testnet {
    process: GroupChild,
    pub rpcurl: String, //http://localhost:8545
    pub chain_name: String,
    pub chain_id: u64,
    pub which: WhichTestnet,
    pub provider: Arc<Provider<Http>>,
    pub deploy_address: Address,
    pub node_accounts: Arc<Vec<NodeAccount>>,
    pub deploy_account: NodeAccount,
    pub existing_config_path: Option<String>,
    /// Number of validators that have only staked but not joined, exclusive of those already accounted for in `num_staked_and_joined_validators`.
    pub num_staked_only_validators: usize,
    /// Number of validators that have staked and joined, exclusive of those already accounted for in `num_staked_only_validators`.
    pub num_staked_and_joined_validators: usize,
    staker_account_setup_mapper: Option<
        Box<dyn StakerAccountSetupMapper<Future = BoxFuture<'static, Result<(), anyhow::Error>>>>,
    >,
}

impl Testnet {
    pub fn builder() -> TestnetBuilder {
        TestnetBuilder::default()
    }

    pub fn total_num_validators(&self) -> usize {
        self.num_staked_only_validators + self.num_staked_and_joined_validators
    }

    // stop testnet and clean up
    fn stop(&mut self) {
        // return; // uncomment this if you want to keep anvil running
        if self.which != WhichTestnet::NoChain {
            self.process.kill().unwrap_or_else(|e| {
                panic!(
                    "Testnet process {:?} couldn't be killed: {}",
                    self.process, e
                )
            });
        }

        //ps x -o  "%p %r %y %x %c "
        self.process.wait().unwrap();
        // if hardhat or node are spawning something and leaving it running after kill
        // Command::new("pkill").arg("node").spawn().unwrap();
    }

    pub fn actions(&self, contracts: &Contracts) -> Actions {
        Actions::new(
            contracts.clone(),
            self.deploy_account.signing_provider.clone(),
            self.which.clone(),
            self.deploy_address,
        )
    }

    pub async fn setup_contracts(
        testnet: &mut Testnet,
        staking_contract_initial_config: Option<StakingContractConfig>,
    ) -> anyhow::Result<TestnetContracts> {
        let ca = match testnet.existing_config_path.clone() {
            Some(_path) => {
                Contracts::contract_addresses_from_resolver(
                    _path,
                    testnet.deploy_account.signing_provider.clone(),
                )
                .await
            }
            None => contract_addresses_from_deployment().await,
        };

        let deployer_signing_provider = testnet.deploy_account.signing_provider.clone();

        info!("contract addresses: {:?}", &ca);
        let contracts = Contracts::new(
            &ca,
            testnet,
            deployer_signing_provider.clone(),
            staking_contract_initial_config,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to deploy/resolve contracts: {}", e))?;

        Ok(TestnetContracts {
            contracts,
            contract_addresses: ca,
        })
    }
}

pub trait StakerAccountSetupMapper {
    type Future: Future<Output = Result<(), anyhow::Error>>;

    fn run(&mut self, args: (usize, NodeAccount, Contracts)) -> Self::Future;
}

impl<
        T: Future<Output = Result<(), anyhow::Error>>,
        F: FnMut((usize, NodeAccount, Contracts)) -> T,
    > StakerAccountSetupMapper for F
{
    type Future = T;

    fn run(&mut self, args: (usize, NodeAccount, Contracts)) -> Self::Future {
        self(args)
    }
}

// Implementing drop means we don't have to remember to clean up the testnet, and is more able to clean up even when there is a panic, since drop may still be called.
impl Drop for Testnet {
    fn drop(&mut self) {
        info!("Attempting to stop Testnet");
        self.stop();
    }
}

pub(crate) trait SimpleTomlValue {
    fn get_address(&self, section: &str, key: &str) -> Option<H160>;
    fn get_signing_key(&self) -> Option<Vec<u8>>;
}

impl SimpleTomlValue for SimpleToml {
    fn get_address(&self, section: &str, key: &str) -> Option<H160> {
        let section = self.data().get(section);
        section?;
        let value = section.unwrap().get(key);

        value?;
        let value = value.unwrap().as_str();
        let bytes = hex_to_bytes(value).expect("Could not parse hex");
        let address = H160::from_slice(&bytes);
        Some(address)
    }

    fn get_signing_key(&self) -> Option<Vec<u8>> {
        let section = self.data().get("blockchain.wallet.default");
        section?;
        let value = section.unwrap().get("private_key");

        value?;
        let value = value.unwrap().as_str();
        let bytes = hex_to_bytes(value).expect("Could not parse hex");

        Some(bytes)
    }
}
