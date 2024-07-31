// To use this, you need to install Foundry using this command: curl -L https://foundry.paradigm.xyz | bash
use super::ChainTrait;
use crate::common::testnet::contracts_repo::compile_contracts;
use crate::common::testnet::NodeAccount;
use command_group::{CommandGroup, GroupChild}; // node/anvil launches many processes to manage the testnet, so we need to use a group interface to manage them, as killing only the process we know about will leave zombies.
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::k256::SecretKey;
use ethers::prelude::*;
use generic_array::GenericArray;
use lit_core::utils::binary::hex_to_bytes;
use lit_node::models::coms_keys::ComsKeys;

use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{TcpStream, ToSocketAddrs};
use tracing::info;

pub struct Anvil {
    num_nodes: usize,
    // num_staked: usize,
}

impl Anvil {
    // pub fn new(num_nodes: usize, num_staked: usize) -> impl ChainTrait {
    pub fn new(num_nodes: usize) -> impl ChainTrait {
        Anvil {
            num_nodes,
            // num_staked,
        }
    }
}

use async_trait::async_trait;
use lit_blockchain::resolver::rpc::RpcHealthcheckPoller;
// impl chain for Anvil
#[async_trait]
impl ChainTrait for Anvil {
    fn chain_id(&self) -> u64 {
        31337
    }

    fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    fn rpc_url(&self) -> String {
        "127.0.0.1:8545".to_string()
    }

    fn chain_name(&self) -> &'static str {
        "anvil"
    }

    async fn start_chain(&self) -> GroupChild {
        compile_contracts();

        // when running in CI, anvil is already running in a docker container, so no need to start it.
        // we run echo 'hi' as a dummy process instead.
        let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string());
        if in_github_ci == "1" {
            info!("Not starting chain in CI");
            if !is_anvil_running(&self.rpc_url()).await {
                panic!("anvil is not running in CI.  It should have been loaded by the docker container.");
            }
            // restart docker to reset chain since anvil_reset isn't working for non-forked chains right now https://github.com/foundry-rs/foundry/issues/6018
            let docker_ps = Command::new("docker")
                .args(["ps"])
                .output()
                .expect("failed to get docker ps");
            let output = String::from_utf8_lossy(&docker_ps.stdout);
            info!("Docker ps output: {}", output);
            let lines: Vec<&str> = output.split('\n').collect();
            let mut container_id = String::new();
            for line in lines {
                if line.contains("litptcl/anvil-lit:latest") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    container_id = parts[0].to_string();
                    break;
                }
            }
            if container_id.is_empty() {
                panic!("Failed to find container id for litptcl/anvil-lit:latest");
            }

            let restart_result = Command::new("docker")
                .args(["restart", &container_id])
                .output()
                .expect("failed to restart anvil docker container");
            let output = String::from_utf8_lossy(&restart_result.stdout);
            info!("Docker restart output: {}", output);

            // give docker a few secs to come up
            tokio::time::sleep(Duration::from_secs(5)).await;

            return Command::new("/bin/bash")
                .args(["-c", "echo 'hi'"])
                .group_spawn()
                .expect("Could not spawn echo process");
        }

        if is_anvil_running(&self.rpc_url()).await {
            info!("anvil is already running.  Attempting to kill");
            Command::new("pkill")
                .arg("anvil")
                .output()
                .expect("failed to kill anvil");

            tokio::time::sleep(Duration::from_millis(500)).await;
            if is_anvil_running(&self.rpc_url()).await {
                panic!("anvil running and couldn't be killed");
            }
        }

        // We use group_spawn because node launches several subprocesses,
        // and we need to kill them using group api to stop the testnet
        let command_path;
        if std::env::var("IN_GITHUB_CI").is_ok() {
            let home_dir = std::env::var("HOME").expect("Could not get home dir");
            command_path = format!("{}/.cargo/bin/anvil", home_dir);
            let path = std::path::PathBuf::from(command_path.clone());
            if !path.is_file() {
                panic!("can't find anvil. Aborting test.");
            }
        } else {
            let home_dir = std::env::var("HOME").expect("Could not get home dir");
            command_path = format!("{}/.foundry/bin/anvil", home_dir);
        }
        info!("found path for anvil: {}", &command_path);

        let rv = Command::new(command_path)
            // .env("RUST_LOG", "trace") // if you need to debug anvil you can uncomment this.
            // .env("RUST_LOG", "info") // if you just need to see console.log from the contract uncomment this instead
            .env("ETHERNAL_API_TOKEN", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJmaXJlYmFzZVVzZXJJZCI6IlQ5Sk1xZjgwMUVoUk9XSTNaTVRTM2dQRTRrdjIiLCJhcGlLZXkiOiJBRFlSRUVOLVhSRE1DVEgtSjNXTUdIWC1IQ1haSE0yXHUwMDAxIiwiaWF0IjoxNjkxMDk0NDczfQ.Rpc_oExqnwCl-iRKLQbQCN7P7nUIuucJtoiE46xVn3g") // localhost
            .stderr(Stdio::null()) // comment this out to see what's going on with anvil
            .stdout(Stdio::null()) // comment this out to see what's going on with anvil
            .group_spawn()
            .expect("Failed to launch Anvil testnet.  Are you sure Foundry is installed?");
        if !has_anvil_started(&self.rpc_url(), Duration::new(10, 0)).await {
            panic!("anvil has not come up.  Aborting test.  You may comment out the stdout/stderr lines above to see what's going on.");
        }
        info!("Anvil has started");
        rv
    }

    // for hardhat and no_chain, this trait function should be overriden.
    fn deployer(&self) -> NodeAccount {
        let secret = first_anvil_account_private_key();

        let sk =
            SigningKey::from(SecretKey::from_bytes(GenericArray::from_slice(&secret)).unwrap());
        let private_key = H256::from_slice(&sk.to_bytes());

        let wallet = LocalWallet::from(sk).with_chain_id(self.chain_id());
        let address = wallet.address();
        let provider = lit_blockchain::resolver::rpc::ENDPOINT_MANAGER
            .get_provider(self.chain_name())
            .unwrap();

        let signing_provider = Arc::new(SignerMiddleware::new(provider, wallet));

        let coms_keys = ComsKeys::new();

        let staker_address = address;

        NodeAccount {
            node_address: Address::zero(),
            signing_provider,
            node_address_private_key: H256::zero(),
            staker_address_private_key: private_key,
            staker_address,
            coms_keys,
        }
    }
}

async fn is_anvil_running<A: ToSocketAddrs + ?Sized>(host: &A) -> bool {
    match TcpStream::connect(host).await {
        Ok(..) => true,
        Err(..) => false,
    }
}

async fn has_anvil_started<A: ToSocketAddrs + ?Sized>(host: &A, waitfor: Duration) -> bool {
    async fn waitfor_anvil_to_start<A: ToSocketAddrs + ?Sized>(host: &A) {
        loop {
            if is_anvil_running(host).await {
                return;
            }
            info!("Waiting for anvil to come up...");
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    match tokio::time::timeout(waitfor, waitfor_anvil_to_start(host)).await {
        Err(..) => false,
        Ok(..) => true,
    }
}

pub fn first_anvil_account_private_key() -> Vec<u8> {
    hex_to_bytes("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80").unwrap()
}
