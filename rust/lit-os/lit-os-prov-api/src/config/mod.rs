use lit_api_core::config::{
    http_section_key, LitApiConfig, CFG_KEY_IDENT, CFG_KEY_REDIRECT_TO_HTTPS,
};
use lit_core::config::{LitConfig, LitConfigBuilder, ReloadableLitConfig};
use lit_logging::config::LitLoggingConfig;
use lit_os_core::config::LitOsConfig;
use lit_os_core::error::Result;
use lit_os_prov_core::config::LitOsProvConfig;

pub trait LitApiProvConfig {
    fn try_new() -> Result<LitConfig>;
    fn must_new() -> LitConfig;
    fn from_builder(builder: LitConfigBuilder) -> Result<LitConfig>;
    fn verify(&self) -> Result<()>;
}

impl LitApiProvConfig for LitConfig {
    fn try_new() -> Result<Self> {
        <LitConfig as LitApiProvConfig>::from_builder(LitConfigBuilder::default())
    }

    fn must_new() -> Self {
        <LitConfig as LitApiProvConfig>::try_new().expect("failed to load config")
    }

    fn from_builder(mut builder: LitConfigBuilder) -> Result<LitConfig> {
        // Set defaults
        builder = builder
            .set_key(Some("api.prov".to_string()))
            .set_section_default(CFG_KEY_IDENT, "Lit Prov API")
            .set_section_default(http_section_key(CFG_KEY_REDIRECT_TO_HTTPS), "true");
        builder = <LitConfig as LitOsConfig>::apply_defaults(builder)?;
        builder = <LitConfig as LitOsProvConfig>::apply_defaults(builder)?;
        builder = <LitConfig as LitLoggingConfig>::apply_defaults(builder)?;

        <LitConfig as LitApiConfig>::from_builder(builder)
    }

    fn verify(&self) -> Result<()> {
        Ok(())
    }
}

pub fn load_cfg() -> Result<ReloadableLitConfig> {
    ReloadableLitConfig::new(|| {
        let cfg = <LitConfig as LitApiProvConfig>::try_new()?;

        // Verify every load (will not replace running config unless it works)
        cfg.verify()?;

        Ok(cfg)
    })
}
