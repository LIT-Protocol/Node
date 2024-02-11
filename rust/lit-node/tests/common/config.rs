extern crate dotenv;

#[cfg(all(feature = "proxy_http", feature = "testing"))]
use crate::common::faults::FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS;

use super::node_collection::NodeConfig;
use super::testnet::{scenario::Scenario, WhichTestnet};
use ethers::core::k256::SecretKey;
use ethers::prelude::k256::elliptic_curve::group::GroupEncoding;
use ethers::types::Address;
use generic_array::GenericArray;
use lit_api_core::config::CFG_KEY_ADDRESS;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use lit_core::utils::toml::SimpleToml;
#[cfg(all(feature = "proxy_http", feature = "testing"))]
use lit_node::config::CFG_KEY_HTTP_CLIENT_TIMEOUT;
use lit_node::config::{load_cfg, CFG_KEY_ECDSA_ROUND_TIMEOUT, CFG_KEY_ENABLE_PROXIED_HTTP_CLIENT};

use lit_node::config::{
    CFG_KEY_ADMIN_ADDRESS, CFG_KEY_CHAIN_POLLING_INTERVAL_MS, CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY,
    CFG_KEY_COMS_KEYS_SENDER_PRIVKEY, CFG_KEY_ENABLE_ACTIONS_ALLOWLIST, CFG_KEY_ENABLE_ECDSA_DKG,
    CFG_KEY_ENABLE_EPOCH_TRANSITIONS, CFG_KEY_ENABLE_RATE_LIMITING, CFG_KEY_RPC_URL,
    CFG_KEY_STAKER_ADDRESS,
};
use lit_node::utils::encoding::bytes_to_hex;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tracing::info;

use lit_api_core::config::{CFG_KEY_DOMAIN, CFG_KEY_PORT};

pub fn load_config() -> (Arc<LitConfig>, Arc<ContractResolver>) {
    // Load config
    let cfg = load_cfg().expect("failed to load LitConfig");
    let loaded_config = cfg.load_full();

    let resolver = Arc::new(
        ContractResolver::try_from(cfg.load().as_ref()).expect("failed to load ContractResolver"),
    );

    (loaded_config, resolver)
}

trait SimpleTomlInsertHelper {
    fn insertstr(&mut self, section: &str, key: &str, value: &str);
    fn insert_address(&mut self, section: &str, key: &str, value: Address) {
        self.insertstr(
            section,
            key,
            &format!("0x{}", data_encoding::HEXLOWER.encode(value.as_bytes())),
        )
    }
    fn insert_bytes(&mut self, section: &str, key: &str, value: &[u8]) {
        self.insertstr(
            section,
            key,
            &format!("0x{}", data_encoding::HEXLOWER.encode(value)),
        )
    }
}

impl SimpleTomlInsertHelper for SimpleToml {
    fn insertstr(&mut self, section: &str, key: &str, value: &str) {
        self.insert(section.to_string(), key.to_string(), value.to_string());
    }
}

