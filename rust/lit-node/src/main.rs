#![feature(async_closure)]
#![allow(unused_variables)]

extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate clap;

use std::collections::HashMap;
use std::sync::Arc;

use crate::config::load_cfg;
use crate::error::{unexpected_err_code, EC};
use crate::models::AuthContextCacheExpiry;
use crate::peers::PeerState;
use crate::tasks::fsm_worker::CounterBasedFSMWorkerMetadata;
use crate::tss::common::{
    restore::RestoreState, traits::fsm_worker_metadata::FSMWorkerMetadata, tss_state,
};
use config::chain::ChainDataConfigManager;
use ethers::types::U256;
use lit_api_core::context::{HEADER_KEY_X_CORRELATION_ID, HEADER_KEY_X_REQUEST_ID};
use lit_api_core::error::ApiError;
#[cfg(feature = "testing")]
use lit_api_core::logging::FileLoggingPlugin;
use lit_api_core::logging::TracingPlugin;
use lit_api_core::{Engine, Launcher};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_logging::plugin::log_service::LogServicePlugin;
use lit_node::config::LitNodeConfig;
use lit_node::error::PKG_NAME;
use lit_node::version;

use moka::future::Cache;
use networking::http::client::HttpClientFactory;
use rate_limiting::models::RateLimitDB;
use rocket::fairing::AdHoc;
use rocket::http::Header;
use rocket::http::Method;
use rocket::http::Status;
use rocket::response::status;

#[cfg(feature = "testing")]
use rocket::serde::json::json;
use rocket::serde::json::Value;
use rocket::Request;
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio::sync::{Mutex, RwLock};

mod config;
mod constants;
mod endpoints;
mod models;
mod networking;
mod peers;
mod siwe_db;
mod utils {
    pub mod attestation;
    pub mod chain;
    pub mod consensus;
    pub mod contract;
    pub mod cose_keys;
    pub mod encoding;
    pub mod future;
    pub mod networking;
    pub mod rocket;
    pub mod serde_encrypt;
    pub mod siwe;
    #[allow(dead_code)]
    pub mod web;
}

pub mod access_control;
#[allow(dead_code)]
pub mod auth;
pub mod contracts;
pub mod error;
#[cfg(feature = "lit-actions")]
pub mod functions;
pub mod jwt;
mod node_state;
pub mod pkp;
pub mod rate_limiting;
pub mod services;
pub mod tasks;
#[cfg(test)]
mod tests;
pub mod tss;
// mod test_nodes;

#[catch(422)]
fn bad_input_data_catcher(status: Status, req: &Request<'_>) -> status::Custom<Value> {
    let request_id = req
        .headers()
        .get_one(HEADER_KEY_X_REQUEST_ID)
        .unwrap_or("unknown_req_id")
        .to_string();
    let correlation_id = req
        .headers()
        .get_one(HEADER_KEY_X_CORRELATION_ID)
        .unwrap_or("unknown_correlation_id")
        .to_string();
    error!(
        "{}: Bad data input for request id: {}, correlation id: {}",
        status,
        request_id.as_str(),
        correlation_id.as_str()
    );
    unexpected_err_code("caught rocket 422: bad input error", EC::NodeBadInput, None).handle()
}

#[catch(500)]
fn internal_server_error_catcher(status: Status, req: &Request<'_>) -> status::Custom<Value> {
    let request_id = req
        .headers()
        .get_one(HEADER_KEY_X_REQUEST_ID)
        .unwrap_or("unknown_req_id")
        .to_string();
    let correlation_id = req
        .headers()
        .get_one(HEADER_KEY_X_CORRELATION_ID)
        .unwrap_or("unknown_correlation_id")
        .to_string();
    error!(
        "{}: Internal server error for request id: {}, correlation id: {}",
        status,
        request_id.as_str(),
        correlation_id.as_str()
    );
    unexpected_err_code(
        "caught rocket 500: internal server error",
        EC::NodeSystemFault,
        None,
    )
    .handle()
}

