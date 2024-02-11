use crate::error::Result;
use std::env;

use lit_core::config::{LitConfig, LitConfigBuilder};
use std::path::PathBuf;

const ATTESTATION_SERVICE_SOCK_PATH: &str = "/var/run/lit-attestation-service.sock";

pub const ENV_ATTESTATION_SERVICE_SOCK_PATH: &str = "LIT_ATTESTATION_SERVICE_SOCK_PATH";

pub trait LitAttestationConfig {
    fn apply_defaults(builder: LitConfigBuilder) -> Result<LitConfigBuilder>;
    fn attestation_service_socket_path(&self) -> PathBuf;
}

impl LitAttestationConfig for LitConfig {
    #[inline]
    fn apply_defaults(builder: LitConfigBuilder) -> Result<LitConfigBuilder> {
        // Set defaults

        Ok(builder)
    }

    fn attestation_service_socket_path(&self) -> PathBuf {
        attestation_service_socket_path()
    }
}

pub fn attestation_service_socket_path() -> PathBuf {
    if let Ok(path) = env::var(ENV_ATTESTATION_SERVICE_SOCK_PATH) {
        return PathBuf::from(path);
    }

    PathBuf::from(ATTESTATION_SERVICE_SOCK_PATH)
}
