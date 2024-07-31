use super::dkg::BasicDkg;
use super::epoch_manager::EpochManager;
use crate::error::Result; // EC , conversion_err_code
use blsful::{Bls12381G2Impl, SignatureShare};
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait Cipherable: BasicDkg + Debug + Send + Sync + EpochManager {
    async fn sign(
        &self,
        message_bytes: Vec<u8>,
        epoch: Option<u64>,
    ) -> Result<(SignatureShare<Bls12381G2Impl>, u32)>;

    async fn sign_with_pubkey(
        &self,
        message_bytes: &[u8],
        public_key: Vec<u8>,
        epoch: Option<u64>,
    ) -> Result<(SignatureShare<Bls12381G2Impl>, u32)>;
}
