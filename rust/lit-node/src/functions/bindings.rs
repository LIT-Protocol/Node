use crate::auth::auth_material::AuthSigItem;
use crate::auth::auth_material::JsonAuthSig;
use crate::models::JsonPKPClaimKeyResponse;
use crate::utils::encoding;
// use crate::{tss::     common::traits::signabletss::SignableTss} ;
// use crate::encoding_utils::{self, ipfs_cid_to_bytes, string_to_eth_address, string_to_u256};
use crate::error::conversion_err;
use crate::models;
use crate::pkp;
// use crate::tss::common::storage::{any_key_share_exists, key_share_exists, KeyType};

use super::aes;
use crate::error::unexpected_err;
use crate::models::SignedData;
use anyhow::{bail, Result};
use deno_core::error::AnyError;
use deno_core::{op2, OpState};
use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::utils::keccak256;
use ethers::utils::rlp::Rlp;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_blockchain::resolver::rpc::get_provider;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use rocket::serde::Serialize;
use serde::Deserialize;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stderr, stdout, Write};
use std::rc::Rc;
use std::str::FromStr;
use tracing::instrument;
// pub fn transaction_count(&self, address: Address, block: Option<BlockNumber>) -> CallFuture<U256, T::Out>

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecryptedData {
    pub algorithm_type: String,
    pub ciphertext: String,
    pub decryption_share: String,
    pub share_index: u32,
    pub public_key: String,
    pub decryption_name: String,
}

#[derive(Debug, Default)]
pub struct RustJsComms {
    pub claim_data: HashMap<String, models::JsonPKPClaimKeyResponse>,
    pub node_address: String,
    pub node_port: String,
    pub signed_data: HashMap<String, SignedData>,
    pub decrypted_data: HashMap<String, DecryptedData>,
    pub deno_execution_env: models::DenoExecutionEnv,
    pub claim_count: u32,
    pub fetch_count: u32,
    pub contract_call_count: u32,
    pub sign_count: u32,
    pub lit_action_ipfs_id: Option<String>,
    pub auth_sig: JsonAuthSig,
    pub response: String,
    pub logs: String,
    pub error: Option<String>,
    pub auth_context: models::AuthContext,
    pub request_id: String,
}

#[instrument(name = "op_pkp_permissions_is_permitted", skip_all, ret)]
#[op2(async)]
pub async fn op_pkp_permissions_is_permitted(
    state: Rc<RefCell<OpState>>,
    #[string] method: String,
    #[string] token_id_str: String,
    #[serde] params: Vec<serde_json::Value>,
) -> Result<bool, AnyError> {
    let cfg = {
        let state = state.borrow();
        let rust_js_comms = state.borrow::<RustJsComms>();
        rust_js_comms.deno_execution_env.cfg.clone()
    };

    return pkp::utils::pkp_permissions_is_permitted(token_id_str, cfg.as_ref(), method, params)
        .await
        .map_err(|e| anyhow::anyhow!(e));
}

#[instrument(name = "op_pkp_permissions_is_permitted_auth_method", skip_all, ret)]
#[op2(async)]
pub async fn op_pkp_permissions_is_permitted_auth_method(
    state: Rc<RefCell<OpState>>,
    #[string] token_id_str: String,
    #[string] auth_method_type_str: String,
    #[buffer(copy)] user_id_vec: Vec<u8>,
) -> Result<bool, AnyError> {
    let cfg = {
        let state = state.borrow();
        let rust_js_comms = state.borrow::<RustJsComms>();
        rust_js_comms.deno_execution_env.cfg.clone()
    };

    let resolver =
        ContractResolver::try_from(cfg.as_ref()).expect("failed to load ContractResolver");
    let contract = resolver.pkp_permissions_contract(&cfg).await?;

    return pkp::utils::pkp_permissions_is_permitted_auth_method(
        token_id_str,
        cfg.as_ref(),
        auth_method_type_str,
        user_id_vec,
    )
    .await
    .map_err(|e| anyhow::anyhow!(e));
}

