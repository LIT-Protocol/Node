use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{anyhow, Context, Result};
use deno_core::{error::custom_error, extension, op2, OpState};
use ethabi::ethereum_types::Address;
use lit_actions_grpc::proto::*;
use serde_json::json;
use tracing::instrument;

use crate::validation::*;

// Macro to simplify implementing synchronous ops over gRPC.
macro_rules! remote_op {
    ($name:ident, $state:ident, $send:expr, $match:pat => $res:expr) => {{
        let _: &mut OpState = $state;
        let (tx, rx) = {
            let tx = $state
                .borrow::<flume::Sender<tonic::Result<ExecuteJsResponse>>>()
                .clone();
            let rx = $state.borrow::<flume::Receiver<ExecuteJsRequest>>().clone();
            (tx, rx)
        };

        let op_name = stringify!($name);
        tx.send(Ok($send.into())).context(op_name)?;

        let resp = rx.recv().context(op_name)?;
        match resp.union {
            Some($match) => $res,
            Some(UnionRequest::ReportError(ErrorResponse { error })) => Err(anyhow!(error)),
            other => Err(anyhow!("unexpected response: {other:?}").context(op_name)),
        }
    }};
}

// Macro to simplify implementing asynchronous ops over gRPC.
macro_rules! remote_op_async {
    ($name:ident, $state:ident, $send:expr, $match:pat => $res:expr) => {{
        let _: Rc<RefCell<OpState>> = $state;
        let (tx, rx) = {
            let state = $state.borrow();
            let tx = state
                .borrow::<flume::Sender<tonic::Result<ExecuteJsResponse>>>()
                .clone();
            let rx = state.borrow::<flume::Receiver<ExecuteJsRequest>>().clone();
            (tx, rx)
        };

        let op_name = stringify!($name);
        tx.send_async(Ok($send.into())).await.context(op_name)?;

        // Must be blocking to preserve ops order
        let resp = rx.recv().context(op_name)?;
        match resp.union {
            Some($match) => $res,
            Some(UnionRequest::ReportError(ErrorResponse { error })) => Err(anyhow!(error)),
            other => Err(anyhow!("unexpected response: {other:?}").context(op_name)),
        }
    }};
}

#[instrument(skip_all, ret)]
#[op2(fast)]
fn op_print(state: &mut OpState, #[string] msg: &str, is_err: bool) -> Result<()> {
    use std::io::{stderr, stdout, Write};

    lazy_static::lazy_static! {
        static ref IS_ATTY_STDOUT: bool = atty::is(atty::Stream::Stdout);
        static ref IS_ATTY_STDERR: bool = atty::is(atty::Stream::Stderr);
    }

    let prepended = format!("[JSEnv] {msg}");
    if is_err && *IS_ATTY_STDERR {
        stderr()
            .write_all(prepended.as_bytes())
            .context("op_print: failed to write to stderr")?;
        stderr()
            .flush()
            .context("op_print: failed to flush stderr")?;
    } else if *IS_ATTY_STDOUT {
        stdout()
            .write_all(prepended.as_bytes())
            .context("op_print: failed to write to stdout")?;
        stdout()
            .flush()
            .context("op_print: failed to flush stdout")?;
    }

    // Ignore Deno logs enabled by WorkerLogLevel::Debug
    if msg.starts_with("DEBUG JS") {
        return Ok(());
    }

    remote_op!(op_print,
        state,
        PrintRequest { message: msg.to_string() }, // may be empty
        UnionRequest::Print(_) => Ok(())
    )
}

// Deny use of Deno.exit, which would terminate lit-actions via std::process::exit.
// Mimics Deno Deploy's behavior of patching Deno.exit like this:
//
// function exit() {
//   throw new errors.PermissionDenied(
//     "'Deno.exit' is not allowed in this context.",
//   );
// }
#[instrument(skip_all, ret)]
#[op2(fast)]
fn op_exit(_state: &mut OpState) -> Result<()> {
    Err(custom_error(
        "PermissionDenied",
        "'Deno.exit' is not allowed in this context.",
    ))
}

