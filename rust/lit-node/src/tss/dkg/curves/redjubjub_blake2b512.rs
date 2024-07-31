use super::common::*;
use crate::error::{unexpected_err, Result};
use crate::tss::frost::FrostState;
use jubjub::{Scalar, SubgroupPoint};

#[async_trait::async_trait]
impl BasicDkg for FrostState<SubgroupPoint> {
    fn tss_state(&self) -> &crate::tss::common::tss_state::TssState {
        &self.state
    }

    fn curve_type(&self) -> CurveType {
        CurveType::RedJubjub
    }
}

#[async_trait::async_trait]
impl EpochManager for FrostState<SubgroupPoint> {}

#[async_trait::async_trait]
impl KeyPersistence<SubgroupPoint> for KeyHelper<SubgroupPoint> {
    fn curve_type(&self) -> CurveType {
        CurveType::RedJubjub
    }

    fn secret_from_hex(&self, hex_private_key: &str) -> Result<Scalar> {
        Scalar::from_be_hex(hex_private_key).ok_or(unexpected_err(
            "Failed to convert hex to redjubjub private key",
            None,
        ))
    }

    fn secret_from_bytes(&self, private_key: &[u8]) -> Result<Scalar> {
        self.validate_scalar_len(private_key)?;
        Scalar::from_be_bytes(&private_key[..32]).ok_or(unexpected_err(
            "Failed to convert bytes to redjubjub private key",
            None,
        ))
    }

    fn pk_from_hex(&self, hex_public_key: &str) -> Result<SubgroupPoint> {
        SubgroupPoint::from_compressed_hex(hex_public_key).ok_or(unexpected_err(
            "Failed to convert hex to redjubjub  public key",
            None,
        ))
    }

    fn pk_to_hex(&self, public_key: &SubgroupPoint) -> String {
        public_key.to_compressed_hex().to_string()
    }

    fn secret_to_hex(&self, share: &Scalar) -> String {
        share.to_be_hex().to_string()
    }

    fn pk_from_bytes(&self, public_key: &[u8]) -> Result<SubgroupPoint> {
        self.validate_pk_len(public_key)?;
        SubgroupPoint::from_compressed(&public_key[..32]).ok_or(unexpected_err(
            "Failed to convert bytes to redjubjub  public key",
            None,
        ))
    }
}

//        fn private_share_from_bytes(&self, private_key: &[u8]) -> <SubgroupPoint as Group>::Scalar {
//         let bytes : [u8; 32] = private_key[..32].try_into().unwrap();
//         <SubgroupPoint as Group>::Scalar::from_be_bytes(&bytes).unwrap()
//     }

//     fn public_key_from_bytes(&self, public_key: &[u8]) -> SubgroupPoint {
//         let bytes : [u8; 48] = public_key[..48].try_into().unwrap();
//         SubgroupPoint::from_compressed(&bytes).unwrap()
//     }
// }
