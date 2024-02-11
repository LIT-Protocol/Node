use crate::error::{unexpected_err, Result};
use crate::tss::common::models::NodeTransmissionEntry;
use lit_core::utils::binary::bytes_to_hex;
use sha2::{Digest, Sha256};
use std::time::SystemTime;

pub fn hash_message_to_hex_str(message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    bytes_to_hex(hasher.finalize())
}

pub fn hash_message_bytes_to_hex_str(message_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message_bytes);
    bytes_to_hex(hasher.finalize())
}

pub fn random_txn_id() -> Result<String> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| unexpected_err(e, Some("Could not get current time".into())))?
        .as_millis()
        .to_string())
}

pub fn get_body_descriptor_for_node_transmission_entry(message: &NodeTransmissionEntry) -> String {
    message.key.clone()
}
