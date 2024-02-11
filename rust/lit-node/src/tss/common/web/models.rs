use crate::auth::auth_material::JsonAuthSig;
use crate::models::{AccessControlConditionItem, EVMContractConditionItem, SolRpcConditionItem};
use debug_ignore::DebugIgnore;
use lit_core::config::LitConfig;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// know static public string

#[derive(Serialize, Deserialize)]
pub struct ValidateConditionRequest {
    pub access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    pub evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    pub sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    pub chain: String,
    pub auth_sig: JsonAuthSig,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignedMessageShare {
    pub digest: String,
    pub result: String,
    pub share_index: usize,
    pub signature_share: String,
    pub big_r: String,
    pub public_key: String,
    pub sig_type: String,
}

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub addr: String,
    pub current_index: u16, // this is the index within the current set of peers.  it may or may not be the original dkg index
    pub share_index: u16,   // this is the value of the original dkg index including all peers
    pub txn_prefix: String,
    pub peers: Vec<String>,
    pub peer_indexes: Vec<u16>,
    pub lit_config: DebugIgnore<Arc<LitConfig>>,
    pub threshold: u16,
    pub total_shares: u16,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PKPKeyRequest {
    pub id: String,
    pub chain: String,
    pub key_type: String,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PKPKeyResponse {
    pub chain: String,
    pub public_key: String,
    pub signature: String,
    pub key_type: String,
    pub signature_r: ethers::types::U256,
    pub signature_s: ethers::types::U256,
    pub signature_v: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignWithPublicHashRequest {
    pub pubkey: Vec<u8>,
    pub auth_sig: JsonAuthSig,
    pub iat: u64,
    pub exp: u64,
}