#[instrument(name = "op_pkp_permissions_get_permitted", skip_all, ret)]
#[op2(async)]
#[serde]
pub async fn op_pkp_permissions_get_permitted(
    state: Rc<RefCell<OpState>>,
    #[string] method: String,
    #[string] token_id_str: String,
) -> Result<Vec<Value>, AnyError> {
    let cfg = {
        let state = state.borrow();
        let rust_js_comms = state.borrow::<RustJsComms>();
        rust_js_comms.deno_execution_env.cfg.clone()
    };

    return pkp::utils::pkp_permissions_get_permitted(method, cfg.as_ref(), token_id_str)
        .await
        .map_err(|e| anyhow::anyhow!(e));
}

#[instrument(
    name = "op_pkp_permissions_get_permitted_auth_method_scopes",
    skip_all,
    ret
)]
#[op2(async)]
#[serde]
pub async fn op_pkp_permissions_get_permitted_auth_method_scopes(
    state: Rc<RefCell<OpState>>,
    #[string] token_id_str: String,
    #[string] auth_method_type_str: String,
    #[buffer(copy)] id_vec: Vec<u8>,
    #[bigint] max_scope_id_int: u64,
) -> Result<Vec<bool>, AnyError> {
    let cfg = {
        let state = state.borrow();
        let rust_js_comms = state.borrow::<RustJsComms>();
        rust_js_comms.deno_execution_env.cfg.clone()
    };

    return pkp::utils::pkp_permissions_get_permitted_auth_method_scopes(
        token_id_str,
        cfg.as_ref(),
        auth_method_type_str,
        id_vec,
        max_scope_id_int,
    )
    .await
    .map_err(|e| anyhow::anyhow!(e));
}

