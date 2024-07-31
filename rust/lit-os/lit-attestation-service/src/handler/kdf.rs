use lit_api_core::server::hyper::handler::types::{Request, Response};
use log::{as_serde, trace};
use serde_bytes_base64::Bytes;

use crate::context::ContextHelper;
use lit_attestation::kdf::Kdf;
use lit_attestation::service::types::{KdfReq, KdfResp};

use crate::error::Result;

const KDF_NS_PREFIX: &str = "AS";

pub(crate) async fn handle_req(req: Request) -> Result<Response> {
    let ctx = req.ctx().service_ctx()?;
    let req: KdfReq = req.deserialize().await?;

    trace!(req = as_serde!(req); "kdf::handle_req");

    // Full context
    let context = format!("{}::{}", KDF_NS_PREFIX, &req.context);

    // Derive key
    let key = Kdf::try_derive(ctx.cfg.as_ref(), context).await?;

    // Return response.
    Response::try_from(KdfResp { key: Bytes::from(key.to_vec()) })
}

/// Note that this test is only intended to be run with AMD SEV SNP firmware support.
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::{env, sync::Arc};

    use lit_api_core::server::hyper::handler::types::{Context, Request};

    use lit_attestation::config::ENV_ATTESTATION_SERVICE_SOCK_PATH;
    use lit_attestation::kdf::ENV_KDF_TYPE_OVERRIDE;
    use lit_attestation::service::types::{KdfReq, KdfResp};
    use lit_core::config::LitConfig;
    use lit_core::config::ENV_LIT_CONFIG_FILE;

    use crate::config::LitAttestationServiceConfig;
    use crate::context::{ServiceContext, CTX_KEY_SERVICE_CTX};
    use crate::handler::kdf::handle_req;

    const RESOURCES_TEST_DIR: &str = "resources/test";

    #[tokio::test]
    async fn test_handle_req_success() {
        // Init
        setup_test_env();
        let cfg = Arc::new(<LitConfig as LitAttestationServiceConfig>::must_new());
        let req_body = KdfReq::new("test-123");

        // Handle request
        let mut ctx = Context::new();
        ctx.insert(CTX_KEY_SERVICE_CTX, ServiceContext::from_lit_config(cfg.clone()).unwrap());

        let req = Request::try_from(&req_body, Arc::new(ctx)).expect("failed to extract Request");
        let res = handle_req(req).await;

        // Assertions
        if let Err(e) = res {
            panic!("Res is error: {e:?}")
        }

        let kdf_platform_resp: KdfResp =
            res.expect("failed to request kdf").deserialize().await.expect("failed to deserialize");

        assert!(!kdf_platform_resp.key.is_empty());
    }

    // Util
    fn get_test_path(path: &str) -> PathBuf {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push(RESOURCES_TEST_DIR);
        test_path.push(path);
        test_path
    }

    fn setup_test_env() {
        env::set_var(ENV_LIT_CONFIG_FILE, get_test_path("config/config.toml"));
        env::set_var(ENV_ATTESTATION_SERVICE_SOCK_PATH, "/tmp/lit-attestation-service.sock");
        env::set_var(ENV_KDF_TYPE_OVERRIDE, "AMD_SEV_SNP");

        if !PathBuf::from("/dev/sev-guest").exists() {
            env::set_var(ENV_KDF_TYPE_OVERRIDE, "LOCAL");
        }
    }
}
