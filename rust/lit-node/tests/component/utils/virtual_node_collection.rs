use super::channel_mapper::setup_background_channels;
use crate::common::{
    config::write_new_config_from_scenario, node_collection::NodeConfig,
    testnet::scenario::Scenario,
};
use arc_swap::ArcSwap;
use ethers::{prelude::*, utils::secret_key_to_address};
use flume::Receiver;
use generic_array::GenericArray;
use k256::ecdsa::SigningKey;
use lit_blockchain::{
    config::LitBlockchainConfig,
    contracts::{backup_recovery::BackupRecovery, staking::Staking},
};
use lit_core::{
    config::{LitConfig, ReloadableLitConfig, ENV_LIT_CONFIG_FILE},
    utils::binary::bytes_to_hex,
};
use lit_node::{
    config::{
        chain::{ChainDataConfigManager, PeersForEpoch},
        load_cfg, LitNodeConfig,
    },
    models::{coms_keys::ComsKeys, PeerValidator},
    networking::http::client::HttpClientFactory,
    peers::PeerState,
    peers::{
        peer_item::PeerData, peer_reviewer::PeerComplaint, peer_state::models::PeerSocketAddress,
    },
    tasks::beaver_manager::models::{generate_hash, BeaverMessage},
    tss::{
        common::{
            models::{NodeTransmissionDetails, RoundData},
            tss_state::TssState,
        },
        ecdsa_cait_sith::CsEcdsaState,
    },
    utils::encoding,
};
use std::{
    collections::HashMap,
    fs,
    net::{Ipv4Addr, Ipv6Addr},
    path::Path,
    sync::{Arc, Mutex},
};
use xor_name::XorName;

pub struct VirtualNodeCollection {
    pub nodes: Vec<VirtualNode>,
    pub peers: Vec<PeerSocketAddress>,
}

pub struct VirtualNode {
    pub node_id: usize,
    pub addr: String,
    pub tss_state: TssState,
    pub config_file: String,
    pub staker_address: String,
    pub tx: tokio::sync::mpsc::Sender<bool>,
}

