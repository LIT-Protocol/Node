use serde::{Deserialize, Serialize};

use lit_blockchain::contracts::release::{ReleasePlatform, ReleaseStatus, ReleaseType};
use lit_core::config::envs::LitEnv;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    id: String,
    env: LitEnv,
    status: ReleaseStatus,
    typ: ReleaseType,
    platform: ReleasePlatform,
    manifest_cid: String,
}

impl Release {
    pub(crate) fn new(
        id: String, env: LitEnv, status: ReleaseStatus, typ: ReleaseType,
        platform: ReleasePlatform, manifest_cid: String,
    ) -> Self {
        Self { id, env, status, typ, platform, manifest_cid }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn env(&self) -> &LitEnv {
        &self.env
    }

    pub fn status(&self) -> &ReleaseStatus {
        &self.status
    }

    pub fn typ(&self) -> &ReleaseType {
        &self.typ
    }

    pub fn platform(&self) -> &ReleasePlatform {
        &self.platform
    }

    pub fn manifest_cid(&self) -> &String {
        &self.manifest_cid
    }
}
