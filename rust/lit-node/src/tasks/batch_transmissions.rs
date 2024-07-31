use crate::config::{LitNodeConfig, CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT};
use crate::error::{unexpected_err, Result};
use crate::p2p_comms::web::grpc_transmissions::chatter::{
    chatter_service_client::ChatterServiceClient, NodeShareRequestProto,
};
use crate::peers::peer_reviewer::{Issue, PeerComplaint};
use crate::peers::PeerState;
use crate::tss::common::models::NodeTransmissionDetails;
use crate::utils;
use crate::utils::future::call_with_retry_condition;
use lit_api_core::config::LitApiConfig;
use lit_core::config::LitConfig;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::fs;
use tonic;
use tonic::transport::{Channel, ClientTlsConfig};
use tracing::instrument;

#[allow(clippy::unwrap_used)]
pub mod chatter {
    tonic::include_proto!("chatter");
}

pub static INTERNAL_CHATTER_PORT_OFFSET: u16 = 19608;

pub async fn batch_transaction_worker(
    mut quit_rx: tokio::sync::broadcast::Receiver<bool>,
    lit_config: Arc<LitConfig>,
    peer_state: Arc<PeerState>,
    rx_node_transmission_details: flume::Receiver<NodeTransmissionDetails>,
    http_client: Client,
) {
    let prefix = lit_config.http_prefix_when_talking_to_other_nodes();
    info!("Starting: tasks::batch_transaction_worker");

    let mut clients: HashMap<
        String,
        (SystemTime, ChatterServiceClient<tonic::transport::Channel>),
    > = HashMap::new();
    let timeout = lit_config
        .ecdsa_round_timeout()
        .unwrap_or(CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT);
    let mut heartbeat = tokio::time::interval(Duration::from_millis(timeout as u64));
    heartbeat.tick().await;

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
                info!("Shutting down: tasks::batch_transaction_worker");
                break;
            }
            msg = rx_node_transmission_details.recv_async() => {
                let transmission_details = match msg {
                    Ok(transmission_details) => transmission_details,
                    Err(e) => {
                        error!("Error receiving NodeTransmissionDetails: {}", e);
                        continue;
                    }
                };
                let peers = peer_state.clone();

                        //grpc
                        // trace!("Sending direct grpc transmission to: {:?}", transmission_details.dest_peer.socket_address);
                        let peer_addr_full = transmission_details.dest_peer.socket_address.clone();
                        let client = match clients.get(peer_addr_full.as_str()) {
                            Some(client) => {
                                //update timestamp
                                let c = client.1.clone();
                                clients.insert(peer_addr_full.to_string(), (SystemTime::now(), client.1.clone()));
                                c
                            }
                            None => {
                                debug!("Creating new grpc client for peer: {}", peer_addr_full);
                                let mut peer_addr_parts = peer_addr_full.split(':');
                                let peer_addr = match peer_addr_parts.next() {
                                    Some(addr) => addr,
                                    None => {
                                        error!("Failed to parse peer address from: {}", peer_addr_full);
                                        continue;
                                    }
                                };
                                let peer_port: u16 = match peer_addr_parts.next() {
                                    Some(port) => port.parse().expect("Failed to parse peer port to u16"),
                                    None => 443,
                                };
                                let url = format!("{}{}:{}", prefix, peer_addr, INTERNAL_CHATTER_PORT_OFFSET + peer_port);
                                let uri = url.clone().parse().expect("Failed to parse URL");
                                let mut channel_builder = Channel::builder(uri).connect_timeout(Duration::from_secs(CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT as u64));
                                if lit_config.https_enabled() {
                                    match lit_config.tls_certs() {
                                        Some(path) => {
                                            match fs::read(path).await {
                                                Ok(cert_bytes) => {
                                                    let tls_cert = tonic::transport::Certificate::from_pem(cert_bytes);
                                                    channel_builder = match channel_builder.tls_config(ClientTlsConfig::new().ca_certificate(tls_cert)) {
                                                        Ok(c) => c,
                                                        Err(e) => {
                                                            error!("Problem creating channel builder with tls: {:?}", e);
                                                            continue;
                                                        }
                                                    };
                                                },
                                                Err(e) => {
                                                    error!("Problem reading tls certificate: {:?}", e);
                                                }
                                            };
                                        },
                                        None => error!("tls_certs not set in LitConfig"),
                                    }
                                }

                                let client = match channel_builder.connect().await {
                                    Ok(channel) => ChatterServiceClient::new(channel),
                                    Err(e) => {
                                        error!(
                                            "Problem connecting to node #{} ({}): {:?}",
                                            peer_addr, url, e
                                        );
                                        warn!("connecting via grpc to {:?} has failed, attempting to get peer staker address for complaint {:?}", &transmission_details.dest_peer.socket_address, e);
                                        let complainer = peer_state.addr.clone();
                                        let complaint_channel = peer_state.complaint_channel.clone();
                                        if let Err(e) = complaint_channel
                                            .send_async(PeerComplaint {
                                                complainer,
                                                issue: Issue::Unresponsive,
                                                peer_node_staker_address: transmission_details
                                                    .dest_peer
                                                    .staker_address,
                                            })
                                            .await
                                        {
                                            error!("Failed to send complaint to complaint_channel: {:?}", e);
                                        }
                                        continue;
                                    }
                                };
                                clients.insert(peer_addr_full.to_string(), (SystemTime::now(), client.clone()));
                                client
                            }
                        };

                        tokio::spawn(async move {
                            let r = send_direct_grpc(&transmission_details.clone(), &peers, client).await;
                            if let Err(e) = r {
                                let peer_addr_full = transmission_details.dest_peer.socket_address.clone();
                                error!("Error sending direct grpc transmission to {:?}: {}", peer_addr_full, e);
                            };
                        });
                    }
            _ = heartbeat.tick() => prune_old_clients(&mut clients, timeout as u128).await,
        }
    }
}

