pub mod backup;
pub mod execution;
pub mod gennaro_dkg;
pub mod models;
pub mod protocols;
pub mod protocols256k1;
use crate::config::LitNodeConfig;
use crate::error::{unexpected_err, unexpected_err_code, Result, EC};
use crate::peers::peer_state::models::{PeerSocketAddress, PeerSocketAddressVec};
use crate::peers::utils::derministic_subset::DeterministicSubset;
use crate::tasks::beaver_manager::models::generate_hash;
#[cfg(feature = "rtmetrics")]
use crate::tasks::realtime_metrics::{MetricAction, MetricActionType, MetricsMessage::NewAction};
use crate::tss::common::key_type::{DkgType, KeyType};
use elliptic_curve::sec1::ToEncodedPoint;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use reqwest::Client;
use std::sync::Arc;
use tracing::instrument;

use super::common::traits::dkg::DkgSupport;
use super::common::traits::epoch_manager::EpochManager;
use super::common::traits::signable::Signable;
use super::common::web::models::SignedMessageShare;
use crate::tss::common::tss_state::TssState;

use super::dkg::gennaro::models::GennaroState;

#[derive(Debug)]
pub struct CsEcdsaState {
    pub state: TssState,
    pub http_client: Client,
    pub dkg_type: DkgType,
}

impl Clone for CsEcdsaState {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            dkg_type: self.dkg_type,
        }
    }
}

impl CsEcdsaState {
    fn get_gennaro_state(&self) -> GennaroState<k256::ProjectivePoint> {
        let _phantom = std::marker::PhantomData::<k256::ProjectivePoint>;
        GennaroState::<k256::ProjectivePoint> {
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            _phantom,
            dkg_type: self.dkg_type,
        }
    }

    async fn sign_with_pubkey_internal(
        &mut self,
        message_bytes: &[u8],
        public_key: Vec<u8>,
        root_pubkeys: Option<Vec<String>>,
        tweak_preimage: Option<Vec<u8>>,
        request_id: Vec<u8>,
    ) -> Result<SignedMessageShare> {
        debug!(
            "sign_with_pubkey() with public_key {:?} and tweak_preimage {:?}",
            bytes_to_hex(&public_key),
            tweak_preimage
        );

        let root_pubkeys = root_pubkeys.expect_or_err("No root pubkeys provided!")?;
        let tweak_preimage = tweak_preimage.expect_or_err("No hd_key_id provided!")?;
        let epoch = self.state.peer_state.epoch().await;
        let threshold = self.get_hd_key_threshold(&root_pubkeys[0], epoch).await?;
        let ds = DeterministicSubset::new(&self.state.peer_state).await;
        let peers = ds.get_subset(message_bytes, &request_id, threshold as usize)?;

        trace!(
            "All nodes: {:?}",
            &self
                .state
                .peer_state
                .peer_socket_addresses()
                .await?
                .all_addresses()
        );
        trace!(
            "Active nodes: {:?}",
            &self
                .state
                .peer_state
                .peer_socket_addresses()
                .await?
                .active_addresses()
        );

        trace!("Signing subset: {:?}", peers);
        if peers.is_empty() {
            return Err(unexpected_err(
                "No peers available to sign message!".to_string(),
                None,
            ));
        }

        let message = message_bytes;
        let txn_prefix = &String::from_utf8(request_id.clone()).map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeUnknownError,
                Some("Error converting request id to string".to_string()),
            )
        })?;

        let txn_params = models::ProtocolTransactionParams::from_config(
            self.protocol_params_with_options(txn_prefix, Some(peers.clone()), None)
                .await?,
        );

        let triple_pair = self
            .get_triple_pair(
                message_bytes,
                public_key.clone(),
                request_id,
                txn_prefix,
                threshold,
                peers.clone(),
            )
            .await?;
        debug!("Got triple pair during signing: {}", txn_prefix);

        // FIXME/TODO: we wait until here to check if the peers contains the current node, in order to use existing BT generation code, if a BT is needed.
        // It would be better to do this earlier, and generate a BT specific to just this transactions, however, if we need BTs for ALL nodes, this is
        // our trigger, and must include all nodes.
        if !peers.contains_address(&self.addr()) {
            trace!("Peers doesn't contain self, returning empty failed message share");
            return Ok(self.failed_message_share());
        }

        let triple_pair = match triple_pair {
            Some(tp) => tp,
            None => {
                error!("Triple pair is none, though peers contains this node address!");
                return Ok(self.failed_message_share());
            }
        };

        let sig_share = self
            .generate_hd_key_signature_share_from_key_id(
                txn_params,
                message,
                &tweak_preimage,
                root_pubkeys,
                triple_pair,
            )
            .await
            .map_err(|e| {
                error!(
                    "Error generating signature share for hd_key_id: {:?}, {:?}",
                    tweak_preimage, e
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
            sig_type: KeyType::EcdsaCaitSith.to_string(),
        };

        debug!("Signature share pubkey: {:?}", signature_share.public_key);

        Ok(signature_share)
    }

    fn failed_message_share(&self) -> SignedMessageShare {
        self.state.failed_message_share()
    }
}

