use log::{as_error, warn};

use lit_api_core::server::hyper::handler::types::{Request, Response};
use lit_logging::service::types::{SubmitReq, SubmitResp};

use crate::context::ContextHelper;
use crate::error::Result;

pub(crate) async fn handle_req(req: Request) -> Result<Response> {
    let log_svc = req.ctx().log_service()?;
    let req: SubmitReq = req.deserialize().await?;

    let mut submitted = 0;
    for entry in req.entries.into_iter() {
        if let Err(e) = log_svc.send(entry) {
            warn!(error = as_error!(e); "failed to send log entry to log svc");

            continue;
        }

        submitted += 1;
    }

    // Return response.
    Response::try_from(SubmitResp { submitted })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::{env, sync::Arc};

    use serde_json::Value;

    use lit_api_core::server::hyper::handler::types::{Context, Request};
    use lit_core::config::LitConfig;
    use lit_core::config::ENV_LIT_CONFIG_FILE;
    use lit_logging::config::ENV_LOGGING_SERVICE_SOCK_PATH;
    use lit_logging::service::types::{SubmitReq, SubmitResp};

    use crate::config::LitLoggingServiceConfig;
    use crate::context::{CTX_KEY_CONFIG_CTX, CTX_KEY_LOG_SVC_CTX};
    use crate::handler::submit::handle_req;
    use crate::service::log::LogService;

    const RESOURCES_TEST_DIR: &str = "resources/test";

    #[tokio::test]
    async fn test_handle_req_success() {
        // Init
        setup_test_env();
        let cfg = Arc::new(<LitConfig as LitLoggingServiceConfig>::must_new());
        let log_svc = Arc::new(LogService::new());
        let req_body = SubmitReq::new(vec![
            serde_json::from_str::<Value>("{\"test1\": true}").unwrap(),
            serde_json::from_str::<Value>("{\"test2\": true}").unwrap(),
        ]);

        // Handle request
        let mut ctx = Context::new();
        ctx.insert_raw(CTX_KEY_CONFIG_CTX, cfg.clone());
        ctx.insert_raw(CTX_KEY_LOG_SVC_CTX, log_svc);

        let req = Request::try_from(&req_body, Arc::new(ctx)).expect("failed to extract Request");
        let res = handle_req(req).await;

        // Assertions
        if let Err(e) = res {
            panic!("Res is error: {e:?}")
        }

        let resp: SubmitResp = res.unwrap().deserialize().await.expect("failed to deserialize");

        assert_eq!(resp.submitted, 2);
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
        env::set_var(ENV_LOGGING_SERVICE_SOCK_PATH, "/tmp/lit-logging-service.sock");
    }
}
