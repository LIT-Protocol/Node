use std::collections::HashMap;

use super::models::{
    BeaverManager, BeaverTriplePair, TripleListByGroup, TripleListByGroupTrait,
    XorFilterWithThreshold,
};
use crate::config::beaver_triple_path;
use crate::error::{unexpected_err, Result};
use crate::peers::peer_state::models::{PeerSocketAddress, PeerSocketAddressVec};
use crate::tasks::beaver_manager::listener::addr_is_leader;
use crate::tss::common::storage::read_beaver_triple_from_disk_direct;
use async_recursion::async_recursion;
use async_std::fs::{self, DirEntry};
use async_std::io::Error;
use async_std::path::PathBuf;
use futures::StreamExt;
use xorf::Filter;
impl BeaverManager {
    pub async fn load_from_disk(&mut self, initial_load: bool) -> TripleListByGroup {
        match self.do_load_from_disk().await {
            Ok(triple_list) => triple_list,
            Err(e) => {
                match initial_load {
                    true => warn!("Initial BT loading returned (no path is normal): {}", e),
                    false => error!("Error loading triples from disk: {}", e),
                }
                TripleListByGroup::new()
            }
        }
    }

    async fn do_load_from_disk(&mut self) -> Result<TripleListByGroup> {
        let mut triple_list = TripleListByGroup::new();
        let mut peers = self.tss_state.peer_state.peer_socket_addresses().await?;
        let node_addr = self.tss_state.peer_state.addr.clone();
        // until we have peers, we can't really do anything
        while peers.is_empty() {
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
            peers = self.tss_state.peer_state.peer_socket_addresses().await?;
        }

        let path = beaver_triple_path();
        info!("Loading beaver triples from disk");
        self.recurse_dirs(path, &mut triple_list, &peers, &node_addr)
            .await?;

        Ok(triple_list)
    }

    #[async_recursion]
    async fn recurse_dirs(
        &mut self,
        path: PathBuf,
        triple_list: &mut TripleListByGroup,
        peers: &Vec<PeerSocketAddress>,
        node_addr: &String,
    ) -> Result<()> {
        let mut dirs = fs::read_dir(path)
            .await
            .map_err(|e| unexpected_err(e, Some("Beaver Triple Path not found.".into())))?;

        while let Some(res) = dirs.next().await {
            let entry = res
                .map_err(|e| unexpected_err(e, Some("Beaver Triple folder read error.".into())))?;
            let filetype = entry.file_type().await.map_err(|e| {
                unexpected_err(e, Some("Beaver Triple file type read error.".into()))
            })?;

            if filetype.is_dir() {
                let path = entry.path();
                self.recurse_dirs(path, triple_list, peers, node_addr)
                    .await?;
            } else if filetype.is_file() {
                self.attempt_load_beaver_triple(entry, triple_list, peers, node_addr)
                    .await?;
            }
        }
        Ok(())
    }

    async fn attempt_load_beaver_triple(
        &mut self,
        entry: DirEntry,
        triple_list: &mut TripleListByGroup,
        peers: &Vec<PeerSocketAddress>,
        node_addr: &String,
    ) -> Result<()> {
        let share_index = peers.index_of(node_addr)?;

        let filename = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(e) => {
                error!("Error reading filename: {:?}", e);
                return Err(unexpected_err(
                    Error::new(std::io::ErrorKind::Other, "file"),
                    Some("Beaver Triple filename read error.".into()),
                ));
            }
        };

        let entry_path = entry.path();
        let filename = match entry_path.to_str() {
            Some(s) => s,
            None => {
                error!("Error reading filename: {:?}", entry.path());
                return Err(unexpected_err(
                    Error::new(std::io::ErrorKind::Other, "file"),
                    Some("Beaver Triple filename read error.".into()),
                ));
            }
        };

        let share_ending = format! {"{}-H.cbor", share_index};
        if filename.ends_with(share_ending.as_str()) {
            let triple_pair =
                match read_beaver_triple_from_disk_direct::<BeaverTriplePair>(filename).await {
                    Ok(s) => s as BeaverTriplePair,
                    Err(e) => {
                        error!("Error reading triple file: {:?}", e);
                        return Err(e);
                    }
                };
            // self.current_triples += 1;
            let peer_group_id = triple_pair.peer_group_id;

            let triple_storage_key = Self::triple_key_from_triple_pair(&triple_pair);
            let triple_storage_key: u64 =
                match filename.split('-').collect::<Vec<&str>>()[3].parse() {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Error parsing triple storage key: {:?}", e);
                        return Err(unexpected_err(
                            Error::new(std::io::ErrorKind::Other, "file"),
                            Some("Beaver Triple filename read error.".into()),
                        ));
                    }
                };

            let xor_filter_with_threshold = XorFilterWithThreshold {
                filter: triple_pair.xor_filter,
                threshold: triple_pair.pub0.threshold,
            };

            self.xor_filters
                .entry(peer_group_id)
                .or_insert(xor_filter_with_threshold);

            let triple_creation_addresses = self
                .node_socket_addresses_from_peer_group_id(peer_group_id, peers)
                .await;

            if addr_is_leader(triple_storage_key, &triple_creation_addresses, node_addr) {
                triple_list.add_storage_key(peer_group_id, triple_storage_key);
            }

            self.current_generation_count += 1; // technically this is the "loaded" amount right now.  But it will soon be reset by one of the leaders.
        }
        Ok(())
    }

    pub fn get_peer_group_id_from_xor_filter(
        &self,
        triple_list: &TripleListByGroup,
        peers: &Vec<PeerSocketAddress>,
        threshold: usize,
    ) -> u64 {
        let keys = peers.peer_keys();

        for (peer_group_id, xor_filter_with_threshold) in self.xor_filters.iter() {
            let mut found = true;
            if xor_filter_with_threshold.threshold == threshold {
                for key in &keys {
                    if !xor_filter_with_threshold.filter.contains(key) {
                        found = false;
                        break;
                    }
                }
                if found {
                    // even if there is a peer_group, we need to check if it's empty
                    if let Some(s) = triple_list.get(peer_group_id) {
                        if !s.is_empty() {
                            return *peer_group_id;
                        }
                    }
                }
            }
        }
        0
    }

    pub async fn node_socket_addresses_from_peer_group_id(
        &self,
        peer_group_id: u64,
        all_peers: &Vec<PeerSocketAddress>,
    ) -> Vec<String> {
        let mut addresses = Vec::new();
        let xor_filter_with_threshold = match self.xor_filters.get(&peer_group_id) {
            Some(s) => s,
            None => return addresses,
        };

        for peer in all_peers {
            if xor_filter_with_threshold.filter.contains(&peer.key_hash) {
                addresses.push(peer.address.clone());
            }
        }
        addresses
    }
}

pub async fn staker_hashes_from_peer_group_id(
    peer_group_id: u64,
    xor_filters: HashMap<super::models::PeerGroupId, XorFilterWithThreshold>,
    all_peers: &Vec<PeerSocketAddress>,
) -> Vec<u64> {
    let mut peer_keys = Vec::new();
    let xor_filter_with_threshold = match xor_filters.get(&peer_group_id) {
        Some(s) => s,
        None => return peer_keys,
    };

    for peer in all_peers {
        if xor_filter_with_threshold.filter.contains(&peer.key_hash) {
            peer_keys.push(peer.key_hash);
        }
    }
    peer_keys
}
