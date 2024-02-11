pub mod peer_item;
pub mod peer_reviewer;
pub mod peer_state;
pub mod utils;

use arc_swap::ArcSwap;
use ethers::prelude::k256::ecdsa::{SigningKey, VerifyingKey};
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::{Address, H160};
use ethers::utils::secret_key_to_address;
use generic_array::GenericArray;
use lit_blockchain::config::LitBlockchainConfig;
use lit_blockchain::contracts::backup_recovery::BackupRecovery;
use lit_blockchain::contracts::staking::Staking;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use log::trace;
use peer_item::PeerData;
use peer_reviewer::PeerComplaint;
use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};
use xor_name::XorName;

use crate::config::{chain::ChainDataConfigManager, LitNodeConfig};
use crate::error::{unexpected_err, unexpected_err_code, Result, EC};
use crate::models::{coms_keys::ComsKeys, PeerValidator};
use crate::tasks::beaver_manager::models::BeaverMessage;
use crate::utils::encoding;

use self::peer_state::models::PeerSocketAddress;

#[derive(Debug)]
pub struct PeerState {
    pub data: ArcSwap<PeerData>,
    pub union_data: ArcSwap<PeerData>, // union of current peers & next peers
    pub curr_data: ArcSwap<PeerData>,  // TODO?: Change to ref instead
    pub peer_id: XorName,
    pub addr: String,
    pub public_key: VerifyingKey,
    pub node_address: Address,
    pub secret_key: SigningKey,
    pub connecting: Mutex<bool>, // flag held while connected_nodes may be changing
    pub staking_address: Address, // address of the contract
    pub staker_address: Address, // address of staking wallet
    pub rpc_url: String,
    pub backup_recovery_contract: BackupRecovery<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub staking_contract: Staking<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub complaint_channel: flume::Sender<PeerComplaint>,
    pub chain_data_config_manager: Arc<ChainDataConfigManager>,
    pub comskeys: ComsKeys,
    pub lit_config: Arc<LitConfig>,
    pub http_client: reqwest::Client,
    pub bm_tx: flume::Sender<BeaverMessage>,
}

#[allow(dead_code)]
impl PeerState {
    pub async fn new(
        addr: String,
        complaint_channel: flume::Sender<PeerComplaint>,
        cfg: Arc<LitConfig>,
        chain_data_config_manager: Arc<ChainDataConfigManager>,
        http_client: reqwest::Client,
        bm_tx: flume::Sender<BeaverMessage>,
    ) -> Result<PeerState> {
        let resolver = ContractResolver::try_from(cfg.as_ref())
            .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;

        let secret_key = SigningKey::from_bytes(GenericArray::from_slice(
            &encoding::hex_to_bytes(cfg.blockchain_wallet_private_key(None)?)
                .expect_or_err("Failed to hex encode node.private_key config var")?,
        ))
        .expect_or_err("Could not convert node.private_key config var to SigningKey")?;

        let node_address = secret_key_to_address(&secret_key);
        let public_key = secret_key.verifying_key();

        let backup_recovery_contract = resolver.backup_recovery_contract_with_signer(&cfg).await?;
        let staking_contract = resolver.staking_contract_with_signer(&cfg).await?;
        let staker_address: H160 = cfg
            .staker_address()?
            .parse()
            .expect_or_err("failed to parse staker_address")?;

        Ok(PeerState {
            data: ArcSwap::from(Arc::new(PeerData::new())),
            curr_data: ArcSwap::from(Arc::new(PeerData::new())),
            union_data: ArcSwap::from(Arc::new(PeerData::new())),
            peer_id: XorName::from_content(addr.clone().as_bytes()),
            addr,
            public_key: *public_key,
            node_address,
            secret_key,
            connecting: Mutex::new(false),
            staking_address: staking_contract.address(),
            staker_address,
            rpc_url: cfg.rpc_url()?,
            backup_recovery_contract,
            staking_contract,
            complaint_channel,
            chain_data_config_manager,
            comskeys: ComsKeys::from_node_config(&cfg)?,
            lit_config: cfg,
            http_client,
            bm_tx,
        })
    }

    pub fn node_address(&self) -> Address {
        self.node_address
    }

    pub async fn epoch(&self) -> u64 {
        let peer_data = self
            .chain_data_config_manager
            .peers_for_current_epoch
            .read()
            .await;
        peer_data.epoch_number
    }

    pub async fn node_index_in_current_epoch(&self) -> Result<usize> {
        Ok(
            match self
                .validators_in_current_epoch()
                .await?
                .into_iter()
                .find(|v| v.address == self.node_address)
            {
                Some(v) => v.index as usize,
                None => {
                    error!(
                        "Failed to find validator with address: {:?}",
                        self.node_address
                    );
                    return Err(unexpected_err(
                        "Failed to find validator with address",
                        None,
                    ));
                }
            },
        )
    }

