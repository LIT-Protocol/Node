use crate::error::{unexpected_err, Result};
use crate::peers::peer_state::models::SimplePeer;
use crate::tss::common::models::NodeTransmissionBatchEntries;
use ethers::types::U256;
use lit_blockchain::contracts::staking::Version;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tracing::instrument;

pub fn get_body_descriptor_for_node_transmission_batch_entries(
    message: &NodeTransmissionBatchEntries,
) -> Vec<String> {
    message.entries.iter().map(|e| e.key.clone()).collect()
}

// functions that deal with leader selection.
pub fn generate_hash<T: Hash>(input: T) -> u64 {
    let mut s = DefaultHasher::new();
    input.hash(&mut s);
    s.finish()
}

#[instrument(skip_all)]
pub fn addr_is_leader(request: u64, peers: &Vec<SimplePeer>, addr: &String) -> bool {
    let leader_address = match leader_addr(request, peers) {
        Ok(addr) => addr,
        Err(e) => {
            error!("Error getting leader address: {}", e);
            return false;
        }
    };

    addr == leader_address
}

#[instrument(skip_all)]
pub fn leader_addr(request: u64, peers: &Vec<SimplePeer>) -> Result<&String> {
    let leader_hash = generate_hash(request);
    let group_size = match peers.len() {
        0 => {
            return Err(unexpected_err("No peers found in leader_addr.", None));
        }
        size => size,
    };
    let leader_id = leader_hash % group_size as u64;
    Ok(&peers[leader_id as usize].socket_address)
}

pub trait SimpleHash {
    fn hash(&self) -> u64;
}

pub fn parse_version(version: &str) -> Result<Version> {
    let version_parts = version.split('.').collect::<Vec<&str>>();
    Ok(Version {
        major: U256::from_dec_str(version_parts[0])
            .map_err(|e| unexpected_err(e, Some("Failed to parse major version.".into())))?,
        minor: U256::from_dec_str(version_parts[1])
            .map_err(|e| unexpected_err(e, Some("Failed to parse minor version.".into())))?,
        patch: U256::from_dec_str(version_parts[2])
            .map_err(|e| unexpected_err(e, Some("Failed to parse patch version.".into())))?,
    })
}
