use super::contracts::{Contracts, StakingContractConfig};
use super::{NodeAccount, WhichTestnet};
use crate::models::VotingStatusToKickValidator;
use crate::node_collection::{handshake_returns_keys, NodeStakingStatus};
use crate::recovery_party::{
    check_share_data, download_share, get_contracts_for_recovery_member,
    get_validator_struct_for_recovery_share, get_wallet_addresses,
};
use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::utils;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain::contracts::backup_recovery::BackupRecovery;
use lit_blockchain::contracts::pubkey_router::RootKey;
use lit_blockchain::contracts::staking::ComplaintConfig;
use lit_blockchain::contracts::{
    lit_token::lit_token::LITToken,
    staking::{Staking, StakingErrors, Validator},
    staking_balances::StakingBalances,
};
use lit_core::utils::binary::bytes_to_hex;
use lit_node::peers::{peer_item::PeerItem, peer_state::models::PeerValidatorStatus};
use lit_node::tasks::utils::parse_version;
use lit_node::tss::common::curve_type::CurveType;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, warn};

#[derive(Clone)]
pub struct Actions {
    contracts: Contracts,
    deployer_signing_provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    which_testnet: WhichTestnet,
    deploy_address: Address,
}

impl Actions {
    pub fn new(
        contracts: Contracts,
        deployer_signing_provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        which_testnet: WhichTestnet,
        deploy_address: Address,
    ) -> Self {
        Self {
            contracts,
            deployer_signing_provider,
            which_testnet,
            deploy_address,
        }
    }

