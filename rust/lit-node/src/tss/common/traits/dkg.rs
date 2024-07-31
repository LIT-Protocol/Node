use crate::error::Result;
use crate::peers::peer_state::models::SimplePeer;
use crate::tss::common::key_share_helper::KeyHelper;
use crate::tss::common::{curve_type::CurveType, dkg_type::DkgType};
use crate::tss::dkg::gennaro::GennaroMpcDkg;
use blsful::inner_types::G1Projective;
use std::fmt::Debug;
#[async_trait::async_trait]
pub trait BasicDkg: Debug + Send + Sync {
    fn tss_state(&self) -> &crate::tss::common::tss_state::TssState;
    fn curve_type(&self) -> CurveType;

    async fn keygen(
        &self,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<SimplePeer>,
        dkg_type: DkgType,
    ) -> Result<String> {
        let state = self.tss_state().clone();
        let curve_type = self.curve_type();

        match curve_type {
            CurveType::BLS => {
                let key_helper = KeyHelper::<G1Projective>::default();
                GennaroMpcDkg::<G1Projective>::new(state, curve_type)
                    .do_keygen(dkg_id, epoch, peers, dkg_type, curve_type, key_helper)
                    .await
            }
            CurveType::K256 => {
                let key_helper = KeyHelper::<k256::ProjectivePoint>::default();
                GennaroMpcDkg::<k256::ProjectivePoint>::new(state, curve_type)
                    .do_keygen(dkg_id, epoch, peers, dkg_type, curve_type, key_helper)
                    .await
            }
            CurveType::P256 => {
                let key_helper = KeyHelper::<p256::ProjectivePoint>::default();
                GennaroMpcDkg::<p256::ProjectivePoint>::new(state, curve_type)
                    .do_keygen(dkg_id, epoch, peers, dkg_type, curve_type, key_helper)
                    .await
            }
            CurveType::P384 => {
                let key_helper = KeyHelper::<p384::ProjectivePoint>::default();
                GennaroMpcDkg::<p384::ProjectivePoint>::new(state, curve_type)
                    .do_keygen(dkg_id, epoch, peers, dkg_type, curve_type, key_helper)
                    .await
            }
            CurveType::Ed25519 => {
                let key_helper = KeyHelper::<curve25519_dalek::edwards::SubgroupPoint>::default();
                GennaroMpcDkg::<curve25519_dalek::edwards::SubgroupPoint>::new(state, curve_type)
                    .do_keygen(dkg_id, epoch, peers, dkg_type, curve_type, key_helper)
                    .await
            }
            CurveType::Ristretto25519 => {
                let key_helper = KeyHelper::<curve25519_dalek::RistrettoPoint>::default();
                GennaroMpcDkg::<curve25519_dalek::RistrettoPoint>::new(state, curve_type)
                    .do_keygen(dkg_id, epoch, peers, dkg_type, curve_type, key_helper)
                    .await
            }
            CurveType::Ed448 => {
                let key_helper = KeyHelper::<ed448_goldilocks::EdwardsPoint>::default();
                GennaroMpcDkg::<ed448_goldilocks::EdwardsPoint>::new(state, curve_type)
                    .do_keygen(dkg_id, epoch, peers, dkg_type, curve_type, key_helper)
                    .await
            }
            CurveType::RedJubjub => {
                let key_helper = KeyHelper::<jubjub::SubgroupPoint>::default();
                GennaroMpcDkg::<jubjub::SubgroupPoint>::new(state, curve_type)
                    .do_keygen(dkg_id, epoch, peers, dkg_type, curve_type, key_helper)
                    .await
            }
        }
    }

