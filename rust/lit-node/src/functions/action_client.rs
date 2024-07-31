//! This module implements the gRPC client for the Lit Actions server (lit_actions).
//! The client initiates JS code execution and handles op requests from the server.
//! It holds all configuration data (including secrets) and manages state; none of
//! which are shared with lit_actions, enabling a secure execution environment.

use std::borrow::BorrowMut;
use std::collections::{BTreeMap, HashMap};
use std::error::Error as _;
use std::path::PathBuf;

use anyhow::{bail, Context as _, Result};
use base64_light::base64_decode;
use blsful::{Bls12381G2Impl, SignatureShare};
use derive_builder::Builder;
use ethers::utils::keccak256;
use lit_actions_grpc::tokio_stream::StreamExt as _;
use lit_actions_grpc::tonic::{
    metadata::MetadataMap, transport::Error as TransportError, Code, Extensions, Request, Status,
};
use lit_actions_grpc::{proto::*, unix};
use lit_blockchain::resolver::rpc::{RpcHealthcheckPoller, ENDPOINT_MANAGER};
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use tracing::{debug, instrument};

use crate::access_control::rpc_url;
use crate::auth::auth_material::{AuthSigItem, JsonAuthSig};
use crate::auth::lit_resource::LitResource;
use crate::auth::resources::AccessControlConditionResource;
use crate::config::LitNodeConfig as _;
use crate::error::{connect_err, conversion_err, memory_limit_err, timeout_err, unexpected_err};
use crate::models::{self, RequestConditions, UnifiedConditionCheckResult};
use crate::p2p_comms::CommsManager;
use crate::peers::peer_state::models::SimplePeerExt;
use crate::pkp;
use crate::tasks::beaver_manager::listener::leader_addr;
use crate::tasks::beaver_manager::models::generate_hash;
use crate::tss::dkg::curves::common::CurveType;
use crate::utils::encoding::{self, BeBytes, BeHex, CompressedPointHex, UncompressedPointHex};
use crate::utils::web::{get_bls_root_pubkey, hash_access_control_conditions, EndpointVersion};

const DEFAULT_TIMEOUT_MS: u64 = 30000; // 30s
const DEFAULT_MEMORY_LIMIT_MB: u32 = 256; // 256MB

const DEFAULT_MAX_CONSOLE_LOG_LENGTH: usize = 1024 * 100; // 100KB
const DEFAULT_MAX_CONTRACT_CALL_COUNT: u32 = 30;
const DEFAULT_MAX_FETCH_COUNT: u32 = 50;
const DEFAULT_MAX_RESPONSE_LENGTH: usize = 1024 * 100; // 100KB
const DEFAULT_MAX_SIGN_COUNT: u32 = 10; // 10 signature requests per action execution
const DEFAULT_MAX_BROADCAST_AND_COLLECT_COUNT: u32 = 30;
const DEFAULT_MAX_CALL_DEPTH: u32 = 5;

#[derive(Debug, Default, Builder)]
pub struct Client {
    // Config
    #[builder(default, setter(into, strip_option))]
    socket_path: Option<PathBuf>,
    #[builder(default, setter(into))]
    js_env: models::DenoExecutionEnv,
    #[builder(default, setter(into))]
    auth_sig: Option<JsonAuthSig>,
    #[builder(default, setter(into))]
    request_id: Option<String>,
    #[builder(default, setter(into))]
    http_headers: BTreeMap<String, String>,
    #[builder(default, setter(into))]
    epoch: Option<u64>,
    #[builder(default, setter(into))]
    endpoint_version: EndpointVersion,

    // Limits
    #[builder(default = "DEFAULT_TIMEOUT_MS")]
    timeout_ms: u64,
    #[builder(default = "DEFAULT_MEMORY_LIMIT_MB")]
    memory_limit_mb: u32,
    #[builder(default = "DEFAULT_MAX_RESPONSE_LENGTH")]
    max_response_length: usize,
    #[builder(default = "DEFAULT_MAX_CONSOLE_LOG_LENGTH")]
    max_console_log_length: usize,
    #[builder(default = "DEFAULT_MAX_FETCH_COUNT")]
    max_fetch_count: u32,
    #[builder(default = "DEFAULT_MAX_SIGN_COUNT")]
    max_sign_count: u32,
    #[builder(default = "DEFAULT_MAX_CONTRACT_CALL_COUNT")]
    max_contract_call_count: u32,
    #[builder(default = "DEFAULT_MAX_BROADCAST_AND_COLLECT_COUNT")]
    max_broadcast_and_collect_count: u32,
    #[builder(default = "DEFAULT_MAX_CALL_DEPTH")]
    max_call_depth: u32,

