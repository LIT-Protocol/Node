use super::super::dkg::gennaro::{GennaroDkg, Mode, RoundResult};
use crate::error::{conversion_err, unexpected_err, Result};
use crate::peers::peer_state::models::{PeerSocketAddress, PeerSocketAddressVec};
use crate::tss::common::peer_reviewer_bridge::{Message, RegisterPeersMessage};
use crate::tss::common::web::models::NodeConfig;
use crate::tss::dkg::gennaro::GennaroState;
use crate::utils::consensus::get_threshold_count;
use elliptic_curve::group::{Group, GroupEncoding};
use gennaro_dkg::Parameters;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tracing::instrument;

use super::super::common::comms::channels::register_comms_channel;
use super::super::common::comms::push::node_share_push_direct;
use super::super::common::comms::wait::node_share_await;

impl<G: Group + GroupEncoding + Default> GennaroState<G> {
    #[instrument(skip_all, fields(txn_prefix = txn_prefix))]
    pub async fn execute(
        &self,
        mode: Mode,
        txn_prefix: &str,
        current_peers: &Vec<PeerSocketAddress>,
        next_peers: &Vec<PeerSocketAddress>,
        share: Option<<G as Group>::Scalar>,
    ) -> Result<(G, G::Scalar, usize)> {
        // setup Gennaro DKG

        let usize_limit = next_peers.len();
        let usize_threshold = get_threshold_count(usize_limit);
        let threshold = NonZeroUsize::new(get_threshold_count(usize_limit))
            .expect_or_err("Empty non-zero usize threshold")?;
        let limit = NonZeroUsize::new(usize_limit).expect_or_err("Empty non-zero usize limit")?;
        let parameters = Parameters::<G>::new(threshold, limit);
        let expected_items = usize_limit - 1;

        // Register the peers involved in the DKG with the Peer Reviewer Bridge
        if let Err(e) = self
            .state
            .tx_peer_reviewer_bridge
            .send_async(Message::RegisterPeers(RegisterPeersMessage {
                txn_id: txn_prefix.to_string(),
                peers: next_peers.clone(),
            }))
            .await
        {
            error!(
                "Error sending RegisterPeers message to peer_reviewer_bridge: {:?}",
                e
            );
        }

        let peer_id = self.get_peer_id(next_peers)?;
        let mut dkg = GennaroDkg::<G>::default();
        dkg.init_participant(
            &mode,
            peer_id,
            next_peers,
            current_peers,
            &self.state.addr,
            share,
        )?;

        info!(
            "Peer ID: {:?}  /   Threshold: {:?}  /  Limit:  {:?} / TXN {} :",
            peer_id, usize_threshold, usize_limit, txn_prefix
        );

        let mut data_to_send: Vec<(u16, String)> = Vec::with_capacity(next_peers.len());
        loop {
            let round_output = match dkg.execute()? {
                Some(round_output) => round_output,
                None => {
                    dkg.clear_dkg_data();
                    break;
                }
            };
            for (index, peer) in next_peers.iter().enumerate() {
                if peer.address != self.state.addr {
                    if let Some(value) = RoundResult::from_round_outputs(&round_output, index) {
                        data_to_send.push((
                            index as u16,
                            serde_json::to_string(&value).map_err(|e| {
                                conversion_err(e, Some("Unable to serialize round output".into()))
                            })?,
                        ));
                    }
                }
            }

            let next_peers_without_self = next_peers
                .iter()
                .filter(|p| p.address != self.state.addr)
                .collect::<Vec<&PeerSocketAddress>>();
            let received = self
                .send_receive(
                    txn_prefix,
                    next_peers,
                    &dkg.get_round().expect_or_err("Empty round")?.to_string(),
                    &next_peers_without_self,
                    &data_to_send,
                )
                .await?;

            for (index, data) in received.iter() {
                let data = std::str::from_utf8(data).unwrap_or("ERR");
                let peer_data: RoundResult<G> = serde_json::from_str(data).map_err(|e| {
                    conversion_err(e, Some("Unable to deserialize round output".into()))
                })?;
                if let Err(e) = dkg.add_peer_data((index + 1) as usize, peer_data) {
                    error!("Error while adding peer data: {}", e);
                }
            }
            data_to_send.clear();
        }

        Ok((
            dkg.get_public_key().expect_or_err("Empty public key")?,
            dkg.get_secret_share().expect_or_err("Empty secret share")?,
            peer_id,
        ))
    }

