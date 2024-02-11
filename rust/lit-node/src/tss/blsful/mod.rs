pub mod backup;
pub mod models;
use super::common::storage::delete_keyshare;
use super::dkg::gennaro::Mode;
use crate::error::{unexpected_err, unexpected_err_code, Result, EC};
use crate::peers::peer_state::models::{PeerSocketAddress, PeerSocketAddressVec};
use crate::tss::common::{
    key_type::{DkgType, KeyType},
    storage::{read_key_share_from_disk, write_backup_to_disk, write_key_share_to_disk},
    traits::cipherable::Cipherable,
};
use crate::utils::consensus::get_threshold_count;
use blsful::inner_types::{G1Projective, Scalar};
use blsful::{vsss_rs::Share, Bls12381G2Impl, Pairing, SecretKeyShare, SignatureShare};
use elliptic_curve::group::GroupEncoding;
use lit_core::config::LitConfig;
use lit_core::utils::binary::{bytes_to_hex, hex_to_bytes};
use models::BlsKeyShare;
use std::sync::Arc;
use tracing::instrument;

use super::common::traits::dkg::DkgSupport;
use super::common::traits::epoch_manager::EpochManager;
use super::dkg::gennaro::models::GennaroState;

#[async_trait::async_trait]
impl DkgSupport for GennaroState<G1Projective> {
    async fn keygen(
        &self,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<PeerSocketAddress>,
        dkg_type: DkgType,
    ) -> Result<String> {
        let (pk, share, index) = self
            .execute(Mode::Initial, dkg_id, peers, peers, None)
            .await?;

        self.share_is_zero(share)?;
        let epoch = epoch + 1; // because we are generating a new key, it will be for the next epoch.
        let hex_pubkey = self
            .write_key(
                None,
                pk,
                share,
                index,
                None,
                dkg_id,
                epoch,
                &peers.active_addresses(),
                dkg_type,
            )
            .await?;

        Ok(hex_pubkey)
    }

    async fn refresh(
        &self,
        pubkey: &str,
        dkg_id: &str,
        current_epoch: u64,
        peers: &Vec<PeerSocketAddress>,
    ) -> Result<bool> {
        info!("refreshing key: {} for dkg_id: {}", pubkey, dkg_id);
        let index = self.get_peer_id(peers)? as u16;
        let index = index - 1;
        let key_share =
            read_key_share_from_disk::<BlsKeyShare<G1Projective>>(pubkey, index, current_epoch)
                .await?;

        let pubkey_projective = key_share.public_key;

        let (pk, share, index) = self
            .execute(
                Mode::RefreshPeer,
                dkg_id,
                peers,
                peers,
                Some(key_share.private_share),
            )
            .await?;

        let share = key_share.private_share + share;
        self.share_is_zero(share)?;
        let new_epoch = current_epoch + 1;

        let _hex_pubkey = self
            .write_key(
                Some(pubkey.to_string()),
                pubkey_projective,
                share,
                index,
                Some(index),
                dkg_id,
                new_epoch,
                &peers.active_addresses(),
                DkgType::Standard,
            )
            .await?;

        Ok(true)
    }

    async fn reshare(
        &self,
        pubkey: &str,
        dkg_id: &str,
        current_epoch: u64,
        peers: &Vec<PeerSocketAddress>,
        next_peers: &Vec<PeerSocketAddress>,
    ) -> Result<bool> {
        let (private_share, mode, old_index) = match self.get_peer_id(peers) {
            Ok(index) => {
                let index = (index - 1) as u16;
                match read_key_share_from_disk::<BlsKeyShare<G1Projective>>(
                    pubkey,
                    index,
                    current_epoch,
                )
                .await
                {
                    Ok(key_share) => (
                        Some(key_share.private_share),
                        Mode::ExistingPeer,
                        Some(index as usize),
                    ),
                    Err(_) => (None, Mode::NewPeer, None),
                }
            }
            Err(_) => {
                // The node is not in the current peer set and is dealt in.
                (None, Mode::NewPeer, None)
            }
        };

        let (pk, share, index) = self
            .execute(mode, dkg_id, peers, next_peers, private_share)
            .await?;

        self.share_is_zero(share)?;
        let next_epoch = current_epoch + 1;

        let _hex_pubkey = self
            .write_key(
                Some(pubkey.to_string()),
                pk,
                share,
                index,
                old_index,
                dkg_id,
                next_epoch,
                &next_peers.active_addresses(),
                DkgType::Standard,
            )
            .await?;

        Ok(true)
    }

    fn addr(&self) -> String {
        self.generic_addr()
    }

    async fn peer_socket_addresses(&self) -> Vec<PeerSocketAddress> {
        self.generic_peers().await
    }

