use super::super::dkg::gennaro::models::GennaroMpcDkg;
use crate::p2p_comms::CommsManager;
use crate::peers::peer_state::models::{SimplePeer, SimplePeerExt};
use crate::tss::common::traits::key_persistence::KeyPersistence;
use crate::tss::common::{curve_type::CurveType, dkg_type::DkgType};
use elliptic_curve::group::{Group, GroupEncoding};

use super::super::dkg::gennaro::{GennaroDkg, Mode, RoundResult};
use crate::error::{unexpected_err_code, Result, EC};
use crate::utils::consensus::get_threshold_count;
use elliptic_curve::Field;
use gennaro_dkg::Parameters;
use lit_core::error::Unexpected;
use std::num::NonZeroUsize;
use tracing::instrument;

impl<G: Group + GroupEncoding + Default> GennaroMpcDkg<G> {
    pub async fn do_keygen(
        &self,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<SimplePeer>,
        dkg_type: DkgType,
        curve_type: CurveType,
        key_state: impl KeyPersistence<G>,
    ) -> Result<String> {
        let (pk, share, index) = self
            .execute(Mode::Initial, dkg_id, peers, peers, None)
            .await?;

        let epoch = epoch + 1; // because we are generating a new key, it will be for the next epoch.
        let pubkey = None;
        let peers = &peers.active_peers();
        let staker_address = &self.state.peer_state.hex_staker_address();
        let pubkey = key_state
            .write_key(
                pubkey,
                pk,
                share,
                index,
                dkg_id,
                epoch,
                peers,
                dkg_type,
                curve_type,
                staker_address,
            )
            .await?;
        Ok(pubkey)
    }