#[op2(async)]
#[string]
pub async fn op_aes_decrypt(
    #[buffer(copy)] symmetric_key: Vec<u8>,
    #[buffer(copy)] ciphertext_with_iv: Vec<u8>,
) -> Result<String> {
    aes::aes_decrypt(symmetric_key, ciphertext_with_iv)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

#[instrument(name = "op_get_latest_nonce", skip_all, ret)]
#[op2(async)]
#[serde]
pub async fn op_get_latest_nonce(
    #[string] address: String,
    #[string] chain: String,
) -> Result<ethers::types::U256, AnyError> {
    let provider = get_provider(&chain, 0)?;
    let addr = ethers::types::Address::from_str(&address).map_err(|e| conversion_err(e, None))?;
    let latest_nonce = provider.get_transaction_count(addr, None).await?;
    debug!(
        "op_get_latest_nonce; addr: {}, latest_nonce: {}",
        addr, latest_nonce
    );
    return Ok(latest_nonce);
}

#[instrument(name = "op_sign_ecdsa_eth_personal_sign_message", skip_all, ret)]
#[op2(async)]
#[string]
pub async fn op_sign_ecdsa_eth_personal_sign_message(
    state: Rc<RefCell<OpState>>,
    #[buffer(copy)] to_sign: Vec<u8>,
    #[string] pubkey: String,
    #[string] sig_name: String,
) -> Result<String> {
    // Prepend the Ethereum Signed Message according to EIP-191
    let mut message = format!("\x19Ethereum Signed Message:\n{}", to_sign.len()).into_bytes();
    message.extend(&to_sign);

    // Hash it using keccak256
    let hashed_message = keccak256(message);

    // 2 is the AuthMethodScope::SignPersonalMessage scope
    sign_ecdsa_helper(state, hashed_message.into(), pubkey, sig_name, &[2]).await
}

#[instrument(name = "op_sign_ecdsa", skip_all, ret)]
#[op2(async)]
#[string]
pub async fn op_sign_ecdsa(
    state: Rc<RefCell<OpState>>,
    #[buffer(copy)] to_sign: Vec<u8>,
    #[string] pubkey: String,
    #[string] sig_name: String,
) -> Result<String> {
    // 1 is the AuthMethodScope::SignAnything scope
    sign_ecdsa_helper(state, to_sign, pubkey, sig_name, &[1]).await
}

#[instrument(name = "op_increment_fetch_count", skip_all, ret)]
#[op2(async)]
pub async fn op_increment_fetch_count(state: Rc<RefCell<OpState>>) -> Result<u32, AnyError> {
    let fetch_count = {
        let mut state = state.borrow_mut();
        let rust_js_comms = state.borrow_mut::<RustJsComms>();
        rust_js_comms.fetch_count += 1;
        rust_js_comms.fetch_count
    };

    if fetch_count > crate::functions::MAX_FETCH_COUNT {
        bail!("You may not send more than {} HTTP requests per session and you have attempted to exceed that limit.", crate::functions::MAX_FETCH_COUNT);
    }

    Ok(fetch_count)
}

#[instrument(name = "op_call_contract", skip_all, ret)]
#[op2(async)]
#[string]
pub async fn op_call_contract(
    state: Rc<RefCell<OpState>>,
    #[string] chain: String,
    #[string] txn: String,
) -> Result<String> {
    let call_count = {
        let mut state = state.borrow_mut();
        let rust_js_comms = state.borrow_mut::<RustJsComms>();
        rust_js_comms.contract_call_count += 1;
        rust_js_comms.contract_call_count
    };

    if call_count > crate::functions::MAX_CONTRACT_CALL_COUNT {
        bail!("You may invoke contract calls more than {} times per session and you have attempted to exceed that limit.", crate::functions::MAX_CONTRACT_CALL_COUNT);
    }

    // FIXME: ideally we should try each index until we find one that works.  0 is hardcoded for now.
    let provider: Provider<Http> = get_provider(chain.as_str(), 0)?;
    let txn_bytes = encoding::hex_to_bytes(&txn)?;
    let rlp = Rlp::new(&txn_bytes);
    let mut decoded_txn = TransactionRequest::decode_unsigned_rlp(&rlp)?;

    // set gas limit if none is passed, otherwise the txn call will fail
    if decoded_txn.gas.is_none()
        || decoded_txn.gas.expect("Could not pull out gas from struct") == U256::zero()
    {
        // set 10 million gas limit.  chain gas limit is 30m on ethereum but it used to be 10m.
        decoded_txn = decoded_txn.gas(ethers::types::U256::from(10000000));
    }

    let typed_txn: TypedTransaction = decoded_txn.into();
    let result = provider.call_raw(&typed_txn).await;
    let result = match result {
        Ok(r) => r,
        Err(e) => {
            error!("Error calling contract: {:?}", e);
            return Err(e.into());
        }
    };

    Ok(format!("0x{}", bytes_to_hex(result)))
}

#[instrument(name = "op_check_conditions", skip_all, ret)]
#[op2(async)]
pub async fn op_check_conditions(
    state: Rc<RefCell<OpState>>,
    #[serde] conditions: Vec<models::UnifiedAccessControlConditionItem>,
    #[serde] auth_sig: AuthSigItem,
    #[string] chain: Option<String>,
) -> Result<bool> {
    let (cfg, request_id) = {
        let state = state.borrow();
        let rust_js_comms = state.borrow::<RustJsComms>();
        (
            rust_js_comms.deno_execution_env.cfg.clone(),
            rust_js_comms.request_id.clone(),
        )
    };

    let lit_action_resource = crate::auth::resources::LitActionResource::new("".to_string());
    let res = crate::access_control::unified::check_access_control_conditions(
        &conditions,
        &auth_sig,
        chain,
        &lit_action_resource.execution_ability(),
        cfg,
        &request_id.clone(),
    )
    .await
    .map_err(|e| anyhow::anyhow!(format!("Error checking access control conditions: {:?}", e)))?;

    Ok(res.result)
}

#[instrument(name = "op_pubkey_to_token_id", skip_all, ret)]
#[op2]
#[string]
pub fn op_pubkey_to_token_id(#[string] pubkey: &str) -> Result<String, AnyError> {
    let bytes = encoding::hex_to_bytes(pubkey)?;
    let token_id = format!("0x{}", encoding::bytes_to_hex(keccak256(bytes).as_slice()));
    Ok(token_id)
}

#[instrument(name = "op_set_response", skip_all, ret)]
#[op2(fast)]
pub fn op_set_response(state: &mut OpState, #[string] resp: String) -> Result<()> {
    debug!("Setting response");
    // TODO dynamic limit depending on if the user is paying or not
    if resp.len() > crate::functions::MAX_RESPONSE_LENGTH {
        return Err(anyhow::anyhow!(format!(
            "Response is too long. Max length is {} bytes",
            crate::functions::MAX_RESPONSE_LENGTH
        )));
    }

    let rust_js_comms = state.try_borrow_mut::<RustJsComms>()
        .expect_or_err("Could not borrow state.  You probably forgot to await on a Lit function that was called before this one.")?;
    rust_js_comms.response = resp;

    Ok(())
}

/// override builtin utility to print to stdout/stderr
#[instrument(name = "op_print", skip_all, ret)]
#[op2(fast)]
pub fn op_print(state: &mut OpState, #[string] msg: &str, is_err: bool) -> Result<(), AnyError> {
    let rust_js_comms = state.borrow_mut::<RustJsComms>();

    // TODO dynamic limit depending on if the user is paying or not
    if rust_js_comms.logs.len() + msg.len() > crate::functions::MAX_CONSOLE_LOG_LENGTH {
        return Err(anyhow::anyhow!(format!(
            "Console.log is printing something that is too long. Max length for all logs in a single request is {} bytes",
            crate::functions::MAX_CONSOLE_LOG_LENGTH
        )));
    }
    let prepended = format!("[JSEnv] {}", msg);
    if is_err {
        stderr()
            .write_all(prepended.as_bytes())
            .map_err(|e| unexpected_err(e, Some("Failed to write to stderr".into())))?;
        stderr()
            .flush()
            .map_err(|e| unexpected_err(e, Some("Failed to flush stderr".into())))?;
    } else {
        stdout()
            .write_all(prepended.as_bytes())
            .map_err(|e| unexpected_err(e, Some("Failed to write to stdout".into())))?;
        stdout()
            .flush()
            .map_err(|e| unexpected_err(e, Some("Failed to flush stdout".into())))?;
    }

    rust_js_comms.logs.push_str(msg);

    Ok(())
}

#[instrument(name = "op_call_child", skip_all, ret)]
#[op2(async)]
#[string]
pub async fn op_call_child(
    state: Rc<RefCell<OpState>>,
    #[string] ipfs_id: String,
    #[serde] params: serde_json::Value,
) -> Result<String, AnyError> {
    let (deno_execution_env, auth_sig, request_id) = {
        let state = state.borrow();
        let rust_js_comms = state.borrow::<RustJsComms>();
        (
            rust_js_comms.deno_execution_env.clone(),
            rust_js_comms.auth_sig.clone(),
            rust_js_comms.request_id.clone(),
        )
    };

    // pull down the lit action from IPFS and run it
    let code_to_run = crate::utils::web::get_ipfs_file(&ipfs_id, &deno_execution_env.cfg).await?;

    // run the lit action.  this will spawn another runtime
    let rust_js_comms_from_child = crate::functions::execute_js(
        code_to_run,
        Some(ipfs_id),
        auth_sig,
        Some(params),
        deno_execution_env,
        request_id,
        None,
        None,
    )
    .await?;

    {
        let mut state = state.borrow_mut();
        let rust_js_comms = state.borrow_mut::<RustJsComms>();

        // now take the stuff in rust_js_comms_from_child and merge it into rust_js_comms
        rust_js_comms
            .signed_data
            .extend(rust_js_comms_from_child.signed_data);
        rust_js_comms
            .decrypted_data
            .extend(rust_js_comms_from_child.decrypted_data);
        rust_js_comms.fetch_count += rust_js_comms_from_child.fetch_count;
        rust_js_comms.logs += &rust_js_comms_from_child.logs;

        Ok(rust_js_comms_from_child.response)
    }
}

#[instrument(name = "op_claim_key_identifier", skip_all, ret)]
#[op2(async)]
#[string]
pub async fn op_claim_key_identifier(
    state: Rc<RefCell<OpState>>,
    #[string] key_id: String,
) -> Result<String, AnyError> {
    let (ipfs_id, cfg) = {
        let mut state = state.borrow_mut();
        let rust_js_comms = state.borrow_mut::<RustJsComms>();
        rust_js_comms.claim_count += 1;

        let ipfs_id = rust_js_comms.lit_action_ipfs_id.clone();
        let cfg = rust_js_comms.deno_execution_env.cfg.as_ref();
        (ipfs_id, cfg.clone())
    };

    let ipfs_id = match ipfs_id {
        Some(id) => id,
        None => {
            bail!("Could not find IPFS ID for this action, aborting claim operation");
        }
    };

    let serialized = format!("{}_{}", ipfs_id.clone(), key_id.clone());
    let as_bytes = serialized.as_bytes().to_vec();
    let formatted_key_id = keccak256(as_bytes).to_vec();
    let wallet =
        lit_blockchain::contracts::load_wallet(&cfg, None).map_err(|e| unexpected_err(e, None))?;
    let signature = wallet
        .sign_message(&formatted_key_id)
        .await
        .map_err(|e| unexpected_err(e, Some("Could not sign message".into())))?;

    let signature = encoding::bytes_to_hex(signature.to_vec());
    let key_id_hex = encoding::bytes_to_hex(formatted_key_id.clone());

    let mut state = state.borrow_mut();
    let rust_js_comms = state.borrow_mut::<RustJsComms>();
    rust_js_comms.claim_data.insert(
        key_id,
        JsonPKPClaimKeyResponse {
            signature,
            derived_key_id: key_id_hex,
        },
    );
    Ok("success".to_string())
}
async fn sign_ecdsa_helper(
    state: Rc<RefCell<OpState>>,
    to_sign: Vec<u8>,
    pubkey: String,
    sig_name: String,
    required_scopes: &[usize],
) -> Result<String> {
    let sign_count = {
        let mut state = state.borrow_mut();
        let rust_js_comms = state.borrow_mut::<RustJsComms>();
        rust_js_comms.sign_count += 1;
        rust_js_comms.sign_count
    };

    if sign_count > crate::functions::MAX_SIGN_COUNT {
        return Err(anyhow::anyhow!(
        format!(
            "You may not sign more than {} times per session and you have attempted to exceed that limit.",
            crate::functions::MAX_SIGN_COUNT
        )));
    }

    debug!(
        "op_sign_ecdsa() called with to_sign: {:?}, pubkey: {}, sig_name: {}",
        encoding::bytes_to_hex(to_sign.clone()),
        pubkey,
        sig_name
    );

    // accept pubkey with and without 0x prefix
    let pubkey = pubkey.replace("0x", "");

    let (deno_execution_env, lit_action_ipfs_id, auth_sig, request_id) = {
        let state = state.borrow();
        let rust_js_comms = state.borrow::<RustJsComms>();
        (
            rust_js_comms.deno_execution_env.clone(),
            rust_js_comms.lit_action_ipfs_id.clone(),
            rust_js_comms.auth_sig.clone(),
            rust_js_comms.request_id.clone(),
        )
    };

    let (result, key_type) = crate::pkp::utils::sign_ecdsa(
        &deno_execution_env.cfg,
        &to_sign,
        pubkey,
        request_id,
        lit_action_ipfs_id,
        Some(auth_sig),
        deno_execution_env.auth_context,
        deno_execution_env.tss_state,
        required_scopes,
    )
    .await
    .map_err(|e| anyhow::anyhow!(format!("Failed to sign ecdsa: {:?}", e)))?;

    debug!("ECDSA signing complete");

    // pad the pubkey with a zero at the front if it's odd because hex strings should be even and zero padded
    let mut padded_pubkey = result.public_key;
    if padded_pubkey.len() % 2 == 1 {
        padded_pubkey = format!("0{}", &padded_pubkey);
    }

    {
        let mut state = state.borrow_mut();
        let rust_js_comms_mut = state.borrow_mut::<RustJsComms>();

        // this state is persisted across calls by deno, and so we can use it to
        // return data to the client that called this Lit function via HTTP
        rust_js_comms_mut.signed_data.insert(
            sig_name.to_string(),
            SignedData {
                sig_type: key_type.to_string(),
                // data_signed: crate::encoding::bytes_to_hex(&to_sign),
                data_signed: result.digest,
                signature_share: result.signature_share,
                share_index: result.share_index as u32,
                big_r: result.big_r,
                public_key: padded_pubkey,
                sig_name,
            },
        );

        debug!(
            "op_sign_ecdsa() returning: {:?}",
            rust_js_comms_mut.signed_data
        );
    }

    Ok("success".to_string())
}
