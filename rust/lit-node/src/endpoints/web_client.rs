use ethers::types::Bytes;
use lit_api_core::context::Tracer;
use lit_api_core::context::{with_context, SdkVersion, Tracing, TracingRequired};
use lit_api_core::error::ApiError;
use lit_blockchain::resolver::rpc::get_provider;
use lit_core::config::{LitConfig, ReloadableLitConfig};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::State;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;
use tracing::instrument;
use url::Url;

use crate::auth::auth_material::AuthSigItem;
use crate::auth::lit_resource::LitResource;
use crate::auth::resources::{AccessControlConditionResource, LitResourceAbility};
use crate::config::LitNodeConfig;
use crate::constants::CHAIN_ETHEREUM;
use crate::error::{conversion_err, unexpected_err, validation_err_code, EC};
use crate::error::{parser_err, parser_err_code};
use crate::models::auth::LitResourcePrefix;
use crate::models::{
    self, AccessControlConditionItem, EVMContractConditionItem, JwtPayloadV2, RequestConditions,
    SigningAccessControlConditionResponse, SolRpcConditionItem, UnifiedAccessControlConditionItem,
};
use crate::pkp;
use crate::pkp::auth::serialize_auth_context_for_checking_against_contract_data;
use crate::rate_limiting::models::UserContext;
use crate::rate_limiting::{check_rate_limit, models::RateLimitDB};
use crate::tss::common::key_type::KeyType;
use crate::tss::common::tss_state::TssState;
use crate::utils::attestation::create_attestation;
use crate::utils::encoding;
use crate::utils::rocket::guards::{ClientContext, RequestHeaders};
use crate::utils::web::get_auth_context;
use crate::utils::web::ConcurrencyGuard;
use crate::utils::web::{check_condition_count, hash_access_control_conditions};
use crate::{access_control, error};
use crate::{
    jwt,
    siwe_db::{db, rpc::EthBlockhashCache},
};

// Not dead code, rather a lint bug
// see https://github.com/rust-lang/rust/issues/92554
#[allow(dead_code)]
const MAX_JWT_EXPIRATION: u64 = 12 * 60 * 60; // 12 hours as seconds

