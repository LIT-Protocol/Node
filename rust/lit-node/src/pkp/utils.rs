use crate::{
    auth::auth_material::JsonAuthSig,
    error::validation_err_code,
    models::AuthContext,
    p2p_comms::web::models::SignedMessageShare,
    pkp::auth::verify_auth_method_for_claim,
    tss::common::{curve_type::CurveType, storage::any_key_share_exists, tss_state::TssState},
    utils::{
        contract::decode_revert,
        encoding::{self, ipfs_cid_to_bytes, string_to_eth_address, string_to_u256},
    },
};

#[cfg(feature = "rtmetrics")]
use crate::tasks::{
    realtime_metrics::MetricsMessage::NewAction,
    realtime_metrics::{MetricAction, MetricActionType},
    utils::generate_hash,
};

use ethers::{prelude::*, utils::keccak256};
use lit_blockchain::{contracts::pubkey_router::RootKey, resolver::contract::ContractResolver};
use lit_core::{config::LitConfig, error::Unexpected};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::instrument;
extern crate alloc;
use crate::{
    error::{
        conversion_err, unexpected_err, unexpected_err_code, Result, EC, EC::NodePKPNotAuthorized,
        EC::NodeUnknownError,
    },
    models::{JsonPKPClaimKeyRequest, JsonPKPClaimKeyResponse},
};

use ethers::{signers::Signer, types::U256};
use lit_blockchain::contracts::load_wallet;

use super::auth::serialize_auth_context_for_checking_against_contract_data;

pub async fn pkp_permissions_is_permitted(
    token_id_str: String,
    cfg: &LitConfig,
    method: String,
    params: Vec<Value>,
) -> Result<bool> {
    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;
    let contract = resolver.pkp_permissions_contract(cfg).await?;

    let token_id = match string_to_u256(token_id_str) {
        Ok(token_id) => token_id,
        Err(e) => {
            let msg = "Could not convert token id to u256";
            error!("{}", msg);
            return Err(unexpected_err_code(
                e,
                NodeUnknownError,
                Some(msg.to_owned()),
            ));
        }
    };
    let res;

    if method == "isPermittedAction" {
        let param_str = match params[0].as_str() {
            Some(param_str) => param_str,
            None => {
                let msg = "ipfs_id is not a string";
                error!("{}", msg);
                return Err(unexpected_err_code(msg, NodeUnknownError, None));
            }
        };
        let ipfs_id = match ipfs_cid_to_bytes(param_str.to_string()) {
            Ok(ipfs_id) => ipfs_id,
            Err(e) => {
                let msg = "Could not convert ipfs id to bytes";
                error!("{}", msg);
                return Err(unexpected_err_code(
                    e,
                    NodeUnknownError,
                    Some(msg.to_owned()),
                ));
            }
        };

        res = contract
            .is_permitted_action(token_id, Bytes::from(ipfs_id.to_vec()))
            .call()
            .await;
    } else if method == "isPermittedAddress" {
        let param_str = match params[0].as_str() {
            Some(param_str) => param_str,
            None => {
                let msg = "address is not a string";
                error!("{}", msg);
                return Err(unexpected_err_code(msg, NodeUnknownError, None));
            }
        };
        let address = match string_to_eth_address(param_str) {
            Ok(address) => address,
            Err(e) => {
                let msg = "Could not convert eth address to bytes";
                error!("{}", msg);
                return Err(unexpected_err_code(
                    e,
                    NodeUnknownError,
                    Some(msg.to_owned()),
                ));
            }
        };

        res = contract
            .is_permitted_address(token_id, address)
            .call()
            .await;
    } else if method == "isPermittedAuthMethod" {
        let param_str = match params[0].as_str() {
            Some(param_str) => param_str,
            None => {
                let msg = "auth_method_type is not a string";
                error!("{}", msg);
                return Err(unexpected_err_code(msg, NodeUnknownError, None));
            }
        };
        let auth_method_type = match string_to_u256(param_str) {
            Ok(auth_method_type) => auth_method_type,
            Err(e) => {
                let msg = "Could not convert auth_method_type to u256";
                error!("{}", msg);
                return Err(unexpected_err_code(
                    e,
                    NodeUnknownError,
                    Some(msg.to_owned()),
                ));
            }
        };
        let param_array = match params[1].as_array() {
            Some(param_array) => param_array,
            None => {
                let msg = "user_id is not an array";
                error!("{}", msg);
                return Err(unexpected_err_code(msg, NodeUnknownError, None));
            }
        };

        let mut user_id: Vec<u8> = Vec::new();
        for _user_id in param_array {
            match _user_id.as_u64() {
                Some(_user_id_u64) => user_id.push(_user_id_u64 as u8),
                None => {
                    return Err(unexpected_err_code(
                        "user_id is not an array of u8 bytes",
                        NodeUnknownError,
                        None,
                    ))
                }
            }
        }

        let user_id = Bytes::from(user_id);
        res = contract
            .is_permitted_auth_method(token_id, auth_method_type, user_id)
            .call()
            .await;
    } else {
        return Err(unexpected_err_code(
            format!("Method not found: {}", method),
            NodeUnknownError,
            None,
        ));
    }

    res.map_err(|e| {
        let msg = format!("Error calling {}: {}", method, e);
        error!("{}", msg);
        unexpected_err_code(e, NodeUnknownError, Some(msg))
    })
}

