use crate::auth::auth_material::AuthMaterialType;
use crate::endpoints::pkp;
use crate::endpoints::web_client;
use crate::error::{validation_err_code, EC};
use crate::models;
use crate::rate_limiting::models::RateLimitDB;
use crate::tss::common::tss_state::TssState;
use crate::utils::rocket::guards::ClientContext;
use crate::utils::web::EndpointVersion;

use crate::tss::dkg::curves::common::CurveType;
use crate::utils::rocket::guards::RequestHeaders;
use crate::utils::web::ConcurrencyGuard;
use lit_api_core::context::Tracing;
use lit_api_core::error::ApiError;
use lit_core::config::ReloadableLitConfig;
use moka::future::Cache;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use rocket::{Route, State};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[allow(dead_code)]
pub(crate) fn routes() -> Vec<Route> {
    routes![
        encryption_sign,
        signing_access_control_condition,
        sign_session_key,
        pkp_sign,
        execute_function
    ]
}

#[allow(clippy::too_many_arguments)]
#[post(
    "/web/encryption/sign/v1",
    format = "json",
    data = "<encryption_sign_request>"
)]
#[instrument(name = "POST /web/encryption/sign/v1", skip_all, ret)]
pub(crate) async fn encryption_sign(
    guard: ConcurrencyGuard<'_>,
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    encryption_sign_request: Json<models::EncryptionSignRequest>,
    tracing: Tracing,
    client_context: ClientContext,
) -> status::Custom<Value> {
    web_client::encryption_sign(
        session,
        remote_addr,
        rate_limit_db,
        ipfs_cache,
        cfg,
        encryption_sign_request,
        tracing,
        EndpointVersion::V1,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
#[post(
    "/web/signing/access_control_condition/v1",
    format = "json",
    data = "<signing_access_control_condition_request>"
)]
#[instrument(name = "POST /web/signing/access_control_condition/v1", skip_all, ret)]
pub(crate) async fn signing_access_control_condition(
    guard: ConcurrencyGuard<'_>,
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    signing_access_control_condition_request: Json<models::SigningAccessControlConditionRequest>,
    tracing: Tracing,
    client_context: ClientContext,
) -> status::Custom<Value> {
    web_client::signing_access_control_condition(
        guard,
        session,
        remote_addr,
        rate_limit_db,
        ipfs_cache,
        cfg,
        signing_access_control_condition_request,
        tracing,
        client_context,
        EndpointVersion::V1,
    )
    .await
}

#[post(
    "/web/sign_session_key/v1",
    format = "json",
    data = "<json_sign_session_key_request>"
)]
#[instrument(name = "POST /web/sign_session_key/v1", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn sign_session_key(
    guard: ConcurrencyGuard<'_>,
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    json_sign_session_key_request: Json<models::JsonSignSessionKeyRequestV1>,
    tracing: Tracing,
    request_headers: RequestHeaders<'_>,
) -> status::Custom<Value> {
    if json_sign_session_key_request.curve_type != CurveType::BLS {
        return validation_err_code(
            format!(
                "Invalid curve_type: {} - only BLS curve type is allowed for this endpoint",
                &json_sign_session_key_request.curve_type
            ),
            EC::NodeInvalidCurveType,
            None,
        )
        .handle();
    }

    if json_sign_session_key_request.auth_sig.is_some() {
        return validation_err_code(
           "You can't provide an AuthSig on this endpoint.  Instead, send it as an AuthMethod in the authMethods array.",
            EC::NodeInvalidAuthSigForSessionKey,
            None,
        )
        .handle();
    }

    web_client::sign_session_key(
        guard,
        remote_addr,
        tss_state,
        auth_context_cache,
        rate_limit_db,
        ipfs_cache,
        cfg,
        json_sign_session_key_request.into_inner(),
        tracing,
        request_headers,
        EndpointVersion::V1,
    )
    .await
}

#[post(
    "/web/pkp/sign/v1",
    format = "json",
    data = "<json_pkp_signing_request>"
)]
#[instrument(name = "POST /web/pkp/sign/v1", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn pkp_sign(
    guard: ConcurrencyGuard<'_>,
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    json_pkp_signing_request: Json<models::JsonPKPSigningRequest>,
    tracing: Tracing,
    client_context: ClientContext,
) -> status::Custom<Value> {
    if json_pkp_signing_request.auth_methods.is_some() {
        return validation_err_code(
            "Can't provide AuthMethods for pkpSign. You have to provide an SessionSig.",
            EC::NodeCannotProvideAuthMethodForEndpoint,
            None,
        )
        .handle();
    }

    match json_pkp_signing_request.auth_sig.get_auth_type() {
        Ok(auth_material_type) => {
            if auth_material_type != &AuthMaterialType::SessionSig {
                return validation_err_code(
                    "Can't provide AuthSig for pkpSign. You have to provide a SessionSig.",
                    EC::NodeCannotProvideAuthSigForEndpoint,
                    None,
                )
                .handle();
            }
        }
        Err(e) => return e.handle(),
    };

    pkp::pkp_sign(
        guard,
        remote_addr,
        tss_state,
        auth_context_cache,
        rate_limit_db,
        cfg,
        allowlist_cache,
        json_pkp_signing_request,
        tracing,
        client_context,
        EndpointVersion::V1,
    )
    .await
}

#[cfg(feature = "lit-actions")]
#[post("/web/execute/v1", format = "json", data = "<json_execution_request>")]
#[instrument(name = "POST /web/execute/v1", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn execute_function(
    guard: ConcurrencyGuard<'_>,
    remote_addr: SocketAddr,
    tss_state: &State<Arc<crate::tss::common::tss_state::TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    json_execution_request: Json<models::JsonExecutionRequest>,
    tracing: Tracing,
    client_context: ClientContext,
    request_headers: RequestHeaders<'_>,
) -> status::Custom<Value> {
    if json_execution_request.auth_methods.is_some() {
        return validation_err_code(
            "Can't provide AuthMethods for executeJs. You have to provide an SessionSig.",
            EC::NodeCannotProvideAuthMethodForEndpoint,
            None,
        )
        .handle();
    }

    match json_execution_request.auth_sig.get_auth_type() {
        Ok(auth_material_type) => {
            if auth_material_type != &AuthMaterialType::SessionSig {
                return validation_err_code(
                    "Can't provide AuthSig for execute. You have to provide a SessionSig.",
                    EC::NodeCannotProvideAuthSigForEndpoint,
                    None,
                )
                .handle();
            }
        }
        Err(e) => return e.handle(),
    };

    web_client::execute_function(
        guard,
        remote_addr,
        tss_state,
        auth_context_cache,
        rate_limit_db,
        cfg,
        allowlist_cache,
        ipfs_cache,
        json_execution_request,
        tracing,
        client_context,
        request_headers,
        EndpointVersion::V1,
    )
    .await
}