    // State
    #[builder(setter(skip))]
    state: ExecutionState,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ExecutionState {
    pub response: String,
    pub logs: String,
    pub fetch_count: u32,
    pub sign_count: u32,
    pub signed_data: HashMap<String, models::SignedData>,
    pub claim_count: u32,
    pub claim_data: HashMap<String, models::JsonPKPClaimKeyResponse>,
    pub contract_call_count: u32,
    pub broadcast_and_collect_count: u32,
}

#[derive(Debug, Default, Clone)]
pub struct ExecutionOptions {
    pub code: String,
    pub globals: Option<serde_json::Value>,
    pub action_ipfs_id: Option<String>,
}

impl From<&str> for ExecutionOptions {
    fn from(code: &str) -> Self {
        Self {
            code: code.to_string(),
            ..Default::default()
        }
    }
}

impl From<String> for ExecutionOptions {
    fn from(code: String) -> Self {
        Self {
            code,
            ..Default::default()
        }
    }
}

impl Client {
    pub fn new(socket_path: impl Into<PathBuf>) -> Self {
        ClientBuilder::default()
            .socket_path(socket_path)
            .build()
            .expect("cannot fail")
    }

    fn lit_config(&self) -> &LitConfig {
        self.js_env.cfg.as_ref()
    }

    fn socket_path(&self) -> PathBuf {
        self.socket_path.clone().unwrap_or_else(|| {
            self.lit_config()
                .actions_socket()
                .expect("invalid socket path in config")
        })
    }

    pub fn request_id(&self) -> String {
        self.request_id.clone().unwrap_or_default()
    }

    pub fn logs(&self) -> &str {
        &self.state.logs
    }

    fn tss_state_and_txn_prefix(
        &self,
    ) -> Result<(crate::tss::common::tss_state::TssState, String)> {
        let tss_state = self
            .js_env
            .tss_state
            .clone()
            .expect_or_err("No TSS state found")?;
        let txn_prefix = self.request_id();
        Ok((tss_state, txn_prefix))
    }

    fn metadata(&self) -> Result<MetadataMap> {
        let mut md = MetadataMap::new();
        md.insert(
            "x-host",
            self.lit_config()
                .external_addr()
                .unwrap_or_default()
                .parse()?,
        );
        md.insert("x-request-id", self.request_id().parse()?);
        Ok(md)
    }

    fn reset_state(&mut self) {
        std::mem::take(&mut self.state);
    }

