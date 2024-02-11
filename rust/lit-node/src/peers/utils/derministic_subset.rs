use crate::error::{unexpected_err, Result};
use crate::peers::peer_state::models::{PeerSocketAddress, PeerSocketAddressVec};
use crate::peers::PeerState;
use crate::tasks::beaver_manager::models::generate_hash;
use rand::seq::SliceRandom;
use rand::{rngs::StdRng, SeedableRng};
use std::time::SystemTime;

#[derive(Debug)]
pub struct DeterministicSubset {
    pub all_peers: Vec<PeerSocketAddress>, // list of nodes, by index >>> replace this with complaints?
    pub created_at: SystemTime,
}

impl DeterministicSubset {
    pub async fn new(peer_state: &PeerState) -> DeterministicSubset {
        let all_peers = match peer_state.peer_socket_addresses().await {
            Ok(peers) => peers,
            Err(e) => {
                error!("Error getting connected node addresses: {}", e);
                Vec::new()
            }
        };

        DeterministicSubset {
            all_peers,
            created_at: SystemTime::now(),
        }
    }

    #[allow(dead_code)] // used for a component test.
    pub fn new_from_peer_sets(all_peers: Vec<PeerSocketAddress>) -> DeterministicSubset {
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
    ) -> Result<Vec<PeerSocketAddress>> {
        if self.all_peers.is_empty() {
            return Err(unexpected_err(
                "No peers available to create a subset",
                None,
            ));
        }

        if threshold == self.all_peers.len() {
            return Ok(self.all_peers.clone());
        }

        if self.all_peers.active_addresses().len() < threshold {
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
            .collect::<Vec<PeerSocketAddress>>();

        shuffled_subset.sort_by(|a, b| a.share_index.cmp(&b.share_index));

        Ok(shuffled_subset)

        // let mut peer_subset = self.all_peers.clone();
        // peer_subset.retain(|x| to_keep.contains(x));

        // Ok(peer_subset)
    }
}
