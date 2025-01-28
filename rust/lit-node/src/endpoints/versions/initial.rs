use crate::endpoints::admin::guards::AdminAuthSig;
use crate::endpoints::{admin, pkp, recovery, web_client};
use crate::models;
use crate::rate_limiting::models::RateLimitDB;
use crate::siwe_db::rpc::EthBlockhashCache;
use crate::tss::common::restore::RestoreState;
use crate::tss::common::tss_state::TssState;
use crate::tss::dkg::curves::common::CurveType;
use crate::utils::rocket::guards::RequestHeaders;
use crate::utils::web::{with_context_and_timeout, with_timeout, EndpointVersion};
use lit_api_core::context::{SdkVersion, Tracing, TracingRequired};
use lit_api_core::http::rocket::helper::stream::ChildStream;
use lit_core::config::ReloadableLitConfig;
use moka::future::Cache;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use rocket::{Data, Route, State};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::instrument;

#[allow(dead_code)]
pub(crate) fn routes() -> Vec<Route> {
    routes![
        admin_set,
        admin_get,
        admin_get_key_backup,
        admin_set_key_backup,
        admin_get_blinders,
        recovery_set_dec_share,
        recovery_get_dec_key_share,
        recovery_delete_dec_key_share,
        handshake,
        encryption_sign,
        signing_access_control_condition,
        sign_session_key,
        pkp_sign,
        pkp_claim,
        execute_function
    ]
}

#[post("/web/admin/set", format = "json", data = "<request>")]
#[instrument(name = "POST /web/admin/set", skip_all, ret)]
pub async fn admin_set(
    remote_addr: SocketAddr,
    reloadable_cfg: &State<ReloadableLitConfig>,
    request: Json<models::JsonAdminSetRequest>,
) -> status::Custom<Value> {
    admin::endpoints::admin_set(remote_addr, reloadable_cfg, request).await
}

#[post("/web/admin/get", format = "json", data = "<auth>")]
#[instrument(name = "POST /web/admin/get", skip_all, ret)]
pub async fn admin_get(
    cfg: &State<ReloadableLitConfig>,
    auth: Json<models::AdminAuth>,
) -> status::Custom<Value> {
    admin::endpoints::admin_get(cfg, auth).await
}

#[get("/web/admin/get_key_backup?<auth>&<node_set_hash>")]
#[instrument(name = "GET /web/admin/get_key_backup", skip_all, ret)]
pub async fn admin_get_key_backup(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RwLock<RestoreState>>>,
    auth: models::AdminAuth,
    node_set_hash: Option<String>,
) -> Result<ChildStream, status::Custom<Value>> {
    admin::endpoints::admin_get_key_backup(cfg, restore_state, auth, node_set_hash).await
}

#[post("/web/admin/set_key_backup", format = "binary", data = "<data>")]
#[instrument(name = "POST /web/admin/set_key_backup", skip_all, ret)]
pub async fn admin_set_key_backup(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RwLock<RestoreState>>>,
    admin_auth_sig: AdminAuthSig,
    data: Data<'_>,
) -> status::Custom<Value> {
    admin::endpoints::admin_set_key_backup(cfg, restore_state, admin_auth_sig, data).await
}

#[post("/web/admin/get_blinders", format = "json", data = "<auth>")]
#[instrument(name = "POST /web/admin/get_blinders", skip_all, ret)]
pub async fn admin_get_blinders(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RwLock<RestoreState>>>,
    auth: Json<models::AdminAuth>,
) -> status::Custom<Value> {
    admin::endpoints::admin_get_blinders(cfg, restore_state, auth).await
}

#[post("/web/recovery/set_dec_share", format = "json", data = "<request>")]
#[instrument(name = "POST /web/recovery/set_dec_share", skip_all, ret)]
#[allow(dead_code)]
pub async fn recovery_set_dec_share(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RwLock<RestoreState>>>,
    request: Json<models::JsonRecoverySetDecShare>,
) -> status::Custom<Value> {
    recovery::endpoints::recovery_set_dec_share(cfg, restore_state, request).await
}

