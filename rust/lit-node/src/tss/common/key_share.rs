use crate::error::unexpected_err;
use crate::tss::common::curve_type::CurveType;
use crate::tss::common::key_share_helper::KeyHelper;
use crate::tss::common::traits::key_persistence::KeyPersistence;
use crate::tss::dkg::curves::common::LeBytes;
use crate::utils::encoding::CompressedPointBytes;
use crate::{error::Result, utils::encoding::BeBytes};
use elliptic_curve::group::GroupEncoding;
use elliptic_curve::{Group, PrimeField};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyShare {
    pub hex_private_share: String,
    pub hex_public_key: String,
    pub curve_type: u8,
    pub index: u16,
    pub threshold: u16,
    pub total_shares: u16,
    pub txn_prefix: String,
}

impl KeyShare {
    pub fn new<G: Group + GroupEncoding + Default>(
        secret: <G as Group>::Scalar,
        public_key: G,
        curve_type: CurveType,
        index: u16,
        threshold: u16,
        total_shares: u16,
        txn_prefix: String,
    ) -> Result<Self> {
        let repr = secret.to_repr();
        let hex_private_share = Self::private_key_to_hex(repr.as_ref(), curve_type)?;
        let repr_pk = public_key.to_bytes();
        let hex_public_key = Self::public_key_to_hex(repr_pk.as_ref(), curve_type)?;

        Ok(KeyShare {
            hex_private_share,
            hex_public_key,
            curve_type: curve_type.into(),
            index,
            threshold,
            total_shares,
            txn_prefix,
        })
    }

    pub fn secret<F: PrimeField>(&self) -> Result<F> {
        let curve_type = CurveType::try_from(self.curve_type)?;
        let src = Self::private_share_bytes(&self.hex_private_share, curve_type)?;
        let src = src.as_slice();
        let mut repr = F::Repr::default();
        repr.as_mut().copy_from_slice(src);

        Option::from(<F as PrimeField>::from_repr(repr)).ok_or(unexpected_err(
            "Failed to convert private share to prime field element".to_string(),
            None,
        ))
    }

    pub fn public_key<G: GroupEncoding>(&self, curve_type: CurveType) -> Result<G> {
        let src = Self::public_key_bytes(&self.hex_public_key, curve_type)?;
        let src = src.as_slice();
        let mut repr = G::Repr::default();
        repr.as_mut().copy_from_slice(src);

        Option::from(G::from_bytes(&repr)).ok_or(unexpected_err(
            "Failed to convert public key to group element".to_string(),
            None,
        ))
    }

    pub fn secret_as_bytes(&self, curve_type: CurveType) -> Result<Vec<u8>> {
        Self::private_share_bytes(&self.hex_private_share, curve_type)
    }

    pub fn public_key_as_bytes(&self, curve_type: CurveType) -> Result<Vec<u8>> {
        Self::public_key_bytes(&self.hex_public_key, curve_type)
    }

    pub fn private_share_bytes(hex_private_share: &str, curve_type: CurveType) -> Result<Vec<u8>> {
        match curve_type {
            CurveType::BLS => {
                let kh = KeyHelper::<blsful::inner_types::G1Projective>::default();
                Ok(kh
                    .secret_from_hex(hex_private_share)?
                    .to_le_bytes()
                    .to_vec())
            }
            CurveType::K256 => {
                let kh = KeyHelper::<k256::ProjectivePoint>::default();
                Ok(kh
                    .secret_from_hex(hex_private_share)?
                    .to_be_bytes()
                    .to_vec())
            }
            CurveType::Ed25519 => {
                let kh = KeyHelper::<curve25519_dalek::edwards::SubgroupPoint>::default();
                Ok(kh
                    .secret_from_hex(hex_private_share)?
                    .to_le_bytes()
                    .to_vec())
            }
            CurveType::Ed448 => {
                let kh = KeyHelper::<ed448_goldilocks::EdwardsPoint>::default();
                Ok(kh
                    .secret_from_hex(hex_private_share)?
                    .to_be_bytes()
                    .to_vec())
            }
            CurveType::P256 => {
                let kh = KeyHelper::<p256::ProjectivePoint>::default();
                Ok(kh
                    .secret_from_hex(hex_private_share)?
                    .to_be_bytes()
                    .to_vec())
            }
            CurveType::P384 => {
                let kh = KeyHelper::<p384::ProjectivePoint>::default();
                Ok(kh
                    .secret_from_hex(hex_private_share)?
                    .to_be_bytes()
                    .to_vec())
            }
            CurveType::Ristretto25519 => {
                let kh = KeyHelper::<curve25519_dalek::RistrettoPoint>::default();
                Ok(kh
                    .secret_from_hex(hex_private_share)?
                    .to_le_bytes()
                    .to_vec())
            }
            CurveType::RedJubjub => {
                let kh = KeyHelper::<jubjub::SubgroupPoint>::default();
                Ok(kh
                    .secret_from_hex(hex_private_share)?
                    .to_be_bytes()
                    .to_vec())
            }
        }
    }

