use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::error::{unexpected_err, Result};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerValidatorStatus {
    Entering, // Not in current, but in locked next
    Exiting,  // in current, but not in locked next
    Survivor, // in both
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NetworkState {
    Active = 0,
    NextValidatorSetLocked = 1,
    ReadyForNextEpoch = 2,
    Unlocked = 3,
    Paused = 4,
    Restore = 5,
    Unknown = 255,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PeerSocketAddress {
    pub address: String,
    pub share_index: u32,
    pub key_hash: u64, // Hash of the address
    pub kicked: bool,
}

impl std::convert::From<u8> for NetworkState {
    fn from(value: u8) -> Self {
        match value {
            0 => NetworkState::Active,
            1 => NetworkState::NextValidatorSetLocked,
            2 => NetworkState::ReadyForNextEpoch,
            3 => NetworkState::Unlocked,
            4 => NetworkState::Paused,
            5 => NetworkState::Restore,
            _ => NetworkState::Unknown,
        }
    }
}

pub trait PeerSocketAddressVec {
    fn active_peers(&self) -> Vec<PeerSocketAddress>;
    fn kicked_peers(&self) -> Vec<PeerSocketAddress>;
    fn index_of(&self, address: &str) -> Result<u32>;
    fn address_at(&self, index: u32) -> Result<String>;
    fn all_addresses(&self) -> Vec<String>;
    fn active_addresses(&self) -> Vec<String>;
    fn kicked_addresses(&self) -> Vec<String>;
    fn contains_address(&self, address: &str) -> bool;
    fn scoped_peers(&self, scope: &Vec<String>) -> Vec<PeerSocketAddress>;
    fn peer_keys(&self) -> Vec<u64>;
    fn peer_group_id(&self) -> u64;
}

impl PeerSocketAddressVec for Vec<PeerSocketAddress> {
    fn active_peers(&self) -> Vec<PeerSocketAddress> {
        self.iter()
            .filter(|p| !p.kicked)
            .cloned()
            .collect::<Vec<PeerSocketAddress>>()
    }

    fn kicked_peers(&self) -> Vec<PeerSocketAddress> {
        self.iter()
            .filter(|p| p.kicked)
            .cloned()
            .collect::<Vec<PeerSocketAddress>>()
    }

    fn all_addresses(&self) -> Vec<String> {
        self.iter()
            .map(|p| p.address.clone())
            .collect::<Vec<String>>()
    }

    fn active_addresses(&self) -> Vec<String> {
        self.iter()
            .filter(|p| !p.kicked)
            .map(|p| p.address.clone())
            .collect::<Vec<String>>()
    }

    fn kicked_addresses(&self) -> Vec<String> {
        self.iter()
            .filter(|p| p.kicked)
            .map(|p| p.address.clone())
            .collect::<Vec<String>>()
    }

    fn index_of(&self, address: &str) -> Result<u32> {
        for p in self {
            if p.address == address {
                return Ok(p.share_index);
            }
        }
        let msg = format!("Peer {} not found in peer list", address);
        Err(unexpected_err("Peer not found in peer list", Some(msg)))
    }

    fn address_at(&self, index: u32) -> Result<String> {
        for p in self {
            if p.share_index == index {
                return Ok(p.address.clone());
            }
        }
        let msg = format!("Peer {} not found in peer list", index);
        Err(unexpected_err("Peer not found in peer list", Some(msg)))
    }

    fn contains_address(&self, address: &str) -> bool {
        for p in self {
            if p.address == address {
                return true;
            }
        }
        false
    }

    fn scoped_peers(&self, scope: &Vec<String>) -> Vec<PeerSocketAddress> {
        self.iter()
            .filter(|p| scope.contains(&p.address))
            .cloned()
            .collect::<Vec<PeerSocketAddress>>()
    }

    fn peer_keys(&self) -> Vec<u64> {
        self.iter().map(|p| p.key_hash).collect::<Vec<u64>>()
    }

    fn peer_group_id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.peer_keys().hash(&mut hasher);
        hasher.finish()
    }
}
