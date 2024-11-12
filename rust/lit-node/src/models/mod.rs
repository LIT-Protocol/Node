use blsful::{Bls12381G2Impl, SignatureShare as BlsfulSignatureShare};
use ethers::types::{Address, U256};

use iri_string::spec::UriSpec;
use iri_string::types::RiString;
use lit_attestation::Attestation;
use lit_blockchain::resolver::rpc::config::RpcConfig;
#[cfg(feature = "lit-actions")]
use lit_core::config::LitConfig;
use moka::future::Cache;
use rocket::form;
use rocket::form::{FromFormField, ValueField};
use rocket::serde::{Deserialize, Serialize};
use serde_encrypt::{serialize::impls::BincodeSerializer, traits::SerdeEncryptPublicKey};
use serde_json::Value;
use std::collections::HashMap;
#[cfg(feature = "lit-actions")]
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use web3::types::{Bytes, CallRequest};
use webauthn_rs_core::proto::PublicKeyCredential;
use xor_name::XorName;

use crate::auth::auth_material::{AuthSigItem, JsonAuthSig};
use crate::tss::dkg::curves::common::CurveType;

pub mod auth;
pub mod coms_keys;
pub mod siwe;
pub mod webauthn_signature_verification_material;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminAuth {
    pub auth_sig: JsonAuthSig,
}

impl<'r> FromFormField<'r> for AdminAuth {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let v = data_encoding::BASE64
            .decode(field.value.as_bytes())
            .map_err(|e| {
                form::Error::validation(format!("auth field needs to be base64: {:?}", e))
            })?;
        let auth: AdminAuth = serde_json::from_slice(&v).map_err(|e| {
            form::Error::validation(format!("auth failed to decode from JSON: {:?}", e))
        })?;

