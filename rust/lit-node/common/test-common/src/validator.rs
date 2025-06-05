use std::borrow::BorrowMut;
use std::fs;
use std::io::BufReader;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::sync::Arc;

use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_attestation::attestation::ENV_ATTESTATION_TYPE_OVERRIDE;
use lit_blockchain::contracts::staking::Staking;
use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_core::utils::binary::bytes_to_hex;
use lit_core::utils::toml::SimpleToml;
use lit_logging::config::ENV_LOGGING_TIMESTAMP;
use rand::Rng;
use std::fs::File;
use std::path::Path;
use tracing::{debug, info, trace, warn};

use crate::node_collection::choose_random_indices;

use super::testnet::actions::Actions;
use super::testnet::contracts::Contracts;
use super::testnet::contracts_repo::node_configs_path;
use super::testnet::NodeAccount;
use super::testnet::Testnet;

#[must_use]
pub struct ValidatorCollectionBuilder {
    node_config_folder_path: String,
    wait_initial_epoch: bool,
    wait_for_root_keys: bool,
    num_staked_nodes: usize,
    num_asleep_initially: usize,
    asleep_initially_override: Option<Vec<usize>>,
    custom_binary_path: Option<String>,
}

impl Default for ValidatorCollectionBuilder {
    fn default() -> Self {
        Self {
            node_config_folder_path: node_configs_path(),
            wait_initial_epoch: true,
            wait_for_root_keys: true,
            num_staked_nodes: 10,
            num_asleep_initially: 0,
            asleep_initially_override: None,
            custom_binary_path: None,
        }
    }
}

impl ValidatorCollectionBuilder {
    pub fn node_config_folder_path(mut self, node_config_folder_path: String) -> Self {
        self.node_config_folder_path = node_config_folder_path;
        self
    }

    pub fn wait_initial_epoch(mut self, wait_initial_epoch: bool) -> Self {
        self.wait_initial_epoch = wait_initial_epoch;
        self
    }

    pub fn wait_for_root_keys(mut self, wait_for_root_keys: bool) -> Self {
        self.wait_for_root_keys = wait_for_root_keys;
        self
    }

    pub fn num_staked_nodes(mut self, num_staked_nodes: usize) -> Self {
        self.num_staked_nodes = num_staked_nodes;
        self
    }

    pub fn num_asleep_initially(mut self, num_asleep_initially: usize) -> Self {
        self.num_asleep_initially = num_asleep_initially;
        self
    }

    pub fn asleep_initially_override(
        mut self,
        asleep_initially_override: Option<Vec<usize>>,
    ) -> Self {
        self.asleep_initially_override = asleep_initially_override;
        self
    }

    pub fn custom_binary_path(mut self, custom_binary_path: Option<String>) -> Self {
        self.custom_binary_path = custom_binary_path;
        self
    }

