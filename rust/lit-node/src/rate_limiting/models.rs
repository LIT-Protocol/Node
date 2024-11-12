use crate::config::chain::ChainDataConfigManager;
use ethers::prelude::*;
use moka::future::Cache;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::RwLock;

/// This struct encapsulates everything about the user (client) that is relevant
/// for validating against the rate limit policies.
pub(crate) struct UserContext {
    /// This is the user's wallet address.
    pub user_address: Option<Address>,
}

pub(crate) struct RateLimitDB {
    // pub owner_to_nft_id_map: Cache<Address, Vec<U256>>,
    // maps rate limit NFT ID to rate limit data
    pub nft_cache: Cache<U256, RateLimitNft>,
    /// Maps Rate Limit NFT ID to all authorized usage data against it (across all wallet addresses)
    pub nft_usage_map: RwLock<HashMap<U256, UsageEntries>>,

    /// Map that is really used as a set to track which wallet addresses DO NOT need to have their
    /// rate limit NFTs fetched.
    // pub latest_cache_miss_map: Cache<String, bool>,

    // stores config for defaults like rate limit window, etc.
    pub chain_data_config_manager: Arc<ChainDataConfigManager>,

    pub delegation_uses_map: Cache<Vec<u8>, u32>,
}

impl RateLimitDB {
    // False positive warning from the lint.
    #[allow(dead_code)]
    pub fn default_with_chain_data_config_manager(
        chain_data_config_manager: Arc<ChainDataConfigManager>,
    ) -> Self {
        Self {
            // owner_to_nft_id_map: Cache::builder()
            //     .build()
            //     .time_to_live(Duration::from_secs(86400)),
            // 1m item max capacity.  each item is a RateLimitNft which is 116 bytes, so our max memory usage is 116mb.
            nft_cache: Cache::builder().max_capacity(1_000_000).build(),
            nft_usage_map: RwLock::new(HashMap::new()),
            // latest_cache_miss_map: Cache::builder()
            //     .time_to_live(Duration::from_secs(60))
            //     .build(),
            chain_data_config_manager,
            // 30 day TTL
            delegation_uses_map: Cache::builder()
                .time_to_live(Duration::from_secs(30 * 24 * 60 * 60))
                .max_capacity(1000000)
                .build(),
        }
    }
}

pub(crate) struct UsageEntries {
    pub timestamps: RwLock<Vec<SystemTime>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct RateLimitNft {
    pub id: U256,
    pub requests_per_kilosecond: U256,
    pub expires_at: U256,
    pub owner: Address,
}

#[derive(Debug)]
pub(crate) struct RateLimitCheckReturn {
    pub rate_limit_exceeded: bool,
    pub try_again_after: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct PossiblyDelegatedRateLimitNft {
    pub nft: RateLimitNft,
    pub signature_hash_uses_key: Option<Vec<u8>>,
}
