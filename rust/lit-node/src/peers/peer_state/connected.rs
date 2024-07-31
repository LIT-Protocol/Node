use super::super::{
    peer_item::{PeerData, PeerItem},
    peer_reviewer::{Issue, PeerComplaint},
    PeerState,
};
use std::sync::Arc;

use ethers::types::{Address, U256};
use libsecp256k1::PublicKey;
use lit_attestation::AttestationType;
use rand_core::RngCore;
use tracing::warn;

use crate::config::LitNodeConfig;
use crate::error::{blockchain_err_code, http_client_err, lock_err, unexpected_err, Result, EC};
use crate::models::PeerValidator;
use crate::utils::networking::get_web_addr_from_chain_info;
use crate::version;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;

use ethers::abi::AbiEncode;
use lit_attestation::kdf::Kdf;
use lit_attestation::verification::Policy;
use openssl::sha::sha256;
use rand::rngs::StdRng;
use rand_core::SeedableRng;
use std::time::{SystemTime, UNIX_EPOCH};

use super::models::PeerValidatorStatus;

pub async fn rng<C: AsRef<LitConfig>>(cfg: C) -> Result<StdRng> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| unexpected_err(e, Some("cannot produce secure RNG".into())))?;
    let hex_nanos = time.as_nanos().encode_hex();
    let secure_entropy = Kdf::try_derive(cfg.as_ref(), hex_nanos).await?;
    let seed = sha256(&secure_entropy);
    let rng = StdRng::from_seed(seed);
    Ok(rng)
}

#[allow(dead_code)]
impl PeerState {
    // ############# Functions to read and alter the connected peers (and struct)
    pub async fn find_peers(&self, peers: Vec<PeerValidator>) -> Result<()> {
        self.find_peers_ext(peers, false).await
    }

    pub async fn find_peers_ext(&self, peers: Vec<PeerValidator>, is_union: bool) -> Result<()> {
        let mut futures = Vec::new();
        for peer in peers.iter() {
            futures.push(self.connect_to_node(peer));
        }

        {
            *self
                .connecting
                .lock()
                .map_err(|e| lock_err(e.to_string(), Some("Locked Connecting Flag".into())))? =
                true;
        }

        // join_all is hanging.  removing for now.  this may mean these requests happen sequentially but that's better than this whole thing hanging.
        let mut node_infos = Vec::with_capacity(peers.len());
        for f in futures {
            node_infos.push(f.await);
        }

        let mut data = PeerData::clone(&self.data.load());

        for node_info in node_infos {
            let node_info = match node_info {
                Ok(node_info) => {
                    // trace!("Connected to peer: {:?}", node_info.addr);
                    node_info
                }
                Err(e) => {
                    warn!("Error connecting to peer: {:?}", e);
                    continue;
                }
            };

            // to believe the node (public key is not on contract, but addr and staker addr is)
            let pi = PeerItem::new(
                &node_info.addr,
                node_info.public_key,
                node_info.node_address,
                node_info.sender_public_key,
                node_info.receiver_public_key,
                node_info.staker_address,
                PeerValidatorStatus::Unknown,
                None,
                node_info.version,
            );
            data.insert(pi)
                .expect_or_err("failed to insert into PeerItem")?;
        }

        data.table
            .sort_by(|a, b| a.staker_address.cmp(&b.staker_address)); // keep it sorted

        if is_union {
            self.union_data.store(Arc::new(data.clone()));
        }
        self.data.store(Arc::new(data));

        // unlock the connecting lock
        {
            *self
                .connecting
                .lock()
                .map_err(|e| lock_err(e.to_string(), Some("Locked Connecting Flag".into())))? =
                false
        }

        Ok(())
    }