#[allow(clippy::too_many_arguments)]
#[post(
    "/web/signing/access_control_condition",
    format = "json",
    data = "<signing_access_control_condition_request>"
)]
#[instrument(name = "POST /web/signing/access_control_condition", skip_all, ret)]
pub(crate) async fn signing_access_control_condition(
    _guard: ConcurrencyGuard<'_>,
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    cfg: &State<ReloadableLitConfig>,
    signing_access_control_condition_request: Json<models::SigningAccessControlConditionRequest>,
    tracing: Tracing,
    client_context: ClientContext,
) -> status::Custom<Value> {
    with_context(tracing.clone(), async move {
        debug!(
            "signing_access_control_condition_request, request: {:}",
            format!("{:?}", signing_access_control_condition_request)
        );

        let cfg = cfg.load_full();

        let cc_check = check_condition_count(
            &signing_access_control_condition_request.access_control_conditions,
            &signing_access_control_condition_request.evm_contract_conditions,
            &signing_access_control_condition_request.sol_rpc_conditions,
            &signing_access_control_condition_request.unified_access_control_conditions,
        );
        if let Err(e) = cc_check {
            return e.handle();
        }

        // Hash the access control condition
        let hash_res = hash_access_control_conditions(
            RequestConditions {
                access_control_conditions: signing_access_control_condition_request.access_control_conditions.clone(),
                evm_contract_conditions: signing_access_control_condition_request.evm_contract_conditions.clone(),
                sol_rpc_conditions: signing_access_control_condition_request.sol_rpc_conditions.clone(),
                unified_access_control_conditions: signing_access_control_condition_request.unified_access_control_conditions.clone(),
            },
        );
        let hashed_access_control_conditions = match hash_res {
            Ok(hashed_access_control_conditions) => hashed_access_control_conditions,
            Err(e) => {
                return e.handle();
            }
        };

        let lit_acc_resource = AccessControlConditionResource::new(hashed_access_control_conditions);

        // Validate auth sig item
        let validated_address = {
            match signing_access_control_condition_request.auth_sig.validate_and_get_user_address(
                &lit_acc_resource.signing_ability(),
                &signing_access_control_condition_request.chain.clone(),
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
            match signing_access_control_condition_request.auth_sig.resources() {
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
            &signing_access_control_condition_request.auth_sig,
            signing_access_control_condition_request.chain.clone(),
            rate_limit_db,
            remote_addr,
            &lit_acc_resource.signing_ability(),
            &cfg,
        )
        .await;

        let rate_limit_check_return = match rate_limit_res {
            Ok(rate_limit_check_return) => rate_limit_check_return,
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
                Status::TooManyRequests,
                json!({"message": msg, "errorCode": "rate_limit_exceeded"}),
            );
        }

        // Check whether user satisfies access control conditions
        let check_result = check_multiple_access_control_conditions(
            &signing_access_control_condition_request.auth_sig,
            &signing_access_control_condition_request.access_control_conditions,
            &signing_access_control_condition_request.evm_contract_conditions,
            &signing_access_control_condition_request.sol_rpc_conditions,
            &signing_access_control_condition_request.unified_access_control_conditions,
            cfg,
            &lit_acc_resource.signing_ability(),
            &signing_access_control_condition_request.chain,
            tracing.clone().correlation_id().to_string()
        ).await;

        let result = match check_result {
            Ok(result) => result,
            Err(e) => {
                return e.handle();
            }
        };
        if !result.result {
            return validation_err_code("The access control condition check returned that you are not permitted to access this content.  Are you sure you meet the conditions?  Check the auth_sig and the other conditions", EC::NodeAccessControlConditionsReturnedNotAuthorized, None)
                .handle();
        }

        // Take iat and exp from the client and validate that it's within a grace period. Because
        // we can only insert deterministic values on the node side because nodes may
        // have different clock times.
        let grace_period_seconds = 3 * 60; // 3 mins in seconds
        let now: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(v) => v.as_secs(),
            Err(e) => {
                return unexpected_err(e, Some("System time is before unix epoch. Your computer clock is probably wrong".into()))
                    .handle();
            }
        };

        if signing_access_control_condition_request.iat < now - grace_period_seconds
            || signing_access_control_condition_request.iat > now + grace_period_seconds
        {
            return validation_err_code(
                "iat on JWT is outside the grace period",
                EC::NodeIatOutsideGracePeriod,
                None,
            )
                .add_detail(
                    "Please check your system clock and make sure it is accurate for your timezone.",
                )
                .handle();
        }

        if signing_access_control_condition_request.exp < now - grace_period_seconds
            || signing_access_control_condition_request.exp > now + grace_period_seconds + MAX_JWT_EXPIRATION
        {
            return validation_err_code(
                format!(
                    "exp is larger than the max expiration time of {} seconds",
                    MAX_JWT_EXPIRATION
                ),
                EC::NodeExpWrongOrTooLarge,
                None,
            )
                .handle();
        }

        // Assemble JWT to be signed.
        let payload = JwtPayloadV2 {
            iss: "LIT".to_string(),
            sub: result.successful_auth_sig.address,
            chain: signing_access_control_condition_request.chain.clone(),
            iat: signing_access_control_condition_request.iat,
            exp: signing_access_control_condition_request.exp,
            access_control_conditions: signing_access_control_condition_request.access_control_conditions.clone(),
            evm_contract_conditions: signing_access_control_condition_request.evm_contract_conditions.clone(),
            sol_rpc_conditions: signing_access_control_condition_request.sol_rpc_conditions.clone(),
            unified_access_control_conditions: signing_access_control_condition_request.unified_access_control_conditions.clone(),
        };

        let message_to_sign = match jwt::generate_unsigned_jwt_v2(&payload) {
            Ok(message_to_sign) => message_to_sign,
            Err(e) => {
                return e.handle();
            }
        };

        // Load the BLS secret key share as a blsful key for signing.
        let cipher_state = match session.get_cipher_state(KeyType::BLS) {
            Ok(cipher_state) => cipher_state,
            Err(e) => {
                return e.handle();
            }
        };

        // Sign the JWT using the blsful secret key share.
        let (signature_share, share_index) = match cipher_state.sign(message_to_sign.clone().into_bytes()).await {
            Ok(signature_share) => signature_share,
            Err(e) => {
                return e.handle();
            }
        };

        status::Custom(
            Status::Ok,
            json!(SigningAccessControlConditionResponse {
                result: "success".to_string(),
                signature_share,
                share_index,
                unsigned_jwt: message_to_sign,
            }),
        )
    }).await
}

