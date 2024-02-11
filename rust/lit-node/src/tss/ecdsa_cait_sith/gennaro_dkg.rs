use crate::error::{unexpected_err_code, Result, EC};
use crate::peers::peer_state::models::{PeerSocketAddress, PeerSocketAddressVec};
use crate::tss::common::key_type::{DkgType, KeyType};
use crate::tss::common::storage::{
    delete_keyshare, read_key_share_from_disk, write_backup_to_disk, write_key_share_to_disk,
};
use crate::utils::consensus::get_threshold_count;

use super::super::dkg::gennaro::Mode;
use elliptic_curve::group::GroupEncoding;
use elliptic_curve::CurveArithmetic;
use k256::{Scalar, Secp256k1};
use lit_core::config::LitConfig;
use lit_core::utils::binary::bytes_to_hex;
use std::sync::Arc;
use tracing::instrument;

use super::super::common::traits::dkg::DkgSupport;
use super::super::dkg::gennaro::models::GennaroState;

use crate::tss::ecdsa_cait_sith::models::EcdsaKeyShare;

#[async_trait::async_trait]
impl DkgSupport for GennaroState<k256::ProjectivePoint> {
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
        let pubkey = self
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
        Ok(pubkey)
    }

    async fn refresh(
        &self,
        pubkey: &str,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<PeerSocketAddress>,
    ) -> Result<bool> {
        info!("refreshing key: {} for dkg_id: {}", pubkey, dkg_id);
        let index = self.get_peer_id(peers)? as u16;
        let index = index - 1; // key shares are 0-indexed
        let key_share = match read_key_share_from_disk::<EcdsaKeyShare<Secp256k1>>(
            pubkey, index, epoch,
        )
        .await
        {
            Ok(key_share) => key_share,
            Err(e) => {
                info!("key share not found on disk for public key {}", pubkey);
                return Ok(false);
            }
        };

        let pubkey_projective = k256::ProjectivePoint::from(&key_share.public_key);
        let (pk, share, index) = self
            .execute(
                Mode::RefreshPeer,
                dkg_id,
                peers,
                peers,
                Some(key_share.private_share),
            )
            .await?;

        // refresh!!!
        let share = share + key_share.private_share;
        self.share_is_zero(share)?;

        let next_epoch = epoch + 1;

        let _pubkey = self
            .write_key(
                Some(pubkey.to_string()),
                pubkey_projective,
                share,
                index,
                Some(index),
                dkg_id,
                next_epoch,
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
        epoch: u64,
        peers: &Vec<PeerSocketAddress>,
        next_peers: &Vec<PeerSocketAddress>,
    ) -> Result<bool> {
        let (private_share, mode, old_index) = match self.get_peer_id(peers) {
            Ok(index) => {
                let index = (index - 1) as u16;
                match read_key_share_from_disk::<EcdsaKeyShare<Secp256k1>>(pubkey, index, epoch)
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
        let next_epoch = epoch + 1;

        let _pubkey = self
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

impl GennaroState<k256::ProjectivePoint> {
    #[allow(clippy::too_many_arguments)]
    pub async fn write_key(
        &self,
        pubkey: Option<String>,
        pk: k256::ProjectivePoint,
        share: k256::Scalar,
        index: usize,
        old_index: Option<usize>,
        dkg_id: &str,
        epoch: u64,
        peers: &[String],
        dkg_type: DkgType,
    ) -> Result<String> {
        let total_shares = peers.len() as u16;
        let threshold = get_threshold_count(total_shares as usize) as u16;
        let index = index - 1; // for 0-indexing

        use crate::utils::encoding::UncompressedPointHex;
        // because refreshing return 0x0 as a result, we need to check for the exisitence of a passed public key
        // and use this value as the PK parameter and file name.
        let (hex_pubkey, pk) = match pubkey {
            Some(pubkey) => {
                let pk = match  <k256::Secp256k1 as CurveArithmetic>::ProjectivePoint::from_uncompressed_hex(&pubkey) {
                    Some(pk) => pk,
                    None => return Err(unexpected_err_code("Could not load projective point from pubkey hex",EC::NodeDKGInvalidValue, None)),
                };
                (pubkey, pk)
            }
            None => (bytes_to_hex(pk.to_bytes()), pk),
        };

        let local_key = EcdsaKeyShare::<Secp256k1> {
            private_share: share,
            public_key: pk.to_affine(),
            index: index as u16,
            threshold,
            total_shares,
            txn_prefix: dkg_id.to_string(),
        };

        let epoch = match dkg_type {
            DkgType::Standard => epoch,
            DkgType::RecoveryParty => 0,
        };

        let write_result = write_key_share_to_disk::<EcdsaKeyShare<Secp256k1>>(
            &hex_pubkey,
            index as u16,
            epoch,
            &local_key,
        )
        .await?;

        // delete the previous epoch's keyshare, and log if there is an error - happens on initial epoch right now.
        if let Some(old_index) = old_index {
            if let Err(e) = delete_keyshare(
                KeyType::EcdsaCaitSith,
                &hex_pubkey,
                old_index as u16,
                epoch - 1,
            )
            .await
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
                    "Writing the backup data for ECDSA pubkey: {} to the disk failed: {}",
                    &hex_pubkey, e
                );
            }
        }

        let pubkey = bytes_to_hex(pk.to_affine().to_bytes());

        Ok(pubkey)
    }

    pub fn share_is_zero(&self, share: Scalar) -> Result<bool> {
        if share.is_zero().unwrap_u8() == 1 {
            return Err(unexpected_err_code(
                "Resulting share for DKG is zero.",
                EC::NodeDKGInvalidValue,
                None,
            ));
        };
        Ok(false)
    }

    fn key_type(&self) -> KeyType {
        KeyType::EcdsaCaitSith
    }
}
