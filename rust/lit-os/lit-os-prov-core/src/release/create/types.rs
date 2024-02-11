use serde::{Deserialize, Serialize};
use serde_bytes_base64::Bytes;
use serde_json::json;
use sha2::digest::Output;
use sha2::{Digest, Sha512};

use async_trait::async_trait;
use lit_attestation::attestation::TryGenerate;
use lit_attestation::{Attestation, AttestedRequest};
use lit_blockchain::util::release::release_id_bin_from_string;
use lit_core::config::LitConfig;

use crate::error::{validation_err, Result};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRelease {
    release_id: String,
    manifest_cid: String,
    password: Bytes,
    public_key: Bytes,
}

impl CreateRelease {
    pub fn new(
        release_id: String, manifest_cid: String, password: Vec<u8>, public_key: Vec<u8>,
    ) -> Self {
        Self {
            release_id,
            manifest_cid,
            password: Bytes::from(password),
            public_key: Bytes::from(public_key),
        }
    }

    pub fn release_id(&self) -> &String {
        &self.release_id
    }

    pub fn release_id_as_b32_slice(&self) -> Result<[u8; 32]> {
        release_id_bin_from_string(&self.release_id).map_err(|e| {
            validation_err(e, Some("failed to produce zero padded b32 from release_id".into()))
        })
    }

    pub fn manifest_cid(&self) -> &String {
        &self.manifest_cid
    }

    pub fn password(&self) -> &Bytes {
        &self.password
    }

    pub fn public_key(&self) -> &Bytes {
        &self.public_key
    }

    pub fn verify(&self) -> Result<()> {
        if self.release_id.is_empty() {
            return Err(validation_err("missing required field: release_id", None));
        }
        if self.manifest_cid.is_empty() {
            return Err(validation_err("missing required field: manifest_cid", None));
        }
        if self.password.len() == 0 {
            return Err(validation_err("missing required field: password", None));
        }
        if self.public_key.len() == 0 {
            return Err(validation_err("missing required field: public_key", None));
        }

        Ok(())
    }

    pub fn sha512(&self) -> Output<Sha512> {
        let mut hasher = Sha512::new();
        hasher.update("release_id");
        hasher.update(self.release_id.as_bytes());
        hasher.update("manifest_cid");
        hasher.update(self.manifest_cid.as_bytes());
        hasher.update("password");
        hasher.update(self.password.as_slice());
        hasher.update("public_key");
        hasher.update(self.public_key.as_slice());

        hasher.finalize()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReleaseRequest {
    auth: Attestation,
    body: CreateRelease,
}

impl CreateReleaseRequest {
    pub fn new(auth: Attestation, body: CreateRelease) -> Self {
        Self { auth, body }
    }

    pub fn body(&self) -> &CreateRelease {
        &self.body
    }

    pub fn to_vec(&self) -> Vec<u8> {
        json!(self).to_string().as_bytes().to_vec()
    }
}

impl AttestedRequest for CreateReleaseRequest {
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
impl TryGenerate<(CreateRelease, Option<Vec<u8>>)> for CreateReleaseRequest {
    async fn try_generate(
        cfg: &LitConfig, values: (CreateRelease, Option<Vec<u8>>),
    ) -> Result<Self> {
        let (body, noonce) = values;

        CreateReleaseRequest::try_generate(cfg, (body, noonce, None)).await
    }
}

/// body, noonce, key
#[async_trait]
impl TryGenerate<(CreateRelease, Option<Vec<u8>>, Option<String>)> for CreateReleaseRequest {
    async fn try_generate(
        cfg: &LitConfig, values: (CreateRelease, Option<Vec<u8>>, Option<String>),
    ) -> Result<Self> {
        let (body, noonce, key) = values;

        let auth = Attestation::try_generate(cfg, (body.sha512().to_vec(), noonce, key)).await?;

        Ok(Self { auth, body })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReleaseResponse {
    pub success: bool,
    pub release_id: String,
}

impl CreateReleaseResponse {
    pub fn new(success: bool, release_id: String) -> Self {
        Self { success, release_id }
    }
}
