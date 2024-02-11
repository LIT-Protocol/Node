use crate::common::testnet::contracts_repo::{compile_contracts, start_hardhat_chain};
use crate::common::testnet::NodeAccount;

use super::ChainTrait;
use command_group::GroupChild; // node/hardhat launches many processes to manage the testnet, so we need to use a group interface to manage them, as killing only the process we know about will leave zombies.
use ethers::prelude::*;
use hex_literal::hex as hexl;

use std::process::Command;
use std::time::Duration;
use tokio::net::{TcpStream, ToSocketAddrs};
use tracing::info;

pub struct Hardhat {
    num_nodes: usize,
}

impl Hardhat {
    pub fn new(num_nodes: usize) -> impl ChainTrait {
        Hardhat { num_nodes }
    }
}

use async_trait::async_trait;
// impl chain for HardHat
#[async_trait]
impl ChainTrait for Hardhat {
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
        "hardhat"
    }

    fn deployer(&self) -> NodeAccount {
        self.accounts()[0].clone()
    }

    async fn start_chain(&self) -> GroupChild {
        compile_contracts();

        if is_hardhat_running(&self.rpc_url()).await {
            info!("Hardhat is already running.  Attempting to kill");
            Command::new("pkill")
                .arg("node")
                .output()
                .expect("failed to kill hardhat");

            tokio::time::sleep(Duration::from_millis(500)).await;
            if is_hardhat_running(&self.rpc_url()).await {
                panic!("Hardhat running and couldn't be killed");
            }
        }

        // We use group_spawn because node launches several subprocesses,
        // and we need to kill them using group api to stop the testnet
        let rv = start_hardhat_chain();
        if !has_hardhat_started(&self.rpc_url(), Duration::new(10, 0)).await {
            panic!("Hardhat has not come up.  Aborting test.  You may comment out the stdout/stderr lines above to see what's going on.");
        }
        rv
    }
}

async fn is_hardhat_running<A: ToSocketAddrs + ?Sized>(host: &A) -> bool {
    match TcpStream::connect(host).await {
        Ok(..) => true,
        Err(..) => false,
    }
}

async fn has_hardhat_started<A: ToSocketAddrs + ?Sized>(host: &A, waitfor: Duration) -> bool {
    async fn waitfor_hardhat_to_start<A: ToSocketAddrs + ?Sized>(host: &A) {
        loop {
            if is_hardhat_running(host).await {
                return;
            }
            info!("Waiting for Hardhat to come up...");
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    match tokio::time::timeout(waitfor, waitfor_hardhat_to_start(host)).await {
        Err(..) => false,
        Ok(..) => true,
    }
}

pub fn first_anvil_account_private_key() -> H256 {
    hexl!("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80").into()
}

pub fn hardhat_account_private_keys(nodenum: usize) -> Vec<H256> {
    let mut accts: Vec<H256> = Vec::new();
    accts.push(hexl!("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80").into());
    accts.push(hexl!("59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d").into());
    accts.push(hexl!("5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a").into());
    accts.push(hexl!("7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6").into());
    accts.push(hexl!("47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a").into());
    accts.push(hexl!("8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba").into());
    accts.push(hexl!("92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e").into());
    accts.push(hexl!("4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356").into());
    accts.push(hexl!("dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97").into());
    accts.push(hexl!("2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6").into());
    accts.push(hexl!("f214f2b2cd398c806f84e317254e0f0b801d0643303237d97a22a48e01628897").into());
    accts.push(hexl!("701b615bbdfb9de65240bc28bd21bbc0d996645a3dd57e7b12bc2bdf6f192c82").into());
    accts.push(hexl!("a267530f49f8280200edf313ee7af6b827f2a8bce2897751d06a843f644967b1").into());
    accts.push(hexl!("47c99abed3324a2707c28affff1267e45918ec8c3f20b8aa892e8b065d2942dd").into());
    accts.push(hexl!("c526ee95bf44d8fc405a158bb884d9d1238d99f0612e9f33d006bb0789009aaa").into());
    accts.push(hexl!("8166f546bab6da521a8369cab06c5d2b9e46670292d85c875ee9ec20e84ffb61").into());
    accts.push(hexl!("ea6c44ac03bff858b476bba40716402b03e41b8e97e276d1baec7c37d42484a0").into());
    accts.push(hexl!("689af8efa8c651a91ad287602527f3af2fe9f6501a7ac4b061667b5a93e037fd").into());
    accts.push(hexl!("de9be858da4a475276426320d5e9262ecfc3ba460bfac56360bfa6c4c28b4ee0").into());
    accts.push(hexl!("df57089febbacf7ba0bc227dafbffa9fc08a93fdc68e1e42411a14efcf23656e").into());

    if nodenum > 20 {
        info!(
            "{:} nodes were requested, but only 20 addresses exist!",
            nodenum
        )
    };

    let mut selected: Vec<H256> = Vec::new();
    for addr in accts.as_slice().iter().take(nodenum) {
        selected.push(*addr);
    }
    selected
}
