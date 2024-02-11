use lit_api_core::config::{
    http_section_key, https_section_key, LitApiConfig, CFG_KEY_ENABLED, CFG_KEY_IDENT,
    CFG_KEY_PORT, CFG_KEY_REDIRECT_TO_HTTPS, CFG_KEY_TLS_AUTO_ENABLED,
};
use lit_core::config::{LitConfig, LitConfigBuilder, ReloadableLitConfig};

use lit_core::error::Result;
use lit_logging::config::LitLoggingConfig;

pub static CFG_KEY_SESSION_CACHE_SIZE: &str = "session_cache_size";
pub static CFG_KEY_SESSION_CACHE_TTI: &str = "session_cache_tti";
pub static CFG_KEY_TWILLIO_ACCT_ACCESS_TOKEN: &str = "twillio.acct_access_token";
pub static CFG_KEY_TWILLIO_ACCT_SERVICE_ID: &str = "twillio.acct_service_id";
pub static CFG_KEY_TWILLIO_VERIFY_SERVICE_ID: &str = "twillio.verify_service_id";

pub trait LitAuthApiConfig {
    fn try_new() -> Result<LitConfig>;
    fn must_new() -> LitConfig;
    fn from_builder(builder: LitConfigBuilder) -> Result<LitConfig>;
    fn verify(&self) -> Result<()>;
    fn twillio_acct_access_token(&self) -> Result<String>;
    fn twillio_acct_service_id(&self) -> Result<String>;
    fn twillio_verify_service_id(&self) -> Result<String>;
    fn session_cache_size(&self) -> Result<u64>;
    fn session_cache_tti(&self) -> Result<u64>;
}

impl LitAuthApiConfig for LitConfig {
    fn try_new() -> Result<Self> {
        let builder = LitConfigBuilder::default();

        <LitConfig as LitAuthApiConfig>::from_builder(builder)
    }

    fn must_new() -> Self {
        <LitConfig as LitAuthApiConfig>::try_new().expect("failed to load config")
    }

    fn from_builder(mut builder: LitConfigBuilder) -> Result<LitConfig> {
        builder = builder
            .set_key(Some("verification.api".to_string()))
            .set_section_default(CFG_KEY_SESSION_CACHE_SIZE, 10_000)
            .set_section_default(CFG_KEY_SESSION_CACHE_TTI, 5)
            .set_section_default(CFG_KEY_IDENT, "Lit Verification API")
            .set_section_default(http_section_key(CFG_KEY_REDIRECT_TO_HTTPS), "true")
            .set_section_default(http_section_key(CFG_KEY_PORT), "8080")
            .set_section_default(http_section_key(CFG_KEY_ENABLED), "true")
            .set_section_default(https_section_key(CFG_KEY_PORT), "8443")
            .set_section_default(https_section_key(CFG_KEY_ENABLED), "false")
            .set_section_default(CFG_KEY_TLS_AUTO_ENABLED, "false");
        builder = <LitConfig as LitLoggingConfig>::apply_defaults(builder)?;

        <LitConfig as LitApiConfig>::from_builder(builder)
    }

    fn verify(&self) -> Result<()> {
        Ok(())
    }

    fn twillio_acct_access_token(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_TWILLIO_ACCT_ACCESS_TOKEN)
    }

    fn twillio_acct_service_id(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_TWILLIO_ACCT_SERVICE_ID)
    }

    fn twillio_verify_service_id(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_TWILLIO_VERIFY_SERVICE_ID)
    }

    fn session_cache_size(&self) -> Result<u64> {
        match self.get_section_int(CFG_KEY_SESSION_CACHE_SIZE) {
            Ok(size) => Ok(size as u64),
            Err(e) => Err(e),
        }
    }

    fn session_cache_tti(&self) -> Result<u64> {
        match self.get_section_int(CFG_KEY_SESSION_CACHE_TTI) {
            Ok(size) => Ok(size as u64),
            Err(e) => Err(e),
        }
    }
}

pub fn load_cfg() -> Result<ReloadableLitConfig> {
    ReloadableLitConfig::new(|| {
        let cfg = <LitConfig as LitAuthApiConfig>::try_new()?;

        // Verify every load (will not replace running config unless it works)
        cfg.verify()?;

        Ok(cfg)
    })
}