#[instrument(skip_all, ret)]
#[op2(fast)]
fn op_set_response(state: &mut OpState, #[string] response: String) -> Result<()> {
    remote_op!(op_set_response,
        state,
        SetResponseRequest { response }, // may be empty
        UnionRequest::SetResponse(_) => Ok(())
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
async fn op_increment_fetch_count(state: Rc<RefCell<OpState>>) -> Result<u32> {
    remote_op_async!(op_increment_fetch_count,
        state,
        IncrementFetchCountRequest {},
        UnionRequest::IncrementFetchCount(resp) => Ok(resp.fetch_count)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[serde]
async fn op_pkp_permissions_get_permitted(
    state: Rc<RefCell<OpState>>,
    #[string] method: String,
    #[string] token_id: String,
) -> Result<Vec<serde_json::Value>> {
    ensure_one_of!(
        method,
        [
            "getPermittedActions",
            "getPermittedAddresses",
            "getPermittedAuthMethods",
        ]
    );
    ensure_u256!(&token_id, "tokenId");

    remote_op_async!(op_pkp_permissions_get_permitted,
        state,
        PkpPermissionsGetPermittedRequest { method, token_id },
        UnionRequest::PkpPermissionsGetPermitted(resp) => serde_json::from_slice(&resp.resources).map_err(Into::into)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[serde]
async fn op_pkp_permissions_get_permitted_auth_method_scopes(
    state: Rc<RefCell<OpState>>,
    #[string] token_id: String,
    #[string] method: String,
    #[buffer(copy)] user_id: Vec<u8>,
    #[bigint] max_scope_id: u64,
) -> Result<Vec<bool>> {
    ensure_u256!(&token_id, "tokenId");
    ensure_u256!(&method, "authMethodType");
    ensure_not_empty!(user_id, "userId");

    remote_op_async!(op_pkp_permissions_get_permitted_auth_method_scopes,
        state,
        PkpPermissionsGetPermittedAuthMethodScopesRequest {
            token_id,
            method,
            user_id,
            max_scope_id,
        },
        UnionRequest::PkpPermissionsGetPermittedAuthMethodScopes(resp) => Ok(resp.scopes)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
async fn op_pkp_permissions_is_permitted(
    state: Rc<RefCell<OpState>>,
    #[string] method: String,
    #[string] token_id: String,
    #[serde] params: Vec<serde_json::Value>,
) -> Result<bool> {
    ensure_one_of!(method, ["isPermittedAction", "isPermittedAddress"]);
    ensure_u256!(&token_id, "tokenId");
    ensure_not_empty!(params);

    remote_op_async!(op_pkp_permissions_is_permitted,
        state,
        PkpPermissionsIsPermittedRequest {
            method,
            token_id,
            params: serde_json::to_vec(&params)?,
        },
        UnionRequest::PkpPermissionsIsPermitted(resp) => Ok(resp.is_permitted)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
async fn op_pkp_permissions_is_permitted_auth_method(
    state: Rc<RefCell<OpState>>,
    #[string] token_id: String,
    #[string] method: String,
    #[buffer(copy)] user_id: Vec<u8>,
) -> Result<bool> {
    ensure_u256!(&token_id, "tokenId");
    ensure_u256!(&method, "authMethodType");
    ensure_not_empty!(user_id, "userId");

    remote_op_async!(op_pkp_permissions_is_permitted_auth_method,
        state,
        PkpPermissionsIsPermittedAuthMethodRequest {
            token_id,
            method,
            user_id,
        },
        UnionRequest::PkpPermissionsIsPermittedAuthMethod(resp) => Ok(resp.is_permitted)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
async fn op_check_conditions(
    state: Rc<RefCell<OpState>>,
    #[serde] conditions: Vec<serde_json::Value>, // Vec<UnifiedAccessControlConditionItem>
    #[serde] auth_sig: Option<serde_json::Value>, // AuthSigItem
    #[string] chain: Option<String>,
) -> Result<bool> {
    ensure_not_empty!(conditions);
    if let Some(chain) = &chain {
        ensure_not_blank!(chain);
    }

    remote_op_async!(op_check_conditions,
        state,
        CheckConditionsRequest {
            conditions: serde_json::to_vec(&conditions)?,
            auth_sig: auth_sig.as_ref().map(serde_json::to_vec).transpose()?,
            chain,
        },
        UnionRequest::CheckConditions(resp) => Ok(resp.success)
    )
}

#[instrument(skip_all, ret)]
#[op2]
#[string]
fn op_pubkey_to_token_id(state: &mut OpState, #[string] public_key: String) -> Result<String> {
    ensure_not_blank!(public_key, "publicKey");

    remote_op!(op_pubkey_to_token_id,
        state,
        PubkeyToTokenIdRequest { public_key },
        UnionRequest::PubkeyToTokenId(resp) => Ok(resp.token_id)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_sign_ecdsa(
    state: Rc<RefCell<OpState>>,
    #[buffer(copy)] to_sign: Vec<u8>,
    #[string] public_key: String,
    #[string] sig_name: String,
) -> Result<String> {
    ensure_not_empty!(to_sign, "toSign");
    ensure_not_blank!(public_key, "publicKey");
    ensure_not_blank!(sig_name, "sigName");

    remote_op_async!(op_sign_ecdsa,
        state,
        SignEcdsaRequest {
            to_sign,
            public_key,
            sig_name,
            eth_personal_sign: false,
        },
        UnionRequest::SignEcdsa(resp) => Ok(resp.success)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_sign_ecdsa_eth_personal_sign_message(
    state: Rc<RefCell<OpState>>,
    #[buffer(copy)] to_sign: Vec<u8>,
    #[string] public_key: String,
    #[string] sig_name: String,
) -> Result<String> {
    ensure_not_empty!(to_sign, "toSign");
    ensure_not_blank!(public_key, "publicKey");
    ensure_not_blank!(sig_name, "sigName");

    remote_op_async!(op_sign_ecdsa_eth_personal_sign_message,
        state,
        SignEcdsaRequest {
            to_sign,
            public_key,
            sig_name,
            eth_personal_sign: true,
        },
        UnionRequest::SignEcdsa(resp) => Ok(resp.success)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_aes_decrypt(
    state: Rc<RefCell<OpState>>,
    #[buffer(copy)] symmetric_key: Vec<u8>,
    #[buffer(copy)] ciphertext: Vec<u8>,
) -> Result<String> {
    ensure_not_empty!(symmetric_key, "symmetricKey");
    ensure_not_empty!(ciphertext);

    remote_op_async!(op_aes_decrypt,
        state,
        AesDecryptRequest { symmetric_key, ciphertext },
        UnionRequest::AesDecrypt(resp) => Ok(resp.plaintext)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_get_latest_nonce(
    state: Rc<RefCell<OpState>>,
    #[serde] address: Address,
    #[string] chain: String,
) -> Result<String> {
    ensure_not_blank!(chain);

    remote_op_async!(op_get_latest_nonce,
        state,
        GetLatestNonceRequest {
            address: format!("{address:x}"),
            chain,
        },
        UnionRequest::GetLatestNonce(resp) => Ok(resp.nonce)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_claim_key_identifier(
    state: Rc<RefCell<OpState>>,
    #[string] key_id: String,
) -> Result<String> {
    ensure_not_blank!(key_id, "keyId");

    remote_op_async!(op_claim_key_identifier,
        state,
        ClaimKeyIdentifierRequest { key_id },
        UnionRequest::ClaimKeyIdentifier(resp) => Ok(resp.success)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_call_contract(
    state: Rc<RefCell<OpState>>,
    #[string] chain: String,
    #[string] txn: String,
) -> Result<String> {
    ensure_not_blank!(chain); // ensure_one_of not feasible as there are too many supported blockchains
    ensure_not_blank!(txn);

    remote_op_async!(op_call_contract,
        state,
        CallContractRequest { chain, txn },
        UnionRequest::CallContract(resp) => Ok(resp.result)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_call_child(
    state: Rc<RefCell<OpState>>,
    #[string] ipfs_id: String,
    #[serde] params: Option<serde_json::Value>,
) -> Result<String> {
    ensure_not_blank!(ipfs_id, "ipfsId");

    remote_op_async!(op_call_child,
        state,
        CallChildRequest {
            ipfs_id,
            params: params.as_ref().map(serde_json::to_vec).transpose()?,
        },
        UnionRequest::CallChild(resp) => Ok(resp.response)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[serde]
async fn op_broadcast_and_collect(
    state: Rc<RefCell<OpState>>,
    #[string] name: String,
    #[string] value: String,
) -> Result<Vec<String>> {
    remote_op_async!(op_broadcast_and_collect,
        state,
        BroadcastAndCollectRequest { name, value },
        UnionRequest::BroadcastAndCollect(resp) => Ok(resp.values)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_decrypt_and_combine(
    state: Rc<RefCell<OpState>>,
    #[serde] access_control_conditions: Vec<serde_json::Value>, // Vec<UnifiedAccessControlConditionItem>
    #[string] ciphertext: String,
    #[string] data_to_encrypt_hash: String,
    #[serde] auth_sig: Option<serde_json::Value>, // AuthSigItem
    #[string] chain: String,
) -> Result<String> {
    remote_op_async!(op_decrypt_and_combine,
        state,
        DecryptAndCombineRequest {
            access_control_conditions: serde_json::to_vec(&access_control_conditions)?,
            ciphertext,
            data_to_encrypt_hash,
            auth_sig: auth_sig.as_ref().map(serde_json::to_vec).transpose()?,
            chain,
        },
        UnionRequest::DecryptAndCombine(resp) => Ok(resp.result)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_sign_and_combine_ecdsa(
    state: Rc<RefCell<OpState>>,
    #[buffer(copy)] to_sign: Vec<u8>,
    #[string] public_key: String,
    #[string] sig_name: String,
) -> Result<String> {
    remote_op_async!(op_sign_and_combine_ecdsa,
        state,
        SignAndCombineEcdsaRequest { to_sign, public_key, sig_name },
        UnionRequest::SignAndCombineEcdsa(resp) => Ok(resp.result)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_get_rpc_url(state: Rc<RefCell<OpState>>, #[string] chain: String) -> Result<String> {
    remote_op_async!(op_get_rpc_url,
        state,
        GetRpcUrlRequest { chain },
        UnionRequest::GetRpcUrl(resp) => Ok(resp.result)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
async fn op_p2p_broadcast(
    state: Rc<RefCell<OpState>>,
    #[string] name: String,
    #[string] value: String,
) -> Result<bool> {
    remote_op_async!(op_p2p_broadcast,
        state,
        P2pBroadcastRequest { name, value },
        UnionRequest::P2pBroadcast(resp) => Ok(resp.result)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_p2p_collect_from_leader(
    state: Rc<RefCell<OpState>>,
    #[string] name: String,
) -> Result<String> {
    remote_op_async!(op_p2p_collect_from_leader,
        state,
        P2pCollectFromLeaderRequest { name  },
        UnionRequest::P2pCollectFromLeader(resp) => Ok(resp.value)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
async fn op_is_leader(state: Rc<RefCell<OpState>>) -> Result<bool> {
    remote_op_async!(op_is_leader,
        state,
        IsLeaderRequest { },
        UnionRequest::IsLeader(resp) => Ok(resp.result)
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[serde]
async fn op_encrypt_bls(
    state: Rc<RefCell<OpState>>,
    #[serde] access_control_conditions: Vec<serde_json::Value>, // Vec<UnifiedAccessControlConditionItem>
    #[buffer(copy)] to_encrypt: Vec<u8>,
) -> Result<serde_json::Value> {
    remote_op_async!(op_encrypt_bls,
        state,
        EncryptBlsRequest { access_control_conditions: serde_json::to_vec(&access_control_conditions)?, to_encrypt},
        UnionRequest::EncryptBls(resp) => Ok(json!({"ciphertext": resp.ciphertext, "dataToEncryptHash": resp.data_to_encrypt_hash}))
    )
}

#[instrument(skip_all, ret)]
#[op2(async, reentrant)]
#[string]
async fn op_decrypt_to_single_node(
    state: Rc<RefCell<OpState>>,
    #[serde] access_control_conditions: Vec<serde_json::Value>, // Vec<UnifiedAccessControlConditionItem>
    #[string] ciphertext: String,
    #[string] data_to_encrypt_hash: String,
    #[serde] auth_sig: Option<serde_json::Value>, // AuthSigItem
    #[string] chain: String,
) -> Result<String> {
    let auth_sig = match auth_sig {
        Some(auth_sig) => Some(serde_json::to_vec(&auth_sig)?),
        None => None,
    };
    remote_op_async!(op_decrypt_to_single_node,
        state,
        DecryptToSingleNodeRequest { access_control_conditions: serde_json::to_vec(&access_control_conditions)?, ciphertext, data_to_encrypt_hash, auth_sig, chain },
        UnionRequest::DecryptToSingleNode(resp) => Ok(resp.result)
    )
}

// Build a deno_core::Extension providing custom ops
extension!(
    lit_actions,
    deps = [runtime],
    ops = [
        op_aes_decrypt,
        op_call_child,
        op_call_contract,
        op_check_conditions,
        op_claim_key_identifier,
        op_get_latest_nonce,
        op_increment_fetch_count,
        op_pkp_permissions_get_permitted_auth_method_scopes,
        op_pkp_permissions_get_permitted,
        op_pkp_permissions_is_permitted_auth_method,
        op_pkp_permissions_is_permitted,
        op_pubkey_to_token_id,
        op_set_response,
        op_sign_ecdsa_eth_personal_sign_message,
        op_sign_ecdsa,
        op_broadcast_and_collect,
        op_decrypt_and_combine,
        op_sign_and_combine_ecdsa,
        op_get_rpc_url,
        op_p2p_broadcast,
        op_p2p_collect_from_leader,
        op_is_leader,
        op_encrypt_bls,
        op_decrypt_to_single_node,
    ],
    esm_entry_point = "ext:lit_actions/99_patches.js",
    esm = [
        dir "js",
        "00_ethers.js",
        "01_uint8arrays.js",
        "02_litActionsSDK.js",
        "03_jsonwebtoken.js",
        "99_patches.js",
    ],
    middleware = |op| match op.name {
        "op_print" => op_print(),
        "op_exit" | "op_set_exit_code" => op.with_implementation_from(&op_exit()),
        _ => op,
    },
);
