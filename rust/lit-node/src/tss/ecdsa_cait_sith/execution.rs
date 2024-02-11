use std::time::{Duration, Instant, SystemTime};

use super::CsEcdsaState;
use crate::error::{unexpected_err, Result};
use crate::peers::peer_state::models::PeerSocketAddress;
#[cfg(feature = "rtmetrics")]
use crate::tasks::realtime_metrics::MetricsMessage::{IncomingMessage, OutgoingMessage};
use crate::tss::common::comms::channels::{deregister_comms_channel, register_comms_channel};
use crate::tss::common::comms::push::node_share_push_direct;
use crate::tss::common::models::RoundsShareSet;
use crate::tss::common::peer_communication::PeerCommunicationChecker;
use crate::tss::common::peer_reviewer_bridge::{Message, RegisterPeersMessage};
use crate::tss::common::traits::dkg::DkgSupport;
use crate::tss::common::web::models::NodeConfig;
use cait_sith::protocol::{Action, Participant, Protocol};
use tokio::time::MissedTickBehavior;
use tracing::{debug, info_span, instrument, Instrument};

impl CsEcdsaState {
    #[doc = "Runs the protocol"]
    #[instrument(skip(protocol, self), fields(txn_prefix = node_config.txn_prefix))]
    pub async fn execute_protocol<T>(
        &self,
        protocol_name: &str,
        participants: &Vec<Participant>,
        self_participant: &Participant,
        mut protocol: impl Protocol<Output = T>,
        node_config: &NodeConfig,
        all_participants: &Vec<Participant>,
    ) -> Result<(u16, T)> {
        info!(
            "Executing protocol {} for {} at {:?}",
            protocol_name,
            node_config.txn_prefix,
            SystemTime::now()
        );
        let tx_batch_sender = self.state.tx_batch_manager.clone(); // channel to send batched messages
        let tx_round_sender = self.state.tx_round_manager.clone(); // channel to receive round messages
        let peer_state = self.state.peer_state.clone();
        let tx_metrics = self.state.tx_metrics_manager.clone(); // channel to send metrics

        // we're going to skip node_share_await() calls and just create a channel directly and poll it as required.
        let round = "CS";
        let channels =
            register_comms_channel(tx_round_sender.clone(), &node_config.txn_prefix, round).await?;

        let tx_timeout_update = self.set_timeout(node_config, channels.tx.clone()).await;

        let start_time = std::time::Instant::now();

        let participants_without_self = participants
            .iter()
            .filter(|p| *p != self_participant)
            .cloned()
            .collect::<Vec<Participant>>();
        let mut peer_communication_checker =
            PeerCommunicationChecker::new(&participants_without_self);

        loop {
            let action = info_span!("protocol.poke", txn_prefix = node_config.txn_prefix)
                .in_scope(|| {
                    protocol
                        .poke()
                        .map_err(|e| unexpected_err(e, Some("Error poking protocol".to_owned())))
                })?;
            match action {
                Action::Wait => {
                    let msg = channels
                        .rx
                        .recv_async()
                        .instrument(info_span!("execute_protocol.channels.rx.recv_async"))
                        .await;
                    let msg = msg.expect("error loading message from channel");

                    if msg.key == "abort" {
                        let msg =
                            format!(
                            "Received abort from channel for protocol {}, txn_prefix: {} at {:?}",
                            protocol_name, node_config.txn_prefix, SystemTime::now()
                        );
                        error!("{}", &msg);

                        // For all peers that have not communicated with this node yet, complain about them.
                        let tss_state_clone = self.state.clone();
                        let err_msg_clone = msg.clone();
                        let txn_id = node_config.txn_prefix.clone();
                        tokio::spawn(async move {
                            for peer in
                                peer_communication_checker.peers_not_communicated_with_self_yet()
                            {
                                if let Err(e) = tss_state_clone.tx_peer_reviewer_bridge.send_async(
                                    Message::SendComplaint(
                                        crate::tss::common::peer_reviewer_bridge::SendComplaintMessage {
                                            txn_id: txn_id.clone(),
                                            index_of_participant_to_complain: u32::from_le_bytes(peer.bytes()) as usize,
                                            error: format!("Peer {:?} did not communicate with self: ", peer),
                                        },
                                    ),
                                ).await {
                                    error!("Error sending SendComplaint message to peer_reviewer_bridge: {:?}", e);
                                }
                            }
                        });

                        return Err(unexpected_err(msg, None));
                    }

                    let _ = tx_timeout_update.send_async(true).await;

                    #[cfg(feature = "rtmetrics")]
                    if let Err(e) = tx_metrics
                        .send_async(IncomingMessage(msg.from_index as u16))
                        .await
                    {
                        error!("Error sending metrics message: {:?}", e);
                    };

                    let data = msg.value.clone();
                    let peer_participant = Participant::from(msg.from_index as u32);
                    info_span!("protocol.message", txn_prefix = node_config.txn_prefix).in_scope(
                        || {
                            protocol.message(peer_participant, data);
                        },
                    );

                    peer_communication_checker.mark_peer_as_communicated_with(&peer_participant);
                }
                Action::SendMany(m) => {
                    let data = m;
                    for (index, j) in participants.iter().enumerate() {
                        if j == self_participant {
                            continue;
                        }
                        let dest_index = match all_participants.iter().position(|&r| r == *j) {
                            Some(i) => i,
                            None => {
                                return Err(unexpected_err(
                                    format!("Unable to find index for participant {:?}", j),
                                    None,
                                ))
                            }
                        };

                        let dest_addr = &node_config.peers[dest_index].clone();
                        let http_client = self.http_client.clone();
                        #[cfg(feature = "rtmetrics")]
                        if let Err(e) = tx_metrics
                            .send_async(OutgoingMessage(dest_index as u16))
                            .await
                        {
                            error!("Error sending metrics message: {:?}", e);
                        };

                        let result = node_share_push_direct(
                            &tx_batch_sender,
                            node_config,
                            dest_addr,
                            dest_index,
                            round,
                            data.clone(),
                        )
                        .await?;
                    }
                }
                Action::SendPrivate(to, m) => {
                    let data = m;
                    let dest_index = match all_participants.iter().position(|&r| r == to) {
                        Some(i) => i,
                        None => {
                            return Err(unexpected_err(
                                format!("Unable to find index for participant {:?}", to),
                                None,
                            ))
                        }
                    };

                    let dest_addr = &node_config.peers[dest_index].clone();
                    let http_client = self.http_client.clone();
                    #[cfg(feature = "rtmetrics")]
                    if let Err(e) = tx_metrics
                        .send_async(OutgoingMessage(dest_index as u16))
                        .await
                    {
                        error!("Error sending metrics message: {:?}", e);
                    };

                    let result = node_share_push_direct(
                        &tx_batch_sender,
                        node_config,
                        dest_addr,
                        dest_index,
                        round,
                        data.clone(),
                    )
                    .await?;
                }
                Action::Return(r) => {
                    info!(
                        "Protocol {}: {} complete in {:?}. ",
                        protocol_name,
                        node_config.txn_prefix,
                        start_time.elapsed(),
                    );

                    deregister_comms_channel(
                        tx_round_sender.clone(),
                        &node_config.txn_prefix,
                        round,
                    )
                    .await;
                    return Ok((node_config.current_index, r));
                }
            }
        }
    }

