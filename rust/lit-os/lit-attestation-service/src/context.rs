use lit_api_core::server::hyper::handler::types::Context;
use std::{fs, sync::Arc};

use lit_core::error::Unexpected;
use lit_core::{
    config::LitConfig,
    utils::{binary::bytes_to_hex, pem::ec_key_pem_to_bytes},
};
use lit_os_core::config::LitOsGuestConfig;

use crate::error::{conversion_err, io_err, unexpected_err, Result};

pub(crate) const CTX_KEY_SERVICE_CTX: &str = "CTX";

#[derive(Debug, Clone)]
pub struct ServiceContext {
    pub cfg: Arc<LitConfig>,
    pub build_priv_key: String,
    pub instance_id: String,
    pub build_id: String,
    pub release_id: Option<String>,
    pub subnet_id: Option<String>,
}

impl ServiceContext {
    pub fn from_lit_config(lit_config: Arc<LitConfig>) -> Result<Self> {
        // Parse build priv key
        let build_priv_key_path = lit_config.litos_build_priv_key_path();
        let build_priv_key_str =
            String::from_utf8(fs::read(build_priv_key_path.as_path()).map_err(|e| {
                io_err(
                    e,
                    Some(format!("failed to read build priv key: {:?}", &build_priv_key_path)),
                )
            })?)
            .map_err(|e| {
                conversion_err(
                    e,
                    Some(format!(
                        "Failed to parse build priv key ({:?}) pem as utf8",
                        &build_priv_key_path
                    )),
                )
            })?;
        let build_priv_key =
            bytes_to_hex(ec_key_pem_to_bytes(build_priv_key_str.as_str()).map_err(|e| {
                conversion_err(
                    e,
                    Some(format!(
                        "failed to read build priv key ({:?}) as EC Key pem",
                        &build_priv_key_path
                    )),
                )
            })?);

        let instance_id = lit_config.litos_guest_instance_id()?;

        // Optional configs that may not be present.
        let build_id =
            lit_config.litos_build_id().expect_or_err("expected build id to be present")?;
        let release_id = lit_config.litos_guest_release_id().ok();
        let subnet_id = lit_config.subnet_id().ok();

        Ok(Self { cfg: lit_config, build_priv_key, build_id, instance_id, release_id, subnet_id })
    }
}

pub(crate) trait ContextHelper {
    fn service_ctx(&self) -> Result<Arc<ServiceContext>>;
}

impl ContextHelper for Arc<Context> {
    fn service_ctx(&self) -> Result<Arc<ServiceContext>> {
        match self.get(CTX_KEY_SERVICE_CTX) {
            Some(v) => {
                let v: Arc<ServiceContext> = v.clone().downcast().map_err(|_e| {
                    unexpected_err(
                        format!("Failed to downcast context key: {CTX_KEY_SERVICE_CTX}"),
                        None,
                    )
                })?;

                Ok(v)
            }
            None => Err(unexpected_err(
                format!("Missing expected context key: {CTX_KEY_SERVICE_CTX}"),
                None,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;
    use std::sync::Arc;

    use lit_core::config::{LitConfig, ENV_LIT_CONFIG_FILE};
    use lit_os_core::config::ENV_BUILD_PRIV_KEY_PATH;

    use crate::config::LitAttestationServiceConfig;

    const RESOURCES_TEST_DIR: &str = "resources/test";

    #[test]
    fn test_service_context_from_lit_config() {
        setup_test_env();

        let cfg = Arc::new(<LitConfig as LitAttestationServiceConfig>::must_new());
        let ctx =
            super::ServiceContext::from_lit_config(cfg).expect("Failed to create ServiceContext");

        // Assertions
        assert_eq!(ctx.build_priv_key.len(), 64);
        assert_eq!(ctx.instance_id.len(), 8);
        assert_eq!(ctx.build_id.len(), 8);

        // Everything else is optional
    }

    // Utils
    fn get_test_path(path: &str) -> PathBuf {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push(RESOURCES_TEST_DIR);
        test_path.push(path);
        test_path
    }

    fn setup_test_env() {
        env::set_var(ENV_LIT_CONFIG_FILE, get_test_path("config/config.toml"));
        env::set_var(ENV_BUILD_PRIV_KEY_PATH, get_test_path("config/build.pem"));
    }
}
