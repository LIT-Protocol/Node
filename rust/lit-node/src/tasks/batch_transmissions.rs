use crate::config::{LitNodeConfig, CFG_KEY_ECDSA_BATCH_SEND_INTERVAL_MS_DEFAULT};
use crate::error::{unexpected_err, Result};
use crate::networking::http::headers::{self, join_multiple_body_descriptor_parameters};
use crate::peers::peer_reviewer::{Issue, PeerComplaint};
use crate::peers::peer_state::models::PeerSocketAddressVec;
use crate::peers::PeerState;
use crate::tasks::utils::get_body_descriptor_for_node_transmission_batch_entries;
use crate::tss::common::models::{
    NodeShareBatchSetRequest, NodeShareSetRequest, NodeShareSetResponse,
    NodeTransmissionBatchEntries, NodeTransmissionDetails, NodeTransmissionEntry,
};
use crate::tss::common::utils::get_body_descriptor_for_node_transmission_entry;
use crate::utils;
use crate::utils::future::call_with_retry_condition;
use lit_api_core::client::reqwest::handler::handle_post;
use lit_core::config::LitConfig;
use lit_core::error::Kind;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::instrument;

/// Module-internal struct for holding peer-peer communication details.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct NodeCommunicationDetails {
    /// The peer's address.
    pub addr: String,
    /// The fully qualified URL to be sending a request to.
    pub url: String,
}

/// re-factor all of this:
pub async fn batch_transaction_worker(
    mut quit_rx: mpsc::Receiver<bool>,
    lit_config: Arc<LitConfig>,
    peer_state: Arc<PeerState>,
    rx_node_transmission_details: flume::Receiver<NodeTransmissionDetails>,
    http_client: Client,
) {
    // these 4 vars need to be updated occasionally
    let batch_send_interval = lit_config
        .ecdsa_batch_send_interval()
        .unwrap_or(CFG_KEY_ECDSA_BATCH_SEND_INTERVAL_MS_DEFAULT)
        as u64;
    let prefix = lit_config.http_prefix_when_talking_to_other_nodes();
    let mut transmission_queue: HashMap<NodeCommunicationDetails, Vec<NodeTransmissionEntry>> =
        HashMap::new();
    info!("Starting: tasks::batch_transaction_worker");
    let mut trigger_time = tokio::time::Instant::now();

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
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

                let node_config = &transmission_details.node_config;
                let peer_addr = transmission_details.peer_address.clone();
                let url_path =  match transmission_details.is_batch {
                        true => "node_share_batch_set",
                        false => "node_share_set"
                    };

                let url = format!(
                        "{}{}/{}",
                        node_config
                            .lit_config
                            .http_prefix_when_talking_to_other_nodes(),
                        peer_addr,
                        url_path
                    );

                if !transmission_details.is_batch {

                    let http_client = http_client.clone();
                    let peer_state = peer_state.clone();
                    tokio::spawn(async move{
                        let r = send_direct(url, &transmission_details, &peer_state, http_client).await;
                        match r {
                            Err(e) => {
                                error!("Error sending direct transmission: {}", e);
                            },
                            Ok(r) => {
                                if !r {
                                    error!("Response for node_share_push_direct was not success");
                                }
                            }
                        }
                        // trace!("Send direct transmission to {} for round {}.", peer_addr, &transmission_details.round);
                    });
                }
                else {
                    let round = &transmission_details.round;
                    let dest_index = transmission_details.dest_index;
                    let entry = transmission_details.node_transmission_entry.clone();

                    let peer_comm_details = NodeCommunicationDetails {
                        addr: peer_addr.clone(),
                        url: url.clone(),
                    };

                    if !transmission_queue.contains_key(&peer_comm_details) {
                        transmission_queue.insert(peer_comm_details.clone(), Vec::new());
                    }
                    let queue = match transmission_queue.get_mut(&peer_comm_details) {
                        Some(queue) => queue,
                        None => {
                            error!("Could not get queue");
                            continue;
                        }
                    };
                    queue.push(entry);
                    // trace!("Queued batch transmission to {} for round {}.  Queue length: {}", peer_addr, round, queue.len());
                }
            }

            _ = check_batch_send(&mut transmission_queue, http_client.clone(), &prefix, &mut trigger_time, batch_send_interval, peer_state.clone(), lit_config.clone()) => {
                // do nothing
            }

        }
    }

    info!("Stopped: tasks::batch_transaction_worker");
}