#[allow(clippy::too_many_arguments)]
#[post(
    "/web/encryption/sign",
    format = "json",
    data = "<encryption_sign_request>"
)]
#[instrument(name = "POST /web/encryption/sign", skip_all, ret)]
pub(crate) async fn encryption_sign(
    _guard: ConcurrencyGuard<'_>,
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    cfg: &State<ReloadableLitConfig>,
    encryption_sign_request: Json<models::EncryptionSignRequest>,
    tracing: Tracing,
    client_context: ClientContext,
) -> status::Custom<Value> {
    with_context(tracing.clone(), async move {
        debug!(
            "encryption_sign, request: {:}",
            format!("{:?}", encryption_sign_request)
        );

        let cfg = cfg.load_full();

        let cc_check = check_condition_count(
            &encryption_sign_request.access_control_conditions,
            &encryption_sign_request.evm_contract_conditions,
            &encryption_sign_request.sol_rpc_conditions,
            &encryption_sign_request.unified_access_control_conditions,
        );
        if let Err(e) = cc_check {
            return e.handle();
        }

        // Hash the access control condition
        let hash_res = hash_access_control_conditions(
            RequestConditions {
                access_control_conditions: encryption_sign_request.access_control_conditions.clone(),
                evm_contract_conditions: encryption_sign_request.evm_contract_conditions.clone(),
                sol_rpc_conditions: encryption_sign_request.sol_rpc_conditions.clone(),
                unified_access_control_conditions: encryption_sign_request.unified_access_control_conditions.clone(),
            },
        );
        let hashed_access_control_conditions = match hash_res {
            Ok(hashed_access_control_conditions) => hashed_access_control_conditions,
            Err(e) => {
                return e.handle();
            }
        };

        let lit_acc_resource = AccessControlConditionResource::new(format!("{}/{}", hashed_access_control_conditions, encryption_sign_request.data_to_encrypt_hash));
        debug!("lit_acc_resource: {:?}", lit_acc_resource);

        // Validate auth sig item
        let validated_address = {
            match encryption_sign_request.auth_sig.validate_and_get_user_address(
                &lit_acc_resource.decrypt_ability(),
                &encryption_sign_request.chain.clone(),
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
        let authorized_rate_limit_nft_ids = vec![];
        info!("Validated user address: {:?}", validated_address);

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
            &encryption_sign_request.auth_sig,
            encryption_sign_request.chain.clone(),
            rate_limit_db,
            remote_addr,
            &lit_acc_resource.decrypt_ability(),
            &cfg,
        )
        .await;

        let rate_limit_check_return = match rate_limit_res {
            Ok(rate_limit_check_return) => rate_limit_check_return,
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

        // Check whether user satisfies access control conditions
        let check_result = check_multiple_access_control_conditions(
            &encryption_sign_request.auth_sig,
            &encryption_sign_request.access_control_conditions,
            &encryption_sign_request.evm_contract_conditions,
            &encryption_sign_request.sol_rpc_conditions,
            &encryption_sign_request.unified_access_control_conditions,
            cfg,
            &lit_acc_resource.decrypt_ability(),
            &encryption_sign_request.chain,
            tracing.clone().correlation_id().to_string()
        ).await;
        let result = match check_result {
            Ok(result) => result,
            Err(e) => {
                return e.handle();
            }
        };
        if !result.result {
            return validation_err_code("The access control condition check returned that you are not permitted to access this content.  Are you sure you meet the conditions?  Check the auth_sig and the other conditions", EC::NodeAccessControlConditionsReturnedNotAuthorized, None)
                .handle();
        }

        // Get the identity parameter to be signed.
        let identity_parameter = lit_acc_resource.get_resource_key().into_bytes();
        debug!("identity_parameter: {:?}", identity_parameter);


        // Load the BLS secret key share as a blsful key for signing.
        let cipher_state = match session.get_cipher_state(KeyType::BLS) {
            Ok(cipher_state) => cipher_state,
            Err(e) => {
                return e.handle();
            }
        };

        // Sign the identity parameter using the blsful secret key share.
        let (signature_share, share_index) = match cipher_state.sign(identity_parameter).await {
            Ok(signature_share) => signature_share,
            Err(e) => {
                return e.handle();
            }
        };

        status::Custom(
            Status::Ok,
            json!(models::EncryptionSignResponse {
                result: "success".to_string(),
                signature_share,
                share_index,
            }),
        )
    }).await
}

/*
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"clientPublicKey": "test"}' \
  http://localhost:7470/web/handshake
*/
#[post("/web/handshake", format = "json", data = "<json_handshake_request>")]
#[instrument(name = "POST /web/handshake", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub async fn handshake(
    _guard: ConcurrencyGuard<'_>,
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    json_handshake_request: Json<models::JsonSDKHandshakeRequest>,
    tracing_required: TracingRequired,
    version: SdkVersion,
    cfg: &State<ReloadableLitConfig>,
    eth_blockhash_cache: &State<Arc<EthBlockhashCache>>,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    let ecdsa_root_keys = match session.get_signing_state(KeyType::EcdsaCaitSith) {
        Ok(signing_state) => signing_state.root_keys().await,
        Err(_) => {
            warn!("Failed to aquire lock on hd_root_keys for ECDSA.");
            vec![]
        }
    };

    let bls_root_keys = match session.get_cipher_state(KeyType::BLS) {
        Ok(cipher_state) => cipher_state.root_keys().await,
        Err(_) => {
            warn!("Failed to aquire lock on hd_root_keys for BLS.");
            vec![]
        }
    };

    // FIXME: remove this "unwrap_or" once we've shipped sending the challenge in the SDK
    let challenge = json_handshake_request
        .challenge
        .clone()
        .unwrap_or("0x1234".to_string());

    // run the attestation
    let attestation = match create_attestation(cfg, challenge.as_str())
        .await
        .map_err(|e| unexpected_err(e, Some("error producing attestation".into())))
    {
        Ok(attestation) => Some(attestation),
        Err(e) => {
            #[cfg(not(feature = "testing"))]
            warn!("Error creating attestation: {:?}", e);
            None
        }
    };

    let latest_blockhash = eth_blockhash_cache.blockhash.read().await.clone();

    // the public key set is currently the bls root key... of which there is only one.
    if !bls_root_keys.is_empty() {
        let network_public_key = &bls_root_keys[0];

        return status::Custom(
            Status::Ok,
            json!(models::JsonSDKHandshakeResponse {
                server_public_key: "".to_string(),
                subnet_public_key: network_public_key.clone(),
                network_public_key: network_public_key.clone(),
                network_public_key_set: network_public_key.clone(),
                client_sdk_version: version.to_string(),
                hd_root_pubkeys: ecdsa_root_keys,
                attestation,
                latest_blockhash,
            }),
        );
    }

    status::Custom(
        Status::Ok,
        json!(models::JsonSDKHandshakeResponse {
            server_public_key: "ERR".to_string(),
            subnet_public_key: "ERR".to_string(),
            network_public_key: "ERR".to_string(),
            network_public_key_set: "ERR".to_string(),
            client_sdk_version: version.to_string(),
            hd_root_pubkeys: ecdsa_root_keys,
            attestation,
            latest_blockhash,
        }),
    )
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
    _guard: ConcurrencyGuard<'_>,
    remote_addr: SocketAddr,
    tss_state: &State<Arc<crate::tss::common::tss_state::TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    json_execution_request: Json<models::JsonExecutionRequest>,
    tracing: Tracing,
    client_context: ClientContext,
) -> status::Custom<Value> {
    use crate::auth::resources::LitActionResource;
    use crate::config::LitNodeConfig;
    use crate::constants::CHAIN_ETHEREUM;
    use crate::error::memory_limit_err_code;
    use crate::error::{timeout_err_code, unexpected_err_code};
    use crate::functions;
    use crate::utils::web::check_allowlist;
    use crate::utils::web::get_ipfs_file;
    use ethers::utils::keccak256;
    use ipfs_hasher::IpfsHasher;
    use std::error::Error;
    use tokio::sync::oneshot;

    with_context(tracing.clone(), async move {
        debug!(
            "execute, request: {:}",
            format!("{:?}", json_execution_request)
        );

        let cfg = cfg.load_full();

        let capability_protocol_prefix = &"litAction".to_string();
        let lit_action_resource = LitActionResource::new("".to_string());

        // Validate auth sig item
        let validated_address = {
            match json_execution_request.auth_sig.validate_and_get_user_address(
                &lit_action_resource.execution_ability(),
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
            match json_execution_request.auth_sig.resources() {
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
            &json_execution_request.auth_sig,
            // paying for Lit Actions is only supported on EVM right now, so using ethereum as the chain will always work
            Some(CHAIN_ETHEREUM.to_string()),
            rate_limit_db,
            remote_addr,
            &lit_action_resource.execution_ability(),
            &cfg,
        )
        .await;

        let rate_limit_check_return = match rate_limit_res {
            Ok(rate_limit_check_return) => rate_limit_check_return,
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

        // determine if the user passed code or an ipfs hash
        let derived_ipfs_id;
        let code_to_run: String;
        if let Some(code) = &json_execution_request.code {
            let decoded_bytes = match data_encoding::BASE64.decode(code.as_bytes()) {
                Ok(decoded_bytes) => decoded_bytes,
                Err(err) => {
                    return conversion_err(err, Some("Error decoding base64".into()))
                        .add_msg_to_details()
                        .handle();
                }
            };

            match String::from_utf8(decoded_bytes) {
                Ok(string_result) => code_to_run = string_result,
                Err(err) => {
                    return conversion_err(err, Some("Error converting bytes to string".into()))
                        .add_msg_to_details()
                        .handle();
                }
            };

            // hash the code to get the ipfs id
            let ipfs_hasher = IpfsHasher::default();
            let cid = ipfs_hasher.compute(code_to_run.as_bytes());
            derived_ipfs_id = cid;
        } else if let Some(ipfs_id) = &json_execution_request.ipfs_id {
            // pull down the code from ipfs
            match get_ipfs_file(&ipfs_id.to_string(), &cfg).await {
                Ok(ipfs_result) => code_to_run = ipfs_result,
                Err(err) => {
                    return err.handle();
                }
            };
            derived_ipfs_id = ipfs_id.clone();
        } else {
            return validation_err_code("No code or ipfs hash provided", EC::NodeInvalidIPFSID, None)
                .add_source_to_details()
                .handle();
        }

        debug!("derived_ipfs_id: {}", derived_ipfs_id);

        // check if the IPFS id is in the allowlist
        if matches!(cfg.enable_actions_allowlist(), Ok(true)) {
            let allowlist_entry_id = keccak256(format!("LIT_ACTION_{}", derived_ipfs_id).as_bytes());
            let action_is_allowed = match check_allowlist(allowlist_cache, &allowlist_entry_id, &cfg).await {
                Ok(action_is_allowed) => action_is_allowed,
                Err(e) => {
                    return e.handle();
                }
            };
            if !action_is_allowed {
                return validation_err_code("Action not allowed", EC::NodeActionNotAllowed, None)
                    .handle();
            }
        }

        // create the lit resource
        let lit_action_resource = LitActionResource::new(json_execution_request.ipfs_id.clone().unwrap_or("".to_string()));

        // check for single or multiple auth sigs and do the session key
        // capability check.  set the wallet that provided the capabilities as the
        // main auth sig wallet.
        let auth_sig = match &json_execution_request.auth_sig {
            AuthSigItem::Single(single_auth_sig) => single_auth_sig,
            AuthSigItem::Multiple(_) => {
                return status::Custom(
                    Status::BadRequest,
                    json!({"message": "Multiple auth sigs not supported by Lit Actions", "errorCode": "unsupported_auth_sig"}),
                );
            }
        };

        let auth_context = match get_auth_context(
            Some(auth_sig.clone()),
            json_execution_request.auth_methods.clone(),
            json_execution_request.ipfs_id.clone(),
            Some(auth_context_cache),
            false,
            cfg.clone(),
            None
        )
            .await
        {
            Ok(auth_context) => auth_context,
            Err(e) => {
                return e.handle();
            }
        };
        trace!("Got auth context");

        let deno_execution_env = models::DenoExecutionEnv {
            tss_state: Some(tss_state.as_ref().clone()),
            auth_context,
            cfg
        };

        trace!("spawning js execution task");

        let spawner = functions::spawner::LocalSpawner::new();
        let (send, response) = oneshot::channel();
        if let Err(e) = spawner.spawn(functions::spawner::Task::ExecuteJs(
            code_to_run,
            Some(derived_ipfs_id),
            auth_sig.clone(),
            json_execution_request.js_params.clone(),
            deno_execution_env,
            tracing.clone().correlation_id().to_string(),
            send,
        )) {
            return e.handle();
        }

        let execution_result = match response.await {
            Ok(execution_result) => execution_result,
            Err(err) => {
                return unexpected_err_code(err, EC::NodeJsExecutionError, Some("Error talking to JS runtime".into()))
                    .handle();
            }
        };

        let rust_js_comms = match execution_result {
            Ok(rust_js_comms) => rust_js_comms,
            Err(err) => {
                match err.kind() {
                    lit_api_core::error::Kind::Timeout => {
                        return timeout_err_code(err, EC::NodeJsTimeoutError, None).handle();
                    },
                    lit_api_core::error::Kind::MemoryLimit => {
                        return memory_limit_err_code(err, EC::NodeJsMemoryLimitError, None).handle();
                    },
                    _ => {}
                }
                if let Some(source_err) = err.source() {
                    return status::Custom(
                        Status::BadRequest,
                        json!({"success": false, "error": source_err.to_string()}),
                    );
                }
                return unexpected_err_code(err, EC::NodeJsExecutionError, Some("Error executing JS".into())).handle();    
            }
        };
        if let Some(err) = rust_js_comms.error {
            return unexpected_err_code(err, EC::NodeJsExecutionError, Some("Error executing JS generated in Rust bindings".into())).handle();
        }

        trace!("js execution task completed");

        status::Custom(
            Status::Ok,
            json!({"success": true, "signedData": rust_js_comms.signed_data, "decryptedData": rust_js_comms.decrypted_data, "claimData": rust_js_comms.claim_data, "response": rust_js_comms.response, "logs": rust_js_comms.logs}),
        )
    }).await
}

// #[get("/web/eeid", format = "json")]
// #[instrument(name = "GET /web/eeid", skip_all, ret)]
// pub fn eeid(session: &State<Arc<bls::BlsState>>) -> Json<models::JsonEEID> {
//     Json(models::JsonEEID {
//         eeid: session.get_cdkg_eeid().ok(),
//     })
// }

#[post(
    "/web/sign_session_key",
    format = "json",
    data = "<json_sign_session_key_request>"
)]
#[instrument(name = "POST /web/sign_session_key", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn sign_session_key(
    _guard: ConcurrencyGuard<'_>,
    remote_addr: SocketAddr,
    tss_state: &State<Arc<crate::tss::common::tss_state::TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    cfg: &State<ReloadableLitConfig>,
    json_sign_session_key_request: Json<models::JsonSignSessionKeyRequest>,
    tracing: Tracing,
    request_headers: RequestHeaders<'_>,
) -> status::Custom<Value> {
    use crate::{
        error::validation_err,
        services::contract::get_pkp_pubkey,
        utils::web::pubkey_bytes_to_eth_address_bytes,
        utils::{contract::get_pkp_permissions_contract, siwe::validate_siwe},
    };

    with_context(tracing.clone(), async move {
        let cfg = cfg.load_full();

        debug!(
            "sign_session_key, request: {}",
            format!("{:?}", json_sign_session_key_request)
        );

        let mut auth_sig = None;
        if let Some(auth_sig_from_request) = &json_sign_session_key_request.auth_sig {
            match auth_sig_from_request {
                AuthSigItem::Single(single_auth_sig) => {
                    auth_sig = Some(single_auth_sig.clone());
                }
                AuthSigItem::Multiple(_) => {
                    return validation_err_code(
                        "Multiple auth sigs not supported by Lit Actions",
                        EC::NodeAuthSigNotSupported,
                        None,
                    )
                    .add_source_to_details()
                    .handle();
                }
            }
        }

        // Parse the SIWE message.
        let parsed_siwe = match siwe::Message::from_str(&json_sign_session_key_request.siwe_message)
        {
            Ok(parsed_siwe) => parsed_siwe,
            Err(e) => {
                return parser_err_code(
                    e,
                    EC::NodeSIWEMessageError,
                    Some("Error parsing SIWE message".into()),
                )
                .add_msg_to_details()
                .handle();
            }
        };

        let origin_domain = match get_domain_from_request_origin(
            request_headers
                .headers
                .get_one("Origin")
                .unwrap_or("http://localhost"),
        ) {
            Ok(origin_domain) => origin_domain,
            Err(e) => {
                error!(
                    "Error getting origin domain - swallowing and using default of localhost: {:?}",
                    e
                );
                "http://localhost".into()
            }
        };
        debug!("Origin: {:?}", origin_domain);

        // convert the auth methods into an auth context by resolving the oauth ids
        // from the oauth endpoints
        let auth_context = match get_auth_context(
            auth_sig.clone(),
            Some(json_sign_session_key_request.auth_methods.clone()),
            None,
            Some(auth_context_cache),
            true, // mark the auth method as used to prevent replay attacks
            cfg.clone(),
            Some(origin_domain),
        )
        .await
        {
            Ok(auth_context) => auth_context,
            Err(e) => {
                return e.handle();
            }
        };

        // Retrieve the PKP pubkey to sign. If the PKP pubkey is provided in the request, use that.
        // Otherwise, retrieve it from the smart contract using the auth method ID.
        let pkp_public_key = {
            if let Some(pkp_public_key) = &json_sign_session_key_request.pkp_public_key {
                let pubkey = encoding::hex_to_bytes(pkp_public_key).map_err(|e| {
                    conversion_err(
                        e,
                        Some("Unable to convert PKP public key from hex to bytes".into()),
                    )
                });
                let pubkey = match pubkey {
                    Ok(pubkey) => pubkey,
                    Err(e) => {
                        return e.handle();
                    }
                };

                // Convert the PKP public key to an ETH address.
                let pkp_eth_address = match pubkey_bytes_to_eth_address_bytes(pubkey.to_vec()) {
                    Ok(pkp_eth_address) => pkp_eth_address,
                    Err(e) => {
                        return parser_err(e, Some("Error hex decoding pkpPublicKey".into()))
                            .add_msg_to_details()
                            .handle();
                    }
                };

                // Validate the SIWE message contains the correct PKP public key.
                if parsed_siwe.address != pkp_eth_address {
                    return parser_err_code(
                        format!(
                            "Address in SIWE message {} does not match PKP ETH address {}",
                            encoding::bytes_to_hex(parsed_siwe.address),
                            encoding::bytes_to_hex(pkp_eth_address)
                        ),
                        EC::NodeSIWEMessageError,
                        None,
                    )
                    .add_source_to_details()
                    .handle();
                }

                Bytes::from(pubkey)
            } else {
                // Derive auth method ID. For now, just use the auth method from the auth context.
                if auth_context.auth_method_contexts.is_empty() {
                    return validation_err("No auth methods provided", None)
                        .add_source_to_details()
                        .handle();
                }
                let auth_method = auth_context.auth_method_contexts[0].clone();
                let auth_method_id =
                    match serialize_auth_context_for_checking_against_contract_data(&auth_method) {
                        Ok(auth_method_id) => auth_method_id,
                        Err(e) => {
                            return e.handle();
                        }
                    };

                // Get PKP permissions contract.
                let pkp_permissions_contract = match get_pkp_permissions_contract(cfg.clone()).await
                {
                    Ok(pkp_permissions_contract) => pkp_permissions_contract,
                    Err(e) => {
                        error!("Failed to get PKP permissions contract");
                        return e.handle();
                    }
                };

                // Get PKP public key.
                let pubkey = get_pkp_pubkey(
                    &pkp_permissions_contract,
                    auth_method.auth_method_type,
                    Bytes::from(auth_method_id),
                )
                .await;
                match pubkey {
                    Ok(pubkey) => pubkey,
                    Err(e) => {
                        error!("Failed to get PKP pubkey");
                        return e.handle();
                    }
                }
            }
        };

        // Convert the PKP public key to an ETH address.
        let pkp_eth_address = match pubkey_bytes_to_eth_address_bytes(pkp_public_key.to_vec()) {
            Ok(pkp_eth_address) => pkp_eth_address,
            Err(e) => {
                return parser_err(e, Some("Error hex decoding pkpPublicKey".into()))
                    .add_msg_to_details()
                    .handle();
            }
        };

        // Validate the SIWE message.
        {
            // check the uri field
            if parsed_siwe.uri != json_sign_session_key_request.session_key {
                return parser_err_code(
                    format!(
                        "URI in SIWE message {} does not match URI in request {}",
                        parsed_siwe.uri, json_sign_session_key_request.session_key
                    ),
                    EC::NodeSIWEMessageError,
                    None,
                )
                .add_source_to_details()
                .handle();
            }

            // Validate SIWE.
            let validate_res = validate_siwe(&parsed_siwe);
            if let Err(e) = validate_res {
                return e.handle();
            }
        }

        let port = match cfg.as_ref().external_port() {
            Ok(port) => port,
            Err(e) => {
                return unexpected_err(e, Some("Error getting port from config".into()))
                    .add_msg_to_details()
                    .handle();
            }
        };

        let rpc_provider = match get_provider(CHAIN_ETHEREUM, 0) {
            Ok(rpc_provider) => rpc_provider,
            Err(e) => {
                return unexpected_err(e, Some("Error getting RPC Provider for DB".into()))
                    .add_msg_to_details()
                    .handle();
            }
        };

        let retrieve_blockhash_res =
            match db::retrieve_and_store_blockhash(parsed_siwe.nonce, port, rpc_provider).await {
                Ok(retrieve_blockhash_res) => retrieve_blockhash_res,
                Err(e) => {
                    return validation_err(e, Some("Error fetching block from hash".into()))
                        .add_msg_to_details()
                        .handle();
                }
            };

        let (block_hash, block_timestamp) = retrieve_blockhash_res;

        // Construct a new SIWE message with the PKP address populated.
        let siwe_to_sign = siwe::Message {
            domain: parsed_siwe.domain,
            address: pkp_eth_address,
            statement: parsed_siwe.statement,
            uri: parsed_siwe.uri,
            version: parsed_siwe.version,
            chain_id: parsed_siwe.chain_id,
            nonce: block_hash,
            issued_at: block_timestamp,
            expiration_time: parsed_siwe.expiration_time,
            not_before: parsed_siwe.not_before,
            request_id: parsed_siwe.request_id,
            resources: parsed_siwe.resources,
        };

        // Construct the payload to sign.
        let to_sign = match siwe_to_sign.eip191_hash() {
            Ok(to_sign) => to_sign,
            Err(e) => {
                return parser_err_code(
                    e,
                    EC::NodeSIWEMessageError,
                    Some("Error hashing SIWE message".into()),
                )
                .add_msg_to_details()
                .handle();
            }
        };

        // sign the session key using the PKP
        debug!(
            "Signing session key {} with PKP {}",
            json_sign_session_key_request.session_key, pkp_public_key
        );
        let request_id = tracing.correlation_id().clone();

        let signing_state = match tss_state
            .get_signing_state(crate::tss::common::key_type::KeyType::EcdsaCaitSith)
        {
            Ok(s) => s,
            Err(e) => {
                return e.handle();
            }
        };
        let hex_pubkey = encoding::bytes_to_hex(&pkp_public_key);

        let result = match pkp::utils::sign_ecdsa(
            cfg.as_ref(),
            to_sign.as_ref(),
            hex_pubkey,
            request_id,
            None,
            auth_sig.clone(),
            auth_context,
            Some(tss_state.as_ref().clone()),
            &[2],
        )
        .await
        .map_err(|e| unexpected_err(e, Some("Error while signing with pkp".into())))
        {
            Ok(res) => res,
            Err(e) => {
                return e.handle();
            }
        };

        let sig_type = match result.1 {
            crate::tss::common::key_type::KeyType::EcdsaCaitSith => "ECDSA_CAIT_SITH",
            crate::tss::common::key_type::KeyType::BLS => "BLS",
        };

        if sig_type == "BLS" {
            return unexpected_err("Invalid keytype BLS for session signing", None).handle();
        }

        // return the signature share
        status::Custom(
            Status::Ok,
            json!({
                "success": true,
                "signedData": {
                    "sessionSig" : {
                        "sigType": sig_type,
                        "dataSigned": encoding::bytes_to_hex(to_sign),
                        "signatureShare": result.0.signature_share,
                        "shareIndex": result.0.share_index as u32,
                        "bigr": result.0.big_r,
                        "publicKey": encoding::bytes_to_hex(pkp_public_key).replace("0x", ""),
                        "sigName": "sessionSig",
                        "siweMessage": siwe_to_sign.to_string(),
                    }
                }
            }),
        )
    })
    .await
}

// Not dead code, rather a lint bug
// see https://github.com/rust-lang/rust/issues/92554
#[allow(dead_code)]
fn get_domain_from_request_origin(origin: &str) -> error::Result<String> {
    let origin = Url::parse(origin).map_err(|e| {
        conversion_err(e, Some(format!("Unable to parse origin URL of {}", origin)))
    })?;
    let domain = origin.domain().ok_or_else(|| {
        conversion_err(
            format!("Unable to parse domain from origin URL {}", origin),
            None,
        )
    })?;
    Ok(domain.to_string())
}

// Not dead code, rather a lint bug
// see https://github.com/rust-lang/rust/issues/92554
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
async fn check_multiple_access_control_conditions(
    auth_sig_item: &AuthSigItem,
    access_control_conditions: &Option<Vec<AccessControlConditionItem>>,
    evm_contract_conditions: &Option<Vec<EVMContractConditionItem>>,
    sol_rpc_conditions: &Option<Vec<SolRpcConditionItem>>,
    unified_access_control_conditions: &Option<Vec<UnifiedAccessControlConditionItem>>,
    cfg: Arc<LitConfig>,
    requested_lit_resource_ability: &LitResourceAbility,
    chain: &Option<String>,
    request_id: String,
) -> error::Result<models::UnifiedConditionCheckResult> {
    if let Some(access_control_conditions) = &access_control_conditions {
        let auth_sig = access_control::get_ethereum_auth_sig(auth_sig_item)?;

        let result = access_control::check_access_control_conditions(
            access_control_conditions,
            auth_sig,
            requested_lit_resource_ability,
            chain,
            cfg,
            &request_id,
        )
        .await?;
        return Ok(models::UnifiedConditionCheckResult {
            result,
            successful_auth_sig: (*auth_sig).clone(),
        });
    } else if let Some(evm_contract_conditions) = &evm_contract_conditions {
        let auth_sig = access_control::get_ethereum_auth_sig(auth_sig_item)?;

        let result = access_control::evm_contract::check_access_control_conditions(
            evm_contract_conditions,
            auth_sig,
            requested_lit_resource_ability,
            chain,
            &cfg,
        )
        .await?;

        return Ok(models::UnifiedConditionCheckResult {
            result,
            successful_auth_sig: auth_sig.clone(),
        });
    } else if let Some(sol_rpc_conditions) = &sol_rpc_conditions {
        let auth_sig = access_control::get_solana_auth_sig(auth_sig_item)?;

        let result =
            access_control::sol_rpc::check_access_control_conditions(sol_rpc_conditions, auth_sig)
                .await?;
        return Ok(models::UnifiedConditionCheckResult {
            result,
            successful_auth_sig: auth_sig.clone(),
        });
    } else if let Some(unified_access_control_conditions) = &unified_access_control_conditions {
        return access_control::unified::check_access_control_conditions(
            unified_access_control_conditions,
            auth_sig_item,
            chain.clone(),
            requested_lit_resource_ability,
            cfg,
            &request_id,
        )
        .await;
    }

    Err(validation_err_code("Missing access control conditions", EC::NodeMissingAccessControlConditions, None)
        .add_detail("You must pass either access_control_conditions or evm_contract_conditions or sol_rpc_conditions or unified_access_control_conditions"))
}
