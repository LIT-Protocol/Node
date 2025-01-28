use crate::error::{blockchain_err, blockchain_err_code, Result, EC};
use crate::utils::contract::decode_revert;
use ethers::types::{U256, U64};
use std::time::Duration;
#[cfg(not(feature = "testing"))]
use std::time::SystemTime;
use tracing::trace;

use super::super::PeerState;

#[allow(dead_code)]
impl PeerState {
    // ############ Functions to mutate on-chain peer state using the node wallet ############
    // Note that if we fail to signal ready, it is a problem and we error, but if we fail to lock or advance, probably it is fine, so we just log.
    #[allow(non_snake_case)]
    pub async fn rpc_signal_ready_for_next_epoch(&self, epoch_number: U256) -> Result<()> {
        let func = self
            .staking_contract
            .signal_ready_for_next_epoch(epoch_number);

        let gas_estimate = match func.estimate_gas().await {
            Ok(gas_estimate) => gas_estimate,
            Err(e) => {
                debug!(
                    "Gas Est: {}",
                    decode_revert(&e, self.staking_contract.abi())
                );

                return Err(blockchain_err(e, None));
            }
        };

        let func_with_gas = func.gas(gas_estimate * U256::from(5));
        let tx = func_with_gas
            .send()
            .await
            .map_err(|e| blockchain_err(e, None))?
            .interval(Duration::from_millis(500));
        // wait for tx to be confirmed
        match tx.await.map_err(|e| blockchain_err(e, None))? {
            Some(receipt) => {
                trace!(
                    "Confirmed signal ready for tx {:?}",
                    receipt.transaction_hash
                );

                // the below code parses out the epoch from the event
                // we don't need it, because the txn will fail if we are trying to signal for the wrong epoch.
                // but please leave it below in case we need it again for debugging or something else
                // let event = self.staking_contract.abi().event("ReadyForNextEpoch");
                // let event = match event {
                //     Ok(event) => event,
                //     Err(e) => {
                //         error!("Failed to find ReadyForNextEpoch event");
                //         return Err(blockchain_err_code(
                //             e,
                //             EC::NodeRpcError,
                //             Some("Failed to find ReadyForNextEpoch event".to_string()),
                //         ));
                //     }
                // };

                // let log = receipt
                //     .logs
                //     .iter()
                //     .find(|log| log.topics.first() == Some(&event.signature()));
                // let log = match log {
                //     Some(log) => log,
                //     None => {
                //         error!("Failed to find epoch in event");
                //         return Err(blockchain_err_code(
                //             "Failed to find epoch in event",
                //             EC::NodeRpcError,
                //             None,
                //         ));
                //     }
                // };

                // let parsed = event.parse_log(RawLog::from(log.clone()));
                // let parsed = match parsed {
                //     Ok(parsed) => parsed,
                //     Err(e) => {
                //         error!("Failed to parse epoch in event");
                //         return Err(blockchain_err_code(
                //             e,
                //             EC::NodeRpcError,
                //             Some("Failed to parse epoch in event".to_string()),
                //         ));
                //     }
                // };
                // // trace!("parsed log: {:?}", parsed);
                // let epoch_number = parsed
                //     .params
                //     .iter()
                //     .find(|param| param.name == "epochNumber");
                // let epoch_number = match epoch_number {
                //     Some(epoch_number) => epoch_number,
                //     None => {
                //         error!("Failed to find epoch in event");
                //         return Err(blockchain_err_code(
                //             "Failed to find epoch in event",
                //             EC::NodeRpcError,
                //             None,
                //         ));
                //     }
                // };
                // trace!("Signaled ready for epoch {}", epoch_number.value);

                if let Some(status) = receipt.status {
                    // if we did get a txn receipt, and it was a success, then we don't really need to confirm we actually signalled ready below, and can early exit.  the chain has already confirmed that the txn took effect.
                    if status == U64::from(1) {
                        return Ok(());
                    }
                }
            }
            None => {
                error!("Failed to get transaction receipt for signal ready for next epoch, but the transaction might still have worked.");
                // Err(blockchain_err_code(
                //     "Failed to signal ready for next epoch",
                //     EC::NodeRpcError,
                //     None,
                // ))
            }
        }
        // check if the signal ready actually took effect.  make up to 3 attempts.
        for i in 0..3 {
            let is_ready = match self.get_ready_signal(self.staker_address).await {
                Ok(r) => r,
                Err(e) => {
                    error!("Error in get_ready_signal: {}", e);
                    return Err(blockchain_err_code(
                        "Error in get_ready_signal",
                        EC::NodeRpcError,
                        Some(e.to_string()),
                    ));
                }
            };
            if is_ready {
                return Ok(());
            }
            // sleep for 2s so we give the chain state time to propagate
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
        Err(blockchain_err_code(
            "Failed to signal ready for next epoch",
            EC::NodeRpcError,
            None,
        ))
    }

    pub async fn rpc_lock_validators_for_next_epoch(&self) {
        // when testing, do estimate gas.  this is fine because anvil returns the current time in block.timestamp, even in view methods
        {
            let func = self.staking_contract.lock_validators_for_next_epoch();
            // println!("Sending txn to lock validators: {:?}", func);
            let gas_estimate = func.estimate_gas().await;
            match gas_estimate {
                Ok(gas_estimate) => {
                    let func_with_gas = func.gas(gas_estimate * U256::from(2));

                    let result = func_with_gas.send().await;

                    match result {
                        Ok(_) => info!("locked the validators"),
                        Err(e) => {
                            trace!("failed to lock validators for next epoch (only one caller need succeed though) w/ err {:?}", e);
                            trace!("{}", decode_revert(&e, self.staking_contract.abi()));
                        }
                    }
                }
                Err(e) => {
                    trace!("failed to estimate gas to lock validators for next epoch (only one caller need succeed though) w/ err {:?}", e);
                    trace!("{}", decode_revert(&e, self.staking_contract.abi()));
                }
            }
        }
    }

    pub async fn rpc_advance_epoch(&self) {
        let func = self.staking_contract.advance_epoch();
        let gas_estimate = func.estimate_gas().await;
        match gas_estimate {
            Ok(gas_estimate) => {
                let func_with_gas = func.gas(gas_estimate * U256::from(2));
                let result = func_with_gas.send().await;

                match result {
                    Ok(_) => debug!("advanced the epoch"),
                    Err(e) => {
                        debug!(
                    "failed to advance the epoch (only one caller need succeed though) w/ err {:?}",
                    e
                );
                        debug!("{}", decode_revert(&e, self.staking_contract.abi()));
                    }
                }
            }
            Err(e) => {
                debug!(
                    "failed to advance the epoch (only one caller need succeed though) w/ err {:?}",
                    e
                );
                debug!("{}", decode_revert(&e, self.staking_contract.abi()));
            }
        }
    }

    pub async fn request_to_leave(&self) -> Result<()> {
        self.staking_contract
            .request_to_leave_as_node()
            .send()
            .await
            .map_err(|e| {
                error!(
                    "Failed to request to leave as node: {:?}",
                    decode_revert(&e, self.staking_contract.abi())
                );
                blockchain_err(e, Some("Failed to request to leave as node".to_string()))
            })?;
        Ok(())
    }
}