pub async fn pkp_permissions_is_permitted_auth_method(
    token_id_str: String,
    cfg: &LitConfig,
    auth_method_type_str: String,
    user_id_vec: Vec<u8>,
) -> Result<bool> {
    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;
    let contract = resolver.pkp_permissions_contract(cfg).await?;

    let token_id = match string_to_u256(token_id_str) {
        Ok(token_id) => token_id,
        Err(e) => {
            let msg = "Could not convert token id to u256";
            error!("{}", msg);
            return Err(unexpected_err_code(
                e,
                NodeUnknownError,
                Some(msg.to_owned()),
            ));
        }
    };

    let auth_method_type = match string_to_u256(auth_method_type_str) {
        Ok(auth_method_type) => auth_method_type,
        Err(e) => {
            let msg = "Could not convert auth_method_type to u256";
            error!("{}", msg);
            return Err(unexpected_err_code(
                e,
                NodeUnknownError,
                Some(msg.to_owned()),
            ));
        }
    };

    let user_id = Bytes::from(user_id_vec);
    contract
        .is_permitted_auth_method(token_id, auth_method_type, user_id)
        .call()
        .await
        .map_err(|e| {
            let msg = format!("Error calling isPermittedAuthMethod: {}", e);
            error!("{}", msg);
            unexpected_err_code(e, NodeUnknownError, Some(msg))
        })
}

pub async fn pkp_permissions_get_permitted(
    method: String,
    cfg: &LitConfig,
    token_id_str: String,
) -> Result<Vec<Value>> {
    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;
    let contract = resolver.pkp_permissions_contract(cfg).await?;

    let token_id = string_to_u256(token_id_str).map_err(|e| {
        unexpected_err_code(
            e,
            NodeUnknownError,
            Some("Could not convert token id to u256".into()),
        )
    })?;
    let ret_val;

    if method == "getPermittedAddresses" {
        let res = contract
            .get_permitted_addresses(token_id)
            .call()
            .await
            .map_err(|e| {
                unexpected_err_code(
                    e,
                    NodeUnknownError,
                    Some(format!("Error calling {}", method)),
                )
            })?;
        ret_val = res
            .iter()
            .map(|x| json!(format!("0x{}", encoding::bytes_to_hex(x.as_bytes()))))
            .collect::<Vec<Value>>();
    } else if method == "getPermittedActions" {
        let res = contract
            .get_permitted_actions(token_id)
            .call()
            .await
            .map_err(|e| {
                unexpected_err_code(
                    e,
                    NodeUnknownError,
                    Some(format!("Error calling {}", method)),
                )
            })?;
        ret_val = res
            .iter()
            .map(|x| {
                json!(encoding::bytes_to_ipfs_cid(x).expect("Could not convert bytes to ipfs cid"))
            })
            .collect::<Vec<Value>>();
    } else if method == "getPermittedAuthMethods" {
        let res = contract
            .get_permitted_auth_methods(token_id)
            .call()
            .await
            .map_err(|e| {
                unexpected_err_code(
                    e,
                    NodeUnknownError,
                    Some(format!("Error calling {}", method)),
                )
            })?;
        ret_val = res.iter().map(|x| json!(x)).collect::<Vec<Value>>();
    } else {
        return Err(unexpected_err_code(
            format!("Method not found: {}", method),
            NodeUnknownError,
            None,
        ));
    }

    Ok(ret_val)
}

