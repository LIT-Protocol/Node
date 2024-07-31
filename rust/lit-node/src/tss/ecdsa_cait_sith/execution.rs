use super::models::ProtocolTransactionParams;
use super::CsEcdsaState;
use crate::error::{unexpected_err, Result};
use crate::p2p_comms::CommsManager;
use crate::peers::peer_reviewer::{Issue, PeerComplaint};
use crate::peers::peer_state::models::{SimplePeer, SimplePeerExt};
#[cfg(feature = "rtmetrics")]
use crate::tasks::realtime_metrics::MetricsMessage::IncomingMessage;
use crate::tss::common::models::RoundsShareSet;
use crate::tss::common::peer_communication::PeerCommunicationChecker;
use cait_sith::protocol::{Action, Participant, Protocol};
use lit_core::error::Unexpected;
use std::time::{Duration, Instant, SystemTime};
use tokio::time::MissedTickBehavior;
use tracing::{debug, info_span, instrument, Instrument};

impl CsEcdsaState {
    #[doc = "Runs the protocol"]
    #[instrument(skip(protocol, self), fields(txn_prefix))]
    pub async fn execute_protocol<T>(
        &self,
        protocol_name: &str,
        params: &ProtocolTransactionParams,
        mut protocol: impl Protocol<Output = T>,
    ) -> Result<(u16, T)> {
        let (txn_prefix, self_participant, participants, peers, threshold) = params.clone().into();

        info!(
            "Protocol {} -  {} for {:?} with participants {:?}, threshold: {:?}, peers {}",
            protocol_name,
            txn_prefix,
            &self_participant,
            &participants,
            threshold,
            peers.debug_addresses()
        );

        let self_index: u32 = self_participant.into();
        let self_peer = peers.peer_at_protocol_index(self_index as u16)?;
        let cm = CommsManager::new(&self.state, 0, &txn_prefix, "CS").await?;
        let tx_metrics = self.state.tx_metrics_manager.clone(); // channel to send metrics
        let tx_timeout_update = self
            .set_timeout(cm.get_timeout(), cm.channels.tx.clone())
            .await;

        let start_time = std::time::Instant::now();

        let participants_without_self = participants
            .iter()
            .filter(|p| *p != &self_participant)
            .cloned()
            .collect::<Vec<Participant>>();
        let mut peer_communication_checker =
            PeerCommunicationChecker::new(&participants_without_self);

        // we're going to skip node_share_await() calls and just create a channel directly and poll it as required.
        loop {
            let action = info_span!("protocol.poke", txn_prefix).in_scope(|| {
                protocol
                    .poke()
                    .map_err(|e| unexpected_err(e, Some("Error poking protocol".to_owned())))
            })?;
            match action {
                Action::Wait => {
                    let msg = cm
                        .channels
                        .rx
                        .recv_async()
                        .instrument(info_span!("execute_protocol.channels.rx.recv_async"))
                        .await
                        .expect("error receiving message from channel");

                    if msg.key == "abort" {
                        return self.do_abort(
                            protocol_name,
                            &txn_prefix,
                            peers,
                            peer_communication_checker,
                        );
                    }

                    let _ = tx_timeout_update.send_async(true).await;
                    #[cfg(feature = "rtmetrics")]
                    {
                        if let Err(e) = tx_metrics.send_async(IncomingMessage(msg.from_index)).await
                        {
                            error!("Error sending metrics message: {:?}", e);
                        };
                    };

                    let data = msg.value.clone();

                    let protocol_index = peers
                        .peer_at_share_index(msg.from_index)?
                        .protocol_index
                        .expect("Failed to get protocol index.");

                    let peer_participant = Participant::from(protocol_index as u32);

                    info_span!("protocol.message", txn_prefix).in_scope(|| {
                        protocol.message(peer_participant, data);
                    });

                    peer_communication_checker.mark_peer_as_communicated_with(&peer_participant);
                }

                // Broadcast to all nodes
                Action::SendMany(data) => {
                    cm.broadcast_bytes(data.clone()).await?;
                }

                // send direct to a node
                Action::SendPrivate(to, data) => {
                    let dest_peer: SimplePeer = peers
                        .peer_at_protocol_index(u32::from(to) as u16)
                        .expect_or_err("SendPrivatE: Failed to find peer at protocol index.")?;
                    cm.send_bytes_direct(&dest_peer, data.clone()).await?;
                }

                // Return the result
                Action::Return(r) => {
                    debug!(
                        "Protocol {}: {} complete in {:?}. ",
                        protocol_name,
                        txn_prefix,
                        start_time.elapsed(),
                    );
                    return Ok((self_index as u16, r));
                }
            }
        }
    }