    #[instrument(skip_all, fields(txn_prefix = txn_prefix))]
    async fn send_receive(
        &self,
        txn_prefix: &str,
        peers: &Vec<PeerSocketAddress>,
        round: &str,
        expected_items_from: &Vec<&PeerSocketAddress>,
        data_to_send: &[(u16, String)],
    ) -> Result<Vec<(u16, Vec<u8>)>> {
        // channel setup -> we're going to set up a channel for each round.  We could do this for each DKG as well, but this is a bit easier to debug :-)
        let tx_batch_sender = &self.state.tx_batch_manager.clone(); // channel to send batched messages
        let tx_round_sender = self.state.tx_round_manager.clone(); // channel to send round messages
        let peer_state = &self.state.peer_state.clone();
        let node_config = &self.node_config_for_dkg_with_options(peers, txn_prefix, false)?;
        let http_client = self.http_client.clone();

        let channels =
            register_comms_channel(tx_round_sender.clone(), &node_config.txn_prefix, round).await?;

        let peers = peers.all_addresses();

        for (dest_index, dest_addr) in peers.iter().enumerate() {
            if dest_addr != &self.state.addr {
                let data = data_to_send.iter().find(|d| d.0 == dest_index as u16);
                if data.is_some() {
                    let data = &data.expect_or_err("Empty data")?.1;
                    // debug!("Data to send to {}: {}", &dest_addr, &data);
                    node_share_push_direct(
                        tx_batch_sender,
                        node_config,
                        dest_addr,
                        dest_index,
                        round,
                        data.as_bytes().to_vec(),
                    )
                    .await?;
                }
            }
        }

        trace!("Waiting for {} items.", expected_items_from.len());
        node_share_await(
            node_config,
            tx_round_sender,
            Some(channels),
            round,
            expected_items_from,
            self.state.tx_peer_reviewer_bridge.clone(),
        )
        .await
        .map_err(|e| unexpected_err(e, Some("Error while waiting for round data".into())))
    }

    #[instrument(skip_all)]
    pub fn get_peer_id(&self, next_peers: &Vec<PeerSocketAddress>) -> Result<usize> {
        match next_peers
            .iter()
            .position(|peer| peer.address == self.state.addr)
        {
            Some(index) => Ok(index + 1),
            None => {
                error!(
                    "Peer address not found in the contract: {:?}",
                    &self.state.addr
                );
                Err(unexpected_err(
                    "Peer address not found in the contract",
                    None,
                ))
            }
        }
    }

    pub fn node_config_for_dkg(
        &self,
        peers: &Vec<PeerSocketAddress>,
        txn_prefix: &str,
    ) -> Result<NodeConfig> {
        self.node_config_for_dkg_with_options(peers, txn_prefix, true)
    }

    fn node_config_for_dkg_with_options(
        &self,
        peers: &Vec<PeerSocketAddress>,
        txn_prefix: &str,
        is_one_based: bool,
    ) -> Result<NodeConfig> {
        let addr = &self.addr();

        trace!(
            "Preparing configuration for node {}, connected to {} peers {:?}.",
            addr,
            &peers.len(),
            &peers
        );

        let total_shares = peers.len() as u16;
        let threshold = crate::utils::consensus::get_threshold_count(total_shares as usize) as u16;

        let share_index = peers.index_of(addr)?;

        let share_index = match is_one_based {
            true => share_index as u16 + 1,
            false => share_index as u16,
        };

        let peer_indexes: Vec<u16> = match is_one_based {
            true => (1..(total_shares + 1)).collect(),
            false => (0..total_shares).collect(),
        };

        let node_config = NodeConfig {
            addr: addr.clone(),
            current_index: share_index,
            share_index,
            txn_prefix: txn_prefix.to_string(),
            peers: peers.all_addresses(),
            peer_indexes,
            lit_config: debug_ignore::DebugIgnore(self.lit_config()),
            threshold,
            total_shares,
        };

        Ok(node_config)
    }

    fn addr(&self) -> String {
        self.state.addr.clone()
    }

    pub fn generic_addr(&self) -> String {
        self.addr()
    }

    pub async fn peers(&self) -> Vec<String> {
        match self.state.peer_state.peer_socket_addresses().await {
            Ok(peers) => peers.all_addresses(),
            Err(e) => {
                error!("Error getting node socket addresses: {}", e);
                Vec::new()
            }
        }
    }

    pub async fn generic_peers(&self) -> Vec<PeerSocketAddress> {
        match self.state.peer_state.peer_socket_addresses().await {
            Ok(peers) => peers,
            Err(e) => {
                error!("Error getting node socket addresses: {}", e);
                Vec::new()
            }
        }
    }

    fn lit_config(&self) -> Arc<LitConfig> {
        self.state.lit_config.clone()
    }

    pub fn generic_lit_config(&self) -> Arc<LitConfig> {
        self.lit_config()
    }
}

impl<G: Group + GroupEncoding + Default> GennaroState<G> {}
