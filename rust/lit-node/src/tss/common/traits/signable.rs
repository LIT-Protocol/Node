use super::dkg::BasicDkg;
use super::epoch_manager::EpochManager;
use crate::error::Result; // EC , conversion_err_code
use crate::p2p_comms::web::models::SignedMessageShare;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait Signable: BasicDkg + EpochManager + Debug + Send + Sync {
    async fn sign(
        &self,
        message_bytes: Vec<u8>,
        nonce_bytes: Vec<u8>,
        epoch: Option<u64>,
    ) -> Result<SignedMessageShare>;

    async fn sign_with_pubkey(
        &mut self,
        message_bytes: &[u8],
        public_key: Vec<u8>,
        root_pubkeys: Option<Vec<String>>,
        tweak_preimage: Option<Vec<u8>>,
        request_id: Vec<u8>,
        epoch: Option<u64>,
    ) -> Result<SignedMessageShare>;

    fn failed_message_share(&self) -> SignedMessageShare {
        SignedMessageShare {
            digest: "".to_string(),
            result: "fail".to_string(),
            signature_share: "".to_string(),
            share_index: 0_usize,
            big_r: "".to_string(),
            public_key: "".to_string(),
            sig_type: "".to_string(),
        }
    }
}