#[async_trait::async_trait]
impl DkgSupport for CsEcdsaState {
    #[doc = "Generate a new ECDSA key share using DKG"]
    async fn keygen(
        &self,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<PeerSocketAddress>,
        dkg_type: DkgType,
    ) -> Result<String> {
        let txn_prefix = format!("DKG-{}", dkg_id);
        let gennaro_state = self.get_gennaro_state();
        let pubkey = gennaro_state.keygen(dkg_id, epoch, peers, dkg_type).await?;
        Ok(pubkey)
    }

    #[doc = "Refresh an existing ECDSA key share amoung the same node set"]
    async fn refresh(
        &self,
        pubkey: &str,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<PeerSocketAddress>,
    ) -> Result<bool> {
        let gennaro_state = self.get_gennaro_state();
        let result = gennaro_state.refresh(pubkey, dkg_id, epoch, peers).await?;
        Ok(result)
    }

    #[doc = "Reshare an existing ECDSA key share  with a new set of nodes"]
    async fn reshare(
        &self,
        pubkey: &str,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<PeerSocketAddress>,
        next_peers: &Vec<PeerSocketAddress>,
    ) -> Result<bool> {
        let gennaro_state = self.get_gennaro_state();
        let result = gennaro_state
            .reshare(pubkey, dkg_id, epoch, peers, next_peers)
            .await?;
        Ok(result)
    }

    #[doc = "Get the current nodes address."]
    fn addr(&self) -> String {
        self.state.addr.clone()
    }

    async fn peer_socket_addresses(&self) -> Vec<PeerSocketAddress> {
        match self.state.peer_state.peer_socket_addresses().await {
            Ok(addrs) => addrs,
            Err(e) => {
                error!("Error getting connected node addresses: {:?}", e);
                vec![]
            }
        }
    }

    fn lit_config(&self) -> Arc<LitConfig> {
        self.state.lit_config.clone()
    }

    #[instrument]
    async fn root_keys(&self) -> Vec<String> {
        let crk = self.state.chain_data_manager.root_keys.read().await.clone();
        let key_type = self.key_type();
        crk.iter()
            .filter(|k| k.key_type == key_type)
            .map(|k| k.public_key.clone())
            .collect()
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
    ) -> Result<SignedMessageShare> {
        let tx_metrics = self.state.tx_metrics_manager.clone();
        let txn_id = generate_hash(request_id.clone());
        #[cfg(feature = "rtmetrics")]
        let _ = tx_metrics
            .send_async(NewAction(MetricAction {
                type_id: MetricActionType::SignEcdsa,
                txn_id,
                is_start: true,
                is_success: true,
            }))
            .await;

        let signature_share = self
            .sign_with_pubkey_internal(
                message_bytes,
                public_key,
                root_pubkeys,
                tweak_preimage,
                request_id,
            )
            .await;

        #[cfg(feature = "rtmetrics")]
        let _ = tx_metrics
            .send_async(NewAction(MetricAction {
                type_id: MetricActionType::SignEcdsa,
                txn_id,
                is_start: false,
                is_success: signature_share.is_ok(),
            }))
            .await;

        signature_share
    }
}