    pub async fn connect_to_node(&self, peer: &PeerValidator) -> Result<PeerItem> {
        // hang on, are we trying to connect to ourselves?
        // if so, let's just not do that.  let's just load up our own nodeinfo.

        let addr_string = get_web_addr_from_chain_info(peer.ip, peer.port);
        let addr = addr_string.as_str();
        let peer_item;
        if addr == self.addr {
            let key_as_bytes = self.public_key.to_encoded_point(false);
            let key_as_bytes = key_as_bytes.as_bytes();
            let key_as_bytes: &[u8; 65] = key_as_bytes.try_into().map_err(|e| {
                unexpected_err(
                    e,
                    Some(format!(
                        "Could not convert key_as_bytes into length 65.  It's length is {}",
                        key_as_bytes.len()
                    )),
                )
            })?;
            let public_key = PublicKey::parse(key_as_bytes).map_err(|e| {
                unexpected_err(
                    e,
                    Some(format!(
                        "Failed to convert VerifyingKey to PublicKey for node {}",
                        self.addr
                    )),
                )
            })?;
            peer_item = PeerItem {
                id: self.peer_id,
                public_key,
                node_address: self.node_address,
                sender_public_key: self.comskeys.sender_public_key(),
                receiver_public_key: self.comskeys.receiver_public_key(),
                staker_address: self.staker_address,
                addr: self.addr.clone(),
                status: PeerValidatorStatus::Unknown,
                attestation: None,
                version: version::get_version().to_string(),
            }
        } else {
            let mut gen = rng(&self.lit_config).await?;
            let mut noonce_bytes = vec![0u8; 32];
            gen.try_fill_bytes(&mut noonce_bytes).map_err(|e| {
                unexpected_err(
                    e,
                    Some("cannot create noonce for attestment challenge".into()),
                )
            })?;
            let noonce = hex::encode(noonce_bytes.as_slice());

            let url = format!(
                "{}{}/{}/{}",
                self.lit_config.http_prefix_when_talking_to_other_nodes(),
                addr,
                "connect",
                noonce
            );
            let resp = self.http_client.get(url.clone()).send().await;
            match resp {
                Err(e) => {
                    if e.is_timeout() || e.is_connect() {
                        warn!("Connect request to {:?} has failed, attempting to get peer staker address for complaint: {:?}", addr, e);

                        match self.get_staker_address_from_socket_addr(addr).await {
                            Ok(peer_staker_address_to_complain) => {
                                let complainer = self.addr.clone();
                                let complaint_channel = self.complaint_channel.clone();
                                if let Err(e) = complaint_channel
                                    .send_async(PeerComplaint {
                                        complainer,
                                        issue: Issue::Unresponsive,
                                        peer_node_staker_address: peer_staker_address_to_complain,
                                    })
                                    .await
                                {
                                    error!("Failed to send complaint to channel: {:?}", e);
                                }
                            }
                            Err(e) => {
                                error!("Unable to get peer staker address for complaint: {:?}", e);
                            }
                        }
                    }
                    return Err(http_client_err(e, Some("Failed to connect to peer".into())));
                }
                Ok(body) => {
                    peer_item = body
                        .json::<PeerItem>()
                        .await
                        .map_err(|e| http_client_err(e, None))?;

                    // verify NodeInfo against node information that is registered in the Staking contract
                    // upon registration.
                    let verify_res = self
                        .verify_peer_item(
                            peer_item.node_address,
                            peer,
                            peer_item.clone(),
                            noonce_bytes.as_slice(),
                            addr,
                        )
                        .await;
                    if let Err(verify_err) = verify_res {
                        // If the error is EC::NodeRpcError, log error and rethrow Error without complaining Peer.
                        // Rethrowing Error will cause this code path to be run again at some later time by the caller.
                        if verify_err.is_code(EC::NodeRpcError, true) {
                            error!("Error verifying node info: {:?}", verify_err);
                            return Err(verify_err);
                        } else {
                            warn!(
                                "Node {:?} provided incorrect node info {} - Err: {:?}. Complaining.",
                                addr,
                                peer_item,
                                verify_err
                            );
                            let err_msg = "Node provided incorrect info.";
                            let peer_staker_address_to_complain = peer.staker_address;
                            let complainer = self.addr.clone();
                            let complaint_channel = self.complaint_channel.clone();

                            if let Err(e) = complaint_channel
                                .send_async(PeerComplaint {
                                    complainer,
                                    issue: Issue::IncorrectInfo {
                                        err: anyhow::Error::msg(err_msg),
                                    },
                                    peer_node_staker_address: peer_staker_address_to_complain,
                                })
                                .await
                            {
                                error!("Failed to send complaint to channel: {:?}", e);
                            }

                            return Err(unexpected_err(err_msg, None));
                        }
                    }
                }
            }
        }

        Ok(peer_item)
    }

