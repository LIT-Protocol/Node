use super::common::*;
use crate::error::{unexpected_err, Result};
use crate::tss::frost::FrostState;
use p384::{ProjectivePoint, Scalar};

#[async_trait::async_trait]
impl BasicDkg for FrostState<ProjectivePoint> {
    fn tss_state(&self) -> &crate::tss::common::tss_state::TssState {
        &self.state
    }

    fn curve_type(&self) -> CurveType {
        CurveType::P384
    }
}

#[async_trait::async_trait]
impl EpochManager for FrostState<ProjectivePoint> {}

#[async_trait::async_trait]
impl KeyPersistence<ProjectivePoint> for KeyHelper<ProjectivePoint> {
    fn curve_type(&self) -> CurveType {
        CurveType::P384
    }

    fn secret_from_hex(&self, hex_private_key: &str) -> Result<Scalar> {
        Scalar::from_be_hex(hex_private_key).ok_or(unexpected_err(
            "Failed to convert hex to p384 private key",
            None,
        ))
    }

    fn secret_from_bytes(&self, private_key: &[u8]) -> Result<Scalar> {
        self.validate_scalar_len(private_key)?;
        Scalar::from_be_bytes(&private_key[..48]).ok_or(unexpected_err(
            "Failed to convert bytes to p384 private key",
            None,
        ))
    }

    fn pk_from_hex(&self, hex_public_key: &str) -> Result<ProjectivePoint> {
        p384::ProjectivePoint::from_compressed_hex(hex_public_key).ok_or(unexpected_err(
            "Failed to convert hex top384 public key",
            None,
        ))
    }

    fn pk_to_hex(&self, public_key: &ProjectivePoint) -> String {
        public_key.to_compressed_hex().to_string()
    }

    fn secret_to_hex(&self, share: &Scalar) -> String {
        share.to_be_hex().to_string()
    }

    fn pk_from_bytes(&self, public_key: &[u8]) -> Result<ProjectivePoint> {
        self.validate_pk_len(public_key)?;
        p384::ProjectivePoint::from_compressed(&public_key[..49]).ok_or(unexpected_err(
            "Failed to convert bytes to p384 public key",
            None,
        ))
    }
}