    pub async fn build(self, testnet: &Testnet, actions: &Actions) -> Result<ValidatorCollection> {
        // make sure the old nodes are killed
        Command::new("pkill")
            .arg("-9")
            .arg("lit_node")
            .output()
            .map_err(|e| anyhow::anyhow!("failed to kill lit_node: {}", e))?;

        // erase all the old node logs
        let logs_path = "./test_logs";
        if std::path::Path::new(&logs_path).exists() {
            std::fs::remove_dir_all(logs_path)
                .map_err(|e| anyhow::anyhow!("failed to remove test_logs directory: {}", e))?;
        }
        std::fs::create_dir_all(logs_path)
            .map_err(|e| anyhow::anyhow!("failed to create test_logs directory: {}", e))?;

        // Choose random indices between 0 and num_nodes that will NOT be awake initially.
        let asleep_initially =
            choose_random_indices(self.num_staked_nodes, self.num_asleep_initially);
        info!(
            "TEST: Nodes at indices {:?} will be asleep initially",
            asleep_initially
        );

        let mut validators = vec![];
        for i in 0..self.num_staked_nodes {
            let node_account = &testnet.node_accounts[i];
            let node_config_file_path =
                format!("{}/lit_config{:?}.toml", self.node_config_folder_path, i);

            validators.push(
                ValidatorBuilder::default()
                    .build_mode(
                        self.custom_binary_path
                            .clone()
                            .map(|p| BuildMode::UseCustomBuild(p)),
                    )
                    .build(
                        node_config_file_path,
                        node_account,
                        actions.clone(),
                        testnet.deploy_account.signing_provider.clone(),
                    )
                    .await?,
            );
        }

        let mut validator_ports_to_check_awake = Vec::new();
        for (idx, validator) in validators.iter_mut().enumerate() {
            if let Some(asleep_initially) = &self.asleep_initially_override {
                info!("Using asleep_initially_override: {:?}", asleep_initially);
                if asleep_initially.contains(&idx) {
                    continue;
                } else {
                    validator.start_node(true, false).await?;
                    validator_ports_to_check_awake.push(validator.node.port);
                }
            } else {
                // only start the nodes meant to be awake.
                if !asleep_initially.contains(&idx) {
                    validator.start_node(true, false).await?;
                    validator_ports_to_check_awake.push(validator.node.port);
                }
            }
        }

        // wait for all nodes to be awake once all node processes have been started - this is faster than
        // starting a process and waiting for each node to be awake one by one.
        ValidatorCollection::ensure_all_nodes_awake(validator_ports_to_check_awake).await?;

        // wait for active
        if self.wait_initial_epoch {
            actions.wait_for_active().await;
        }

        // wait for the root keys to be registered
        if self.wait_for_root_keys {
            actions.wait_for_root_keys().await;
        }

        Ok(ValidatorCollection {
            validators,
            actions: actions.clone(),
            testnet_deployer_signing_provider: testnet.deploy_account.signing_provider.clone(),
            testnet_node_accounts: testnet.node_accounts.clone(),
            node_config_folder_path: self.node_config_folder_path,
        })
    }
}

pub struct ValidatorCollection {
    validators: Vec<Validator>,
    actions: Actions,
    testnet_deployer_signing_provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    testnet_node_accounts: Arc<Vec<NodeAccount>>,

    // build parameters
    node_config_folder_path: String,
}

impl ValidatorCollection {
    pub fn asleep_nodes(&self) -> Vec<&Validator> {
        self.validators
            .iter()
            .filter(|v| v.node.is_offline())
            .collect::<Vec<_>>()
    }

    pub fn builder() -> ValidatorCollectionBuilder {
        ValidatorCollectionBuilder::default()
    }

    pub fn config_files(&self) -> Vec<String> {
        self.validators
            .iter()
            .map(|v| v.node.config_file.to_string())
            .collect()
    }

