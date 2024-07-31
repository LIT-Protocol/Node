use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use super::SimpleTomlValue;

use super::Testnet;
use super::WhichTestnet;
use anyhow::Result;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::builders::ContractCall;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain::contracts::staking::staking;
use lit_blockchain::contracts::staking::Config;
use lit_blockchain::contracts::{
    backup_recovery::BackupRecovery, contract_resolver::*, lit_token::lit_token::LITToken,
    payment_delegation::PaymentDelegation, pkp_helper::pkp_helper::PKPHelper,
    pkp_permissions::PKPPermissions, pkpnft::PKPNFT, pubkey_router::PubkeyRouter,
    rate_limit_nft::RateLimitNFT, staking::Staking, staking_balances::StakingBalances,
};
use lit_core::utils::toml::SimpleToml;
use lit_node::contracts::erc20::ERC20;

use tracing::info;

#[derive(Debug, Clone)]
pub struct Contracts {
    pub lit_token: LITToken<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub erc20: ERC20<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub backup_recovery: BackupRecovery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub staking: Staking<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub staking_balances: StakingBalances<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub rate_limit_nft: RateLimitNFT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub pkpnft: PKPNFT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub pubkey_router: PubkeyRouter<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub pkp_permissions: PKPPermissions<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub pkp_helper: PKPHelper<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub contract_resolver: ContractResolver<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub payment_delegation: PaymentDelegation<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

#[derive(Debug)]
pub struct ContractAddresses {
    pub lit_token: Address,
    pub backup_recovery: Address,
    pub staking: Address,
    pub staking_balances: Address,
    pub rate_limit_nft: Address,
    pub pkpnft: Address,
    pub pubkey_router: Address,
    pub pkp_permissions: Address,
    pub pkp_helper: Address,
    pub contract_resolver: Address,
    pub key_deriver: Address,
    pub payment_delegation: Address,
}

#[derive(Default)]
#[must_use]
pub struct StakingContractConfigBuilder {
    token_reward_per_token_per_epoch: Option<U256>,
    key_types: Option<Vec<U256>>,
    minimum_validator_count: Option<U256>,
    epoch_length: Option<U256>,
    max_concurrent_requests: Option<U256>,
    max_triple_count: Option<U256>,
    min_triple_count: Option<U256>,
    peer_checking_interval_secs: Option<U256>,
    max_triple_concurrency: Option<U256>,
    complaint_reason_to_config: Option<HashMap<U256, ComplaintConfig>>,
}

impl StakingContractConfigBuilder {
    pub fn token_reward_per_token_per_epoch(mut self, value: U256) -> Self {
        self.token_reward_per_token_per_epoch = Some(value);
        self
    }

    pub fn key_types(mut self, value: Vec<U256>) -> Self {
        self.key_types = Some(value);
        self
    }

    pub fn minimum_validator_count(mut self, value: U256) -> Self {
        self.minimum_validator_count = Some(value);
        self
    }

    pub fn epoch_length(mut self, value: U256) -> Self {
        self.epoch_length = Some(value);
        self
    }

    pub fn max_concurrent_requests(mut self, value: U256) -> Self {
        self.max_concurrent_requests = Some(value);
        self
    }

    pub fn max_triple_count(mut self, value: U256) -> Self {
        self.max_triple_count = Some(value);
        self
    }

    pub fn min_triple_count(mut self, value: U256) -> Self {
        self.min_triple_count = Some(value);
        self
    }

    pub fn peer_checking_interval_secs(mut self, value: U256) -> Self {
        self.peer_checking_interval_secs = Some(value);
        self
    }

    pub fn max_triple_concurrency(mut self, value: U256) -> Self {
        self.max_triple_concurrency = Some(value);
        self
    }

    pub fn complaint_reason_to_config(mut self, value: HashMap<U256, ComplaintConfig>) -> Self {
        self.complaint_reason_to_config = Some(value);
        self
    }

