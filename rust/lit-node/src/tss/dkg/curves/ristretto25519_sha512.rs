use super::common::*;
use crate::error::{unexpected_err, Result};
use crate::tss::frost::FrostState;
use curve25519_dalek::{RistrettoPoint, Scalar};

#[async_trait::async_trait]
impl BasicDkg for FrostState<RistrettoPoint> {
    fn tss_state(&self) -> &crate::tss::common::tss_state::TssState {
        &self.state
    }

    fn curve_type(&self) -> CurveType {
        CurveType::Ristretto25519
    }
}

#[async_trait::async_trait]
impl EpochManager for FrostState<RistrettoPoint> {}

#[async_trait::async_trait]
impl KeyPersistence<RistrettoPoint> for KeyHelper<RistrettoPoint> {
    fn curve_type(&self) -> CurveType {
        CurveType::Ristretto25519
    }
    fn secret_from_hex(&self, hex_private_key: &str) -> Result<Scalar> {
        Scalar::from_le_hex(hex_private_key).ok_or(unexpected_err(
            "Failed to convert hex to Ristretto 25519 private key",
            None,
        ))
    }

    fn secret_from_bytes(&self, private_key: &[u8]) -> Result<Scalar> {
        self.validate_scalar_len(private_key)?;
        Scalar::from_le_bytes(&private_key[..32]).ok_or(unexpected_err(
            "Failed to convert bytes to  Ristretto 25519 private key",
            None,
        ))
    }

    fn pk_from_hex(&self, hex_public_key: &str) -> Result<RistrettoPoint> {
        RistrettoPoint::from_compressed_hex(hex_public_key).ok_or(unexpected_err(
            "Failed to convert hex to Ristretto 25519 public key",
            None,
        ))
    }

    fn pk_to_hex(&self, public_key: &RistrettoPoint) -> String {
        public_key.to_compressed_hex().to_string()
    }

    fn secret_to_hex(&self, share: &Scalar) -> String {
        share.to_le_hex().to_string()
    }

    fn pk_from_bytes(&self, public_key: &[u8]) -> Result<RistrettoPoint> {
        self.validate_pk_len(public_key)?;
        RistrettoPoint::from_compressed(&public_key[..32]).ok_or(unexpected_err(
            "Failed to convert bytes to Ristretto 25519 public key",
            None,
        ))
    }
}