    pub fn log_readers(&self) -> Vec<BufReader<File>> {
        self.validators.iter().map(|v| v.log_reader()).collect()
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub fn addresses(&self) -> Vec<String> {
        self.validators
            .iter()
            .map(|v| format!("http://{}:{}", v.node.ip, v.node.port))
            .collect()
    }

    pub fn ports(&self) -> Vec<usize> {
        self.validators.iter().map(|v| v.node.port).collect()
    }

    pub fn max_port(&self) -> usize {
        self.validators.iter().map(|v| v.node.port).max().unwrap()
    }

    pub fn size(&self) -> usize {
        self.validators.len()
    }

    pub fn threshold(&self) -> usize {
        (self.validators.len() * 2) / 3
    }

    pub fn get_validator_by_idx(&self, idx: usize) -> &Validator {
        &self.validators[idx]
    }

    pub fn get_validator_by_account(&self, account: &NodeAccount) -> Option<&Validator> {
        self.validators.iter().find(|v| v.account == *account)
    }

    /// Builds a new validator node, starts it if not specified to be asleep, and then waits for it to sync up to the chain.
    pub async fn add_one(
        &mut self,
        is_asleep: bool,
        build_mode: Option<BuildMode>,
    ) -> Result<&Validator> {
        // get the next node account from the testnet
        let validator_idx = self.validators.len();
        let node_account = &self.testnet_node_accounts[validator_idx];
        let node_config_file_path = format!(
            "{}/lit_config{:?}.toml",
            self.node_config_folder_path, validator_idx
        );

        // build the validator
        let mut validator = ValidatorBuilder::default()
            .build_mode(build_mode)
            .build(
                node_config_file_path,
                node_account,
                self.actions.clone(),
                self.testnet_deployer_signing_provider.clone(),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to build validator with error: {}", e))?;

        // start the validator if not specified to be asleep
        if !is_asleep {
            validator
                .start_node(true, true)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to start validator with error: {}", e))?;
        }

        // the validator requests to join the next validator set
        validator
            .request_to_join(&self.actions)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to request to join with error: {}", e))?;

        // Check that all the nodes have synced up to chain.
        self.actions.wait_for_root_keys().await;

        self.validators.push(validator);

        Ok(&self.validators[validator_idx])
    }

    /// Builds a new validator node, starts it if not specified to be asleep, and then waits for it to sync up to the chain,
    /// using custom node config file path and node account.
    pub async fn add_one_custom(
        &mut self,
        is_asleep: bool,
        node_config_file_path: String,
        node_account: &NodeAccount,
        build_mode: Option<BuildMode>,
    ) -> Result<&Validator> {
        // build the validator
        let mut validator = ValidatorBuilder::default()
            .build_mode(build_mode)
            .build(
                node_config_file_path,
                node_account,
                self.actions.clone(),
                self.testnet_deployer_signing_provider.clone(),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to build validator with error: {}", e))?;

        // start the validator if not specified to be asleep
        if !is_asleep {
            validator
                .start_node(true, true)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to start validator with error: {}", e))?;
        }

        // the validator requests to join the next validator set
        validator
            .request_to_join(&self.actions)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to request to join with error: {}", e))?;

        // Check that all the nodes have synced up to chain.
        self.actions.wait_for_root_keys().await;

        self.validators.push(validator);

        Ok(&self.validators[self.validators.len() - 1])
    }

    pub async fn ensure_all_nodes_started(&mut self) -> Result<()> {
        for validator in &mut self.validators {
            if validator.node.is_offline() {
                validator.start_node(true, true).await?;
            }
        }

        Ok(())
    }

    pub async fn ensure_all_nodes_awake(ports: Vec<usize>) -> Result<()> {
        info!("Waiting for ports to be awake: {:?}", ports);
        for port in ports {
            Node::wait_for_node_awake(port).await?;
        }

        Ok(())
    }

    pub async fn start_node(&mut self, idx: usize) -> Result<()> {
        self.validators[idx].start_node(false, true).await
    }

    pub async fn start_node_from_clean_slate(&mut self, idx: usize) -> Result<()> {
        self.validators[idx].start_node(true, true).await
    }

    pub fn stop_all_nodes(&mut self) -> Result<()> {
        for validator in &mut self.validators {
            validator.node.stop_node_process()?;
        }

        Ok(())
    }

    pub fn stop_nodes_with_accounts(&mut self, accounts: Vec<NodeAccount>) -> Result<()> {
        // first find the indices of the validators with these accounts
        let indices = accounts
            .iter()
            .map(|account| {
                self.validators
                    .iter()
                    .position(|v| v.account == *account)
                    .expect("Failed to find validator with this account")
            })
            .collect::<Vec<_>>();

        // then, stop the node process for each of these accounts
        for idx in indices {
            self.validators[idx].node.stop_node_process()?;
        }

        Ok(())
    }

    pub async fn stop_node(&mut self, idx: usize) -> Result<()> {
        info!("Stopping node at index: {}", idx);
        self.validators[idx].node.stop_node_process()
    }

    pub async fn stop_random_node(&mut self) -> Result<usize> {
        let mut rng = crate::rand::thread_rng();
        let random_node_idx_to_shutdown = rng.gen_range(0..self.size());
        info!("Stopping node at index: {}", random_node_idx_to_shutdown);
        self.validators[random_node_idx_to_shutdown]
            .node
            .stop_node_process()?;
        Ok(random_node_idx_to_shutdown)
    }

    pub async fn get_active_validators(&self) -> Result<Vec<&Validator>> {
        let current_active_validators = self.actions.get_current_validators().await;

        Ok(self
            .validators
            .iter()
            .filter(|v| current_active_validators.contains(&v.account.staker_address))
            .collect::<Vec<_>>())
    }

    pub async fn get_inactive_validators(&self) -> Result<Vec<&Validator>> {
        let current_active_validators = self.actions.get_current_validators().await;

        Ok(self
            .validators
            .iter()
            .filter(|v| !current_active_validators.contains(&v.account.staker_address))
            .collect::<Vec<_>>())
    }

    pub async fn random_validators_request_to_leave(
        &self,
        num_dealt_out: usize,
    ) -> Result<Vec<&Validator>> {
        // first get the current active validators
        let current_active_validators = self.get_active_validators().await?;

        // randomly choose validators from this list
        let random_validators_to_leave =
            choose_random_nums_in_range(num_dealt_out, 0, current_active_validators.len());
        let random_validators_to_leave = random_validators_to_leave
            .iter()
            .map(|i| current_active_validators[*i])
            .collect::<Vec<_>>();

        // for each of these validators, request to leave
        for validator in &random_validators_to_leave {
            validator.request_to_leave(&self.actions).await?;
        }

        Ok(random_validators_to_leave)
    }

    pub async fn random_validators_request_to_join(
        &mut self,
        num_dealt_in: usize,
    ) -> Result<Vec<&Validator>> {
        // scoped block so the immutable borrow is within it, and we can continue the mutable borrow outside of this block below it.
        let random_validators_to_join = {
            // first get the current inactive validators
            let current_inactive_validators = self.get_inactive_validators().await?;

            // randomly choose validators from this list
            let random_validators_to_join =
                choose_random_nums_in_range(num_dealt_in, 0, current_inactive_validators.len());
            // find which indices they belong to in the original list in self
            random_validators_to_join
                .iter()
                .map(|i| {
                    self.validators
                        .iter()
                        .position(|v| v.account == current_inactive_validators[*i].account)
                        .unwrap()
                })
                .collect::<Vec<_>>()
        };

        // for each of these validators, first spin up the node (remember, once the validator has joined in the contract,
        // they are assumed to already be online as its peers will be sending them messages)
        for idx in random_validators_to_join.clone() {
            let validator = self.validators[idx].borrow_mut();
            validator.start_node(false, true).await?;
        }

        // for each of these validators, request to join
        for idx in random_validators_to_join.clone() {
            let validator = &self.validators[idx];
            validator.request_to_join(&self.actions).await?;
        }

        Ok(self
            .validators
            .iter()
            .enumerate()
            .filter(|(i, _)| random_validators_to_join.contains(i))
            .map(|(_, v)| v)
            .collect::<Vec<_>>())
    }
}

impl Drop for ValidatorCollection {
    fn drop(&mut self) {
        info!("Stopping processes");
        self.stop_all_nodes().expect("Failed to stop nodes");
    }
}

pub struct ValidatorBuilder {
    node_binary_feature_flags: String,
    build_mode: Option<BuildMode>,
}

impl Default for ValidatorBuilder {
    fn default() -> Self {
        Self {
            node_binary_feature_flags: "lit-actions,testing".into(),
            build_mode: None,
        }
    }
}

impl ValidatorBuilder {
    pub fn node_binary_feature_flags(mut self, node_binary_feature_flags: String) -> Self {
        self.node_binary_feature_flags = node_binary_feature_flags;
        self
    }

    pub fn build_mode(mut self, build_mode: Option<BuildMode>) -> Self {
        self.build_mode = build_mode;
        self
    }

    pub async fn build(
        self,
        node_config_file_path: String,
        node_account: &NodeAccount,
        _actions: Actions,
        deployer_signing_provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    ) -> Result<Validator> {
        // check and top up node funds if necessary
        ensure_node_account_funds(
            deployer_signing_provider,
            node_account.signing_provider.clone(),
        )
        .await?;

        // build the corresponding node if not in CI or overridden.
        let is_not_in_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string()) != "1";
        if self.build_mode.is_some() {
            match self.build_mode.unwrap() {
                BuildMode::UseNewBuild => {
                    let binary_path = Node::build_binary(
                        self.node_binary_feature_flags.clone(),
                        node_config_file_path.clone(),
                    )?;
                    return Ok(Validator {
                        node: NodeBuilder::new()
                            .binary_path(binary_path)
                            .build(node_config_file_path)
                            .await?,
                        account: node_account.clone(),
                    });
                }
                BuildMode::UseCustomBuild(binary_path) => {
                    info!("Using custom binary path: {}", binary_path);
                    return Ok(Validator {
                        node: NodeBuilder::new()
                            .binary_path(binary_path)
                            .build(node_config_file_path)
                            .await?,
                        account: node_account.clone(),
                    });
                }
            }
        } else if is_not_in_ci {
            let binary_path = Node::build_binary(
                self.node_binary_feature_flags.clone(),
                node_config_file_path.clone(),
            )?;
            return Ok(Validator {
                node: NodeBuilder::new()
                    .binary_path(binary_path)
                    .build(node_config_file_path)
                    .await?,
                account: node_account.clone(),
            });
        }

        Ok(Validator {
            node: NodeBuilder::new().build(node_config_file_path).await?,
            account: node_account.clone(),
        })
    }
}

pub struct Validator {
    node: Node,
    account: NodeAccount,
}

impl Validator {
    fn log_reader(&self) -> BufReader<File> {
        let path = format!(
            "./test_logs/0x{}",
            bytes_to_hex(self.account.staker_address.as_bytes())
        );
        info!("Trying to open path: {}", path);
        let log_path = PathBuf::from(path);
        let file = File::open(log_path).expect("Failed to open log file");
        BufReader::new(file)
    }

    pub fn account(&self) -> &NodeAccount {
        &self.account
    }

    pub async fn start_node(&mut self, clean_slate: bool, wait_for_node_awake: bool) -> Result<()> {
        if clean_slate {
            // remove the validator-specific files
            info!(
                "Cleaning environment of any files for node: {}",
                self.account.staker_address
            );
            remove_files(format!(
                "0x{}",
                bytes_to_hex(self.account.staker_address.as_bytes())
            ));
        }

        // start the node
        self.node
            .start_node_process()
            .map_err(|e| anyhow::anyhow!("Failed to start node with error: {}", e))?;

        if wait_for_node_awake {
            // check the node is awake
            Node::wait_for_node_awake(self.node.port)
                .await
                .map_err(|e| {
                    anyhow::anyhow!("Failed to wait for node to wake up with error: {}", e)
                })?;
        }

        Ok(())
    }

    pub fn stop_node(&mut self) -> Result<()> {
        info!("Stopping node at port {}", self.node.port);
        self.node
            .stop_node_process()
            .map_err(|e| anyhow::anyhow!("Failed to stop node with error: {}", e))
    }

    pub async fn request_to_leave(&self, actions: &Actions) -> Result<()> {
        info!(
            "Node {} ({}) requesting to leave",
            self.node.port, self.account.staker_address
        );

        let staking = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            actions.contracts().staking.address(),
            self.account.signing_provider.clone(),
        );

        let cc = staking.request_to_leave();
        Contracts::process_contract_call(cc, "request to leave").await;

        Ok(())
    }

    pub async fn request_to_join(&self, actions: &Actions) -> Result<()> {
        info!(
            "Node {} ({}) requesting to join",
            self.node.port, self.account.staker_address
        );

        let staking = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
            actions.contracts().staking.address(),
            self.account.signing_provider.clone(),
        );

        let cc = staking.request_to_join(
            self.node.ip.into(),
            0,
            self.node.port as u32,
            self.account.node_address,
            self.account.coms_keys.sender_public_key().into(),
            self.account.coms_keys.receiver_public_key().into(),
        );
        Contracts::process_contract_call(cc, "request to join").await;

        Ok(())
    }
}

