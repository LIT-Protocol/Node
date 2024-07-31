use crate::constants::CHAIN_ETHEREUM;
use crate::error::{validation_err, validation_err_code, Result, EC};
use crate::models::EthBlock;
use crate::utils::encoding;
use ethers::providers::Middleware;
use ethers::types::{H256, U256, U64};
use lit_blockchain::resolver::rpc::{RpcHealthcheckPoller, ENDPOINT_MANAGER};
use lit_core::error::Unexpected;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};

use crate::siwe_db::utils::check_block_timestamp_validity;

const ETH_BLOCK_TIME_SECONDS: u64 = 12;

#[derive(Debug)]
pub struct EthBlockhashCache {
    pub blockhash: RwLock<String>,
    pub timestamp: RwLock<U256>,
}

impl EthBlockhashCache {
    pub async fn fetch_and_store_latest_blockhash(&self, mut quit_rx: mpsc::Receiver<bool>) {
        info!("Starting: EthBlockhashCache::fetch_and_store_latest_blockhash");

        let mut block_interval = tokio::time::interval(Duration::from_secs(ETH_BLOCK_TIME_SECONDS));

        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    break;
                }

                _ = block_interval.tick() => {
                    // info!("Periodic ({}s): Fetching the latest Eth blockhash", ETH_BLOCK_TIME_SECONDS);

                    let rpc_provider = match ENDPOINT_MANAGER.get_provider(CHAIN_ETHEREUM) {
                        Ok(rpc_provider) => rpc_provider,
                        Err(e) => {
                            error!("Error getting RPC Provider for latest blockhash: {}", e);
                            continue;
                        }
                    };


                    let block = match fetch_latest_blockhash(rpc_provider).await {
                            Ok(block) => block,
                            Err(e) => {
                            error!("Error fetching latest block hash: {}", e);
                            continue;
                        }
                    };

                    let mut write_blockhash = self.blockhash.write().await;
                    *write_blockhash = block.blockhash;

                    let timestamp_u256 = U256::from_dec_str(&block.timestamp);
                    if let Ok(timestamp_u256) = timestamp_u256 {
                        let mut write_timestamp = self.timestamp.write().await;
                        *write_timestamp = timestamp_u256;
                    } else {
                        error!("Error converting timestamp to U256");
                    }
                }
            }
        }

        info!("Stopped: EthBlockhashCache::fetch_and_store_latest_blockhash");
    }
}

pub async fn fetch_block_info<M: Middleware + 'static>(
    block_number: U64,
    rpc_provider: M,
) -> Result<EthBlock> {
    let block = rpc_provider
        .get_block(block_number)
        .await
        .map_err(|e| {
            validation_err_code(
                e,
                EC::NodeRpcError,
                Some("failed to get latest block when fetching blocks for db init".into()),
            )
        })?
        .expect_or_err("block is none")
        .map_err(|e| validation_err(e, None))?;

    let block_hash = block
        .hash
        .expect_or_err("block hash is none")
        .map_err(|e| validation_err_code(e, EC::NodeRpcError, None))?;
    let block_timestamp = block.timestamp;
    let block_number = block
        .number
        .expect_or_err("block number is none")
        .map_err(|e| validation_err_code(e, EC::NodeRpcError, None))?;

    Ok(EthBlock {
        blockhash: format!("0x{}", hex::encode(block_hash.to_fixed_bytes())),
        timestamp: block_timestamp.to_string(),
        block_number: block_number.as_usize(),
    })
}

