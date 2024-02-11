use crate::common::testnet::contracts_repo::node_configs_path;

use super::testnet::scenario::{InitialDKGState, Scenario};

use anyhow::Result;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::{Http, Provider, SignerMiddleware, Wallet};
use ethers::providers::Middleware;
use ethers::types::{TransactionRequest, U256};
use futures::future::join_all;
use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_core::utils::binary::bytes_to_hex;
use lit_core::utils::toml::SimpleToml;
use lit_logging::config::ENV_LOGGING_TIMESTAMP;
use lit_node::models::JsonSDKHandshakeResponse;
use lit_node::peers::peer_item::PeerItem;
use log::{info, trace};
use reqwest::Response;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::fs::File;

use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Child;
use tracing::{error, warn};

use lit_attestation::attestation::ENV_ATTESTATION_TYPE_OVERRIDE;
use reqwest;
use std::process::Command;
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeStakingStatus {
    PreviouslyStaked,
    Staked,
    FailedToStake,
    Unstaked,
}

///////////////////////////////
// This makes concurrent tests easier as we can be running many teams of nodes at the same time.
// No secrets are stored because we want to load them fresh from the file during testing
// However initial port 7470 is currently hardcoded in models.rs
pub struct NodeCollection {
    pub processes: Vec<Option<std::process::Child>>,
    pub portnames: Vec<String>,
    pub node_info_arr: Vec<PeerItem>,
    pub config_files: Vec<String>,
    pub log_readers: Vec<BufReader<File>>,

    /// The indicies of the nodes that are asleep initially.
    asleep_initially: HashSet<usize>,
}

// Config that will be used for launching node.
// May either be passed as environment variables, or loaded into a config file.
#[derive(Debug)]
pub struct NodeConfig {
    pub lit_domain_name: String,
    pub rocket_port: String,
    pub staker_address: String,
    pub enable_proxied_http_client: Option<bool>,
}

/// NOTE: This does not currently support building the lit_node for fault tests yet.
pub fn build_lit_node() {
    // First, clear the executable at ./target/debug/lit_node
    std::fs::remove_file("./target/debug/lit_node").expect("Failed to remove lit_node");
    info!("Removed old lit_node executable");

    let args = ["build", "--features", "lit-actions,testing"];
    let start = std::time::Instant::now();
    info!(
        "Building node code with these args : {:?}  - please be patient.",
        &args
    );
    let build_command = Command::new("cargo")
        .args(args)
        .spawn()
        .expect("Could not spawn build")
        .wait()
        .expect("Failed to compile node with cargo build");
    if !build_command.success() {
        panic!("We failed to build the node");
    }

    info!(
        "Building took {} ms.  Starting nodes.  Env: {:?}",
        start.elapsed().as_millis(),
        std::env::current_dir()
    );
}

impl NodeCollection {
    pub fn init_log_readers(&mut self) {
        let mut log_readers = Vec::new();
        for node in self.node_info_arr.iter() {
            // read from the log file
            let path = format!(
                "./test_logs/0x{}",
                bytes_to_hex(node.staker_address.as_bytes())
            );
            info!("Trying to open path {:?}", path);
            let log_path = PathBuf::from(path);
            let file = File::open(log_path);
            match file {
                Err(e) => {
                    error!("Could not open log file: {:?}", e);
                }
                Ok(file) => {
                    let reader = BufReader::new(file);

                    log_readers.push(reader);
                }
            }
        }
        self.log_readers = log_readers;
    }

    pub fn clear_log_buffers(&mut self) {
        // clear the log buffer
        for reader in self.log_readers.iter_mut() {
            let _lines = reader
                .lines()
                .map(|line| line.unwrap_or("".to_string()))
                .collect::<Vec<String>>();
        }
    }

    pub fn get_logs(&mut self) -> Vec<Vec<String>> {
        let mut logs = Vec::new();
        for reader in self.log_readers.iter_mut() {
            let lines = reader
                .lines()
                .map(|line| line.unwrap_or("".to_string()))
                .collect::<Vec<String>>();
            logs.push(lines);
        }
        logs
    }

    pub fn asleep_initially(&mut self) -> &HashSet<usize> {
        &self.asleep_initially
    }