#[must_use]
pub struct NodeBuilder {
    asleep_initially: bool,
    binary_path: Option<String>,
    in_github_ci: bool,
    log_mode: String,
    extra_env_vars: Vec<(String, String)>,
}

impl NodeBuilder {
    pub fn new() -> Self {
        let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string()) == "1";
        let is_perf_test = std::env::var("LIT_PERFORMANCE_TEST").unwrap_or("0".to_string()) == "1";

        Self {
            asleep_initially: false,
            binary_path: None,
            in_github_ci,
            log_mode: in_github_ci
                .then(|| "_=warn,lit_node=trace,lit_actions=trace".to_string())
                .unwrap_or("_=warn,lit_node=trace".to_string()),
            //.unwrap_or("_=warn,test_common=trace,lit_node::tss=trace,lit_node::peers::peer_state::backup_recovery=debug,lit_node::pkp::utils=debug,lit_node::tasks::beaver_manager::listener=trace,lit_node::peers::peer_state=info,lit_node::peers::peer_reviewer=info,lit_node::tasks::fsm=trace,lit_node::tss::ecdsa_cait_sith=trace,lit_actions=trace,lit_node::p2p_comms::comms=debug,lit_actions=trace,lit_node::functions::action_client=trace".to_string()),
            extra_env_vars: is_perf_test
                .then(|| vec![("LIT_LOGGING_JAEGER".to_string(), "1".to_string())])
                .unwrap_or(vec![]),
        }
    }

    pub fn asleep_initially(mut self, asleep_initially: bool) -> Self {
        self.asleep_initially = asleep_initially;
        self
    }

    pub fn binary_path(mut self, binary_path: String) -> Self {
        self.binary_path = Some(binary_path);
        self
    }

    pub fn log_mode(mut self, log_mode: String) -> Self {
        self.log_mode = log_mode;
        self
    }

    pub fn extra_env_vars(mut self, extra_env_vars: Vec<(String, String)>) -> Self {
        self.extra_env_vars = extra_env_vars;
        self
    }

    pub async fn build(self, config_file: String) -> Result<Node> {
        // if we're in CI, it's already built and in the root
        let path = self
            .binary_path
            .unwrap_or("./target/debug/lit_node".to_string());
        info!("Config file: {}", config_file);

        // get the actual node port from the config file
        let node_config = SimpleToml::try_from(Path::new(&config_file))
            .map_err(|e| anyhow::anyhow!("Failed to load config file: {}", e))?;
        debug!("Node config: {:?}", node_config.data());
        let node_external_port = Node::get_node_port_from_config_file(&config_file)?;
        let node_ip = Node::get_node_ip_from_config_file(&config_file)?;

        info!("Node port: {}", node_external_port);
        Ok(Node {
            process: None,
            config_file,
            binary_path: path,
            log_mode: self.log_mode,
            extra_env_vars: self.extra_env_vars,

            port: node_external_port,
            ip: node_ip,
        })
    }
}