    #[doc = "Refresh an existing ECDSA key share amoung the same node set"]
    async fn refresh(
        &self,
        pubkey: &str,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<SimplePeer>,
    ) -> Result<bool> {
        let state = self.tss_state().clone();
        let curve_type = self.curve_type();

        match curve_type {
            CurveType::BLS => {
                let key_helper = KeyHelper::<G1Projective>::default();
                GennaroMpcDkg::<G1Projective>::new(state, curve_type)
                    .do_refresh(pubkey, dkg_id, epoch, peers, curve_type, key_helper)
                    .await
            }
            CurveType::K256 => {
                let key_helper = KeyHelper::<k256::ProjectivePoint>::default();
                GennaroMpcDkg::<k256::ProjectivePoint>::new(state, curve_type)
                    .do_refresh(pubkey, dkg_id, epoch, peers, curve_type, key_helper)
                    .await
            }
            CurveType::P256 => {
                let key_helper = KeyHelper::<p256::ProjectivePoint>::default();
                GennaroMpcDkg::<p256::ProjectivePoint>::new(state, curve_type)
                    .do_refresh(pubkey, dkg_id, epoch, peers, curve_type, key_helper)
                    .await
            }
            CurveType::P384 => {
                let key_helper = KeyHelper::<p384::ProjectivePoint>::default();
                GennaroMpcDkg::<p384::ProjectivePoint>::new(state, curve_type)
                    .do_refresh(pubkey, dkg_id, epoch, peers, curve_type, key_helper)
                    .await
            }
            CurveType::Ed25519 => {
                let key_helper = KeyHelper::<curve25519_dalek::edwards::SubgroupPoint>::default();
                GennaroMpcDkg::<curve25519_dalek::edwards::SubgroupPoint>::new(state, curve_type)
                    .do_refresh(pubkey, dkg_id, epoch, peers, curve_type, key_helper)
                    .await
            }
            CurveType::Ristretto25519 => {
                let key_helper = KeyHelper::<curve25519_dalek::RistrettoPoint>::default();
                GennaroMpcDkg::<curve25519_dalek::RistrettoPoint>::new(state, curve_type)
                    .do_refresh(pubkey, dkg_id, epoch, peers, curve_type, key_helper)
                    .await
            }
            CurveType::Ed448 => {
                let key_helper = KeyHelper::<ed448_goldilocks::EdwardsPoint>::default();
                GennaroMpcDkg::<ed448_goldilocks::EdwardsPoint>::new(state, curve_type)
                    .do_refresh(pubkey, dkg_id, epoch, peers, curve_type, key_helper)
                    .await
            }
            CurveType::RedJubjub => {
                let key_helper = KeyHelper::<jubjub::SubgroupPoint>::default();
                GennaroMpcDkg::<jubjub::SubgroupPoint>::new(state, curve_type)
                    .do_refresh(pubkey, dkg_id, epoch, peers, curve_type, key_helper)
                    .await
            }
        }
    }

    #[doc = "Reshare an existing ECDSA key share  with a new set of nodes"]
    async fn reshare(
        &self,
        pubkey: &str,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<SimplePeer>,
        next_peers: &Vec<SimplePeer>,
    ) -> Result<bool> {
        let state = self.tss_state().clone();
        let curve_type = self.curve_type();

        match curve_type {
            CurveType::BLS => {
                let key_helper = KeyHelper::<G1Projective>::default();
                GennaroMpcDkg::<G1Projective>::new(state, curve_type)
                    .do_reshare(
                        pubkey, dkg_id, epoch, peers, next_peers, curve_type, key_helper,
                    )
                    .await
            }
            CurveType::K256 => {
                let key_helper = KeyHelper::<k256::ProjectivePoint>::default();
                GennaroMpcDkg::<k256::ProjectivePoint>::new(state, curve_type)
                    .do_reshare(
                        pubkey, dkg_id, epoch, peers, next_peers, curve_type, key_helper,
                    )
                    .await
            }
            CurveType::P256 => {
                let key_helper = KeyHelper::<p256::ProjectivePoint>::default();
                GennaroMpcDkg::<p256::ProjectivePoint>::new(state, curve_type)
                    .do_reshare(
                        pubkey, dkg_id, epoch, peers, next_peers, curve_type, key_helper,
                    )
                    .await
            }
            CurveType::P384 => {
                let key_helper = KeyHelper::<p384::ProjectivePoint>::default();
                GennaroMpcDkg::<p384::ProjectivePoint>::new(state, curve_type)
                    .do_reshare(
                        pubkey, dkg_id, epoch, peers, next_peers, curve_type, key_helper,
                    )
                    .await
            }
            CurveType::Ed25519 => {
                let key_helper = KeyHelper::<curve25519_dalek::edwards::SubgroupPoint>::default();
                GennaroMpcDkg::<curve25519_dalek::edwards::SubgroupPoint>::new(state, curve_type)
                    .do_reshare(
                        pubkey, dkg_id, epoch, peers, next_peers, curve_type, key_helper,
                    )
                    .await
            }
            CurveType::Ristretto25519 => {
                let key_helper = KeyHelper::<curve25519_dalek::RistrettoPoint>::default();
                GennaroMpcDkg::<curve25519_dalek::RistrettoPoint>::new(state, curve_type)
                    .do_reshare(
                        pubkey, dkg_id, epoch, peers, next_peers, curve_type, key_helper,
                    )
                    .await
            }
            CurveType::Ed448 => {
                let key_helper = KeyHelper::<ed448_goldilocks::EdwardsPoint>::default();
                GennaroMpcDkg::<ed448_goldilocks::EdwardsPoint>::new(state, curve_type)
                    .do_reshare(
                        pubkey, dkg_id, epoch, peers, next_peers, curve_type, key_helper,
                    )
                    .await
            }
            CurveType::RedJubjub => {
                let key_helper = KeyHelper::<jubjub::SubgroupPoint>::default();
                GennaroMpcDkg::<jubjub::SubgroupPoint>::new(state, curve_type)
                    .do_reshare(
                        pubkey, dkg_id, epoch, peers, next_peers, curve_type, key_helper,
                    )
                    .await
            }
        }
    }

    async fn root_keys(&self) -> Vec<String> {
        let crk = self
            .tss_state()
            .chain_data_manager
            .root_keys
            .read()
            .await
            .clone();
        let curve_type = self.curve_type();
        crk.iter()
            .filter(|k| k.curve_type == curve_type)
            .map(|k| k.public_key.clone())
            .collect()
    }
}
