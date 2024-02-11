use std::sync::Arc;

use lit_api_core::server::hyper::handler::types::Context;
use lit_core::config::LitConfig;

use crate::error::{unexpected_err, Result};
use crate::service::log::LogService;

pub(crate) const CTX_KEY_CONFIG_CTX: &str = "CFG";
pub(crate) const CTX_KEY_LOG_SVC_CTX: &str = "LOG_SVC";

pub(crate) trait ContextHelper {
    fn cfg(&self) -> Result<Arc<LitConfig>>;
    fn log_service(&self) -> Result<Arc<LogService>>;
}

impl ContextHelper for Arc<Context> {
    fn cfg(&self) -> Result<Arc<LitConfig>> {
        match self.get(CTX_KEY_CONFIG_CTX) {
            Some(v) => {
                let v: Arc<LitConfig> = v.clone().downcast().map_err(|_e| {
                    unexpected_err(
                        format!("Failed to downcast context key: {CTX_KEY_CONFIG_CTX}"),
                        None,
                    )
                })?;

                Ok(v)
            }
            None => Err(unexpected_err(
                format!("Missing expected context key: {CTX_KEY_CONFIG_CTX}"),
                None,
            )),
        }
    }

    fn log_service(&self) -> Result<Arc<LogService>> {
        match self.get(CTX_KEY_LOG_SVC_CTX) {
            Some(v) => {
                let v: Arc<LogService> = v.clone().downcast().map_err(|_e| {
                    unexpected_err(
                        format!("Failed to downcast context key: {CTX_KEY_LOG_SVC_CTX}"),
                        None,
                    )
                })?;

                Ok(v)
            }
            None => Err(unexpected_err(
                format!("Missing expected context key: {CTX_KEY_LOG_SVC_CTX}"),
                None,
            )),
        }
    }
}
