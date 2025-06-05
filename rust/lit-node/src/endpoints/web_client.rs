#[allow(unused_imports)]
use ethers::types::Bytes;
use ipfs_hasher::IpfsHasher;
use lit_api_core::context::Tracer;
use lit_api_core::context::{SdkVersion, Tracing, TracingRequired};
use lit_api_core::error::ApiError;
use lit_blockchain::resolver::rpc::{RpcHealthcheckPoller, ENDPOINT_MANAGER};
use lit_core::config::{LitConfig, ReloadableLitConfig};
use moka::future::Cache;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::State;
use siwe_recap::Capability;
use std::collections::BTreeMap;
use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tracing::instrument;
use url::Url;

use crate::auth::auth_material::{siwe_hash_to_bls_session_hash, AuthSigItem};
use crate::auth::lit_resource::LitResource;
use crate::auth::resources::{AccessControlConditionResource, LitResourceAbility};
use crate::config::LitNodeConfig;
use crate::constants::CHAIN_ETHEREUM;
use crate::error::{
    connect_err_code, conversion_err, memory_limit_err_code, timeout_err_code, unexpected_err,
    unexpected_err_code, validation_err_code, EC,
};
use crate::error::{parser_err, parser_err_code};
use crate::functions::action_client;
use crate::models::auth::SessionKeySignedMessage;
use crate::models::auth::{LIT_RESOURCE_KEY_RAC, LIT_RESOURCE_PREFIX_RAC};
use crate::models::{
    self, AccessControlConditionItem, EVMContractConditionItem, JsonSignSessionKeyResponse,
    JsonSignSessionKeyResponseInnerSigShare, JsonSignSessionKeyResponseInnerSignedData,
    JwtPayloadV2, RequestConditions, SigningAccessControlConditionResponse, SolRpcConditionItem,
    UnifiedAccessControlConditionItem,
};
use crate::pkp;
use crate::pkp::auth::{
    serialize_auth_context_for_checking_against_contract_data, AuthMethodScope,
};
use crate::rate_limiting::models::UserContext;
use crate::rate_limiting::{check_rate_limit, models::RateLimitDB};
use crate::siwe_db::utils::make_timestamp_siwe_compatible;
use crate::tss::common::curve_type::CurveType;
use crate::tss::common::tss_state::TssState;
use crate::utils::attestation::create_attestation;
use crate::utils::encoding;
use crate::utils::rocket::guards::RequestHeaders;
use crate::utils::web::get_auth_context_from_session_sigs;
use crate::utils::web::EndpointVersion;
use crate::utils::web::{
    check_condition_count, get_auth_context, get_bls_root_pubkey, get_ipfs_file,
    hash_access_control_conditions,
};
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
#[instrument(name = "POST /web/signing/access_control_condition", skip_all, ret)]
pub(crate) async fn signing_access_control_condition(
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    signing_access_control_condition_request: Json<models::SigningAccessControlConditionRequest>,
    tracing: Tracing,
    endpoint_version: EndpointVersion,
) -> status::Custom<Value> {
    let request_start = std::time::Instant::now();
    debug!(
        "signing_access_control_condition_request, request: {:}",
        format!("{:?}", signing_access_control_condition_request)
    );

    let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

    let cfg = cfg.load_full();

    let before = std::time::Instant::now();
    let cc_check = check_condition_count(
        &signing_access_control_condition_request.access_control_conditions,
        &signing_access_control_condition_request.evm_contract_conditions,
        &signing_access_control_condition_request.sol_rpc_conditions,
        &signing_access_control_condition_request.unified_access_control_conditions,
    );
    if let Err(e) = cc_check {
        return e.handle();
    }
    timing.insert("check condition count".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    // Hash the access control condition
    let hash_res = hash_access_control_conditions(RequestConditions {
        access_control_conditions: signing_access_control_condition_request
            .access_control_conditions
            .clone(),
        evm_contract_conditions: signing_access_control_condition_request
            .evm_contract_conditions
            .clone(),
        sol_rpc_conditions: signing_access_control_condition_request
            .sol_rpc_conditions
            .clone(),
        unified_access_control_conditions: signing_access_control_condition_request
            .unified_access_control_conditions
            .clone(),
    });
    let hashed_access_control_conditions = match hash_res {
        Ok(hashed_access_control_conditions) => hashed_access_control_conditions,
        Err(e) => {
            return e.handle();
        }
    };
    timing.insert(
        "hash access control conditions".to_string(),
        before.elapsed(),
    );

    let before = std::time::Instant::now();

    let lit_acc_resource = AccessControlConditionResource::new(hashed_access_control_conditions);

    // Validate auth sig item
    let bls_root_pubkey = match get_bls_root_pubkey(session).await {
        Ok(bls_root_pubkey) => bls_root_pubkey,
        Err(e) => {
            return e.handle();
        }
    };
    let validated_address = {
        match signing_access_control_condition_request
            .auth_sig
            .validate_and_get_user_address(
                &lit_acc_resource.signing_ability(),
                &signing_access_control_condition_request.chain.clone(),
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
    timing.insert("validate auth sig".to_string(), before.elapsed());

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
        &UserContext { user_address },
        &signing_access_control_condition_request.auth_sig,
        signing_access_control_condition_request.chain.clone(),
        rate_limit_db,
        remote_addr,
        &lit_acc_resource.signing_ability(),
        &cfg,
        &bls_root_pubkey,
    )
    .await;
    timing.insert("check rate limit".to_string(), before.elapsed());

    let rate_limit_check_return = match rate_limit_res {
        Ok(rate_limit_check_return) => rate_limit_check_return,
        Err(e) => {
            return e.handle();
        }
    };
    if rate_limit_check_return.rate_limit_exceeded {
        let msg = match rate_limit_check_return.try_again_after {
            Some(try_again_after) => {
                format!("Rate limit exceeded.  Try again at {}", try_again_after)
            }
            None => "Rate limit exceeded.  Try again later.".to_string(),
        };
        warn!("{}", msg);
        return status::Custom(
            Status::PaymentRequired,
            json!({"message": msg, "errorCode": "rate_limit_exceeded"}),
        );
    }

    let before = std::time::Instant::now();
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
        tracing.clone().correlation_id().to_string(),
        &bls_root_pubkey,
        &endpoint_version,
        None,
        ipfs_cache,
    )
    .await;
    timing.insert(
        "check access control conditions".to_string(),
        before.elapsed(),
    );

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
            return unexpected_err(
                e,
                Some(
                    "System time is before unix epoch. Your computer clock is probably wrong"
                        .into(),
                ),
            )
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
        || signing_access_control_condition_request.exp
            > now + grace_period_seconds + MAX_JWT_EXPIRATION
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
        access_control_conditions: signing_access_control_condition_request
            .access_control_conditions
            .clone(),
        evm_contract_conditions: signing_access_control_condition_request
            .evm_contract_conditions
            .clone(),
        sol_rpc_conditions: signing_access_control_condition_request
            .sol_rpc_conditions
            .clone(),
        unified_access_control_conditions: signing_access_control_condition_request
            .unified_access_control_conditions
            .clone(),
    };

    let message_to_sign = match jwt::generate_unsigned_jwt_v2(&payload) {
        Ok(message_to_sign) => message_to_sign,
        Err(e) => {
            return e.handle();
        }
    };

    let before = std::time::Instant::now();
    // Load the BLS secret key share as a blsful key for signing.
    let cipher_state = match session.get_cipher_state(CurveType::BLS) {
        Ok(cipher_state) => cipher_state,
        Err(e) => {
            return e.handle();
        }
    };
    timing.insert("get cipher state".to_string(), before.elapsed());

    let before = std::time::Instant::now();

    let epoch = match signing_access_control_condition_request.epoch {
        0 => None,
        _ => Some(signing_access_control_condition_request.epoch),
    };

    // Sign the JWT using the blsful secret key share.
    let (signature_share, share_index) = match cipher_state
        .sign(message_to_sign.clone().into_bytes(), epoch)
        .await
    {
        Ok(signature_share) => signature_share,
        Err(e) => {
            return e.handle();
        }
    };
    timing.insert("sign JWT".to_string(), before.elapsed());

    timing.insert("total".to_string(), request_start.elapsed());

    debug!(
        "POST /web/signing/access_control_condition timing: {:?}",
        timing
    );

    status::Custom(
        Status::Ok,
        json!(SigningAccessControlConditionResponse {
            result: "success".to_string(),
            signature_share,
            share_index,
            unsigned_jwt: message_to_sign,
        }),
    )
}

