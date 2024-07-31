#[macro_use]
extern crate rocket;

use std::sync::Arc;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

use lit_api_core::logging::TracingPlugin;
use lit_api_core::{Engine, Launcher};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_logging::plugin::log_service::LogServicePlugin;

use crate::config::load_cfg;
use crate::error::PKG_NAME;

pub(crate) mod api;
pub mod config;
pub mod error;
pub(crate) mod release;

pub fn main() {
    // Init config
    let cfg = load_cfg().expect("failed to load config");

    // Init logger
    lit_logging::builder(cfg.load().as_ref(), PKG_NAME)
        .plugin(TracingPlugin::new())
        .plugin(LogServicePlugin::new())
        .init()
        .expect("failed to init logging");

    // Init resolver
    let resolver = Arc::new(
        ContractResolver::try_from(cfg.load().as_ref())
            .expect("failed to construct ContractResolver"),
    );

    // Init rocket
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch].into_iter().map(From::from).collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("CORS failed to build");

    Engine::new(move || {
        let cfg = cfg.clone();
        let resolver = resolver.clone();
        let cors = cors.clone();

        Box::pin(async move {
            Launcher::try_new(cfg.clone(), None)
                .expect("failed to construct rocket launcher")
                .mount("/", api::status::routes())
                .mount("/", api::attest::routes())
                .mount("/release", api::release::routes())
                .attach(cors.clone())
                .manage(cfg)
                .manage(resolver)
        })
    })
    .start();
}