    // Use this blocking version of execute_js when running into this error:
    // "implementation of `std::marker::Send` is not general enough"
    // Calling block_in_place should be fine if the code is already running in
    // a multi-threaded Tokio runtime, e.g. with Rocket.
    // See https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html
    #[instrument(skip_all, ret)]
    pub fn execute_js_blocking(
        &mut self,
        opts: impl Into<ExecutionOptions>,
    ) -> Result<ExecutionState, crate::error::Error> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async { self.execute_js(opts).await })
        })
    }

    #[instrument(skip_all, ret)]
    pub async fn execute_js(
        &mut self,
        opts: impl Into<ExecutionOptions>,
    ) -> Result<ExecutionState, crate::error::Error> {
        self.reset_state();
        let opts = opts.into();
        self.execute_js_inner(opts.code, opts.globals, opts.action_ipfs_id, 0)
            .await
            .map_err(|e| {
                if let Some(status) = e.downcast_ref::<Status>() {
                    match status.code() {
                        Code::DeadlineExceeded => timeout_err(status.message(), None),
                        Code::ResourceExhausted => memory_limit_err(status.message(), None),
                        _ => unexpected_err(e, None),
                    }
                } else if let Some(te) = e.downcast_ref::<TransportError>() {
                    connect_err(te.source().unwrap_or(te).to_string(), None)
                } else {
                    unexpected_err(e, None)
                }
            })
    }

    #[instrument(skip(self), err)]
    async fn execute_js_inner(
        &mut self,
        code: String,
        globals: Option<serde_json::Value>,
        action_ipfs_id: Option<String>,
        call_depth: u32,
    ) -> Result<ExecutionState> {
        let (outbound_tx, outbound_rx) = flume::bounded(0);
        let channel = unix::connect_to_socket(self.socket_path()).await?;
        let mut stream = ActionClient::new(channel)
            .execute_js(Request::from_parts(
                self.metadata()?,
                Extensions::default(),
                outbound_rx.into_stream(),
            ))
            .await?
            .into_inner();

        let auth_context = {
            // Add current ipfs id to the end of the action_ipfs_ids vec
            let mut ctx = self.js_env.auth_context.clone();
            if let Some(id) = &action_ipfs_id {
                ctx.action_ipfs_ids.push(id.clone());
            }
            ctx
        };

        // Send initial execution request to server
        outbound_tx
            .send_async(
                ExecutionRequest {
                    code,
                    js_params: globals.and_then(|v| serde_json::to_vec(&v).ok()),
                    auth_context: serde_json::to_vec(&auth_context).ok(),
                    http_headers: self.http_headers.clone(),
                    timeout: Some(self.timeout_ms),
                    memory_limit: Some(self.memory_limit_mb),
                }
                .into(),
            )
            .await
            .context("failed to send execution request")?;

        // Handle responses from server
        while let Some(resp) = stream.next().await {
            debug!(?resp);
            match resp {
                Ok(resp) => {
                    match resp.union {
                        // Return final result from server
                        Some(UnionResponse::Result(res)) => {
                            if !res.success {
                                bail!(res.error);
                            }
                            // Return current state, which might be updated by subsequent code executions
                            return Ok(self.state.clone());
                        }
                        // Handle op requests
                        Some(op) => {
                            let resp = match self
                                .handle_op(op, action_ipfs_id.clone(), call_depth)
                                .await
                            {
                                Ok(resp) => resp,
                                Err(e) => ErrorResponse {
                                    error: e.to_string(),
                                }
                                .into(),
                            };
                            outbound_tx
                                .send_async(resp)
                                .await
                                .context("failed to send op response")?;
                        }
                        // Ignore empty responses
                        None => {}
                    };
                }
                Err(e) => return Err(e.into()),
            }
        }

        unreachable!()
    }

    #[instrument(skip(self), err)]
    async fn handle_op(
        &mut self,
        op: UnionResponse,
        action_ipfs_id: Option<String>,
        mut call_depth: u32,
    ) -> Result<ExecuteJsRequest> {
        Ok(match op {
            UnionResponse::SetResponse(SetResponseRequest { response }) => {
                if response.len() > self.max_response_length {
                    bail!(
                        "Response is too long. Max length is {} bytes",
                        self.max_response_length
                    );
                }
                self.state.response = response;
                SetResponseResponse {}.into()
            }
            UnionResponse::Print(PrintRequest { message }) => {
                if self.state.logs.len() + message.len() > self.max_console_log_length {
                    bail!("Console.log is printing something that is too long. Max length for all logs in a single request is {} bytes",
                        self.max_console_log_length
                    );
                }
                self.state.logs.push_str(&message);
                PrintResponse {}.into()
            }
            UnionResponse::IncrementFetchCount(IncrementFetchCountRequest {}) => {
                self.state.fetch_count += 1;
                if self.state.fetch_count > self.max_fetch_count {
                    bail!("You may not send more than {} HTTP requests per session and you have attempted to exceed that limit.",
                        self.max_fetch_count
                    );
                }
                IncrementFetchCountResponse {
                    fetch_count: self.state.fetch_count,
                }
                .into()
            }
            UnionResponse::PkpPermissionsGetPermitted(PkpPermissionsGetPermittedRequest {
                method,
                token_id,
            }) => {
                let resources =
                    pkp::utils::pkp_permissions_get_permitted(method, self.lit_config(), token_id)
                        .await?;
                PkpPermissionsGetPermittedResponse {
                    resources: serde_json::to_vec(&resources)?,
                }
                .into()
            }
            UnionResponse::PkpPermissionsGetPermittedAuthMethodScopes(
                PkpPermissionsGetPermittedAuthMethodScopesRequest {
                    token_id,
                    method,
                    user_id,
                    max_scope_id,
                },
            ) => {
                let scopes = pkp::utils::pkp_permissions_get_permitted_auth_method_scopes(
                    token_id,
                    self.lit_config(),
                    method,
                    user_id,
                    max_scope_id,
                )
                .await?;
                PkpPermissionsGetPermittedAuthMethodScopesResponse { scopes }.into()
            }
            UnionResponse::PkpPermissionsIsPermitted(PkpPermissionsIsPermittedRequest {
                method,
                token_id,
                params,
            }) => {
                let is_permitted = pkp::utils::pkp_permissions_is_permitted(
                    token_id,
                    self.lit_config(),
                    method,
                    serde_json::from_slice(&params)?,
                )
                .await?;
                PkpPermissionsIsPermittedResponse { is_permitted }.into()
            }
            UnionResponse::PkpPermissionsIsPermittedAuthMethod(
                PkpPermissionsIsPermittedAuthMethodRequest {
                    token_id,
                    method,
                    user_id,
                },
            ) => {
                use lit_blockchain::resolver::contract::ContractResolver;

                let cfg = self.lit_config();
                let resolver = ContractResolver::try_from(cfg)?;
                let contract = resolver.pkp_permissions_contract(cfg).await?;
                let is_permitted = pkp::utils::pkp_permissions_is_permitted_auth_method(
                    token_id, cfg, method, user_id,
                )
                .await?;
                PkpPermissionsIsPermittedAuthMethodResponse { is_permitted }.into()
            }
            UnionResponse::PubkeyToTokenId(PubkeyToTokenIdRequest { public_key }) => {
                let bytes = encoding::hex_to_bytes(public_key)?;
                let token_id = format!("0x{}", encoding::bytes_to_hex(keccak256(bytes).as_slice()));
                PubkeyToTokenIdResponse { token_id }.into()
            }
            UnionResponse::SignEcdsa(SignEcdsaRequest {
                to_sign,
                public_key,
                sig_name,
                eth_personal_sign,
            }) => {
                let success = if eth_personal_sign {
                    // Prepend the Ethereum Signed Message according to EIP-191
                    let mut message =
                        format!("\x19Ethereum Signed Message:\n{}", to_sign.len()).into_bytes();
                    message.extend(&to_sign);

                    // Hash it using keccak256
                    let hashed_message = keccak256(message);

                    self.sign_ecdsa_helper(
                        hashed_message.into(),
                        public_key,
                        sig_name,
                        &[2], // AuthMethodScope::SignPersonalMessage
                        self.epoch,
                        action_ipfs_id,
                    )
                    .await
                } else {
                    self.sign_ecdsa_helper(
                        to_sign,
                        public_key,
                        sig_name,
                        &[1], // AuthMethodScope::SignAnything
                        self.epoch,
                        action_ipfs_id,
                    )
                    .await
                }?;
                SignEcdsaResponse { success }.into()
            }
            UnionResponse::AesDecrypt(AesDecryptRequest {
                symmetric_key,
                ciphertext,
            }) => {
                let plaintext = super::aes::aes_decrypt(symmetric_key, ciphertext).await?;
                AesDecryptResponse { plaintext }.into()
            }
            UnionResponse::GetLatestNonce(GetLatestNonceRequest { address, chain }) => {
                use ethers::prelude::*;
                use std::str::FromStr;

                let provider = ENDPOINT_MANAGER.get_provider(&chain)?;
                let addr = ethers::types::Address::from_str(&address)
                    .map_err(|e| conversion_err(e, None))?;
                let latest_nonce = provider.get_transaction_count(addr, None).await?;
                debug!("op_get_latest_nonce; addr: {addr}, latest_nonce: {latest_nonce}");
                GetLatestNonceResponse {
                    nonce: format!("0x{latest_nonce:x}"),
                }
                .into()
            }
            UnionResponse::CheckConditions(CheckConditionsRequest {
                conditions,
                auth_sig,
                chain,
            }) => {
                let json_auth_sig = self.parse_json_authsig_helper(auth_sig)?;
                let conditions: Vec<models::UnifiedAccessControlConditionItem> =
                    serde_json::from_slice(&conditions)?;

                let chain = match chain {
                    Some(chain) => chain,
                    None => {
                        bail!("Chain is required for access control checks");
                    }
                };
                let res = self
                    .check_access_control_conditions_helper(
                        &conditions,
                        json_auth_sig,
                        chain,
                        action_ipfs_id,
                    )
                    .await?;

                CheckConditionsResponse {
                    success: res.result,
                }
                .into()
            }
            UnionResponse::ClaimKeyIdentifier(ClaimKeyIdentifierRequest { key_id }) => {
                use ethers::prelude::*;

                // XXX: This value is never used. Should we enforce a limit?
                self.state.claim_count += 1;

                let ipfs_id = match action_ipfs_id {
                    Some(id) => id,
                    None => {
                        bail!("Could not find IPFS ID for this action, aborting claim operation")
                    }
                };

                let serialized = format!("{}_{}", ipfs_id.clone(), key_id.clone());
                let as_bytes = serialized.as_bytes().to_vec();
                let formatted_key_id = keccak256(as_bytes).to_vec();
                let wallet = lit_blockchain::contracts::load_wallet(self.lit_config(), None)
                    .map_err(|e| unexpected_err(e, None))?;
                let signature = wallet
                    .sign_message(&formatted_key_id)
                    .await
                    .map_err(|e| unexpected_err(e, Some("Could not sign message".into())))?;

                let signature = encoding::bytes_to_hex(signature.to_vec());
                let key_id_hex = encoding::bytes_to_hex(formatted_key_id.clone());

                self.state.claim_data.insert(
                    key_id,
                    models::JsonPKPClaimKeyResponse {
                        signature,
                        derived_key_id: key_id_hex,
                    },
                );

                ClaimKeyIdentifierResponse {
                    success: "success".to_string(),
                }
                .into()
            }
            UnionResponse::CallContract(CallContractRequest { chain, txn }) => {
                use ethers::prelude::*;
                use ethers::types::transaction::eip2718::TypedTransaction;
                use ethers::utils::rlp::Rlp;

                self.state.contract_call_count += 1;
                if self.state.contract_call_count > self.max_contract_call_count {
                    bail!("You may invoke contract calls more than {} times per session and you have attempted to exceed that limit.",
                        self.max_contract_call_count
                    );
                }

                // FIXME: ideally we should try each index until we find one that works.  0 is hardcoded for now.
                let provider: Provider<Http> = ENDPOINT_MANAGER.get_provider(chain.as_str())?;
                let txn_bytes = encoding::hex_to_bytes(&txn)?;
                let rlp = Rlp::new(&txn_bytes);
                let mut decoded_txn = TransactionRequest::decode_unsigned_rlp(&rlp)?;

                // set gas limit if none is passed, otherwise the txn call will fail
                if decoded_txn.gas.is_none()
                    || decoded_txn.gas.unwrap_or(U256::zero()) == U256::zero()
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

                CallContractResponse {
                    result: format!("0x{}", encoding::bytes_to_hex(result)),
                }
                .into()
            }
            UnionResponse::CallChild(CallChildRequest { ipfs_id, params }) => {
                call_depth += 1;
                if call_depth > self.max_call_depth {
                    bail!("The recursion limit of a child action is {} and you have attempted to exceed that limit.",
                        self.max_call_depth
                    );
                }

                // Pull down the lit action code from IPFS
                let code = crate::utils::web::get_ipfs_file(&ipfs_id, self.lit_config()).await?;

                let globals = params
                    .map(|params| serde_json::from_slice::<serde_json::Value>(&params))
                    .transpose()?;

                // NB: Using execute_js_inner instead of execute_js to avoid resetting state
                let res = Box::pin(self.execute_js_inner(code, globals, Some(ipfs_id), call_depth))
                    .await?;

                CallChildResponse {
                    response: res.response,
                }
                .into()
            }
            UnionResponse::BroadcastAndCollect(BroadcastAndCollectRequest { name, value }) => {
                trace!("Starting BroadcastAndCollect with name: {}", &name);
                self.increment_broad_and_collect_counter()?;

                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let txn_prefix = format!("{}_{}", txn_prefix, name);

                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0").await?;
                let values = cm
                    .broadcast_and_collect::<String, String>(value.clone())
                    .await?;

                let mut values: Vec<String> = values.into_iter().map(|(k, v)| v).collect();
                values.push(value);

                trace!("BroadcastAndCollect returns: {:?}", &values);
                BroadcastAndCollectResponse { name, values }.into()
            }
            UnionResponse::DecryptAndCombine(DecryptAndCombineRequest {
                access_control_conditions,
                ciphertext,
                data_to_encrypt_hash,
                auth_sig,
                chain,
            }) => {
                trace!("DecryptAndCombine is in BETA mode.");
                trace!("Ciphertext: {:?}", &ciphertext);

                self.increment_broad_and_collect_counter()?;

                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let json_auth_sig = self.parse_json_authsig_helper(auth_sig)?;
                let txn_prefix = format!("{}_{}", txn_prefix, generate_hash(ciphertext.clone()));

                let conditions: Vec<models::UnifiedAccessControlConditionItem> =
                    serde_json::from_slice(&access_control_conditions)?;

                if !self
                    .check_access_control_conditions_helper(
                        &conditions,
                        json_auth_sig,
                        chain,
                        action_ipfs_id,
                    )
                    .await?
                    .result
                {
                    bail!("Access control conditions check failed.  Check that you are allowed to decrypt this item.");
                }

                let bls_root_pubkey = self.get_bls_root_pubkey().await?;

                let identity_parameter =
                    self.get_identity_param(&conditions, &data_to_encrypt_hash)?;

                // Load the BLS secret key share as a blsful key for signing.
                let cipher_state = match tss_state.get_cipher_state(CurveType::BLS) {
                    Ok(cipher_state) => cipher_state,
                    Err(e) => {
                        bail!("Couldn't get BLS ciper state: {:?}", e);
                    }
                };

                // Sign the identity parameter using the blsful secret key share.
                let (signature_share, share_index) = match cipher_state
                    .sign(identity_parameter.clone(), self.epoch)
                    .await
                {
                    Ok(signature_share) => signature_share,
                    Err(e) => {
                        bail!("Couldn't sign the identity parameter: {:?}", e);
                    }
                };

                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0").await?;
                let mut shares = cm
                    .broadcast_and_collect::<SignatureShare<Bls12381G2Impl>, SignatureShare<Bls12381G2Impl>>(
                        signature_share
                    )
                    .await?;

                shares.push((0, signature_share)); // lazy - it's not zero, but we don't seem to care!

                let network_pubkey = &get_bls_root_pubkey(&tss_state).await?;

                let serialized_decryption_shares: Vec<String> = shares
                    .iter()
                    .map(|(index, share)| serde_json::to_string(share).unwrap_or("".to_string()))
                    .collect();

                let decrypted = lit_bls_wasm::verify_and_decrypt::<Bls12381G2Impl>(
                    network_pubkey,
                    &identity_parameter,
                    &base64_decode(&ciphertext),
                    &serialized_decryption_shares,
                );

                let decrypted = match decrypted {
                    Ok(decrypted) => decrypted,
                    Err(e) => {
                        bail!("Failed to decrypt and combine: {:?}", e);
                    }
                };

                let result_as_bytes = base64_decode(&decrypted);
                let result = match std::str::from_utf8(&result_as_bytes) {
                    Ok(result) => result.to_string(),
                    Err(e) => {
                        bail!("Failed to convert decrypted bytes to string.")
                    }
                };

                DecryptAndCombineResponse { result }.into()
            }
            UnionResponse::DecryptToSingleNode(DecryptToSingleNodeRequest {
                access_control_conditions,
                ciphertext,
                data_to_encrypt_hash,
                auth_sig,
                chain,
            }) => {
                trace!("Decrypt to Single node is in BETA mode.");
                trace!("Ciphertext: {:?}", &ciphertext);

                self.increment_broad_and_collect_counter()?;

                let json_auth_sig = self.parse_json_authsig_helper(auth_sig)?;

                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let request_hash = generate_hash(txn_prefix.clone());

                let peers = tss_state.peer_state.peers().await?;
                let (leader_addr, is_leader) = self.leader_helper(request_hash).await?;

                let txn_prefix = format!("{}_{}", txn_prefix, generate_hash(ciphertext.clone()));

                let conditions: Vec<models::UnifiedAccessControlConditionItem> =
                    serde_json::from_slice(&access_control_conditions)?;

                let bls_root_pubkey = self.get_bls_root_pubkey().await?;

                if !self
                    .check_access_control_conditions_helper(
                        &conditions,
                        json_auth_sig,
                        chain,
                        action_ipfs_id,
                    )
                    .await?
                    .result
                {
                    bail!("Access control conditions check failed.  Check that you are allowed to decrypt this item.");
                }

                let identity_parameter =
                    self.get_identity_param(&conditions, &data_to_encrypt_hash)?;

                // Load the BLS secret key share as a blsful key for signing.
                let cipher_state = match tss_state.get_cipher_state(CurveType::BLS) {
                    Ok(cipher_state) => cipher_state,
                    Err(e) => {
                        bail!("Couldn't get BLS ciper state: {:?}", e);
                    }
                };

                // Sign the identity parameter using the blsful secret key share.
                let (signature_share, share_index) = match cipher_state
                    .sign(identity_parameter.clone(), self.epoch)
                    .await
                {
                    Ok(signature_share) => signature_share,
                    Err(e) => {
                        bail!("Couldn't sign the identity parameter: {:?}", e);
                    }
                };

                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0").await?;
                let leader_peer = peers.peer_at_address(&leader_addr)?;

                let result = match is_leader {
                    false => {
                        let r = cm
                            .send_direct::<SignatureShare<Bls12381G2Impl>>(
                                &leader_peer,
                                signature_share,
                            )
                            .await?;
                        "".to_string() // empty string?
                    }
                    true => {
                        let expected_peers = peers.all_peers_except(&leader_peer.socket_address); // I'm the leader!

                        let threshold =
                            (crate::utils::consensus::get_threshold_count(peers.len()) as u16) - 1;
                        let mut shares = cm
                            .collect_from_earliest::<SignatureShare<Bls12381G2Impl>>(
                                &expected_peers,
                                threshold,
                            )
                            .await?;

                        shares.push((0, signature_share)); // lazy - it's not zero, but we don't seem to care!

                        let network_pubkey = &get_bls_root_pubkey(&tss_state).await?;

                        let serialized_decryption_shares: Vec<String> = shares
                            .iter()
                            .map(|(index, share)| {
                                serde_json::to_string(share).unwrap_or("".to_string())
                            })
                            .collect();

                        let decrypted = lit_bls_wasm::verify_and_decrypt::<Bls12381G2Impl>(
                            network_pubkey,
                            &identity_parameter,
                            &base64_decode(&ciphertext),
                            &serialized_decryption_shares,
                        );

                        let decrypted = match decrypted {
                            Ok(decrypted) => decrypted,
                            Err(e) => {
                                bail!("Failed to decrypt and combine: {:?}", e);
                            }
                        };

                        let result_as_bytes = base64_decode(&decrypted);
                        let result = match std::str::from_utf8(&result_as_bytes) {
                            Ok(result) => result.to_string(),
                            Err(e) => {
                                bail!("Failed to convert decrypted bytes to string.")
                            }
                        };

                        result
                    }
                };

                DecryptToSingleNodeResponse { result }.into()
            }
            UnionResponse::SignAndCombineEcdsa(SignAndCombineEcdsaRequest {
                to_sign,
                public_key,
                sig_name,
            }) => {
                self.increment_broad_and_collect_counter()?;
                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let txn_prefix = format!("{}_combine_{}", txn_prefix, sig_name);

                let result = self
                    .sign_ecdsa_helper(
                        to_sign.clone(),
                        public_key.clone(),
                        sig_name.clone(),
                        &[1], // AuthMethodScope::SignAnything
                        self.epoch,
                        action_ipfs_id,
                    )
                    .await?;

                // remove the signed data from the state if we find it.  May want to check scaling of borrow_mut after POC.
                let signed_data = self
                    .state
                    .signed_data
                    .borrow_mut()
                    .remove(&sig_name)
                    .expect_or_err("No signed data found")?;

                use crate::models::SignedData;
                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0").await?;
                let mut shares = cm
                    .broadcast_and_collect::<SignedData, SignedData>(signed_data.clone())
                    .await?;

                let msg_hash = k256::Scalar::from_be_bytes(&to_sign).unwrap_or_default();
                let public_key = k256::AffinePoint::from_uncompressed_hex(&public_key)
                    .ok_or(anyhow::Error::msg("Failed to parse public key"))?;

                shares.push((signed_data.share_index as u16, signed_data.clone()));

                // remove shares where the big_r is blank
                shares.retain(|(_, share)| !share.signature_share.is_empty());

                let first_share = shares.first().expect_or_err("No shares found")?.1.clone();

                let presignature_big_r: k256::AffinePoint =
                    serde_json::from_str(&first_share.big_r).unwrap_or(k256::AffinePoint::IDENTITY);

                let shares: Vec<k256::Scalar> = shares
                    .iter()
                    .map(|(_, share)| {
                        serde_json::from_str::<k256::Scalar>(&share.signature_share)
                            .unwrap_or_default()
                    })
                    .collect();

                let sig = lit_ecdsa_wasm_combine::combiners::k256_cait_sith::do_combine_signature(
                    public_key,
                    presignature_big_r,
                    msg_hash,
                    shares,
                );
                // done inline, as we may remove this code.
                #[derive(serde::Serialize, serde::Deserialize)]
                struct Rsv {
                    r: String,
                    s: String,
                    v: u8,
                }

                let result = serde_json::to_string(&Rsv {
                    r: sig.r.to_compressed_hex(),
                    s: sig.s.to_be_hex(),
                    v: sig.recid,
                })
                .unwrap_or("".to_string());

                SignAndCombineEcdsaResponse { result }.into()
            }
            UnionResponse::GetRpcUrl(GetRpcUrlRequest { chain }) => {
                let result = match rpc_url(chain) {
                    Ok(url) => url,
                    Err(e) => format!("Error getting RPC URL: {:?}", e).to_string(),
                };
                GetRpcUrlResponse { result }.into()
            }

            UnionResponse::P2pBroadcast(P2pBroadcastRequest { name, value }) => {
                trace!("Starting Broadcast  with name/value: {}/{}", &name, &value);
                self.increment_broad_and_collect_counter()?;
                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let txn_prefix = format!("{}_{}", txn_prefix, name);

                trace!(
                    "Broadcasting to all peers: {} with header {}",
                    &value,
                    &txn_prefix
                );

                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0").await?;
                let result = cm.broadcast::<String>(value.clone()).await?;

                trace!("Broadcast done - {}.", result);

                P2pBroadcastResponse { result }.into()
            }

            UnionResponse::P2pCollectFromLeader(P2pCollectFromLeaderRequest { name }) => {
                trace!("Starting P2PCollect  with name: {}", &name);

                self.increment_broad_and_collect_counter()?;

                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                // note that the default leader function doesn't take a function paramater, thus we need to generate a hash from the transaction id only
                let request_hash = generate_hash(txn_prefix.clone());
                let txn_prefix = format!("{}_{}", txn_prefix, name);
                let (leader_addr, is_leader) = self.leader_helper(request_hash).await?;

                let mut peers = tss_state.peer_state.peers().await?;
                peers.set_all_protocol_indices(0);

                let leader = peers
                    .peer_at_address(&leader_addr)
                    .expect_or_err("Leader address not in peer list")?;
                let from_peers = vec![leader.clone()];

                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0").await?;
                let values = cm.collect_from::<String>(&from_peers).await?;

                let value = match values.is_empty() {
                    true => "".to_string(),
                    false => values[0].1.clone().to_string(),
                };

                trace!("P2P Collect done - received: {:?}", value);

                P2pCollectFromLeaderResponse { name, value }.into()
            }

            UnionResponse::IsLeader(IsLeaderRequest {}) => {
                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let request_hash = generate_hash(txn_prefix.clone());
                let (leader_addr, result) = self.leader_helper(request_hash).await?;
                IsLeaderResponse { result }.into()
            }

            UnionResponse::EncryptBls(EncryptBlsRequest {
                access_control_conditions,
                to_encrypt,
            }) => {
                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let network_pubkey = &get_bls_root_pubkey(&tss_state).await?;

                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(&to_encrypt);
                let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

                let conditions: Vec<models::UnifiedAccessControlConditionItem> =
                    serde_json::from_slice(&access_control_conditions)?;

                let message_bytes = &to_encrypt;
                let identity_param = self.get_identity_param(&conditions, &data_to_encrypt_hash)?;

                trace!("Identity parameter: {:?}", identity_param);

                let ciphertext = match lit_bls_wasm::encrypt_time_lock::<Bls12381G2Impl>(
                    network_pubkey,
                    message_bytes.to_vec(),
                    identity_param.clone(),
                ) {
                    Ok(ciphertext) => ciphertext,
                    Err(e) => {
                        bail!("Failed to encrypt: {:?}", e);
                    }
                };

                EncryptBlsResponse {
                    ciphertext,
                    data_to_encrypt_hash,
                }
                .into()
            }

            UnionResponse::Result(_) => unreachable!(), // handled in main loop
        })
    }

    fn parse_json_authsig_helper(&self, auth_sig: Option<Vec<u8>>) -> Result<JsonAuthSig> {
        match auth_sig {
            Some(auth_sig) => match serde_json::from_slice(&auth_sig)? {
                AuthSigItem::Single(auth_sig) => Ok(auth_sig),
                _ => bail!("Only supports Json AuthSig"),
            },
            None => match &self.auth_sig {
                Some(auth_sig) => Ok(auth_sig.clone()),
                None => bail!("No auth_sig found.  You must either pass one to the function, or set one in your client when sending the request"),
            },
        }
    }

    fn increment_broad_and_collect_counter(&mut self) -> Result<()> {
        self.state.broadcast_and_collect_count += 1;
        if self.state.broadcast_and_collect_count > self.max_broadcast_and_collect_count {
            bail!("You may not use broadcast and collect functionality more than {} times per session and you have attempted to exceed that limit.",
                self.max_broadcast_and_collect_count,
            );
        };
        Ok(())
    }

    fn get_identity_param(
        &self,
        conditions: &Vec<models::UnifiedAccessControlConditionItem>,
        data_to_encrypt_hash: &str,
    ) -> Result<Vec<u8>> {
        let hash_res = hash_access_control_conditions(RequestConditions {
            access_control_conditions: None,
            evm_contract_conditions: None,
            sol_rpc_conditions: None,
            unified_access_control_conditions: Some(conditions.clone()),
        });
        let hashed_access_control_conditions = match hash_res {
            Ok(hashed_access_control_conditions) => hashed_access_control_conditions,
            Err(e) => {
                bail!("Couldn't hash access control conditions: {:?}", e);
            }
        };

        let identity_param = AccessControlConditionResource::new(format!(
            "{}/{}",
            hashed_access_control_conditions, data_to_encrypt_hash
        ))
        .get_resource_key()
        .into_bytes();
        // Get the identity parameter to be signed.
        Ok(identity_param)
    }

    async fn sign_ecdsa_helper(
        &mut self,
        to_sign: Vec<u8>,
        pubkey: String,
        sig_name: String,
        required_scopes: &[usize],
        epoch: Option<u64>,
        action_ipfs_id: Option<String>,
    ) -> Result<String> {
        self.state.sign_count += 1;
        if self.state.sign_count > self.max_sign_count {
            bail!("You may not sign more than {} times per session and you have attempted to exceed that limit.",
                self.max_sign_count,
            );
        }

        debug!(
            "sign_ecdsa_helper() called with to_sign: {:?}, pubkey: {}, sig_name: {}",
            encoding::bytes_to_hex(to_sign.clone()),
            pubkey,
            sig_name
        );

        let bls_root_pubkey = self.get_bls_root_pubkey().await?;

        // accept pubkey with and without 0x prefix
        let pubkey = pubkey.replace("0x", "");

        if self.auth_sig.is_none() {
            return Err(anyhow::anyhow!(
                "You can not sign without providing an auth_sig. You must create a session with the PKP, and then pass session sigs in, which will be converted to an auth sig per node. Refer the the docs on creating and using session sigs."
            ));
        }

        let (result, key_type) = crate::pkp::utils::sign_ecdsa(
            self.lit_config(),
            &to_sign,
            pubkey,
            self.request_id(),
            action_ipfs_id,
            self.auth_sig.clone(),
            self.js_env.auth_context.clone(),
            self.js_env.tss_state.clone(),
            required_scopes,
            epoch,
            &bls_root_pubkey,
        )
        .await
        .map_err(|e| anyhow::anyhow!(format!("Failed to sign ecdsa: {:?}", e)))?;

        debug!("ECDSA signing complete");

        // pad the pubkey with a zero at the front if it's odd because hex strings should be even and zero padded
        let mut padded_pubkey = result.public_key;
        if padded_pubkey.len() % 2 == 1 {
            padded_pubkey = format!("0{}", &padded_pubkey);
        }

        // this state is persisted across calls by deno, and so we can use it to
        // return data to the client that called this Lit function via HTTP
        self.state.signed_data.insert(
            sig_name.to_string(),
            models::SignedData {
                sig_type: key_type.to_string(),
                data_signed: result.digest,
                signature_share: result.signature_share,
                share_index: result.share_index as u32,
                big_r: result.big_r,
                public_key: padded_pubkey,
                sig_name,
            },
        );

        debug!(
            "sign_ecdsa_helper() returning: {:?}",
            self.state.signed_data
        );

        Ok("success".to_string())
    }

    async fn check_access_control_conditions_helper(
        &self,
        conditions: &Vec<models::UnifiedAccessControlConditionItem>,
        json_auth_sig: JsonAuthSig,
        chain: String,
        action_ipfs_id: Option<String>,
    ) -> Result<UnifiedConditionCheckResult> {
        trace!("access_control_conditions: {:?}", &conditions);

        let bls_root_pubkey = self.get_bls_root_pubkey().await?;
        let lit_action_resource = crate::auth::resources::LitActionResource::new("".to_string());

        crate::access_control::unified::check_access_control_conditions(
            conditions,
            &AuthSigItem::Single(json_auth_sig),
            Some(chain),
            &lit_action_resource.execution_ability(),
            self.js_env.cfg.clone(),
            &self.request_id(),
            &bls_root_pubkey,
            &self.endpoint_version,
            action_ipfs_id.as_ref(),
        )
        .await
        .map_err(|e| anyhow::anyhow!(format!("Error checking access control conditions: {:?}", e)))
    }

    async fn get_bls_root_pubkey(&self) -> Result<String> {
        let tss_state = match &self.js_env.tss_state {
            Some(tss_state) => tss_state,
            None => {
                return Err(anyhow::anyhow!("No TSS state found"));
            }
        };

        let cipher_state = match tss_state.get_cipher_state(CurveType::BLS) {
            Ok(cipher_state) => cipher_state,
            Err(e) => {
                return Err(e.into());
            }
        };
        let bls_root_pubkeys = cipher_state.root_keys().await;
        match bls_root_pubkeys.first() {
            Some(bls_root_key) => Ok(bls_root_key.clone()),
            None => Err(anyhow::anyhow!("No BLS root key found")),
        }
    }

    async fn leader_helper(&self, request_hash: u64) -> Result<(String, bool)> {
        let tss_state = match &self.js_env.tss_state {
            Some(tss_state) => tss_state,
            None => {
                return Err(anyhow::anyhow!("No TSS state found"));
            }
        };
        let peers = tss_state.peer_state.peers().await?;
        let addr = &tss_state.addr;
        let leader_addr = leader_addr(request_hash, &peers)?;

        let is_leader = addr == leader_addr;

        Ok((leader_addr.clone(), is_leader))
    }
}
