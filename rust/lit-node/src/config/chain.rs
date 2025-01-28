use ethers::types::{H160, U256};
use lit_blockchain::contracts::staking::{staking, AddressMapping, Version};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::{LitConfig, ReloadableLitConfig};
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use moka::future::Cache;
use rocket::serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tracing::trace;

use crate::config::{LitNodeConfig, CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT};
use crate::error::blockchain_err;
use crate::error::Result;
use crate::error::{conversion_err, unexpected_err_code, EC};
use crate::models::PeerValidator;
use crate::peers::peer_reviewer::MAX_COMPLAINT_REASON_VALUE;
use crate::peers::peer_state::models::{SimplePeer, SimplePeerExt};
use crate::tss::common::curve_type::CurveType;
use crate::utils::networking::get_web_addr_from_chain_info;

#[derive(PartialEq, Debug)]
pub enum PeerGroupEpoch {
    Current,
    Next,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimitDataConfig {
    pub default_window_duration_secs: Duration,
    pub default_max_requests: u64,
    pub rli_holder_window_duration_secs: Duration,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenericConfig {
    pub token_reward_per_token_per_epoch: u64,
    pub key_types: Vec<CurveType>,
    pub minimum_validator_count: u64,
    pub max_triple_count: u64,
    pub min_triple_count: u64,
    pub peer_checking_interval_secs: u64,
    pub max_triple_concurrency: u64,
    pub rpc_healthcheck_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ComplaintConfig {
    pub tolerance: u64,
    pub interval_secs: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PeersForEpoch {
    pub validators: Vec<PeerValidator>,
    pub epoch_id: String,
    pub epoch_number: u64,
    pub epoch_read_time: std::time::SystemTime,
}

#[derive(Debug, Clone)]
pub struct CachedRootKey {
    pub public_key: String,
    pub curve_type: CurveType,
}

#[derive(Debug)]
pub struct ChainDataConfigManager {
    pub rate_limit_config: RwLock<RateLimitDataConfig>,
    pub peers_for_prior_epoch: RwLock<PeersForEpoch>,
    pub peers_for_current_epoch: RwLock<PeersForEpoch>,
    pub peers_for_next_epoch: RwLock<PeersForEpoch>,
    pub config: ReloadableLitConfig,
    pub root_keys: RwLock<Vec<CachedRootKey>>,
    pub generic_config: RwLock<GenericConfig>,
    pub complaint_reason_to_config: Cache<U256, ComplaintConfig>,
    pub version_requirements: Cache<U256, Version>,
}

impl ChainDataConfigManager {
    pub async fn new(config: ReloadableLitConfig) -> Self {
        let peers_for_epoch = PeersForEpoch {
            validators: Vec::new(),
            epoch_id: "".to_string(),
            epoch_number: 0,
            epoch_read_time: std::time::SystemTime::now(),
        };

        // Set up initial values for the complaint reason.
        let complaint_reason_to_config = Cache::builder().build();
        let default_complaint_config = ComplaintConfig {
            tolerance: 999,
            interval_secs: 60,
        };
        complaint_reason_to_config
            .insert(U256::from(1), default_complaint_config.clone())
            .await;
        complaint_reason_to_config
            .insert(U256::from(2), default_complaint_config.clone())
            .await;

        Self {
            rate_limit_config: RwLock::new(RateLimitDataConfig {
                default_window_duration_secs: Duration::from_secs(30),
                default_max_requests: 2,
                rli_holder_window_duration_secs: Duration::from_secs(30),
            }),
            peers_for_current_epoch: RwLock::new(peers_for_epoch.clone()),
            peers_for_next_epoch: RwLock::new(peers_for_epoch.clone()),
            peers_for_prior_epoch: RwLock::new(peers_for_epoch.clone()),
            config,
            root_keys: RwLock::new(Vec::new()),
            generic_config: RwLock::new(GenericConfig {
                token_reward_per_token_per_epoch: 0,
                key_types: Vec::new(),
                minimum_validator_count: 2,
                max_triple_count: 25,
                min_triple_count: 10,
                peer_checking_interval_secs: 5,
                max_triple_concurrency: 2,
                rpc_healthcheck_enabled: true,
            }),
            complaint_reason_to_config,
            version_requirements: Cache::builder().build(),
        }
    }

    pub async fn watch_chain_data_config(&self, mut quit_rx: mpsc::Receiver<bool>) {
        let interval_delay =
            self.config
                .load()
                .chain_polling_interval_ms()
                .unwrap_or(CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT) as u64;
        let mut interval = tokio::time::interval(Duration::from_millis(interval_delay));

        let mut ticks: u64 = 0;

        info!("Starting: ChainDataConfigManager::watch_chain_data_config");

        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    break;
                }
                _ = interval.tick() => {
                    // Continue below.
                }
            }

            ticks += 1;

            let res = self.set_rate_limit_config_from_chain().await;
            if let Err(e) = res {
                warn!("Error setting rate limit config: {e:?}");
            }

            let res = self.set_peer_and_epoch_data_from_chain().await;
            if let Err(e) = res {
                warn!("Error setting peer and epoch config: {e:?}");
            }

            let res = self.set_root_keys_from_chain().await;
            if let Err(e) = res {
                warn!("Error setting root pubkeys from chain: {e:?}");
            }

            let res = self.set_all_config_from_chain().await;
            if let Err(e) = res {
                warn!("Error setting complaint config from chain: {e:?}");
            }

            let res = self.set_version_requirements(None).await;
            if let Err(e) = res {
                warn!("Error setting version requirements from chain: {e:?}");
            }
        }

        info!("Stopped: ChainDataConfigManager::watch_chain_data_config");
    }

    pub fn get_config_with_resolver(&self) -> Result<(Arc<LitConfig>, ContractResolver)> {
        let config = self.config.load_full();
        let contract_resolver = ContractResolver::try_from(config.as_ref())
            .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;

        Ok((config, contract_resolver))
    }

    pub async fn set_root_keys_from_chain(&self) -> Result<()> {
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let staking_contract = contract_resolver.staking_contract(&config).await?;
        let staking_contract_address = staking_contract.address();
        let contract = contract_resolver.pub_key_router_contract(&config).await?;

        let root_keys: Vec<lit_blockchain::contracts::pubkey_router::RootKey> = contract
            .get_root_keys(staking_contract_address)
            .call()
            .await
            .map_err(|e| blockchain_err(e, Some("Unable to get root keys from contract".into())))?;

        let mut root_pubkey_cache = self.root_keys.write().await;
        root_pubkey_cache.clear();
        for k in root_keys.into_iter() {
            root_pubkey_cache.push(CachedRootKey {
                public_key: bytes_to_hex(&k.pubkey),
                curve_type: CurveType::try_from(k.key_type)?,
            })
        }
        Ok(())
    }

    pub async fn set_beaver_triple_values_from_chain(&self) -> Result<()> {
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let staking_contract = contract_resolver.staking_contract(&config).await?;
        let staking_contract_address = staking_contract.address();
        let contract = contract_resolver.pub_key_router_contract(&config).await?;

        let root_keys: Vec<lit_blockchain::contracts::pubkey_router::RootKey> = contract
            .get_root_keys(staking_contract_address)
            .call()
            .await
            .map_err(|e| blockchain_err(e, Some("Unable to get root keys from contract".into())))?;

        let mut root_pubkey_cache = self.root_keys.write().await;
        root_pubkey_cache.clear();
        for k in root_keys.into_iter() {
            root_pubkey_cache.push(CachedRootKey {
                public_key: bytes_to_hex(&k.pubkey),
                curve_type: CurveType::try_from(k.key_type)?,
            })
        }
        Ok(())
    }

    pub async fn set_rate_limit_config_from_chain(&self) -> Result<()> {
        trace!("set_rate_limit_config_from_chain()");
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let contract = contract_resolver.rate_limit_nft_contract(&config).await?;
        let default_window_duration_secs = contract
            .default_rate_limit_window_seconds()
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get default rate limit window seconds".into()),
                )
            })?
            .as_u64();
        let default_max_requests = contract
            .free_requests_per_rate_limit_window()
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get free requests per rate limit window".into()),
                )
            })?
            .as_u64();
        let rli_holder_window_duration_secs = contract
            .rli_holder_rate_limit_window_seconds()
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get rli holder rate limit window seconds".into()),
                )
            })?
            .as_u64();

        let mut rate_limit_config = self.rate_limit_config.write().await;
        rate_limit_config.default_window_duration_secs =
            Duration::from_secs(default_window_duration_secs);
        rate_limit_config.default_max_requests = default_max_requests;
        rate_limit_config.rli_holder_window_duration_secs =
            Duration::from_secs(rli_holder_window_duration_secs);

        Ok(())
    }

    pub async fn set_peer_and_epoch_data_from_chain(&self) -> Result<()> {
        trace!("set_peer_and_epoch_data_from_chain");
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let staking = contract_resolver.staking_contract(&config).await?;

        let epoch = staking
            .epoch()
            .call()
            .await
            .map_err(|e| blockchain_err(e, Some("Unable to get epoch".into())))?;

        let validators = self.get_sorted_validators(PeerGroupEpoch::Current).await?;

        let epoch_number = epoch.number;
        let epoch_retries = epoch.retries;

        let mut hasher = Sha256::new();
        // we should be able to use validators.join(",") but it complains about
        // unsatisfied trait bounds so let's craft this manually
        let all_validator_addressess = validators
            .iter()
            .map(|v| {
                serde_json::to_string(&v.address)
                    .map_err(|e| conversion_err(e, Some("Unable to serialize validator".into())))
            })
            .collect::<Result<Vec<String>>>()?
            .join(",");

        let to_hash = format!(
            "{}-{}-{}",
            all_validator_addressess, epoch_number, epoch_retries
        );
        trace!(
            "{} Epoch id contents to be hashed: {}",
            config.internal_port()?,
            to_hash
        );
        hasher.update(to_hash.as_bytes());
        let epoch_id = bytes_to_hex(hasher.finalize());
        let epoch_number = epoch.number.as_u64();

        let mut peers_for_current_epoch = self.peers_for_current_epoch.write().await;
        if peers_for_current_epoch.epoch_number < epoch_number {
            let mut peers_for_prior_epoch = self.peers_for_prior_epoch.write().await;
            peers_for_prior_epoch.epoch_id = peers_for_current_epoch.epoch_id.clone();
            peers_for_prior_epoch.epoch_number = peers_for_current_epoch.epoch_number;
            peers_for_prior_epoch.epoch_read_time = peers_for_current_epoch.epoch_read_time;
            peers_for_prior_epoch.validators = peers_for_current_epoch.validators.clone();
            peers_for_current_epoch.epoch_read_time = std::time::SystemTime::now();
        };
        peers_for_current_epoch.validators = validators;
        peers_for_current_epoch.epoch_id = epoch_id.clone();
        peers_for_current_epoch.epoch_number = epoch_number;

        let validators = self.get_sorted_validators(PeerGroupEpoch::Next).await?;
        let mut peers_for_next_epoch = self.peers_for_next_epoch.write().await;
        peers_for_next_epoch.validators = validators;
        peers_for_next_epoch.epoch_id = format!("{}-next", epoch_id);

        if peers_for_next_epoch.epoch_number < epoch_number + 1 {
            // this isn't super meaningful at this point.
            peers_for_next_epoch.epoch_read_time = std::time::SystemTime::now();
        };
        peers_for_next_epoch.epoch_number = epoch_number + 1;

        Ok(())
    }

    pub async fn set_all_config_from_chain(&self) -> Result<()> {
        trace!("set_all_config_from_chain()");
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let contract = contract_resolver.staking_contract(&config).await?;
        let staking_contract_config = contract.config().call().await.map_err(|e| {
            blockchain_err(
                e,
                Some("Unable to get staking contract config from chain.".into()),
            )
        })?;
        let token_reward_per_token_per_epoch = staking_contract_config
            .token_reward_per_token_per_epoch
            .as_u64();

        let key_types = staking_contract_config
            .key_types
            .iter()
            .map(|k| CurveType::try_from(*k).expect("Key Types in Staking Config should be valid."))
            .collect::<Vec<CurveType>>();
        let minimum_validator_count = staking_contract_config.minimum_validator_count.as_u64();
        let max_triple_count = staking_contract_config.max_triple_count.as_u64();
        let min_triple_count = staking_contract_config.min_triple_count.as_u64();
        let peer_checking_interval_secs =
            staking_contract_config.peer_checking_interval_secs.as_u64();
        let max_triple_concurrency = staking_contract_config.max_triple_concurrency.as_u64();
        let rpc_healthcheck_enabled = staking_contract_config.rpc_healthcheck_enabled;

        let mut generic_config = self.generic_config.write().await;
        generic_config.key_types = key_types;
        generic_config.minimum_validator_count = minimum_validator_count;
        generic_config.max_triple_count = max_triple_count;
        generic_config.min_triple_count = min_triple_count;
        generic_config.peer_checking_interval_secs = peer_checking_interval_secs;
        generic_config.max_triple_concurrency = max_triple_concurrency;
        generic_config.rpc_healthcheck_enabled = rpc_healthcheck_enabled;

        // Set complaint configs from chain - we want to fetch for reasons from 1 to MAX_COMPLAINT_REASON_VALUE.
        for i in 1..=MAX_COMPLAINT_REASON_VALUE {
            let complaint_config: staking::ComplaintConfig = contract
                .complaint_config(U256::from(i))
                .call()
                .await
                .map_err(|e| {
                    blockchain_err(
                        e,
                        Some(format!("Unable to get complaint config for reason {}", i)),
                    )
                })?;

            // If the config for this reason has not been set, we want to preserve the current values.
            if complaint_config.tolerance == U256::zero()
                && complaint_config.interval_secs == U256::zero()
            {
                continue;
            }

            // trace!(
            //     "Setting complaint config for reason {} from chain: {:?}",
            //     i,
            //     complaint_config
            // );
            self.complaint_reason_to_config
                .insert(
                    U256::from(i),
                    ComplaintConfig {
                        tolerance: complaint_config.tolerance.as_u64(),
                        interval_secs: complaint_config.interval_secs.as_u64(),
                    },
                )
                .await;
        }

        Ok(())
    }

    /// This function updates the version requirement data that is cached against chain data.
    ///
    /// If `contract_event_data` is `None`, then all version requirements are fetched from the chain.
    /// If `contract_event_data` is `Some`, then only the specific version requirement is fetched from the chain.
    pub async fn set_version_requirements(
        &self,
        contract_event_data: Option<(U256, Version)>,
    ) -> Result<()> {
        trace!("set_version_requirements_from_chain()");

        // If we have detected a version requirement change from a contract event, then we only need to update that version requirement.
        if let Some((version, version_requirement)) = contract_event_data {
            self.version_requirements
                .insert(version, version_requirement)
                .await;
            return Ok(());
        }

        // Otherwise, we need to update all version requirements.
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let contract = contract_resolver.staking_contract(&config).await?;

        let minimum_supported_version: Version =
            contract.get_min_version().call().await.map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get min version from staking contract".into()),
                )
            })?;

        let maximum_supported_version: Version =
            contract.get_max_version().call().await.map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get max version from staking contract".into()),
                )
            })?;

        self.version_requirements
            .insert(U256::from(0), minimum_supported_version)
            .await;
        self.version_requirements
            .insert(U256::from(1), maximum_supported_version)
            .await;

        Ok(())
    }

    pub async fn get_min_version_requirement(&self) -> Result<Version> {
        self.version_requirements
            .get(&U256::from(0))
            .await
            .expect_or_err("Minimum version requirement not found")
    }

    pub async fn get_max_version_requirement(&self) -> Result<Version> {
        self.version_requirements
            .get(&U256::from(1))
            .await
            .expect_or_err("Maximum version requirement not found")
    }

    pub async fn get_sorted_validators(
        &self,
        current_or_next: PeerGroupEpoch,
    ) -> Result<Vec<PeerValidator>> {
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let staking = contract_resolver.staking_contract(&config).await?;

        let validators = match current_or_next {
            PeerGroupEpoch::Current => staking
                .get_validators_structs_in_current_epoch()
                .call()
                .await
                .map_err(|e| {
                    blockchain_err(e, Some("Unable to get validators in current epoch".into()))
                })?,
            PeerGroupEpoch::Next => staking
                .get_validators_structs_in_next_epoch()
                .call()
                .await
                .map_err(|e| {
                    blockchain_err(e, Some("Unable to get validators in next epoch".into()))
                })?,
        };

        let node_addresses = validators
            .iter()
            .map(|v| v.node_address)
            .collect::<Vec<_>>();
        let address_mapping: Vec<AddressMapping> = staking
            .get_node_staker_address_mappings(node_addresses)
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get node and staker addresses in next epoch".into()),
                )
            })?;

        let kicked_nodes = staking.get_kicked_validators().call().await.map_err(|e| {
            blockchain_err(
                e,
                Some("Unable to get kicked validators in next epoch".into()),
            )
        })?;

        let mut peer_validators: Vec<PeerValidator> = validators
            .into_iter()
            .map(|validator| PeerValidator {
                ip: validator.ip,
                ipv6: validator.ipv_6,
                port: validator.port,
                address: validator.node_address,
                reward: validator.reward,
                coms_sender_pub_key: validator.sender_pub_key,
                coms_receiver_pub_key: validator.receiver_pub_key,
                index: 0,
                key_hash: 0,
                socket_addr: get_web_addr_from_chain_info(validator.ip, validator.port),
                staker_address: address_mapping
                    .iter()
                    .find(|am| am.node_address == validator.node_address)
                    .map(|am| am.staker_address)
                    .unwrap_or(H160::zero()),
                is_kicked: kicked_nodes.contains(
                    &address_mapping
                        .iter()
                        .find(|am| am.node_address == validator.node_address)
                        .map(|am| am.staker_address)
                        .unwrap_or(H160::zero()),
                ),
                version: "0.0.0".to_string(),
            })
            .collect();

        peer_validators.sort_by(|a, b| a.staker_address.cmp(&b.staker_address));

        // set the index on each validator
        peer_validators.iter_mut().enumerate().for_each(|(i, pv)| {
            pv.index = i as u16;
            pv.key_hash = Vec::<SimplePeer>::generate_hash(pv.staker_address);
        });

        Ok(peer_validators)
    }
}
