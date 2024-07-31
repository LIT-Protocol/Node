use super::super::peer_state::models::NetworkState;
use super::super::PeerState;
use crate::error::{blockchain_err, unexpected_err, Result};
use ethers::types::{Address, H160, U256};
use lit_blockchain::config::LitBlockchainConfig;
use lit_blockchain::resolver::rpc::RpcHealthcheckPoller;
use std::collections::BTreeSet;
use std::iter::FromIterator;
use tracing::trace;

#[allow(dead_code)]
impl PeerState {
    #[doc = "Get the current epoch along with length, number and retries."]
    pub async fn get_epoch(&self) -> Result<(U256, U256, U256, U256)> {
        let epoch = self
            .staking_contract
            .epoch()
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;

        Ok((
            epoch.epoch_length,
            epoch.number,
            epoch.end_time,
            epoch.retries,
        ))
    }

    #[doc = "Get the current block number."]
    pub async fn get_block_number(&self) -> Result<ethers::types::U64> {
        use ethers::providers::Middleware;
        let chain = self.lit_config.blockchain_chain_name()?;
        let wallet = lit_blockchain::contracts::load_wallet(&self.lit_config, None)?;
        let provider =
            lit_blockchain::resolver::rpc::ENDPOINT_MANAGER.get_provider(chain.as_str())?;

        provider
            .get_block_number()
            .await
            .map_err(|e| unexpected_err(e, Some("Failed to get block number from chain".into())))
    }

    pub async fn validators_for_next_epoch_locked(&self) -> Result<bool> {
        let state = self.network_state().await?;
        Ok(state == NetworkState::NextValidatorSetLocked)
    }

    pub async fn validators_in_active_state(&self) -> Result<bool> {
        let state = self.network_state().await?;
        Ok(state == NetworkState::Active)
    }

    pub async fn network_state(&self) -> Result<NetworkState> {
        let state = self
            .staking_contract
            .state()
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;

        Ok(NetworkState::from(state))
    }

    pub async fn get_ready_signal(&self, addr: H160) -> Result<bool> {
        let ready_signal = self
            .staking_contract
            .ready_for_next_epoch(addr)
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;

        Ok(ready_signal)
    }

    pub async fn current_validator_count_for_consensus(&self) -> Result<U256> {
        let count = self
            .staking_contract
            .current_validator_count_for_consensus()
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;

        Ok(count)
    }

    pub async fn next_validator_count_for_consensus(&self) -> Result<U256> {
        let count = self
            .staking_contract
            .next_validator_count_for_consensus()
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;

        Ok(count)
    }

    pub async fn get_count_of_validators_ready_for_next_epoch(&self) -> Result<U256> {
        let count = self
            .staking_contract
            .count_of_next_validators_ready_for_next_epoch()
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;
        Ok(count)
    }

    pub async fn part_of_validators_union(&self) -> Result<bool> {
        // TODO?: Store in CDM cache
        let vin_current = BTreeSet::from_iter(self.validators_in_current_epoch().await?);
        let vin_next = BTreeSet::from_iter(self.validators_in_next_epoch().await?);
        let vi_union_of_epochs = vin_current.union(&vin_next);

        let res = vi_union_of_epochs
            .filter(|v| !v.is_kicked)
            .map(|v| v.address)
            .collect::<Vec<Address>>()
            .contains(&self.node_address());
        trace!(
            "Node: {:?} is part of union: {:?}",
            self.node_address(),
            res
        );
        Ok(res)
    }
}
