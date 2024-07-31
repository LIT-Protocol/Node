use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::digest::Output;
use sha2::{Digest, Sha512};

use lit_attestation::attestation::TryGenerate;
use lit_attestation::{Attestation, AttestedRequest};
use lit_core::config::LitConfig;
pub use lit_os_core::guest::types::GuestCpuType;

use crate::error::{validation_err, Result};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelease {
    release_id: String,
    vcpu_type: GuestCpuType,
    vcpus: u16,
}

impl IssueRelease {
    pub fn new(release_id: String, vcpu_type: GuestCpuType, vcpus: u16) -> Self {
        Self { release_id, vcpu_type, vcpus }
    }

    pub fn release_id(&self) -> &String {
        &self.release_id
    }

    pub fn vcpu_type(&self) -> &GuestCpuType {
        &self.vcpu_type
    }

    pub fn vcpus(&self) -> u16 {
        self.vcpus
    }

    pub fn verify(&self) -> Result<()> {
        if self.release_id.is_empty() {
            return Err(validation_err("missing required field: release_id", None));
        }
        if self.vcpus == 0 {
            return Err(validation_err("missing required field: vcpus", None));
        }

        Ok(())
    }

    pub fn sha512(&self) -> Output<Sha512> {
        let mut hasher = Sha512::new();
        hasher.update("release_id");
        hasher.update(self.release_id.as_bytes());
        hasher.update("vcpu_type");
        hasher.update(self.vcpu_type.to_string().as_bytes());
        hasher.update("vcpus");
        hasher.update(self.vcpus.to_le_bytes());

        hasher.finalize()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueReleaseRequest {
    auth: Attestation,
    body: IssueRelease,
}

impl IssueReleaseRequest {
    pub fn new(auth: Attestation, body: IssueRelease) -> Self {
        Self { auth, body }
    }

    pub fn body(&self) -> &IssueRelease {
        &self.body
    }

    pub fn to_vec(&self) -> Vec<u8> {
        json!(self).to_string().as_bytes().to_vec()
    }
}

impl AttestedRequest for IssueReleaseRequest {
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
impl TryGenerate<(IssueRelease, Option<Vec<u8>>)> for IssueReleaseRequest {
    async fn try_generate(
        cfg: &LitConfig, values: (IssueRelease, Option<Vec<u8>>),
    ) -> Result<Self> {
        let (body, noonce) = values;

        IssueReleaseRequest::try_generate(cfg, (body, noonce, None)).await
    }
}

/// body, noonce, key
#[async_trait]
impl TryGenerate<(IssueRelease, Option<Vec<u8>>, Option<String>)> for IssueReleaseRequest {
    async fn try_generate(
        cfg: &LitConfig, value: (IssueRelease, Option<Vec<u8>>, Option<String>),
    ) -> Result<Self> {
        let (body, noonce, key) = value;

        let auth = Attestation::try_generate(cfg, (body.sha512().to_vec(), noonce, key)).await?;

        Ok(Self { auth, body })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueReleaseResponse {
    pub success: bool,
    pub release_id: String,
    pub artifacts: Vec<String>,
}

impl IssueReleaseResponse {
    pub fn new(success: bool, release_id: String, artifacts: Vec<String>) -> Self {
        Self { success, release_id, artifacts }
    }
}
