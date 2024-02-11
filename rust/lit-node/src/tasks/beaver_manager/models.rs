use cait_sith::triples::{TriplePub, TripleShare};
use k256::Secp256k1;
use reqwest::Client;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;
use std::sync::Arc;
use std::{collections::VecDeque, hash::Hash};

use crate::error;
use crate::peers::peer_state::models::PeerSocketAddress;
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
pub struct BeaverTriplePair {
    pub share_index: u16,
    pub share0: TripleShare<Secp256k1>,
    pub pub0: TriplePub<Secp256k1>,
    pub share1: TripleShare<Secp256k1>,
    pub pub1: TriplePub<Secp256k1>,
    // hash of the peers used to generate the pairs
    pub peer_group_id: PeerGroupId,
    pub xor_filter: xorf::BinaryFuse16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripleRequest {
    pub message_bytes: Vec<u8>,
    pub public_key: Vec<u8>,
    pub request_id: Vec<u8>,
    pub txn_prefix: String,
    pub peers: Vec<PeerSocketAddress>,
    pub threshold: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct TripleRequestKey {
    pub message_bytes: Vec<u8>,
    pub public_key: Vec<u8>,
    pub request_id: Vec<u8>,
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
        Vec<PeerSocketAddress>,
        Sender<error::Result<Option<BeaverTriplePair>>>,
    ),
    RemoveGenerationHash(u64),
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
