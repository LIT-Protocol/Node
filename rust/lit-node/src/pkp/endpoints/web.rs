#![allow(dead_code)]
use crate::auth::auth_material::AuthSigItem;
use crate::auth::resources::PKPNFTResource;
use crate::constants::CHAIN_ETHEREUM;
use crate::error::unexpected_err;
use crate::models;
use crate::models::auth::LitResourcePrefix;
use crate::pkp::auth::AuthMethodScope;
use crate::pkp::utils::{claim_key, sign_ecdsa};
use crate::rate_limiting::models::UserContext;
use crate::rate_limiting::{check_rate_limit, models::RateLimitDB};
use crate::tss::common::tss_state::TssState;

use crate::utils::rocket::guards::ClientContext;
use crate::utils::web::ConcurrencyGuard;
use crate::utils::web::{get_auth_context, pubkey_to_token_id};
use lit_api_core::context::{with_context, Tracer, Tracing};
use lit_api_core::error::ApiError;
use lit_core::config::ReloadableLitConfig;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{post, State};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[post("/web/pkp/sign", format = "json", data = "<json_pkp_signing_request>")]
#[instrument(name = "POST /web/pkp/sign", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn pkp_sign(
    _guard: ConcurrencyGuard<'_>,
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
    with_context(tracing.clone(), async move {
        debug!(
            "pkp sign, request: {:}",
            format!("{:?}", json_pkp_signing_request)
        );

        let cfg = cfg.load_full();

        let token_id = match pubkey_to_token_id(&json_pkp_signing_request.pubkey) {
            Ok(token_id) => token_id,
            Err(e) => {
                return e.handle();
            }
        };
        let resource = PKPNFTResource::new(token_id);
        let resource_ability = resource.signing_ability();


        // Validate auth sig item
        let validated_address = {
            match json_pkp_signing_request.auth_sig.validate_and_get_user_address(
                &resource_ability,
                &Some(CHAIN_ETHEREUM.to_string()),
                &cfg,
            )
            .await
            {
                Err(e) => {
                    return e.handle();
                }
                Ok(resp) => resp,
            }
        };

        // Get the authorized rate limit NFT IDs.
        let authorized_rate_limit_nft_ids = {
            match json_pkp_signing_request.auth_sig.resources() {
                Err(e) => {
                    return e.handle();
                }
                Ok(resources) => {
                    resources
                        .iter()
                        .filter(|r| r.get_resource_prefix() == LitResourcePrefix::RLI)
                        .map(|r| r.get_resource_id().to_owned())
                        .collect()
                }
            }
        };

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
                ip_address: client_context.ip_address,
                user_address,
                authorized_rate_limit_nft_ids,
            },
            &json_pkp_signing_request.auth_sig,
            // paying for Lit Actions is only supported on EVM right now, so using ethereum as the chain will always work
            Some(CHAIN_ETHEREUM.to_string()),
            rate_limit_db,
            remote_addr,
            &resource_ability,
            &cfg,
        )
        .await;

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

        let auth_context = get_auth_context(
            Some(auth_sig.clone()),
            json_pkp_signing_request.auth_methods.clone(),
            None,
            Some(auth_context_cache),
            false,
            cfg.clone(),
            None
        )
        .await;

        let auth_context = match auth_context {
            Ok(auth_context) => auth_context,
            Err(e) => {
                return e.handle();
            }
        };
        trace!("Got auth context");

        let is_root_key = false; // simple check against the root keys.

        let result = sign_ecdsa(
            cfg.as_ref(),
            &json_pkp_signing_request.to_sign,
            json_pkp_signing_request.pubkey.clone(),
            tracing.clone().correlation_id().to_string(),
            None,
            Some(auth_sig.clone()),
            auth_context,
            Some(tss_state.as_ref().clone()),
            &[AuthMethodScope::SignAnything as usize],
        )
        .await.map_err(|e| unexpected_err(e, Some("Error signing with the PKP".to_string())));

        let result = match result {
            Ok(result) => result,
            Err(e) => {
                return e.handle();
            }
        };

        status::Custom(
            Status::Ok,
            json!({"success": true, "signedData": &json_pkp_signing_request.to_sign, "signatureShare": result.0}),
        )
    }).await
}

#[post("/web/pkp/claim", format = "json", data = "<json_pkp_claim_request>")]
#[instrument(name = "POST /web/pkp/claim", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn pkp_claim(
    _guard: ConcurrencyGuard<'_>,
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    json_pkp_claim_request: Json<models::JsonPKPClaimKeyRequest>,
    tracing: Tracing,
    client_context: ClientContext,
) -> status::Custom<Value> {
    with_context(tracing.clone(), async move {
        let cfg = cfg.load_full();

        let claim_res = claim_key(cfg.as_ref(), &json_pkp_claim_request.0).await;

        match claim_res {
            Ok(resp) => status::Custom(Status::Accepted, json!(resp)),
            Err(e) => unexpected_err(e, Some("Error occured in claim process".into())).handle(),
        }
    })
    .await
}
