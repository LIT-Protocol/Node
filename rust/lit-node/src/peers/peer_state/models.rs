use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{
    error::{unexpected_err, Result},
    models::PeerValidator,
    utils::consensus::get_threshold_count,
};
use ethers::types::H160;
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Ord, PartialOrd)]
pub struct SimplePeer {
    pub socket_address: String,
    pub share_index: u16,
    pub protocol_index: Option<u16>,
    pub staker_address: H160,
    pub key_hash: u64,
    pub kicked: bool,
    pub version: semver::Version,
}

// This is accurate for the current implementation
// the protocol index might change, but it would still represent the same peer.
// the key hash is a deterministict value of the address.
impl std::cmp::PartialEq for SimplePeer {
    fn eq(&self, other: &Self) -> bool {
        self.key_hash == other.key_hash
            && self.socket_address == other.socket_address
            && self.share_index == other.share_index
            && self.kicked == other.kicked
    }
}

impl SimplePeer {
    pub fn get_protocol_index(&self) -> Result<u16> {
        match self.protocol_index {
            Some(i) => Ok(i),
            None => Err(unexpected_err(
                "Protocol index not set",
                Some(format!("Peer: {}", self.socket_address)),
            )),
        }
    }
}

impl From<&PeerValidator> for SimplePeer {
    fn from(validator: &PeerValidator) -> Self {
        let version = match semver::Version::parse(&validator.version) {
            Ok(v) => v,
            Err(_) => semver::Version::new(0, 0, 0),
        };

        SimplePeer {
            socket_address: validator.socket_addr.clone(),
            share_index: validator.index,
            kicked: validator.is_kicked,
            staker_address: validator.staker_address,
            key_hash: validator.key_hash,
            protocol_index: None,
            version,
        }
    }
}

pub trait SimplePeerExt {
    fn active_peers(&self) -> Vec<SimplePeer>;
    fn share_index(&self, address: &str) -> Result<u16>;
    fn contains_address(&self, address: &str) -> bool;
    fn all_peers_except(&self, address: &str) -> Vec<SimplePeer>;
    fn peer_keys(&self) -> Vec<u64>;
    fn peer_group_id(&self) -> u64;
    fn debug_addresses(&self) -> String;
    fn get_threshold_count(&self) -> u16;
    fn peer_at_address(&self, address: &str) -> Result<SimplePeer>;
    fn peer_at_share_index(&self, share_index: u16) -> Result<SimplePeer>;
    fn peer_at_protocol_index(&self, protocol_index: u16) -> Result<SimplePeer>;
    fn set_all_protocol_indices(&mut self, offset: u16);
    fn set_protocol_index_for_share_index(&mut self, share_index: u16, protocol_index: u16);
    fn share_indices(&self) -> Vec<u16>;
    fn threshold_for_set(&self) -> u16;
    fn min_version_in_group(&self) -> semver::Version;
    fn leader_for_active_peers(&self, hash_key: u64) -> Result<SimplePeer>;
    fn address_is_leader(&self, hash_key: u64, addr: &String) -> bool;
    fn generate_hash<T: Hash>(input: T) -> u64;
}

impl SimplePeerExt for Vec<SimplePeer> {
    fn active_peers(&self) -> Vec<SimplePeer> {
        self.iter()
            .filter(|p| !p.kicked)
            .cloned()
            .collect::<Vec<SimplePeer>>()
    }

    fn share_index(&self, address: &str) -> Result<u16> {
        for p in self {
            if p.socket_address == address {
                return Ok(p.share_index);
            }
        }
        let msg = format!("Peer {} not found in {}", address, self.debug_addresses());
        Err(unexpected_err(
            "Peer not found in peer list (share_index)",
            Some(msg),
        ))
    }

    fn contains_address(&self, address: &str) -> bool {
        for p in self {
            if p.socket_address == address {
                return true;
            }
        }
        false
    }