pub struct Node {
    process: Option<Child>,
    config_file: String,
    binary_path: String,
    log_mode: String,
    extra_env_vars: Vec<(String, String)>,

    // convenience things
    port: usize,
    ip: Ipv4Addr,
}

impl Node {
    pub fn is_offline(&self) -> bool {
        self.process.is_none()
    }

    fn start_node_process(&mut self) -> Result<()> {
        if self.process.is_some() {
            warn!("Node {} is already online", self.port);
            return Ok(());
        }

        info!("Starting node at port {}", self.port);
        let mut process = Command::new(&self.binary_path)
            .env(ENV_LIT_CONFIG_FILE, &self.config_file)
            .env("RUST_LOG", &self.log_mode)
            .env(ENV_ATTESTATION_TYPE_OVERRIDE, "ADMIN_SIGNED")
            .env(
                ENV_LOGGING_TIMESTAMP,
                std::env::var(ENV_LOGGING_TIMESTAMP).unwrap_or("0".to_string()),
            )
            .envs(self.extra_env_vars.clone())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to launch node with error: {}", e))?;

        // Check if the process has exited due to some failure using the specified binary, after waiting for 1s.
        std::thread::sleep(std::time::Duration::from_secs(1));
        if let Some(exit_status) = process.try_wait()? {
            return Err(anyhow::anyhow!(
                "Node process could not be started, exit code: {:?}",
                exit_status
            ));
        }

        self.process = Some(process);
        Ok(())
    }