    /// This method adds a new node to the node collection and starts it.
    pub async fn extend(
        &mut self,
        scenario: &Scenario,
        portname: String,
        config_file: String,
        node_index: usize,
        force_rebuild: bool,
    ) {
        // clean env before booting nodes
        info!("Cleaning environment of any files for new node");
        let staker_address = get_staker_address_from_config_file(&config_file);
        Self::remove_files(vec![staker_address]);

        let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string());
        let mut binary_path_and_name = None;
        if in_github_ci != "1" || force_rebuild {
            build_lit_node();
            // copy to a new name because we can't overwrite the existing running binary
            let formatted_path = format!("./target/debug/lit_node_{}", portname.clone());
            binary_path_and_name = Some(formatted_path.clone());
            let _ = std::fs::copy("./target/debug/lit_node", formatted_path)
                .expect("Failed to copy lit_node");
        }

        // you can use the below if you want to have special case logging for a certain node number.  Set the first 0 to _ to have it apply to all nodes
        let node_process = Self::start_node_process(&config_file, node_index, binary_path_and_name);
        let check_awake_res = NodeCollection::nodes_awake(vec![portname.clone()]).await;
        if check_awake_res.is_err() {
            panic!(
                "Unable to check node is awake: {}",
                check_awake_res.unwrap_err()
            );
        }

        info!("In extend() and node {} is awake.", portname);

        // Update parameters
        self.portnames.push(portname.clone());
        self.config_files.push(config_file.clone());
        self.processes.push(Some(node_process));
        self.node_info_arr.extend(check_awake_res.unwrap());

