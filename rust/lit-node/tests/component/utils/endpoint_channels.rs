use lit_core::config::LitConfig;
use lit_node::config::{LitNodeConfig, CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT};
use lit_node::tss::common::models::{RoundCommand, RoundCommsChannel, RoundData, RoundsShareSet};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::mpsc;
use tracing::{error, info, trace};

pub async fn rounds_worker(
    mut quit_rx: mpsc::Receiver<bool>,
    lit_config: Arc<LitConfig>,
    rx_rounds_manager: flume::Receiver<RoundData>,
    tx_rounds_manager: Arc<flume::Sender<RoundData>>,
) {
    trace!("Starting: tasks::rounds_worker");
    let mut channels: HashMap<String, RoundCommsChannel> = HashMap::new();
    let mut message_queue: HashMap<String, (SystemTime, Vec<RoundsShareSet>)> = HashMap::new();
    let timeout = lit_config
        .ecdsa_round_timeout()
        .unwrap_or(CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT) as u128;
    let mut heartbeat = tokio::time::interval(Duration::from_millis(timeout as u64));
    heartbeat.tick().await; // First tick is immediate

    let _ = tokio::spawn(async move {
        let mut pinger = tokio::time::interval(Duration::from_millis(1000));
        loop {
            pinger.tick().await; // First tick is immediate
            let msg = RoundData {
                command: RoundCommand::Heartbeat,
                round_registration: None,
                round_share_set: None,
            };
            let _ = tx_rounds_manager.send(msg);
        }
    });

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
                trace!("Stopping: tasks::rounds_worker");
                break;
            }
            incoming_msg = rx_rounds_manager.recv_async() => {

                let incoming_msg = match incoming_msg {
                    Ok(incoming_msg) => incoming_msg,
                    Err(e) => {
                        error!("Error receiving from rounds_manager: {:?}", e);
                        continue;
                    }
                };

                match incoming_msg.command {
                    RoundCommand::Heartbeat => {
                        // Do nothing.  Just wakes things up if no messages are being received.
                    },
                    RoundCommand::AddChannel => {
                        if let Some(round_registration) = incoming_msg.round_registration {
                            let new_channels = match round_registration.channels {
                                Some(channels) => channels,
                                None => {
                                    error!("Unable to get channels from round_registration");
                                    continue;
                                }
                            };

                            trace!("BG: added channel: {} - total: {}", &round_registration.id.clone(), channels.len());
                            // Check message queue for pending messages
                            if let Entry::Occupied(o) = message_queue.entry(round_registration.id.clone()) {
                                for msg in o.get().1.iter() {
                                    trace!("BG: found pending message for channel: {}", &round_registration.id);
                                    let new_channels = new_channels.clone();
                                    let msg = msg.clone();
                                    tokio::spawn(async move {
                                        let _ = new_channels.tx.send_async(msg).await;
                                    });
                                }
                            }
                            channels.insert(round_registration.id.clone(), new_channels);
                            message_queue.remove(&round_registration.id);
                        } else {
                            error!("BG: AddChannel command received without round_registration");
                        }
                    },
                    RoundCommand::RemoveChannel => {
                        if let Some(round_registration) = incoming_msg.round_registration {
                            trace!("BG: removed channel: {} - total: {}", &round_registration.id, channels.len());
                            channels.remove(&round_registration.id.clone());
                            // Delete pending messages
                            message_queue.remove(&round_registration.id);
                        } else {
                            error!("BG: RemoveChannel command received without round_registration");
                        }
                    },
                    RoundCommand::IncomingData => {
                        if let Some(round_share_set) = incoming_msg.round_share_set {

                            if let Some(channel) = channels.get(&round_share_set.channel_id) {
                                // trace!("BG: new message from: {} for channel: {}", &round_share_set.from_index, &round_share_set.channel_id);
                                let channel = channel.clone();
                                tokio::spawn(async move {
                                    let r = channel.tx.send_async(round_share_set).await;
                                    if let Err(e) = r {
                                        error!("Error sending message in rounds_worker: {:?}", e);
                                    }
                                });
                            } else {
                                    trace!("BG: Channel {} not found, adding to queue", round_share_set.channel_id);
                                    // check this message queue later
                                    let pending = message_queue.entry(round_share_set.channel_id.clone()).or_insert((SystemTime::now(), Vec::new()));
                                    pending.1.push(round_share_set);
                            }
                        } else {
                            error!("BG: IncomingData command received without round_share_set");
                        }
                    }
                }
            }
            _ = heartbeat.tick() => prune_old_messages(&mut message_queue, timeout).await,
        }
    }

    trace!("Stopped: tasks::rounds_worker");
}

async fn prune_old_messages(
    message_queue: &mut HashMap<String, (SystemTime, Vec<RoundsShareSet>)>,
    timeout: u128,
) {
    tracing::warn!("BG: pruning messages: {:?}", std::time::SystemTime::now());

    let now = SystemTime::now();
    let mut to_remove: Vec<String> = Vec::new();
    for (channel_id, (time, _)) in message_queue.iter() {
        let elapsed = match now.duration_since(*time) {
            Ok(elapsed) => elapsed,
            Err(e) => {
                error!("Error getting elapsed time: {:?}", e);
                return;
            }
        };
        if elapsed.as_millis() > timeout {
            info!("BG: Removing old message queue for channel: {}", channel_id);
            to_remove.push(channel_id.clone());
        }
    }
    for channel_id in to_remove {
        let msgs = &message_queue[&channel_id].1;
        error!(
            "BG: Removing old message queue for channel: {} with {:?} messages",
            channel_id, msgs
        );
        message_queue.remove(&channel_id);
    }
}
