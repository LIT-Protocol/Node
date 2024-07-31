use crate::peers::peer_state::models::SimplePeer;
use serde::{Deserialize, Serialize};
use serde_encrypt::{serialize::impls::BincodeSerializer, traits::SerdeEncryptPublicKey};
use std::time::SystemTime;
use xor_name::XorName;

use crate::peers::peer_reviewer::PeerComplaint;

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
    pub dest_peer: SimplePeer,
    pub round: String,
    pub node_transmission_entry: NodeTransmissionEntry,
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

#[derive(Debug, Clone)]
pub struct NodeWaitParams {
    pub txn_prefix: String,
    pub channels: Option<RoundCommsChannel>,
    pub tx_pr: flume::Sender<PeerComplaint>,
    pub round: String,
    pub share_index: u16,
    pub timeout: u64,
    pub exit_on_qty_recvd: Option<u16>,
}
