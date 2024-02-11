use async_trait::async_trait;
use libsecp256k1::PublicKey;
use lit_blockchain::resolver::contract::ContractResolver;
use sha2::digest::Output;
use sha2::Sha512;
use tracing::trace;

use lit_core::config::LitConfig;

use crate::attestation::DATA_KEY_REQ_BODY_HASH;
use crate::error::{attestation_err_code, Result, EC};
use crate::verification::VerificationPolicy;
use crate::Attestation;

#[async_trait]
pub trait AttestedRequest {
    fn auth(&self) -> &Attestation;
    fn mut_auth(&mut self) -> &mut Attestation;
    fn body_sha512(&self) -> Output<Sha512>;

    fn update(&mut self) -> Result<()> {
        let body_hash = self.body_sha512();

        self.mut_auth().insert_data(DATA_KEY_REQ_BODY_HASH, body_hash.to_vec());

        Ok(())
    }

    async fn verify(
        &self, cfg: &LitConfig, resolver: Option<&ContractResolver>,
        policy: Option<impl VerificationPolicy>,
    ) -> Result<Option<PublicKey>> {
        trace!("AttestedRequest::verify");
        let sig_public_key =
            self.auth().verify_full(cfg, resolver, policy).await.map_err(|e| {
                attestation_err_code(e, EC::AttestationRequestAuthVerifyFailed, None)
            })?;

        let auth_body_hash = self.auth().get_data(DATA_KEY_REQ_BODY_HASH);
        if auth_body_hash.is_none() {
            return Err(attestation_err_code(
                "AttestedRequest auth is missing the body hash data",
                EC::AttestationRequestAuthBodyHashMissing,
                None,
            ));
        }

        let auth_body_hash = auth_body_hash.unwrap().to_vec();
        let body_hash = self.body_sha512();
        let body_hash = body_hash.to_vec();

        if !auth_body_hash.eq(&body_hash) {
            return Err(attestation_err_code(
                "AttestedRequest body hash does not match",
                EC::AttestationRequestAuthBodyHashInvalid,
                None,
            ));
        }

        Ok(sig_public_key)
    }
}
