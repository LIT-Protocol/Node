use crate::p2p_comms::comms::channels::{deregister_comms_channel, register_comms_channel};
use crate::peers::peer_reviewer::{Issue, PeerComplaint};
use crate::peers::peer_state::models::{SimplePeer, SimplePeerExt};
use crate::tss::common::models::{NodeWaitParams, RoundData};
use crate::tss::common::peer_communication::PeerCommunicationChecker;
use crate::{
    error::{unexpected_err, Result},
    tss::common::models::RoundsShareSet,
};
use flume::Sender;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tracing::{info_span, instrument, Instrument};

#[instrument(skip_all, fields(txn_prefix))]
pub async fn node_share_await(
    params: NodeWaitParams,
    tx_round_sender: Arc<Sender<RoundData>>,
    peers: &Vec<SimplePeer>,
    expected_items_from: &Vec<SimplePeer>,
) -> Result<Vec<(u16, Vec<u8>)>> {
    let start = Instant::now();

    let channels = match params.channels {
        Some(channels) => channels,
        None => register_comms_channel(tx_round_sender.clone(), &params.txn_prefix, &params.round)
            .await
            .map_err(|e| unexpected_err(e, Some("Error registering comms channel".into())))?,
    };

    let mut remaining_share_indices = expected_items_from.share_indices();

    trace!(
        "Node: share_index: {} node_share_await() round {}",
        &params.share_index,
        &params.round
    );
    let mut recvd_ans: Vec<(u16, Vec<u8>)> = Vec::new();

    // Intentionally keeping PeerCommunicationChecker simple without doing the job of `recvd_ans`.
    let mut peer_communication_checker = PeerCommunicationChecker::new(
        &expected_items_from
            .iter()
            .map(|p| p.share_index)
            .collect::<Vec<u16>>(),
    );

    // spawn a task to send abort messages after a timeout
    let (from_share_index, channel_id, timeout) = (
        params.share_index,
        params.txn_prefix.clone(),
        params.timeout,
    );

    // set a timeout for the round
    let timeout_task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(timeout)).await;
        let sent = channels.tx.send(RoundsShareSet {
            key: "abort".to_string(),
            value: "".to_string().as_bytes().to_vec(),
            from_index: from_share_index,
            channel_id,
            retry: 0,
            created: SystemTime::now(),
        });
    });

    'waiting_loop: while let Ok(msg) = channels
        .rx
        .recv_async()
        .instrument(info_span!("node_share_await.channels.rx.recv_async"))
        .await
    {
        if msg.key == "abort" {
            let recvd_ans_idx: Vec<u16> = recvd_ans.iter().map(|x| x.0).collect();
            deregister_comms_channel(tx_round_sender.clone(), &params.txn_prefix, &params.round)
                .await;
            error!(
                "Timeout waiting for all nodes to respond to round {} from node #{}.  We received {} of {} responses from : {:?}",
                params.round, params.share_index, recvd_ans.len(), expected_items_from.len(), recvd_ans_idx
            );

            // For all peers that have not communicated with this node yet, complain about them.
            let txn_id = params.txn_prefix.clone();
            match peers.peer_at_share_index(params.share_index) {
                Ok(complainer) => {
                    for peer in peer_communication_checker.peers_not_communicated_with_self_yet() {
                        match peers.peer_at_share_index(*peer) {
                            Ok(peer) => {
                                let complaint = PeerComplaint {
                                    complainer: complainer.socket_address.clone(),
                                    issue: Issue::NonParticipation,
                                    peer_node_staker_address: peer.staker_address,
                                };
                                if let Err(e) = params.tx_pr.send_async(complaint).await {
                                    debug!(
                                        "Error sending complaint to PeerReviewer worker: {:?}",
                                        e
                                    );
                                    continue;
                                }
                            }
                            Err(e) => {
                                debug!("Error getting peer at share index: {:?}", e);
                                continue;
                            }
                        }
                    }
                }
                Err(e) => {
                    debug!("Error getting peer at share index: {:?}", e);
                    continue;
                }
            }

            return Err(unexpected_err(
                format!(
                       "Timeout waiting for all nodes to respond to round {} from node #{}.  We received {} of {} responses from : {:?}",
                    params.round, &params.share_index, recvd_ans.len(), expected_items_from.len(), recvd_ans_idx
                ),
                None,
            ));
        }
        trace!(
            "Node: {} node_share_await() round {} received from {}",
            params.share_index,
            &params.round,
            &msg.from_index
        );

        match remaining_share_indices
            .iter()
            .position(|&r| r == msg.from_index)
        {
            Some(idx) => {
                recvd_ans.push((msg.from_index, msg.value.clone()));
                remaining_share_indices.remove(idx);
            }
            None => {
                error!(
                    "Index {} received a message from a protocol index that was not expected: {}",
                    params.share_index, msg.from_index
                );
            }
        }

        peer_communication_checker.mark_peer_as_communicated_with(&msg.from_index);

        if remaining_share_indices.is_empty() {
            break 'waiting_loop;
        }
        // if recvd_ans.len() == expected_items_from.len() {
        //     break 'waiting_loop;
        // }

        // optionally exit early.
        if let Some(exit_on_qty_recvd) = params.exit_on_qty_recvd {
            if recvd_ans.len() >= exit_on_qty_recvd as usize {
                break 'waiting_loop;
            };
        };
    }

    // kill the task
    timeout_task.abort();
    deregister_comms_channel(tx_round_sender.clone(), &params.txn_prefix, &params.round).await;
    recvd_ans.sort_by(|a, b| a.0.cmp(&b.0));
    // debug!("Waiting took: {:?}", start.elapsed().as_millis());
    Ok(recvd_ans)
}