    async fn verify_peer_item(
        &self,
        registered_node_address: Address,
        validator: &PeerValidator,
        peer_item_to_verify: PeerItem,
        noonce: &[u8],
        address_we_talked_to: &str,
    ) -> Result<()> {
        // first, get relevant info from chain
        let registered_staker_address = self
            .staking_contract
            .node_address_to_staker_address(registered_node_address)
            .call()
            .await
            .map_err(|e| blockchain_err_code(e, EC::NodeRpcError, None))?;

        // verify node address
        if peer_item_to_verify.node_address != registered_node_address {
            return Err(unexpected_err(
                format!(
                    "node_address different from chain.  Peer item node_address: {:?}, chain node_address: {:?}",
                    peer_item_to_verify.node_address,
                    registered_node_address
                ),
                None
            ));
        }

        // verify web address
        if peer_item_to_verify.addr != get_web_addr_from_chain_info(validator.ip, validator.port) {
            return Err(unexpected_err(
                format!(
                    "addr different from chain.  Peer item addr: {:?}, chain addr: {:?}",
                    peer_item_to_verify.addr,
                    get_web_addr_from_chain_info(validator.ip, validator.port)
                ),
                None,
            ));
        }

        // verify communication keys
        if U256::from_big_endian(&peer_item_to_verify.sender_public_key)
            != validator.coms_sender_pub_key
        {
            return Err(unexpected_err(
                format!("sender_pubkey different from chain.  Peer item sender_pubkey little endian: {:?}, Peer item sender_pubkey big endian: {:?}, chain sender_pubkey: {:?}",
                        U256::from_little_endian(&peer_item_to_verify.sender_public_key),
                        U256::from_big_endian(&peer_item_to_verify.sender_public_key), validator.coms_sender_pub_key),
                None
            ));
        }

        if U256::from_big_endian(&peer_item_to_verify.receiver_public_key)
            != validator.coms_receiver_pub_key
        {
            return Err(unexpected_err(
                format!("receiver_pubkey different from chain.  Peer item receiver_pubkey little endian: {:?}, Local receiver_pubkey big endian: {:?}, chain receiver_pubkey: {:?}",
                        U256::from_little_endian(&peer_item_to_verify.receiver_public_key), U256::from_big_endian(&peer_item_to_verify.receiver_public_key), validator.coms_receiver_pub_key),
                None
            ));
        }

        // verify staker address
        if peer_item_to_verify.staker_address != registered_staker_address {
            tracing::debug!("Peer item: {:?}", peer_item_to_verify);
            tracing::debug!("Validator from chain: {:?}", validator);

            return Err(unexpected_err(
                format!("staker_address different from chain.  Peer item staker_address: {:?}, chain staker_address: {:?}",
                        peer_item_to_verify.staker_address, registered_staker_address),
                None
            ));
        }

        async fn check_attestation<N: AsRef<[u8]>>(
            lit_config: Arc<LitConfig>,
            peer_item_to_verify: &PeerItem,
            noonce: N,
            address_we_talked_to: &str,
        ) -> Result<()> {
            let attestation = peer_item_to_verify.attestation.as_ref().ok_or_else(|| {
                unexpected_err(
                    format!(
                        "empty attestation from peer. peer's node_address: {:?}",
                        peer_item_to_verify.node_address,
                    ),
                    None,
                )
            })?;

            let cfg = lit_config.as_ref();
            attestation
                .verify_full(cfg, None, Some(Policy::NodeConnect))
                .await
                .map_err(|e| {
                    unexpected_err(
                        e,
                        Some(format!(
                            "invalid attestation from peer. peer's node_address: {:?}",
                            peer_item_to_verify.node_address,
                        )),
                    )
                })?;

            if attestation.noonce().as_slice() != noonce.as_ref() {
                return Err(unexpected_err(
                    format!(
                        "invalid attestation from peer; incorrect noonce. peer's node_address: {:?}",
                        peer_item_to_verify.node_address,
                    ),
                    None,
                ));
            }

            // also check ip address and port from attestation report
            let split_addr: Vec<&str> = address_we_talked_to.split(':').collect();
            let ip_address_we_talked_to = split_addr[0];
            let port_we_talked_to = split_addr[1];

            let external_addr_from_report = attestation
                .external_addr()
                .expect("Could not get external_addr from attestation report");
            let split_addr_from_report: Vec<&str> = external_addr_from_report.split(':').collect();
            let ip_address_from_report = split_addr_from_report[0];
            let port_from_report = split_addr_from_report[1];

            if ip_address_from_report != ip_address_we_talked_to {
                return Err(unexpected_err(
                    format!(
                        "invalid attestation from peer; incorrect ip address. peer's node_address: {:?}, ip address we talked to: {}, ip address from report: {}",
                        peer_item_to_verify.node_address,
                        ip_address_we_talked_to,
                        ip_address_from_report,
                    ),
                    None,
                ));
            }

            if port_from_report != port_we_talked_to {
                return Err(unexpected_err(
                    format!(
                        "invalid attestation from peer; incorrect port. peer's node_address: {:?}, port we talked to: {}, port from report: {}",
                        peer_item_to_verify.node_address,
                        port_we_talked_to,
                        port_from_report,
                    ),
                    None,
                ));
            }

            Ok(())
        }

        // verify attestation
        if AttestationType::from_system().is_some() {
            // trace!(
            //     "verifying attestation for peer {:?}",
            //     peer_item_to_verify.node_address
            // );
            check_attestation(
                self.lit_config.clone(),
                &peer_item_to_verify,
                noonce,
                address_we_talked_to,
            )
            .await?
        } else {
            #[cfg(not(feature = "testing"))]
            warn!("skipping attestation check because attestation type is not known or is none");
        }

        Ok(())
    }

