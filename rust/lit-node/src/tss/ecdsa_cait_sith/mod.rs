pub mod backup;
pub mod execution;
pub mod models;
pub mod protocols;
pub mod protocols256k1;

use super::common::traits::epoch_manager::EpochManager;
use super::common::traits::signable::Signable;
use crate::error::{unexpected_err, unexpected_err_code, Result, EC};
use crate::p2p_comms::web::models::SignedMessageShare;
use crate::peers::peer_state::models::{SimplePeer, SimplePeerExt};
use crate::peers::utils::derministic_subset::DeterministicSubset;
#[cfg(feature = "rtmetrics")]
use crate::tasks::realtime_metrics::{MetricAction, MetricActionType, MetricsMessage::NewAction};
use crate::tss::common::tss_state::TssState;
use crate::tss::common::{curve_type::CurveType, dkg_type::DkgType};
use elliptic_curve::sec1::ToEncodedPoint;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use tracing::instrument;

#[derive(Debug)]
pub struct CsEcdsaState {
    pub state: TssState,
    pub dkg_type: DkgType,
}

impl Clone for CsEcdsaState {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            dkg_type: self.dkg_type,
        }
    }
}

impl CsEcdsaState {
    pub fn new(state: TssState) -> Self {
        Self::new_with_dkg_type(state, DkgType::Standard)
    }

    pub fn new_with_dkg_type(state: TssState, dkg_type: DkgType) -> Self {
        CsEcdsaState { state, dkg_type }
    }

    pub async fn get_epoch(
        &self,
        message_bytes: &[u8],
        public_key: &[u8],
        request_id: &[u8],
    ) -> u64 {
        self.state.peer_state.epoch().await
    }

    async fn sign_with_pubkey_internal(
        &mut self,
        message_bytes: &[u8],
        public_key: Vec<u8>,
        root_pubkeys: Option<Vec<String>>,
        tweak_preimage: Option<Vec<u8>>,
        request_id: Vec<u8>,
        epoch: Option<u64>,
    ) -> Result<SignedMessageShare> {
        debug!(
            "sign_with_pubkey() with public_key {:?} and tweak_preimage {:?}",
            bytes_to_hex(&public_key),
            match tweak_preimage.clone() {
                Some(t) => bytes_to_hex(t),
                None => "None".to_string(),
            }
        );

        let root_pubkeys = root_pubkeys.expect_or_err("No root pubkeys provided!")?;
        let tweak_preimage = tweak_preimage.expect_or_err("No hd_key_id provided!")?;
        // note that this epoch call is used to look only at some internal key files - not to interact with other nodes, so it is safe to do.

        let self_epoch = self.state.peer_state.epoch().await;

        let epoch = match epoch {
            Some(e) => match e > self_epoch {
                true => {
                    warn!(
                        "Requested epoch is in the future. Using current epoch: {}",
                        self_epoch
                    );
                    self_epoch
                }
                false => e,
            },
            None => self_epoch,
        };

        let (epoch_number, peers) = match self_epoch - epoch {
            0 => (epoch, self.state.peer_state.peers().await?),
            1 => (epoch, self.state.peer_state.peers_in_prior_epoch().await?),
            _ => (self_epoch, self.state.peer_state.peers().await?),
        };

        let ds = DeterministicSubset::new(&self.state.peer_state, epoch_number).await;
        let all_peers = ds.all_peers.clone();

        let threshold = self
            .get_hd_key_threshold(&root_pubkeys[0], &all_peers, epoch_number)
            .await?;

        let peers = ds.get_subset(message_bytes, &request_id, threshold as usize)?;

        let psa = &self.state.peer_state.peers().await?;
        trace!(
            "Nodes (all, active,subset): \n {:?} \n {:?} \n {:?}",
            psa.debug_addresses(),
            psa.active_peers().debug_addresses(),
            peers.debug_addresses()
        );

        if peers.is_empty() {
            return Err(unexpected_err(
                "No peers available to sign message!".to_string(),
                None,
            ));
        }

        let message = message_bytes;

        let min_peer_version = all_peers.min_version_in_group();
        let min_version = semver::Version::new(0, 2, 15);
        let txn_prefix = match min_peer_version.gt(&min_version) {
            false => String::from_utf8(request_id.clone()).map_err(|e| {
                unexpected_err_code(
                    e,
                    EC::NodeUnknownError,
                    Some("Error converting request id to string".to_string()),
                )
            })?,
            true => {
                let mut txn_prefix_bytes = request_id.clone();
                // the message itself can be anything, including invalid UTF8 bytes.  so we convert it to hex, and then send those bytes in.  When those get converted back to string, they will be valid UTF8 hex characters.
                txn_prefix_bytes.extend_from_slice(bytes_to_hex(message).as_bytes());
                String::from_utf8(txn_prefix_bytes).map_err(|e| {
                    unexpected_err_code(
                        e,
                        EC::NodeUnknownError,
                        Some("Error converting request id to string".to_string()),
                    )
                })?
            }
        };

        let txn_prefix = &txn_prefix;
        // get a triple pair, even if we're not part of the signing set - this is needed to trigger BT Pregeneration in certain cases.
        let enable_triple_pregen = true;
        let triple_pair = self
            .get_triple_pair(
                message_bytes,
                public_key.clone(),
                request_id,
                txn_prefix,
                threshold,
                peers.clone(),
                enable_triple_pregen,
            )
            .await;

        // Exit, if we're not part of the signing set.
        if !peers.contains_address(&self.state.addr) {
            trace!("Peers doesn't contain self, returning empty failed message share");
            return Ok(self.failed_message_share());
        }

        let triple_pair = match triple_pair? {
            Some(tp) => tp,
            None => {
                error!("Triple pair is none, though peers contains this node address!");
                return Ok(self.failed_message_share());
            }
        };
        debug!("Got triple pair for during signing: {}", txn_prefix);

        let txn_params = models::ProtocolTransactionParams::from_config(
            self.protocol_params_with_options(txn_prefix, Some(peers.clone()), None)
                .await?
                .into(),
        );

        let sig_share = self
            .generate_hd_key_signature_share_from_key_id(
                &txn_params,
                message,
                &tweak_preimage,
                root_pubkeys,
                &triple_pair,
                epoch_number,
            )
            .await
            .map_err(|e| {
                error!(
                    "Error generating signature share for hd_key_id: {:?}, {:?}",
                    bytes_to_hex(tweak_preimage),
                    e
                );
                unexpected_err_code(e, EC::NodeUnknownError, None)
            })?;

        let signature_share = SignedMessageShare {
            signature_share: serde_json::to_string(&sig_share.share)
                .map_err(|e| unexpected_err_code(e, EC::NodeUnknownError, None))?,
            public_key: serde_json::to_string(&sig_share.public_key.to_encoded_point(false))
                .map_err(|e| unexpected_err_code(e, EC::NodeUnknownError, None))?,
            result: "success".to_string(),
            share_index: 0,
            big_r: serde_json::to_string(&sig_share.presignature_big_r)
                .map_err(|e| unexpected_err_code(e, EC::NodeUnknownError, None))?, // just shoving stuff into a known structure - to be refactored.
            digest: serde_json::to_string(&sig_share.msg_hash)
                .map_err(|e| unexpected_err_code(e, EC::NodeUnknownError, None))?,
            sig_type: CurveType::K256.to_string(),
        };

        debug!("Signature share pubkey: {:?}", signature_share.public_key);

        Ok(signature_share)
    }

