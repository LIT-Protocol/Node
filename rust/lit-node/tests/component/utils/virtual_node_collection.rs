use super::channel_mapper::setup_background_channels;
use arc_swap::ArcSwap;
use ethers::{prelude::*, utils::secret_key_to_address};
use flume::Receiver;
use generic_array::GenericArray;
use k256::ecdsa::SigningKey;
use libsecp256k1::{PublicKey, SecretKey};
use lit_blockchain::{
    config::LitBlockchainConfig,
    contracts::{backup_recovery::BackupRecovery, staking::Staking},
};
use lit_core::{config::LitConfig, utils::binary::bytes_to_hex};
use lit_node::{
    config::{
        chain::{ChainDataConfigManager, PeersForEpoch},
        load_cfg, LitNodeConfig,
    },
    models::{coms_keys::ComsKeys, PeerValidator},
    networking::http::client::HttpClientFactory,
    peers::{
        peer_item::{PeerData, PeerItem},
        peer_reviewer::PeerComplaint,
        peer_state::models::{PeerValidatorStatus, SimplePeer},
        PeerState,
    },
    tasks::{beaver_manager::models::BeaverMessage, utils::generate_hash},
    tss::{
        common::{
            models::{NodeTransmissionDetails, RoundData},
            tss_state::TssState,
        },
        ecdsa_cait_sith::CsEcdsaState,
    },
    utils::encoding,
    version::get_version,
};
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
    sync::{Arc, Mutex},
};
use test_common::testnet::Testnet;
use tracing::debug;

use xor_name::XorName;

pub struct VirtualNodeCollection {
    pub nodes: Vec<VirtualNode>,
    pub testnet: Testnet,
}

impl VirtualNode {
    pub async fn ecdsa_state(&self) -> CsEcdsaState {
        CsEcdsaState {
            state: self.tss_state.clone(),
            dkg_type: lit_node::tss::common::dkg_type::DkgType::Standard,
        }
    }
}

pub struct VirtualNode {
    pub node_id: usize,
    pub addr: String,
    pub tss_state: TssState,
    pub quit_rm_tx: tokio::sync::mpsc::Sender<bool>,
    pub quit_dm_tx: tokio::sync::mpsc::Sender<bool>,
    pub hex_staker_address: String,
    pub staker_address: H160,
    pub peer: SimplePeer,
    pub validator: PeerValidator,
    pub node_channels: BgChannels,
}

pub struct BgChannels {
    pub rx_node_transmission_details: flume::Receiver<NodeTransmissionDetails>,
    pub rx_round_data: flume::Receiver<RoundData>,
    pub tx_round_manager: Arc<flume::Sender<RoundData>>,
}

// a set of virtual nodes, each with their own TssState
// these nodes are not connected to any network, and use an internal channel to communicate

impl VirtualNodeCollection {
    pub async fn new(num_nodes: usize) -> Self {
        if num_nodes == 0 {
            panic!("num_nodes must be greater than 0");
        }
        let testnet = test_common::testnet::Testnet::builder()
            .which_testnet(test_common::testnet::WhichTestnet::NoChain)
            .build()
            .await;

        let mut vnc = VirtualNodeCollection {
            nodes: Vec::new(),
            testnet,
        };

        for i in 0..num_nodes {
            let port = 7470 + i as u16;
            // the key hash must be unique, so if we're using 0 as the staker address, we generate a random one
            let staker_address = H160::random();
            vnc.add_node_internal(port, staker_address).await;
        }
        // setup all the background channels
        vnc.update_internal_state().await;

        vnc
    }
    pub async fn add_node(&mut self) -> bool {
        let port = self.max_port() + 1;
        tracing::info!(
            "Adding a node at a random position on virtual port {} and remapping background channels.",
            port
        );
        // the key hash must be unique, so if we're using 0 as the staker address, we generate a random one
        let staker_address = H160::random();

        self.add_node_internal(port, staker_address).await;
        self.update_internal_state().await;
        true
    }

    pub fn node_by_staker_address(&self, staker_address: H160) -> Option<&VirtualNode> {
        self.nodes
            .iter()
            .find(|n| n.staker_address == staker_address)
    }

    pub async fn insert_node(&mut self, index: usize) -> bool {
        let port = self.max_port() + 1;
        tracing::info!(
            "Inserting a node at position {} on virtual port {} and remapping background channels.",
            index,
            port
        );

        let offset = u64::from_str("100").unwrap();
        // note that this technically could be incorrect if two key hashes are closer than 100 apart.
        let staker_address = if index >= self.nodes.len() {
            self.max_staker_address().to_low_u64_be() + offset
        } else {
            self.nodes
                .get(index - 1)
                .unwrap()
                .validator
                .staker_address
                .to_low_u64_be()
                - offset
        };

        let staker_address = H160::from_low_u64_be(staker_address);

        self.add_node_internal(port, H160::from(staker_address))
            .await;
        self.update_internal_state().await;
        true
    }

