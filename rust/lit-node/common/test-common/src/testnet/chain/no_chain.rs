use crate::testnet::SimpleTomlValue;

use super::super::NodeAccount;
use super::ChainTrait;
use command_group::{CommandGroup, GroupChild};
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::signers::{LocalWallet, Signer};
use generic_array::GenericArray;
use lit_core::utils::binary::hex_to_bytes;
use lit_core::utils::toml::SimpleToml;
use lit_node::models::coms_keys::ComsKeys;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct NoChain {
    num_nodes: usize,
    pub internal_accounts: Vec<NodeAccount>,
}

impl NoChain {
    pub fn new(num_nodes: usize) -> impl ChainTrait {
        NoChain {
            num_nodes,

            internal_accounts: Vec::new(),
        }
    }
}

// impl chaintrait for no_chain

// For the no chain implementation, we'll pull the hardhat accounts to fill in some values...
#[async_trait::async_trait]
impl ChainTrait for NoChain {
    fn chain_id(&self) -> u64 {
        0
    }

    fn chain_name(&self) -> &'static str {
        "hardhat" // not really, but it allows us to have a proper resolver.
    }

    fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    fn accounts(&self) -> Arc<Vec<NodeAccount>> {
        let mut accounts: Vec<NodeAccount> = Vec::new();

        let reuse_config_path = self.reuse_config_path();

        for i in 0..self.num_nodes {
            let provider = Provider::<Http>::try_from(self.rpc_url()).unwrap();

            let (staker_address_sk, _node_address) = match reuse_config_path.clone() {
                Some(path) => {
                    let config_file =
                        &format!("./config/test/{}/lit_config{}.toml", path, 7470 + i);
                    let config_path = Path::new(config_file);
                    let config =
                        SimpleToml::try_from(config_path).expect("Couldn't read config file");
                    let private_key = config
                        .get_signing_key()
                        .expect("Couldn't retrieve the signing key");

                    let sk = SigningKey::from_bytes(GenericArray::from_slice(&private_key))
                        .expect("Couldn't parse the received key");

                    let node_address = config
                        .get_address("node", "staker_address")
                        .expect("Couldn't retrieve the staking address");

                    (sk, node_address)
                }
                None => (SigningKey::random(&mut rand::thread_rng()), Address::zero()),
            };

            let staker_address_private_key = H256::from_slice(&staker_address_sk.to_bytes());
            let wallet = LocalWallet::from(staker_address_sk).with_chain_id(self.chain_id());
            let staker_address = wallet.address();

            let signing_provider = Arc::new(SignerMiddleware::new(provider, wallet));

            let (node_address_sk, _node_address) = match reuse_config_path.clone() {
                Some(path) => {
                    let config_file =
                        &format!("./config/test/{}/lit_config{}.toml", path, 7470 + i);
                    let config_path = Path::new(config_file);
                    let config =
                        SimpleToml::try_from(config_path).expect("Couldn't read config file");
                    let private_key = hex_to_bytes(
                        config
                            .get("test", "staker_address_private_key")
                            .expect("Couldn't retrieve the signing key"),
                    )
                    .expect("Couldn't parse the private key from hex into a vec");

                    let sk = SigningKey::from_bytes(GenericArray::from_slice(&private_key))
                        .expect("Couldn't parse the received key");

                    let node_address = config
                        .get_address("node", "staker_address")
                        .expect("Couldn't retrieve the staking address");

                    (sk, node_address)
                }
                None => (SigningKey::random(&mut rand::thread_rng()), Address::zero()),
            };
            let node_address_private_key = H256::from_slice(&node_address_sk.to_bytes());
            let wallet = LocalWallet::from(node_address_sk).with_chain_id(self.chain_id());
            let node_address = wallet.address();

            let coms_keys = ComsKeys::new();

            accounts.push(NodeAccount {
                node_address,
                signing_provider,
                staker_address_private_key,
                staker_address,
                coms_keys,
                node_address_private_key,
            });
        }

        Arc::new(accounts)
    }

    fn deployer(&self) -> NodeAccount {
        self.accounts()[0].clone()
    }

    async fn start_chain(&self) -> GroupChild {
        info!("No chain to launch.");

        Command::new("/bin/bash")
            .args(["-c", "echo 'hi'"])
            .group_spawn()
            .expect("Could not spawn echo process")
    }
}
