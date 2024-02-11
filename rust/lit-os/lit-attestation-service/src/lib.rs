use std::path::PathBuf;
use std::{fs, sync::Arc};

use futures::FutureExt;

use config::LitAttestationServiceConfig;
use error::PKG_NAME;
use lit_api_core::logging::TracingPlugin;
use lit_api_core::server::hyper::bind_unix_socket;
use lit_api_core::server::hyper::handler::router::Router;
use lit_attestation::config::LitAttestationConfig;
use lit_core::config::LitConfig;
use lit_logging::plugin::log_service::LogServicePlugin;

use crate::context::{ServiceContext, CTX_KEY_SERVICE_CTX};
use crate::handler::{attestation, attestation_intent, kdf};

mod config;
mod context;
mod data;
mod error;
mod handler;

pub async fn start(
    init_logger: bool, socket_path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cfg = Arc::new(<LitConfig as LitAttestationServiceConfig>::must_new());

    if init_logger {
        // Init logger
        lit_logging::builder(&cfg, PKG_NAME)
            .plugin(TracingPlugin::new())
            .plugin(LogServicePlugin::new())
            .init()
            .expect("failed to init logging");
    }

    let socket_path = socket_path.unwrap_or_else(|| cfg.attestation_service_socket_path());

    // Remove existing socket file if needed.
    if socket_path.exists() {
        fs::remove_file(&socket_path)
            .unwrap_or_else(|_| panic!("Unable to remove existing socket: {:?}", &socket_path));
    }

    #[rustfmt::skip]
    bind_unix_socket(socket_path, Router::new()
        .attach(CTX_KEY_SERVICE_CTX, Arc::new(ServiceContext::from_lit_config(cfg)
            .expect("Unable to init service context")))
        .post("/attestation/intent", move |req| {
            attestation_intent::handle_req(req).boxed()
        })
        .post("/attestation/confirm", move |req| {
            attestation::handle_req(req).boxed()
        })
        .post("/kdf", move |req| {
            kdf::handle_req(req).boxed()
        }))
        .await;

    Ok(())
}