pub async fn fetch_block_from_hash<M: Middleware + 'static>(
    block_hash: &str,
    rpc_provider: M,
) -> Result<EthBlock> {
    let hash = H256::from(encoding::bytes_to_zero_padded_32(encoding::hex_to_bytes(
        block_hash,
    )?)?);

    let block = rpc_provider.get_block(hash).await.map_err(|e| {
        validation_err_code(
            e,
            EC::NodeRpcError,
            Some("failed to get block when fetching blocks for query".to_string()),
        )
    })?;

    let Some(block_info) = block else {
        return Err(validation_err_code(
            "Invalid block hash",
            EC::NodeInvalidBlockhash,
            None,
        ));
    };

    let block_number = block_info
        .number
        .expect_or_err("block number is none")
        .map_err(|e| validation_err_code(e, EC::NodeRpcError, None))?;

    let block_timestamp = block_info.timestamp;
    check_block_timestamp_validity(&block_timestamp.to_string())?;
    let retrieved_block_hash = block_info
        .hash
        .ok_or_else(|| validation_err_code("block hash is none", EC::NodeRpcError, None))?;

    Ok(EthBlock {
        blockhash: format!("0x{}", hex::encode(retrieved_block_hash.to_fixed_bytes())),
        timestamp: block_timestamp.to_string(),
        block_number: block_number.as_usize(),
    })
}

pub async fn fetch_latest_blockhash<M: Middleware + 'static>(rpc_provider: M) -> Result<EthBlock> {
    let latest_block_number = rpc_provider.get_block_number().await.map_err(|e| {
        validation_err_code(
            e,
            EC::NodeRpcError,
            Some("failed to get latest block when fetching block for handshake".to_string()),
        )
    })?;

    fetch_block_info(latest_block_number, rpc_provider).await
}

#[cfg(test)]
mod siwe_block_fetch_tests {
    use chrono::{Duration, Utc};
    use ethers::{
        providers::{MockProvider, Provider},
        types::{Block, H256, U256, U64},
    };

    use crate::{
        siwe_db::{rpc::fetch_block_from_hash, utils::MAX_TIMESTAMP_VALIDITY_DAYS},
        utils::encoding,
    };

    #[tokio::test]
    async fn test_fetch_block_from_hash() {
        let block_timestamp = Utc::now().timestamp();
        let (provider, mock, block_hash, block_number) = mock_provider(block_timestamp);

        let row = fetch_block_from_hash(&block_hash, provider.clone()).await;
        assert!(row.is_ok());

        let row = row.unwrap();
        assert_eq!(row.blockhash, block_hash);
        assert_eq!(row.timestamp, block_timestamp.to_string());
        assert_eq!(row.block_number, block_number);

        // NOTE: Requests are retrieved back-to-front irrespective of invalid hashes. Hence we're keeping the first block invalid
        assert!(mock.push(U64::from(5)).is_ok());

        let invalid_row = fetch_block_from_hash(
            "0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae", // Hash doesn't matter since we always get the first mocked block
            provider,
        )
        .await;

        assert!(invalid_row.is_err());
    }

    #[tokio::test]
    async fn test_fetch_expired_block_from_hash() {
        let block_timestamp =
            (Utc::now() - Duration::days(MAX_TIMESTAMP_VALIDITY_DAYS + 1)).timestamp();

        let (provider, _, block_hash, _) = mock_provider(block_timestamp);

        let invalid_row = fetch_block_from_hash(&block_hash, provider).await;

        assert!(invalid_row.is_err());
    }

    fn mock_provider(
        block_timestamp: i64,
    ) -> (Provider<MockProvider>, MockProvider, String, usize) {
        let (provider, mock) = Provider::mocked();
        let block_hash = "0x70dd3646979bc3d49af8ad6320d2b03149a72863f8e08f254e54fa8954f59143";
        let block_number: usize = 14402566;

        let block: Block<H256> = Block {
            hash: Some(H256::from(
                encoding::bytes_to_zero_padded_32(encoding::hex_to_bytes(block_hash).unwrap())
                    .unwrap(),
            )),
            timestamp: U256::from(block_timestamp),
            number: Some(U64::from(block_number)),
            ..Default::default()
        };

        // NOTE: Requests are retrieved back-to-front irrespective of invalid hashes.
        assert!(mock.push(block.clone()).is_ok());

        (provider, mock, block_hash.to_string(), block_number)
    }
}