    pub fn public_key_bytes(hex_public_key: &str, curve_type: CurveType) -> Result<Vec<u8>> {
        match curve_type {
            CurveType::BLS => {
                let kh = KeyHelper::<blsful::inner_types::G1Projective>::default();
                Ok(kh.pk_from_hex(hex_public_key)?.to_compressed().to_vec())
            }
            CurveType::K256 => {
                let kh = KeyHelper::<k256::ProjectivePoint>::default();
                Ok(kh.pk_from_hex(hex_public_key)?.to_compressed().to_vec())
            }
            CurveType::Ed25519 => {
                let kh = KeyHelper::<curve25519_dalek::edwards::SubgroupPoint>::default();
                Ok(kh.pk_from_hex(hex_public_key)?.to_compressed().to_vec())
            }
            CurveType::Ed448 => {
                let kh = KeyHelper::<ed448_goldilocks::EdwardsPoint>::default();
                Ok(kh.pk_from_hex(hex_public_key)?.to_compressed().to_vec())
            }
            CurveType::P256 => {
                let kh = KeyHelper::<p256::ProjectivePoint>::default();
                Ok(kh.pk_from_hex(hex_public_key)?.to_compressed().to_vec())
            }
            CurveType::P384 => {
                let kh = KeyHelper::<p384::ProjectivePoint>::default();
                Ok(kh.pk_from_hex(hex_public_key)?.to_compressed().to_vec())
            }
            CurveType::Ristretto25519 => {
                let kh = KeyHelper::<curve25519_dalek::RistrettoPoint>::default();
                Ok(kh.pk_from_hex(hex_public_key)?.to_compressed().to_vec())
            }
            CurveType::RedJubjub => {
                let kh = KeyHelper::<jubjub::SubgroupPoint>::default();
                Ok(kh.pk_from_hex(hex_public_key)?.to_compressed().to_vec())
            }
        }
    }

    pub fn public_key_to_hex(public_key: &[u8], curve_type: CurveType) -> Result<String> {
        match curve_type {
            CurveType::BLS => {
                let kh = KeyHelper::<blsful::inner_types::G1Projective>::default();
                Ok(kh.pk_to_hex(&kh.pk_from_bytes(public_key)?))
            }
            CurveType::K256 => {
                let kh = KeyHelper::<k256::ProjectivePoint>::default();
                Ok(kh.pk_to_hex(&kh.pk_from_bytes(public_key)?))
            }
            CurveType::Ed25519 => {
                let kh = KeyHelper::<curve25519_dalek::edwards::SubgroupPoint>::default();
                Ok(kh.pk_to_hex(&kh.pk_from_bytes(public_key)?))
            }
            CurveType::Ed448 => {
                let kh = KeyHelper::<ed448_goldilocks::EdwardsPoint>::default();
                Ok(kh.pk_to_hex(&kh.pk_from_bytes(public_key)?))
            }
            CurveType::P256 => {
                let kh = KeyHelper::<p256::ProjectivePoint>::default();
                Ok(kh.pk_to_hex(&kh.pk_from_bytes(public_key)?))
            }
            CurveType::P384 => {
                let kh = KeyHelper::<p384::ProjectivePoint>::default();
                Ok(kh.pk_to_hex(&kh.pk_from_bytes(public_key)?))
            }
            CurveType::Ristretto25519 => {
                let kh = KeyHelper::<curve25519_dalek::RistrettoPoint>::default();
                Ok(kh.pk_to_hex(&kh.pk_from_bytes(public_key)?))
            }
            CurveType::RedJubjub => {
                let kh = KeyHelper::<jubjub::SubgroupPoint>::default();
                Ok(kh.pk_to_hex(&kh.pk_from_bytes(public_key)?))
            }
        }
    }

    pub fn private_key_to_hex(private_key: &[u8], curve_type: CurveType) -> Result<String> {
        match curve_type {
            CurveType::BLS => {
                let kh = KeyHelper::<blsful::inner_types::G1Projective>::default();
                Ok(kh.secret_to_hex(&kh.secret_from_bytes(private_key)?))
            }
            CurveType::K256 => {
                let kh = KeyHelper::<k256::ProjectivePoint>::default();
                Ok(kh.secret_to_hex(&kh.secret_from_bytes(private_key)?))
            }
            CurveType::Ed25519 => {
                let kh = KeyHelper::<curve25519_dalek::edwards::SubgroupPoint>::default();
                Ok(kh.secret_to_hex(&kh.secret_from_bytes(private_key)?))
            }
            CurveType::Ed448 => {
                let kh = KeyHelper::<ed448_goldilocks::EdwardsPoint>::default();
                Ok(kh.secret_to_hex(&kh.secret_from_bytes(private_key)?))
            }
            CurveType::P256 => {
                let kh = KeyHelper::<p256::ProjectivePoint>::default();
                Ok(kh.secret_to_hex(&kh.secret_from_bytes(private_key)?))
            }
            CurveType::P384 => {
                let kh = KeyHelper::<p384::ProjectivePoint>::default();
                Ok(kh.secret_to_hex(&kh.secret_from_bytes(private_key)?))
            }
            CurveType::Ristretto25519 => {
                let kh = KeyHelper::<curve25519_dalek::RistrettoPoint>::default();
                Ok(kh.secret_to_hex(&kh.secret_from_bytes(private_key)?))
            }
            CurveType::RedJubjub => {
                let kh = KeyHelper::<jubjub::SubgroupPoint>::default();
                Ok(kh.secret_to_hex(&kh.secret_from_bytes(private_key)?))
            }
        }
    }
}
