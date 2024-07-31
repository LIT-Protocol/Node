use std::time::Duration;

use crate::ecdsa::SigningPair;
use crate::errors::PKG_NAME;
use crate::limit::rocket_governor_catcher;
use crate::models::CacheEntry;
use config::{load_cfg, LitAuthApiConfig};
use ecdsa::endpoints::web::{get_key_eth, get_key_secp};
use jwx::endpoints::web::verify_jwt;
use lit_api_core::logging::TracingPlugin;
use lit_api_core::{Engine, Launcher};
use lit_logging::plugin::log_service::LogServicePlugin;
use lit_os_core::error::attestation_err;
use log::trace;
use moka::future::Cache;
use rocket::http::Method;
use rocket::{catchers, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use twillio::endpoints::web::{check_request_id, check_verification, start_verification};

mod api_key;
mod client_ip;
mod config;
mod constants;
mod ecdsa;
mod errors;
mod jwx;
mod limit;
mod models;
mod twillio;
mod utils;

pub fn main() {
    // Init config
    let cfg = load_cfg().expect("failed to load config");
    let loaded_cfg = cfg.load_full();
    // Init logger
    lit_logging::builder(cfg.load().as_ref(), PKG_NAME)
        .plugin(TracingPlugin::new())
        .plugin(LogServicePlugin::new())
        .init()
        .expect("failed to init logging");

    let cache_size = loaded_cfg.session_cache_size().expect("Error while configuring cache size");
    trace!("Configuring session cache with max entries: {}", cache_size);
    let cache_tti =
        loaded_cfg.session_cache_tti().expect("Error while configuring cache expiration");
    trace!("Configuring session cache with tti: {} minutes", cache_tti);

    let request_id_context_cache: Cache<String, CacheEntry> = Cache::builder()
        .time_to_idle(Duration::from_secs(cache_tti * 60))
        .max_capacity(cache_size)
        .build();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch].into_iter().map(From::from).collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("CORS failed to configure");

    Engine::new(move || {
        let cfg = cfg.clone();
        let cors = cors.clone();
        let request_id_content_cache = request_id_context_cache.clone();

        Box::pin(async move {
            let key_shares = SigningPair::from_seed(cfg.to_owned())
                .await
                .map_err(|e| {
                    attestation_err(e, Some("Error while deriving singing key".to_string()))
                })
                .unwrap(); // probably shouldnt unwrap this

            #[allow(unused_mut)]
            let mut l = Launcher::try_new(cfg.clone())
                .expect("Failed to construct launcher")
                .attach(cors.clone())
                .attach(rocket_governor::LimitHeaderGen::default())
                .register("/", catchers![rocket_governor_catcher])
                .mount("/", routes![get_key_eth])
                .mount("/", routes![get_key_secp])
                .mount("/", routes![check_request_id])
                .mount("/", routes![verify_jwt])
                .mount("/", routes![start_verification])
                .mount("/", routes![check_verification])
                .manage(cfg)
                .manage(key_shares)
                .manage(request_id_content_cache);

            l
        })
    })
    .start();
}
