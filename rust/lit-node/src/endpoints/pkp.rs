#![allow(dead_code)]
use crate::auth::auth_material::AuthSigItem;
use crate::auth::resources::PKPNFTResource;
use crate::constants::CHAIN_ETHEREUM;
use crate::error::unexpected_err;
use crate::models;
use crate::models::auth::SessionKeySignedMessage;
use crate::pkp::auth::AuthMethodScope;
use crate::pkp::utils::{claim_key, sign_ecdsa};
use crate::rate_limiting::models::UserContext;
use crate::rate_limiting::{check_rate_limit, models::RateLimitDB};
use crate::tss::common::tss_state::TssState;
use crate::utils::web::get_auth_context;
use crate::utils::web::EndpointVersion;
use lit_api_core::context::Tracer;

use crate::utils::web::pubkey_to_token_id;
use crate::utils::web::{get_auth_context_from_session_sigs, get_bls_root_pubkey};
use lit_api_core::context::{with_context, Tracing};
use lit_api_core::error::ApiError;
use lit_core::config::ReloadableLitConfig;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::State;
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

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
    endpoint_version: EndpointVersion,
) -> status::Custom<Value> {
    let request_start = std::time::Instant::now();
    with_context(tracing.clone(), async move {
        debug!(
            "pkp sign, request: {:}",
            format!("{:?}", json_pkp_signing_request)
        );

        let cfg = cfg.load_full();

        let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

        let token_id = match pubkey_to_token_id(&json_pkp_signing_request.pubkey) {
            Ok(token_id) => token_id,
            Err(e) => {
                return e.handle();
            }
        };
        let resource = PKPNFTResource::new(token_id);
        let resource_ability = resource.signing_ability();

        let before = std::time::Instant::now();
        // Validate auth sig item
        let bls_root_pubkey = match get_bls_root_pubkey(tss_state).await {
            Ok(bls_root_pubkey) => bls_root_pubkey,
            Err(e) => {
                return e.handle();
            }
        };

        let validated_address = {
            match json_pkp_signing_request.auth_sig.validate_and_get_user_address(
                &resource_ability,
                &Some(CHAIN_ETHEREUM.to_string()),
                &cfg,
                &bls_root_pubkey,
                &endpoint_version,
            )
            .await
            {
                Err(e) => {
                    return e.handle();
                }
                Ok(resp) => resp,
            }
        };
        timing.insert("auth sig validation".to_string(), before.elapsed());

        let before = std::time::Instant::now();
        // Check the rate limit
        let user_address = {
            if validated_address.is_evm_user_address() {
                match validated_address.evm_address() {
                    Ok(address) => Some(address),
                    Err(e) => {
                        return e.handle();
                    }
                }
            } else {
                None
            }
        };
        let rate_limit_res = check_rate_limit(
            &UserContext {
                user_address,
            },
            &json_pkp_signing_request.auth_sig,
            // paying for Lit Actions is only supported on EVM right now, so using ethereum as the chain will always work
            Some(CHAIN_ETHEREUM.to_string()),
            rate_limit_db,
            remote_addr,
            &resource_ability,
            &cfg,
            &bls_root_pubkey,
        )
        .await;
        timing.insert("rate limit check".to_string(), before.elapsed());

        let rate_limit_check_return = match rate_limit_res {
            Ok(resp) => resp,
            Err(e) => {
                return e.handle();
            }
        };

        if rate_limit_check_return.rate_limit_exceeded {
            let msg = match rate_limit_check_return.try_again_after {
                Some(try_again_after) => format!("Rate limit exceeded.  Try again at {}", try_again_after),
                None => "Rate limit exceeded.  Try again later.".to_string(),
            };
            warn!("{}", msg);
            return status::Custom(
                Status::BadRequest,
                json!({"message": msg, "errorCode": "rate_limit_exceeded"}),
            );
        }

        // check for single or multiple auth sigs and do the session key
        // capability check.  set the wallet that provided the capabilities as the
        // main auth sig wallet.
        let auth_sig = match &json_pkp_signing_request.auth_sig {
            AuthSigItem::Single(single_auth_sig) => single_auth_sig.clone(),
            AuthSigItem::Multiple(_) => {
                return status::Custom(
                    Status::BadRequest,
                    json!({"message": "Multiple auth sigs not supported by Lit Actions", "errorCode": "unsupported_auth_sig"}),
                );
            }
        };

        let before = std::time::Instant::now();

        let auth_context = match endpoint_version {
            EndpointVersion::Initial => {
                let auth_context = get_auth_context(
                    Some(auth_sig.clone()),
                    json_pkp_signing_request.auth_methods.clone(),
                    None,
                    Some(auth_context_cache),
                    false,
                    cfg.clone(),
                    None,
                    &bls_root_pubkey,
                    &endpoint_version,
                )
                .await;

                match auth_context {
                    Ok(auth_context) => auth_context,
                    Err(e) => {
                        return e.handle();
                    }
                }
            },
            EndpointVersion::V1 => {
                let session_key_signed_message: std::result::Result<
                    SessionKeySignedMessage,
                    serde_json::Error,
                > = serde_json::from_str(&auth_sig.signed_message);
                let session_key_signed_message = match session_key_signed_message {
                    Ok(session_key_signed_message) => session_key_signed_message,
                    Err(e) => {
                        error!("Error parsing session sig in pkp_sign");
                        return status::Custom(
                            Status::BadRequest,
                            json!({"message": "Either you've have passed an AuthSig or the sessionSig is incorrectly formatted", "errorCode": "unsupported_auth_sig"}),
                        );
                    }
                };

                timing.insert("parsed session sig".to_string(), before.elapsed());

                let resolved_auth_context = match get_auth_context_from_session_sigs(session_key_signed_message).await {
                    Ok(resolved_auth_context) => resolved_auth_context,
                    Err(e) => {
                        error!("Error parsing AuthContext from sessionSig");
                        return e.handle();
                    }
                };

                debug!("resolved_auth_context- {:?}", resolved_auth_context);

                match resolved_auth_context {
                    Some(resolved_auth_context) => resolved_auth_context,
                    None => {
                        // Also create new auth_context for EOA authSig/sessionSigs
                        let new_auth_context = get_auth_context(
                            Some(auth_sig.clone()),
                            None,
                            None,
                            None,
                            false,
                            cfg.clone(),
                            None,
                            &bls_root_pubkey,
                            &endpoint_version,
                        )
                        .await;

                        match new_auth_context {
                            Ok(new_auth_context) => new_auth_context,
                            Err(e) => {
                                return e.handle();
                            }
                        }
                    }
                }
            }
        };

        timing.insert("auth context".to_string(), before.elapsed());
        trace!("Got auth context");

        let epoch = match json_pkp_signing_request.epoch {
            0 => None,
            i => Some(i),
        };

        let before = std::time::Instant::now();
        let result = sign_ecdsa(
            cfg.as_ref(),
            &json_pkp_signing_request.to_sign,
            json_pkp_signing_request.pubkey.clone(),
            tracing.clone().correlation_id().to_string(),
            None, // Only the first one as we only allow running a single Lit Action now for session creation
            Some(auth_sig.clone()), // This works with EOA wallets as well as we're passing SessionSig/AuthSig here
            auth_context,
            Some(tss_state.as_ref().clone()),
            &[AuthMethodScope::SignAnything as usize],
            epoch,
            &bls_root_pubkey
        )
        .await.map_err(|e| unexpected_err(e, Some("Error signing with the PKP".to_string())));
        timing.insert("sign ecdsa".to_string(), before.elapsed());

        let result = match result {
            Ok(result) => result,
            Err(e) => {
                return e.handle();
            }
        };

        timing.insert("total".to_string(), request_start.elapsed());

        debug!("POST /web/pkp/sign timing: {:?}", timing);

        status::Custom(
            Status::Ok,
            json!({"success": true, "signedData": &json_pkp_signing_request.to_sign, "signatureShare": result.0}),
        )
    }).await
}

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
    let request_start = std::time::Instant::now();

    with_context(tracing.clone(), async move {
        debug!(
            "pkp claim, request: {:}",
            format!("{:?}", json_pkp_claim_request)
        );
        let cfg = cfg.load_full();

        let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

        let before = std::time::Instant::now();
        let claim_res = claim_key(cfg.as_ref(), &json_pkp_claim_request.0).await;
        timing.insert("claim key".to_string(), before.elapsed());

        timing.insert("total".to_string(), request_start.elapsed());

        debug!("POST /web/pkp/claim timing: {:?}", timing);

        match claim_res {
            Ok(resp) => status::Custom(Status::Accepted, json!(resp)),
            Err(e) => unexpected_err(e, Some("Error occured in claim process".into())).handle(),
        }
    })
    .await
}
