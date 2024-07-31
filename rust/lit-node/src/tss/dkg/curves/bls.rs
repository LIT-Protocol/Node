use super::common::*;
use crate::error::{unexpected_err, Result};
use crate::tss::blsful::models::BlsState;
use blsful::inner_types::{G1Projective, Scalar};
use elliptic_curve::PrimeField;

#[async_trait::async_trait]
impl BasicDkg for BlsState<G1Projective> {
    fn tss_state(&self) -> &crate::tss::common::tss_state::TssState {
        &self.state
    }

    fn curve_type(&self) -> CurveType {
        CurveType::BLS
    }
}

#[async_trait::async_trait]
impl EpochManager for BlsState<G1Projective> {}

#[async_trait::async_trait]
impl KeyPersistence<G1Projective> for KeyHelper<G1Projective> {
    fn curve_type(&self) -> CurveType {
        CurveType::BLS
    }

    fn secret_from_hex(&self, hex_private_key: &str) -> Result<Scalar> {
        Option::from(Scalar::from_be_hex(hex_private_key)).ok_or(unexpected_err(
            "Failed to convert bytes to private key",
            None,
        ))
    }

    fn pk_from_hex(&self, hex_public_key: &str) -> Result<G1Projective> {
        Option::from(G1Projective::from_compressed_hex(hex_public_key)).ok_or(unexpected_err(
            "Failed to convert bytes to public key",
            None,
        ))
    }

    fn secret_from_bytes(&self, private_key: &[u8]) -> Result<Scalar> {
        self.validate_scalar_len(private_key)?;
        let mut repr = <bulletproofs::blstrs_plus::Scalar as PrimeField>::Repr::default();
        repr.as_mut().copy_from_slice(&private_key[..32]);
        Option::from(Scalar::from_le_bytes(&repr)).ok_or(unexpected_err(
            "Failed to convert bytes to private key",
            None,
        ))
    }

    fn pk_to_hex(&self, public_key: &G1Projective) -> String {
        public_key.to_compressed_hex().to_string()
    }

    fn secret_to_hex(&self, share: &Scalar) -> String {
        share.to_be_hex().to_string()
    }

    fn pk_from_bytes(&self, public_key: &[u8]) -> Result<G1Projective> {
        self.validate_pk_len(public_key)?;
        let mut repr = G1Projective::default().to_compressed();
        repr.as_mut().copy_from_slice(&public_key[..48]);
        Option::from(G1Projective::from_compressed(&repr)).ok_or(unexpected_err(
            "Failed to convert bytes to public key",
            None,
        ))
    }
}
