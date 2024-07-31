use crate::error::Result;
use lit_core::config::{LitConfig, LitConfigBuilder};
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::path::PathBuf;

pub const CFG_KEY_LOGGING_LEVEL: &str = "logging.level";
pub const CFG_KEY_LOGGING_TIMESTAMP: &str = "logging_timestamp";
pub const CFG_KEY_LOGGING_JAEGER: &str = "logging_jaeger";

pub const DEFAULT_LOGGING_LEVEL: &str = "info";

#[cfg(feature = "service")]
const LOGGING_SERVICE_SOCK_PATH: &str = "/var/run/lit-logging-service.sock";
#[cfg(feature = "service")]
pub const ENV_LOGGING_SERVICE_SOCK_PATH: &str = "LIT_LOGGING_SERVICE_SOCK_PATH";
#[cfg(feature = "service")]
pub const ENV_LOGGING_SERVICE_DISABLED: &str = "LIT_LOGGING_SERVICE_DISABLED";
#[cfg(feature = "service")]
pub const ENV_LOGGING_TIMESTAMP: &str = "LIT_LOGGING_TIMESTAMP";

pub trait LitLoggingConfig {
    fn apply_defaults(builder: LitConfigBuilder) -> Result<LitConfigBuilder>;
    fn logging_level(&self) -> Result<String>;
    fn logging_timestamp(&self) -> bool;
    fn logging_jaeger(&self) -> bool;
    #[cfg(feature = "service")]
    fn logging_service_enabled(&self) -> bool;
    #[cfg(feature = "service")]
    fn logging_service_socket_path(&self) -> PathBuf;
}

impl LitLoggingConfig for LitConfig {
    #[inline]
    fn apply_defaults(mut builder: LitConfigBuilder) -> Result<LitConfigBuilder> {
        // Set defaults
        builder = builder.set_default(CFG_KEY_LOGGING_LEVEL, DEFAULT_LOGGING_LEVEL);

        Ok(builder)
    }

    #[inline]
    fn logging_level(&self) -> Result<String> {
        self.get_string(CFG_KEY_LOGGING_LEVEL)
    }

    #[inline]
    fn logging_timestamp(&self) -> bool {
        self.get_bool(CFG_KEY_LOGGING_TIMESTAMP).unwrap_or_default() && !self.is_prod()
    }

    #[inline]
    fn logging_jaeger(&self) -> bool {
        self.get_bool(CFG_KEY_LOGGING_JAEGER).unwrap_or_default() && !self.is_prod()
    }

    #[inline]
    #[cfg(feature = "service")]
    fn logging_service_enabled(&self) -> bool {
        if env::var(ENV_LOGGING_SERVICE_DISABLED).is_ok() {
            return false;
        }

        self.is_litos_guest()
    }

    #[cfg(feature = "service")]
    fn logging_service_socket_path(&self) -> PathBuf {
        logging_service_socket_path()
    }
}

#[cfg(feature = "service")]
pub fn logging_service_socket_path() -> PathBuf {
    if let Ok(path) = env::var(ENV_LOGGING_SERVICE_SOCK_PATH) {
        return PathBuf::from(path);
    }

    PathBuf::from(LOGGING_SERVICE_SOCK_PATH)
}
