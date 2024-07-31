use crate::error::{unexpected_err, Result};
use crate::peers::peer_state::models::{SimplePeer, SimplePeerExt};
use crate::peers::PeerState;
use crate::tasks::utils::generate_hash;
use rand::seq::SliceRandom;
use rand::{rngs::StdRng, SeedableRng};
use std::time::SystemTime;

#[derive(Debug)]
pub struct DeterministicSubset {
    pub all_peers: Vec<SimplePeer>, // list of nodes, by index >>> replace this with complaints?
    pub created_at: SystemTime,
}

impl DeterministicSubset {
    pub async fn new(peer_state: &PeerState, epoch_number: u64) -> DeterministicSubset {
        let epoch_peers = match epoch_number == peer_state.epoch().await {
            true => peer_state.peers().await,
            false => peer_state.peers_in_prior_epoch_current_intersection().await,
        };

        let all_peers = match epoch_peers {
            Ok(peers) => peers,
            Err(e) => {
                error!(
                    "Error getting epohc {} connected node addresses: {}",
                    epoch_number, e
                );
                Vec::new()
            }
        };

        DeterministicSubset {
            all_peers,
            created_at: SystemTime::now(),
        }
    }

    #[allow(dead_code)] // used for a component test.
    pub fn new_from_peer_sets(all_peers: Vec<SimplePeer>) -> DeterministicSubset {
        DeterministicSubset {
            all_peers,
            created_at: SystemTime::now(),
        }
    }

    pub fn get_subset(
        &self,
        message_bytes: &[u8],
        request_id: &[u8],
        threshold: usize,
    ) -> Result<Vec<SimplePeer>> {
        if self.all_peers.is_empty() {
            return Err(unexpected_err(
                "No peers available to create a subset",
                None,
            ));
        }

        if threshold == self.all_peers.len() {
            return Ok(self.all_peers.clone());
        }

        if self.all_peers.active_peers().len() < threshold {
            return Err(unexpected_err(
                "Threshold is greater than the number of available peers",
                None,
            ));
        }

        let mut to_hash = message_bytes.to_vec();
        to_hash.extend_from_slice(request_id);
        let message_hash = generate_hash(&to_hash);
        let mut rng = StdRng::seed_from_u64(message_hash);

        let mut shuffled_subset = self.all_peers.active_peers().clone();
        debug!("Subset before shuffling: {:?}", shuffled_subset);
        // remove bad peers from the list

        shuffled_subset.shuffle(&mut rng);
        debug!("Shuffled subset: {:?}", shuffled_subset);

        let mut shuffled_subset = shuffled_subset
            .iter()
            .take(threshold)
            .cloned()
            .collect::<Vec<SimplePeer>>();

        shuffled_subset.sort_by(|a, b| a.share_index.cmp(&b.share_index));

        Ok(shuffled_subset)

        // let mut peer_subset = self.all_peers.clone();
        // peer_subset.retain(|x| to_keep.contains(x));

        // Ok(peer_subset)
    }
}