    pub async fn do_refresh(
        &self,
        pubkey: &str,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<SimplePeer>,
        curve_type: CurveType,
        key_state: impl KeyPersistence<G>,
    ) -> Result<bool> {
        info!("refreshing key: {} for dkg_id: {}", pubkey, dkg_id);

        let share_index = peers.share_index(&self.state.addr)?;
        let staker_address = &self.state.peer_state.hex_staker_address();

        let (private_share, pk) = match key_state
            .read_key(pubkey, share_index, epoch, staker_address)
            .await
        {
            Ok(Some((private_share, public_key))) => (private_share, public_key),
            Ok(None) => {
                error!("key share not found on disk for public key {}", pubkey);
                return Ok(false);
            }
            Err(e) => {
                error!("key share not found on disk for public key {}", pubkey);
                return Ok(false);
            }
        };

        let (result_pk, secret_share, index) = self
            .execute(Mode::RefreshPeer, dkg_id, peers, peers, Some(private_share))
            .await?;

        // refresh!!!
        let share = secret_share + private_share;
        let next_epoch = epoch + 1;
        let pubkey = Some(pubkey.to_string());
        let peers = &peers.active_peers();
        let dkg_type = DkgType::Standard;

        let _pubkey = key_state
            .write_key(
                pubkey,
                pk,
                share,
                index,
                dkg_id,
                next_epoch,
                peers,
                dkg_type,
                curve_type,
                staker_address,
            )
            .await?;

        Ok(true)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn do_reshare(
        &self,
        pubkey: &str,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<SimplePeer>,
        next_peers: &Vec<SimplePeer>,
        curve_type: CurveType,
        key_state: impl KeyPersistence<G>,
    ) -> Result<bool> {
        let share_index = match peers.share_index(&self.state.addr) {
            Ok(share_index) => share_index,
            Err(e) => {
                info!("share index not found for peer: {}", self.state.addr);
                u16::MAX // this will be incorrect, but that's okay.
            }
        };
        let staker_address = &self.state.peer_state.hex_staker_address();

        let (private_share, existing_public_key, mode) = match key_state
            .read_key(pubkey, share_index, epoch, staker_address)
            .await
        {
            Ok(Some((private_share, public_key))) => {
                (Some(private_share), Some(public_key), Mode::ExistingPeer)
            }
            Ok(None) => (None, None, Mode::NewPeer),
            Err(_) => (None, None, Mode::NewPeer),
        };

        let (pk, share, index) = self
            .execute(mode, dkg_id, peers, next_peers, private_share)
            .await?;

        let next_epoch = epoch + 1;

        let pk = match existing_public_key {
            Some(existing_public_key) => existing_public_key,
            None => pk,
        };

        let pubkey = Some(pubkey.to_string());
        let peers = &next_peers.active_peers();
        let dkg_type = DkgType::Standard;

        let _pubkey = key_state
            .write_key(
                pubkey,
                pk,
                share,
                index,
                dkg_id,
                next_epoch,
                peers,
                dkg_type,
                curve_type,
                staker_address,
            )
            .await?;

        Ok(true)
    }

    #[instrument(skip_all, fields(txn_prefix = txn_prefix))]
    pub async fn execute(
        &self,
        mode: Mode,
        txn_prefix: &str,
        current_peers: &Vec<SimplePeer>,
        next_peers: &Vec<SimplePeer>,
        share: Option<<G as Group>::Scalar>,
    ) -> Result<(G, G::Scalar, u16)> {
        // setup Gennaro DKG

        // Set the protocol identifier; for Generao, this is 1 based, so we just offset it by 1
        let mut current_peers = current_peers.clone();
        current_peers.set_all_protocol_indices(1);
        let mut next_peers = next_peers.clone();
        next_peers.set_all_protocol_indices(1);

        let self_peer = next_peers.peer_at_address(&self.state.addr)?;

        let usize_limit = next_peers.len();
        let usize_threshold = get_threshold_count(usize_limit);
        let threshold = NonZeroUsize::new(get_threshold_count(usize_limit))
            .expect_or_err("Empty non-zero usize threshold")?;
        let limit = NonZeroUsize::new(usize_limit).expect_or_err("Empty non-zero usize limit")?;
        let parameters = Parameters::<G>::new(threshold, limit);
        let expected_items = usize_limit - 1;

        let share_index = self_peer.share_index;
        let peer_id = self_peer.get_protocol_index()? as usize;

        let (old_peer_id, old_share_index) = match current_peers.peer_at_address(&self.state.addr) {
            Ok(peer) => (peer.get_protocol_index()? as usize, peer.share_index),
            Err(_) => (0, 0),
        };

        let mut dkg = GennaroDkg::<G>::default();
        dkg.init_participant(
            &mode,
            peer_id,
            &next_peers,
            &current_peers,
            &self.state.addr,
            share,
        )?;

        let old_threshold = get_threshold_count(current_peers.len());

        info!(
            "Node #{}: {}  |  Peer ID {:?} -> {:?}  (Disk) Share Index : {:?} -> {:?}  Threshold : {} -> {:?} ",
            peer_id, self_peer.socket_address, old_peer_id, peer_id, old_share_index, share_index, old_threshold,  usize_threshold,
        );

        loop {
            let round_output = match dkg.execute()? {
                Some(round_output) => round_output,
                None => {
                    dkg.clear_dkg_data();
                    break;
                }
            };

            let round = dkg.get_round().expect_or_err("Empty round")?.to_string();
            let cm =
                CommsManager::new_with_peers(&self.state, txn_prefix, &next_peers, &round).await?;

            for dest_peer in next_peers.all_peers_except(&self.state.addr).iter() {
                let index = dest_peer.get_protocol_index()? as usize;
                if let Some(value) = RoundResult::from_round_outputs(&round_output, index) {
                    let r = cm.send_direct(dest_peer, value).await?;
                }
            }

            let received = cm.collect::<RoundResult<G>>().await?;

            for (share_index, data) in received.iter() {
                let peer_id = next_peers
                    .peer_at_share_index(*share_index)
                    .expect_or_err("Empty peer id")?
                    .get_protocol_index()?;
                if let Err(e) = dkg.add_peer_data(peer_id as usize, data.clone()) {
                    error!("Error while adding peer data: {}", e);
                }
            }
        }

        let secret_share = dkg.get_secret_share().expect_or_err("Empty secret share")?;
        if secret_share.is_zero().into() {
            return Err(unexpected_err_code(
                "Resulting share for DKG is zero.",
                EC::NodeDKGInvalidValue,
                None,
            ));
        }

        Ok((
            dkg.get_public_key().expect_or_err("Empty public key")?,
            dkg.get_secret_share().expect_or_err("Empty secret share")?,
            share_index,
        ))
    }
}
