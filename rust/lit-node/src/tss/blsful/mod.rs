pub mod backup;
pub mod models;
use super::common::traits::dkg::BasicDkg;
use crate::error::{unexpected_err, Result};
use crate::peers::peer_state::models::SimplePeerExt;
use crate::tss::blsful::models::BlsState;
use crate::tss::common::curve_type::CurveType;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::{storage::read_key_share_from_disk, traits::cipherable::Cipherable};
use blsful::inner_types::G1Projective;
use blsful::{vsss_rs::Share, Bls12381G2Impl, Pairing, SecretKeyShare, SignatureShare};
use lit_core::utils::binary::{bytes_to_hex, hex_to_bytes};
use tracing::instrument;

#[async_trait::async_trait]
impl Cipherable for BlsState<G1Projective> {
    #[instrument]
    async fn sign(
        &self,
        message_bytes: Vec<u8>,
        epoch: Option<u64>,
    ) -> Result<(SignatureShare<Bls12381G2Impl>, u32)> {
        let root_keys = self.root_keys().await;
        if root_keys.is_empty() {
            return Err(unexpected_err(
                "No primary BLS key found!".to_string(),
                None,
            ));
        }

        let root_key = &root_keys[0].clone();
        let public_key = hex_to_bytes(root_key)?;
        self.sign_with_pubkey(&message_bytes, public_key, epoch)
            .await
    }

    #[instrument]
    async fn sign_with_pubkey(
        &self,
        message_bytes: &[u8],
        public_key: Vec<u8>,
        epoch: Option<u64>,
    ) -> Result<(SignatureShare<Bls12381G2Impl>, u32)> {
        let pub_key = &bytes_to_hex(&public_key);

        let (secret_key_share, index) = self.get_keyshare(pub_key, epoch).await?;

        let sks = secret_key_share
            .sign(blsful::SignatureSchemes::ProofOfPossession, &message_bytes)
            .map_err(|e| unexpected_err(format!("Failed to sign message: {:?}", e), None))?;

        Ok((sks, index))
    }
}

impl BlsState<G1Projective> {
    async fn get_keyshare(
        &self,
        pubkey: &str,
        epoch: Option<u64>,
    ) -> Result<(SecretKeyShare<Bls12381G2Impl>, u32)> {
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

        let (epoch, peers) = match self_epoch - epoch {
            0 => (epoch, self.state.peer_state.peers().await?),
            1 => (epoch, self.state.peer_state.peers_in_prior_epoch().await?),
            _ => (self_epoch, self.state.peer_state.peers().await?),
        };

        let share_index = peers.share_index(&self.state.addr)?;

        let staker_address = &self.state.peer_state.hex_staker_address();

        let bls_key_share = read_key_share_from_disk::<KeyShare>(
            pubkey,
            share_index,
            epoch,
            self.curve_type(),
            staker_address,
        )
        .await?;

        // info!(
        //     "BLS Pubkey [{}]: {}",
        //     pubkey,
        //     bytes_to_hex(&bls_key_share.hex_public_key)
        // );
        let identifier = (bls_key_share.index + 1) as u8; // Participant ID is expected to be 1-indexed.
                                                          // let value = hex_to_bytes(bls_key_share.hex_private_share)?;
        let value = bls_key_share.secret_as_bytes(CurveType::BLS)?;

        let secret_key_share = SecretKeyShare(
            <Bls12381G2Impl as Pairing>::SecretKeyShare::with_identifier_and_value(
                identifier, &value,
            ),
        );

        Ok((secret_key_share, share_index as u32))
    }
}
