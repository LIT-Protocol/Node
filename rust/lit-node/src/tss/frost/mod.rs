use super::common::signing_scheme::{SigningAlgorithm, SigningScheme};
use super::dkg::curves::common::KeyHelper;
use crate::error::{unexpected_err_code, EC};
use crate::p2p_comms::CommsManager;
use crate::tss::common::traits::key_persistence::KeyPersistence;
use crate::{
    error::Result,
    peers::peer_state::models::{SimplePeer, SimplePeerExt},
    tss::common::{dkg_type::DkgType, tss_state::TssState},
};
use elliptic_curve::group::{Group, GroupEncoding};
use lit_frost::{Identifier, KeyPackage, SigningCommitments, VerifyingKey, VerifyingShare};
use lit_frost::{Scheme, SignatureShare};
use std::{marker::PhantomData, num::NonZeroU8};

#[derive(Debug, Clone)]
pub struct FrostState<G: Group + GroupEncoding + Default> {
    pub state: TssState,
    pub dkg_type: DkgType,
    pub _phantom: PhantomData<G>,
}

impl<G: Group + GroupEncoding + Default> FrostState<G> {
    pub fn new(state: TssState) -> Self {
        Self::new_with_dkg_type(state, DkgType::Standard)
    }

    pub fn new_with_dkg_type(state: TssState, dkg_type: DkgType) -> Self {
        FrostState {
            state,
            dkg_type,
            _phantom: PhantomData,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn sign_internal(
        &self,
        txn_prefix: &str,
        peers: Vec<SimplePeer>,
        message: &[u8],
        signature_scheme: SigningScheme,
        pubkey: Vec<u8>,
        secret_share: Vec<u8>,
        threshold: u16,
    ) -> Result<(
        Identifier,
        SignatureShare,
        SigningCommitments,
        VerifyingShare,
        VerifyingKey,
    )> {
        if !signature_scheme.supports_algorithm(SigningAlgorithm::Schnorr) {
            let msg = format!(
                "Requested signature scheme {:?} does not support Schnorr",
                signature_scheme
            );
            return Err(unexpected_err_code(
                "Unsupported signature curve for Schnorr signature",
                EC::NodeSignatureNotSupported,
                Some(msg),
            ));
        }

        // setup communications
        let round = "frost1";
        let mut peers = peers;
        peers.set_all_protocol_indices(1);

        let cm = CommsManager::new_with_peers(&self.state, txn_prefix, &peers, round).await?;

        // setup signing protocol
        let mut rng = rand::rngs::OsRng;
        let self_peer = peers.peer_at_address(&self.state.addr)?;
        let peer_id = self_peer.get_protocol_index()?;
        let scheme = signature_scheme.try_into().map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeUnknownError,
                Some("SigningScheme::try_into".to_string()),
            )
        })?;
        let group_key = self.get_verifying_key(scheme, pubkey)?;
        let secret_share = lit_frost::SigningShare {
            scheme,
            value: secret_share,
        };
        let identifier = Identifier::from((scheme, peer_id));