pub fn main() {
    // Load config
    let cfg = load_cfg().expect("failed to load LitConfig");

    let logging_rt = tokio::runtime::Runtime::new().expect("failed to create Logging Runtime");

    let cloned_cfg = cfg.clone();
    logging_rt.spawn_blocking(move || {
        #[cfg(feature = "testing")]
        lit_logging::builder(cloned_cfg.load().as_ref(), PKG_NAME)
            .plugin(FileLoggingPlugin::new())
            .plugin(TracingPlugin::new())
            .plugin(LogServicePlugin::new())
            .add_field(
                "port",
                json!(cloned_cfg
                    .load()
                    .internal_port()
                    .expect("failed to load internal_port")),
            )
            .init()
            .expect("failed to init logging");

        // Init logger
        #[cfg(not(feature = "testing"))]
        lit_logging::builder(cloned_cfg.load().as_ref(), PKG_NAME)
            .plugin(TracingPlugin::new())
            .plugin(LogServicePlugin::new())
            .init()
            .expect("failed to init logging");
    });

    debug!("Loaded config: {:#?}", cfg.load_full());

    // Load contract resolver
    let resolver = Arc::new(
        ContractResolver::try_from(cfg.load().as_ref()).expect("failed to load ContractResolver"),
    );

    let a = 0;
    let addr = cfg
        .load()
        .external_addr()
        .expect("failed to load external_addr");

    let chain_data_manager = Arc::new(ChainDataConfigManager::new(cfg.clone()));

    let (pr_tx, pr_rx) = flume::unbounded();

    let local_rt = tokio::runtime::Runtime::new().expect("failed to create local Runtime");

    // init HTTP client
    let http_client =
        HttpClientFactory::new_client(cfg.load().as_ref()).expect("Unable to init HTTP client");

    let (metrics_tx, metrics_rx) = flume::unbounded();
    let metrics_tx = Arc::new(metrics_tx);
    let port = cfg
        .load()
        .external_port()
        .expect("Unable to load config port");
    siwe_db::db::db_initial_setup(port).expect("Initial SQLite db setup failed");

    let (bm_tx, bm_rx) = flume::unbounded();

    let peer_state = Arc::new(
        local_rt
            .block_on(PeerState::new(
                addr,
                pr_tx.clone(),
                cfg.load_full(),
                chain_data_manager.clone(),
                http_client.clone(),
                bm_tx.clone(),
            ))
            .expect("failed to create PeerState"),
    );

    let (tss_state, rx_round_manager, rx_batch_manager, rx_peer_reviewer_bridge) =
        tss_state::TssState::init(
            peer_state.clone(),
            cfg.load_full(),
            http_client.clone(),
            chain_data_manager.clone(),
            metrics_tx.clone(),
        )
        .expect("Error initializing tss state");

    let tss_state = Arc::new(tss_state);
    let node_state = Arc::new(Mutex::new(node_state::NodeState::new()));

    let rate_limit_db = Arc::new(RateLimitDB::default_with_chain_data_config_manager(
        chain_data_manager.clone(),
    ));
    let cache: Cache<String, crate::models::AuthMethodResponse> = Cache::builder()
        .max_capacity(100_000)
        .expire_after(AuthContextCacheExpiry)
        .build();

    let eth_blockhash_cache = Arc::new(siwe_db::rpc::EthBlockhashCache {
        blockhash: RwLock::new("0x".to_owned()),
        timestamp: RwLock::new(U256::zero()),
    });

    let auth_context_cache = Arc::new(models::AuthContextCache {
        auth_contexts: cache,
    });

    let allowlist_cache = Arc::new(models::AllowlistCache {
        entries: RwLock::new(HashMap::new()),
    });

    let restore_state = RestoreState::new(&cfg.load())
        .expect("`enter_restore_state` is set true but the RestoreState constructor failed.");
    let restore_state = Arc::new(RwLock::new(restore_state));

    let fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64>> =
        Arc::new(CounterBasedFSMWorkerMetadata::new());
    let t = crate::tasks::launch(
        cfg.clone(),
        chain_data_manager,
        eth_blockhash_cache.clone(),
        pr_tx,
        pr_rx,
        auth_context_cache.clone(),
        peer_state.clone(),
        tss_state.clone(),
        node_state.clone(),
        restore_state.clone(),
        tss_state.tx_round_manager.clone(),
        rx_round_manager,
        rx_batch_manager,
        rx_peer_reviewer_bridge,
        bm_rx,
        bm_tx,
        http_client,
        fsm_worker_metadata.clone(),
        metrics_rx,
    )
    .expect("failed to launch tasks");

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("CORS failed to build");

    Engine::new(move || {
        let cfg = cfg.clone();
        let resolver = resolver.clone();
        let cors = cors.clone();

        let node_state = node_state.clone();
        let tss_state = tss_state.clone();
        let peer_state = peer_state.clone();
        let rate_limit_db = rate_limit_db.clone();
        let auth_context_cache = auth_context_cache.clone();
        let eth_blockhash_cache = eth_blockhash_cache.clone();
        let allowlist_cache = allowlist_cache.clone();
        let tx_round_sender = tss_state.tx_round_manager.clone();
        let restore_state = restore_state.clone();
        let metrics_tx = metrics_tx.clone();
        let fsm_worker_metadata = fsm_worker_metadata.clone();

        Box::pin(async move {
            #[allow(unused_mut)]
            let mut l = Launcher::try_new(cfg.clone())
                .expect("failed to construct rocket launcher")
                .mount("/", routes![endpoints::web_client::handshake])
                .mount("/", routes![endpoints::web_client::encryption_sign])
                .mount(
                    "/",
                    routes![endpoints::web_client::signing_access_control_condition],
                )
                .mount("/", endpoints::admin::endpoints::routes())
                .mount("/", routes![pkp::endpoints::web::pkp_sign])
                .mount("/", routes![tss::common::web::internal::connect])
                .attach(cors.clone())
                .attach(AdHoc::on_response("Version Header", |_, resp| {
                    Box::pin(async move {
                        resp.set_header(Header::new("X-Lit-Node-Version", version::NODE_VERSION));
                    })
                }))
                .manage(cfg)
                .manage(resolver)
                .manage(peer_state)
                .manage(tx_round_sender)
                .manage(rate_limit_db)
                .manage(auth_context_cache)
                .manage(eth_blockhash_cache)
                .manage(allowlist_cache)
                .manage(fsm_worker_metadata)
                .manage(metrics_tx)
                .register(
                    "/",
                    catchers![bad_input_data_catcher, internal_server_error_catcher],
                );
            // serverless functions:
            l = l
                .manage(tss_state)
                .manage(restore_state)
                .mount("/", routes![endpoints::web_client::sign_session_key])
                .mount("/", routes![pkp::endpoints::web::pkp_claim])
                .mount("/", routes![tss::common::web::internal::node_share_set])
                .mount("/", routes![tss::common::web::internal::select_triple])
                .mount("/", endpoints::recovery::endpoints::routes())
                .mount(
                    "/",
                    routes![tss::common::web::internal::node_share_batch_set],
                );

            // lit actions endpoints:
            #[cfg(feature = "lit-actions")]
            {
                l = l.mount("/", routes![endpoints::web_client::execute_function]);
            }

            #[cfg(feature = "rtmetrics")]
            {
                l = l.mount("/", routes![endpoints::realtime_metrics::metrics]);
            }

            l
        })
    })
    .start();

    // Shutdown tasks
    t.shutdown();
}
