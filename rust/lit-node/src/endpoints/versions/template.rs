// this commented code is left as part of the example of how to add a new version of an endpoint
// use crate::endpoints::admin::guards::AdminAuthSig;
// use crate::endpoints::{admin, pkp, recovery, web_client};
// use crate::models;
// use crate::rate_limiting::models::RateLimitDB;
// use crate::siwe_db::rpc::EthBlockhashCache;
// use crate::tss::common::restore::RestoreState;
// use crate::tss::common::tss_state::TssState;
// use crate::utils::rocket::guards::{ClientContext, RequestHeaders};
// use lit_api_core::context::{SdkVersion, Tracing, TracingRequired};
// use lit_api_core::http::rocket::helper::stream::ChildStream;
// use lit_core::config::ReloadableLitConfig;
// use rocket::response::status;
// use rocket::serde::json::{Json, Value};
// use rocket::{Data, Route, State};
// use std::net::SocketAddr;
// use std::sync::Arc;
// use tokio::sync::RwLock;
// use tracing::instrument;
use rocket::Route;

#[allow(dead_code)]
pub(crate) fn routes() -> Vec<Route> {
    routes![]
}

// you can delete this comment and replace it with the actual code
// #[post(
//     "/web/pkp/sign/v1",
//     format = "json",
//     data = "<json_pkp_signing_request>"
// )]
// #[instrument(name = "POST /web/pkp/sign/v1", skip_all, ret)]
// #[allow(clippy::too_many_arguments)]
// pub(crate) async fn pkp_sign(
//     remote_addr: SocketAddr,
//     tss_state: &State<Arc<TssState>>,
//     auth_context_cache: &State<Arc<models::AuthContextCache>>,
//     rate_limit_db: &State<Arc<RateLimitDB>>,
//     cfg: &State<ReloadableLitConfig>,
//     allowlist_cache: &State<Arc<models::AllowlistCache>>,
//     json_pkp_signing_request: Json<models::JsonPKPSigningRequest>,
//     tracing: Tracing,
// ) -> status::Custom<Value> {
//     pkp::pkp_sign(
//         guard,
//         remote_addr,
//         tss_state,
//         auth_context_cache,
//         rate_limit_db,
//         cfg,
//         allowlist_cache,
//         json_pkp_signing_request,
//         tracing,
//         client_context,
//     )
//     .await
// }
