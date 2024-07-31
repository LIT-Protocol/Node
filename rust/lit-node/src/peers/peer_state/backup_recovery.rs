// use std::time::Duration;

use std::time::Duration;

use crate::{
    error::blockchain_err,
    tasks::fsm::DKGRootKey,
    utils::{
        contract::decode_revert, encoding::hex_to_bytes, networking::get_web_addr_from_chain_info,
    },
};
use ethers::types::{Bytes, U256};
use lit_blockchain::contracts::{backup_recovery::RecoveryKey, staking::Validator};
use lit_core::error::Result;

use super::{super::PeerState, models::SimplePeer};

impl PeerState {
    pub async fn set_recovery_dkg_member(&self) -> Result<()> {
        let all_backup_members_mapped = self
            .backup_recovery_contract
            .all_backup_members_mapped()
            .call()
            .await
            .map_err(|e| {
                blockchain_err(e, Some("Unable to check if all parties are mapped".into()))
            })?;

        let is_node_mapped = self
            .is_node_mapped()
            .await
            .map_err(|e| blockchain_err(e, Some("Unable to check if our node is mapped".into())))?;

        if all_backup_members_mapped || is_node_mapped {
            info!("Done setMemberForDkg");
            return Ok(());
        }

        self.rpc_set_member_for_dkg().await;

        Ok(())
    }

    async fn rpc_set_member_for_dkg(&self) {
        let func = self.backup_recovery_contract.set_member_for_dkg();
        let gas_estimate = func.estimate_gas().await;

        match gas_estimate {
            Ok(gas_estimate) => {
                let func_with_gas = func.gas(gas_estimate * U256::from(2));
                let result = func_with_gas.send().await;

                match result {
                    Ok(_) => debug!("set member for dkg"),
                    Err(e) => {
                        debug!("Failed to set member for dkg w/ err {:?}", e);
                        debug!("{}", decode_revert(&e, self.backup_recovery_contract.abi()));
                    }
                }
            }
            Err(e) => {
                debug!("Failed to estimate gas for setMemberForDkg w/ err {:?}", e);
                debug!("{}", decode_revert(&e, self.backup_recovery_contract.abi()));
            }
        }
    }

    pub async fn is_node_mapped(&self) -> Result<bool> {
        let is_node_mapped = self
            .backup_recovery_contract
            .is_node_for_dkg()
            .call()
            .await
            .map_err(|e| blockchain_err(e, Some("Unable to check if the node is mapped".into())))?;

        info!("is_node_mapped- {:?}", is_node_mapped);
        Ok(is_node_mapped)
    }

    pub async fn get_recovery_dkg_peers(&self) -> Result<Vec<SimplePeer>> {
        // Fetch the peer addresses participating in the Recovery DKG
        let recovery_peer_addresses = self
            .backup_recovery_contract
            .get_staker_addresses_for_dkg()
            .call()
            .await
            .map_err(|e| blockchain_err(e, Some("Unable to get Recovery peer addresses".into())))?;

        info!("recovery_peer_addresses- {:?}", recovery_peer_addresses);

        // Map the peer addresses to `SimplePeer` by fetching their `Validator` struct
        let mut recovery_peers: Vec<SimplePeer> = Vec::new();
        for (index, staker_address) in recovery_peer_addresses.iter().enumerate() {
            let validator: Validator = self
                .staking_contract
                .validators(*staker_address)
                .call()
                .await
                .map_err(|e| {
                    blockchain_err(
                        e,
                        Some("Unable to get Validator from peer's staker address".into()),
                    )
                })?;

            recovery_peers.push(SimplePeer {
                socket_address: get_web_addr_from_chain_info(validator.ip, validator.port),
                share_index: index as u16, // Every peer has the same index based on the Backup party member mapping on-chain which is an array
                kicked: false,
                staker_address: ethers::types::H160::zero(), // Not used for Recovery DKG
                key_hash: 0,                                 // Not used for Recovery DKG
                protocol_index: None,                        // Not used for Recovery DKG
                version: crate::version::get_version(),
            });
        }

        Ok(recovery_peers)
    }

    pub async fn backup_party_not_empty(&self) -> Result<bool> {
        let backup_party = self
            .backup_recovery_contract
            .get_next_backup_party_members()
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to check whether Recovery party changed".into()),
                )
            })?;

        Ok(!backup_party.is_empty()) // As `registerNewBackupParty` updates the backup_party to length > 1
    }

    pub async fn is_recovery_dkg_registered(&self) -> Result<bool> {
        let is_recovery_dkg_completed = self
            .backup_recovery_contract
            .is_recovery_dkg_completed()
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to check whether Recovery DKG is completed".into()),
                )
            })?;

        Ok(is_recovery_dkg_completed)
    }

    // No retries for this function similar to Standard DKG
    pub async fn register_recovery_keys(&self, recovery_root_keys: Vec<DKGRootKey>) {
        let mut recovery_keys: Vec<RecoveryKey> = Vec::new();

        for recovery_key in recovery_root_keys {
            let pubkey_bytes = match hex_to_bytes(&recovery_key.pubkey) {
                Ok(pubkey_bytes) => pubkey_bytes,
                Err(e) => {
                    debug!("Error converting pubkey to bytes w/: {:?}", e);
                    return;
                }
            };
            recovery_keys.push(RecoveryKey {
                pubkey: Bytes::from(pubkey_bytes),
                key_type: recovery_key.curve_type.into(),
            });
        }

        let func = self
            .backup_recovery_contract
            .register_recovery_keys(recovery_keys);
        let gas_estimate = func.estimate_gas().await;
        match gas_estimate {
            Ok(gas_estimate) => {
                let func_with_gas = func.gas(gas_estimate * U256::from(5));
                let result = func_with_gas.send().await;

                match result {
                    Ok(_) => {
                        debug!("register pubkey for Recovery dkg");

                        // Once the recovery keys are registered, we sleep briefly to make sure any future chain reads will see the updated state.
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                    Err(e) => {
                        debug!("Failed to register pubkey for Recovery dkg w/ err {:?}", e);
                        debug!("{}", decode_revert(&e, self.backup_recovery_contract.abi()));
                    }
                }
            }
            Err(e) => {
                debug!(
                    "Failed to estimate gas for registerRecoveryKeys w/ err {:?}",
                    e
                );
                debug!("{}", decode_revert(&e, self.backup_recovery_contract.abi()));
            }
        }
    }
}