    fn lit_config(&self) -> Arc<LitConfig> {
        self.generic_lit_config()
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
impl Cipherable for GennaroState<G1Projective> {
    #[instrument]
    async fn sign(&self, message_bytes: Vec<u8>) -> Result<(SignatureShare<Bls12381G2Impl>, u32)> {
        let root_keys = self.root_keys().await;
        if root_keys.is_empty() {
            return Err(unexpected_err(
                "No primary BLS key found!".to_string(),
                None,
            ));
        }

        let root_key = &root_keys[0].clone();
        let public_key = hex_to_bytes(root_key)?;
        self.sign_with_pubkey(&message_bytes, public_key).await
    }

    #[instrument]
    async fn sign_with_pubkey(
        &self,
        message_bytes: &[u8],
        public_key: Vec<u8>,
    ) -> Result<(SignatureShare<Bls12381G2Impl>, u32)> {
        let pub_key = &bytes_to_hex(&public_key);

        let (secret_key_share, index) = self.get_keyshare(pub_key).await?;

        let sks = secret_key_share
            .sign(blsful::SignatureSchemes::ProofOfPossession, &message_bytes)
            .map_err(|e| unexpected_err(format!("Failed to sign message: {:?}", e), None))?;

        Ok((sks, index))
    }
}

impl GennaroState<G1Projective> {
    async fn get_keyshare(&self, pubkey: &str) -> Result<(SecretKeyShare<Bls12381G2Impl>, u32)> {
        let peers = self.generic_peers().await;
        let epoch = self.state.peer_state.epoch().await;
        let node_config = self.node_config_for_dkg(&peers, "")?;

        let share_index = node_config.share_index - 1;

        let bls_key_share =
            read_key_share_from_disk::<BlsKeyShare<G1Projective>>(pubkey, share_index, epoch)
                .await?;

        let identifier = (bls_key_share.index + 1) as u8; // Participant ID is expected to be 1-indexed.
        let value = bls_key_share.private_share.to_le_bytes();
        let secret_key_share = SecretKeyShare(
            <Bls12381G2Impl as Pairing>::SecretKeyShare::with_identifier_and_value(
                identifier, &value,
            ),
        );

        Ok((secret_key_share, share_index as u32))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn write_key(
        &self,
        pubkey: Option<String>,
        pk: G1Projective,
        share: Scalar,
        index: usize,
        old_index: Option<usize>,
        dkg_id: &str,
        new_epoch: u64,
        peers: &[String],
        dkg_type: DkgType,
    ) -> Result<String> {
        let total_shares = peers.len() as u16;
        let threshold = get_threshold_count(total_shares as usize) as u16;
        let index = index - 1; // for 0-indexing

        // because refreshing return 0x0 as a pk result, we need to check for the exisitence of a passed public key
        // and use this value as the PK parameter and file name.
        let (hex_pubkey, public_key) = match pubkey {
            None => (bytes_to_hex(pk.to_bytes()), pk),
            Some(pubkey) => (
                pubkey.clone(),
                G1Projective::from_compressed_hex(&pubkey).unwrap(),
            ),
        };

        let local_key = BlsKeyShare::<G1Projective> {
            private_share: share,
            public_key: pk,
            index: index as u16,
            threshold,
            total_shares,
            txn_prefix: dkg_id.to_string(),
        };

        let epoch = match dkg_type {
            DkgType::Standard => new_epoch,
            DkgType::RecoveryParty => 0,
        };

        let write_result = write_key_share_to_disk::<BlsKeyShare<G1Projective>>(
            &hex_pubkey,
            index as u16,
            epoch,
            &local_key,
        )
        .await?;

        // delete the previous epoch's keyshare, and log if there is an error - happens on initial epoch right now.
        if let Some(old_index) = old_index {
            if let Err(e) =
                delete_keyshare(KeyType::BLS, &hex_pubkey, old_index as u16, new_epoch - 1).await
            {
                error!(
                    "Failed to delete keyshare {} with err : {:?}",
                    &hex_pubkey, e
                );
            } // This all needs to change for support signatures occuring during of overlapping epochs (epoch changes)
        }

        if dkg_type.is_to_be_backed_up() {
            let backup_result =
                write_backup_to_disk(&hex_pubkey, index as u16, &local_key, peers).await;
            // Do not fail if backup fails, rather report it:
            if let Err(e) = backup_result {
                error!(
                    "Writing the backup data for BLS pubkey: {} to the disk failed: {}",
                    &hex_pubkey, e
                );
            }
        }

        let pubkey = bytes_to_hex(pk.to_bytes());

        Ok(pubkey)
    }

    pub fn share_is_zero(&self, share: Scalar) -> Result<bool> {
        if share == Scalar::ZERO {
            return Err(unexpected_err_code(
                "Resulting share for DKG is zero.",
                EC::NodeDKGInvalidValue,
                None,
            ));
        };
        Ok(false)
    }
}

#[async_trait::async_trait]
impl EpochManager for GennaroState<G1Projective> {
    async fn change_epoch(
        &self,
        dkg_id: String,
        current_epoch: u64,
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

        info!(
            "Doing Epoch DKG: {} {:?} {:?}",
            dkg_id, current_peers, new_peers
        );
        let start = std::time::Instant::now();

        // this is the first time we are launching DKG to generate the nodes default BLS key
        if current_peers.is_empty() {
            info!("Launching initial BLS DKG!");
            let public_key = self
                .keygen(&dkg_id, current_epoch, new_peers, self.dkg_type)
                .await?;

            info!(
                "DKG Completed - generated BLS pubkey {} in {} ms.",
                &public_key,
                start.elapsed().as_millis()
            );

            return Ok((true, vec![public_key]));
        }

        let pubkey = self.root_keys().await[0].clone();
        if current_peers == new_peers {
            info!(
                "Launching Epoch DKG with same nodes - refresh key shares for {}!",
                &pubkey
            );
            let r = self
                .refresh(&pubkey, &dkg_id, current_epoch, new_peers)
                .await?;
            info!("DKG Completed - refreshed BLS key {} = {} ", &pubkey, r);
        } else {
            info!(
                "Launching Epoch DKG with new nodes - resharing key shares for {}!",
                &pubkey
            );
            let r = self
                .reshare(&pubkey, &dkg_id, current_epoch, current_peers, new_peers)
                .await?;
            info!("DKG Completed - reshared BLS key {} = {} ", &pubkey, r);
        }
        // don't return the reshared/refreshed keys.
        Ok((true, vec![]))
    }

    fn key_type(&self) -> KeyType {
        KeyType::BLS
    }
}