pub async fn pkp_permissions_get_permitted_auth_method_scopes(
    token_id_str: String,
    cfg: &LitConfig,
    auth_method_type_str: String,
    id_vec: Vec<u8>,
    max_scope_id_int: u64,
) -> Result<Vec<bool>> {
    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;
    let contract = resolver.pkp_permissions_contract(cfg).await?;

    let token_id = string_to_u256(token_id_str).map_err(|e| {
        unexpected_err_code(
            e,
            NodeUnknownError,
            Some("Could not convert token id to u256".into()),
        )
    })?;

    let auth_method_type = string_to_u256(auth_method_type_str).map_err(|e| {
        unexpected_err_code(
            e,
            NodeUnknownError,
            Some("Could not convert auth_method_type to u256".into()),
        )
    })?;
    let id = Bytes::from(id_vec);
    let max_scope_id = U256::from(max_scope_id_int);

    contract
        .get_permitted_auth_method_scopes(token_id, auth_method_type, id, max_scope_id)
        .call()
        .await
        .map_err(|e| {
            let msg = format!("Error calling get_permitted_auth_method_scopes: {}", e);
            error!("{}", msg);
            unexpected_err_code(e, NodeUnknownError, Some(msg))
        })
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip(tss_state, cfg))]
pub async fn sign_ecdsa(
    cfg: &LitConfig,
    to_sign: &[u8],
    pubkey: String,
    request_id: String,
    lit_action_ipfs_id: Option<String>,
    auth_sig: Option<JsonAuthSig>,
    auth_context: AuthContext,
    tss_state: Option<TssState>,
    required_scopes: &[usize],
    epoch: Option<u64>,
    bls_root_pubkey: &String,
) -> Result<(SignedMessageShare, CurveType)> {
    // auth check
    let is_authed = crate::pkp::auth::check_pkp_auth(
        lit_action_ipfs_id,
        auth_sig.clone(),
        pubkey.clone(),
        auth_context,
        cfg,
        required_scopes,
        bls_root_pubkey,
    )
    .await?;

    if !is_authed {
        return Err(validation_err_code(
            format!(
                "Neither you nor this Lit Action are authorized to sign using this PKP: {}",
                pubkey
            ),
            NodePKPNotAuthorized,
            None,
        ));
    }

    let tweak_preimage = get_tweak_preimage_from_pubkey(cfg, &pubkey).await;
    let tss_state = tss_state.expect_or_err("tss_state not set in RustJsComms")?;

    // if this is a HD key, we need to get the root pubkeys, otherwise check the fs for the key share
    let (tweak_preimage, root_pubkeys, key_type) = match tweak_preimage {
        Ok(_) => {
            let tweak_preimage = tweak_preimage.expect_or_err("hd_key_id is None")?;
            let key_type = CurveType::K256; // maybe inspect root keys to determine key type?
            let temp_signable = tss_state.get_signing_state(key_type)?;
            let root_pub_keys = temp_signable.root_keys().await;
            (Some(tweak_preimage.to_vec()), Some(root_pub_keys), key_type)
        }
        Err(_) => {
            let staker_address = &tss_state.peer_state.hex_staker_address();

            let result = any_key_share_exists(&pubkey, staker_address).await;
            info!(
                "op_sign_ecdsa() any_key_share_exists() result: {:?}",
                &result
            );

            let (share_index, key_type) = match result {
                Ok(Some((key_type, share_index))) => (share_index, key_type),
                Err(err) => {
                    debug!("op_sign_ecdsa() any_key_share_exists() error: {:?}", &err);
                    return Err(unexpected_err_code(
                        err,
                        NodeUnknownError,
                        Some(format!(
                            "Pubkey share not found on this node PKP: {}",
                            pubkey
                        )),
                    ));
                }
                Ok(None) => {
                    debug!(
                        "op_sign_ecdsa() pubkey share not found on this node PKP: {}",
                        pubkey
                    );
                    return Err(unexpected_err_code(
                        format!("Pubkey share not found on this node PKP: {}", pubkey),
                        NodeUnknownError,
                        None,
                    ));
                }
            };

            (None, None, key_type)
        }
    };

    trace!(
        "sign_ecdsa() pubkey: {}, hd_key_id: {:?}, root_pubkeys: {:?}",
        pubkey,
        tweak_preimage,
        root_pubkeys
    );

    let mut signing_state = tss_state.get_signing_state(key_type)?;
    let public_key = encoding::hex_to_bytes(&pubkey)?;
    let request_id = request_id.into_bytes();
    let sign_start = std::time::Instant::now();

    debug!("sign start: {:?}", sign_start);

    let tx_metrics = tss_state.tx_metrics_manager.clone();
    #[cfg(feature = "rtmetrics")]
    let _ = tx_metrics
        .send_async(NewAction(MetricAction {
            type_id: MetricActionType::SignEcdsa,
            txn_id: generate_hash(&request_id),
            is_start: true,
            is_success: true,
        }))
        .await;

    let sign_result = signing_state
        .sign_with_pubkey(
            to_sign,
            public_key,
            root_pubkeys,
            tweak_preimage,
            request_id.clone(),
            epoch,
        )
        .await
        .map_err(|e| unexpected_err_code(e, NodeUnknownError, Some("ECDSA signing failed".into())));

    #[cfg(feature = "rtmetrics")]
    let _ = tx_metrics
        .send_async(NewAction(MetricAction {
            type_id: MetricActionType::SignEcdsa,
            txn_id: generate_hash(request_id),
            is_start: false,
            is_success: sign_result.is_ok(),
        }))
        .await;

    let sign_result = sign_result?;

    Ok((sign_result, key_type))
}