#[instrument(name = "POST /web/encryption/sign", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn encryption_sign(
    session: &Arc<TssState>,
    remote_addr: SocketAddr,
    rate_limit_db: &Arc<RateLimitDB>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &ReloadableLitConfig,
    encryption_sign_request: Json<models::EncryptionSignRequest>,
    tracing: Tracing,
    endpoint_version: EndpointVersion,
) -> status::Custom<Value> {
    let request_start = std::time::Instant::now();
    debug!(
        "encryption_sign, request: {:}",
        format!("{:?}", encryption_sign_request)
    );

    let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

    let cfg = cfg.load_full();

    let before = std::time::Instant::now();
    let cc_check = check_condition_count(
        &encryption_sign_request.access_control_conditions,
        &encryption_sign_request.evm_contract_conditions,
        &encryption_sign_request.sol_rpc_conditions,
        &encryption_sign_request.unified_access_control_conditions,
    );
    if let Err(e) = cc_check {
        return e.handle();
    }
    timing.insert("check condition count".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    // Hash the access control condition
    let hash_res = hash_access_control_conditions(RequestConditions {
        access_control_conditions: encryption_sign_request.access_control_conditions.clone(),
        evm_contract_conditions: encryption_sign_request.evm_contract_conditions.clone(),
        sol_rpc_conditions: encryption_sign_request.sol_rpc_conditions.clone(),
        unified_access_control_conditions: encryption_sign_request
            .unified_access_control_conditions
            .clone(),
    });
    let hashed_access_control_conditions = match hash_res {
        Ok(hashed_access_control_conditions) => hashed_access_control_conditions,
        Err(e) => {
            return e.handle();
        }
    };
    timing.insert(
        "hash access control conditions".to_string(),
        before.elapsed(),
    );

    let lit_acc_resource = AccessControlConditionResource::new(format!(
        "{}/{}",
        hashed_access_control_conditions, encryption_sign_request.data_to_encrypt_hash
    ));
    debug!("lit_acc_resource: {:?}", lit_acc_resource);

    let before = std::time::Instant::now();
    // Validate auth sig item
    let bls_root_pubkey = match get_bls_root_pubkey(session).await {
        Ok(bls_root_pubkey) => bls_root_pubkey,
        Err(e) => {
            return e.handle();
        }
    };
    let validated_address = {
        match encryption_sign_request
            .auth_sig
            .validate_and_get_user_address(
                &lit_acc_resource.decrypt_ability(),
                &encryption_sign_request.chain.clone(),
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
    timing.insert("validate auth sig".to_string(), before.elapsed());
    info!("Validated user address: {:?}", validated_address);

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
        &UserContext { user_address },
        &encryption_sign_request.auth_sig,
        encryption_sign_request.chain.clone(),
        rate_limit_db,
        remote_addr,
        &lit_acc_resource.decrypt_ability(),
        &cfg,
        &bls_root_pubkey,
    )
    .await;
    timing.insert("check rate limit".to_string(), before.elapsed());

    let rate_limit_check_return = match rate_limit_res {
        Ok(rate_limit_check_return) => rate_limit_check_return,
        Err(e) => {
            return e.handle();
        }
    };
    if rate_limit_check_return.rate_limit_exceeded {
        let msg = match rate_limit_check_return.try_again_after {
            Some(try_again_after) => {
                format!("Rate limit exceeded.  Try again at {}", try_again_after)
            }
            None => "Rate limit exceeded.  Try again later.".to_string(),
        };
        warn!("{}", msg);
        return status::Custom(
            Status::PaymentRequired,
            json!({"message": msg, "errorCode": "rate_limit_exceeded"}),
        );
    }

    let before = std::time::Instant::now();
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
        tracing.clone().correlation_id().to_string(),
        &bls_root_pubkey,
        &endpoint_version,
        None,
        ipfs_cache,
    )
    .await;
    timing.insert(
        "check access control conditions".to_string(),
        before.elapsed(),
    );
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

    let before = std::time::Instant::now();
    // Load the BLS secret key share as a blsful key for signing.
    let cipher_state = match session.get_cipher_state(CurveType::BLS) {
        Ok(cipher_state) => cipher_state,
        Err(e) => {
            return e.handle();
        }
    };
    timing.insert("get cipher state".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    let epoch = match encryption_sign_request.epoch {
        0 => None,
        _ => Some(encryption_sign_request.epoch),
    };
    // Sign the identity parameter using the blsful secret key share.
    let (signature_share, share_index) = match cipher_state.sign(identity_parameter, epoch).await {
        Ok(signature_share) => signature_share,
        Err(e) => {
            return e.handle();
        }
    };
    timing.insert("sign identity parameter".to_string(), before.elapsed());
    timing.insert("total".to_string(), request_start.elapsed());

    debug!("POST /web/encryption/sign timing: {:?}", timing);

    status::Custom(
        Status::Ok,
        json!(models::EncryptionSignResponse {
            result: "success".to_string(),
            signature_share,
            share_index,
        }),
    )
}

/*
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"clientPublicKey": "test"}' \
  http://localhost:7470/web/handshake
*/

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
    let request_start = std::time::Instant::now();
    let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

    let cfg = cfg.load_full();

    let before = std::time::Instant::now();
    let ecdsa_root_keys = match session.get_signing_state(CurveType::K256) {
        Ok(signing_state) => signing_state.root_keys().await,
        Err(_) => {
            warn!("Failed to aquire lock on hd_root_keys for ECDSA.");
            vec![]
        }
    };
    timing.insert("get ecdsa root keys".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    let bls_root_keys = match session.get_cipher_state(CurveType::BLS) {
        Ok(cipher_state) => cipher_state.root_keys().await,
        Err(_) => {
            warn!("Failed to aquire lock on hd_root_keys for BLS.");
            vec![]
        }
    };
    timing.insert("get bls root keys".to_string(), before.elapsed());

    // FIXME: remove this "unwrap_or" once we've shipped sending the challenge in the SDK
    let challenge = json_handshake_request
        .challenge
        .clone()
        .unwrap_or("0x1234".to_string());

    let before = std::time::Instant::now();
    // run the attestation
    let attestation = match tokio::time::timeout(
        Duration::from_secs(cfg.http_client_timeout().expect(
            "http_client_timeout is required, and defined in defaults, so this should never happen",
        )),
        create_attestation(cfg, challenge.as_str()),
    )
    .await
    .map_err(|e| unexpected_err(e, Some("error producing attestation".into())))
    {
        Ok(attestation) => match attestation {
            Ok(attestation) => Some(attestation),
            Err(e) => {
                #[cfg(not(feature = "testing"))]
                warn!("Error creating attestation: {:?}", e);
                None
            }
        },
        Err(e) => {
            #[cfg(not(feature = "testing"))]
            warn!("Timeout error creating attestation: {:?}", e);
            None
        }
    };
    timing.insert("create attestation".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    let latest_blockhash = eth_blockhash_cache.blockhash.read().await.clone();
    timing.insert("get latest blockhash".to_string(), before.elapsed());

    timing.insert("total".to_string(), request_start.elapsed());

    debug!("POST /web/handshake timing: {:?}", timing);

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

#[cfg(feature = "lit-actions")]
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
    endpoint_version: EndpointVersion,
) -> status::Custom<Value> {
    use crate::auth::resources::LitActionResource;
    use crate::config::LitNodeConfig;
    use crate::constants::CHAIN_ETHEREUM;
    use crate::utils::web::check_allowlist;
    use ethers::utils::keccak256;

    let request_start = std::time::Instant::now();

    debug!(
        "execute, request: {:}",
        format!("{:?}", json_execution_request)
    );

    let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

    let cfg = cfg.load_full();

    // get the derived IPFS ID so that we can auth the user against it
    let before = std::time::Instant::now();
    // determine if the user passed code or an ipfs hash
    let derived_ipfs_id;
    let code_to_run: Arc<String>;
    if let Some(code) = &json_execution_request.code {
        let decoded_bytes = match data_encoding::BASE64.decode(code.as_bytes()) {
            Ok(decoded_bytes) => decoded_bytes,
            Err(err) => {
                return conversion_err(
                    err,
                    Some("Your Lit Action code could not be decoded from base64".into()),
                )
                .add_msg_to_details()
                .handle();
            }
        };

        match String::from_utf8(decoded_bytes) {
            Ok(string_result) => code_to_run = Arc::new(string_result),
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
        match get_ipfs_file(
            &ipfs_id.to_string(),
            &cfg,
            moka::future::Cache::clone(ipfs_cache),
        )
        .await
        {
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
    timing.insert("derived IPFS CID".to_string(), before.elapsed());

    debug!("derived_ipfs_id: {}", derived_ipfs_id);

    let capability_protocol_prefix = &"litAction".to_string();
    let lit_action_resource = LitActionResource::new(derived_ipfs_id.clone());

    let before = std::time::Instant::now();
    // Validate auth sig item
    let bls_root_pubkey = match get_bls_root_pubkey(tss_state).await {
        Ok(bls_root_pubkey) => bls_root_pubkey,
        Err(e) => {
            return e.handle();
        }
    };

    let validated_address = {
        match json_execution_request
            .auth_sig
            .validate_and_get_user_address(
                &lit_action_resource.execution_ability(),
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
        &UserContext { user_address },
        &json_execution_request.auth_sig,
        // paying for Lit Actions is only supported on EVM right now, so using ethereum as the chain will always work
        Some(CHAIN_ETHEREUM.to_string()),
        rate_limit_db,
        remote_addr,
        &lit_action_resource.execution_ability(),
        &cfg,
        &bls_root_pubkey,
    )
    .await;
    timing.insert("rate limit check".to_string(), before.elapsed());

    let rate_limit_check_return = match rate_limit_res {
        Ok(rate_limit_check_return) => rate_limit_check_return,
        Err(e) => {
            return e.handle();
        }
    };

    if rate_limit_check_return.rate_limit_exceeded {
        let msg = match rate_limit_check_return.try_again_after {
            Some(try_again_after) => {
                format!("Rate limit exceeded.  Try again at {}", try_again_after)
            }
            None => "Rate limit exceeded.  Try again later.".to_string(),
        };
        warn!("{}", msg);
        return status::Custom(
            Status::PaymentRequired,
            json!({"message": msg, "errorCode": "rate_limit_exceeded"}),
        );
    }

    let before = std::time::Instant::now();
    // check if the IPFS id is in the allowlist
    if matches!(cfg.enable_actions_allowlist(), Ok(true)) {
        let allowlist_entry_id = keccak256(format!("LIT_ACTION_{}", derived_ipfs_id).as_bytes());
        let action_is_allowed =
            match check_allowlist(allowlist_cache, &allowlist_entry_id, &cfg).await {
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
    timing.insert("checked allowlist".to_string(), before.elapsed());

    // create the lit resource
    let lit_action_resource = LitActionResource::new(derived_ipfs_id.clone());

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

    let before = std::time::Instant::now();
    trace!("getting auth context");

    let auth_context = match endpoint_version {
        EndpointVersion::Initial => {
            let auth_context = get_auth_context(
                Some(auth_sig.clone()),
                json_execution_request.auth_methods.clone(),
                json_execution_request.ipfs_id.clone(),
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
        }
        EndpointVersion::V1 => {
            let session_key_signed_message: std::result::Result<
                SessionKeySignedMessage,
                serde_json::Error,
            > = serde_json::from_str(&auth_sig.signed_message);
            let session_key_signed_message = match session_key_signed_message {
                Ok(session_key_signed_message) => session_key_signed_message,
                Err(e) => {
                    error!("Error parsing session sig in execute_js");
                    return status::Custom(
                        Status::BadRequest,
                        json!({"message": "Either you've have passed an AuthSig or the sessionSig is incorrectly formatted", "errorCode": "unsupported_auth_sig"}),
                    );
                }
            };

            timing.insert("parsed session sig".to_string(), before.elapsed());

            let resolved_auth_context =
                match get_auth_context_from_session_sigs(session_key_signed_message).await {
                    Ok(resolved_auth_context) => resolved_auth_context,
                    Err(e) => {
                        error!("Error parsing AuthContext from sessionSig");
                        return e.handle();
                    }
                };

            match resolved_auth_context {
                Some(resolved_auth_context) => resolved_auth_context,
                None => {
                    // Also create new auth_context for EOA authSig/sessionSigs
                    let new_auth_context = get_auth_context(
                        Some(auth_sig.clone()),
                        None,
                        Some(derived_ipfs_id.clone()),
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

    let deno_execution_env = models::DenoExecutionEnv {
        tss_state: Some(tss_state.as_ref().clone()),
        auth_context,
        cfg,
        ipfs_cache: Some(moka::future::Cache::clone(ipfs_cache)),
    };

    let http_headers = {
        let mut res: BTreeMap<String, String> = BTreeMap::new();
        for h in request_headers.headers.iter() {
            let (name, value) = (h.name.to_string(), h.value.to_string());
            // If necessary, combine multiple headers with the same name into a single header
            res.entry(name)
                .and_modify(|e| e.push_str(&format!(", {value}")))
                .or_insert(value);
        }
        res
    };

    trace!("spawning js execution task");

    let epoch = match json_execution_request.epoch {
        0 => None,
        i => Some(i),
    };

    let lit_action_config = tss_state
        .peer_state
        .chain_data_config_manager
        .generic_config
        .read()
        .await
        .lit_action_config
        .clone();
    let before = std::time::Instant::now();
    let mut client = match action_client::ClientBuilder::default()
        .js_env(deno_execution_env)
        .auth_sig(Some(auth_sig.clone()))
        .request_id(tracing.clone().correlation_id().to_string())
        .http_headers(http_headers)
        .timeout_ms(lit_action_config.timeout_ms)
        .memory_limit_mb(lit_action_config.memory_limit_mb as u32)
        .max_code_length(lit_action_config.max_code_length as usize)
        .max_response_length(lit_action_config.max_response_length as usize)
        .max_fetch_count(lit_action_config.max_fetch_count as u32)
        .max_sign_count(lit_action_config.max_sign_count as u32)
        .max_contract_call_count(lit_action_config.max_contract_call_count as u32)
        .max_broadcast_and_collect_count(lit_action_config.max_broadcast_and_collect_count as u32)
        .max_call_depth(lit_action_config.max_call_depth as u32)
        .max_retries(lit_action_config.max_retries as u32)
        .epoch(epoch)
        .endpoint_version(endpoint_version)
        .build()
        .map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeJsExecutionError,
                Some("Error building action client".into()),
            )
        }) {
        Ok(client) => client,
        Err(e) => return e.handle(),
    };
    let execution_result = client
        .execute_js(action_client::ExecutionOptions {
            code: code_to_run,
            globals: json_execution_request.js_params.clone(),
            action_ipfs_id: Some(derived_ipfs_id),
        })
        .await;
    timing.insert("js execution".to_string(), before.elapsed());

    let execution_state = match execution_result {
        Ok(state) => state,
        Err(err) => {
            error!("error in Js comms result: {err:?}");
            let logs = client.logs();
            match err.kind() {
                lit_api_core::error::Kind::Timeout => {
                    return timeout_err_code(err, EC::NodeJsTimeoutError, None)
                        .handle_with_logs(logs);
                }
                lit_api_core::error::Kind::MemoryLimit => {
                    return memory_limit_err_code(err, EC::NodeJsMemoryLimitError, None)
                        .handle_with_logs(logs);
                }
                lit_api_core::error::Kind::Connect => {
                    return connect_err_code(err, EC::NodeJsConnectionError, None)
                        .handle_with_logs(logs);
                }
                _ => {}
            }
            if let Some(source_err) = err.source() {
                return status::Custom(
                    Status::BadRequest,
                    json!({"success": false, "error": source_err.to_string(), "logs": logs}),
                );
            }
            return unexpected_err_code(
                err,
                EC::NodeJsExecutionError,
                Some("Error executing JS".into()),
            )
            .handle_with_logs(logs);
        }
    };

    trace!("js execution task completed");

    timing.insert("total".to_string(), request_start.elapsed());
    debug!("POST /web/execute timing: {:?}", timing);

    status::Custom(
        Status::Ok,
        json!({"success": true, "signedData": execution_state.signed_data, "decryptedData": {}, "claimData": execution_state.claim_data, "response": execution_state.response, "logs": execution_state.logs}),
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn sign_session_key(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<crate::tss::common::tss_state::TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    rate_limit_db: &State<Arc<RateLimitDB>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    json_sign_session_key_request: models::JsonSignSessionKeyRequestV1,
    tracing: Tracing,
    request_headers: RequestHeaders<'_>,
    endpoint_version: EndpointVersion,
) -> status::Custom<Value> {
    use crate::{
        error::validation_err,
        services::contract::get_pkp_pubkey,
        utils::web::pubkey_bytes_to_eth_address_bytes,
        utils::{contract::get_pkp_permissions_contract, siwe::validate_siwe},
    };

    let request_start = std::time::Instant::now();

    let cfg = cfg.load_full();

    debug!(
        "sign_session_key, request: {}",
        format!("{:?}", json_sign_session_key_request)
    );

    let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

    let before = std::time::Instant::now();
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
    let parsed_siwe = match siwe::Message::from_str(&json_sign_session_key_request.siwe_message) {
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
    timing.insert("parsed auth sig".to_string(), before.elapsed());

    if let Some(statement) = &parsed_siwe.statement {
        if statement.contains(LIT_RESOURCE_PREFIX_RAC) {
            return validation_err_code(
                "Can't define Auth Context resources in capability",
                EC::NodeInvalidAuthContextResource,
                None,
            )
            .add_msg_to_details()
            .handle();
        }
    }

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

    let before = std::time::Instant::now();
    // convert the auth methods into an auth context by resolving the oauth ids
    // from the oauth endpoints
    let bls_root_pubkey = match get_bls_root_pubkey(tss_state).await {
        Ok(bls_root_pubkey) => bls_root_pubkey,
        Err(e) => {
            return e.handle();
        }
    };
    let mut auth_context = match get_auth_context(
        auth_sig.clone(),
        Some(json_sign_session_key_request.auth_methods.clone()),
        None,
        Some(auth_context_cache),
        true, // mark the auth method as used to prevent replay attacks
        cfg.clone(),
        Some(origin_domain),
        &bls_root_pubkey,
        &endpoint_version,
    )
    .await
    {
        Ok(auth_context) => auth_context,
        Err(e) => {
            return e.handle();
        }
    };
    timing.insert("auth context".to_string(), before.elapsed());

    let before = std::time::Instant::now();
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
            let pkp_permissions_contract = match get_pkp_permissions_contract(cfg.clone()).await {
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
    timing.insert("retrieved pkp pubkey".to_string(), before.elapsed());

    // Convert the PKP public key to an ETH address.
    let pkp_eth_address = match pubkey_bytes_to_eth_address_bytes(pkp_public_key.to_vec()) {
        Ok(pkp_eth_address) => pkp_eth_address,
        Err(e) => {
            return parser_err(e, Some("Error hex decoding pkpPublicKey".into()))
                .add_msg_to_details()
                .handle();
        }
    };

    let before = std::time::Instant::now();
    // Validate the SIWE message.
    if let Ok(true) = cfg.enable_siwe_validation() {
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
        if let Err(e) = validate_siwe(&parsed_siwe) {
            return e.handle();
        }
    }
    timing.insert("validated siwe".to_string(), before.elapsed());

    let epoch = match json_sign_session_key_request.epoch {
        0 => None,
        i => Some(i),
    };

    let mut derived_ipfs_id = None;

    // Validate the Lit Action result if present
    match (
        &json_sign_session_key_request.code,
        &json_sign_session_key_request.lit_action_ipfs_id,
    ) {
        (Some(_), Some(_)) => {
            return unexpected_err("Can't provide both code & lit_action_ipfs_id", None)
                .add_source_to_details()
                .handle();
        }
        (None, None) => trace!("Not running Lit Action for signing session sigs"),
        (Some(_), None) | (None, Some(_)) => {
            trace!("Running Lit Action for signing session sigs");

            let code_to_run;

            if let Some(code) = &json_sign_session_key_request.code {
                let decoded_bytes = match data_encoding::BASE64.decode(code.as_bytes()) {
                    Ok(decoded_bytes) => decoded_bytes,
                    Err(err) => {
                        return conversion_err(
                            err,
                            Some("Your Lit Action code could not be decoded from base64".into()),
                        )
                        .add_msg_to_details()
                        .handle();
                    }
                };

                match String::from_utf8(decoded_bytes) {
                    Ok(string_result) => code_to_run = Arc::new(string_result),
                    Err(err) => {
                        return conversion_err(
                                err,
                                Some("Your Lit Action code could not be converted from base64 decoded bytes into a string.  Please check your base64 encoding code.".into()),
                            )
                            .add_msg_to_details()
                            .handle();
                    }
                };

                // hash the code to get the ipfs id
                let ipfs_hasher = IpfsHasher::default();
                let cid = ipfs_hasher.compute(code_to_run.as_bytes());
                derived_ipfs_id = Some(cid);
            } else {
                #[allow(clippy::unwrap_used)]
                let ipfs_id = json_sign_session_key_request
                    .lit_action_ipfs_id
                    .as_ref()
                    .unwrap(); // We check that either the code is provided or the ipfs_cid
                               // pull down the code from ipfs
                match get_ipfs_file(&ipfs_id.to_string(), &cfg, ipfs_cache.inner().clone()).await {
                    Ok(ipfs_result) => code_to_run = ipfs_result,
                    Err(err) => {
                        return err.handle();
                    }
                };
                derived_ipfs_id = Some(ipfs_id.clone());
            }

            let deno_execution_env = models::DenoExecutionEnv {
                tss_state: Some(tss_state.as_ref().clone()),
                auth_context: auth_context.clone(),
                cfg: cfg.clone(),
                ipfs_cache: Some(moka::future::Cache::clone(ipfs_cache)),
            };

            trace!("spawning js execution task");

            let http_headers = {
                let mut res: BTreeMap<String, String> = BTreeMap::new();
                for h in request_headers.headers.iter() {
                    let (name, value) = (h.name.to_string(), h.value.to_string());
                    // If necessary, combine multiple headers with the same name into a single header
                    res.entry(name)
                        .and_modify(|e| e.push_str(&format!(", {value}")))
                        .or_insert(value);
                }
                res
            };

            let mut client = match action_client::ClientBuilder::default()
                .js_env(deno_execution_env)
                .request_id(tracing.clone().correlation_id().to_string())
                .http_headers(http_headers)
                .epoch(epoch)
                .endpoint_version(endpoint_version)
                .build()
                .map_err(|e| {
                    unexpected_err_code(
                        e,
                        EC::NodeJsExecutionError,
                        Some("Error building action client".into()),
                    )
                }) {
                Ok(client) => client,
                Err(e) => return e.handle(),
            };
            let execution_result = client
                .execute_js(action_client::ExecutionOptions {
                    code: code_to_run,
                    globals: json_sign_session_key_request.js_params.clone(),
                    action_ipfs_id: derived_ipfs_id.clone(),
                })
                .await;
            let execution_state = match execution_result {
                Ok(state) => state,
                Err(err) => {
                    let logs = client.logs();
                    match err.kind() {
                        lit_api_core::error::Kind::Timeout => {
                            return timeout_err_code(err, EC::NodeJsTimeoutError, None)
                                .handle_with_logs(logs);
                        }
                        lit_api_core::error::Kind::MemoryLimit => {
                            return memory_limit_err_code(err, EC::NodeJsMemoryLimitError, None)
                                .handle_with_logs(logs);
                        }
                        _ => {}
                    }

                    if let Some(source_err) = err.source() {
                        return status::Custom(
                            Status::BadRequest,
                            json!({"success": false, "error": source_err.to_string(), "logs": logs}),
                        );
                    }

                    return unexpected_err_code(
                        err,
                        EC::NodeJsExecutionError,
                        Some("Error executing JS".into()),
                    )
                    .handle_with_logs(logs);
                }
            };

            trace!("js execution task completed");

            if execution_state.response.contains("true") {
                info!("Successful Lit Actions validation");

                match &derived_ipfs_id {
                    Some(derived_ipfs_id) => {
                        // Add to auth_context as we ran & verfied Lit Action auth
                        auth_context
                            .action_ipfs_id_stack
                            .push(derived_ipfs_id.clone());

                        auth_context.custom_auth_resource = execution_state.response;
                    }
                    None => {
                        warn!("Undefined derived_ipfs_id despite the user providing code/lit_actions_ipfs_id");
                    }
                };
            } else {
                return validation_err_code(
                    "Authentication failed for session_sig signing via Lit Action",
                    EC::NodeLitActionsSessionSigAuthenticationFailed,
                    None,
                )
                .add_source_to_details()
                .handle();
            }
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

    let rpc_provider = match ENDPOINT_MANAGER.get_provider(CHAIN_ETHEREUM) {
        Ok(rpc_provider) => rpc_provider,
        Err(e) => {
            return unexpected_err(e, Some("Error getting RPC Provider for DB".into()))
                .add_msg_to_details()
                .handle();
        }
    };

    let before = std::time::Instant::now();
    let block =
        match db::retrieve_and_store_blockhash(parsed_siwe.nonce.clone(), port, rpc_provider).await
        {
            Ok(retrieve_blockhash_res) => retrieve_blockhash_res,
            Err(e) => {
                return validation_err(e, Some("Error fetching block from hash".into()))
                    .add_msg_to_details()
                    .handle();
            }
        };
    let siwe_timestamp = match make_timestamp_siwe_compatible(&block.timestamp) {
        Ok(res) => res,
        Err(e) => {
            return e.handle();
        }
    };
    timing.insert("retrieved blockhash".to_string(), before.elapsed());

    let hex_pubkey = encoding::bytes_to_hex(&pkp_public_key);

    let siwe_to_sign;
    let to_sign;
    let before = std::time::Instant::now();
    // are we gonna sign with ECDSA or BLS?
    match &json_sign_session_key_request.curve_type {
        CurveType::K256 => {
            // sign the session key via ECDSA using the PKP

            // Construct a new SIWE message with the PKP address populated.
            siwe_to_sign = siwe::Message {
                domain: parsed_siwe.domain,
                address: pkp_eth_address,
                statement: parsed_siwe.statement,
                uri: parsed_siwe.uri,
                version: parsed_siwe.version,
                chain_id: parsed_siwe.chain_id,
                nonce: block.blockhash,
                issued_at: siwe_timestamp,
                expiration_time: parsed_siwe.expiration_time,
                not_before: parsed_siwe.not_before,
                request_id: parsed_siwe.request_id,
                resources: parsed_siwe.resources,
            };

            // Construct the payload to sign.
            to_sign = match siwe_to_sign.eip191_hash() {
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
            debug!(
                "Signing session key {} with PKP {}",
                json_sign_session_key_request.session_key, pkp_public_key
            );
            let request_id = tracing.correlation_id().clone();

            let before = std::time::Instant::now();
            let signing_state = match tss_state
                .get_signing_state(crate::tss::common::curve_type::CurveType::K256)
            {
                Ok(s) => s,
                Err(e) => {
                    return e.handle();
                }
            };
            timing.insert("get signing state".to_string(), before.elapsed());

            let before = std::time::Instant::now();
            let result = match pkp::utils::sign_ecdsa(
                cfg.as_ref(),
                to_sign.as_ref(),
                hex_pubkey,
                request_id,
                derived_ipfs_id,
                auth_sig.clone(),
                auth_context,
                Some(tss_state.as_ref().clone()),
                &[AuthMethodScope::SignPersonalMessage as usize],
                epoch,
                &bls_root_pubkey,
            )
            .await
            .map_err(|e| unexpected_err(e, Some("Error while signing with pkp".into())))
            {
                Ok(res) => res,
                Err(e) => {
                    return e.handle();
                }
            };
            timing.insert("signing".to_string(), before.elapsed());

            timing.insert("total".to_string(), request_start.elapsed());

            debug!("POST /web/sign_session_key timing: {:?}", timing);

            status::Custom(
                Status::Ok,
                json!(JsonSignSessionKeyResponse {
                    success: true,
                    signed_data: JsonSignSessionKeyResponseInnerSignedData {
                        session_sig: JsonSignSessionKeyResponseInnerSigShare {
                            sig_type: json_sign_session_key_request.curve_type.to_string(),
                            data_signed: encoding::bytes_to_hex(to_sign),
                            signature_share: result.0.signature_share,
                            share_index: result.0.share_index as u32,
                            bigr: result.0.big_r,
                            public_key: encoding::bytes_to_hex(pkp_public_key).replace("0x", ""),
                            sig_name: "sessionSig".to_string(),
                            siwe_message: siwe_to_sign.to_string(),
                        }
                    }
                }),
            )
        }
        CurveType::BLS => {
            // sign the session key via BLS using the lit network key
            // Add auth_context to resources
            let mut notabene = BTreeMap::new();
            notabene.insert(
                LIT_RESOURCE_KEY_RAC.to_string(),
                match serde_json::to_value(&auth_context).map_err(|e| {
                    unexpected_err(
                        e,
                        Some("Error while inserting auth_context into siwe resource".into()),
                    )
                }) {
                    Ok(res) => res,
                    Err(e) => {
                        return e.handle();
                    }
                },
            );
            let mut capabilities = Capability::<Value>::default();
            let resource = "Auth/Auth".to_string();
            let resource_prefix = format!("{}://*", LIT_RESOURCE_PREFIX_RAC); // TODO: Scope with uri
            let capabilities = match capabilities
                .with_actions_convert(resource_prefix, [(resource, [notabene])])
                .map_err(|e| {
                    unexpected_err(
                        e,
                        Some("Error while converting capabilities resource into actions".into()),
                    )
                }) {
                Ok(res) => res,
                Err(e) => {
                    return e.handle();
                }
            };

            let user_capabilities = match Capability::<Value>::extract_and_verify(&parsed_siwe)
                .map_err(|err| {
                    parser_err_code(
                        err,
                        EC::NodeSIWECapabilityInvalid,
                        Some("Unable to extract and verify user's capability object".into()),
                    )
                }) {
                Ok(res) => match res {
                    Some(res) => res,
                    None => {
                        return conversion_err(
                            "Unable to convert SIWE into Capability object",
                            None,
                        )
                        .add_msg_to_details()
                        .handle()
                    }
                },
                Err(e) => {
                    return e.handle();
                }
            };

            for ability in user_capabilities.abilities().iter() {
                if ability.0.scheme_str() == LIT_RESOURCE_PREFIX_RAC {
                    return validation_err_code(
                        "Can't define Auth Context resources in capability",
                        EC::NodeInvalidAuthContextResource,
                        None,
                    )
                    .add_msg_to_details()
                    .handle();
                }
            }

            let merged_capabilities: Capability<Value> =
                capabilities.clone().merge(user_capabilities);

            // Construct a new SIWE message with the PKP address populated.
            siwe_to_sign = match merged_capabilities
                .build_message(siwe::Message {
                    domain: parsed_siwe.domain,
                    address: pkp_eth_address,
                    statement: parsed_siwe.statement,
                    uri: parsed_siwe.uri,
                    version: parsed_siwe.version,
                    chain_id: parsed_siwe.chain_id,
                    nonce: block.blockhash,
                    issued_at: siwe_timestamp,
                    expiration_time: parsed_siwe.expiration_time,
                    not_before: parsed_siwe.not_before,
                    request_id: parsed_siwe.request_id,
                    resources: vec![],
                })
                .map_err(|e| {
                    unexpected_err(
                        e,
                        Some("Error while building Siwe message from capabilities".into()),
                    )
                }) {
                Ok(res) => res,
                Err(e) => {
                    return e.handle();
                }
            };

            // Construct the payload to sign.
            to_sign = match siwe_to_sign.eip191_hash() {
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

            debug!(
                "Signing session key using BLS {} with PKP {}",
                json_sign_session_key_request.session_key, pkp_public_key
            );
            let request_id = tracing.correlation_id().clone();

            let before = std::time::Instant::now();
            let bls_root_pubkey = match get_bls_root_pubkey(tss_state).await {
                Ok(bls_root_pubkey) => bls_root_pubkey,
                Err(e) => {
                    return e.handle();
                }
            };
            timing.insert("get bls root pubkey".to_string(), before.elapsed());

            let before = std::time::Instant::now();
            // check permissions
            let is_authed = match pkp::auth::check_pkp_auth(
                derived_ipfs_id,
                auth_sig.clone(),
                hex_pubkey.clone(),
                auth_context.clone(),
                &cfg,
                &[2],
                &bls_root_pubkey,
            )
            .await
            {
                Ok(is_authed) => is_authed,
                Err(e) => {
                    return e.handle();
                }
            };

            if !is_authed {
                return validation_err_code(
                    format!(
                        "You are not authorized to sign using this PKP: {}",
                        hex_pubkey
                    ),
                    EC::NodePKPNotAuthorized,
                    None,
                )
                .handle();
            }
            timing.insert("check pkp auth".to_string(), before.elapsed());

            let before = std::time::Instant::now();
            let cipher_state = match tss_state.get_cipher_state(CurveType::BLS) {
                Ok(cipher_state) => cipher_state,
                Err(e) => {
                    return e.handle();
                }
            };
            timing.insert("get cipher state".to_string(), before.elapsed());

            let to_sign = siwe_hash_to_bls_session_hash(to_sign.into());

            // sign the session key with BLS
            let before = std::time::Instant::now();
            let (signature_share, share_index) =
                match cipher_state.sign(to_sign.clone(), epoch).await {
                    Ok(signature_share) => signature_share,
                    Err(e) => {
                        return e.add_detail("Error signing with BLS key").handle();
                    }
                };

            timing.insert("signing".to_string(), before.elapsed());

            let bls_root_keys = cipher_state.root_keys().await;

            timing.insert("total".to_string(), request_start.elapsed());
            debug!("POST /web/sign_session_key timing: {:?}", timing);

            // return the signature share
            status::Custom(
                Status::Ok,
                json!(models::JsonSignSessionKeyResponseV1 {
                    result: "success".to_string(),
                    signature_share,
                    share_index,
                    curve_type: json_sign_session_key_request.curve_type.to_string(),
                    siwe_message: siwe_to_sign.to_string(),
                    data_signed: encoding::bytes_to_hex(to_sign),
                    bls_root_pubkey,
                }),
            )
        }
        _ => validation_err_code(
            format!(
                "Invalid curve_type: {}",
                json_sign_session_key_request.curve_type
            ),
            EC::NodeInvalidCurveType,
            None,
        )
        .handle(),
    }
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
pub async fn check_multiple_access_control_conditions(
    auth_sig_item: &AuthSigItem,
    access_control_conditions: &Option<Vec<AccessControlConditionItem>>,
    evm_contract_conditions: &Option<Vec<EVMContractConditionItem>>,
    sol_rpc_conditions: &Option<Vec<SolRpcConditionItem>>,
    unified_access_control_conditions: &Option<Vec<UnifiedAccessControlConditionItem>>,
    cfg: Arc<LitConfig>,
    requested_lit_resource_ability: &LitResourceAbility,
    chain: &Option<String>,
    request_id: String,
    bls_root_pubkey: &String,
    endpoint_version: &EndpointVersion,
    current_action_ipfs_id: Option<&String>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
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
            bls_root_pubkey,
            endpoint_version,
            current_action_ipfs_id,
            moka::future::Cache::clone(ipfs_cache),
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
            bls_root_pubkey,
            endpoint_version,
            current_action_ipfs_id,
        )
        .await?;

        return Ok(models::UnifiedConditionCheckResult {
            result,
            successful_auth_sig: auth_sig.clone(),
        });
    } else if let Some(sol_rpc_conditions) = &sol_rpc_conditions {
        let auth_sig = access_control::get_solana_auth_sig(auth_sig_item)?;

        let result = access_control::sol_rpc::check_access_control_conditions(
            sol_rpc_conditions,
            auth_sig,
            bls_root_pubkey,
            current_action_ipfs_id,
        )
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
            bls_root_pubkey,
            endpoint_version,
            current_action_ipfs_id,
            moka::future::Cache::clone(ipfs_cache),
        )
        .await;
    }

    Err(validation_err_code("Missing access control conditions", EC::NodeMissingAccessControlConditions, None)
        .add_detail("You must pass either access_control_conditions or evm_contract_conditions or sol_rpc_conditions or unified_access_control_conditions"))
}
