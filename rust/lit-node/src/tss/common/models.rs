use serde::{Deserialize, Serialize};
use serde_encrypt::{serialize::impls::BincodeSerializer, traits::SerdeEncryptPublicKey};
use std::time::SystemTime;
use xor_name::XorName;

use crate::tss::common::web::models::NodeConfig;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeTransmissionEntry {
    pub key: String,
    pub src_index: u16,
    pub dest_index: u16,
    pub value: Vec<u8>,
    pub timestamp: u128,
}

impl SerdeEncryptPublicKey for NodeTransmissionEntry {
    type S = BincodeSerializer<Self>;
}

#[derive(Clone, Debug)]
pub struct NodeTransmissionDetails {
    pub peer_address: String,
    pub node_config: NodeConfig,
    pub dest_index: usize,
    pub round: String,
    pub node_transmission_entry: NodeTransmissionEntry,
    pub is_batch: bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeTransmissionBatchEntries {
    pub entries: Vec<NodeTransmissionEntry>,
}

impl SerdeEncryptPublicKey for NodeTransmissionBatchEntries {
    type S = BincodeSerializer<Self>;
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeShareSetRequest {
    pub sender_id: XorName,
    pub encrypted_entry: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeShareSetResponse {
    pub success: bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeShareBatchSetRequest {
    pub sender_id: XorName,
    pub encrypted_entries: Vec<u8>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeResponse {
    pub nodeindex: u16,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct RoundsShareSet {
    pub key: String,
    pub value: Vec<u8>,
    pub channel_id: String,
    pub from_index: u16,
    pub retry: u16,
    pub created: SystemTime,
}

#[derive(Debug, Clone)]
pub struct RoundRegistration {
    pub id: String,
    pub channels: Option<RoundCommsChannel>,
}

pub enum RoundCommand {
    IncomingData,
    AddChannel,
    RemoveChannel,
    Heartbeat,
}

pub struct RoundData {
    pub command: RoundCommand,
    pub round_registration: Option<RoundRegistration>,
    pub round_share_set: Option<RoundsShareSet>,
}

#[derive(Debug, Clone)]
pub struct RoundCommsChannel {
    pub tx: flume::Sender<RoundsShareSet>,
    pub rx: flume::Receiver<RoundsShareSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZgPaillierDK {
    pub p: String,
    pub q: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZgPoint {
    pub curve: String,
    #[serde(with = "serde_bytes")]
    pub point: Vec<u8>, //33
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZgScalar {
    pub curve: String,
    #[serde(with = "serde_bytes")]
    pub scalar: Vec<u8>, //32
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZgKeysLinear {
    pub y: ZgPoint,
    pub x_i: ZgScalar,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct ZgPaillierKey {
    n: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
pub struct ZgH1H2NTilde {
    #[serde(with = "serde_bytes")]
    pub N: Vec<u8>, //256
    #[serde(with = "serde_bytes")]
    pub g: Vec<u8>, //256
    #[serde(with = "serde_bytes")]
    pub ni: Vec<u8>, //256
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZgVssScheme {
    pub parameters: ZgParameters,
    pub commitments: Vec<ZgPoint>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZgParameters {
    pub threshold: u16,
    pub share_count: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZgKeyShare {
    pub paillier_dk: ZgPaillierDK,
    pub pk_vec: Vec<ZgPoint>,
    pub keys_linear: ZgKeysLinear,
    pub paillier_key_vec: Vec<ZgPaillierKey>,
    pub y_sum_s: ZgPoint,
    pub h1_h2_n_tilde_vec: Vec<ZgH1H2NTilde>,
    pub vss_scheme: ZgVssScheme,
    pub i: u16,
    pub t: u16,
    pub n: u16,
}
