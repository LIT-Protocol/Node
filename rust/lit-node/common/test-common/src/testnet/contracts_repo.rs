use std::borrow::BorrowMut;
use std::env;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;
use std::{fs, process::Stdio};

use crate::testnet::contracts::ContractAddresses;

use super::{NodeAccount, SimpleTomlValue};
use anyhow::Result;
use command_group::{CommandGroup, GroupChild};
use ethers::prelude::*;
use generic_array::GenericArray;
use k256::ecdsa::SigningKey;
use lit_core::utils::binary::hex_to_bytes;
use lit_core::utils::toml::SimpleToml;
use lit_node::models::coms_keys::ComsKeys;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_encrypt_core::key::key_pair::private_key::{ReceiverPrivateKey, SenderPrivateKey};
use serde_encrypt_core::key::key_pair::public_key::{ReceiverPublicKey, SenderPublicKey};
use serde_json::Value;
use std::process::Command;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tracing::{error, info};

#[cfg(feature = "shiva-path")]
const LITCONTRACTPATH: &str = "../../../../blockchain/contracts";
#[cfg(not(feature = "shiva-path"))]
const LITCONTRACTPATH: &str = "../../blockchain/contracts";

// Required environment variables for the deployment scripts
const ENV_IPFS_API_KEY: &str = "IPFS_API_KEY";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAliasManifest {
    pub deployed_node_contracts_path: String,
    pub existing_staker_wallet_private_key: String,
    pub node_config_admin_address: String,
    pub node_config_ipfs_api_key: String,
    pub alias_ip: String,
    pub alias_port: usize,
    pub node_custom_runtime_config_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletManifestItem {
    pub idx: usize,
    pub node: WalletManifestNodeWallet,
    pub staker: WalletManifestStakerWallet,
}

impl WalletManifestItem {
    pub fn map_to_node_account(&self, provider: Provider<Http>, chain_id: u64) -> NodeAccount {
        let staker_private_key = hex_to_bytes(self.staker.private_key.as_str())
            .expect("Couldn't parse the private key from hex into a vec");
        let staker_sk = SigningKey::from_bytes(GenericArray::from_slice(&staker_private_key))
            .expect("Couldn't parse the received key");
        let staker_wallet = LocalWallet::from(staker_sk.clone()).with_chain_id(chain_id);
        let staker_address_private_key = H256::from_slice(&staker_sk.to_bytes());

        let signing_provider = Arc::new(SignerMiddleware::new(provider, staker_wallet.clone()));

        let coms_keys_sender_priv_key =
            ComsKeys::parse_secret_key(&self.node.coms_keys_sender.private_key)
                .expect("Couldn't parse the coms keys sender private key");
        let coms_keys_sender_public_key = coms_keys_sender_priv_key.public_key();
        let coms_keys_receiver_priv_key =
            ComsKeys::parse_secret_key(&self.node.coms_keys_receiver.private_key)
                .expect("Couldn't parse the coms keys receiver private key");
        let coms_keys_receiver_public_key = coms_keys_receiver_priv_key.public_key();
        let coms_keys = ComsKeys::new_from_keypairs(
            SenderPrivateKey::from(coms_keys_sender_priv_key),
            SenderPublicKey::from(coms_keys_sender_public_key),
            ReceiverPrivateKey::from(coms_keys_receiver_priv_key),
            ReceiverPublicKey::from(coms_keys_receiver_public_key),
        );

        let node_address = H160::from_slice(
            &hex_to_bytes(self.node.address.as_str())
                .expect("Could not convert node_address hex to bytes"),
        );

        let node_private_key = hex_to_bytes(self.node.private_key.as_str())
            .expect("Couldn't parse the private key from hex into a vec");
        let node_sk = SigningKey::from_bytes(GenericArray::from_slice(&node_private_key))
            .expect("Couldn't parse the received key");
        let node_address_private_key = H256::from_slice(&node_sk.to_bytes());

        NodeAccount {
            signing_provider,
            staker_address_private_key,
            staker_address: staker_wallet.address(),
            coms_keys,
            node_address,
            node_address_private_key,
        }
    }

    /// Asserts that the wallet manifest item is the same as the parameters in the corresponding
    /// node config file.
    pub fn assert_against_node_config(&self) {
        let config_file = &format!(
            "{}/node_configs/lit_config{}.toml",
            contracts_dir().display(),
            self.idx
        );
        let config_path = Path::new(config_file);
        let config = SimpleToml::try_from(config_path).expect("Couldn't read config file");

        // Assert staker address
        let staker_private_key = hex_to_bytes(self.staker.private_key.as_str())
            .expect("Couldn't parse the private key from hex into a vec");
        let staker_sk = SigningKey::from_bytes(GenericArray::from_slice(&staker_private_key))
            .expect("Couldn't parse the received key");
        let staker_wallet = LocalWallet::from(staker_sk);
        let staker_address = config
            .get_address("node", "staker_address")
            .expect("Couldn't retrieve the staking address");
        assert!(
            staker_address == staker_wallet.address(),
            "Staker address read from lit_configX.toml does not match one read from wallets.json"
        );

        // Assert node private key
        let node_private_key = hex_to_bytes(self.node.private_key.as_str())
            .expect("Couldn't parse the private key from hex into a vec");
        let node_sk_bytes = SigningKey::from_bytes(GenericArray::from_slice(&node_private_key))
            .expect("Couldn't parse the received key")
            .to_bytes();
        let config_node_private_key = SigningKey::from_bytes(GenericArray::from_slice(
            &config
                .get_signing_key()
                .expect("Couldn't retrieve the node wallet key"),
        ))
        .expect("Could not convert node wallet key to Signing Key")
        .to_bytes();
        assert_eq!(config_node_private_key, node_sk_bytes);
        let _node_address_private_key = H256::from_slice(
            &SigningKey::from_bytes(GenericArray::from_slice(
                &config
                    .get_signing_key()
                    .expect("Couldn't retrieve the node wallet key"),
            ))
            .expect("Could not convert node wallet key to Signing Key")
            .to_bytes(),
        );
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletManifestNodeWallet {
    pub address: String,
    pub private_key: String,
    pub public_key: String,
    pub coms_keys_sender: WalletManifestComsKeysItem,
    pub coms_keys_receiver: WalletManifestComsKeysItem,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletManifestComsKeysItem {
    pub public_key: String,
    pub private_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletManifestStakerWallet {
    pub address: String,
    pub private_key: String,
    pub public_key: String,
}

pub fn node_configs_path() -> String {
    format!("{}/node_configs", contracts_dir().display())
}

pub fn alias_node_configs_path() -> String {
    format!("{}/alias_node_configs", contracts_dir().display())
}

pub fn request_to_leave(staker_wallet_private_key: &str, staking_contract_address: &str) {
    // Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/requestToLeave.ts --staker-wallet-private-key <PRIVATE_KEY> --staking-address <STAKING_CONTRACT_ADDRESS>
    let script_path = contracts_dir().join("scripts").join("requestToLeave.ts");
    let args = [
        "ts-node",
        "--files",
        script_path.to_str().unwrap(),
        "--staker-wallet-private-key",
        staker_wallet_private_key,
        "--staking-address",
        staking_contract_address,
    ];
    info!(
        "Running full command in {}: HARDHAT_NETWORK=localchain npx {}",
        contracts_dir().display(),
        args.join(" ")
    );
    let mut rv = Command::new("npx")
        .args(args)
        .env("HARDHAT_NETWORK", "localchain")
        .current_dir(contracts_dir())
        // .stderr(Stdio::null()) // comment this out to see what's going on
        // .stdout(Stdio::null()) // comment this out to see what's going on
        .group_spawn()
        .expect("Failed to launch request to leave script");
    let exit_code = rv
        .wait()
        .expect("Failed to wait on request to leave script");
    if !exit_code.success() {
        panic!(
            "Request to leave script failed with exit code {:?}",
            exit_code
        );
    }
}

pub fn request_to_join<T>(
    staker_wallet_private_key: T,
    staking_contract_address: T,
    staking_balances_contract_address: T,
    validator_ip: T,
    validator_port: T,
    validator_node_address: T,
    validator_comms_sender_pubkey: T,
    validator_comms_receiver_pubkey: T,
) where
    T: AsRef<str>,
{
    // Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/requestToJoin.ts --staker-wallet-private-key <PRIVATE_KEY> --staking-address <STAKING_CONTRACT_ADDRESS> --staking-balances-address <STAKING_BALANCES_CONTRACT_ADDRESS> --validator-ip <VALIDATOR_IP> --validator-port <VALIDATOR_PORT> --validator-node-address <VALIDATOR_NODE_ADDRESS> --validator-comms-sender-pubkey <VALIDATOR_COMMS_SENDER_PUBKEY> --validator-comms-receiver-pubkey <VALIDATOR_COMMS_RECEIVER_PUBKEY>
    let args = [
        "ts-node",
        "--files",
        "scripts/requestToJoin.ts",
        "--staker-wallet-private-key",
        staker_wallet_private_key.as_ref(),
        "--staking-address",
        staking_contract_address.as_ref(),
        "--staking-balances-address",
        staking_balances_contract_address.as_ref(),
        "--validator-ip",
        validator_ip.as_ref(),
        "--validator-port",
        validator_port.as_ref(),
        "--validator-node-address",
        validator_node_address.as_ref(),
        "--validator-comms-sender-pubkey",
        validator_comms_sender_pubkey.as_ref(),
        "--validator-comms-receiver-pubkey",
        validator_comms_receiver_pubkey.as_ref(),
    ];
    info!(
        "Running full command in {}: HARDHAT_NETWORK=localchain npx {}",
        contracts_dir().display(),
        args.join(" ")
    );

    let mut rv = Command::new("npx")
        .args(args)
        .env("HARDHAT_NETWORK", "localchain")
        .current_dir(contracts_dir())
        // .stderr(Stdio::null()) // comment this out to see what's going on
        // .stdout(Stdio::null()) // comment this out to see what's going on
        .group_spawn()
        .expect("Failed to launch request to join script");
    let exit_code = rv.wait().expect("Failed to wait on request to join script");
    if !exit_code.success() {
        panic!(
            "Request to join script failed with exit code {:?}",
            exit_code
        );
    }
}

/// A wallet manifest is a JSON file that gets generated when the contract deployment tooling has
/// successfully made a deployment, that contains an array of wallets that were used during the deployment
/// and setup.
pub fn latest_wallet_manifest(is_alias_wallet_manifest: bool) -> Vec<WalletManifestItem> {
    // Fetch the latest manifest of the deployed wallets.
    let path = contracts_dir().join("wallets");

    // Wallet manifests are named similar to this example: `wallets-1698822800413-localchain-3.json`
    let re = if is_alias_wallet_manifest {
        Regex::new(r"alias-wallets-(\d+)\.json").unwrap()
    } else {
        Regex::new(r"wallets-(\d+)-(.*)\.json").unwrap()
    };

    // First use regex to filter for matched files, then sort by descending order of the 1st
    // capture group (the timestamp), and then take the first one.
    let manifests: Vec<String> = fs::read_dir(path.clone())
        .expect("Failed to read directory")
        .filter(|entry| {
            let entry = entry.as_ref().unwrap();
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            re.is_match(filename)
        })
        .map(|entry| {
            let entry = entry.expect("Failed to get entry");
            let path = entry.path();
            path.file_name().unwrap().to_str().unwrap().to_string()
        })
        .collect();

    // Sort by descending order and take the first one.
    let latest_manifest = manifests
        .iter()
        .max_by_key(|filename| {
            let captures = re.captures(filename).unwrap();
            captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .expect("Failed to parse timestamp")
        })
        .unwrap()
        .to_string();
    info!("Fetched latest wallet manifest: {:?}", latest_manifest);

    // Parse the wallet manifest and select a random wallet that we will add an alias for.
    let manifest_path = path.join(latest_manifest);
    let manifest = fs::read_to_string(manifest_path).expect("Failed to read manifest");
    serde_json::from_str::<Vec<WalletManifestItem>>(&manifest).expect("Failed to parse JSON")
}

/// An alias manifest is a JSON file that declares the details of how to (generate and) add a new wallet
/// as an alias of an existing wallet / node.
pub fn get_alias_manifest_template() -> AddAliasManifest {
    let alias_manifest_template_path = contracts_dir()
        .join("scripts")
        .join("generate_wallet_and_add_as_alias_manifest.template.json");
    let alias_manifest_template =
        fs::read_to_string(alias_manifest_template_path).expect("Failed to read template");
    serde_json::from_str::<AddAliasManifest>(&alias_manifest_template)
        .expect("Failed to parse JSON")
}

pub fn save_alias_manifest(alias_manifest: &AddAliasManifest) {
    let alias_manifest_path = contracts_dir()
        .join("scripts")
        .join("generate_wallet_and_add_as_alias_manifest.json");
    let alias_manifest =
        serde_json::to_string_pretty(&alias_manifest).expect("Failed to serialize JSON");
    info!("Generating alias manifest: {:?}", alias_manifest);
    fs::write(alias_manifest_path, alias_manifest).expect("Failed to write alias manifest to file");
}

pub fn generate_wallet_and_add_as_alias() {
    let args = [
        "hardhat",
        "run",
        "--network",
        "localchain",
        "scripts/generate_wallet_and_add_as_alias.ts",
    ];
    info!(
        "Running full command in {}: npx {}",
        contracts_dir().display(),
        args.join(" ")
    );
    let mut rv = populate_required_environment_variables(Command::new("npx").borrow_mut())
        .args(args)
        .current_dir(contracts_dir())
        // .stderr(Stdio::null()) // comment this out to see what's going on
        // .stdout(Stdio::null()) // comment this out to see what's going on
        .group_spawn()
        .expect("Failed to launch generate wallet and add as alias script");
    let exit_code = rv
        .wait()
        .expect("Failed to wait on generate wallet and add as alias script");
    if !exit_code.success() {
        panic!(
            "Generate wallet and add as alias script failed with exit code {:?}",
            exit_code
        );
    }
}

pub fn start_hardhat_chain() -> GroupChild {
    Command::new("npx")
        .current_dir(contracts_dir())
        // .env("ETHERNAL_EMAIL", "user@litprotocol.com") // localhost
        // .env("ETHERNAL_PASSWORD", "somepassword")
        .arg("hardhat")
        .arg("node")
        .stderr(Stdio::null()) // comment this out to see what's going on with hardhat
        .stdout(Stdio::null()) // comment this out to see what's going on with hardhat
        .group_spawn()
        .expect("Failed to launch Testnet")
}

pub fn compile_contracts() {
    // First, check if the contracts are compiled, and if not recompile them by running npx anvil test.
    if !artifacts_exist() {
        info!("Compiling contracts");
        _compile_contracts();
    } else {
        info!("Contracts are already compiled");
    }
}

fn contracts_dir() -> PathBuf {
    let dir = fs::canonicalize(LITCONTRACTPATH);
    match dir {
        Ok(path) => path,
        Err(_) => {
            // let's try and find the contracts dir by moving up the directory tree until we find blockchain/contracts
            let mut current_dir = env::current_dir().unwrap();
            while !current_dir.join("blockchain").join("contracts").exists() {
                current_dir = current_dir.parent().unwrap().to_path_buf();
            }
            fs::canonicalize(current_dir.join("blockchain").join("contracts")).unwrap()
        }
    }
}

fn lit_node_dir() -> PathBuf {
    // let's try and find the lit-node dir by moving up the directory tree until we find lit-node/config
    let mut current_dir = env::current_dir().unwrap();
    while !current_dir.join("lit-node").join("config").exists() {
        current_dir = current_dir.parent().unwrap().to_path_buf();
    }
    fs::canonicalize(current_dir.join("lit-node")).unwrap()
}

fn artifacts_exist() -> bool {
    let path = contracts_dir();
    path.join("artifacts/contracts/lit-node/Staking.sol/Staking.json")
        .exists()
        && path
            .join("artifacts/contracts/lit-node/LITToken.sol/LITToken.json")
            .exists()
        && path
            .join("artifacts/@openzeppelin/contracts/token/ERC20/ERC20.sol/ERC20.json")
            .exists()
}

fn _compile_contracts() {
    info!("{:?}", fs::canonicalize("./").unwrap());
    let path = contracts_dir();
    info!("Compiling in {:?}", path);
    let res = Command::new("npx")
        .current_dir(path)
        .arg("hardhat")
        .arg("compile")
        .output()
        //        .group_spawn()
        .expect("compile command failed");
    info!("{:?}", res);
}

pub fn default_staker_ip_addresses(base_port: usize, num_nodes: usize) -> Vec<String> {
    let mut ip_addresses = Vec::new();
    for i in 0..num_nodes {
        ip_addresses.push(format!("127.0.0.1:{}", base_port + i));
    }
    ip_addresses
}

pub async fn remote_deployment_and_config_creation(
    num_staked_and_joined_validators: usize,
    num_staked_only_validators: usize,
    generated_custom_node_runtime_config: bool,
) -> bool {
    // read and modify the config template
    let config_template_path = lit_node_dir()
        .join("config")
        .join("test")
        .join("deploy-config-template.json");
    let file = fs::File::open(config_template_path).expect("File should open read only");
    let reader = BufReader::new(file);
    let mut config: serde_json::Map<String, Value> =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");
    if let Some(deploy_node_config) = config
        .get_mut("deployNodeConfig")
        .and_then(|v| v.as_object_mut())
    {
        deploy_node_config["numberOfStakedAndJoinedWallets"] =
            num_staked_and_joined_validators.into();
        deploy_node_config["numberOfStakedOnlyWallets"] = num_staked_only_validators.into();

        // Set the IP addresses
        let ip_addresses = default_staker_ip_addresses(
            7470,
            num_staked_and_joined_validators + num_staked_only_validators,
        );
        deploy_node_config["ipAddresses"] =
            Value::Array(ip_addresses.into_iter().map(Value::String).collect());

        // if custom node runtime configs were generated, then set customNodeRuntimeConfigPath to ../../rust/lit-node/config/test/custom_node_runtime_config.toml
        if generated_custom_node_runtime_config {
            deploy_node_config.insert(
                "customNodeRuntimeConfigPath".into(),
                Value::String(
                    "../../rust/lit-node/config/test/custom_node_runtime_config.toml".to_string(),
                ),
            );
        }
    } else {
        panic!("deployNodeConfig key is missing or is not an object");
    }

    // write the config
    let config_path = lit_node_dir()
        .join("config")
        .join("test")
        .join("deploy-config.json");
    let output = serde_json::to_string_pretty(&config).expect("Failed to serialize config to JSON");
    fs::write(config_path, output).expect("Unable to write config to file");

    // use the JS deployment script to deploy the network
    let args = [
        "ts-node",
        "./scripts/deploy.ts",
        "--network",
        "localchain",
        "--deployConfig",
        "../../rust/lit-node/config/test/deploy-config.json",
    ];

    info!(
        "Running full command in {}: npx {}",
        contracts_dir().display(),
        args.join(" ")
    );

    let mut rv = populate_required_environment_variables(Command::new("npx").borrow_mut())
        .args(args)
        .current_dir(contracts_dir())
        // .stderr(Stdio::null()) // comment this out to see what's going on with hardhat deploy
        // .stdout(Stdio::null()) // comment this out to see what's going on with hardhat deploy
        .group_spawn()
        .expect("Failed to launch contract deploy script");
    let exit_code = rv.wait().expect("Failed to wait on contract deploy script");
    if !exit_code.success() {
        panic!(
            "Contract deploy script failed with exit code {:?}",
            exit_code
        );
    }

    // Print the wallets that got created.
    let wallet_manifest = latest_wallet_manifest(false);
    for wallet in wallet_manifest {
        info!(
            "Created wallets: idx {:?} node {:?} staker {:?}",
            wallet.idx, wallet.node.address, wallet.staker.address
        );
    }

    true
}

fn populate_required_environment_variables(command: &mut Command) -> &mut Command {
    if let Ok(ipfs_api_key) = std::env::var(ENV_IPFS_API_KEY) {
        command.env(ENV_IPFS_API_KEY, ipfs_api_key)
    } else {
        command
    }
}

pub async fn contract_addresses_from_deployment() -> ContractAddresses {
    // extract the addresses from the deployment script output
    let deployed_core_contracts_path = &format!(
        "{}/deployed-lit-core-contracts-temp.json",
        contracts_dir().display()
    );
    let deployed_node_contracts_path = &format!(
        "{}/deployed-lit-node-contracts-temp.json",
        contracts_dir().display()
    );

    // Read and parse JSON from deployed_core_contracts_path
    let file = fs::File::open(deployed_core_contracts_path).expect("File should open read only");
    let reader = BufReader::new(file);
    let mut core_contracts: serde_json::Map<String, Value> =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");

    // Read and parse JSON from deployed_node_contracts_path
    let file = fs::File::open(deployed_node_contracts_path).expect("File should open read only");
    let reader = BufReader::new(file);
    let node_contracts: serde_json::Map<String, Value> =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");

    // Merge node_contracts into core_contracts
    for (k, v) in node_contracts {
        core_contracts.insert(k, v);
    }

    // Fill and return the struct with values from core_contracts
    let contract_addresses = ContractAddresses {
        lit_token: H160::from_str(core_contracts["litTokenContractAddress"].as_str().unwrap())
            .unwrap(),
        backup_recovery: H160::from_str(
            core_contracts["backupRecoveryContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
        staking: H160::from_str(core_contracts["stakingContractAddress"].as_str().unwrap())
            .unwrap(),
        staking_balances: H160::from_str(
            core_contracts["stakingBalancesContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
        rate_limit_nft: H160::from_str(
            core_contracts["rateLimitNftContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
        pkpnft: H160::from_str(core_contracts["pkpNftContractAddress"].as_str().unwrap()).unwrap(),
        pkp_helper: H160::from_str(core_contracts["pkpHelperContractAddress"].as_str().unwrap())
            .unwrap(),
        pubkey_router: H160::from_str(
            core_contracts["pubkeyRouterContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
        pkp_permissions: H160::from_str(
            core_contracts["pkpPermissionsContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
        contract_resolver: H160::from_str(core_contracts["contractResolver"].as_str().unwrap())
            .unwrap(),
        key_deriver: H160::from_str(
            core_contracts["hdKeyDeriverContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
        payment_delegation: H160::from_str(
            core_contracts["paymentDelegationContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
    };

    contract_addresses
}

pub async fn check_and_load_chain_state_cache(
    provider: Arc<Provider<Http>>,
    num_staked: usize,
    num_nodes: usize,
) -> bool {
    let dir_name = format!("tests/chain_state_cache/{}_{}", num_staked, num_nodes);
    let dir = Path::new(&dir_name);
    if !dir.exists() {
        return false;
    }

    let filename = format!("anvil_state.hex");
    let path = dir.join(&filename);
    let mut file = File::open(&path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();

    let params: Vec<String> = vec![contents];
    let res: bool = provider.request("anvil_loadState", params).await.unwrap();
    if !res {
        error!("Couldn't load chain state into anvil");
        return false;
    }

    // also copy back the node configs
    let node_configs_path = node_configs_path();
    let dir_entries = fs::read_dir(dir).unwrap();
    for entry in dir_entries {
        let entry = entry.unwrap();
        if entry.path().extension().unwrap() == "toml" {
            let dest_path = Path::new(&node_configs_path).join(entry.file_name());
            fs::copy(entry.path(), dest_path).unwrap();
        }
    }

    // finally, put back the wallet
    let wallet_manifest_path = dir.join("wallet.json");
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let dest_path = Path::new(&contracts_dir()).join(format!(
        "wallets/wallets-{}-localchain-{}.json",
        timestamp, num_nodes
    ));
    fs::copy(wallet_manifest_path, dest_path).unwrap();
    info!("Chain state loaded from cache - not deploying contracts");
    true
}

pub async fn save_to_chain_state_cache(
    provider: Arc<Provider<Http>>,
    num_staked_and_joined_validators: usize,
    num_staked_only_validators: usize,
) {
    let params: Vec<String> = vec![];
    let res: String = provider.request("anvil_dumpState", params).await.unwrap();

    let dir_name = format!(
        "tests/chain_state_cache/{}_{}",
        num_staked_and_joined_validators, num_staked_only_validators
    );

    let dir = Path::new(&dir_name);
    if !dir.exists() {
        fs::create_dir_all(dir).unwrap();
    }

    let filename = format!("anvil_state.hex");
    let path = dir.join(&filename);
    let mut file = File::create(&path).await.unwrap();
    file.write_all(res.as_bytes()).await.unwrap();
    file.sync_all().await.unwrap();

    // also save the node configs
    let node_configs_path = node_configs_path();
    let node_configs = fs::read_dir(node_configs_path).unwrap();
    for entry in node_configs {
        let entry = entry.unwrap();
        let dest_path = dir.join(entry.file_name());
        fs::copy(entry.path(), dest_path).unwrap();
    }

    // also save the wallets
    let latest_wallet_manifest = latest_wallet_manifest(false);
    if latest_wallet_manifest.len()
        != (num_staked_and_joined_validators + num_staked_only_validators)
    {
        panic!("When saving chain state cache, number of nodes in latest_wallet_manifest.json does not match num_nodes in chain config");
    }
    // output latest_wallet_manifest into dir/wallet.json as json
    let wallet_manifest_path = dir.join("wallet.json");
    let wallet_manifest_json = serde_json::to_string(&latest_wallet_manifest).unwrap();
    tokio::fs::write(wallet_manifest_path, wallet_manifest_json)
        .await
        .unwrap();
}

/// Search within node_configs_path for the lit_configX.toml file that corresponds to the node_account parameters.
pub fn fetch_node_config_file_from_node_account(node_account: &NodeAccount) -> Result<String> {
    // List all files in node_configs_path
    let node_configs_path = node_configs_path();
    let dir_entries = fs::read_dir(node_configs_path)
        .map_err(|e| anyhow::anyhow!("Couldn't read directory: {}", e))?;

    // For each file, load the TOML and check for matching parameters
    for entry in dir_entries {
        let entry = entry.map_err(|e| anyhow::anyhow!("Couldn't read entry: {}", e))?;
        let path = entry.path();
        let config = SimpleToml::try_from(path.as_path())
            .map_err(|e| anyhow::anyhow!("Couldn't read config file: {}", e))?;

        // Check against node config
        let staker_address = config
            .get_address("node", "staker_address")
            .ok_or(anyhow::anyhow!("Couldn't retrieve the staking address"))?;
        let node_private_key = config
            .get_signing_key()
            .ok_or(anyhow::anyhow!("Couldn't retrieve the node wallet key"))?;

        if staker_address == node_account.staker_address
            && H256::from_slice(&node_private_key) == node_account.node_address_private_key
        {
            return path
                .to_str()
                .ok_or(anyhow::anyhow!("Couldn't convert path to string"))
                .map(|s| s.to_string());
        }
    }

    Err(anyhow::anyhow!("Couldn't find a matching node config file"))
}