    fn all_peers_except(&self, address: &str) -> Vec<SimplePeer> {
        self.iter()
            .filter(|p| p.socket_address != address)
            .cloned()
            .collect::<Vec<SimplePeer>>()
    }

    fn peer_keys(&self) -> Vec<u64> {
        self.iter().map(|p| p.key_hash).collect::<Vec<u64>>()
    }

    fn peer_group_id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.peer_keys().hash(&mut hasher);
        hasher.finish()
    }

    fn get_threshold_count(&self) -> u16 {
        get_threshold_count(self.active_peers().len()) as u16
    }

    fn debug_addresses(&self) -> String {
        let mut addresses = String::new();
        for p in self {
            if !addresses.is_empty() {
                addresses.push_str(", ");
            }
            addresses.push_str(&format!(
                "{} [{}],[{:?}] v:{} ",
                p.socket_address, p.share_index, p.protocol_index, p.version
            ));
        }
        addresses
    }

    fn peer_at_address(&self, address: &str) -> Result<SimplePeer> {
        for p in self {
            if p.socket_address == address {
                return Ok(p.clone());
            }
        }
        let msg = format!("Peer: {}", address);
        Err(unexpected_err(
            "Peer not found in peer list (peer_at_address)",
            Some(msg),
        ))
    }

    fn peer_at_share_index(&self, share_index: u16) -> Result<SimplePeer> {
        for p in self {
            if p.share_index == share_index {
                return Ok(p.clone());
            }
        }
        let msg = format!("Peer: {}", share_index);
        Err(unexpected_err(
            "Peer not found in peer list (peer_at_share_index)",
            Some(msg),
        ))
    }

    fn peer_at_protocol_index(&self, protocol_index: u16) -> Result<SimplePeer> {
        for p in self {
            match p.protocol_index {
                Some(i) => {
                    if i == protocol_index {
                        return Ok(p.clone());
                    }
                }
                None => {
                    continue;
                }
            }
        }
        let msg = format!(
            "Peer {} not found in peer list (peer_at_protocol_index) in {:?}",
            protocol_index, &self
        );
        let msg = msg.as_str();
        Err(unexpected_err(msg, None))
    }

    fn set_all_protocol_indices(&mut self, offset: u16) {
        for p in self {
            p.protocol_index = Some(p.share_index + offset);
        }
    }

    fn set_protocol_index_for_share_index(&mut self, share_index: u16, protocol_index: u16) {
        for p in self {
            if p.share_index == share_index {
                p.protocol_index = Some(protocol_index);
                return;
            }
        }
    }

    fn share_indices(&self) -> Vec<u16> {
        self.iter().map(|p| p.share_index).collect::<Vec<u16>>()
    }

    fn threshold_for_set(&self) -> u16 {
        get_threshold_count(self.len()) as u16
    }

    fn min_version_in_group(&self) -> semver::Version {
        let mut min_version = semver::Version::new(u64::MAX, 0, 0);
        for p in self {
            if p.version < min_version {
                min_version = p.version.clone();
            }
        }
        min_version
    }

    fn leader_for_active_peers(&self, hash_key: u64) -> Result<SimplePeer> {
        let leader_hash = Self::generate_hash(hash_key);
        let group_size = match self.len() {
            0 => {
                return Err(unexpected_err("No peers found in leader_addr.", None));
            }
            size => size,
        };
        let leader_id = leader_hash % group_size as u64;
        Ok(self[leader_id as usize].clone())
    }

    fn address_is_leader(&self, hash_key: u64, addr: &String) -> bool {
        let leader = match self.leader_for_active_peers(hash_key) {
            Ok(leader) => leader,
            Err(e) => {
                tracing::error!("Error getting leader: {}", e);
                return false;
            }
        };

        addr == &leader.socket_address
    }

    fn generate_hash<T: Hash>(input: T) -> u64 {
        let mut s = DefaultHasher::new();
        input.hash(&mut s);
        s.finish()
    }
}
