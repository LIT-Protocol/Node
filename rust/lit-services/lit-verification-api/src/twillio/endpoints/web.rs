use std::{panic::Location, sync::Arc};

use lit_api_core::{
    context::{with_context, Tracing},
    error::{ApiError, Kind},
};
use lit_core::{config::ReloadableLitConfig, error::Error};
use log::trace;
use moka::future::Cache;
use rocket::{get, http::Status, post, response::status, serde::json::Json, State as RocketState};
use rocket_governor::RocketGovernor;
use serde_json::{self, json, Value};

use crate::{
    api_key::ApiKey,
    client_ip::ClientRealAddr,
    ecdsa::SigningPair,
    errors::{validation_err, EC, PKG_NAME},
    limit::RateLimitGuard,
    models::CacheEntry,
    twillio::{
        endpoints::build_jwt,
        models::{
            IdCheckResponse, OtpCheckRequest, OtpCheckResponse, OtpStartRequest, OtpStartResponse,
            RequestOptions, TwillioCheckVerificationResponse, TwillioStartVerificationResponse,
        },
    },
};

use super::{
    build_status_url, check_otp, start_otp_request,
    utils::{check_cache_for_client_ip, get_ip_from_request},
};

#[post("/api/otp/start", format = "json", data = "<input>")]
pub(crate) async fn start_verification(
    _limitguard: RocketGovernor<'_, RateLimitGuard>, cfg: &RocketState<ReloadableLitConfig>,
    request_id_context_cache: &RocketState<Cache<String, CacheEntry>>, tracing: Tracing,
    input: Json<OtpStartRequest>, client_ip: ClientRealAddr, _api_key: ApiKey,
) -> status::Custom<Value> {
    let id = &input.request_id;
    let user_id = &input.otp;
    let email_configuration = input.email_configuration.to_owned();
    let custom_name = input.custom_name.clone();
    let cfg = cfg.load_full();
    let client_addr = get_ip_from_request(&client_ip);

    with_context(tracing, async move {
        if let Some(id) = check_cache_for_client_ip(request_id_context_cache, client_addr.clone()) {
            return Error::new(
                Some(Kind::Lock),
                PKG_NAME.to_string(),
                None,
                Some(Arc::new(EC::VerificationUnknownRequestId)),
                Some(format!(
                    "Request already in progress for client address request id in progress: {}",
                    id
                )),
                Some(Location::caller()),
            )
            .handle();
        }

        let request = request_id_context_cache.get(id);
        match request {
            Some(_) => {
                let err = Error::new(
                    Some(Kind::Lock),
                    PKG_NAME.to_string(),
                    None,
                    Some(Arc::new(EC::VerificationUnknownRequestId)),
                    Some("request already in progress"),
                    Some(Location::caller()),
                );
                return err.handle();
            }
            None => {
                trace!("request {} not found creating entry...", id);
            }
        };

        let request_options = RequestOptions {
            user_id: user_id.clone(),
            code: None,
            email_configuration,
            custom_name,
        };
        match start_otp_request::<TwillioStartVerificationResponse>(&request_options, cfg.as_ref())
            .await
        {
            Ok(resp) => {
                let callback = build_status_url(cfg.as_ref(), id);
                match callback {
                    Ok(cb) => {
                        request_id_context_cache
                            .insert(
                                id.to_string(),
                                CacheEntry {
                                    service_callback: resp.url,
                                    client_ip: client_addr,
                                    code_attempts: 0,
                                },
                            )
                            .await;
                        status::Custom(
                            Status::Ok,
                            json!(OtpStartResponse {
                                message: Some("verification started".to_owned()),
                                callback: Some(cb),
                            }),
                        )
                    }
                    Err(e) => e.handle(),
                }
            }
            Err(e) => e.handle(),
        }
    })
    .await
}

