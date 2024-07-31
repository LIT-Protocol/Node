use crate::peers::peer_state::models::NetworkState;
use crate::peers::PeerState;

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::MissedTickBehavior;

/// This worker simply connects to all nodes on an interval, which will trigger a complaint if they are not online
pub async fn peer_checker_worker(mut quit_rx: mpsc::Receiver<bool>, peer_state: Arc<PeerState>) {
    info!("Starting: tasks::peer_checker_worker");

    // FIXME this should be a config var pulled from chain
    let mut interval = tokio::time::interval(Duration::from_millis(10000));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    interval.tick().await; // first tick is instant - do it now because we already connect to the nodes on startup elsewhere

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
                break;
            }

            _= interval.tick() => {
                let network_state_res = peer_state
                .network_state()
                .await;

                let network_state = match network_state_res {
                    Ok(state) => state,
                    Err(e) => {
                        error!("Error getting network state in peer_checker_worker: {:?}", e);
                        continue;
                    }
                };
                if network_state == NetworkState::Restore || network_state == NetworkState::Paused {
                    continue; // don't connect to peers if we are restoring or paused
                }
                // connect to all nodes in the peer set, which will trigger a complaint if they are not connected
                let peer_state_for_spawning = peer_state.clone();
                tokio::spawn(async move {
                    match peer_state_for_spawning.connect_to_validators_union().await {
                        Ok(_) => {}
                        Err(e) => {
                            error!("Error connecting to all nodes in the peer set in peer_checker_worker: {:?}", e);
                        }
                    }
                });
            }
        }
    }

    info!("Stopped: tasks::peer_checker_worker");
}
