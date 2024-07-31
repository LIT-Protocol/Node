use super::common::*;
use crate::error::{unexpected_err, Result};
use crate::tss::frost::FrostState;
use ed448_goldilocks::{EdwardsPoint, Scalar};

#[async_trait::async_trait]
impl BasicDkg for FrostState<EdwardsPoint> {
    fn tss_state(&self) -> &crate::tss::common::tss_state::TssState {
        &self.state
    }

    fn curve_type(&self) -> CurveType {
        CurveType::Ed448
    }
}

#[async_trait::async_trait]
impl EpochManager for FrostState<EdwardsPoint> {}

#[async_trait::async_trait]
impl KeyPersistence<EdwardsPoint> for KeyHelper<EdwardsPoint> {
    fn curve_type(&self) -> CurveType {
        CurveType::Ed448
    }

    fn secret_from_hex(&self, hex_private_key: &str) -> Result<Scalar> {
        Scalar::from_be_hex(hex_private_key).ok_or(unexpected_err(
            "Failed to convert hex to ed448 private key",
            None,
        ))
    }

    fn secret_from_bytes(&self, private_key: &[u8]) -> Result<Scalar> {
        self.validate_scalar_len(private_key)?;
        Scalar::from_be_bytes(&private_key[..57]).ok_or(unexpected_err(
            "Failed to convert bytes to ed448 private key",
            None,
        ))
    }

    fn pk_from_hex(&self, hex_public_key: &str) -> Result<EdwardsPoint> {
        EdwardsPoint::from_compressed_hex(hex_public_key).ok_or(unexpected_err(
            "Failed to convert hex to ed448 public key",
            None,
        ))
    }

    fn pk_to_hex(&self, public_key: &EdwardsPoint) -> String {
        public_key.to_compressed_hex().to_string()
    }

    fn secret_to_hex(&self, share: &Scalar) -> String {
        share.to_be_hex().to_string()
    }

    fn pk_from_bytes(&self, public_key: &[u8]) -> Result<EdwardsPoint> {
        self.validate_pk_len(public_key)?;
        EdwardsPoint::from_compressed(&public_key[..57]).ok_or(unexpected_err(
            "Failed to convert bytes to ed448 public key",
            None,
        ))
    }
}