    pub fn deployer_signing_provider(
        &self,
    ) -> Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
        self.deployer_signing_provider.clone()
    }

    pub fn deployer_provider(&self) -> Provider<Http> {
        self.deployer_signing_provider.inner().clone()
    }

    pub fn contracts(&self) -> &Contracts {
        &self.contracts
    }

    pub async fn get_latest_block_timestamp(&self) -> Result<U256> {
        let block = self
            .deployer_provider()
            .get_block(
                self.deployer_provider()
                    .get_block_number()
                    .await
                    .map_err(|e| anyhow::anyhow!("Error getting block number: {:?}", e))?,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Error getting block: {:?}", e))?
            .ok_or_else(|| anyhow::anyhow!("Error getting block"))?;
        Ok(block.timestamp)
    }

    pub async fn get_epoch_end_time(&self) -> Result<U256> {
        let epoch = self.contracts.staking.epoch().call().await?;
        Ok(epoch.end_time)
    }

    pub async fn set_epoch_end_time(&self, new_end_time: U256) -> Result<()> {
        let cc = self.contracts.staking.set_epoch_end_time(new_end_time);
        if !Contracts::process_contract_call(cc, "set_epoch_end_time").await {
            return Err(anyhow::anyhow!("Error setting epoch end time"));
        }
        Ok(())
    }

    pub async fn set_epoch_end_time_from_now(&self, seconds_from_now: u64) -> Result<()> {
        let current_time = self.get_latest_block_timestamp().await?;
        let new_end_time = current_time + U256::from(seconds_from_now);
        self.set_epoch_end_time(new_end_time).await
    }

    pub async fn lit_token_balance(&self, address: Address) -> U256 {
        self.contracts
            .lit_token
            .balance_of(address)
            .call()
            .await
            .unwrap()
    }

    pub async fn get_current_validators(&self) -> Vec<H160> {
        self.contracts
            .staking
            .get_validators_in_current_epoch()
            .call()
            .await
            .expect("Error getting validators from chain")
    }

    pub async fn get_current_validator_structs(&self) -> Vec<Validator> {
        self.contracts
            .staking
            .get_validators_structs_in_current_epoch()
            .call()
            .await
            .expect("Error getting validator structs from chain")
    }

    pub async fn get_next_validators(&self) -> Vec<H160> {
        self.contracts
            .staking
            .get_validators_in_next_epoch()
            .call()
            .await
            .expect("Error getting next validators from chain")
    }

    pub async fn get_current_validator_count(&self) -> u32 {
        self.get_current_validators().await.len() as u32
    }

    pub async fn send_approve_and_stake(
        &self,
        staker: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    ) -> Result<()> {
        // give some tokens to the staker

        let deployer_balance = self
            .contracts
            .lit_token
            .balance_of(self.deploy_address)
            .call()
            .await?;
        info!("Deployer balance is {}", deployer_balance);

        info!(
            "Balance before send: {:?}",
            self.lit_token_balance(staker.address()).await
        );

        let amount_to_send = ethers::utils::parse_units(4, 18).unwrap().into();
        let r = self
            .contracts
            .lit_token
            .transfer(staker.address(), amount_to_send);

        let res = r
            .send()
            .await
            .unwrap()
            .interval(Duration::from_millis(500))
            .await;
        if let Err(e) = res {
            panic!("Error sending LIT tokens: {:?}", e);
        }

        info!(
            "Balance after send: {:?}",
            self.lit_token_balance(staker.address()).await
        );

        let lit_token = LITToken::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            self.contracts.lit_token.address(),
            staker.clone(),
        );

        // spender is the deployed staking balances contract
        let spender = self.contracts.staking_balances.address();
        let amount_to_approve = ethers::utils::parse_units(2, 18).unwrap().into();
        let r = lit_token.approve(spender, amount_to_approve);
        let r = r.send().await;
        if r.is_err() {
            panic!("Error Approving ERC20 : {:?}", r);
        }

        let receipt = r.unwrap().await;
        if receipt.is_err() {
            panic!("(Receipt) Error Approving ERC20 : {:?}", receipt);
        }

        let staking_balances = StakingBalances::<
            SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        >::new(
            self.contracts.staking_balances.address(), staker.clone()
        );

        let staking = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            staker.clone(),
        );

        let stake_amount = staking_balances.minimum_stake().call().await?;

        info!("Staking from {:?}", staker.address(),);

        let r = staking.stake(stake_amount);

        let r = r.send().await;
        if let Err(e) = r {
            debug!(
                "Error doing stake.  Revert: {:?}",
                lit_node::utils::contract::decode_revert(&e, staking.abi())
            );

            let revert: Option<StakingErrors> = e.decode_contract_revert();
            match revert {
                Some(r) => {
                    return Err(anyhow::anyhow!(
                        "Error doing stake: {:?}.  Revert: {:?}",
                        e,
                        r
                    ));
                }
                None => {
                    return Err(anyhow::anyhow!(
                        "Error doing stake: {:?}.  Could not decode revert reason.  Revert: {:?}",
                        &e,
                        lit_node::utils::contract::decode_revert(&e, staking.abi())
                    ));
                }
            }
        }

        // make sure it's fully mined so we don't accidently advance then lock the next epoch before the user has actually staked
        let _receipt = r.unwrap().interval(Duration::from_millis(500)).await;

        Ok(())
    }

    pub async fn send_request_to_join(
        &self,
        staker: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        ip: u32,
        port: u32,
        node_info: &PeerItem,
    ) -> Result<()> {
        info!(
            "Staking from {:?} for with node_address {:?} - PeerItem {}",
            staker.address(),
            node_info.node_address,
            node_info
        );

        let staking = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            staker.clone(),
        );

        info!(
            "request to join with sender pub key: {:?}",
            U256::from_big_endian(&node_info.sender_public_key[..])
        );

        let r = staking.request_to_join(
            ip,
            1, //check
            port,
            node_info.node_address,
            U256::from_big_endian(&node_info.sender_public_key[..]),
            U256::from_big_endian(&node_info.receiver_public_key[..]),
        );

        let r = r.send().await;
        if let Err(e) = r {
            debug!(
                "Error doing request_to_join for {:}.  Revert: {:?}",
                node_info.addr,
                lit_node::utils::contract::decode_revert(&e, staking.abi())
            );

            let revert: Option<StakingErrors> = e.decode_contract_revert();
            match revert {
                Some(r) => {
                    return Err(anyhow::anyhow!(
                        "Error doing request_to_join {:} : {:?}.  Revert: {:?}",
                        node_info.addr,
                        e,
                        r
                    ));
                }
                None => {
                    return Err(anyhow::anyhow!(
                        "Error doing request_to_join {:} : {:?}.  Could not decode revert reason.  Revert: {:?}",
                        node_info.addr, &e, lit_node::utils::contract::decode_revert(&e, staking.abi() )
                    ));
                }
            }
        }

        // make sure it's fully mined so we don't accidently advance then lock the next epoch before the user has actually staked
        let _receipt = r.unwrap().interval(Duration::from_millis(500)).await;

        Ok(())
    }

    #[doc = "Wait for state to become active again (DKGs run, advance)"]
    pub async fn wait_for_active(&self) {
        info!("Waiting for network to become active again");
        loop {
            let res = self.contracts.staking.state().call().await;
            match res {
                Ok(res) => {
                    if res == 0 {
                        info!("Network is active");
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
                Err(..) => {
                    debug!(
                        "Error checking if validator state is active : {:?}",
                        res.unwrap_err()
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                }
            }
        }

        info!("Sleeping for 2 seconds to make sure nodes sync up with new peer state...");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }

    #[doc = "Wait for state to become locked"]
    pub async fn wait_for_lock(&self) {
        info!("Waiting for nodes to be locked");
        let res = self
            .contracts
            .staking
            .get_validators_in_next_epoch()
            .call()
            .await;

        if res.is_err() {
            panic!(
                "Error getting validators in next epoch: {:?}",
                res.unwrap_err()
            );
        }

        info!("Validators in next epoch: {:?}", res.unwrap());

        loop {
            let res = self.contracts.staking.state().call().await;

            match res {
                Ok(res) => {
                    debug!("State is {:?}", res);
                    if res == 1 {
                        info!("Next validator set is locked");
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
                Err(..) => {
                    info!(
                        "Error checking if validators in next epoch are locked : {:?}",
                        res.unwrap_err()
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                } // _ => tokio::time::sleep(std::time::Duration::from_secs(1)).await,
            }
        }
    }

    pub async fn wait_for_recovery_keys(&self) {
        info!("Waiting for recovery keys!");

        // Check whether the recovery keys are registered on the chain.
        loop {
            if self
                .contracts
                .backup_recovery
                .is_recovery_dkg_completed()
                .call()
                .await
                .unwrap()
            {
                info!("Got recovery keys!");
                break;
            }

            let _r = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    pub async fn download_recovery_key_shares(&self, rpc_url: &str, chain_id: u64) {
        info!("Attempting to download recovery keys!");
        info!("rpc_url: {}", rpc_url);
        info!("chain_id: {}", chain_id);
        let backup_recovery_address = self.contracts.backup_recovery.address();
        let staking_address = self.contracts.staking.address();
        let (backup_recovery_contract, staking_contract) = get_contracts_for_recovery_member(
            backup_recovery_address,
            staking_address,
            rpc_url,
            chain_id,
        );
        let validator =
            get_validator_struct_for_recovery_share(backup_recovery_contract, staking_contract)
                .await;
        info!(
            "Got validator info for the recovery member: {:?}",
            validator
        );
        let share_data = download_share(&validator).await;
        check_share_data(share_data);
    }

    pub async fn wait_for_root_keys(&self) -> bool {
        info!("Waiting for root keys!");

        // First, check whether the root keys are registered on the chain.
        loop {
            if self.get_root_keys(CurveType::K256).await.is_some()
                && self.get_root_keys(CurveType::BLS).await.is_some()
            {
                break;
            }
            let _r = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        // Then, wait until the nodes have synced the latest chain state.
        loop {
            if handshake_returns_keys(self).await {
                break;
            }
            let _r = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        true
    }

    pub async fn get_root_keys(&self, curve_type: CurveType) -> Option<Vec<String>> {
        let all_root_keys = self.get_all_root_keys().await;
        all_root_keys.as_ref()?;
        let all_root_keys: Vec<RootKey> = all_root_keys.unwrap();

        let root_keys: Vec<String> = all_root_keys
            .iter()
            .filter(|k| k.key_type == curve_type.into())
            .map(|k| bytes_to_hex(k.pubkey.clone()))
            .collect::<Vec<String>>();

        Some(root_keys)
    }

    pub async fn set_complaint_reason_config(
        &self,
        reason: U256,
        config: ComplaintConfig,
    ) -> Result<()> {
        let staking = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            self.deployer_signing_provider.clone(),
        );

        let cc = staking.set_complaint_config(reason, config);
        if !Contracts::process_contract_call(cc, "set complaint config").await {
            return Err(anyhow::anyhow!("Error setting complaint config"));
        }

        Ok(())
    }

    pub async fn get_all_root_keys(&self) -> Option<Vec<RootKey>> {
        let staking_address = self.contracts.staking.address();
        let root_keys = self
            .contracts
            .pubkey_router
            .get_root_keys(staking_address)
            .call()
            .await
            .unwrap();

        if !root_keys.is_empty() {
            info!("Got root keys!");
            tracing::trace!("Root keys: {:?}", root_keys);
            return Some(root_keys);
        } else {
            info!("No root keys yet for contract {:?}", staking_address);
        }

        None
    }

    /// Wait for number of votes to kick validator to reach the expected value.
    ///
    /// Note that the actual number of votes to kick validator may be greater than the expected value.
    pub async fn wait_for_voting_status_to_kick_validator(
        &self,
        epoch_number: U256,
        validator_to_kick_staker_address: Address,
        voter_staker_address: Address,
        expected_num_votes_to_kick_validator: usize,
    ) -> Result<VotingStatusToKickValidator> {
        loop {
            let (votes, voter_voted) = self
                .contracts
                .staking
                .get_voting_status_to_kick_validator(
                    epoch_number,
                    validator_to_kick_staker_address,
                    voter_staker_address,
                )
                .await?;

            if votes.as_usize() >= expected_num_votes_to_kick_validator {
                // is the node actually kicked?
                let kicked_validators = self.contracts.staking.get_kicked_validators().await?;
                assert!(
                    kicked_validators.contains(&validator_to_kick_staker_address),
                    "Validator {:?} is not in the set of kicked validators: {:?}",
                    validator_to_kick_staker_address,
                    kicked_validators
                );
                // verify that the node isn't in the set anymore
                let validators = self
                    .contracts
                    .staking
                    .get_validators_in_next_epoch()
                    .await?;
                assert!(
                    !validators.contains(&validator_to_kick_staker_address),
                    "Validator {:?} is still in the set of validators: {:?}",
                    validator_to_kick_staker_address,
                    validators
                );
                return Ok(VotingStatusToKickValidator {
                    votes,
                    did_voter_kick_validator: voter_voted,
                });
            }

            // Wait for 1 second before checking again.
            let _r = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    #[doc = "Wait for initial epoch to end - a collection of functions to set the state to active and lock validators for next epoch."]
    pub async fn wait_for_initial_epoch(&self) {
        self.start_initial_epoch(true).await
    }

    pub async fn register_recovery_party(
        &self,
        party_size: usize,
        owner: &NodeAccount,
        members: Option<Vec<Address>>,
    ) {
        let members = match members {
            Some(vec) => vec,
            None => get_wallet_addresses(party_size),
        };
        info!("New recovery party members: {:?}", members);

        let backup_recovery =
            BackupRecovery::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
                self.contracts.backup_recovery.address(),
                owner.signing_provider.clone(),
            );

        let tx = backup_recovery.register_new_backup_party(members);

        let result = tx.send().await;

        if result.is_err() {
            panic!(
                "Error Registering Backup Party members: {:?}",
                result.unwrap_err()
            );
        }
    }

    /// Wait for the initial epoch to end - a collection of functions to set the state to active and lock validators for next epoch.
    pub async fn start_initial_epoch(&self, wait_for_active: bool) {
        let deploy_address = self.deploy_address;
        info!(
            "Starting epoch with validators: {:?}",
            self.contracts
                .staking
                .validators(deploy_address)
                .call()
                .await
                .unwrap()
        );

        info!(
            "Staking state (wait_for_initial_epoch) : {:?}",
            self.contracts.staking.state().call().await
        );

        if wait_for_active {
            self.wait_for_active().await;
        }

        info!("Initial Epoch has started.");
    }

    #[doc = "Lock validators for next epoch"]
    pub async fn lock_validators_for_next_epoch(&self) {
        let state = self.contracts.staking.state().call().await;
        if state.is_err() {
            error!("Error getting state...");
            return;
        }
        info!("Staking state (pre lock) : {:?}", state);

        let lock_func = self.contracts.staking.lock_validators_for_next_epoch();
        let lock_res = lock_func.send().await;
        warn!("Locking validators for next epoch: {:?}", lock_res);
        // assert!(lock_res.is_ok());
        info!(
            "Staking state (post lock) : {:?}",
            self.contracts.staking.state().call().await
        );
    }

    pub async fn set_staking_min_version(&self, min_version: &str) -> Result<()> {
        let staking = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            self.deployer_signing_provider.clone(),
        );

        let min_version = parse_version(min_version)?;
        let cc = staking.set_min_version(min_version);
        if !Contracts::process_contract_call(cc, "set minimum version").await {
            return Err(anyhow::anyhow!("Error setting min version"));
        }

        Ok(())
    }

    pub async fn set_staking_max_version(&self, max_version: &str) -> Result<()> {
        let staking = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            self.deployer_signing_provider.clone(),
        );

        let max_version = parse_version(max_version)?;
        let cc = staking.set_max_version(max_version);
        if !Contracts::process_contract_call(cc, "set maximum version").await {
            return Err(anyhow::anyhow!("Error setting max version"));
        }

        Ok(())
    }

    pub async fn ensure_node_unstaked(
        &self,
        node_account: NodeAccount,
    ) -> Result<NodeStakingStatus> {
        info!("Unstaking node: {:?}", node_account.staker_address);

        let staking = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            node_account.signing_provider.clone(),
        );

        let tx = staking.request_to_leave();

        let result = tx.send().await;

        if result.is_err() {
            panic!("Error unstaking node: {:?}", result.unwrap_err());
        }

        Ok(NodeStakingStatus::Unstaked)
    }

    pub async fn sleep_random_millis(&self, min: u64, max: u64) {
        use rand::Rng;
        let millis = rand::thread_rng().gen_range(min..max);
        info!("Sleeping a test for {} millis.", millis);
        tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
    }

    #[doc = "Sleep for a number of milliseconds"]
    pub async fn sleep_millis(&self, millis: u64) {
        info!("Sleeping a test for {} millis.", millis);
        tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
    }

    #[doc = "Fast forward by a number of blocks"]
    pub async fn increase_blockchain_timestamp(&self, seconds_to_increase: usize) {
        // get most recent block timestamp
        let block = self
            .deployer_provider()
            .get_block(self.deployer_provider().get_block_number().await.unwrap())
            .await
            .unwrap()
            .expect("Error getting block");
        let block_timestamp_before = block.timestamp;
        debug!("block_timestamp_before- {}", block_timestamp_before);

        let timestamp = Duration::from_secs(block_timestamp_before.as_u64())
            + Duration::from_secs(seconds_to_increase.try_into().unwrap());
        debug!("timestamp- {}", timestamp.as_secs());

        let res: Result<(), ProviderError> = self
            .deployer_provider()
            .request("evm_setNextBlockTimestamp", [timestamp.as_secs()])
            .await;

        match res {
            Ok(r) => info!(
                "Successfully increased blockchain timestamp by {:?} seconds: {:?}",
                seconds_to_increase, r
            ),
            Err(e) => {
                info!("Error increasing blockchain timestamp: {:?}", e);
                panic!("{}", e);
            }
        }

        // mine a block
        let mine_block_res: Result<(), ProviderError> = self
            .deployer_provider()
            .request("anvil_mine", [utils::serialize(&1), utils::serialize(&0)])
            .await;
        match mine_block_res {
            Ok(r) => info!("Successfully mined block: {:?}", r),
            Err(e) => {
                info!("Error mining block: {:?}", e);
                panic!("{}", e);
            }
        }

        let block = self
            .deployer_provider()
            .get_block(self.deployer_provider().get_block_number().await.unwrap())
            .await
            .unwrap()
            .expect("Error getting block");
        let block_timestamp_after = block.timestamp;
        debug!("block_timestamp_after- {}", block_timestamp_after);
    }

    #[doc = "Fast forward by a number of blocks"]
    pub async fn fast_forward_blocks(&self, blocks_to_mine: usize) {
        info!("Fast forwarding by {:?} blocks...", blocks_to_mine);
        let command = match self.which_testnet {
            WhichTestnet::Anvil => "anvil_mine",
            WhichTestnet::Hardhat => "hardhat_mine",
            _ => panic!("Unsupported network for fastforwarding blocks!"),
        };

        let block_num_before = self.deployer_provider().get_block_number().await.unwrap();

        let mine_blocks_res: Result<(), ProviderError> = self
            .deployer_provider()
            .request(
                command,
                [
                    utils::serialize(&format!("0x{:X}", blocks_to_mine)),
                    utils::serialize(&0),
                ],
            )
            .await;

        match mine_blocks_res {
            Ok(r) => info!("Successfully mined {:?} blocks: {:?}", blocks_to_mine, r),
            Err(e) => info!("Error mining blocks - you can ignore this on Anvil and look at the below Block Number message to check that it actually fast forwarded {:?}", e),
        }

        let block_num_after = self.deployer_provider().get_block_number().await.unwrap();
        info!(
            "Block number before fast forwarding: {}, Block number after fast forwarding: {}",
            block_num_before, block_num_after
        );
    }

    pub async fn get_current_epoch(&self) -> U256 {
        let get_res = self.contracts.staking.epoch().call().await;

        if get_res.is_err() {
            error!("Error in get_epoch: {}", get_res.err().unwrap());
            return U256::zero();
        }
        let epoch = get_res.unwrap();
        let epoch_number = epoch.number;
        debug!("epoch_number- {}", epoch_number);

        epoch_number
    }

    pub async fn wait_for_epoch(&self, epoch: U256) {
        info!(
            "Waiting for epoch {}.  Current epoch is {}.",
            epoch,
            self.get_current_epoch().await
        );
        loop {
            let current_epoch = self.get_current_epoch().await;
            if current_epoch == epoch {
                info!("Advanced! Current epoch is {}.", epoch);
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        info!("Sleeping for 2 seconds to make sure nodes sync up with new peer state...");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }

    pub async fn ensure_node_staked_and_joined(
        &self,
        node_account: &NodeAccount,
        node_addr: &str,
        node_port: usize,
    ) -> Result<NodeStakingStatus> {
        let node_signer = node_account.signing_provider.clone();
        let mut rng = ethers::core::rand::thread_rng();
        let secret_key = libsecp256k1::SecretKey::random(&mut rng);
        let public_key = libsecp256k1::PublicKey::from_secret_key(&secret_key);

        info!(
            "Checking if node {} is already staked...",
            node_signer.address()
        );

        // stake if not already
        let is_staked = self
            .contracts
            .staking_balances
            .check_staking_amounts(node_account.staker_address)
            .call()
            .await;
        if let Ok(is_staked) = is_staked {
            if is_staked {
                info!("Node {} is already staked!", node_signer.address());
            } else {
                info!("Node {} is not staked.  Staking...", node_signer.address());
                self.send_approve_and_stake(node_signer.clone()).await?;
            }
        }

        // request to join if not already
        let next_validators = self
            .contracts
            .staking
            .get_validators_in_next_epoch()
            .call()
            .await?;
        let is_joined = next_validators.contains(&node_account.staker_address);
        if !is_joined {
            info!("Node {} is not joined.  Joining...", node_signer.address());
            let peer_item = PeerItem::new(
                node_addr,
                public_key,
                node_account.node_address,
                node_account.coms_keys.sender_public_key(),
                node_account.coms_keys.receiver_public_key(),
                node_account.staker_address,
                PeerValidatorStatus::Unknown,
                None,
                "0.0.1".to_string(),
            );
            self.send_request_to_join(node_signer, 2130706433u32, node_port as u32, &peer_item)
                .await?;
        }

        Ok(NodeStakingStatus::StakedAndJoined)
    }

    pub async fn update_staking_config(&self, staking_config: StakingContractConfig) -> Result<()> {
        Contracts::update_staking_config(self.contracts.staking.clone(), staking_config).await
    }
}
