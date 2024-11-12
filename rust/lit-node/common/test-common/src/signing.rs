use anyhow::Result;
use ethers::utils::keccak256;
use lit_node::peers::{
    peer_state::models::SimplePeer, utils::derministic_subset::DeterministicSubset,
};
use rand::Rng;
use tracing::debug;

use crate::peers::get_sorted_peers;

use super::testnet::actions::Actions;

/// This function returns a random `node_collection` index within the deterministic subset.
pub async fn get_random_peer_within_deterministic_subset(
    to_sign: &String,
    request_id: &String,
    threshold: usize,
    actions: &Actions,
) -> Result<SimplePeer> {
    // Get the message to send.
    let to_sign: Vec<u8> = keccak256(to_sign).into();

    // Get the sorted peers from chain.
    let sorted_validators = get_sorted_peers(actions)
        .await
        .map_err(|e| anyhow::anyhow!("failed to get sorted peers: {:?}", e))?;

    // Get the deterministic subset.
    let ds = DeterministicSubset::new_from_peer_sets(sorted_validators)
        .get_subset(
            &to_sign,
            request_id.to_owned().into_bytes().as_ref(),
            threshold,
        )
        .map_err(|e| anyhow::anyhow!("failed to get deterministic subset: {:?}", e))?;
    debug!("Deterministic subset: {:?}", ds);

    // Get a random address within the deterministic subset.
    let mut rng = rand::thread_rng();
    let random_idx = rng.gen_range(0..ds.len());
    let random_peer = ds.get(random_idx).expect("failed to get random peer");
    debug!(
        "Random peer within deterministic subset chosen: {:?}",
        random_peer
    );

    Ok(random_peer.to_owned())
}
