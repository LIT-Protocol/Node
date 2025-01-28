use crate::error::{unexpected_err, Result};
use crate::tss::common::models::NodeTransmissionBatchEntries;
use ethers::types::U256;
use lit_blockchain::contracts::staking::Version;

pub fn get_body_descriptor_for_node_transmission_batch_entries(
    message: &NodeTransmissionBatchEntries,
) -> Vec<String> {
    message.entries.iter().map(|e| e.key.clone()).collect()
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