    fn stop_node_process(&mut self) -> Result<()> {
        let process = match self.process.as_mut() {
            Some(process) => process,
            None => {
                debug!("Node is already offline");
                return Ok(());
            }
        };

        let pid = process.id();
        info!("Killing PID: {}, PORT: {}", pid, self.port);

        // just kill the nodes - don't wait for them to stop
        process
            .kill()
            .map_err(|e| anyhow::anyhow!("Failed to kill node with error: {}", e))?;
        process
            .wait()
            .map_err(|e| anyhow::anyhow!("Failed to wait for child PID {pid}: {}", e))?;
        info!("process at port {} was killed", self.port);

        self.process = None;

        Ok(())
    }

    pub async fn wait_for_node_awake(port: usize) -> Result<()> {
        // loop until the node is awake
        let mut node_awake = false;
        while !node_awake {
            let noonce = hex::encode([0u8; 32]);
            node_awake = Self::check_node_awake(port, noonce).await?;
            if !node_awake {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
        info!("Node {} is responding", port);

        Ok(())
    }

    async fn check_node_awake(port: usize, noonce: String) -> Result<bool> {
        if let Ok(resp) =
            reqwest::get(format!("http://127.0.0.1:{}/connect/{}", port, noonce)).await
        {
            trace!("port {} is responding {:#?}", port, resp);
            Ok(true)
        } else {
            trace!("port {} is not responding", port);
            Ok(false)
        }
    }

    fn get_node_config_from_file(config_file: &str) -> Result<SimpleToml> {
        SimpleToml::try_from(Path::new(config_file))
            .map_err(|e| anyhow::anyhow!("Failed to load config file: {}", e))
    }

    pub fn get_node_port_from_config_file(config_file: &str) -> Result<usize> {
        // get the actual node port from the config file
        let node_config = Self::get_node_config_from_file(config_file)?;
        node_config
            .get("node.http", "port")
            .ok_or_else(|| anyhow::anyhow!("Failed to get port from config file"))?
            .parse::<usize>()
            .map_err(|e| anyhow::anyhow!("Failed to parse port from config file: {}", e))
    }

    pub fn get_node_ip_from_config_file(config_file: &str) -> Result<Ipv4Addr> {
        // get the actual node port from the config file
        let node_config = Self::get_node_config_from_file(config_file)?;
        let node_ip = node_config
            .get("node", "domain")
            .ok_or_else(|| anyhow::anyhow!("Failed to get ip from config file"))?
            .parse::<String>()
            .map_err(|e| anyhow::anyhow!("Failed to parse ip from config file: {}", e))?;
        node_ip
            .parse::<Ipv4Addr>()
            .map_err(|e| anyhow::anyhow!("Failed to parse ip from config file: {}", e))
    }

    pub fn get_node_network_addr_from_config_file(config_file: &str) -> Result<String> {
        // get the actual node networking address from the config file
        let _node_config = Self::get_node_config_from_file(config_file)?;
        let node_port = Self::get_node_port_from_config_file(config_file)?;
        let node_ip = Self::get_node_ip_from_config_file(config_file)?;

        Ok(format!("http://{}:{}", node_ip, node_port))
    }

    pub fn build_binary(feature_flags: String, node_config_file_path: String) -> Result<String> {
        #[cfg(not(feature = "shiva-path"))]
        let args = ["build", "--features", &feature_flags];

        #[cfg(feature = "shiva-path")]
        let args = [
            "build",
            "--manifest-path=../../Cargo.toml",
            "--features",
            &feature_flags,
        ];

        let start = std::time::Instant::now();
        info!(
            "Building node code with these args : {:?}  - please be patient.",
            &args
        );
        let build_command = Command::new("cargo")
            .args(args)
            .output()
            .expect("Failed to compile node with cargo build");
        assert!(
            build_command.status.success(),
            "We failed to build the node"
        );

        info!(
            "Building took {} ms.  Starting nodes.  Env: {:?}",
            start.elapsed().as_millis(),
            std::env::current_dir()
        );

        // We copy the binary to a dedicated location. This is necessary because the existing binary cannot be overwritten,
        // especially when the node is running in a CI context.
        let node_external_port = Self::get_node_port_from_config_file(&node_config_file_path)?;
        #[cfg(not(feature = "shiva-path"))]
        let from_binary_path = "./target/debug/lit_node";
        #[cfg(feature = "shiva-path")]
        let from_binary_path = "../../target/debug/lit_node";

        let to_binary_path = &format!("{}_{}", from_binary_path, node_external_port);

        std::fs::copy(from_binary_path, to_binary_path)
            .map_err(|e| anyhow::anyhow!("Failed to copy binary with error: {}", e))?;
        Ok(to_binary_path.to_string())
    }
}

#[derive(Debug)]
pub enum BuildMode {
    UseNewBuild,
    UseCustomBuild(String), // path to the binary
}

fn remove_files(staker_address: String) {
    let path = format!("./node_keys/{}", staker_address);
    if let Err(e) = fs::remove_dir_all(&path) {
        warn!("Failed to remove directory: {} with error {}", path, e);
    }

    let path = format!("./test_logs/{}", staker_address);
    if let Err(e) = fs::remove_file(&path) {
        warn!("Failed to remove file: {} with error {}", path, e);
    }
}

async fn ensure_node_account_funds(
    deployer_signing_provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    node_account: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> Result<bool> {
    let deployer_balance = deployer_signing_provider
        .get_balance(deployer_signing_provider.address(), None)
        .await?;
    let node_balance = deployer_signing_provider
        .get_balance(node_account.address(), None)
        .await?;

    let min_balance: U256 = 1_000_000_000_000_000u128.into(); // .01 MATIC / ETH, whatever

    if deployer_balance < min_balance * 10 {
        return Err(anyhow::anyhow!(
            "Deployer balance {} is too low to top up node {}",
            deployer_balance,
            node_account.address()
        ));
    }

    if node_balance > min_balance {
        info!(
            "Node {} balance: {} - acceptable!",
            node_account.address(),
            node_balance
        );
        return Ok(true);
    }

    info!(
        "Node {} balance {} is low.  Sending funds from deployer who has {}.",
        node_account.address(),
        node_balance,
        deployer_balance,
    );

    let tx = TransactionRequest::new()
        .to(node_account.address())
        .value(min_balance * 2)
        .from(deployer_signing_provider.address());

    let tx_hash = deployer_signing_provider.send_transaction(tx, None).await?;
    let _receipt = tx_hash.await?;

    let node_balance = deployer_signing_provider
        .get_balance(node_account.address(), None)
        .await?;

    info!(
        "Node {} topped up -  balance {}",
        node_account.address(),
        node_balance
    );
    Ok(true)
}

fn choose_random_nums_in_range(random_nums: usize, min: usize, max: usize) -> Vec<usize> {
    if random_nums > max - min {
        panic!(
            "Cannot choose {} random numbers in range {} to {}",
            random_nums, min, max
        );
    }

    debug!(
        "Choosing {} random numbers in range {} to {}",
        random_nums, min, max
    );
    let mut rng = crate::rand::thread_rng();
    let mut random_nums_in_range = vec![];
    while random_nums_in_range.len() < random_nums {
        let random_num = rng.gen_range(min..max);
        if !random_nums_in_range.contains(&random_num) {
            random_nums_in_range.push(random_num);
        }
    }

    random_nums_in_range
}