    pub async fn validators_in_current_epoch(&self) -> Result<Vec<PeerValidator>> {
        let peer_data = self
            .chain_data_config_manager
            .peers_for_current_epoch
            .read()
            .await;
        Ok(peer_data.validators.clone())
    }

    pub async fn validators_in_next_epoch(&self) -> Result<Vec<PeerValidator>> {
        let peer_data = self
            .chain_data_config_manager
            .peers_for_next_epoch
            .read()
            .await;
        Ok(peer_data.validators.clone())
    }

    pub async fn peer_node_addresses(&self) -> Result<Vec<Address>> {
        Ok(self
            .validators_in_current_epoch()
            .await?
            .iter()
            .map(|peer| peer.address)
            .collect())
    }

    pub async fn peer_socket_addresses(&self) -> Result<Vec<PeerSocketAddress>> {
        Ok(self
            .validators_in_current_epoch()
            .await?
            .iter()
            .map(|peer| PeerSocketAddress {
                address: peer.socket_addr.clone(),
                share_index: peer.index,
                kicked: peer.is_kicked,
                key_hash: peer.key_hash,
            })
            .collect())
    }

    pub async fn peer_socket_addresses_in_next_epoch(&self) -> Result<Vec<PeerSocketAddress>> {
        Ok(self
            .validators_in_next_epoch()
            .await?
            .iter()
            .map(|peer| PeerSocketAddress {
                address: peer.socket_addr.clone(),
                share_index: peer.index,
                kicked: peer.is_kicked,
                key_hash: peer.key_hash,
            })
            .collect())
    }

    // get a single Validator struct
    pub async fn get_validator_from_node_address(
        &self,
        node_address: Address,
    ) -> Result<PeerValidator> {
        Ok(
            match self
                .get_current_and_next_validators()
                .await?
                .into_iter()
                .find(|v| v.address == node_address)
            {
                Some(v) => v,
                None => {
                    error!(
                        "get_validator:Failed to find validator with address: {:?}",
                        node_address
                    );
                    return Err(unexpected_err(
                        "Failed to find validator with address",
                        None,
                    ));
                }
            },
        )
    }

    async fn get_current_and_next_validators(&self) -> Result<Vec<PeerValidator>> {
        let mut validators = self.validators_in_current_epoch().await?;
        let mut next_validators = self.validators_in_next_epoch().await?;
        validators.append(&mut next_validators);
        Ok(validators)
    }

    pub async fn get_staker_address_from_socket_addr(&self, addr: &str) -> Result<Address> {
        let validator = self
            .get_current_and_next_validators()
            .await?
            .into_iter()
            .find(|v| v.socket_addr == addr);

        match validator {
            Some(v) => {
                info!(
                    "Validator with socket addr: {:?} has staker address: {:?}",
                    addr, v.staker_address
                );
                Ok(v.staker_address)
            }
            None => {
                error!("Failed to find validator with socket addr: {:?}", addr);
                Err(unexpected_err("Failed to find validator with ip", None))
            }
        }
    }

    pub async fn connect_to_validators_union(&self) -> Result<()> {
        let vin_current = BTreeSet::from_iter(self.validators_in_current_epoch().await?);
        let vin_next = BTreeSet::from_iter(self.validators_in_next_epoch().await?);
        let vi_union_of_epochs = vin_current.union(&vin_next);

        let validators_in_union: Vec<PeerValidator> = vi_union_of_epochs
            .filter(|v| !v.is_kicked)
            .cloned()
            .collect();
        debug!("Validators in union: {:?}", validators_in_union);

        self.find_peers_ext(validators_in_union, true).await?;
        let connected_nodes = self.data.load().table.clone(); // reload since we modified it
        let connected_ethaddrs_after =
            BTreeSet::from_iter(connected_nodes.iter().map(|n| n.staker_address));

        trace!(
            "connect_to_validators_union: {:?}",
            connected_ethaddrs_after
        );

        Ok(())
    }

    pub async fn next_epoch_validators_communicating(&self) -> Result<bool> {
        let next_validators = self.validators_in_next_epoch().await.map_err(|e| {
            error!("[lock validators] Failed to get validators for next epoch from chain.");
            e
        })?;

        match self.find_peers(next_validators).await {
            Ok(_) => Ok(true),
            Err(e) => {
                error!("[lock validators] Failed to communicate with next epoch peers as a pre-requisite for locking the validator set.");
                Ok(false)
            }
        }
    }
}