    fn failed_message_share(&self) -> SignedMessageShare {
        self.state.failed_message_share()
    }
}

#[async_trait::async_trait]
impl Signable for CsEcdsaState {
    #[doc = "Sign using the default ECDSA key generated via DKG"]
    #[instrument]
    async fn sign(
        &self,
        message_bytes: Vec<u8>,
        nonce_bytes: Vec<u8>,
        epoch: Option<u64>,
    ) -> Result<SignedMessageShare> {
        Ok(self.failed_message_share())
    }

    #[doc = "Sign using a specifically identified public key.  This pubkey is the result of PKP generation."]
    #[instrument(skip_all)]
    async fn sign_with_pubkey(
        &mut self,
        message_bytes: &[u8],
        public_key: Vec<u8>,
        root_pubkeys: Option<Vec<String>>,
        tweak_preimage: Option<Vec<u8>>,
        request_id: Vec<u8>,
        epoch: Option<u64>,
    ) -> Result<SignedMessageShare> {
        let tx_metrics = self.state.tx_metrics_manager.clone();
        let txn_id = Vec::<SimplePeer>::generate_hash(request_id.clone());
        #[cfg(feature = "rtmetrics")]
        let _ = tx_metrics
            .send_async(NewAction(MetricAction {
                type_id: MetricActionType::SignEcdsa,
                txn_id,
                is_start: true,
                is_success: true,
            }))
            .await;

        self.sign_with_pubkey_internal(
            message_bytes,
            public_key,
            root_pubkeys,
            tweak_preimage,
            request_id,
            epoch,
        )
        .await

        // removed the final rt_metric call as clippy was complaining when it's compiled out, and it's deprecated in another PR.
    }
}

#[async_trait::async_trait]
impl EpochManager for CsEcdsaState {}