#[async_trait::async_trait]
impl EpochManager for CsEcdsaState {
    #[doc = "Change the epoch of the DKG.  This will either reshare or refresh the DKG depending on the node set, managing as many DKGs as needed to cover the new node set."]
    async fn change_epoch(
        &self,
        dkg_id: String,
        epoch: u64,
        cfg: &LitConfig,
        current_peers: &Vec<PeerSocketAddress>,
        new_peers: &Vec<PeerSocketAddress>,
    ) -> Result<(bool, Vec<String>)> {
        if new_peers.is_empty() {
            return Err(unexpected_err(
                "No new peers to launch DKG with!".to_string(),
                None,
            ));
        };

        let tx_metrics = self.state.tx_metrics_manager.clone();

        info!(
            "Doing Epoch DKG: {} {:?} {:?}",
            dkg_id, current_peers, new_peers
        );
        debug!("dkg_type- {:?}", self.dkg_type);
        let start = std::time::Instant::now();

        // this is the first time we are launching DKG to generate HD root keys.
        if current_peers.is_empty() {
            let hd_root_key_count = match self.dkg_type {
                DkgType::RecoveryParty => 1,
                _ => cfg.ecdsa_root_pubkey_count()?,
            };

            info!("Launching {} initial ECDSA DKGs!", hd_root_key_count);

            let mut hd_root_pubkeys: Vec<String> = Vec::new();

            for i in 0..hd_root_key_count {
                let dkg_id = format!("{}_key_{}", dkg_id, i);

                #[cfg(feature = "rtmetrics")]
                let _ = tx_metrics
                    .send_async(NewAction(MetricAction {
                        type_id: MetricActionType::SignEcdsa,
                        txn_id: generate_hash(dkg_id.clone()),
                        is_start: true,
                        is_success: true,
                    }))
                    .await;

                let public_key = self.keygen(&dkg_id, epoch, new_peers, self.dkg_type).await;

                #[cfg(feature = "rtmetrics")]
                let _ = tx_metrics
                    .send_async(NewAction(MetricAction {
                        type_id: MetricActionType::SignEcdsa,
                        txn_id: generate_hash(dkg_id.clone()),
                        is_start: false,
                        is_success: public_key.is_ok(),
                    }))
                    .await;

                let public_key = public_key?;

                info!(
                    "DKG Completed - Generated key #{} as pubkey {} in {:?} ms",
                    i + 1,
                    public_key,
                    start.elapsed().as_millis()
                );

                hd_root_pubkeys.push(public_key);
            }

            info!("Generated {} HD root keys!", hd_root_pubkeys.len());
            return Ok((true, hd_root_pubkeys));
        }

        let existing_root_keys = self.root_keys().await;

        for pubkey in existing_root_keys {
            let dkg_id = format!("{}_{}", dkg_id, pubkey);
            if current_peers == new_peers {
                info!(
                    "Launching Epoch DKG with same nodes - refresh key shares for {}!",
                    &pubkey
                );
                let r = self.refresh(&pubkey, &dkg_id, epoch, new_peers).await?;
                info!("DKG Completed - refreshed ECDSA key {} = {} ", &pubkey, r);
            } else {
                info!(
                    "Launching Epoch DKG with new nodes - resharing key shares for {}!",
                    &pubkey
                );
                let r = self
                    .reshare(&pubkey, &dkg_id, epoch, current_peers, new_peers)
                    .await?;
                info!("DKG Completed - reshared ECDSA key {} = {} ", &pubkey, r);
            }
        }
        // don't return the reshared/refreshed keys.
        Ok((true, vec![]))
    }

    #[doc = "Get the current key type"]
    fn key_type(&self) -> KeyType {
        KeyType::EcdsaCaitSith
    }
}