#[post("/web/recovery/get_dec_share", format = "json", data = "<request>")]
#[instrument(name = "POST /web/recovery/get_dec_share", skip_all, ret)]
pub async fn recovery_get_dec_key_share(
    restore_state: &State<Arc<TssState>>,
    cfg: &State<ReloadableLitConfig>,
    request: Json<models::DownloadShareRequest>,
) -> status::Custom<Value> {
    recovery::endpoints::recovery_get_dec_key_share(restore_state, cfg, request).await
}

#[post("/web/recovery/delete_dec_share", format = "json", data = "<request>")]
#[instrument(name = "POST /web/recovery/delete_dec_share", skip_all, ret)]
pub async fn recovery_delete_dec_key_share(
    restore_state: &State<Arc<TssState>>,
    cfg: &State<ReloadableLitConfig>,
    request: Json<models::DownloadShareRequest>,
) -> status::Custom<Value> {
    recovery::endpoints::recovery_delete_dec_key_share(restore_state, cfg, request).await
}

/*
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"clientPublicKey": "test"}' \
  http://localhost:7470/web/handshake
*/
#[post("/web/handshake", format = "json", data = "<json_handshake_request>")]
#[instrument(name = "POST /web/handshake", skip_all)]
#[allow(clippy::too_many_arguments)]
pub async fn handshake(
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    json_handshake_request: Json<models::JsonSDKHandshakeRequest>,
    tracing_required: TracingRequired,
    version: SdkVersion,
    cfg: &State<ReloadableLitConfig>,
    eth_blockhash_cache: &State<Arc<EthBlockhashCache>>,
) -> status::Custom<Value> {
    with_timeout(async move {
        web_client::handshake(
            session,
            remote_addr,
            json_handshake_request,
            tracing_required,
            version,
            cfg,
            eth_blockhash_cache,
        )
        .await
    })
    .await
}

#[allow(clippy::too_many_arguments)]
#[post(
    "/web/encryption/sign",
    format = "json",
    data = "<encryption_sign_request>"
)]
#[instrument(name = "POST /web/encryption/sign", skip_all, ret)]
pub(crate) async fn encryption_sign(
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    encryption_sign_request: Json<models::EncryptionSignRequest>,
    tracing: Tracing,
) -> status::Custom<Value> {
    with_context_and_timeout(
        tracing.clone(),
        web_client::encryption_sign(
            session,
            remote_addr,
            rate_limit_db,
            ipfs_cache,
            cfg,
            encryption_sign_request,
            tracing,
            EndpointVersion::Initial,
        ),
    )
    .await
}

#[allow(clippy::too_many_arguments)]
#[post(
    "/web/signing/access_control_condition",
    format = "json",
    data = "<signing_access_control_condition_request>"
)]
#[instrument(name = "POST /web/signing/access_control_condition", skip_all, ret)]
pub(crate) async fn signing_access_control_condition(
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    signing_access_control_condition_request: Json<models::SigningAccessControlConditionRequest>,
    tracing: Tracing,
) -> status::Custom<Value> {
    with_context_and_timeout(
        tracing.clone(),
        web_client::signing_access_control_condition(
            session,
            remote_addr,
            rate_limit_db,
            ipfs_cache,
            cfg,
            signing_access_control_condition_request,
            tracing,
            EndpointVersion::Initial,
        ),
    )
    .await
}

