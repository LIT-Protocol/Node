use super::common::*;
use crate::error::{unexpected_err, Result};
use crate::tss::frost::FrostState;
use curve25519_dalek::{edwards::SubgroupPoint, Scalar};

#[async_trait::async_trait]
impl BasicDkg for FrostState<SubgroupPoint> {
    fn tss_state(&self) -> &crate::tss::common::tss_state::TssState {
        &self.state
    }

    fn curve_type(&self) -> CurveType {
        CurveType::Ed25519
    }
}

#[async_trait::async_trait]
impl EpochManager for FrostState<SubgroupPoint> {}

#[async_trait::async_trait]
impl KeyPersistence<SubgroupPoint> for KeyHelper<SubgroupPoint> {
    fn curve_type(&self) -> CurveType {
        CurveType::Ed25519
    }
    fn secret_from_hex(&self, hex_private_key: &str) -> Result<Scalar> {
        Scalar::from_le_hex(hex_private_key).ok_or(unexpected_err(
            "Failed to convert hex to ed25519 private key",
            None,
        ))
    }

    fn secret_from_bytes(&self, private_key: &[u8]) -> Result<Scalar> {
        self.validate_scalar_len(private_key)?;
        Scalar::from_le_bytes(&private_key[..32]).ok_or(unexpected_err(
            "Failed to convert bytes to ed25519 private key",
            None,
        ))
    }

    fn pk_from_hex(&self, hex_public_key: &str) -> Result<SubgroupPoint> {
        SubgroupPoint::from_compressed_hex(hex_public_key).ok_or(unexpected_err(
            "Failed to convert hex to ed25519 public key",
            None,
        ))
    }

    fn pk_to_hex(&self, public_key: &SubgroupPoint) -> String {
        public_key.to_compressed_hex().to_string()
    }

    fn secret_to_hex(&self, share: &Scalar) -> String {
        share.to_le_hex().to_string()
    }

    fn pk_from_bytes(&self, public_key: &[u8]) -> Result<SubgroupPoint> {
        self.validate_pk_len(public_key)?;
        SubgroupPoint::from_compressed(&public_key[..32]).ok_or(unexpected_err(
            "Failed to convert bytes to ed25519 public key",
            None,
        ))
    }
}