        let verifying_share = scheme.verifying_share(&secret_share).map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeUnknownError,
                Some("VerifyingShare::try_from".to_string()),
            )
        })?;

        // round1
        let (nonces, commitments) =
            scheme
                .signing_round1(&secret_share, &mut rng)
                .map_err(|e| {
                    unexpected_err_code(
                        e,
                        EC::NodeUnknownError,
                        Some("Signing Round 1".to_string()),
                    )
                })?;

        // exchange commitments
        let r_commitments = cm
            .broadcast_and_collect::<SigningCommitments, SigningCommitments>(commitments.clone())
            .await?;

        // store commitments & starting with ours!
        let mut signing_commitments = vec![(identifier.clone(), commitments.clone())];

        for (share_index, peer_commitments) in r_commitments {
            let remote_peer_id = peers
                .peer_at_share_index(share_index)?
                .get_protocol_index()?;
            let remote_identifier = Identifier::from((scheme, remote_peer_id));
            // signing_package.insert(peer_id, (nonces, secret_share));
            signing_commitments.push((remote_identifier, peer_commitments));
        }

        let threshold = match NonZeroU8::new(threshold as u8) {
            Some(threshold) => threshold,
            None => {
                return Err(unexpected_err_code(
                    "threshold must be greater than 0",
                    EC::NodeUnknownError,
                    Some("Signing Round 1".to_string()),
                ))
            }
        };
        // round 2
        let key_package = KeyPackage {
            identifier: identifier.clone(),
            secret_share: secret_share.clone(),
            verifying_key: group_key.clone(),
            threshold: threshold.into(),
        };

        let signature_share = scheme
            .signing_round2(message, &signing_commitments, &nonces, &key_package)
            .map_err(|e| {
                unexpected_err_code(e, EC::NodeUnknownError, Some("Signing Round 2".to_string()))
            })?;

        Ok((
            identifier,
            signature_share,
            commitments,
            verifying_share,
            group_key,
        ))
    }

    fn get_verifying_key(&self, scheme: Scheme, pubkey: Vec<u8>) -> Result<VerifyingKey> {
        let group_key = match scheme {
            Scheme::K256Sha256 => {
                let kh = KeyHelper::<k256::ProjectivePoint>::default();
                let p = kh.pk_from_bytes(&pubkey)?;
                VerifyingKey::try_from((scheme, p))
            }
            Scheme::Ed25519Sha512 => {
                let kh = KeyHelper::<curve25519_dalek::edwards::SubgroupPoint>::default();
                let p = kh.pk_from_bytes(&pubkey)?;

                // FIXME!!! This is a hack to convert the point to a WrappedEdwards point to fit with LitFrost
                let p2 = vsss_rs::curve25519::WrappedEdwards::from_bytes(&p.to_bytes());
                let p2 = p2.unwrap_or(vsss_rs::curve25519::WrappedEdwards::default());

                VerifyingKey::try_from((scheme, p2))
            }
            Scheme::Ristretto25519Sha512 => {
                let kh = KeyHelper::<curve25519_dalek::RistrettoPoint>::default();
                let p = kh.pk_from_bytes(&pubkey)?;

                // FIXME!!! This is a hack to convert the point to a WrappedRistretto point to fit with LitFrost
                let p2 = vsss_rs::curve25519::WrappedRistretto::from_bytes(&p.to_bytes());
                let p2 = p2.unwrap_or(vsss_rs::curve25519::WrappedRistretto::default());

                VerifyingKey::try_from((scheme, p2))
            }
            Scheme::P256Sha256 => {
                let kh = KeyHelper::<p256::ProjectivePoint>::default();
                let p = kh.pk_from_bytes(&pubkey)?;
                VerifyingKey::try_from((scheme, p))
            }
            Scheme::P384Sha384 => {
                let kh = KeyHelper::<p384::ProjectivePoint>::default();
                let p = kh.pk_from_bytes(&pubkey)?;
                VerifyingKey::try_from((scheme, p))
            }
            Scheme::Ed448Shake256 => {
                let kh = KeyHelper::<ed448_goldilocks::EdwardsPoint>::default();
                let p = kh.pk_from_bytes(&pubkey)?;
                VerifyingKey::try_from((scheme, p))
            }
            Scheme::RedJubjubBlake2b512 => {
                let kh = KeyHelper::<jubjub::SubgroupPoint>::default();
                let p = kh.pk_from_bytes(&pubkey)?;
                VerifyingKey::try_from((scheme, p))
            }
            Scheme::K256Taproot => {
                let kh = KeyHelper::<k256::ProjectivePoint>::default();
                let p = kh.pk_from_bytes(&pubkey)?;
                VerifyingKey::try_from((scheme, p))
            }
        };

        let group_key = group_key.map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeUnknownError,
                Some("VerifyingKey::try_from".to_string()),
            )
        })?;

        Ok(group_key)
    }
}
