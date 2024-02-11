use super::key_type::{DkgType, KeyType};
use super::models::{NodeTransmissionDetails, RoundData};
use super::traits::cipherable::Cipherable;
use super::traits::dkg::DkgSupport;
use super::traits::epoch_manager::EpochManager;
use super::traits::signable::Signable;
use crate::config::chain::ChainDataConfigManager;
use crate::config::LitNodeConfig;
use crate::error::{unexpected_err, Result};
use crate::peers::PeerState;
use crate::tasks::realtime_metrics::MetricsMessage;
use crate::tss::common::peer_reviewer_bridge;
use crate::tss::common::web::models::SignedMessageShare;
use crate::tss::dkg::gennaro::models::GennaroState;
use crate::tss::ecdsa_cait_sith::CsEcdsaState;
use blsful::inner_types::G1Projective;
use flume::Receiver;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use reqwest::Client;
use std::marker::PhantomData;
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug)]
pub struct TssState {
    pub addr: String,
    pub port: u32,
    pub peer_state: Arc<PeerState>,
    pub lit_config: Arc<LitConfig>,
    pub chain_data_manager: Arc<ChainDataConfigManager>,
    pub tx_round_manager: Arc<flume::Sender<RoundData>>,
    pub tx_batch_manager: Arc<flume::Sender<NodeTransmissionDetails>>,
    pub tx_metrics_manager: Arc<flume::Sender<MetricsMessage>>,
    pub tx_peer_reviewer_bridge: Arc<flume::Sender<peer_reviewer_bridge::Message>>,
    pub http_client: Client,
}

impl Clone for TssState {
    fn clone(&self) -> Self {
        TssState {
            addr: self.addr.clone(),
            port: self.port,
            peer_state: self.peer_state.clone(),
            lit_config: self.lit_config.clone(),
            chain_data_manager: self.chain_data_manager.clone(),
            tx_round_manager: self.tx_round_manager.clone(),
            tx_batch_manager: self.tx_batch_manager.clone(),
            tx_peer_reviewer_bridge: self.tx_peer_reviewer_bridge.clone(),
            http_client: self.http_client.clone(),
            tx_metrics_manager: self.tx_metrics_manager.clone(),
        }
    }
}