#[post(
    "/web/sign_session_key",
    format = "json",
    data = "<json_sign_session_key_request>"
)]
#[instrument(name = "POST /web/sign_session_key", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn sign_session_key(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<crate::tss::common::tss_state::TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    json_sign_session_key_request: Json<models::JsonSignSessionKeyRequest>,
    tracing: Tracing,
    request_headers: RequestHeaders<'_>,
) -> status::Custom<Value> {
    let json_sign_session_key_request_v1 = models::JsonSignSessionKeyRequestV1 {
        session_key: json_sign_session_key_request.session_key.clone(),
        auth_methods: json_sign_session_key_request.auth_methods.clone(),
        pkp_public_key: json_sign_session_key_request.pkp_public_key.clone(),
        auth_sig: json_sign_session_key_request.auth_sig.clone(),
        siwe_message: json_sign_session_key_request.siwe_message.clone(),
        curve_type: CurveType::K256,
        code: None,
        lit_action_ipfs_id: None,
        js_params: None,
        epoch: json_sign_session_key_request.epoch,
    };
    with_context_and_timeout(
        tracing.clone(),
        web_client::sign_session_key(
            remote_addr,
            tss_state,
            auth_context_cache,
            rate_limit_db,
            ipfs_cache,
            cfg,
            json_sign_session_key_request_v1,
            tracing,
            request_headers,
            EndpointVersion::Initial,
        ),
    )
    .await
}

#[post("/web/pkp/sign", format = "json", data = "<json_pkp_signing_request>")]
#[instrument(name = "POST /web/pkp/sign", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn pkp_sign(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    json_pkp_signing_request: Json<models::JsonPKPSigningRequest>,
    tracing: Tracing,
) -> status::Custom<Value> {
    with_context_and_timeout(
        tracing.clone(),
        pkp::pkp_sign(
            remote_addr,
            tss_state,
            auth_context_cache,
            rate_limit_db,
            cfg,
            allowlist_cache,
            json_pkp_signing_request,
            tracing,
            EndpointVersion::Initial,
        ),
    )
    .await
}

#[post("/web/pkp/claim", format = "json", data = "<json_pkp_claim_request>")]
#[instrument(name = "POST /web/pkp/claim", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn pkp_claim(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    json_pkp_claim_request: Json<models::JsonPKPClaimKeyRequest>,
    tracing: Tracing,
) -> status::Custom<Value> {
    with_context_and_timeout(
        tracing.clone(),
        pkp::pkp_claim(
            remote_addr,
            tss_state,
            auth_context_cache,
            rate_limit_db,
            cfg,
            allowlist_cache,
            json_pkp_claim_request,
            tracing,
        ),
    )
    .await
}

/*
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"js_base64": "Y29uc29sZS5sb2coInJ1bm5pbmchIik7Cgpjb25zdCBnbyA9IGFzeW5jICgpID0+IHsKICBsZXQgZGF0YSA9IGF3YWl0IGZldGNoKAogICAgImh0dHBzOi8vaXBmcy5saXRnYXRld2F5LmNvbS9pcGZzL1FtTmlEckRuVGlTbzR5NzhxS3dhWmJvcThLZlQ5WTNDUnJuTTdwZlVtRzFFRnEiCiAgKS50aGVuKChyZXNwb25zZSkgPT4gcmVzcG9uc2UuanNvbigpKTsKICBjb25zb2xlLmxvZygiZGF0YTogIiArIEpTT04uc3RyaW5naWZ5KGRhdGEsIG51bGwsIDIpKTsKfTsKCmdvKCk7CmNvbnNvbGUubG9nKCJmZXRjaGluZyBraWNrZWQgb2ZmLCBidXQgd2FpdGluZyBmb3IgcHJvbWlzZXMiKTsK"}' \
  http://localhost:7470/web/execute
*/
#[cfg(feature = "lit-actions")]
#[post("/web/execute", format = "json", data = "<json_execution_request>")]
#[instrument(name = "POST /web/execute", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn execute_function(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<crate::tss::common::tss_state::TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    json_execution_request: Json<models::JsonExecutionRequest>,
    tracing: Tracing,
    request_headers: RequestHeaders<'_>,
) -> status::Custom<Value> {
    with_context_and_timeout(
        tracing.clone(),
        web_client::execute_function(
            remote_addr,
            tss_state,
            auth_context_cache,
            rate_limit_db,
            cfg,
            allowlist_cache,
            ipfs_cache,
            json_execution_request,
            tracing,
            request_headers,
            EndpointVersion::Initial,
        ),
    )
    .await
}