    pub async fn remove_first_node(&mut self) -> bool {
        self.remove_node(0).await
    }

    pub async fn remove_last_node(&mut self) -> bool {
        self.remove_node(self.nodes.len() - 1).await
    }

    pub async fn remove_node(&mut self, index: usize) -> bool {
        let index = if index > 0 { index - 1 } else { 0 };
        let _r = self.nodes[index].quit_dm_tx.send(true).await;
        let _r = self.nodes[index].quit_rm_tx.send(true).await;

        self.nodes.remove(index);
        self.update_internal_state().await;
        true
    }

    async fn update_internal_state(&mut self) {
        // update the internal indexation, to match IRL ordering.
        // we're actually using the key_hash as the ordering, since the staker address must be 0x0.
        self.nodes
            .sort_by(|a, b| a.validator.key_hash.cmp(&b.validator.key_hash));
        for (index, node) in self.nodes.iter_mut().enumerate() {
            node.validator.index = index as u16;
            node.peer.share_index = index as u16;
        }
        self.update_background_channels().await;
        self.update_cdm_validators().await;
        self.update_peer_data().await;
    }

    fn max_port(&self) -> u16 {
        self.nodes
            .iter()
            .map(|n| n.validator.port as u16)
            .max()
            .unwrap_or(0)
    }

    fn max_staker_address(&self) -> H160 {
        self.nodes
            .iter()
            .map(|n| n.validator.staker_address)
            .max()
            .unwrap_or(H160::zero())
    }

    async fn add_node_internal(&mut self, port: u16, staker_address: H160) -> bool {
        let node_id = self.nodes.iter().map(|n| n.node_id).max().unwrap_or(0) + 1;

        let key_hash = generate_hash(staker_address);
        let (tss_state, rx_round_data, rx_node_transmission_details) =
            load_virtual_node_defaults(port, &self.testnet, staker_address).await;
        let addr = tss_state.peer_state.addr.clone();

        let peer = SimplePeer {
            socket_address: addr.clone(),
            share_index: node_id as u16,
            key_hash,
            staker_address,
            kicked: false,
            protocol_index: None,
            version: lit_node::version::get_version(),
        };
        let node_id = node_id as usize;

        let addr_no_port = addr.clone().split(':').next().unwrap().to_string();
        let port = addr
            .clone()
            .split(':')
            .last()
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        let ip: Ipv4Addr = addr_no_port.parse().unwrap();
        let ipv6: Ipv6Addr = ip.to_ipv6_mapped();
        let ip: u32 = ip.into();
        let ipv6: u128 = ipv6.into();

        let validator = PeerValidator {
            ip,
            ipv6,
            port,
            address: Address::random(),
            reward: U256::from(10),
            coms_sender_pub_key: U256::from(0),
            coms_receiver_pub_key: U256::from(0),
            index: node_id as u16,
            staker_address,
            socket_addr: addr.clone(),
            is_kicked: false,
            key_hash,
            version: get_version().to_string(),
        };

        // technically not used until the background channels are set up.

        let (quit_dm_tx, _quit_dm_rx) = tokio::sync::mpsc::channel(1);
        let (quit_rm_tx, _quit_rm_rx) = tokio::sync::mpsc::channel(1);

        let node_channels = BgChannels {
            rx_node_transmission_details,
            rx_round_data,
            tx_round_manager: tss_state.tx_round_manager.clone(),
        };

        self.nodes.push(VirtualNode {
            node_id,
            addr: addr.clone(),
            tss_state,
            quit_dm_tx,
            quit_rm_tx,
            staker_address,
            hex_staker_address: bytes_to_hex(staker_address.as_bytes()),
            peer,
            validator,
            node_channels,
        });

        true
    }

