pub mod comms;
pub mod web;

use self::comms::channels::{deregister_comms_channel, register_comms_channel};
use self::comms::push::node_share_push_direct;
use self::comms::wait::node_share_await;
use crate::config::LitNodeConfig;
use crate::error::unexpected_err;
use flume::Sender;
use std::sync::Arc;

use crate::tss::common::models::RoundCommsChannel;
use crate::{
    peers::peer_state::models::{SimplePeer, SimplePeerExt},
    tss::common::{
        models::{NodeTransmissionDetails, NodeWaitParams, RoundData},
        tss_state::TssState,
    },
};
use lit_core::error::Result;

#[cfg(feature = "rtmetrics")]
use crate::tasks::realtime_metrics::{MetricsMessage, MetricsMessage::OutgoingMessage};

pub struct CommsManager {
    pub channels: RoundCommsChannel,
    tx_batch_manager: Arc<Sender<NodeTransmissionDetails>>,
    tx_round_manager: Arc<Sender<RoundData>>,
    #[cfg(feature = "rtmetrics")]
    tx_metrics: Arc<Sender<MetricsMessage>>,
    peers: Vec<SimplePeer>,
    wait_params: NodeWaitParams,
    self_peer: SimplePeer,
    txn_prefix: String,
    round: String,
}

impl Drop for CommsManager {
    fn drop(&mut self) {
        // drop the channels

        let tx_round_manager = self.tx_round_manager.clone();
        let txn_prefix = self.txn_prefix.clone();
        let round = self.round.clone();
        tokio::spawn(async move {
            deregister_comms_channel(tx_round_manager, &txn_prefix, &round).await;
        });
    }
}
impl CommsManager {
    pub async fn new(
        state: &TssState,
        peer_id_offset: u16,
        txn_prefix: &str,
        round: &str,
    ) -> Result<Self> {
        let peers = state.peer_state.peers().await?;
        let mut peers = peers.active_peers();
        peers.set_all_protocol_indices(peer_id_offset);
        Self::new_with_peers(state, txn_prefix, &peers, round).await
    }

    pub async fn new_with_peers(
        state: &TssState,
        txn_prefix: &str,
        peers: &Vec<SimplePeer>,
        round: &str,
    ) -> Result<Self> {
        // channel setup -> we're going to set up a channel for each round.  We could do this for each DKG as well, but this is a bit easier to debug :-)
        let tx_batch_manager = state.tx_batch_manager.clone(); // channel to send batched messages
        let tx_round_manager = state.tx_round_manager.clone(); // channel to send round messages
        let tx_metrics = state.tx_metrics_manager.clone(); // channel to send metrics

        let timeout = state
            .peer_state
            .lit_config
            .ecdsa_round_timeout()
            .unwrap_or(10000) as u64;

        let channels = register_comms_channel(tx_round_manager.clone(), txn_prefix, round).await?;

        let addr = &state.addr;

        let self_peer = peers.peer_at_address(addr)?;

        let wait_params = NodeWaitParams {
            channels: Some(channels.clone()),
            tx_pr: state.peer_state.complaint_channel.clone(),
            txn_prefix: txn_prefix.to_string(),
            round: round.to_string(),
            timeout,
            share_index: self_peer.share_index,
            exit_on_qty_recvd: None,
        };

        let comms_manager = CommsManager {
            channels,
            tx_batch_manager,
            tx_round_manager,
            #[cfg(feature = "rtmetrics")]
            tx_metrics,
            peers: peers.clone(),
            wait_params,
            self_peer,
            txn_prefix: txn_prefix.to_string(),
            round: round.to_string(),
        };

        Ok(comms_manager)
    }

    // pub fn update_round(&mut self, round: &str) {
    //     self.round = round.to_string();
    // }

    pub async fn broadcast<B>(&self, data: B) -> Result<bool>
    where
        B: serde::Serialize,
    {
        let data = serde_json::to_string(&data)
            .map_err(|e| unexpected_err(e, Some("Error while serializing data".into())))?;
        let data = data.as_bytes().to_vec();
        self.broadcast_bytes(data).await
    }

    pub async fn broadcast_and_collect<B, C>(&self, data: B) -> Result<Vec<(u16, C)>>
    where
        B: serde::Serialize,
        C: serde::de::DeserializeOwned,
    {
        let expected_peers = self.peers.all_peers_except(&self.self_peer.socket_address);
        self.broadcast_and_collect_from::<B, C>(data, &expected_peers)
            .await
    }

