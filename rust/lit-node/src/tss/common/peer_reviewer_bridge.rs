// use crate::peers::peer_reviewer::Issue;
use crate::peers::PeerState;
use crate::peers::{peer_reviewer::PeerComplaint, peer_state::models::PeerSocketAddress};

use moka::future::Cache;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Message {
    RegisterPeers(RegisterPeersMessage),
    SendComplaint(SendComplaintMessage),
}

#[derive(Debug)]
pub struct RegisterPeersMessage {
    pub txn_id: String,
    pub peers: Vec<PeerSocketAddress>,
}

#[derive(Debug)]
pub struct SendComplaintMessage {
    pub txn_id: String,
    pub index_of_participant_to_complain: usize,
    pub error: String,
}

/// This worker bridges between TSS execution contexts and the `PeerReviewer` worker.
/// The reason this bridge exists, is because the TSS execution contexts lack peer information,
/// and we rely on the registration of peer information before the TSS protocols start
/// within this worker, in order to map from low-level participant IDs to peer information that
/// we can use to send complaints to the `PeerReviewer` worker.
pub async fn peer_reviewer_bridge_worker(
    mut quit_rx: mpsc::Receiver<bool>,
    rx_peer_reviewer_bridge: flume::Receiver<Message>,
    tx_pr: flume::Sender<PeerComplaint>,
    peer_state: Arc<PeerState>,
) {
    info!("Starting: tasks::peer_reviewer_bridge_worker");

    let store = Cache::builder().max_capacity(10_000).build();

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
                break;
            }

            msg = rx_peer_reviewer_bridge.recv_async() => {
                let msg = match msg {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!("Error receiving from rx_peer_reviewer_bridge: {:?}", e);
                        continue;
                    }
                };

                match msg {
                    Message::RegisterPeers(register_peers_message) => {
                        debug!("Received RegisterPeers message: {:?}", register_peers_message);
                        let peers = register_peers_message.peers;
                        let txn_id = register_peers_message.txn_id;
                        store.insert(txn_id, peers).await;
                    }
                    Message::SendComplaint(send_complaint_message) => {
                        debug!("Received SendComplaint message: {:?}", send_complaint_message);
                        // Get the peers that were registered for this transaction
                        let peers = match store.get(&send_complaint_message.txn_id) {
                            Some(peers) => peers,
                            None => {
                                debug!("No peers registered for txn_id: {:?}", send_complaint_message.txn_id);
                                continue;
                            }
                        };

                        // Then, get the peer at the index specified
                        let peer = match peers.get(send_complaint_message.index_of_participant_to_complain) {
                            Some(peer) => peer,
                            None => {
                                debug!("No peer registered at index: {:?}", send_complaint_message.index_of_participant_to_complain);
                                continue;
                            }
                        };

                        // Try to get the staker address
                        let peer_staker_address_to_complain = match peer_state.get_peer_staker_address_for_complain(&peer.address).await {
                            Ok(peer_staker_address_to_complain) => peer_staker_address_to_complain,
                            Err(e) => {
                                debug!("Error getting peer staker address for complain: {:?}", e);
                                continue;
                            }
                        };

                        // FIXME - commented out due to issues with erroneous kicking that we think might be due to failed DKG attempts for which nodes should not be penalized
                        // Finally, send the complaint to the PeerReviewer worker
                        // let complaint = PeerComplaint {
                        //     complainer: peer_state.addr.clone(),
                        //     issue: Issue::NonParticipation,
                        //     peer_node_staker_address: peer_staker_address_to_complain
                        // };
                        // // if let Err(e) = tx_pr.send_async(complaint).await {
                        // //     debug!("Error sending complaint to PeerReviewer worker: {:?}", e);
                        // //     continue;
                        // // }
                    }
                }
            }
        }
    }

    info!("Stopped: tasks::peer_reviewer_bridge_worker");
}