    pub async fn shutdown(&mut self) {
        for node in self.nodes.iter_mut() {
            let _r = node.quit_dm_tx.send(true).await;
            let _r = node.quit_rm_tx.send(true).await;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    #[doc = "Returns the peers in order"]
    pub fn peers(&self) -> Vec<SimplePeer> {
        self.nodes.iter().map(|n| n.peer.clone()).collect()
    }

    pub fn validators(&self) -> Vec<PeerValidator> {
        self.nodes.iter().map(|n| n.validator.clone()).collect()
    }

    pub async fn update_cdm_epoch(&mut self, epoch: u64) {
        for node in self.nodes.iter_mut() {
            node.tss_state
                .chain_data_manager
                .peers_for_current_epoch
                .write()
                .await
                .epoch_number = epoch;
        }
    }

    pub async fn update_cdm_validators(&self) {
        let validators = self.validators();

        let current_peers = PeersForEpoch {
            epoch_id: "0".to_string(),
            validators,
            epoch_number: 0,
            epoch_read_time: std::time::SystemTime::now(),
        };

        for node in self.nodes.iter() {
            let mut peers_for_epoch = node
                .tss_state
                .chain_data_manager
                .peers_for_current_epoch
                .write()
                .await;
            peers_for_epoch.validators = current_peers.validators.clone();
            peers_for_epoch.epoch_id = current_peers.epoch_id.clone();
        }
    }

    pub async fn update_peer_data(&self) {
        let dummy_comms_keys: [u8; 32] = [0; 32];
        let mut rng = rand::thread_rng();
        let peer_items: Vec<PeerItem> = self
            .peers()
            .iter()
            .map(|sp| {
                PeerItem::new(
                    &sp.socket_address,
                    PublicKey::from_secret_key(&SecretKey::random(&mut rng)),
                    Address::zero(),
                    dummy_comms_keys,
                    dummy_comms_keys,
                    sp.staker_address,
                    PeerValidatorStatus::Survivor,
                    None,
                    sp.version.to_string().clone(),
                )
            })
            .collect();

        let mut peer_data = PeerData::new();
        for peer_item in peer_items {
            peer_data.insert(peer_item).unwrap();
        }

        for node in self.nodes.iter() {
            node.tss_state
                .peer_state
                .data
                .store(Arc::new(peer_data.clone()));
            node.tss_state
                .peer_state
                .curr_data
                .store(Arc::new(peer_data.clone()));
            node.tss_state
                .peer_state
                .union_data
                .store(Arc::new(peer_data.clone()));
        }
    }

    pub async fn update_background_channels(&mut self) {
        let lit_config = Arc::new(LitConfig::default());

        // shut down the old channels & and create a new quit channel
        for node in self.nodes.iter_mut() {
            let _r = node.quit_dm_tx.send(true).await;
            let _r = node.quit_rm_tx.send(true).await;
        }

        // setup all the background channels
        setup_background_channels(self, lit_config).await;
    }
}

// loads the default values for a virtual node
async fn load_virtual_node_defaults(
    port: u16,
    testnet: &test_common::testnet::Testnet,
    staker_address: H160,
) -> (
    TssState,
    Receiver<RoundData>,
    Receiver<NodeTransmissionDetails>,
) {
    let cfg = load_cfg().expect("failed to load LitConfig");
    let addr = format!("127.0.0.1:{}", port);
    let chain_data_manager = Arc::new(ChainDataConfigManager::new(cfg.clone()).await);
    let (pr_tx, _pr_rx) = flume::unbounded();
    let lit_config = cfg.load_full();

    // init HTTP client
    let http_client =
        HttpClientFactory::new_client(&lit_config).expect("Unable to init HTTP client");

    let (bm_tx, _bm_rx) = flume::bounded(10000);

    let peer_state = Arc::new(
        new_peer_state(
            addr.clone(),
            staker_address,
            pr_tx.clone(),
            cfg.load_full(),
            chain_data_manager.clone(),
            http_client.clone(),
            testnet,
            bm_tx.clone(),
        )
        .await,
    );
    let (metrics_tx, _) = flume::unbounded();
    let metrics_tx = Arc::new(metrics_tx);

    let (mut tss_state, rx_round_sender, rx_batch_sender) = TssState::init(
        peer_state.clone(),
        lit_config.clone(),
        chain_data_manager.clone(),
        metrics_tx.clone(),
    )
    .expect("Error initializing tss state");

    tss_state.addr = addr;
    tss_state.port = port as u32;

    (tss_state, rx_round_sender, rx_batch_sender)
}

// an override for peer state, to skip over chain details.
async fn new_peer_state(
    addr: String,
    staker_address: H160,
    complaint_channel: flume::Sender<PeerComplaint>,
    cfg: Arc<LitConfig>,
    chain_data_config_manager: Arc<ChainDataConfigManager>,
    http_client: reqwest::Client,
    testnet: &test_common::testnet::Testnet,
    bm_tx: flume::Sender<BeaverMessage>,
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

    let provider = testnet.deploy_account.signing_provider.clone();
    let staking_contract = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
        Address::zero(),
        provider.clone(),
    );
    let backup_recovery_contract = BackupRecovery::<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >::new(Address::zero(), provider.clone());

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
        staker_address,
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
