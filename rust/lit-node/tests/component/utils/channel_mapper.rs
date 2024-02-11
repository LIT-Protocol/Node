use lit_core::config::LitConfig;
use lit_node::{
    tasks::fsm_worker::NoopFSMWorkerMetadata,
    tss::common::{
        models::{NodeTransmissionDetails, RoundData},
        traits::fsm_worker_metadata::FSMWorkerMetadata,
        web::internal::handle_node_share_set,
    },
};
use log::{error, info};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};

pub async fn setup_background_channels(
    map_rx_node_transmission_details: Vec<Arc<flume::Receiver<NodeTransmissionDetails>>>,
    map_tx_round_senders: HashMap<String, Arc<flume::Sender<RoundData>>>,
    map_rx_round_senders: Vec<(
        tokio::sync::mpsc::Receiver<bool>,
        flume::Receiver<RoundData>,
    )>,
    lit_config: Arc<LitConfig>,
) {
    // we're overriding the sending of messages, and piping them back into the node code
    for rx_node_transmission_details in map_rx_node_transmission_details {
        let map = map_tx_round_senders.clone();
        // receiving handler below
        tokio::spawn(async move {
            data_receiver(rx_node_transmission_details, map).await;
        });
    }

    let d = map_tx_round_senders.into_iter().last().unwrap().1.clone();
    // this is the built in channel handler inside the node code!
    for (quit_rx, rx_round_sender) in map_rx_round_senders {
        // receiving handler below
        let lit_config = lit_config.clone();
        let tx_rounds_manager = d.clone();

        tokio::spawn(async move {
            super::endpoint_channels::rounds_worker(
                quit_rx,
                lit_config,
                rx_round_sender,
                tx_rounds_manager,
            )
            .await;
        });
    }
}

pub async fn data_receiver(
    rx_node_transmission_details: Arc<flume::Receiver<NodeTransmissionDetails>>,
    tx_round_senders: HashMap<String, Arc<flume::Sender<RoundData>>>,
) {
    info!("Starting: TEST::background_channel_mapper");
    let fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64>> =
        Arc::new(NoopFSMWorkerMetadata::new());

    loop {
        let msg = rx_node_transmission_details.recv_async().await;
        if msg.is_err() {
            error!("TEST::background_channel_mapper: rx_node_transmission_details.recv_async().await is_err()");
            break;
        }
        let transmission_details = msg.unwrap();
        let peer_addr = transmission_details.peer_address.clone();
        let entry = transmission_details.node_transmission_entry.clone();
        let tx_round_sender = tx_round_senders.get(&peer_addr).unwrap();
        if let Err(e) = handle_node_share_set(
            tx_round_sender,
            &fsm_worker_metadata,
            entry,
            SocketAddr::new(
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                8080,
            ), // this socket address is just used for debug and tracing and logging in the handle_node_share_set endpoint, so that we can easily see where a message came from in the logs if needed.  it's not actually used by the code anywhere, only outputted for reference.  so sending in localhost is fine.
        )
        .await
        {
            error!(
                "TEST::background_channel_mapper: handle_node_share_set error: {:?}",
                e
            );
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }
}