        Ok(auth)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonAdminSetRequest {
    pub new_config: HashMap<String, String>,
    pub rpc_config: RpcConfig,
    pub auth_sig: JsonAuthSig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonRecoverySetDecShare {
    pub auth_sig: JsonAuthSig,
    pub share_data: UploadedShareData,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadedShareData {
    pub participant_id: usize, // numeric identifier used when the decryption share was generated
    pub session_id: String,    // lower hex encoding of the session_id
    pub encryption_key: String, // lower hex encoding of canonical point form i.e uncompressed point
    pub verification_key: String, // lower hex encoding of canonical point form i.e uncompressed point
    pub decryption_share: String, // lower hex encoding of decryption share bytes
    pub subnet_id: String,        // staking contract address
    pub curve: String, // “BLS12831G1”, “BLS12381G2”, “Secp256k1”, “NistP256”, or “Ed25519”
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadShareRequest {
    pub auth_sig: JsonAuthSig, // auth sig from the backup party member
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadedShareData {
    pub participant_id: usize, // numeric identifier used when the decryption share was generated
    pub session_id: String,    // lower hex encoding of the session_id
    pub encryption_key: String, // lower hex encoding of canonical point form i.e uncompressed point
    pub decryption_key_share: String, // lower hex encoding of canonical scalar form
    pub subnet_id: String,     // staking contract address
    pub curve: String,         // “BLS12381G1”, “BLS12381G2”, “Secp256k1”, “NistP256”, or “Ed25519”
}

pub struct AllowlistCache {
    pub entries: RwLock<HashMap<[u8; 32], AllowlistEntry>>,
}

pub struct AllowlistEntry {
    pub allowed: bool,
    pub last_retrieved_at: SystemTime,
}

pub struct AuthContextCache {
    pub auth_contexts: moka::future::Cache<String, AuthMethodResponse>,
}
pub struct AuthContextCacheExpiry;
impl moka::Expiry<String, AuthMethodResponse> for AuthContextCacheExpiry {
    /// Returns the duration of the expiration of the value that was just
    /// created.
    fn expire_after_create(
        &self,
        _key: &String,
        value: &AuthMethodResponse,
        _current_time: std::time::Instant,
    ) -> Option<Duration> {
        let exp = value.expiration;
        println!("{}", exp);
        Some(std::time::Duration::new(exp as u64, 0))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthContext {
    pub action_ipfs_ids: Vec<String>,
    pub auth_sig_address: Option<String>,
    pub auth_method_contexts: Vec<AuthMethodResponse>,
    pub resources: Vec<RiString<UriSpec>>,
    pub custom_auth_resource: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthMethodResponse {
    pub user_id: String,
    pub app_id: String,
    pub auth_method_type: u32,
    #[serde(skip_serializing)]
    // Since it will be different for every node & which will cause issues for electing the BT leader and signing
    pub last_retrieved_at: SystemTime,
    #[serde(skip_serializing)]
    // Since it will be different for every node & which will cause issues for electing the BT leader and signing
    pub expiration: i64,
    pub used_for_sign_session_key_request: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct AuthMethod {
    pub auth_method_type: u32,
    pub access_token: String,
}

#[cfg(not(test))]
impl std::fmt::Debug for AuthMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthMethod")
            .field("auth_method_type", &self.auth_method_type)
            .field("access_token", &"****filtered****")
            .finish()
    }
}

#[cfg(not(test))]
impl std::fmt::Display for AuthMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AuthMethod {{ auth_method_type: {}, access_token: ****filtered**** }}",
            self.auth_method_type
        )
    }
}

#[cfg(feature = "lit-actions")]
#[derive(Debug, Clone, Default)]
pub struct DenoExecutionEnv {
    pub tss_state: Option<crate::tss::common::tss_state::TssState>,
    pub auth_context: AuthContext,
    pub cfg: Arc<LitConfig>,
    pub ipfs_cache: Option<Cache<String, Arc<String>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyRequest {
    pub session_key: String,
    pub auth_methods: Vec<AuthMethod>,
    pub pkp_public_key: Option<String>,
    pub auth_sig: Option<AuthSigItem>,
    pub siwe_message: String,
    #[serde(default = "default_epoch")]
    pub epoch: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyRequestV1 {
    pub session_key: String,
    pub auth_methods: Vec<AuthMethod>,
    pub pkp_public_key: Option<String>,
    pub auth_sig: Option<AuthSigItem>, // For backwards compatibility
    pub siwe_message: String,
    pub curve_type: CurveType,
    pub code: Option<String>,
    pub lit_action_ipfs_id: Option<String>,
    pub js_params: Option<Value>,
    #[serde(default = "default_epoch")]
    pub epoch: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonExecutionRequest {
    pub code: Option<String>,
    pub ipfs_id: Option<String>,
    pub auth_sig: AuthSigItem,
    pub js_params: Option<Value>,
    pub auth_methods: Option<Vec<AuthMethod>>,
    #[serde(default = "default_epoch")]
    pub epoch: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonPKPSigningRequest {
    pub to_sign: Vec<u8>,
    pub pubkey: String,
    pub auth_sig: AuthSigItem,
    pub auth_methods: Option<Vec<AuthMethod>>, // For backwards compatibility
    #[serde(default = "default_epoch")]
    pub epoch: u64,
}

fn default_epoch() -> u64 {
    0 // this will indicate to the nodes that a valid value isn't coming from the SDK.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonPKPClaimKeyRequest {
    pub auth_method: AuthMethod,
    pub credential_public_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JsonPKPClaimKeyResponse {
    pub signature: String,
    pub derived_key_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IncomingRequest {
    EncryptionSignRequest(EncryptionSignRequest),
    SigningAccessControlConditionRequest(SigningAccessControlConditionRequest),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestConditions {
    pub access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    pub evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    pub sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    pub unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnifiedConditionCheckResult {
    pub result: bool,
    pub successful_auth_sig: JsonAuthSig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoapEvent {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoapEntry {
    pub event: PoapEvent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GennaroDKGRoundMessage {
    pub sender_id: XorName,
    pub encrypted_dkg_round_message: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSDKHandshakeRequest {
    pub client_public_key: String,
    pub challenge: Option<String>, // FIXME: make this non-optional once we've updated the SDK to always send this
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSDKHandshakeResponse {
    pub server_public_key: String,
    pub subnet_public_key: String,
    pub network_public_key: String,
    pub network_public_key_set: String,
    pub client_sdk_version: String,
    pub hd_root_pubkeys: Vec<String>,
    pub attestation: Option<Attestation>,
    pub latest_blockhash: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct JsonEEID {
//     pub eeid: Option<bls::EpochExitId>,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtSignedChainDataPayload {
    pub iss: String,
    pub chain: String,
    pub iat: u64,
    pub exp: u64,
    pub call_requests: Vec<CallRequest>,
    pub call_responses: Vec<Bytes>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonSigningResourceId {
    pub base_url: String,
    pub path: String,
    pub org_id: String,
    pub role: String,
    pub extra_data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SigningAccessControlConditionRequest {
    pub access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    pub evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    pub sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    pub unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
    pub chain: Option<String>,
    pub auth_sig: AuthSigItem,
    pub iat: u64,
    pub exp: u64,
    #[serde(default = "default_epoch")]
    pub epoch: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigningAccessControlConditionResponse {
    pub result: String,
    pub signature_share: BlsfulSignatureShare<Bls12381G2Impl>,
    pub share_index: u32,
    pub unsigned_jwt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionSignRequest {
    pub access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    pub evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    pub sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    pub unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
    pub chain: Option<String>,
    pub data_to_encrypt_hash: String,
    pub auth_sig: AuthSigItem,
    #[serde(default = "default_epoch")]
    pub epoch: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionSignResponse {
    pub result: String,
    pub signature_share: BlsfulSignatureShare<Bls12381G2Impl>,
    pub share_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyResponseV1 {
    pub result: String,
    pub signature_share: BlsfulSignatureShare<Bls12381G2Impl>,
    pub share_index: u32,
    pub curve_type: String,
    pub siwe_message: String,
    pub data_signed: String,
    pub bls_root_pubkey: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyResponse {
    pub success: bool,
    pub signed_data: JsonSignSessionKeyResponseInnerSignedData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyResponseInnerSignedData {
    pub session_sig: JsonSignSessionKeyResponseInnerSigShare,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyResponseInnerSigShare {
    pub signature_share: String,
    pub share_index: u32,
    pub sig_type: String,
    pub siwe_message: String,
    pub data_signed: String,
    pub bigr: String,
    pub public_key: String,
    pub sig_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ControlConditionItem<T> {
    Condition(T),
    Operator(JsonAccessControlConditionOperator),
    Group(Vec<ControlConditionItem<T>>),
}

pub type UnifiedAccessControlConditionItem = ControlConditionItem<UnifiedAccessControlCondition>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
#[allow(clippy::enum_variant_names)]
pub enum UnifiedAccessControlCondition {
    JsonAccessControlCondition(JsonAccessControlCondition),
    SolRpcCondition(SolRpcConditionV2Options),
    EVMContractCondition(EVMContractCondition),
    CosmosCondition(CosmosCondition),
}

pub type CosmosConditionItem = ControlConditionItem<CosmosCondition>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCondition {
    pub path: String,
    pub chain: String,
    pub method: Option<String>,
    pub parameters: Option<Vec<String>>,
    pub return_value_test: JsonReturnValueTestV2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBlock {
    #[serde(rename = "block_id")]
    pub block_id: CosmosBlockId,
    pub block: CosmosBlockBlock,
    #[serde(rename = "sdk_block")]
    pub sdk_block: CosmosBlockSdkBlock,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBlockId {
    pub hash: String,
    #[serde(rename = "part_set_header")]
    pub part_set_header: CosmosPartSetHeader,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosPartSetHeader {
    pub total: i64,
    pub hash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBlockBlock {
    pub header: CosmosBlockHeader,
    pub data: CosmosBlockData,
    pub evidence: CosmosBlockEvidence,
    #[serde(rename = "last_commit")]
    pub last_commit: CosmosBlockLastCommit,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBlockHeader {
    pub version: Version,
    #[serde(rename = "chain_id")]
    pub chain_id: String,
    pub height: String,
    pub time: String,
    #[serde(rename = "last_block_id")]
    pub last_block_id: CosmosBlockId,
    #[serde(rename = "last_commit_hash")]
    pub last_commit_hash: String,
    #[serde(rename = "data_hash")]
    pub data_hash: String,
    #[serde(rename = "validators_hash")]
    pub validators_hash: String,
    #[serde(rename = "next_validators_hash")]
    pub next_validators_hash: String,
    #[serde(rename = "consensus_hash")]
    pub consensus_hash: String,
    #[serde(rename = "app_hash")]
    pub app_hash: String,
    #[serde(rename = "last_results_hash")]
    pub last_results_hash: String,
    #[serde(rename = "evidence_hash")]
    pub evidence_hash: String,
    #[serde(rename = "proposer_address")]
    pub proposer_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub block: String,
    pub app: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBlockData {
    pub txs: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBlockEvidence {
    pub evidence: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBlockLastCommit {
    pub height: String,
    pub round: i64,
    #[serde(rename = "block_id")]
    pub block_id: CosmosBlockId,
    pub signatures: Vec<CosmosBlockSignature>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBlockSignature {
    #[serde(rename = "block_id_flag")]
    pub block_id_flag: String,
    #[serde(rename = "validator_address")]
    pub validator_address: Option<String>,
    pub timestamp: String,
    pub signature: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBlockSdkBlock {
    pub header: CosmosBlockHeader,
    pub data: CosmosBlockData,
    pub evidence: CosmosBlockEvidence,
    #[serde(rename = "last_commit")]
    pub last_commit: CosmosBlockLastCommit,
}

pub type SolRpcConditionItem = ControlConditionItem<SolRpcConditionV2Options>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum SolRpcConditionItemV0 {
    Condition(SolRpcCondition),
    Operator(JsonAccessControlConditionOperator),
    Group(Vec<SolRpcConditionItem>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SolRpcConditionV2Options {
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub pda_params: Option<Vec<String>>,
    pub pda_interface: Option<SolPdaInterface>,
    pub pda_key: Option<String>,
    pub chain: String,
    pub return_value_test: JsonReturnValueTestV2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SolRpcConditionV2 {
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub pda_params: Vec<String>,
    pub pda_interface: SolPdaInterface,
    pub pda_key: String,
    pub chain: String,
    pub return_value_test: JsonReturnValueTestV2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SolRpcCondition {
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub chain: String,
    pub return_value_test: JsonReturnValueTestV2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SolPdaInterface {
    pub offset: usize,
    pub fields: HashMap<String, usize>,
}

pub type EVMContractConditionItem = ControlConditionItem<EVMContractCondition>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EVMContractCondition {
    pub contract_address: String,
    pub function_name: String,
    pub function_params: Vec<String>,
    pub function_abi: ethabi::Function,
    pub chain: String,
    pub return_value_test: JsonReturnValueTestV2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonReturnValueTestV2 {
    pub key: String,
    pub comparator: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonAccessControlCondition {
    pub contract_address: String,
    pub chain: String,
    pub standard_contract_type: String,
    pub method: String,
    pub parameters: Vec<String>,
    pub return_value_test: JsonReturnValueTest,
}

pub type AccessControlConditionItem = ControlConditionItem<JsonAccessControlCondition>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AccessControlBooleanOperator {
    And,
    Or,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonAccessControlConditionOperator {
    pub operator: AccessControlBooleanOperator,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonReturnValueTest {
    pub comparator: String,
    pub value: String,
}

/* accessControlConditions looks like this:
accessControlConditions: [
{
contractAddress: tokenAddress,
chain: 'ethereum',
standardContractType: 'ERC1155',
method: 'balanceOf',
parameters: [
':userAddress',
tokenId
      ],
      returnValueTest: {
        comparator: '>',
        value: 0
      }
    }
  ]
*/

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtHeader {
    pub alg: String,
    pub typ: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    pub iss: String,
    pub sub: String,
    pub chain: String,
    pub iat: u64,
    pub exp: u64,
    pub base_url: String,
    pub path: String,
    pub org_id: String,
    pub role: String,
    pub extra_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayloadV2 {
    pub iss: String,
    pub sub: String,
    pub chain: Option<String>,
    pub iat: u64,
    pub exp: u64,
    pub access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    pub evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    pub sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    pub unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecoveryShare {
    pub recovery_share: [u8; 32],
}

impl SerdeEncryptPublicKey for RecoveryShare {
    type S = BincodeSerializer<Self>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonRecoveryShareResponse {
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerValidator {
    pub ip: u32,
    pub ipv6: u128,
    pub port: u32,
    pub address: Address,
    pub reward: U256,
    pub coms_sender_pub_key: U256,
    pub coms_receiver_pub_key: U256,
    pub index: u16,
    pub staker_address: Address,
    pub socket_addr: String,
    pub key_hash: u64,
    pub is_kicked: bool,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebAuthnAuthenticationRequest {
    pub credential: PublicKeyCredential,
    pub session_pubkey: String,
    pub siwe_message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignedData {
    pub sig_type: String,
    pub data_signed: String,
    pub signature_share: String,
    pub share_index: u32,
    pub big_r: String,
    pub public_key: String,
    pub sig_name: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct EthBlock {
    pub blockhash: String,
    pub timestamp: String,
    pub block_number: usize,
}