    pub async fn broadcast_and_collect_from<B, C>(
        &self,
        data: B,
        expected_peers: &Vec<SimplePeer>,
    ) -> Result<Vec<(u16, C)>>
    where
        B: serde::Serialize,
        C: serde::de::DeserializeOwned,
    {
        let data = serde_json::to_string(&data)
            .map_err(|e| unexpected_err(e, Some("Error while serializing data".into())))?;
        let data = data.as_bytes().to_vec();
        let data = self.broadcast_bytes(data).await?;
        self.collect_from::<C>(expected_peers).await
    }

    pub async fn send_direct<B>(&self, dest_peer: &SimplePeer, data: B) -> Result<bool>
    where
        B: serde::Serialize,
    {
        let data = serde_json::to_string(&data)
            .map_err(|e| unexpected_err(e, Some("Error while serializing data".into())))?;
        let data = data.as_bytes().to_vec();
        self.send_bytes_direct(dest_peer, data).await
    }

    pub async fn collect<C>(&self) -> Result<Vec<(u16, C)>>
    where
        C: serde::de::DeserializeOwned,
    {
        let expected_peers = self.peers.all_peers_except(&self.self_peer.socket_address);
        self.collect_from::<C>(&expected_peers).await
    }

    pub async fn collect_from<C>(&self, expected_peers: &Vec<SimplePeer>) -> Result<Vec<(u16, C)>>
    where
        C: serde::de::DeserializeOwned,
    {
        let data = self.await_bytes_from(expected_peers, None).await?;
        let data = data
            .iter()
            .map(|(index, data)| {
                let data = std::str::from_utf8(data).unwrap_or("ERR");
                let data: C = serde_json::from_str(data).map_err(|e| {
                    unexpected_err(e, Some("Error while deserializing data".into()))
                })?;
                Ok((*index, data))
            })
            .collect::<Result<Vec<(u16, C)>>>()?;

        Ok(data)
    }

    pub async fn collect_from_earliest<C>(
        &self,
        expected_peers: &Vec<SimplePeer>,
        messages_to_collect: u16,
    ) -> Result<Vec<(u16, C)>>
    where
        C: serde::de::DeserializeOwned,
    {
        let mut wait_params = self.wait_params.clone();
        wait_params.exit_on_qty_recvd = Some(messages_to_collect);

        let data = self
            .await_bytes_from(expected_peers, Some(wait_params))
            .await?;
        let data = data
            .iter()
            .map(|(index, data)| {
                let data = std::str::from_utf8(data).unwrap_or("ERR");
                let data: C = serde_json::from_str(data).map_err(|e| {
                    unexpected_err(e, Some("Error while deserializing data".into()))
                })?;
                Ok((*index, data))
            })
            .collect::<Result<Vec<(u16, C)>>>()?;

        Ok(data)
    }

    pub async fn broadcast_bytes(&self, data: Vec<u8>) -> Result<bool> {
        // this isn't in a task because it's being placed on a channel.

        let broadcast_peers = self.peers.all_peers_except(&self.self_peer.socket_address);

        for dest_peer in &broadcast_peers {
            // only broadcast to participants that are part of this protocol run - ie, signing & real-time triples use a subset.
            if dest_peer.protocol_index.is_some() {
                let _ = node_share_push_direct(
                    &self.txn_prefix,
                    &self.tx_batch_manager,
                    &self.self_peer, //&self_addr,
                    dest_peer,
                    &self.round,
                    data.clone(),
                )
                .await?;
            }
        }

        Ok(true)
    }

    pub async fn send_bytes_direct(&self, dest_peer: &SimplePeer, data: Vec<u8>) -> Result<bool> {
        #[cfg(feature = "rtmetrics")]
        if let Err(e) = self
            .tx_metrics
            .send_async(OutgoingMessage(dest_peer.share_index))
            .await
        {
            // error!("Error sending metrics message: {:?}", e);
        };

        node_share_push_direct(
            &self.txn_prefix,
            &self.tx_batch_manager,
            &self.self_peer, //&
            dest_peer,
            &self.round,
            data,
        )
        .await
    }

    // pub async fn await_bytes(&self) -> Result<Vec<(u16, Vec<u8>)>> {
    //     let expected_peers = self.peers.all_peers_except(&self.self_peer.socket_address);
    //     self.await_bytes_from(&expected_peers).await
    // }

    pub async fn await_bytes_from(
        &self,
        expected_peers: &Vec<SimplePeer>,
        wait_params: Option<NodeWaitParams>,
    ) -> Result<Vec<(u16, Vec<u8>)>> {
        let wait_params = match wait_params {
            Some(params) => params,
            None => self.wait_params.clone(),
        };

        node_share_await(
            wait_params,
            self.tx_round_manager.clone(),
            &self.peers,
            expected_peers,
        )
        .await
        .map_err(|e| unexpected_err(e, Some("Error while waiting for incoming data".into())))
    }

    pub fn get_timeout(&self) -> u64 {
        self.wait_params.timeout
    }
}
