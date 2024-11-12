pub mod anvil;
pub mod hardhat;
pub mod no_chain;

use super::NodeAccount;
use crate::testnet::contracts_repo::latest_wallet_manifest;
use command_group::GroupChild;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::k256::SecretKey;
use ethers::prelude::*;
use generic_array::GenericArray;
use lit_blockchain::resolver::rpc::{RpcHealthcheckPoller, ENDPOINT_MANAGER};
use lit_core::utils::toml::SimpleToml;
use lit_node::models::coms_keys::ComsKeys;
use lit_node::utils::encoding::hex_to_bytes;

use std::collections::HashMap;

use std::sync::Arc;
use std::time::Duration;

#[async_trait::async_trait]
pub trait ChainTrait {
    fn chain_name(&self) -> &'static str;
    fn chain_id(&self) -> u64;
    // fn accounts(&self) -> Arc<Vec<NodeAccount>>;
    fn num_nodes(&self) -> usize;

    async fn start_chain(&self) -> GroupChild;

    fn rpc_url(&self) -> String {
        let chain_id = self.chain_name();
        lit_blockchain::resolver::rpc::ENDPOINT_MANAGER
            .rpc_url(chain_id)
            .expect("invalid chain id")
    }

    fn reuse_config_path(&self) -> Option<String> {
        let data = get_reuseable_config_data(self.chain_name());
        data.as_ref()?;
        let data = data.unwrap();

        let mut config_path: Option<String> = None;
        let _reuse_config = match data.get("reuse") {
            Some(hm) => {
                if hm.contains_key("config_path") {
                    let path = hm.get("config_path").unwrap().to_string();
                    config_path = Some(path);
                }
                config_path.is_some() // if there's a value, we have a config.
            }
            None => false,
        };

        config_path
    }

    fn accounts(&self) -> Arc<Vec<NodeAccount>> {
        let latest_wallet_manifest = latest_wallet_manifest(false);
        if latest_wallet_manifest.len() < self.num_nodes() {
            panic!("Number of nodes in latest_wallet_manifest.json is less than the number of nodes in the testnet");
        }

        Arc::new(
            latest_wallet_manifest
                .iter()
                .take(self.num_nodes())
                .map(|wallet_manifest_item| {
                    let mut provider = ENDPOINT_MANAGER
                        .get_provider(self.chain_name())
                        .expect("Error getting provider");
                    provider.set_interval(Duration::new(0, 10));

                    // Check against node config
                    wallet_manifest_item.assert_against_node_config();

                    wallet_manifest_item.map_to_node_account(provider, self.chain_id())
                })
                .collect(),
        )
    }

    // for hardhat and no_chain, this trait function should be overriden.
    fn deployer(&self) -> NodeAccount {
        let data = get_reuseable_config_data(self.chain_name());

        let data = data.unwrap();

        let secret = data
            .get("deployer")
            .expect("No deployer section in toml")
            .get("secret")
            .expect("No deployer.secret in toml")
            .to_string();

        let secret =
            hex_to_bytes(secret).expect("Failed to parse deployer.secret from mumbai.toml");

        let sk =
            SigningKey::from(SecretKey::from_bytes(GenericArray::from_slice(&secret)).unwrap());
        let private_key = H256::from_slice(&sk.to_bytes());

        let wallet = LocalWallet::from(sk).with_chain_id(self.chain_id());
        let provider = ENDPOINT_MANAGER.get_provider(self.chain_name()).unwrap();

        let signing_provider = Arc::new(SignerMiddleware::new(provider, wallet));

        let staker_address = data
            .get("node")
            .expect("No node section toml")
            .get("staker_address")
            .expect("No staker_address toml");

        let bytes = hex_to_bytes(staker_address).expect("Failed to parse staker_address");
        let staker_address = H160::from_slice(&bytes);

        // hold on, we just loaded all the config stuff, why are we using random coms keys?
        // this is because later on, the nodes pull this from chain, where it's already set.
        // the nodes have the data set on chain but just aren't staked yet.  so they pull the coms keys
        // and use them to stake
        let coms_keys = ComsKeys::new();

        NodeAccount {
            node_address: Address::zero(),
            node_address_private_key: H256::zero(),
            signing_provider,
            staker_address_private_key: private_key,
            staker_address,
            coms_keys,
        }
    }
}

// we require a {chain_name}.toml file to be present in the ./config directory with a structure that looks like this:
// [deployer]
// secret = "0x4cda7fa976be61950cb47a112eb3d47accdf8fe5315480ddd5a53cb1e86f8cf4"
// [reuse]
// config_path = "20394890324"  // some guid, or omit if using  a full redeploy.

fn get_reuseable_config_data(chain_name: &str) -> Option<HashMap<String, HashMap<String, String>>> {
    let file_name = format!("./config/test/test_{}.toml", chain_name);
    let path = std::path::Path::new(&file_name);
    if !path.exists() {
        return None;
    }

    let config =
        SimpleToml::try_from(path).unwrap_or_else(|_| panic!("Failed to load {:?}.", &path));
    let data = config.data();
    Some(data.clone())
}