        // Check that all the nodes have synced up to chain.
        scenario.actions.wait_for_root_keys(scenario).await;
    }

    fn get_log_mode_for_node_index(_i: usize) -> String {
        // different log levels for local and CI
        let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string());
        if in_github_ci == "1" {
            "_=warn,lit_node=trace".to_string()
        } else {
            // you can use the below if you want to have special case logging for a certain node number.  Set the first 0 to _ to have it apply to all nodes
            "_=warn,lit_node=warn,lit_node::tss=trace,lit_node::peers::peer_state::backup_recovery=debug,lit_node::pkp::utils=debug,lit_node::tasks::beaver_manager::listener=trace,lit_node::peers=warn,lit_node::tasks::fsm=trace,lit_node::tss::ecdsa_cait_sith=debug".to_string()
        }
    }

    fn get_extra_env_vars_for_node_index(i: usize) -> Vec<(String, String)> {
        // you can use the below if you want to have special case logging for a certain node number.  Set the first 0 to _ to have it apply to all nodes
        if let Ok(s) = std::env::var("LIT_PERFORMANCE_TEST") {
            if s == "1" {
                match i {
                    0 => vec![("LIT_LOGGING_JAEGER".to_string(), "1".to_string())],
                    _ => vec![],
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    pub async fn from_scenario(scenario: Scenario) -> (NodeCollection, Scenario) {
        if scenario.num_awake_initially > scenario.num_nodes {
            panic!("num_awake_initially must be less than or equal to num_nodes");
        } else if scenario.num_staked > scenario.num_nodes {
            panic!("num_staked must be less than or equal to num_nodes");
        }

        // Choose random indices between 0 and num_nodes that will NOT be awake initially.
        let asleep_initially = choose_random_indices(
            scenario.num_nodes,
            scenario.num_nodes - scenario.num_awake_initially,
        );
        info!(
            "TEST: Nodes at indices {:?} will be asleep initially",
            asleep_initially
        );

        let mut processes = Vec::new();
        let mut portnames = Vec::new();
        let mut config_files = Vec::new();

        for i in 0..scenario.num_nodes {
            let rocket_port = scenario.initial_port + i;
            portnames.push(rocket_port.to_string());
        }

        let config_folder = match scenario.testnet.existing_config_path.clone() {
            Some(path) => format!("./config/test/{}", path),
            None => node_configs_path(),
        };

        info!("Using config folder: {}", config_folder);

        // check the nodes to ensure they have funds.
        for i in 0..scenario.num_staked {
            let node_account = scenario.testnet.node_accounts[i].clone();
            let signer = &node_account.signing_provider;

            let r = Self::check_node_funds(
                &scenario.provider,
                &scenario.testnet.deploy_account.signing_provider,
                signer,
            )
            .await;

            if r.is_err() {
                panic!("Unable to check node funds: {}", r.unwrap_err());
            }

            if scenario.initial_dkg_state != InitialDKGState::TestIsStakingAndDKG {
                let node_account = scenario.testnet.node_accounts[i].clone();
                let s = scenario.actions.ensure_node_staked(node_account).await;
                if s.is_err() {
                    panic!(
                        "Unable to validate if node {} has staked: {}",
                        signer.address(),
                        r.unwrap_err()
                    );
                }
            }
        }

        // make sure the old nodes are killed
        Command::new("pkill")
            .arg("-9")
            .arg("lit_node")
            .output()
            .expect("failed to kill lit_node");

        // erase the old node logs
        let logs_path = "./test_logs";
        if std::path::Path::new(&logs_path).exists() {
            std::fs::remove_dir_all(logs_path).expect("Failed to remove test_logs directory");
        }
        std::fs::create_dir_all(logs_path).expect("Failed to create test_logs directory");

        let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string());
        if in_github_ci != "1" {
            let args = {
                if scenario.is_fault_test {
                    ["build", "--features", "proxy_http,lit-actions,testing"]
                } else {
                    ["build", "--features", "lit-actions,testing"]
                }
            };

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
        }

        let mut staker_addresses = Vec::new();
        for i in 0..scenario.num_nodes {
            let config_file = format!("{}/lit_config{:?}.toml", config_folder, i);
            config_files.push(config_file.clone());
            let staker_address = get_staker_address_from_config_file(&config_file);
            staker_addresses.push(staker_address);
        }
        info!("Cleaning environment of any files for nodes");
        Self::remove_files(staker_addresses);

        for i in 0..scenario.num_nodes {
            let config_file = &config_files[i];

            // Only start the node process if it's not been marked to be left asleep initially.
            if asleep_initially.contains(&i) {
                processes.push(None);
            } else {
                let node_process = Self::start_node_process(config_file, i, None);
                processes.push(Some(node_process));
            }
        }

        // Get the array of ports to check for awake response
        let mut portnames_to_check_awake = Vec::new();
        for (i, portname) in portnames.iter().enumerate() {
            if !asleep_initially.contains(&i) {
                portnames_to_check_awake.push(portname.clone());
            }
        }

        let check_awake_res = NodeCollection::nodes_awake(portnames_to_check_awake).await;
        if check_awake_res.is_err() {
            panic!(
                "Unable to check nodes are awake: {}",
                check_awake_res.unwrap_err()
            );
        }

        info!("Nodes are awake.");
        let nc = NodeCollection {
            processes,
            portnames,
            node_info_arr: check_awake_res.unwrap(),
            config_files,
            asleep_initially,
            log_readers: vec![], // will be initialized later
        };

        if scenario.initial_dkg_state != InitialDKGState::TestIsStakingAndDKG {
            scenario
                .actions
                .start_initial_epoch(scenario.wait_for_dkg_to_complete)
                .await;

            if scenario.wait_for_dkg_to_complete {
                scenario.actions.wait_for_root_keys(&scenario).await;
            }
        }

        (nc, scenario)
    }

    pub async fn from_config_files(
        portnames: Vec<String>,
        config_files: Vec<String>,
    ) -> NodeCollection {
        let mut processes = Vec::new();

        // clean env before booting nodes
        info!("Cleaning environment of any files for nodes");
        let staker_addresses = config_files
            .iter()
            .map(|c| get_staker_address_from_config_file(c))
            .collect();
        Self::remove_files(staker_addresses);

        let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string());
        if in_github_ci != "1" {
            build_lit_node();
        }

        for i in 0..portnames.len() {
            let config_file = config_files[i].clone();
            let node_process = Self::start_node_process(&config_file, i, None);
            processes.push(Some(node_process));
        }

        let check_awake_res = NodeCollection::nodes_awake(portnames.clone()).await;
        if check_awake_res.is_err() {
            panic!(
                "Unable to check nodes are awake: {}",
                check_awake_res.unwrap_err()
            );
        }

        info!("Nodes are awake.");

        NodeCollection {
            processes,
            portnames,
            node_info_arr: check_awake_res.unwrap(),
            config_files,
            asleep_initially: HashSet::new(),
            log_readers: vec![],
        }
    }

    pub async fn start_node(&mut self, node_idx: usize) {
        if self.is_node_awake(node_idx).await {
            info!("Node {} is already awake", node_idx);
            panic!("Node {} is already awake", node_idx);
        }

        // Get config file for this process
        let config_file = self
            .config_files
            .get(node_idx)
            .expect("No config file for this node");

        // Start node process
        self.processes[node_idx] = Some(Self::start_node_process(config_file, node_idx, None));
        // Wait for this node to come alive
        let check_awake_res = Self::nodes_awake(vec![self.portnames[node_idx].clone()]).await;
        if check_awake_res.is_err() {
            panic!(
                "Unable to check nodes are awake: {}",
                check_awake_res.unwrap_err()
            );
        }
        info!("Node {} is awake.", node_idx);
    }

    fn start_node_process(
        config_file: &str,
        node_index: usize,
        binary_path_override: Option<String>,
    ) -> Child {
        let log_mode = Self::get_log_mode_for_node_index(node_index);
        let envs = Self::get_extra_env_vars_for_node_index(node_index);
        // if we're in CI, it's already built and in the root
        let path;
        if let Some(binary_path) = binary_path_override {
            path = binary_path;
        } else {
            let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string());
            path = match in_github_ci.as_str() {
                "1" => "./lit_node".to_string(),
                _ => "./target/debug/lit_node".to_string(),
            };
        }
        Command::new(path)
            .env(ENV_LIT_CONFIG_FILE, config_file)
            .env("RUST_LOG", log_mode)
            .env(ENV_ATTESTATION_TYPE_OVERRIDE, "ADMIN_SIGNED")
            .env(
                ENV_LOGGING_TIMESTAMP,
                std::env::var(ENV_LOGGING_TIMESTAMP).unwrap_or("0".to_string()),
            )
            .envs(envs)
            .spawn()
            .expect("Failed to launch a node")
    }

    pub async fn check_node_funds(
        provider: &Arc<Provider<Http>>,
        deployer: &Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        node_account: &Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    ) -> Result<bool> {
        let deployer_balance = provider.get_balance(deployer.address(), None).await?;
        let node_balance = provider.get_balance(node_account.address(), None).await?;

        let min_balance: U256 = 1_000_000_000_000_000u128.into(); // .01 MATIC / ETH, whatever

        if deployer_balance < min_balance * 10 {
            panic!("Deployer running low on funds!");
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
            .from(deployer.address());

        let tx_hash = deployer.send_transaction(tx, None).await?;
        let _receipt = tx_hash.await?;

        let node_balance = provider.get_balance(node_account.address(), None).await?;

        info!(
            "Node {} topped up -  balance {}",
            node_account.address(),
            node_balance
        );
        Ok(true)
    }

    pub async fn nodes_awake(portnames: Vec<String>) -> Result<Vec<PeerItem>> {
        info!(
            "TEST: NodeCollection letting Rocket intialize.  portnames: {:?}",
            portnames
        );
        let mut asleep = VecDeque::from(portnames.clone());
        let mut node_info_arr: Vec<PeerItem> = Vec::new();
        let noonce = hex::encode([0; 32]);

        while let Some(port) = asleep.pop_front() {
            if let Ok(resp) = reqwest::get(format!(
                "http://127.0.0.1:{}/{}/{}",
                port, "connect", noonce
            ))
            .await
            {
                trace!("port {} is responding {:#?}", port, resp);
                let node_info = resp.json::<PeerItem>().await?;
                node_info_arr.push(node_info);
            } else {
                //call back later
                trace!("port {} is not responding", port);
                asleep.push_front(port);
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
        info!("TEST: Nodes are responding");
        Ok(node_info_arr)
    }

    pub async fn is_node_awake(&self, node_index: usize) -> bool {
        let port = self.portnames[node_index].clone();
        let noonce = hex::encode([0; 32]);
        if let Ok(resp) =
            reqwest::get(format!("http://127.0.0.1:{}/connect/{}", port, noonce)).await
        {
            trace!("port {} is responding {:#?}", port, resp);
            true
        } else {
            false
        }
    }

    pub async fn hit_endpoints(&self, cmd: String) {
        for port in &self.portnames {
            // Previously this was blocking, which spawned a tokio runtime; when done from inside another tokio runtime, this is not allowed.
            let resp = reqwest::get(format!("http://127.0.0.1:{}/{}", port, cmd)).await;
            info!("{:#?}", resp);
        }
    }

    pub async fn hit_endpoints_join_all(&self, cmd: String) -> Vec<String> {
        let mut v = Vec::new();

        for port in &self.portnames {
            // Previously this was blocking, which spawned a tokio runtime; when done from inside another tokio runtime, this is not allowed.
            let resp = reqwest::get(format!("http://127.0.0.1:{}/{}", port, cmd));
            v.push(resp);
        }

        let results = join_all(v).await;
        let mut responses: Vec<String> = Vec::new();
        for result in results {
            if result.is_ok() {
                responses.push(result.unwrap().text().await.unwrap());
            }
        }

        responses
    }

    pub async fn hit_endpoints_with_json_body(
        &self,
        cmd: String,
        json_body: String,
    ) -> Vec<String> {
        let mut responses: Vec<String> = Vec::new();
        let request_id = &uuid::Uuid::new_v4().to_string();
        for port in &self.portnames {
            let client = reqwest::Client::new();
            let resp_string = client
                .post(format!("http://127.0.0.1:{}/{}", port, cmd))
                .header("Content-Type", "application/json")
                .header("X-Request-Id", request_id)
                .body(json_body.clone())
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            responses.push(resp_string)
        }
        responses
    }

    /// This function is used to hit endpoints with different json bodies per port.
    #[deprecated(note = "Use hit_endpoints_with_json_body_per_port instead")]
    pub async fn hit_endpoints_with_json_body_per_port(
        &self,
        cmd: String,
        json_body_vec: Vec<String>,
    ) -> Vec<String> {
        // If the number of json bodies is not equal to the number of ports, then panic.
        assert_eq!(json_body_vec.len(), self.portnames.len());

        let mut responses: Vec<String> = Vec::new();
        let request_id = &uuid::Uuid::new_v4().to_string();
        for (idx, port) in self.portnames.iter().enumerate() {
            let json_body = json_body_vec.get(idx).unwrap();
            let client = reqwest::Client::new();
            let resp_string = client
                .post(format!("http://127.0.0.1:{}/{}", port, cmd))
                .header("Content-Type", "application/json")
                .header("X-Request-Id", request_id)
                .body(json_body.clone())
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            responses.push(resp_string)
        }
        responses
    }

    #[deprecated(note = "Use Self::hit_endpoints_with_json_body_join_all instead")]
    pub async fn hit_endpoints_with_json_body_join_all(
        &self,
        cmd: String,
        json_body: String,
    ) -> Result<Vec<String>> {
        #[allow(deprecated)]
        Self::static_hit_endpoints_with_json_body_join_all(self.portnames.clone(), cmd, json_body)
            .await
    }

    #[deprecated(
        note = "Should use hit_endpoints_with_json_body_join_all instead, since this will not account for kicked validators"
    )]
    pub async fn static_hit_endpoints_with_json_body_join_all(
        portnames: Vec<String>,
        cmd: String,
        json_body: String,
    ) -> Result<Vec<String>> {
        let mut v = Vec::new();

        let request_id = &uuid::Uuid::new_v4().to_string();

        for port in portnames {
            let client = reqwest::Client::new();
            let resp = client
                .post(format!("http://127.0.0.1:{}/{}", port, cmd))
                .header("Content-Type", "application/json")
                .header("X-Request-Id", request_id)
                .body(json_body.clone())
                .send();
            v.push(resp);
        }

        let max_duration = std::time::Duration::from_secs(25);
        let results = tokio::time::timeout(max_duration, join_all(v)).await;

        let results = results.expect("Timeout waiting for nodes to respond");

        let mut responses: Vec<String> = Vec::new();
        for result in results {
            match result {
                Ok(resp) => {
                    responses.push(resp.text().await.unwrap());
                }
                Err(e) => {
                    tracing::warn!("Error hitting an endpoint: {:?}", e);
                    // error!("Error in hit_endpoints_with_json_body_join_all: {:?}", e);
                    // return Err(anyhow::anyhow!(
                    //     "Error in hit_endpoints_with_json_body_join_all: {:?}",
                    //     e
                    // ));
                }
            }
        }

        Ok(responses)
    }

    pub fn shutdown_node(&mut self, idx: usize) {
        let process = self
            .processes
            .get_mut(idx)
            .expect("No process at this index")
            .as_mut()
            .expect("No process at this index");
        let port = self.portnames.get(idx).expect("No port at this index");
        Self::kill_single(process, port.clone());
    }

    fn kill_all(&mut self) {
        for (process, port) in self.processes.iter_mut().zip(self.portnames.clone()) {
            let process = match process {
                Some(process) => process,
                None => {
                    info!("process at port {} is None, doing nothing", port);
                    continue;
                }
            };
            Self::kill_single(process, port);
        }
    }

    fn kill_single(process: &mut Child, port: String) {
        let pid = process.id();

        info!("Killing PID: {pid}, PORT: {port}");

        // just kill the nodes - don't wait for them to stop
        process.kill().expect("Could not kill node process");
        process.wait().expect("failed to wait for child PID: {pid}");
        info!("process at port {} was killed", port);
    }

    fn remove_files(staker_addresses: Vec<String>) {
        for staker in staker_addresses {
            let path = format!("./node_keys/{}", staker);
            if let Err(e) = fs::remove_dir_all(&path) {
                warn!("Failed to remove directory: {} with error {}", path, e);
            }

            let path = format!("./test_logs/{}", staker);
            if let Err(e) = fs::remove_file(&path) {
                warn!("Failed to remove file: {} with error {}", path, e);
            }
        }
    }

    pub fn size(&self) -> usize {
        self.portnames.len()
    }
}

impl Drop for NodeCollection {
    fn drop(&mut self) {
        info!("Stopping processes");
        self.kill_all();
    }
}

pub async fn hit_endpoints_with_json_body(
    scenario: &Scenario,
    cmd: String,
    json_body: String,
) -> Vec<String> {
    let portnames = get_current_validator_portnames(scenario).await;

    let mut responses: Vec<String> = Vec::new();
    let request_id = &uuid::Uuid::new_v4().to_string();
    for port in portnames {
        let client = reqwest::Client::new();
        let resp_string = client
            .post(format!("http://127.0.0.1:{}/{}", port, cmd))
            .header("Content-Type", "application/json")
            .header("X-Request-Id", request_id)
            .body(json_body.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        responses.push(resp_string)
    }
    responses
}

pub async fn hit_endpoints_with_json_body_join_all(
    scenario: &Scenario,
    cmd: String,
    json_body: String,
) -> Result<Vec<String>> {
    let portnames = get_current_validator_portnames(scenario).await;

    let mut v = Vec::new();
    let request_id = &uuid::Uuid::new_v4().to_string();
    for port in portnames {
        let client = reqwest::Client::new();
        let resp = client
            .post(format!("http://127.0.0.1:{}/{}", port, cmd))
            .header("Content-Type", "application/json")
            .header("X-Request-Id", request_id)
            .body(json_body.clone())
            .send();
        v.push(resp);
    }

    info!("Starting sign for {}", json_body.clone());
    let results = join_all(v).await;
    info!("Finished sign for {}", json_body.clone());

    let mut responses: Vec<String> = Vec::new();
    for result in results {
        match result {
            Ok(resp) => {
                responses.push(resp.text().await.unwrap());
            }
            Err(e) => {
                error!("Error hitting an endpoint: {:?}", e);
                // return Err(anyhow::anyhow!(
                //     "Error in hit_endpoints_with_json_body_join_all: {:?}",
                //     e
                // ));
            }
        }
    }

    info!("responses: {:?}", responses);

    Ok(responses)
}

pub async fn get_network_pubkey(scenario: &Scenario) -> String {
    let results = hit_endpoints_with_json_body(
        scenario,
        "/web/handshake".to_string(),
        r#"{"clientPublicKey":"blah"}"#.to_string(),
    )
    .await;

    // Parse response
    let responses = results
        .iter()
        .map(|result| serde_json::from_str(result).unwrap())
        .collect::<Vec<JsonSDKHandshakeResponse>>();

    responses[0].network_public_key.clone()
}

pub async fn handshake_returns_keys(scenario: &Scenario) -> bool {
    let results = hit_endpoints_with_json_body(
        scenario,
        "/web/handshake".to_string(),
        r#"{"clientPublicKey":"blah"}"#.to_string(),
    )
    .await;

    // Parse response
    let responses = results
        .iter()
        .map(|result| serde_json::from_str(result).expect("Unable to parse response"))
        .collect::<Vec<JsonSDKHandshakeResponse>>();

    for response in &responses {
        // Ensure no errors
        if (response.subnet_public_key.contains("ERR"))
            || (response.network_public_key.contains("ERR"))
            || (response.network_public_key_set.contains("ERR"))
        {
            info!("Handshake response contains error: {:?}", response);
            return false;
        }

        // Ensure the network public key is the correct length
        if (response.subnet_public_key.len() != 96)
            || (response.network_public_key.len() != 96)
            || (response.network_public_key_set.len() != 96)
        {
            info!(
                "Handshake response contains incorrect length public key: {:?}",
                response
            );
            return false;
        }
    }

    info!("Handshake response contains correct keys");
    true
}

pub async fn get_node_versions(scenario: &Scenario) -> Vec<String> {
    let results = hit_endpoints_with_json_body_raw(
        scenario,
        "/web/handshake".to_string(),
        r#"{"clientPublicKey":"blah"}"#.to_string(),
    )
    .await;

    // Parse response headers
    results
        .iter()
        .map(|response| {
            response
                .headers()
                .get("X-Lit-Node-Version")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
}

pub async fn hit_endpoints_with_json_body_raw(
    scenario: &Scenario,
    cmd: String,
    json_body: String,
) -> Vec<Response> {
    let portnames = get_current_validator_portnames(scenario).await;
    let mut responses: Vec<Response> = Vec::new();
    let request_id = &uuid::Uuid::new_v4().to_string();
    for port in portnames {
        let client = reqwest::Client::new();
        let resp = client
            .post(format!("http://127.0.0.1:{}/{}", port, cmd))
            .header("Content-Type", "application/json")
            .header("X-Request-Id", request_id)
            .body(json_body.clone())
            .send()
            .await
            .unwrap();
        responses.push(resp)
    }
    responses
}

/// This function is used to hit endpoints with different json bodies per port.
pub async fn hit_endpoints_with_json_body_per_port(
    scenario: &Scenario,
    cmd: String,
    json_body_vec: Vec<String>,
) -> Vec<String> {
    let portnames = get_current_validator_portnames(scenario).await;

    // If the number of json bodies is not equal to the number of ports, then panic.
    assert_eq!(json_body_vec.len(), portnames.len());

    let mut responses: Vec<String> = Vec::new();
    let request_id = &uuid::Uuid::new_v4().to_string();
    for (idx, port) in portnames.iter().enumerate() {
        let json_body = json_body_vec.get(idx).unwrap();
        let client = reqwest::Client::new();
        let resp_string = client
            .post(format!("http://127.0.0.1:{}/{}", port, cmd))
            .header("Content-Type", "application/json")
            .header("X-Request-Id", request_id)
            .body(json_body.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        responses.push(resp_string)
    }
    responses
}

pub fn choose_random_indices(array_size: usize, num_random_indices: usize) -> HashSet<usize> {
    let mut indices = HashSet::new();
    for _ in 0..num_random_indices {
        let mut idx = rand::random::<usize>() % array_size;
        while indices.contains(&idx) {
            idx = rand::random::<usize>() % array_size;
        }
        indices.insert(idx);
    }
    indices
}

async fn get_current_validator_portnames(scenario: &Scenario) -> Vec<String> {
    // Fetch the portnames from the chain state
    let validators = scenario.actions.get_current_validator_structs().await;
    validators
        .iter()
        .map(|validator| validator.port.to_string().clone())
        .collect::<Vec<String>>()
}

fn get_staker_address_from_config_file(config_file: &str) -> String {
    let config_path = Path::new(config_file);
    let config = SimpleToml::try_from(config_path).expect("Couldn't read config file");
    config
        .get("node", "staker_address")
        .expect("Couldn't retrieve the staking address")
        .to_owned()
}
