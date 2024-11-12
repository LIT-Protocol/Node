use lit_api_core::config::LitApiConfig;
use lit_core::config::{LitConfig, LitConfigBuilder};
use lit_logging::config::LitLoggingConfig;
use lit_os_core::config::LitOsGuestConfig;
use std::path::PathBuf;

use crate::error::Result;

pub(crate) const LOG_SERVICE_DEVICE: &str = "/dev/virtio-ports/com.litprotocol.logging.port0";

pub trait LitLoggingServiceConfig {
    fn try_new() -> Result<LitConfig>;
    fn must_new() -> LitConfig;
    fn from_builder(builder: LitConfigBuilder) -> Result<LitConfig>;
    fn log_service_device(&self) -> PathBuf;
}

impl LitLoggingServiceConfig for LitConfig {
    fn try_new() -> Result<Self> {
        <LitConfig as LitLoggingServiceConfig>::from_builder(LitConfigBuilder::default())
    }

    fn must_new() -> Self {
        <LitConfig as LitLoggingServiceConfig>::try_new().expect("failed to load config")
    }

    fn from_builder(mut builder: LitConfigBuilder) -> Result<LitConfig> {
        // Set defaults
        builder = <LitConfig as LitOsGuestConfig>::apply_defaults(builder)?;
        builder = <LitConfig as LitLoggingConfig>::apply_defaults(builder)?;

        <LitConfig as LitApiConfig>::from_builder(builder)
    }

    fn log_service_device(&self) -> PathBuf {
        PathBuf::from(LOG_SERVICE_DEVICE)
    }
}