#[instrument(skip(cfg))]
pub async fn get_tweak_preimage_from_pubkey(cfg: &LitConfig, pubkey: &str) -> Result<[u8; 32]> {
    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;
    let contract = resolver.pub_key_router_contract(cfg).await?;

    let pubkey_bytes = encoding::hex_to_bytes(pubkey)?;
    let hashed_pubkey = keccak256(pubkey_bytes);
    let token_id = U256::from_big_endian(hashed_pubkey.as_slice());

    trace!("token_id: {}", token_id);
    let pubkey_routing_data = contract.pubkeys(token_id).call().await.map_err(|e| {
        unexpected_err_code(
            e,
            NodeUnknownError,
            Some("Could not find token id in pubkey routing contract.".to_string()),
        )
    })?;
    Ok(pubkey_routing_data.derived_key_id)
}

pub async fn vote_for_root_key(cfg: &LitConfig, root_keys: Vec<RootKey>) -> Result<bool> {
    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;

    let staking = resolver.staking_contract(cfg).await?;
    let staking_contract_address = staking.address();

    let contract = resolver.pub_key_router_contract_with_signer(cfg).await?;
    let func = contract.vote_for_root_keys(staking_contract_address, root_keys);

    let gas_estimate = match func.estimate_gas().await {
        Ok(gas_estimate) => gas_estimate,
        Err(e) => {
            let msg = format!("Gas estimate err : {:?}", decode_revert(&e, contract.abi()));
            warn!("{}", msg);
            return Err(unexpected_err_code(e, NodeUnknownError, Some(msg)));
        }
    };
    let func_with_gas = func.gas(gas_estimate * U256::from(5));

    match func_with_gas.send().await {
        Ok(_) => debug!(
            "Voted for root keys for contract {:?}.",
            staking_contract_address
        ),
        Err(e) => {
            let err_msg = format!(
                "Failed voting for root keys with error: {:?} and revert reason {}",
                e,
                decode_revert(&e, contract.abi())
            );
            warn!("{}", err_msg);
            return Err(unexpected_err_code(e, NodeUnknownError, Some(err_msg)));
        }
    };

    Ok(true)
}

pub async fn claim_key(
    config: &LitConfig,
    request: &JsonPKPClaimKeyRequest,
) -> Result<JsonPKPClaimKeyResponse> {
    let cfg = config.clone();
    // first auth the user with their given auth method, there should only be one for contextless derivation scheme
    let auth_method = request.auth_method.clone();

    // auth the user
    let resolver = ContractResolver::try_from(&cfg)
        .map_err(|e| unexpected_err(e, Some("Unable to construct resolver from config".into())))?;
    let permissions_contract = resolver.pkp_permissions_contract(&cfg).await.map_err(|e| {
        unexpected_err(
            e,
            Some("Could not resolve pkp permisssions contract".into()),
        )
    })?;
    // wrap in an arc pointer for verify
    let permissions_contract_ref = Arc::new(permissions_contract.to_owned());

    let auth_response = verify_auth_method_for_claim(
        &auth_method,
        Arc::new(cfg.clone()),
        permissions_contract_ref,
        None,
        request.credential_public_key.clone(),
    )
    .await
    .map_err(|e| unexpected_err(e, Some("Could not verify authentication method".into())))?;

    debug!(
        "Authentication method verified, auth response: {:?}",
        auth_response
    );

    let key_id = serialize_auth_context_for_checking_against_contract_data(&auth_response)
        .map_err(|e| conversion_err(e, Some("Could not generate key id from auth data".into())))?;
    debug!("auth context serialized: {:?}", &key_id);

    // return a signed txn that routes the PKP
    let wallet = load_wallet(&cfg, None).map_err(|e| unexpected_err(e, None))?;
    let signature = wallet
        .sign_message(&key_id)
        .await
        .map_err(|e| unexpected_err(e, Some("Could not sign message".into())))?;

    let signature = encoding::bytes_to_hex(signature.to_vec());
    debug!(
        "Signature created from wallet address: {}",
        wallet.address().to_string()
    );

    let key_id_hex = encoding::bytes_to_hex(key_id);
    debug!("key_id_hex: {}", key_id_hex);

    Ok(JsonPKPClaimKeyResponse {
        signature,
        derived_key_id: key_id_hex,
    })
}