impl TssState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        addr: String,
        peer_state: Arc<PeerState>,
        port: String,
        lit_config: Arc<LitConfig>,
        chain_data_manager: Arc<ChainDataConfigManager>,
        tx_round_manager: Arc<flume::Sender<RoundData>>,
        tx_batch_manager: Arc<flume::Sender<NodeTransmissionDetails>>,
        tx_metrics_manager: Arc<flume::Sender<MetricsMessage>>,
        tx_peer_reviewer_bridge: Arc<flume::Sender<peer_reviewer_bridge::Message>>,
        http_client: Client,
    ) -> Result<TssState> {
        Ok(TssState {
            addr: addr.clone(),
            port: port.trim().parse().expect_or_err("Invalid ROCKET Port")?,
            peer_state,
            lit_config,
            chain_data_manager,
            tx_round_manager,
            tx_batch_manager,
            tx_metrics_manager,
            tx_peer_reviewer_bridge,
            http_client,
        })
    }

    #[allow(clippy::type_complexity)]
    pub fn init(
        peer_state: Arc<PeerState>,
        lit_config: Arc<LitConfig>,
        http_client: Client,
        chain_data_manager: Arc<ChainDataConfigManager>,
        tx_metrics_manager: Arc<flume::Sender<MetricsMessage>>,
    ) -> Result<(
        TssState,
        Receiver<RoundData>,
        Receiver<NodeTransmissionDetails>,
        Receiver<crate::tss::common::peer_reviewer_bridge::Message>,
    )> {
        let addr = lit_config
            .external_addr()
            .expect_or_err("No node address set in config.")?; // expect ok, only called from main.rs
        let port = lit_config
            .external_port()
            .expect_or_err("No external port set in config.")? as u32; // expect ok, only called from main.rs

        let (tx_round_manager, rx_round_manager) = flume::unbounded();
        let (tx_batch_manager, rx_batch_manager) = flume::unbounded();
        let (tx_peer_reviewer_bridge, rx_peer_reviewer_bridge) = flume::unbounded();
        let tx_round_manager = Arc::new(tx_round_manager);
        let tx_batch_manager = Arc::new(tx_batch_manager);
        let tx_peer_reviewer_bridge = Arc::new(tx_peer_reviewer_bridge);

        Ok((
            TssState {
                addr: addr.clone(),
                port,
                peer_state,
                lit_config,
                chain_data_manager,
                tx_round_manager,
                tx_batch_manager,
                tx_metrics_manager,
                tx_peer_reviewer_bridge,
                http_client,
            },
            rx_round_manager,
            rx_batch_manager,
            rx_peer_reviewer_bridge,
        ))
    }

    #[instrument]
    pub fn get_signing_state(&self, key_type: KeyType) -> Result<Box<dyn Signable>> {
        let state = self.clone();
        let signing_state = match key_type {
            KeyType::EcdsaCaitSith => Box::new(CsEcdsaState {
                state,
                http_client: self.http_client.clone(),
                dkg_type: DkgType::Standard,
            }) as Box<dyn Signable>,
            _ => {
                return Err(unexpected_err(
                    "Unsupported key type when casting to Signable trait.",
                    None,
                ))
            }
        };

        Ok(signing_state)
    }

    pub fn get_cipher_state(&self, key_type: KeyType) -> Result<Box<dyn Cipherable>> {
        let state = self.clone();
        let _phantom = PhantomData::<G1Projective>;

        let cipher_state = match key_type {
            KeyType::BLS => Box::new(GennaroState::<G1Projective> {
                state,
                http_client: self.http_client.clone(),
                _phantom,
                dkg_type: DkgType::Standard,
            }) as Box<dyn Cipherable>,
            _ => {
                return Err(unexpected_err(
                    "Unsupported key type when casting to Cipherable trait.",
                    None,
                ))
            }
        };

        Ok(cipher_state)
    }

    pub fn get_dkg_state(&self, key_type: KeyType) -> Result<Box<dyn DkgSupport>> {
        let state = self.clone();
        let http_client = self.http_client.clone();
        let _phantom = PhantomData::<G1Projective>;

        let dkg_state = match key_type {
            KeyType::EcdsaCaitSith => Box::new(CsEcdsaState {
                state,
                http_client,
                dkg_type: DkgType::Standard,
            }) as Box<dyn DkgSupport>,
            KeyType::BLS => Box::new(GennaroState::<G1Projective> {
                state,
                http_client,
                _phantom,
                dkg_type: DkgType::Standard,
            }) as Box<dyn DkgSupport>,
        };

        Ok(dkg_state)
    }

    pub fn get_epoch_manager(
        &self,
        key_type: KeyType,
        dkg_type: DkgType,
    ) -> Result<Box<dyn EpochManager>> {
        let state = self.clone();
        let http_client = self.http_client.clone();
        let _phantom = PhantomData::<G1Projective>;

        let epoch_manager = match (key_type, dkg_type) {
            (KeyType::EcdsaCaitSith, dkg_type) => Box::new(CsEcdsaState {
                state,
                http_client,
                dkg_type,
            }) as Box<dyn EpochManager>,
            (KeyType::BLS, dkg_type) => Box::new(GennaroState::<G1Projective> {
                state,
                http_client,
                _phantom,
                dkg_type,
            }) as Box<dyn EpochManager>,
        };

        Ok(epoch_manager)
    }

    pub fn failed_message_share(&self) -> SignedMessageShare {
        SignedMessageShare {
            digest: "fail".to_string(),
            result: "fail".to_string(),
            signature_share: "".to_string(),
            share_index: 0_usize,
            big_r: "".to_string(),
            public_key: "".to_string(),
            sig_type: "".to_string(),
        }
    }
}
