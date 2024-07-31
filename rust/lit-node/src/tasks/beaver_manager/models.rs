use cait_sith::triples::{TriplePub, TripleShare};
use k256::Secp256k1;
use reqwest::Client;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;
use std::sync::Arc;
use std::{collections::VecDeque, hash::Hash};

use crate::error::{self, unexpected_err, Result};
use crate::peers::peer_state::models::{SimplePeer, SimplePeerExt};
use crate::tss::common::tss_state::TssState;
use crate::tss::ecdsa_cait_sith::CsEcdsaState;
use flume::{Receiver, Sender};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct XorFilterWithThreshold {
    pub filter: xorf::BinaryFuse16,
    pub threshold: usize,
}

#[derive(Debug)]
pub struct BeaverManager {
    pub min_triples: u64,
    pub max_triples: u64,
    pub max_triple_concurrency: u64,
    pub rx: Receiver<BeaverMessage>,
    pub tx: Sender<BeaverMessage>,
    pub tss_state: Arc<TssState>,
    pub http_client: Client,
    pub signing_state: CsEcdsaState,
    pub current_generation_count: u64,
    pub generating_txn_ids: Vec<u64>,
    pub last_generated: std::time::Instant, // used to throttle generation
    pub xor_filters: HashMap<PeerGroupId, XorFilterWithThreshold>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeaverTriplePeerId {
    pub staker_hash: u64,
    pub peer_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeaverTriplePair {
    pub share_index: u16,
    pub share0: TripleShare<Secp256k1>,
    pub pub0: TriplePub<Secp256k1>,
    pub share1: TripleShare<Secp256k1>,
    pub pub1: TriplePub<Secp256k1>,
    // hash of the peers used to generate the pairs
    pub peer_group_id: PeerGroupId,
    pub xor_filter: xorf::BinaryFuse16,
    // added to use across epochs (staker_hash & peer_ids)
    pub staker_hash: u64,
    pub peer_ids: Vec<BeaverTriplePeerId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripleRequest {
    pub message_bytes: Vec<u8>,
    pub public_key: Vec<u8>,
    pub request_id: Vec<u8>,
    pub txn_prefix: String,
    pub peers: Vec<SimplePeer>,
    pub threshold: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct TripleRequestKey {
    pub message_bytes: Vec<u8>,
    pub public_key: Vec<u8>,
    pub request_id: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct RequestMapResponse {
    pub triple_map_item: TripleMapItem,
    pub in_request_map: bool,
}

#[derive(Debug)]
pub enum BeaverMessage {
    Generate(u64),
    Clear,
    Store(Box<BeaverTriplePair>, u64),
    RequestTriple(
        TripleRequest,
        Sender<error::Result<Option<BeaverTriplePair>>>,
    ),
    RemoteRequestForStorageKey(TripleRequest, Sender<TripleLeaderResponse>),
    FullfillTripleRequest(
        TripleRequestKeyHash,
        TripleLeaderResponse,
        Vec<SimplePeer>,
        Sender<error::Result<Option<BeaverTriplePair>>>,
    ),
    RemoveGenerationHash(u64),
    ThresholdReceived(
        TripleRequest,
        Sender<error::Result<Option<BeaverTriplePair>>>,
    ),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TripleLeaderResponse {
    pub triple_storage_key: TripleStorageKey,
    pub remaining_triple_pairs: u64,
}

// hash of a triple request key
pub type TripleRequestKeyHash = u64;
// hash of peers used to generate a triple
pub type PeerGroupId = u64;
// hash of the first triple in a triple pair
pub type TripleStorageKey = u64;
pub type TripleList = VecDeque<u64>;
// list of triples
pub type TripleListByGroup = HashMap<PeerGroupId, TripleList>;

pub type LeaderTripleRequestRemaining = u64;

pub type ActiveTripleMap = HashMap<TripleRequestKeyHash, TripleMapItem>;
#[derive(Debug, Clone)]
pub struct TripleMapItem {
    pub triple_key: u64,
    pub threshold: u16,
    pub remaining_triple_pairs: u64,
    pub created_at: u64,
}

impl TripleMapItem {
    pub fn zero() -> Self {
        TripleMapItem {
            triple_key: 0,
            threshold: 0,
            remaining_triple_pairs: 0,
            created_at: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    pub fn to_leader_response(&self) -> TripleLeaderResponse {
        TripleLeaderResponse {
            triple_storage_key: self.triple_key,
            remaining_triple_pairs: self.remaining_triple_pairs,
        }
    }
}

pub trait TripleListByGroupTrait {
    fn add_storage_key(&mut self, key: PeerGroupId, value: TripleStorageKey);
    fn total_shares_count(&self) -> u64;
    fn shares_count_for_peerset(&self, key: u64) -> u64;
    fn triple_list_values(&self) -> VecDeque<u64>;
}

impl TripleListByGroupTrait for TripleListByGroup {
    fn add_storage_key(&mut self, key: PeerGroupId, value: TripleStorageKey) {
        self.entry(key)
            .and_modify(|v| v.push_back(value))
            .or_insert(VecDeque::from(vec![value]));
    }

    fn total_shares_count(&self) -> u64 {
        let mut total = 0;
        for (_, v) in self.iter() {
            total += v.len() as u64;
        }
        total
    }
    fn shares_count_for_peerset(&self, key: u64) -> u64 {
        return match self.get(&key) {
            None => 0,
            Some(v) => v.len() as u64,
        };
    }

    fn triple_list_values(&self) -> VecDeque<u64> {
        self.values().flatten().cloned().collect()
    }
}

pub trait SimpleHash {
    fn hash(&self) -> u64;
}

impl SimpleHash for TripleRequestKey {
    fn hash(&self) -> u64 {
        generate_hash(self)
    }
}

pub fn generate_hash<T: Hash>(input: T) -> u64 {
    let mut s = DefaultHasher::new();
    input.hash(&mut s);
    s.finish()
}

impl BeaverTriplePair {
    pub fn new(
        share_index: u16,
        share0: TripleShare<Secp256k1>,
        pub0: TriplePub<Secp256k1>,
        share1: TripleShare<Secp256k1>,
        pub1: TriplePub<Secp256k1>,
        staker_hash: u64,
        peers: Vec<SimplePeer>,
    ) -> Result<Self> {
        let peer_group_id = peers.peer_group_id();
        let peer_keys: Vec<u64> = peers.peer_keys();

        let xor_filter = match xorf::BinaryFuse16::try_from(&peer_keys) {
            Ok(f) => f,
            Err(e) => {
                return Err(unexpected_err(
                    e,
                    Some("Could not create xor filter from peer keys".into()),
                ))
            }
        };

        let peer_ids = peers
            .iter()
            .map(|p| BeaverTriplePeerId {
                staker_hash: p.key_hash,
                peer_id: p.share_index as u32,
            })
            .collect::<Vec<BeaverTriplePeerId>>();

        Ok(BeaverTriplePair {
            share_index,
            share0,
            pub0,
            share1,
            pub1,
            peer_group_id,
            xor_filter,
            staker_hash,
            peer_ids,
        })
    }

    pub fn indices_from_peers(&self, peers: Vec<SimplePeer>) -> Vec<u32> {
        let mut peer_ids = Vec::new();

        // get peer_ids from self.peer_ids where staker_hash matches a peer in peers
        for peer in peers {
            for bt_peer_id in &self.peer_ids {
                if bt_peer_id.staker_hash == peer.key_hash {
                    peer_ids.push(bt_peer_id.peer_id);
                }
            }
        }

        peer_ids
    }
}
