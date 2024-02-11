use flume::Sender;
use lit_core::error::Unexpected;

use crate::peers::peer_state::models::PeerSocketAddress;
use crate::tss::common::comms::channels::{deregister_comms_channel, register_comms_channel};
use crate::tss::common::models::{RoundCommsChannel, RoundData};
use crate::tss::common::peer_communication::PeerCommunicationChecker;
use crate::tss::common::peer_reviewer_bridge::Message;
use crate::tss::common::web::models::NodeConfig;
use crate::{
    config::LitNodeConfig,
    error::{unexpected_err, Result},
    tss::common::models::RoundsShareSet,
};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tracing::{info_span, instrument, Instrument};

#[instrument(skip_all, fields(txn_prefix = node_config.txn_prefix))]
pub async fn node_share_await(
    node_config: &NodeConfig,
    tx_round_sender: Arc<Sender<RoundData>>,
    channels: Option<RoundCommsChannel>,
    round: &str,
    expected_items_from: &Vec<&PeerSocketAddress>,
    tx_peer_reviewer_bridge: Arc<Sender<Message>>,
) -> Result<Vec<(u16, Vec<u8>)>> {
    let start = Instant::now();

    let channels = match channels {
        Some(channels) => channels,
        None => register_comms_channel(tx_round_sender.clone(), &node_config.txn_prefix, round)
            .await
            .map_err(|e| unexpected_err(e, Some("Error registering comms channel".into())))?,
    };

    trace!(
        "Node: {} node_share_await() round {}",
        &node_config.current_index,
        &round
    );
    let peers = &node_config.peers;
    let mut recvd_ans: Vec<(u16, Vec<u8>)> = Vec::new();

    // Intentionally keeping PeerCommunicationChecker simple without doing the job of `recvd_ans`.
    let mut peer_communication_checker = PeerCommunicationChecker::new(
        &expected_items_from
            .iter()
            .map(|p| p.share_index)
            .collect::<Vec<u32>>(),
    );

    // spawn a task to send abort messages after a timeout
    let (from_index, channel_id) = (node_config.current_index, node_config.txn_prefix.clone());
    let timeout = node_config
        .lit_config
        .ecdsa_round_timeout()
        .expect_or_err("Could not get ECDSA round timeout")? as u64;
    let timeout_task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(timeout)).await;
        let (key, value, retry) = ("abort".to_string(), "".to_string().as_bytes().to_vec(), 0);
        let created = SystemTime::now();
        let round_share_set = RoundsShareSet {
            key,
            value,
            from_index,
            channel_id,
            retry,
            created,
        };
        let sent = channels.tx.send(round_share_set);
    });

    'waiting_loop: while let Ok(msg) = channels
        .rx
        .recv_async()
        .instrument(info_span!("node_share_await.channels.rx.recv_async"))
        .await
    {
        if msg.key == "abort" {
            let recvd_ans_idx: Vec<u16> = recvd_ans.iter().map(|x| x.0).collect();
            deregister_comms_channel(tx_round_sender.clone(), &node_config.txn_prefix, round).await;
            error!(
                "Timeout waiting for all nodes to respond to round {} from node #{}.  We received {} of {} responses from : {:?}",
                 round, &node_config.current_index, recvd_ans.len(), expected_items_from.len(), recvd_ans_idx
            );

            // For all peers that have not communicated with this node yet, complain about them.
            let txn_id = node_config.txn_prefix.clone();
            for peer in peer_communication_checker.peers_not_communicated_with_self_yet() {
                if let Err(e) = tx_peer_reviewer_bridge
                    .send_async(Message::SendComplaint(
                        crate::tss::common::peer_reviewer_bridge::SendComplaintMessage {
                            txn_id: txn_id.clone(),
                            index_of_participant_to_complain: peer.to_owned() as usize,
                            error: format!("Peer {:?} did not communicate with self: ", peer),
                        },
                    ))
                    .await
                {
                    error!(
                        "Error sending SendComplaint message to peer_reviewer_bridge: {:?}",
                        e
                    );
                }
            }

            return Err(unexpected_err(
                format!(
                       "Timeout waiting for all nodes to respond to round {} from node #{}.  We received {} of {} responses from : {:?}",
                 round, &node_config.current_index, recvd_ans.len(), expected_items_from.len(), recvd_ans_idx
                ),
                None,
            ));
        }
        trace!(
            "Node: {} node_share_await() round {} received from {}",
            &node_config.current_index,
            &round,
            &msg.from_index
        );
        recvd_ans.push((msg.from_index, msg.value.clone()));

        peer_communication_checker.mark_peer_as_communicated_with(&u32::from(msg.from_index));

        if recvd_ans.len() == expected_items_from.len() {
            break 'waiting_loop;
        }
    }

    // kill the task
    timeout_task.abort();

    deregister_comms_channel(tx_round_sender.clone(), &node_config.txn_prefix, round).await;

    recvd_ans.sort_by(|a, b| a.0.cmp(&b.0));

    // debug!("Waiting took: {:?}", start.elapsed().as_millis());
    Ok(recvd_ans)
}
