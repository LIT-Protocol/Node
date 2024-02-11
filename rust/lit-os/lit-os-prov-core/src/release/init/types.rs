use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_bytes_base64::Bytes;
use serde_json::json;
use sha2::digest::Output;
use sha2::{Digest, Sha512};

use lit_attestation::attestation::TryGenerate;
use lit_attestation::{Attestation, AttestedRequest};
use lit_core::config::LitConfig;

use crate::error::{validation_err, Result};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitRelease {
    release_id: String,
}

impl InitRelease {
    pub fn new(release_id: String) -> Self {
        Self { release_id }
    }

    pub fn release_id(&self) -> &String {
        &self.release_id
    }

    pub fn verify(&self) -> Result<()> {
        if self.release_id.is_empty() {
            return Err(validation_err("missing required field: release_id", None));
        }

        Ok(())
    }

    pub fn sha512(&self) -> Output<Sha512> {
        let mut hasher = Sha512::new();
        hasher.update("release_id");
        hasher.update(self.release_id.as_bytes());

        hasher.finalize()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitReleaseRequest {
    auth: Attestation,
    body: InitRelease,
}

impl InitReleaseRequest {
    pub fn new(auth: Attestation, body: InitRelease) -> Self {
        Self { auth, body }
    }

    pub fn body(&self) -> &InitRelease {
        &self.body
    }

    pub fn to_vec(&self) -> Vec<u8> {
        json!(self).to_string().as_bytes().to_vec()
    }
}

impl AttestedRequest for InitReleaseRequest {
    fn auth(&self) -> &Attestation {
        &self.auth
    }

    fn mut_auth(&mut self) -> &mut Attestation {
        &mut self.auth
    }

    fn body_sha512(&self) -> Output<Sha512> {
        self.body.sha512()
    }
}

/// body, noonce
#[async_trait]
impl TryGenerate<(InitRelease, Option<Vec<u8>>)> for InitReleaseRequest {
    async fn try_generate(cfg: &LitConfig, values: (InitRelease, Option<Vec<u8>>)) -> Result<Self> {
        let (body, noonce) = values;

        InitReleaseRequest::try_generate(cfg, (body, noonce, None)).await
    }
}

/// body, noonce, key
#[async_trait]
impl TryGenerate<(InitRelease, Option<Vec<u8>>, Option<String>)> for InitReleaseRequest {
    async fn try_generate(
        cfg: &LitConfig, values: (InitRelease, Option<Vec<u8>>, Option<String>),
    ) -> Result<Self> {
        let (body, noonce, key) = values;

        let auth = Attestation::try_generate(cfg, (body.sha512().to_vec(), noonce, key)).await?;

        Ok(Self { auth, body })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitReleaseResponse {
    pub success: bool,
    pub release_id: String,
    pub passphrase: Bytes,
}

impl InitReleaseResponse {
    pub fn new(success: bool, release_id: String, passphrase: Vec<u8>) -> Self {
        Self { success, release_id, passphrase: Bytes::from(passphrase) }
    }
}
