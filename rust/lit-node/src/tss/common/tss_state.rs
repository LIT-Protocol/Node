use super::models::{NodeTransmissionDetails, RoundData};
use super::traits::cipherable::Cipherable;
use super::traits::dkg::BasicDkg;
use super::traits::epoch_manager::EpochManager;
use super::traits::signable::Signable;
use crate::config::chain::ChainDataConfigManager;
use crate::config::LitNodeConfig;
use crate::error::{unexpected_err, Result};
use crate::p2p_comms::web::models::SignedMessageShare;
use crate::peers::PeerState;
use crate::tasks::realtime_metrics::MetricsMessage;
use crate::tss::blsful::models::BlsState;
use crate::tss::common::{curve_type::CurveType, dkg_type::DkgType};
use crate::tss::ecdsa_cait_sith::CsEcdsaState;
use crate::tss::frost::FrostState;
use blsful::inner_types::G1Projective;
use flume::Receiver;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use std::sync::Arc;
use tracing::instrument;

const EPOCH_CHANGE_BUFFER_SECS: u64 = 60;

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
        })
    }

    #[allow(clippy::type_complexity)]
    pub fn init(
        peer_state: Arc<PeerState>,
        lit_config: Arc<LitConfig>,
        chain_data_manager: Arc<ChainDataConfigManager>,
        tx_metrics_manager: Arc<flume::Sender<MetricsMessage>>,
    ) -> Result<(
        TssState,
        Receiver<RoundData>,
        Receiver<NodeTransmissionDetails>,
    )> {
        let addr = lit_config
            .external_addr()
            .expect_or_err("No node address set in config.")?; // expect ok, only called from main.rs
        let port = lit_config
            .external_port()
            .expect_or_err("No external port set in config.")? as u32; // expect ok, only called from main.rs

        let (tx_round_manager, rx_round_manager) = flume::unbounded();
        let (tx_batch_manager, rx_batch_manager) = flume::unbounded();
        let tx_round_manager = Arc::new(tx_round_manager);
        let tx_batch_manager = Arc::new(tx_batch_manager);

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
            },
            rx_round_manager,
            rx_batch_manager,
        ))
    }

    #[instrument]
    pub fn get_signing_state(&self, curve_type: CurveType) -> Result<Box<dyn Signable>> {
        let state = self.clone();
        let signing_state = match curve_type {
            CurveType::K256 => Box::new(CsEcdsaState::new(state)) as Box<dyn Signable>,
            _ => {
                return Err(unexpected_err(
                    "Unsupported key type when for Signable.",
                    None,
                ))
            } // Upcoming!
              // CurveType::BLS => Box::new(BlsState::<G1Projective>::new(state)) as Box<dyn Signable>,
              // CurveType::P256=> Box::new(FrostState::<p256::ProjectivePoint>::new(state)) as Box<dyn Signable>,
              // CurveType::P384=> Box::new(FrostState::<p384::ProjectivePoint>::new(state)) as Box<dyn Signable>,
              // CurveType::Ed25519=> Box::new(FrostState::<curve25519_dalek::edwards::SubgroupPoint>::new(state)) as Box<dyn Signable>,
              // CurveType::Ristretto25519=> Box::new(FrostState::<curve25519_dalek::RistrettoPoint>::new(state)) as Box<dyn Signable>,
              // CurveType::Ed448=> Box::new(FrostState::<ed448_goldilocks::EdwardsPoint>::new(state)) as Box<dyn Signable>,
              // CurveType::RedJubjub=> Box::new(FrostState::<redjubjub::SubgroupPoint>::new(state)) as Box<dyn Signable>,
        };

        Ok(signing_state)
    }

    pub fn get_cipher_state(&self, curve_type: CurveType) -> Result<Box<dyn Cipherable>> {
        let state = self.clone();
        let cipher_state = match curve_type {
            CurveType::BLS => Box::new(BlsState::<G1Projective>::new(state)) as Box<dyn Cipherable>,
            _ => {
                return Err(unexpected_err(
                    "Unsupported key type when casting to Cipherable trait.",
                    None,
                ))
            }
        };

        Ok(cipher_state)
    }

    pub fn get_dkg_state(&self, curve_type: CurveType) -> Result<Box<dyn BasicDkg>> {
        let state = self.clone();
        let dkg_state = match curve_type {
            CurveType::K256 => Box::new(CsEcdsaState::new(state)) as Box<dyn BasicDkg>,
            CurveType::BLS => Box::new(BlsState::<G1Projective>::new(state)) as Box<dyn BasicDkg>,
            CurveType::P256 => {
                Box::new(FrostState::<p256::ProjectivePoint>::new(state)) as Box<dyn BasicDkg>
            }
            CurveType::P384 => {
                Box::new(FrostState::<p384::ProjectivePoint>::new(state)) as Box<dyn BasicDkg>
            }
            CurveType::Ed25519 => Box::new(
                FrostState::<curve25519_dalek::edwards::SubgroupPoint>::new(state),
            ) as Box<dyn BasicDkg>,
            CurveType::Ristretto25519 => {
                Box::new(FrostState::<curve25519_dalek::RistrettoPoint>::new(state))
                    as Box<dyn BasicDkg>
            }
            CurveType::Ed448 => Box::new(FrostState::<ed448_goldilocks::EdwardsPoint>::new(state))
                as Box<dyn BasicDkg>,
            CurveType::RedJubjub => {
                Box::new(FrostState::<jubjub::SubgroupPoint>::new(state)) as Box<dyn BasicDkg>
            }
        };

        Ok(dkg_state)
    }

    pub fn get_epoch_manager(
        &self,
        curve_type: CurveType,
        dkg_type: DkgType,
    ) -> Result<Box<dyn EpochManager>> {
        let state = self.clone();
        let epoch_manager = match curve_type {
            CurveType::K256 => {
                Box::new(CsEcdsaState::new_with_dkg_type(state, dkg_type)) as Box<dyn EpochManager>
            }
            CurveType::BLS => {
                Box::new(BlsState::<G1Projective>::new_with_dkg_type(state, dkg_type))
                    as Box<dyn EpochManager>
            }
            CurveType::P256 => Box::new(FrostState::<p256::ProjectivePoint>::new_with_dkg_type(
                state, dkg_type,
            )) as Box<dyn EpochManager>,
            CurveType::P384 => Box::new(FrostState::<p384::ProjectivePoint>::new_with_dkg_type(
                state, dkg_type,
            )) as Box<dyn EpochManager>,
            CurveType::Ed25519 => Box::new(
                FrostState::<curve25519_dalek::edwards::SubgroupPoint>::new_with_dkg_type(
                    state, dkg_type,
                ),
            ) as Box<dyn EpochManager>,
            CurveType::Ristretto25519 => Box::new(
                FrostState::<curve25519_dalek::RistrettoPoint>::new_with_dkg_type(state, dkg_type),
            ) as Box<dyn EpochManager>,
            CurveType::Ed448 => Box::new(
                FrostState::<ed448_goldilocks::EdwardsPoint>::new_with_dkg_type(state, dkg_type),
            ) as Box<dyn EpochManager>,
            CurveType::RedJubjub => Box::new(
                FrostState::<jubjub::SubgroupPoint>::new_with_dkg_type(state, dkg_type),
            ) as Box<dyn EpochManager>,
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

    pub async fn get_keyshare_epoch(&self) -> u64 {
        let current_peers = self.chain_data_manager.peers_for_current_epoch.read().await;
        // 2 is actually the first epoch (no keys are present in epoch 1)
        if current_peers.epoch_number <= 2 {
            return 2;
        };

        let elapsed =
            match std::time::SystemTime::now().duration_since(current_peers.epoch_read_time) {
                Ok(elapsed) => elapsed.as_secs(),
                Err(e) => {
                    error!("Error getting elapsed time: {:?}", e);
                    EPOCH_CHANGE_BUFFER_SECS + 1 // automatically use the current epoch?  This probably shouldn't happen.
                }
            };

        // if we've held the epoch in memory for at least 60 seconds, use the current epoch, otherwise use the prior one.
        if elapsed > EPOCH_CHANGE_BUFFER_SECS {
            current_peers.epoch_number
        } else {
            current_peers.epoch_number - 1
        }
    }
}