    async fn set_timeout(
        &self,
        node_config: &NodeConfig,
        tx_channel: flume::Sender<RoundsShareSet>,
    ) -> flume::Sender<bool> {
        use crate::config::LitNodeConfig;

        let timeout: u64 = node_config
            .lit_config
            .ecdsa_round_timeout()
            .unwrap_or(crate::config::CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT)
            as u64;

        trace!("Timeout set to {} ms", timeout);
        let mut current_timeout = Instant::now() + Duration::from_millis(timeout);

        let (tx, rx): (flume::Sender<bool>, flume::Receiver<bool>) = flume::unbounded();

        let mut interval = tokio::time::interval(Duration::from_millis(1000));
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    biased;
                    msg = rx.recv_async() => {
                        if let Err(e) = msg {
                            return; // channel dropped, this operation is done
                        } else {
                            current_timeout = Instant::now() + Duration::from_millis(timeout);
                        }
                    },
                    _= interval.tick() => {
                        if Instant::now() > current_timeout {
                            let _send = tx_channel
                                .send_async(RoundsShareSet {
                                    key: "abort".to_string(),
                                    value: "".to_string().as_bytes().to_vec(),
                                    from_index: 0,
                                    channel_id: "".to_string(),
                                    retry: 0,
                                    created: SystemTime::now(),
                                }).await;
                        }
                    }
                }
            }
        });

        tx.clone()
    }

    #[instrument(skip(self))]
    #[allow(clippy::type_complexity)]
    pub async fn protocol_params(
        &self,
        txn_prefix: &str,
    ) -> Result<(
        String,
        Participant,
        Vec<Participant>,
        Vec<Participant>,
        NodeConfig,
        usize,
    )> {
        self.protocol_params_with_peers(None, txn_prefix).await
    }

    #[instrument(skip(self))]
    #[allow(clippy::type_complexity)]
    pub async fn protocol_params_with_peers(
        &self,
        peers: Option<Vec<PeerSocketAddress>>,
        txn_prefix: &str,
    ) -> Result<(
        String,
        Participant,
        Vec<Participant>,
        Vec<Participant>,
        NodeConfig,
        usize,
    )> {
        let result = self
            .protocol_params_with_options(txn_prefix, peers, None)
            .await?;
        Ok(result)
    }

    #[instrument(skip(self))]
    #[allow(clippy::type_complexity)]
    pub async fn protocol_params_with_options(
        &self,
        txn_prefix: &str,
        peers: Option<Vec<PeerSocketAddress>>,
        threshold: Option<usize>,
    ) -> Result<(
        String,
        Participant,
        Vec<Participant>,
        Vec<Participant>,
        NodeConfig,
        usize,
    )> {
        let all_peers = self.peer_socket_addresses().await;
        let peers = match peers {
            Some(p) => p,
            None => all_peers.clone(),
        };
        // let all_peer_indexes: Vec<u16> = (0..all_peers.len() as u16).collect();

        let node_config = self.node_config_for_dkg_with_options(&all_peers, txn_prefix, false)?;

        trace!("Node config: {:?}", &node_config);

        let ordinal_position = node_config.current_index; // the "current index doesn't work for cait-sith"

        let all_participants: Vec<Participant> = all_peers
            .iter()
            .map(|p| Participant::from(p.share_index))
            .collect();
        let participants: Vec<Participant> = peers
            .iter()
            .map(|p| Participant::from(p.share_index))
            .collect();

        // Register these peers (NOT all peers) with the Peer Reviewer Bridge
        if let Err(e) = self
            .state
            .tx_peer_reviewer_bridge
            .send_async(Message::RegisterPeers(RegisterPeersMessage {
                txn_id: txn_prefix.to_string(),
                peers: peers.clone(),
            }))
            .await
        {
            error!(
                "Error sending RegisterPeers message to peer_reviewer_bridge: {:?}",
                e
            );
        }

        let self_participant = Participant::from(ordinal_position as u32);
        // let self_participant = Participant::from(all_peers.index_of(&self.addr()));

        debug!("Participants: {:?}", &participants);
        debug!("Self participant: {:?}", &self_participant);

        let threshold = match threshold {
            Some(t) => t,
            None => node_config.threshold as usize,
        };

        Ok((
            txn_prefix.to_string(),
            self_participant,
            participants,
            all_participants,
            node_config,
            threshold,
        ))
    }

    #[instrument]
    pub fn generate_participants(peer_set_size: u16) -> Vec<Participant> {
        let mut participants = Vec::with_capacity(peer_set_size as usize);
        for i in 0..peer_set_size {
            participants.push(Participant::from(i as u32));
        }

        participants
    }
}