#[allow(clippy::too_many_arguments)]
#[post("/api/otp/check", format = "json", data = "<input>")]
pub(crate) async fn check_verification(
    _limitguard: RocketGovernor<'_, RateLimitGuard>, key_shares: &RocketState<SigningPair>,
    cfg: &RocketState<ReloadableLitConfig>,
    request_id_context_cache: &RocketState<Cache<String, CacheEntry>>, tracing: Tracing,
    input: Json<OtpCheckRequest>, client_ip: ClientRealAddr, _api_key: ApiKey,
) -> status::Custom<Value> {
    let id = &input.request_id;
    let user_id = &input.otp;
    let code = &input.code;
    let cfg = cfg.load_full();
    let client_addr = get_ip_from_request(&client_ip);

    with_context(tracing, async move {
        let request = request_id_context_cache.get(id);
        match request {
            Some(req) => {
                trace!("Request id found: client ip: {}", req.client_ip);
                // Check if the cache entry matches the ip from this request. if not abort the request
                // but keep the cache entry which will expire if not accessed in the timeout period.
                if req.client_ip != client_addr {
                    trace!("client request started with address: {}, found new client address: {}", req.client_ip, client_addr);
                    return Error::new(
                        Some(Kind::Validation),
                        PKG_NAME.to_string(),
                        None,
                        Some(Arc::new(EC::VerificationClientAddress)),
                        Some("client address does not match session address, aborting"),
                        Some(Location::caller())
                    ).handle();
                }
            }
            None => {
                let err = Error::new(
                    Some(Kind::Lock),
                    PKG_NAME.to_string(),
                    None,
                    Some(Arc::new(EC::VerificationUnknownRequestId)),
                    Some("Request already in progress"),
                    Some(Location::caller()),
                );

                return err.handle();
            }
        }
        let request_options = RequestOptions {
            user_id: user_id.to_string(),
            code: Some(code.to_string()),
            email_configuration: None,
            custom_name: None,
        };

        let resp = match start_otp_request::<TwillioCheckVerificationResponse>(
            &request_options,
            cfg.as_ref(),
        )
        .await
        {
            Ok(resp) => {
                if resp.status == "approved" {
                    trace!("Operation resolved, releasing requestId {}", id);
                    request_id_context_cache.invalidate(id).await;
                } else {
                    trace!("Request status not yet approved current value is {} not releasing session cache", resp.status);
                    let entry = request_id_context_cache.get(id);
                    if let Some(mut e) = entry {
                        trace!("invalid code entry for request id: {} ip", id.to_string(), );
                        e.code_attempts += 1;
                        request_id_context_cache.insert(id.to_string(), e).await;
                    }

                    return validation_err("Invalid code provided", Some("Invalid OTP".into())).handle();
                }

                resp
            },
            Err(e) => return e.handle(),
        };

        let token_str = match build_jwt(user_id, key_shares) {
            Ok(token_str) => token_str,
            Err(e) => {
                return e.handle();
            }
        };

        status::Custom(
            Status::Accepted,
            json!(OtpCheckResponse {
                message: Some("verification checked".to_owned()),
                status: Some(resp.status),
                token_jwt: Some(token_str),
            }),
        )
    })
    .await
}

#[get("/api/otp/session/check/<request_id>", format = "json")]
pub(crate) async fn check_request_id(
    tracing: Tracing, _limitguard: RocketGovernor<'_, RateLimitGuard>,
    request_id_context_cache: &RocketState<Cache<String, CacheEntry>>,
    cfg: &RocketState<ReloadableLitConfig>, request_id: &str, _api_key: ApiKey,
) -> status::Custom<Value> {
    with_context(tracing.clone(), async move {
        let cfg = cfg.load_full();
        let request = request_id_context_cache.get(request_id);
        match request {
            Some(entry) => {
                let val = check_otp(cfg.as_ref(), &entry.service_callback).await;
                match val {
                    Ok(resp) => status::Custom(Status::Ok, json!(resp)),
                    Err(e) => e.handle(),
                }
            }

            None => status::Custom(Status::Ok, json!(IdCheckResponse { is_used: false })),
        }
    })
    .await
}