    pub fn do_abort<T>(
        &self,
        protocol_name: &str,
        txn_prefix: &str,
        peers: Vec<SimplePeer>,
        peer_communication_checker: PeerCommunicationChecker<Participant>,
    ) -> Result<(u16, T)> {
        let msg = format!(
            "Received abort from channel for protocol {}, txn_prefix: {} at {:?}",
            protocol_name,
            txn_prefix,
            SystemTime::now()
        );
        error!("{}", &msg);

        // For all peers that have not communicated with this node yet, complain about them.
        let tss_state_clone = self.state.clone();
        let err_msg_clone = msg.clone();
        let txn_id = txn_prefix.to_string();
        let addr = self.state.addr.clone();
        let tx_pr = self.state.peer_state.complaint_channel.clone();
        tokio::spawn(async move {
            for participant in peer_communication_checker.peers_not_communicated_with_self_yet() {
                let protocol_index: u32 = (*participant).into();
                match peers.peer_at_protocol_index(protocol_index as u16) {
                    Ok(peer) => {
                        let complaint = PeerComplaint {
                            complainer: addr.clone(),
                            issue: Issue::NonParticipation,
                            peer_node_staker_address: peer.staker_address,
                        };
                        if let Err(e) = tx_pr.send_async(complaint).await {
                            debug!("Error sending complaint to PeerReviewer worker: {:?}", e);
                            continue;
                        }
                    }
                    Err(e) => continue, // there error was thrown and logged in a lower level function
                };
            }
        });
        Err(unexpected_err(msg, None))
    }

    pub async fn set_timeout(
        &self,
        timeout: u64,
        tx_channel: flume::Sender<RoundsShareSet>,
    ) -> flume::Sender<bool> {
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
    pub async fn protocol_params(&self, txn_prefix: &str) -> Result<ProtocolTransactionParams> {
        self.protocol_params_with_options(txn_prefix, None, None)
            .await
    }

    #[instrument(skip(self))]
    pub async fn protocol_params_with_options(
        &self,
        txn_prefix: &str,
        peers: Option<Vec<SimplePeer>>,
        alternate_threshold: Option<u16>,
    ) -> Result<ProtocolTransactionParams> {
        // for Cait-sith, the peer ids are the same as the share indices
        let mut all_peers = self.state.peer_state.peers().await?;
        let mut peers = match peers {
            Some(p) => p,
            None => all_peers.clone(),
        };

        // generate sequential protocol indices for peers used in this protocol
        let mut participants: Vec<Participant> = Vec::with_capacity(peers.len());
        for (index, peer) in peers.iter_mut().enumerate() {
            let protocol_index = peer.share_index;
            participants.push(Participant::from(protocol_index as u32));
            peer.protocol_index = Some(protocol_index);
            all_peers.set_protocol_index_for_share_index(peer.share_index, protocol_index);
        }

        // a bit of a validity check ;-)
        let self_peer = peers.peer_at_address(&self.state.addr)?;
        let self_participant = Participant::from(self_peer.get_protocol_index()? as u32);

        let threshold = match alternate_threshold {
            Some(t) => t,
            None => all_peers.get_threshold_count(),
        };

        debug!(
            "Self participant: {:?} / Participants {:?}",
            &self_participant, &participants
        );

        Ok(ProtocolTransactionParams {
            txn_prefix: txn_prefix.to_string(),
            self_participant,
            participants,
            peers: all_peers,
            bt_id: None,
            bt_participants: None,
            threshold,
        })
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