pub fn write_new_config_from_scenario(
    scenario: &Scenario,
    file_name: String,
    rocket_port: usize,
    node_config: NodeConfig,
) -> bool {
    // place holder for contracts not yet deployed!  see FIXME below.
    let admin_address = scenario.testnet.deploy_address;
    //FIXME
    let i = rocket_port - 7470;

    let node_account = &scenario.testnet.node_accounts[i];

    // just hardcode these for now

    let node_address_secret_key = SecretKey::from_bytes(GenericArray::from_slice(
        node_account.node_address_private_key.as_bytes(),
    ))
    .unwrap();
    let staker_address_secret_key = SecretKey::from_bytes(GenericArray::from_slice(
        node_account.staker_address_private_key.as_bytes(),
    ))
    .unwrap();

    let public_key = node_address_secret_key.public_key();
    let advance_epoch = scenario.testnet.which != WhichTestnet::NoChain;
    let mut cfg = SimpleToml::new();

    // section lit
    let section = "lit";
    cfg.insertstr(section, "env", "dev");

    // section blockchain
    let section = "blockchain";
    cfg.insertstr(section, "chain_id", &scenario.testnet.chain_id.to_string());
    cfg.insertstr(section, "chain_name", &scenario.testnet.chain_name);

    // section blockchain.wallet.default
    let section = "blockchain.wallet.default";
    cfg.insert_bytes(
        section,
        "private_key",
        node_address_secret_key.to_bytes().as_ref(),
    );
    cfg.insert_bytes(
        section,
        "public key: ",
        public_key.to_projective().to_bytes().as_ref(),
    );
    cfg.insert_bytes(
        section,
        "address: ",
        public_key.to_projective().to_bytes().as_ref(),
    );

    // section test
    let section = "test";
    cfg.insert_bytes(
        section,
        "staker_address_private_key",
        staker_address_secret_key.to_bytes().as_ref(),
    );

    // section subnet
    let subnet_id = &bytes_to_hex(scenario.contracts.staking.address()).replace("0x", "");
    let section = "subnet";
    cfg.insertstr(section, "id", subnet_id);

    // section ipfs
    let section = "ipfs";
    cfg.insertstr(section, "gateway", "https://cloudflare-ipfs.com/ipfs/");

    // section node.http
    let section = "node.http";
    cfg.insertstr(section, CFG_KEY_PORT, &node_config.rocket_port.to_string());

    // section node
    let section = "node";
    cfg.insertstr(section, CFG_KEY_DOMAIN, &node_config.lit_domain_name);
    cfg.insertstr(section, CFG_KEY_RPC_URL, &scenario.testnet.rpcurl);
    cfg.insertstr(section, CFG_KEY_ENABLE_ECDSA_DKG, "true");
    cfg.insertstr(section, CFG_KEY_ENABLE_RATE_LIMITING, "false");
    cfg.insertstr(section, CFG_KEY_ENABLE_ACTIONS_ALLOWLIST, "false");
    cfg.insertstr(
        section,
        CFG_KEY_ENABLE_PROXIED_HTTP_CLIENT,
        &node_config
            .enable_proxied_http_client
            .map(|e| e.to_string())
            .unwrap_or("false".to_string()),
    );
    cfg.insertstr(
        section,
        CFG_KEY_ENABLE_EPOCH_TRANSITIONS,
        &advance_epoch.to_string(),
    );
    cfg.insertstr(section, CFG_KEY_ADDRESS, &node_config.lit_domain_name);
    cfg.insertstr(section, CFG_KEY_STAKER_ADDRESS, &node_config.staker_address);
    cfg.insert_address(section, CFG_KEY_ADMIN_ADDRESS, admin_address);
    cfg.insertstr(
        section,
        CFG_KEY_COMS_KEYS_SENDER_PRIVKEY,
        &data_encoding::HEXLOWER.encode(
            &node_account
                .coms_keys
                .my_sender_private_key()
                .as_ref()
                .to_bytes(),
        ),
    );
    cfg.insertstr(
        section,
        CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY,
        &data_encoding::HEXLOWER.encode(
            &node_account
                .coms_keys
                .my_receiver_private_key()
                .as_ref()
                .to_bytes(),
        ),
    );

    #[cfg(all(feature = "proxy_http", feature = "testing"))]
    {
        if scenario.is_fault_test {
            cfg.insertstr(
                section,
                CFG_KEY_HTTP_CLIENT_TIMEOUT,
                &FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS.to_string(),
            );
            cfg.insertstr(
                section,
                CFG_KEY_ECDSA_ROUND_TIMEOUT,
                &(FAULT_TEST_HTTP_CLIENT_TIMEOUT_SECS * 1000).to_string(),
            );
        }
    }

    #[cfg(not(all(feature = "proxy_http", feature = "testing")))]
    {
        cfg.insertstr(section, CFG_KEY_ECDSA_ROUND_TIMEOUT, "15000");
    }

    cfg.insertstr(section, CFG_KEY_CHAIN_POLLING_INTERVAL_MS, "1000");

    if Path::new(&file_name).exists() {
        fs::remove_file(&file_name).expect("Unable to delete testnet configuration file!");
    }

    cfg.write_file(Path::new(&file_name))
        .expect("Unable to save testnet configuration file!");

    info!("wrote config file: {}", file_name);
    true
}
