use lit_core::config::LitConfig;
use lit_node::{
    p2p_comms::web::internal::handle_node_share_set,
    tasks::fsm_worker::NoopFSMWorkerMetadata,
    tss::common::{
        models::{NodeTransmissionDetails, RoundData},
        traits::fsm_worker_metadata::FSMWorkerMetadata,
    },
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tracing::{error, info, trace};

use super::virtual_node_collection::VirtualNodeCollection;

pub async fn setup_background_channels(
    vnc: &mut VirtualNodeCollection,
    lit_config: Arc<LitConfig>,
) {
    info!(
        "Setting up background channels between {} nodes.",
        vnc.nodes.len()
    );
    let mut all_tx_round_senders: HashMap<String, Arc<flume::Sender<RoundData>>> = HashMap::new();
    for node in vnc.nodes.iter_mut() {
        let tx_round_senders = node.node_channels.tx_round_manager.clone();
        all_tx_round_senders.insert(node.addr.clone(), tx_round_senders.clone());
    }

    for node in vnc.nodes.iter_mut() {
        let rx_node_transmission_details =
            Arc::new(node.node_channels.rx_node_transmission_details.clone());
        let (quit_tx, quit_rx) = tokio::sync::mpsc::channel(1);
        let all_tx_round_senders = all_tx_round_senders.clone();
        tokio::spawn(async move {
            data_receiver(quit_rx, rx_node_transmission_details, all_tx_round_senders).await;
        });
        node.quit_dm_tx = quit_tx;

        let (quit_tx, quit_rx) = tokio::sync::mpsc::channel(1);
        let lit_config = lit_config.clone();
        let tx_rounds_manager = node.node_channels.tx_round_manager.clone();
        let rx_round_sender = node.node_channels.rx_round_data.clone();
        tokio::spawn(async move {
            super::endpoint_channels::rounds_worker(
                quit_rx,
                lit_config,
                rx_round_sender,
                tx_rounds_manager,
            )
            .await;
        });
        node.quit_rm_tx = quit_tx;
    }
}

pub async fn data_receiver(
    mut quit_rx: tokio::sync::mpsc::Receiver<bool>,
    rx_node_transmission_details: Arc<flume::Receiver<NodeTransmissionDetails>>,
    tx_round_senders: HashMap<String, Arc<flume::Sender<RoundData>>>,
) {
    trace!("Starting: channel_mapper::data_receiver");
    let remote_addr = SocketAddr::from(([127, 0, 0, 1], 8080)); // this socket address is just used for debug and tracing and logging in the handle_node_share_set endpoint, so that we can easily see where a message came from in the logs if needed.  it's not actually used by the code anywhere, only outputted for reference.  so sending in localhost is fine.
    let fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64>> =
        Arc::new(NoopFSMWorkerMetadata::new());

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
                trace!("Stopping: tasks::rounds_worker");
                break;
            }

        msg = rx_node_transmission_details.recv_async() => {

                if msg.is_err() {
                    error!("TEST::background_channel_mapper: rx_node_transmission_details.recv_async().await is_err()");
                    break;
                }

                let transmission_details = msg.unwrap();

                let peer_addr = transmission_details.dest_peer.socket_address.clone();
                let entry = transmission_details.node_transmission_entry.clone();
                let tx_round_sender = match tx_round_senders.get(&peer_addr) {
                    Some(tx_round_sender) => tx_round_sender.clone(),
                    None => {
                        error!("TEST::background_channel_mapper: tx_round_senders.get(&peer_addr) is None");
                        continue;
                    }
                };

                if let Err(e) =
                    handle_node_share_set(&tx_round_sender, &fsm_worker_metadata, entry, remote_addr).await
                {
                    error!(
                        "TEST::background_channel_mapper: handle_node_share_set error: {:?}",
                        e
                    );
                    break;
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }

    info!("Stopping: channel_mapper::data_receiver");
}
