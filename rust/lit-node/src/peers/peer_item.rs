use crate::error::Result;
use ethers::types::Address;
use libsecp256k1::PublicKey;
use lit_attestation::Attestation;
use lit_core::error::Unexpected;
use log::warn;
use rocket::serde::{Deserialize, Serialize};
use std::time::Instant;
use xor_name::XorName;

use super::peer_state::models::PeerValidatorStatus;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerItem {
    pub id: XorName,
    pub addr: String,
    pub public_key: PublicKey,
    pub node_address: Address,
    pub sender_public_key: [u8; 32], // SenderPublicKey does not impl Deserialize
    pub receiver_public_key: [u8; 32], // ReceiverPublicKey does not impl Deserialize
    pub staker_address: Address,     // address of staking wallet
    pub status: PeerValidatorStatus,
    pub attestation: Option<Attestation>,
}
impl PeerItem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        addr: &str,
        public_key: PublicKey,
        node_address: Address,
        sender_public_key: [u8; 32],
        receiver_public_key: [u8; 32],
        staker_address: Address,
        status: PeerValidatorStatus,
        attestation: Option<Attestation>,
    ) -> Self {
        PeerItem {
            id: XorName::from_content(addr.as_bytes()),
            addr: addr.into(),
            public_key,
            node_address,
            sender_public_key,
            receiver_public_key,
            staker_address,
            status,
            attestation,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PeerData {
    pub table: Vec<PeerItem>,
    last_updated: Instant, // Last updated, does not currently guarantee it matched a snapshot at that time
                           // indices can be put here if we have enough nodes that a BTree would be faster, and
                           // the linear search in the impl changed to use the index.
}

impl Default for PeerData {
    fn default() -> Self {
        Self::new()
    }
}

impl PeerData {
    pub fn new() -> Self {
        Self {
            table: Vec::new(),
            last_updated: Instant::now(),
        }
    }

    pub fn get_peer_by_peer_id(&self, peer_id: &XorName) -> Option<PeerItem> {
        if let Some(ix) = self.get_index_by_peer_id(peer_id) {
            self.table.get(ix).cloned()
        } else {
            None
        }
    }
    pub fn get_peer_by_addr(&self, addr: &str) -> Option<PeerItem> {
        if let Some(ix) = self.get_index_by_addr(addr) {
            self.table.get(ix).cloned()
        } else {
            None
        }
    }
    pub fn get_peer_by_public_key(&self, public_key: &PublicKey) -> Option<PeerItem> {
        if let Some(ix) = self.get_index_by_public_key(public_key) {
            self.table.get(ix).cloned()
        } else {
            None
        }
    }
    pub fn get_peer_by_staker_address(&self, staker_address: &Address) -> Option<PeerItem> {
        if let Some(ix) = self.get_index_by_staker_address(staker_address) {
            self.table.get(ix).cloned()
        } else {
            None
        }
    }

    /// return the first matching index
    pub fn get_index_by_peer_id(&self, peer_id: &XorName) -> Option<usize> {
        self.table.iter().position(|pi| pi.id == *peer_id)
    }
    /// return the first matching index
    pub fn get_index_by_addr(&self, addr: &str) -> Option<usize> {
        self.table.iter().position(|pi| pi.addr == addr)
    }
    /// return the first matching index
    pub fn get_index_by_public_key(&self, public_key: &PublicKey) -> Option<usize> {
        self.table
            .iter()
            .position(|pi| pi.public_key == *public_key)
    }
    /// return the first matching index
    pub fn get_index_by_staker_address(&self, staker_address: &Address) -> Option<usize> {
        self.table
            .iter()
            .position(|pi| pi.staker_address == *staker_address)
    }
    pub fn check_age(&self) -> bool {
        if self.last_updated.elapsed().as_secs() > 15 {
            warn!("PeerData was over 15s old");
            false
        } else {
            true
        }
    }

    // Upsert
    pub fn insert(&mut self, peeritem: PeerItem) -> Result<()> {
        let current = self.get_peer_by_peer_id(&peeritem.id);
        match current {
            None => self.table.push(peeritem),
            Some(peer) => {
                if peer != peeritem {
                    let ix = self
                        .get_index_by_peer_id(&peeritem.id)
                        .expect_or_err("index error")?;
                    self.table.push(peeritem);
                    self.remove_by_index(ix);
                }
                // else ignore
            }
        }

        self.last_updated = Instant::now();

        Ok(())
    }

    pub fn remove_by_index(&mut self, index: usize) -> PeerItem {
        self.last_updated = Instant::now();
        self.table.swap_remove(index)
    }

    pub fn table(&self) -> &Vec<PeerItem> {
        &self.table
    }

    pub fn clear_table(&mut self) {
        self.table.clear();
    }
}

#[cfg(test)]
mod tests {
    use ethers::prelude::rand::rngs::OsRng;
    use ethers::{prelude::k256::ecdsa::SigningKey, types::Address};
    use hex_literal::hex as hexl;
    use libsecp256k1::PublicKey;
    use serde_encrypt::{
        key::key_pair::{ReceiverKeyPair, SenderKeyPair},
        ReceiverKeyPairCore, SenderKeyPairCore,
    };
    use xor_name::XorName;

    use super::{PeerData, PeerItem, PeerValidatorStatus};

    fn make_fake_peer(i: usize) -> PeerItem {
        let secret_key = SigningKey::random(&mut OsRng);
        let public_key = secret_key.verifying_key();
        let mut fixed_size: [u8; 33] = [0; 33];
        fixed_size.copy_from_slice(&public_key.to_sec1_bytes());
        let public_key = PublicKey::parse_compressed(&fixed_size)
            .expect("Could not convert VerifyingKey to PublicKey");
        let sender_public_key = *SenderKeyPair::generate().public_key().as_ref().as_bytes();
        let receiver_public_key = *ReceiverKeyPair::generate().public_key().as_ref().as_bytes();
        let addr: String = format!("example{}.com", i);
        let id = XorName::from_content(addr.clone().as_bytes());
        let staker_address: Address = hexl!("f39fd6e51aad88f6f4ce6ab8827279cfffb92266").into();

        PeerItem {
            id,
            addr,
            public_key,
            node_address: staker_address,
            sender_public_key,
            receiver_public_key,
            //            staking_address: self.staking_address.swap_remove(index),
            staker_address,
            status: PeerValidatorStatus::Unknown,
            attestation: None,
        }
    }

    #[test]
    fn test_peer_data_get_all_peers() {
        let mut pd = PeerData::new();
        let num_peers = 6;
        for i in 0..num_peers {
            let peer = make_fake_peer(i);
            pd.insert(peer).unwrap();
        }

        let all_peers = pd.table;
        assert_eq!(all_peers.len(), num_peers);
    }

    #[test]
    fn test_peer_table_size() {
        let mut pd = PeerData::new();
        let num_peers = 6;
        for i in 0..num_peers {
            let peer = make_fake_peer(i);
            pd.insert(peer).unwrap();
        }

        assert_eq!(pd.table.len(), num_peers);
    }

    #[test]
    fn peer_table_test() {
        let mut pd = PeerData::new();
        for i in 0..6 {
            let peer = make_fake_peer(i);
            pd.insert(peer).unwrap();
        }
        let peer2 = pd.remove_by_index(2);
        assert_eq!(peer2.addr, "example2.com");
        let five = format!("example{}.com", 5);
        assert_eq!(pd.get_peer_by_addr(&five).unwrap().addr, five);
        let oldthree = pd.get_peer_by_addr("example3.com").unwrap();
        let newthree = make_fake_peer(3);
        pd.insert(newthree).unwrap();
        assert_ne!(oldthree, pd.get_peer_by_addr("example3.com").unwrap());
        assert_eq!(pd.get_peer_by_addr(&five).unwrap().addr, five);
    }
}
