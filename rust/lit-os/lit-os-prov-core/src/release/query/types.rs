use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use lit_core::config::envs::LitEnv;
pub use lit_os_core::guest::types::{GuestCpuType, GuestType};

use crate::error::{validation_err, Result};
use crate::release::common::types::Release;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestTypeQuery {
    typ: GuestType,
    kind: Option<String>,
}

impl GuestTypeQuery {
    pub fn new(typ: GuestType, kind: Option<String>) -> Self {
        Self { typ, kind }
    }

    pub fn typ(&self) -> GuestType {
        self.typ
    }

    pub fn kind(&self) -> Option<&String> {
        self.kind.as_ref()
    }

    /// Will return a binary of the kind string (or default to an empty string)
    pub fn kind_to_vec(&self) -> Vec<u8> {
        self.kind.clone().unwrap_or_default().as_bytes().to_vec()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryReleaseRequest {
    env: LitEnv,
    types: Vec<GuestTypeQuery>,
    vcpu_type: GuestCpuType,
}

impl QueryReleaseRequest {
    pub fn new(env: LitEnv, vcpu_type: GuestCpuType) -> Self {
        Self { env, types: Vec::new(), vcpu_type }
    }

    pub fn with_type(mut self, typ: GuestType, kind: Option<String>) -> Self {
        self.types.push(GuestTypeQuery::new(typ, kind));
        self
    }

    pub fn verify(&self) -> Result<()> {
        if self.types.is_empty() {
            return Err(validation_err("missing required field: types", None));
        }

        Ok(())
    }

    // Accessors
    pub fn env(&self) -> &LitEnv {
        &self.env
    }

    pub fn types(&self) -> &Vec<GuestTypeQuery> {
        &self.types
    }

    pub fn vcpu_type(&self) -> &GuestCpuType {
        &self.vcpu_type
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryReleaseResponse {
    pub success: bool,
    pub releases: HashMap<GuestType, Release>,
}

impl QueryReleaseResponse {
    pub fn new(success: bool, releases: HashMap<GuestType, Release>) -> Self {
        Self { success, releases }
    }
}