    pub fn build(self) -> StakingContractConfig {
        StakingContractConfig {
            token_reward_per_token_per_epoch: self.token_reward_per_token_per_epoch,
            key_types: self.key_types,
            minimum_validator_count: self.minimum_validator_count,
            epoch_length: self.epoch_length,
            max_concurrent_requests: self.max_concurrent_requests,
            max_triple_count: self.max_triple_count,
            min_triple_count: self.min_triple_count,
            peer_checking_interval_secs: self.peer_checking_interval_secs,
            max_triple_concurrency: self.max_triple_concurrency,
            complaint_reason_to_config: self.complaint_reason_to_config,
        }
    }
}

#[derive(Debug)]
pub struct StakingContractConfig {
    token_reward_per_token_per_epoch: Option<U256>,
    key_types: Option<Vec<U256>>,
    minimum_validator_count: Option<U256>,
    epoch_length: Option<U256>,
    max_concurrent_requests: Option<U256>,
    max_triple_count: Option<U256>,
    min_triple_count: Option<U256>,
    peer_checking_interval_secs: Option<U256>,
    max_triple_concurrency: Option<U256>,
    complaint_reason_to_config: Option<HashMap<U256, ComplaintConfig>>,
}

#[derive(Default)]
#[must_use]
pub struct ComplaintConfigBuilder {
    tolerance: Option<U256>,
    interval_secs: Option<U256>,
    kick_penalty_percent: Option<U256>,
}

impl ComplaintConfigBuilder {
    pub fn tolerance(mut self, value: U256) -> Self {
        self.tolerance = Some(value);
        self
    }

    pub fn interval_secs(mut self, value: U256) -> Self {
        self.interval_secs = Some(value);
        self
    }

    pub fn kick_penalty_percent(mut self, value: U256) -> Self {
        self.kick_penalty_percent = Some(value);
        self
    }

    pub fn build(self) -> ComplaintConfig {
        ComplaintConfig {
            tolerance: self.tolerance,
            interval_secs: self.interval_secs,
            kick_penalty_percent: self.kick_penalty_percent,
        }
    }
}

#[derive(Debug)]
pub struct ComplaintConfig {
    tolerance: Option<U256>,
    interval_secs: Option<U256>,
    kick_penalty_percent: Option<U256>,
}

impl ComplaintConfig {
    pub fn builder() -> ComplaintConfigBuilder {
        ComplaintConfigBuilder::default()
    }
}

impl StakingContractConfig {
    pub fn builder() -> StakingContractConfigBuilder {
        StakingContractConfigBuilder::default()
    }
}

impl Contracts {
    pub async fn new(
        ca: &ContractAddresses,
        testnet: &mut Testnet,
        provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        staking_contract_initial_config: Option<StakingContractConfig>,
    ) -> Result<Contracts> {
        let lit_token = LITToken::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            ca.lit_token,
            provider.clone(),
        );

        let contract_resolver = ContractResolver::<
            SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        >::new(ca.contract_resolver, provider.clone());

        let staking_balances = StakingBalances::<
            SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        >::new(ca.staking_balances, provider.clone());

        let backup_recovery =
            BackupRecovery::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
                ca.backup_recovery,
                provider.clone(),
            );

