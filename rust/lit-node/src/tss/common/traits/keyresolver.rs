use blsful::inner_types::Bls12381G1;
use elliptic_curve::group::GroupEncoding;
use elliptic_curve::Group;
use k256::Secp256k1;

use super::super::key_type::KeyType;
use crate::tss::blsful::models::BlsKeyShare;
use crate::tss::common::backup::VerifiableBackup;
use crate::tss::ecdsa_cait_sith::models::EcdsaKeyShare;

pub trait KeyResolver {
    fn resolve_key(&self) -> KeyType;
    fn resolve_key_from_trait() -> KeyType;
}

impl KeyResolver for EcdsaKeyShare<k256::Secp256k1> {
    fn resolve_key(&self) -> KeyType {
        KeyType::EcdsaCaitSith
    }
    fn resolve_key_from_trait() -> KeyType {
        KeyType::EcdsaCaitSith
    }
}

impl KeyResolver for VerifiableBackup<Secp256k1> {
    fn resolve_key(&self) -> KeyType {
        KeyType::EcdsaCaitSith
    }
    fn resolve_key_from_trait() -> KeyType {
        KeyType::EcdsaCaitSith
    }
}

impl<G: Group + GroupEncoding + Default> KeyResolver for BlsKeyShare<G> {
    fn resolve_key(&self) -> KeyType {
        KeyType::BLS
    }
    fn resolve_key_from_trait() -> KeyType {
        KeyType::BLS
    }
}

impl KeyResolver for VerifiableBackup<Bls12381G1> {
    fn resolve_key(&self) -> KeyType {
        KeyType::BLS
    }
    fn resolve_key_from_trait() -> KeyType {
        KeyType::BLS
    }
}
