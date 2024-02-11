use lit_api_core::config::LitApiConfig;
use lit_attestation::config::LitAttestationConfig;
use lit_core::config::{LitConfig, LitConfigBuilder};
use lit_logging::config::LitLoggingConfig;
use lit_os_core::config::LitOsGuestConfig;

use crate::error::Result;

pub trait LitAttestationServiceConfig {
    fn try_new() -> Result<LitConfig>;
    fn must_new() -> LitConfig;
    fn from_builder(builder: LitConfigBuilder) -> Result<LitConfig>;
}

impl LitAttestationServiceConfig for LitConfig {
    fn try_new() -> Result<Self> {
        <LitConfig as LitAttestationServiceConfig>::from_builder(LitConfigBuilder::default())
    }

    fn must_new() -> Self {
        <LitConfig as LitAttestationServiceConfig>::try_new().expect("failed to load config")
    }

    fn from_builder(mut builder: LitConfigBuilder) -> Result<LitConfig> {
        // Set defaults
        builder = <LitConfig as LitOsGuestConfig>::apply_defaults(builder)?;
        builder = <LitConfig as LitLoggingConfig>::apply_defaults(builder)?;
        builder = <LitConfig as LitAttestationConfig>::apply_defaults(builder)?;

        <LitConfig as LitApiConfig>::from_builder(builder)
    }
}