        let staking = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            ca.staking,
            provider.clone(),
        );
        let erc20 = ERC20::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            ca.lit_token,
            provider.clone(),
        );
        let rate_limit_nft =
            RateLimitNFT::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
                ca.rate_limit_nft,
                provider.clone(),
            );
        let pkpnft = PKPNFT::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            ca.pkpnft,
            provider.clone(),
        );

        let pubkey_router =
            PubkeyRouter::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
                ca.pubkey_router,
                provider.clone(),
            );
        let pkp_permissions =
            PKPPermissions::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
                ca.pkp_permissions,
                provider.clone(),
            );

        let pkp_helper = PKPHelper::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            ca.pkp_helper,
            provider.clone(),
        );

        let payment_delegation = PaymentDelegation::<
            SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        >::new(ca.payment_delegation, provider.clone());

        if testnet.existing_config_path.is_none() && testnet.which != WhichTestnet::NoChain {
            if let Some(staking_contract_initial_config) = staking_contract_initial_config {
                Self::update_staking_config(staking.clone(), staking_contract_initial_config)
                    .await?;
            }
        }

        info!(
            "Resolver contract in staking contract {:?}",
            staking.contract_resolver().await.unwrap()
        );

        let contracts = Contracts {
            lit_token,
            erc20,
            backup_recovery,
            staking,
            staking_balances,
            rate_limit_nft,
            pkpnft,
            pubkey_router,
            pkp_permissions,
            pkp_helper,
            contract_resolver,
            payment_delegation,
        };

        // Loop through each staker account to execute each of their setup.
        if let Some(staker_account_setup_mapper) = testnet.staker_account_setup_mapper.as_mut() {
            for (idx, node_account) in testnet.node_accounts.iter().enumerate() {
                info!(
                    "Running custom setup function for account {:?}",
                    node_account
                );

                if let Err(e) = staker_account_setup_mapper
                    .run((idx, node_account.to_owned(), contracts.clone()))
                    .await
                {
                    panic!("Error running staker account setup: {:?}", e);
                }
            }
        }

        Ok(contracts)
    }

    pub async fn process_contract_call(
        cc: ContractCall<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>, ()>,
        desc: &str,
    ) -> bool {
        let tx = cc.send().await;

        match tx {
            Ok(tx) => {
                let r = tx.interval(Duration::from_millis(10)).log_msg(desc).await;
                match r {
                    Ok(_) => {
                        info!("Success {}", desc);
                        return true;
                    }
                    Err(e) => {
                        info!("Error {}: {:?}", desc, e);
                        return false;
                    }
                }
            }
            Err(e) => {
                info!("Error {}: {:?}", desc, e);
                return false;
            }
        }
    }

    pub async fn contract_addresses_from_resolver(
        config_path: String,
        provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    ) -> ContractAddresses {
        let config_path = format!("./{}/lit_config0.toml", config_path); // fix me
        let path = std::path::Path::new(&config_path);
        let cfg = SimpleToml::try_from(path).unwrap();

        info!(
            "Reusing earlier deployment.  Loading contract addresses from '{:?}'",
            config_path
        );

        // get the staking contract address from the config file - it's the subnetid
        let staking = cfg
            .get_address("subnet", "id")
            .expect("couldn't load staking address");

        // get the resolver contract address from the staking contract
        let staking_contract = Staking::new(staking, provider.clone());
        let contract_resolver = staking_contract.contract_resolver().call().await.unwrap();
        let resolver = ContractResolver::new(contract_resolver, provider.clone());

        let env: u8 = 0;

        // get contract addresses from resolver contract
        let lit_token = resolver
            .get_contract(resolver.lit_token_contract().call().await.unwrap(), env)
            .call()
            .await
            .unwrap();
        let rate_limit_nft = resolver
            .get_contract(
                resolver.rate_limit_nft_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let pkpnft = resolver
            .get_contract(resolver.pkp_nft_contract().call().await.unwrap(), env)
            .call()
            .await
            .unwrap();

        let pkp_helper = resolver
            .get_contract(resolver.pkp_helper_contract().call().await.unwrap(), env)
            .call()
            .await
            .unwrap();

        let pubkey_router = resolver
            .get_contract(
                resolver.pub_key_router_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let pkp_permissions = resolver
            .get_contract(
                resolver.pkp_permissions_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let backup_recovery = resolver
            .get_contract(
                resolver.backup_recovery_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let staking_balances = resolver
            .get_contract(
                resolver.staking_balances_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();

        let key_deriver = resolver
            .get_contract(
                resolver.hd_key_deriver_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();

        let payment_delegation = resolver
            .get_contract(
                resolver.payment_delegation_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();

        ContractAddresses {
            lit_token,
            backup_recovery,
            staking,
            staking_balances,
            rate_limit_nft,
            pkpnft,
            pkp_helper,
            pubkey_router,
            pkp_permissions,
            contract_resolver,
            key_deriver,
            payment_delegation,
        }
    }

    pub async fn new_blank(
        client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    ) -> Result<Contracts> {
        let address = Address::zero();
        let lit_token = LITToken::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            address,
            client.clone(),
        );

        let contract_resolver = ContractResolver::<
            SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        >::new(address, client.clone());

        let staking_balances = StakingBalances::<
            SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        >::new(address, client.clone());

        let backup_recovery =
            BackupRecovery::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
                address,
                client.clone(),
            );
        let staking = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            address,
            client.clone(),
        );
        let erc20 = ERC20::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            address,
            client.clone(),
        );
        let rate_limit_nft =
            RateLimitNFT::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
                address,
                client.clone(),
            );
        let pkpnft = PKPNFT::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            address,
            client.clone(),
        );

        let pubkey_router =
            PubkeyRouter::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
                address,
                client.clone(),
            );
        let pkp_permissions =
            PKPPermissions::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
                address,
                client.clone(),
            );

        let pkp_helper = PKPHelper::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            address,
            client.clone(),
        );

        let payment_delegation = PaymentDelegation::<
            SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        >::new(address, client.clone());

        Ok(Contracts {
            lit_token,
            erc20,
            backup_recovery,
            staking,
            staking_balances,
            rate_limit_nft,
            pkpnft,
            pubkey_router,
            pkp_permissions,
            pkp_helper,
            contract_resolver,
            payment_delegation,
        })
    }

    pub async fn update_staking_config(
        staking: Staking<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        staking_config: StakingContractConfig,
    ) -> Result<()> {
        info!("Updating staking contract config");

        // Update the config using the defaults where the user didn't specify a value.
        let config = staking
            .config()
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("unable to get staking config: {:?}", e))?;
        let token_reward_per_token_per_epoch = config.token_reward_per_token_per_epoch;
        let minimum_validator_count = config.minimum_validator_count;
        let max_concurrent_requests = config.max_concurrent_requests;
        let max_triple_count = config.max_triple_count;
        let min_triple_count = config.min_triple_count;
        let peer_checking_interval_secs = config.peer_checking_interval_secs;
        let max_triple_concurrency = config.max_triple_concurrency;

        let key_types = staking
            .get_key_types()
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("unable to get key types: {:?}", e))?;
        let cc = staking.set_config(Config {
            token_reward_per_token_per_epoch: staking_config
                .token_reward_per_token_per_epoch
                .unwrap_or(token_reward_per_token_per_epoch),
            deprecated_complaint_tolerance: U256::zero(),
            deprecated_complaint_interval_secs: U256::zero(),
            key_types: staking_config.key_types.unwrap_or(key_types),
            minimum_validator_count: staking_config
                .minimum_validator_count
                .unwrap_or(minimum_validator_count),
            max_concurrent_requests: staking_config
                .max_concurrent_requests
                .unwrap_or(max_concurrent_requests),
            max_triple_count: staking_config.max_triple_count.unwrap_or(max_triple_count),
            min_triple_count: staking_config.min_triple_count.unwrap_or(min_triple_count),
            peer_checking_interval_secs: staking_config
                .peer_checking_interval_secs
                .unwrap_or(peer_checking_interval_secs),
            max_triple_concurrency: staking_config
                .max_triple_concurrency
                .unwrap_or(max_triple_concurrency),
            rpc_healthcheck_enabled: true,
        });
        if !Self::process_contract_call(cc, "updating staking config").await {
            return Err(anyhow::anyhow!("Error updating staking config"));
        }

        if let Some(complaint_reason_to_config) = staking_config.complaint_reason_to_config {
            info!("Updating staking contract complaint reason configs");

            for (reason, new_config) in complaint_reason_to_config {
                // First, get current chain config for this reason.
                let current_config: staking::ComplaintConfig = staking
                    .complaint_config(reason)
                    .call()
                    .await
                    .map_err(|e| anyhow::anyhow!("unable to get complaint config: {:?}", e))?;

                // Then, set the config with any new values.
                let cc = staking.set_complaint_config(
                    reason,
                    staking::ComplaintConfig {
                        tolerance: new_config.tolerance.unwrap_or(current_config.tolerance),
                        interval_secs: new_config
                            .interval_secs
                            .unwrap_or(current_config.interval_secs),
                        kick_penalty_percent: new_config
                            .kick_penalty_percent
                            .unwrap_or(current_config.kick_penalty_percent),
                    },
                );
                Self::process_contract_call(
                    cc,
                    format!("updating staking complaint config for reason {:?}", reason).as_str(),
                )
                .await;
            }
        }

        if let Some(custom_epoch_length) = staking_config.epoch_length {
            info!("Updating staking contract epoch length");

            let cc = staking.set_epoch_length(custom_epoch_length);
            Self::process_contract_call(cc, "updating staking epoch length").await;
        }

        Ok(())
    }
}