// a set of virtual nodes, each with their own TssState
// these nodes are not connected to any network, and use an internal channel to communicate
pub async fn new_virtual_node_collection(num_nodes: usize) -> (VirtualNodeCollection, Scenario) {
    if num_nodes == 0 {
        panic!("num_nodes must be greater than 0");
    }

    let mut nodes: Vec<VirtualNode> = Vec::with_capacity(num_nodes);
    let initial_port = 7470;
    let num_staked = num_nodes;
    let is_fault_test = false;

    let s = Scenario::builder()
        .num_staked(num_staked)
        .num_nodes(num_nodes)
        .num_awake_initially(num_nodes)
        .initial_port(initial_port)
        .is_fault_test(is_fault_test)
        .build_blank()
        .await;

    let mut peers: Vec<PeerSocketAddress> = Vec::with_capacity(num_nodes);
    let mut validators: Vec<PeerValidator> = Vec::with_capacity(num_nodes);
    let mut map_tx_round_senders: HashMap<String, Arc<flume::Sender<RoundData>>> = HashMap::new();
    let mut map_rx_node_transmission_details: Vec<Arc<flume::Receiver<NodeTransmissionDetails>>> =
        Vec::new();
    let mut map_rx_round_senders: Vec<(
        tokio::sync::mpsc::Receiver<bool>,
        flume::Receiver<RoundData>,
    )> = Vec::new();

    let mut cfg = Arc::new(LitConfig::default());

    for i in 0..num_nodes {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let staker_address = Address::random();
        let key_hash = generate_hash(staker_address);
        let (_success, config_file) =
            write_virtual_node_config(&s, i, bytes_to_hex(staker_address.as_bytes()));
        let (tss_state, rx_round_data, rx_node_transmission_details, lit_config) =
            load_virtual_node_defaults(&s, &config_file).await;
        let addr = tss_state.peer_state.addr.clone();

        map_tx_round_senders.insert(addr.clone(), tss_state.tx_round_manager.clone());
        map_rx_node_transmission_details.push(Arc::new(rx_node_transmission_details));
        map_rx_round_senders.push((rx, rx_round_data));
        peers.push(PeerSocketAddress {
            address: addr.clone(),
            share_index: i as u32,
            key_hash,
            kicked: false,
        });
        let node_id = i;

        nodes.push(VirtualNode {
            node_id,
            addr: addr.clone(),
            tss_state,
            config_file,
            tx,
            staker_address: bytes_to_hex(staker_address.as_bytes()),
        });

        let addr_no_port = addr.split(':').next().unwrap().to_string();
        let ip: Ipv4Addr = addr_no_port.parse().unwrap();
        let ipv6: Ipv6Addr = ip.to_ipv6_mapped();
        let ip: u32 = ip.into();
        let ipv6: u128 = ipv6.into();

        let staker_address = Address::random();
        let key_hash = generate_hash(staker_address);
        validators.push(PeerValidator {
            ip,
            ipv6,
            port: addr.split(':').last().unwrap().parse().unwrap(),
            address: Address::random(),
            reward: U256::from(10),
            coms_sender_pub_key: U256::zero(),
            coms_receiver_pub_key: U256::zero(),
            index: i as u32,
            staker_address,
            socket_addr: addr,
            is_kicked: false,
            key_hash,
        });

        cfg = lit_config.load_full();
    }

    let current_peers = PeersForEpoch {
        epoch_id: "0".to_string(),
        validators,
        epoch_number: 0,
    };

    for node in nodes.iter() {
        let mut peers_for_epoch = node
            .tss_state
            .chain_data_manager
            .peers_for_current_epoch
            .write()
            .await;
        peers_for_epoch.validators = current_peers.validators.clone();
        peers_for_epoch.epoch_id = current_peers.epoch_id.clone();
    }

    let lit_config = cfg;
    let vnc = VirtualNodeCollection { nodes, peers };
    let _l = setup_background_channels(
        map_rx_node_transmission_details,
        map_tx_round_senders,
        map_rx_round_senders,
        lit_config,
    )
    .await;

    (vnc, s)
}

// writes a new config file for a virtual node
fn write_virtual_node_config(
    scenario: &Scenario,
    index: usize,
    _staker_address: String,
) -> (bool, String) {
    let config_folder = match scenario.testnet.existing_config_path.clone() {
        Some(path) => format!("./config/test/{}", path),
        None => format!(
            "./config/test/{}/{}",
            scenario.testnet.chain_name,
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        ),
    };
    if !Path::new(&config_folder).exists() {
        fs::create_dir_all(&config_folder)
            .unwrap_or_else(|_| panic!("Unable to create config folder: {}", config_folder));
    };
    let rocket_port = scenario.initial_port + index;
    let config_file = format!("{}/lit_config{:?}.toml", config_folder, &rocket_port);
    write_new_config_from_scenario(
        scenario,
        config_file.clone(),
        rocket_port,
        NodeConfig {
            lit_domain_name: "127.0.0.1".to_string(),
            rocket_port: rocket_port.to_string(),
            staker_address: "0x0".to_string(),
            enable_proxied_http_client: Some(scenario.is_fault_test),
        },
    );

    (true, config_file)
}

