use std::path::PathBuf;
use std::{fs, sync::Arc};

use futures::FutureExt;

use error::PKG_NAME;
use lit_api_core::logging::TracingPlugin;
use lit_api_core::server::hyper::bind_unix_socket;
use lit_api_core::server::hyper::handler::router::Router;
use lit_core::config::LitConfig;
use lit_logging::config::LitLoggingConfig;

use crate::config::LitLoggingServiceConfig;
use crate::context::{CTX_KEY_CONFIG_CTX, CTX_KEY_LOG_SVC_CTX};
use crate::handler::submit;
use crate::service::log::LogService;

pub(crate) mod config;
pub(crate) mod context;
pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod service;

pub async fn start(
    init_logger: bool, socket_path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cfg = Arc::new(<LitConfig as LitLoggingServiceConfig>::must_new());

    if init_logger {
        // Init logger
        lit_logging::builder(&cfg, PKG_NAME)
            .plugin(TracingPlugin::new())
            // TODO: Logger should have a plugin to send locally
            //.plugin(LogServicePlugin::new())
            .init()
            .expect("failed to init logging");
    }

    let socket_path = socket_path.unwrap_or_else(|| cfg.logging_service_socket_path());

    // Remove existing socket file if needed.
    if socket_path.exists() {
        fs::remove_file(&socket_path)
            .unwrap_or_else(|_| panic!("Unable to remove existing socket: {:?}", &socket_path));
    }

    // Init services
    let log_service =
        LogService::new().start(&cfg).unwrap_or_else(|_| panic!("Unable to start log service"));

    #[rustfmt::skip]
    bind_unix_socket(socket_path, Router::new()
        .attach(CTX_KEY_CONFIG_CTX, cfg.clone())
        .attach(CTX_KEY_LOG_SVC_CTX, Arc::new(log_service))
        .post("/submit", move |req| {
            submit::handle_req(req).boxed()
        }))
        .await;

    Ok(())
}