#[instrument(name = "send_direct", skip_all)]
async fn send_direct_grpc(
    transmission_details: &NodeTransmissionDetails,
    peer_state: &PeerState,
    client: ChatterServiceClient<tonic::transport::Channel>,
) -> Result<bool> {
    let dest_addr_full = transmission_details.dest_peer.socket_address.clone();
    let round = &transmission_details.round;
    let dest_index = transmission_details.dest_peer.share_index;
    let data = transmission_details.node_transmission_entry.clone();

    // Encrypt and serialize entry.
    let encrypted_entry =
        utils::serde_encrypt::encrypt_and_serialize(peer_state, &dest_addr_full, &data)?;
    let client_clone = client.clone();

    let closure = || {
        let mut client_clone = client_clone.clone();
        let encrypted_entry_clone = encrypted_entry.clone(); // Clone encrypted_entry before the closure
        async move {
            let mut request = tonic::Request::new(NodeShareRequestProto {
                sender_id: peer_state.addr.clone(),
                encrypted_entry: encrypted_entry_clone.clone(),
            });
            // set timeout for request
            request.set_timeout(Duration::from_secs(
                CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT as u64,
            ));
            client_clone.send_direct_grpc(request).await
        }
    };

    let res = match call_with_retry_condition(closure, |e: &tonic::Status| {
        e.code() == tonic::Code::DeadlineExceeded
    })
    .await
    {
        Ok(res) => res,
        Err(error) => {
            error!(
                "Problem sending grpc request for round {} from node #{} to node #{}  ({}): {:?}",
                round, &data.src_index, data.dest_index, dest_addr_full, error
            );

            if error.code() == tonic::Code::DeadlineExceeded {
                warn!(
                    "sending grpc message to {:?} has failed, attempting to get peer staker address for complaint {:?}",
                    &transmission_details.dest_peer.socket_address, error
                );
                let complainer = peer_state.addr.clone();
                let complaint_channel = peer_state.complaint_channel.clone();
                if let Err(e) = complaint_channel
                    .send_async(PeerComplaint {
                        complainer,
                        issue: Issue::Unresponsive,
                        peer_node_staker_address: transmission_details.dest_peer.staker_address,
                    })
                    .await
                {
                    error!("Failed to send complaint to complaint_channel: {:?}", e);
                }
            }
            return Err(unexpected_err(
                    error,
                    Some(format!(
                        "[{}] Problem sending grpc request for round {} from node #{} to node #{}  ({}).",
                        &data.src_index, round, data.src_index, dest_index, dest_addr_full
                    )),
                ));
        }
    };

    if !res.into_inner().success {
        error!(
            "Server responded with failure for direct grpc transmission to {:?}.",
            dest_addr_full
        );
        return Ok(false);
    }

    Ok(true)
}

#[instrument(skip_all)]
async fn prune_old_clients(
    clients: &mut HashMap<String, (SystemTime, ChatterServiceClient<tonic::transport::Channel>)>,
    timeout: u128,
) {
    let now = SystemTime::now();
    let mut to_remove: Vec<String> = Vec::new();
    for (peer, (time, _)) in clients.iter() {
        let elapsed = match now.duration_since(*time) {
            Ok(elapsed) => elapsed,
            Err(e) => {
                error!("Error getting elapsed time: {:?}", e);
                return;
            }
        };
        if elapsed.as_millis() > timeout {
            info!("BG: Removing old client for peer: {}", peer);
            to_remove.push(peer.clone());
        }
    }
    for peer in to_remove {
        clients.remove(&peer);
    }
}