// loads the default values for a virtual node
async fn load_virtual_node_defaults(
    scenario: &Scenario,
    config_file: &str,
) -> (
    TssState,
    Receiver<RoundData>,
    Receiver<NodeTransmissionDetails>,
    ReloadableLitConfig,
) {
    std::env::set_var(ENV_LIT_CONFIG_FILE, config_file);
    let cfg = load_cfg().expect(&format!("failed to load LitConfig for {}", config_file));

    let addr = cfg
        .load()
        .external_addr()
        .expect("failed to get external addr");

    let chain_data_manager = Arc::new(ChainDataConfigManager::new(cfg.clone()));

    let (pr_tx, _pr_rx) = flume::unbounded();

    // init HTTP client
    let http_client =
        HttpClientFactory::new_client(cfg.load().as_ref()).expect("Unable to init HTTP client");

    let (bm_tx, _bm_rx) = flume::bounded(10000);

    let peer_state = Arc::new(
        new_peer_state(
            addr.clone(),
            pr_tx.clone(),
            cfg.load_full(),
            chain_data_manager.clone(),
            http_client.clone(),
            bm_tx.clone(),
            scenario.contracts.backup_recovery.clone(),
            scenario.contracts.staking.clone(),
        )
        .await,
    );
    let (metrics_tx, _) = flume::unbounded();
    let metrics_tx = Arc::new(metrics_tx);
    let (tss_state, rx_round_sender, rx_batch_sender, _rx_peer_reviewer_bridge) = TssState::init(
        peer_state.clone(),
        cfg.load_full(),
        http_client.clone(),
        chain_data_manager.clone(),
        metrics_tx.clone(),
    )
    .expect("Error initializing tss state");

    (tss_state, rx_round_sender, rx_batch_sender, cfg)
}

// an override for peer state, to skip over chain details.
async fn new_peer_state(
    addr: String,
    complaint_channel: flume::Sender<PeerComplaint>,
    cfg: Arc<LitConfig>,
    chain_data_config_manager: Arc<ChainDataConfigManager>,
    http_client: reqwest::Client,
    bm_tx: flume::Sender<BeaverMessage>,
    backup_recovery_contract: BackupRecovery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    staking_contract: Staking<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> PeerState {
    let secret_key = SigningKey::from_bytes(GenericArray::from_slice(
        &encoding::hex_to_bytes(
            cfg.blockchain_wallet_private_key(None)
                .expect("Failed to get blockchain wallet private key"),
        )
        .expect("Failed to hex encode node.private_key config var"),
    ))
    .expect("Could not convert node.private_key config var to SigningKey");

    let node_address = secret_key_to_address(&secret_key);
    let public_key = secret_key.verifying_key();

    PeerState {
        data: ArcSwap::from(Arc::new(PeerData::new())),
        curr_data: ArcSwap::from(Arc::new(PeerData::new())),
        union_data: ArcSwap::from(Arc::new(PeerData::new())),
        peer_id: XorName::from_content(addr.clone().as_bytes()),
        addr,
        public_key: *public_key,
        node_address,
        secret_key,
        connecting: Mutex::new(false),
        staking_address: staking_contract.address(),
        staker_address: staking_contract.address(), // wrong, but it doesn't matter
        rpc_url: cfg.rpc_url().expect("failed to get rpc url"),
        backup_recovery_contract,
        staking_contract,
        complaint_channel,
        chain_data_config_manager,
        comskeys: ComsKeys::from_node_config(&cfg)
            .expect("failed to get comskeys from node config"),
        lit_config: cfg,
        http_client,
        bm_tx,
    }
}

impl VirtualNodeCollection {
    pub async fn shutdown(&mut self) {
        for node in self.nodes.iter_mut() {
            let _r = node.tx.send(true).await;
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    pub async fn set_epoch(&mut self, epoch: u64) {
        for node in self.nodes.iter_mut() {
            node.tss_state
                .chain_data_manager
                .peers_for_current_epoch
                .write()
                .await
                .epoch_number = epoch;
        }
    }
}

impl VirtualNode {
    pub async fn ecdsa_state(&self) -> CsEcdsaState {
        CsEcdsaState {
            state: self.tss_state.clone(),
            http_client: reqwest::Client::default(),
            dkg_type: lit_node::tss::common::key_type::DkgType::Standard,
        }
    }
}