    pub fn get_peer_item_from_addr(&self, peer_addr: &str) -> Result<PeerItem> {
        let peer_state_data = self.union_data.load();
        peer_state_data
            .get_peer_by_addr(peer_addr)
            .expect_or_err(format!("PeerItem not found for peer_addr: {}", peer_addr))
    }

    pub fn get_peer_item_from_staker_addr(&self, staker_address: Address) -> Result<PeerItem> {
        let peer_state_data = self.union_data.load();
        peer_state_data
            .get_peer_by_staker_addr(staker_address)
            .expect_or_err(format!(
                "PeerItem not found for staker address: {}",
                staker_address
            ))
    }

    pub fn connected_nodes(&self) -> Result<Vec<PeerItem>> {
        let peer_data = self.data.load();
        let peer_items = &peer_data.table;

        Ok(peer_items.clone())
    }

    pub fn curr_connected_nodes(&self) -> Result<Vec<PeerItem>> {
        let peer_data = self.curr_data.load();
        let peer_items = &peer_data.table;

        Ok(peer_items.clone())
    }

    // Replaces existing curr_data with PeerData of target_peers
    pub fn set_curr_data_peers(&self, target_peers: Vec<PeerValidator>) -> Result<()> {
        let mut curr_data = PeerData::clone(&self.curr_data.load());
        curr_data.clear_table();
        self.append_curr_data_peers(target_peers)?;

        Ok(())
    }

    // Appends to existing curr_data with PeerData of peer_addresses
    pub fn append_curr_data_peers(&self, peer_addresses: Vec<PeerValidator>) -> Result<()> {
        let mut curr_data = PeerData::clone(&self.curr_data.load());
        let data = PeerData::clone(&self.data.load());
        for peer in &self.data.load().table {
            for validator in &peer_addresses {
                if validator.address == peer.node_address {
                    curr_data
                        .insert(peer.clone())
                        .expect_or_err("failed to insert into PeerItem")?;
                }
            }
        }
        curr_data
            .table
            .sort_by(|a, b| a.staker_address.cmp(&b.staker_address));
        self.curr_data.store(Arc::new(curr_data));

        Ok(())
    }

    pub fn connected_node_addrs(&self) -> Result<Vec<String>> {
        let peer_data = self.data.load();
        // collect all addr
        let addrs = peer_data.table.iter().map(|pi| pi.addr.clone()).collect();
        Ok(addrs)
    }

    pub fn is_connecting(&self) -> Result<bool> {
        let is_connecting = self
            .connecting
            .lock()
            .map_err(|e| lock_err(e.to_string(), Some("Locked Connecting Flag".into())))?;

        Ok(*is_connecting)
    }
    pub async fn get_peer_staker_address_for_complain(&self, addr: &str) -> Result<Address> {
        let peer_item = self.get_peer_item_from_addr(addr);
        if let Ok(peer_item) = peer_item {
            return Ok(peer_item.staker_address);
        }

        debug!(
            "Failed to get peer item from addr: {:?}, trying from chain data",
            addr
        );

        self.get_staker_address_from_socket_addr(addr)
            .await
            .map_err(|e| {
                unexpected_err(
                    e,
                    Some("Failed to get peer staker address from chain".into()),
                )
            })
    }
}
