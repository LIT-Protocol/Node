use anyhow::Result;

use lit_node::{config::chain::ChainDataConfigManager, peers::peer_state::models::SimplePeer};

use super::testnet::actions::Actions;

pub async fn get_sorted_peers(actions: &Actions) -> Result<Vec<SimplePeer>> {
    // Get the validators in the current epoch.
    let current_validators = actions.get_current_validator_structs().await;
    let node_addresses = current_validators
        .iter()
        .map(|v| v.node_address)
        .collect::<Vec<_>>();
    // Get the kicked validators
    let kicked_validators = actions
        .contracts()
        .staking
        .get_kicked_validators()
        .await
        .map_err(|e| anyhow::anyhow!("failed to get kicked validators: {:?}", e))?;

    // Get the address mapping.
    let address_mapping = actions
        .contracts()
        .staking
        .get_node_staker_address_mappings(node_addresses)
        .call()
        .await
        .expect("failed to get node staker address mappings");

    // Get the sorted peers.
    let sorted_validators = ChainDataConfigManager::sort_and_filter_validators(
        current_validators,
        kicked_validators,
        address_mapping,
    );

    // Map into peer socket addresses.
    Ok(sorted_validators.iter().map(SimplePeer::from).collect())
}