async fn check_batch_send(
    transmission_queue: &mut HashMap<NodeCommunicationDetails, Vec<NodeTransmissionEntry>>,
    http_client: Client,
    prefix: &str,
    trigger_time: &mut tokio::time::Instant,
    batch_send_interval: u64,
    peer_state: Arc<PeerState>,
    lit_config: Arc<LitConfig>,
) {
    if transmission_queue.is_empty() {
        tokio::time::sleep(Duration::from_millis(1)).await; // yield to other tasks
        return;
    }

    if tokio::time::Instant::now() < *trigger_time {
        tokio::time::sleep(Duration::from_millis(1)).await;
        return;
    }

    let transmission_queue_clone = transmission_queue.clone();
    let peer_state = peer_state.clone();
    tokio::task::spawn(async move {
        let peer_communication_details = transmission_queue_clone
            .keys()
            .collect::<Vec<&NodeCommunicationDetails>>();
        debug!(
            "Background: processing {} batch transmissions for {:?}.",
            transmission_queue_clone.len(),
            &peer_communication_details
        );
        let start = std::time::Instant::now();
        for peer_communication_detail in peer_communication_details {
            let tq_entries = match transmission_queue_clone.get(peer_communication_detail) {
                Some(tq_entries) => tq_entries,
                None => {
                    error!("Could not get transmission_queue_clone entry");
                    return;
                }
            };
            let mut entries = tq_entries.clone();
            entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            let mut err_count: usize = 0;
            let mut success_count: usize = 0;

            let entry = match entries.first() {
                Some(entry) => entry,
                None => {
                    error!("Could not get first entry");
                    return;
                }
            };

            let batch_entries = NodeTransmissionBatchEntries {
                entries: entries.clone(),
            };

            let l = match tq_entries.first() {
                Some(l) => l,
                None => {
                    error!("Could not get first entry");
                    return;
                }
            };
            let all_peers_addresses = match peer_state.peer_socket_addresses().await {
                Ok(all_peers) => all_peers.active_addresses(),
                Err(e) => {
                    error!("Could not get all_peers: {}", e);
                    return;
                }
            };
            let peer_addr = match all_peers_addresses.get(l.dest_index as usize - 1) {
                Some(peer_addr) => peer_addr,
                None => {
                    error!("Could not get peer_addr");
                    return;
                }
            };
            // Encrypt and serialize entry.
            let encrypted_entries = match utils::serde_encrypt::encrypt_and_serialize(
                &peer_state,
                peer_addr,
                &batch_entries,
            ) {
                Ok(encrypted_entries) => encrypted_entries,
                Err(e) => {
                    error!(
                        "Problem encrypting and serializing batch transmission for  ({}).  Error is {}",
                        peer_communication_detail.url.clone(),
                        e
                    );
                    return;
                }
            };

            // Serialize and send as bytes.
            let node_share_set_batch_request = NodeShareBatchSetRequest {
                sender_id: peer_state.peer_id,
                encrypted_entries,
            };

            let mut request_builder = http_client
                .post(peer_communication_detail.url.clone())
                .json(&node_share_set_batch_request);
            let enable_http_header_descriptors = lit_config.enable_http_header_descriptors();
            match enable_http_header_descriptors {
                Err(e) => {
                    error!(
                        "Error getting enable_http_header_descriptors from lit_config: {}",
                        e
                    );
                }
                Ok(enable) => {
                    if enable {
                        let header_value = join_multiple_body_descriptor_parameters(
                            &get_body_descriptor_for_node_transmission_batch_entries(
                                &batch_entries,
                            ),
                        );
                        request_builder = request_builder
                            .header(headers::NODE_REQUEST_BODY_DESCRIPTOR_KEY, header_value);
                    }
                }
            }
            match request_builder.send().await {
                Ok(res) => {
                    success_count += 1;
                    let keys = entries
                        .iter()
                        .map(|e| e.key.clone())
                        .collect::<Vec<String>>();
                    // info!(
                    //     "Sent batch to {}  ({} entries -> {:?})",
                    //     entry.dest_index,
                    //     entries.len(),
                    //     keys
                    // );

                    let res_body = res.text().await;
                }
                Err(error) => {
                    // Complain peer if the error is a timeout
                    if error.is_timeout() || error.is_connect() {
                        warn!("send_message to {:?} has failed, attempting to get peer staker address for complaint {:?}", peer_communication_detail.addr, error);

                        match peer_state
                            .get_peer_staker_address_for_complain(&peer_communication_detail.addr)
                            .await
                        {
                            Ok(peer_staker_address_to_complain) => {
                                let complainer = peer_state.addr.clone();
                                let complaint_channel = peer_state.complaint_channel.clone();

                                if let Err(e) = complaint_channel
                                    .send_async(PeerComplaint {
                                        complainer,
                                        issue: Issue::Unresponsive,
                                        peer_node_staker_address: peer_staker_address_to_complain,
                                    })
                                    .await
                                {
                                    error!(
                                        "Failed to send complaint to complaint_channel: {:?}",
                                        e
                                    );
                                }
                            }
                            Err(e) => {
                                error!("Unable to get peer staker address for complaint: {:?}", e);
                            }
                        }
                    }

                    let msg = format!(
                        "[] Problem batching to  ({}).  Error: {:?}",
                        peer_communication_detail.url.clone(),
                        error
                    );
                    err_count += 1;
                    error!("{}", msg);
                }
            };

            if err_count > 0 {
                error!(
                    "Batching took {} ms and was {} of {} successful for {:?} entries.",
                    start.elapsed().as_millis(),
                    success_count,
                    success_count + err_count,
                    &batch_entries
                );
            };
        }
    });

    transmission_queue.clear();

    let new_trigger_time = tokio::time::Instant::now() + Duration::from_millis(batch_send_interval);
    *trigger_time = new_trigger_time;
}

