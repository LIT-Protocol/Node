use crate::peers::peer_state::models::SimplePeer;
use crate::peers::PeerState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::SystemTime;

pub type NodeIndex = u16;
pub type ActionName = String;
pub type TxnId = String;
pub type IsStart = bool;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MetricActionType {
    DKG,
    BeaverTriple,
    SignEcdsa,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetricAction {
    pub type_id: MetricActionType,
    pub txn_id: u64,
    pub is_start: bool,
    pub is_success: bool,
}

#[derive(Clone, Debug)]
pub enum MetricsMessage {
    OutgoingMessage(NodeIndex),
    IncomingMessage(NodeIndex),
    NewAction(MetricAction),
    TripleStatus(Vec<RealTimeTripleCounter>),
    Poll(flume::Sender<MetricPoll>),
    CheckLastPollPoint,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetricPoll {
    pub idx: u16,
    pub metrics: Vec<RealTimeCounter>,
    pub action: Vec<MetricAction>,
    pub peers: Vec<MiniPeer>,
    pub triple_count: Vec<RealTimeTripleCounter>,
    pub time: SystemTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiniPeer {
    pub idx: NodeIndex,
    pub kicked: bool,
}

impl From<&SimplePeer> for MiniPeer {
    fn from(peer: &SimplePeer) -> Self {
        MiniPeer {
            idx: peer.share_index,
            kicked: peer.kicked,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealTimeCounter {
    pub idx: u16,
    pub inmsg: u64,
    pub outmsg: u64,
    pub badmsg: u64,
    pub time: SystemTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealTimeTripleCounter {
    pub peer_group_id: u64,
    pub count: u64,
    pub peer_keys: Vec<u64>,
}

pub async fn real_time_metrics_worker(
    mut quit_rx: tokio::sync::mpsc::Receiver<bool>,
    metrics_rx: flume::Receiver<MetricsMessage>,
    peer_state: Arc<PeerState>,
) {
    info!("Starting: real_time_metrics-worker");

    let (mut validators, mut metrics, mut actions, mut triple_counter) =
        init_counters(&peer_state).await;

    loop {
        tokio::select! {
            biased;
            _ = quit_rx.recv() => {
                break;
            }

            msg = metrics_rx.recv_async() => {
                let msg = match msg  {
                    Ok(msg) => msg,
                    Err(_) => {
                        error!("rt-metrics: rx_metrics.recv_async().await is_err()");
                        continue;
                    }
                };

                match msg {
                    MetricsMessage::OutgoingMessage(node_index) => {
                        if node_index >= metrics.len() as u16 {
                            // error!("rt-metrics: node_index : {} >= metrics.len() : {}", node_index, metrics.len());
                            continue;
                        }
                        if metrics[node_index as usize].outmsg == 0 {
                            metrics[node_index as usize].time = SystemTime::now();
                        }
                        metrics[node_index as usize].outmsg += 1;
                    }

                    MetricsMessage::IncomingMessage(node_index) => {
                        if node_index >= metrics.len() as u16 {
                            // error!("rt-metrics: node_index : {} >= metrics.len() : {}", node_index, metrics.len());
                            continue;
                        }
                        if metrics[node_index as usize].inmsg == 0 {
                            metrics[node_index as usize].time = SystemTime::now();
                        }
                        metrics[node_index as usize].inmsg += 1;
                    }

                    MetricsMessage::NewAction(action_name) => {
                        actions.push(action_name);
                    }

                    MetricsMessage::TripleStatus(new_triple_counter) => {
                        triple_counter = new_triple_counter;
                    }

                    MetricsMessage::Poll(tx) => {
                        let poll = MetricPoll {
                            idx: peer_state.node_index_in_current_epoch().await.unwrap_or(0) as u16,
                            metrics: metrics.clone(),
                            action: actions.clone(),
                            triple_count: triple_counter.clone(),
                            peers: validators.iter().map(MiniPeer::from).collect(),
                            time: SystemTime::now()
                        };

                        tokio::spawn(async move {
                            let _ = tx.send_async(poll).await;
                        });

                        validators = match peer_state.peers().await {
                            Ok(validators) => validators,
                            Err(_) => {
                                error!("rt-metrics: peer_state.validators_in_current_epoch().await is_err()");
                                Vec::new()
                            }
                        };
                        metrics =  Vec::with_capacity(validators.len());
                        validators.iter().for_each(|v| metrics.push(RealTimeCounter { idx: v.share_index ,   inmsg: 0, outmsg: 0, badmsg: 0, time: SystemTime::now() }));

                        actions = Vec::new();
                        triple_counter = Vec::new();
                    }
                    // reset the status if we haven't received a poll in 60 seconds
                    MetricsMessage::CheckLastPollPoint => {

                        if metrics.is_empty() {
                            continue;
                        }

                        let mut time_since_last_poll = 0;
                        for metric in metrics.iter() {
                            let time_since_last_poll_for_node = match SystemTime::now().duration_since(metric.time) {
                                Ok(duration) => duration.as_secs(),
                                Err(_) => {
                                    error!("rt-metrics: SystemTime::now().duration_since(metric.timestamp) is_err()");
                                    0
                                }
                            };
                            if time_since_last_poll_for_node > time_since_last_poll {
                                time_since_last_poll = time_since_last_poll_for_node;
                            }
                        }

                        if time_since_last_poll > 60 {
                            validators = match peer_state.peers().await {
                                Ok(validators) => validators,
                                Err(_) => {
                                    error!("rt-metrics: peer_state.validators_in_current_epoch().await is_err()");
                                    Vec::new()
                                }
                            };
                            metrics =  Vec::with_capacity(validators.len());
                            validators.iter().for_each(|v| metrics.push(RealTimeCounter { idx: v.share_index ,   inmsg: 0, outmsg: 0,  badmsg: 0, time: SystemTime::now() }));

                            actions = Vec::new();
                        }
                    }
                }

            }

            _= async {} => {
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            }

        }
    }

    async fn init_counters(
        peer_state: &Arc<PeerState>,
    ) -> (
        Vec<SimplePeer>,
        Vec<RealTimeCounter>,
        Vec<MetricAction>,
        Vec<RealTimeTripleCounter>,
    ) {
        let validators = match peer_state.peers().await {
            Ok(validators) => validators,
            Err(_) => {
                error!("rt-metrics: peer_state.validators_in_current_epoch().await is_err()");
                Vec::new()
            }
        };

        let mut metrics = Vec::with_capacity(validators.len());
        validators.iter().for_each(|v| {
            metrics.push(RealTimeCounter {
                idx: v.share_index,
                inmsg: 0,
                outmsg: 0,
                badmsg: 0,
                time: SystemTime::now(),
            })
        });

        let actions: Vec<MetricAction> = Vec::new();
        let triple_counter: Vec<RealTimeTripleCounter> = Vec::new();
        (validators, metrics, actions, triple_counter)
    }
}