#[instrument(skip_all, fields(txn_prefix = transmission_details.node_config.txn_prefix))]
async fn send_direct(
    url: String,
    transmission_details: &NodeTransmissionDetails,
    peer_state: &PeerState,
    http_client: Client,
) -> Result<bool> {
    let node_config = &transmission_details.node_config;
    let peer_addr = transmission_details.peer_address.clone();
    let round = &transmission_details.round;
    let dest_index = transmission_details.dest_index;
    let data = transmission_details.node_transmission_entry.clone();

    // Encrypt and serialize entry.
    let encrypted_entry =
        utils::serde_encrypt::encrypt_and_serialize(peer_state, &peer_addr, &data)?;

    // Serialize and send as bytes.
    let node_share_set_request = NodeShareSetRequest {
        sender_id: peer_state.peer_id,
        encrypted_entry,
    };

    let res = match call_with_retry_condition(
        async || {
            send_request(
                url.clone(),
                transmission_details,
                http_client.clone(),
                &node_share_set_request,
                &data,
            )
            .await
        },
        |error: &crate::error::Error| {
            if error.is_kind(Kind::Timeout, true) {
                debug!("Retrying send_message due to timeout");
                return true;
            }
            false
        },
    )
    .await
    {
        Ok(res) => res,
        Err(error) => {
            // Complain peer if the error is a timeout even after retrying
            if error.is_kind(Kind::Timeout, true) || error.is_kind(Kind::Connect, true) {
                warn!("send_message to {:?} has failed, attempting to get peer staker address for complaint {:?}", transmission_details.peer_address, error);

                match peer_state
                    .get_peer_staker_address_for_complain(&transmission_details.peer_address)
                    .await
                {
                    Ok(peer_staker_address_to_complain) => {
                        let complainer = peer_state.addr.clone();
                        let complaint_channel = peer_state.complaint_channel.clone();

                        if let Err(e) = complaint_channel
                            .send_async(PeerComplaint {
                                complainer,
                                issue: Issue::Unresponsive,
                                peer_node_staker_address: peer_staker_address_to_complain,
                            })
                            .await
                        {
                            error!("Failed to send complaint to complaint_channel: {:?}", e);
                        }
                    }
                    Err(e) => {
                        error!("Unable to get peer staker address for complaint: {:?}", e);
                    }
                }
            }

            return Err(unexpected_err(
                error,
                Some(format!(
                    "[{}] Problem sending request for round {} from node #{} to node #{}  ({}).",
                    &node_config.addr, round, &node_config.current_index, dest_index, peer_addr
                )),
            ));
        }
    };

    if !res.success {
        error!("Response for node_share_push_direct was not success");
        return Ok(false);
    }

    Ok(true)
}

#[instrument(name = "send_request", skip_all)]
async fn send_request(
    url: String,
    transmission_details: &NodeTransmissionDetails,
    http_client: Client,
    node_share_set_request: &NodeShareSetRequest,
    data: &NodeTransmissionEntry,
) -> Result<NodeShareSetResponse> {
    let node_config = &transmission_details.node_config;

    let mut request_builder = http_client.post(&url);
    if node_config.lit_config.enable_http_header_descriptors()? {
        request_builder = request_builder.header(
            headers::NODE_REQUEST_BODY_DESCRIPTOR_KEY,
            get_body_descriptor_for_node_transmission_entry(data),
        );
    }

    handle_post(request_builder, &node_share_set_request).await
}
