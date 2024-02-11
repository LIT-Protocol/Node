#![allow(dead_code)]

use crate::error::{unexpected_err, unexpected_err_code, EC};
use crate::peers::{peer_item::PeerItem, peer_state::models::PeerValidatorStatus, PeerState};
use crate::tasks::beaver_manager::models::{BeaverMessage, TripleRequest};
use crate::tss::common::comms::push::{
    is_operation_epoch_change, parse_epoch_change_operation_id, parse_node_share_key,
};
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;

use crate::tss::common::models::{
    NodeShareBatchSetRequest, NodeShareSetRequest, NodeShareSetResponse,
    NodeTransmissionBatchEntries, NodeTransmissionEntry, RoundCommand, RoundData, RoundsShareSet,
};
use crate::tss::common::tss_state::TssState;
use crate::utils;
use crate::utils::attestation::create_attestation;

use lit_api_core::context::{with_context, Tracing, TracingSpan};
use lit_api_core::error::ApiError;
use lit_blockchain::config::LitBlockchainConfig;
use lit_core::config::ReloadableLitConfig;
use lit_core::error::Result;
#[cfg(not(feature = "testing"))]
use log::{error, trace};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{post, State};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::{instrument, Instrument};

#[get("/connect/<noonce>", format = "json")]
#[instrument(name = "GET /connect", skip_all)]
pub async fn connect(
    cfg: &State<ReloadableLitConfig>,
    session: &State<Arc<TssState>>,
    peer_state: &State<Arc<PeerState>>,
    noonce: &str,
    tracing: Tracing,
    tracing_span: TracingSpan,
) -> status::Custom<Value> {
    with_context(
        tracing.clone(),
        async move {
            let cfg = cfg.load_full();
            let private_key = match peer_state
                .lit_config
                .blockchain_wallet_private_key_bytes(None)
            {
                Ok(private_key) => private_key,
                Err(e) => {
                    return unexpected_err_code(
                        e,
                        EC::NodeUnknownError,
                        Some("Error retrieving private key".into()),
                    )
                    .handle();
                }
            };

            let secret_key = match libsecp256k1::SecretKey::parse_slice(&private_key) {
                Ok(secret_key) => secret_key,
                Err(e) => {
                    return unexpected_err_code(
                        e,
                        EC::NodeUnknownError,
                        Some("Error parsing secret key".into()),
                    )
                    .handle();
                }
            };
            let public_key = libsecp256k1::PublicKey::from_secret_key(&secret_key);

            let mut peer_item = PeerItem {
                id: peer_state.peer_id,
                public_key,
                node_address: session.peer_state.node_address(),
                sender_public_key: session.peer_state.comskeys.sender_public_key(),
                receiver_public_key: session.peer_state.comskeys.receiver_public_key(),
                staker_address: session.peer_state.staker_address,
                addr: session.addr.clone(),
                status: PeerValidatorStatus::Unknown,
                attestation: None,
            };

            peer_item.attestation = match create_attestation(cfg, noonce).await {
                Ok(at) => Some(at),
                Err(e) => {
                    #[cfg(not(feature = "testing"))]
                    warn!("Error creating attestation: {:?}", e);
                    return status::Custom(Status::Ok, json!(peer_item));
                }
            };

            status::Custom(Status::Ok, json!(peer_item))
        }
        .instrument(tracing_span.span().to_owned()),
    )
    .await
}

/// refactor into session state
#[allow(clippy::too_many_arguments)]
#[post("/node_share_batch_set", format = "json", data = "<request>")]
#[instrument(name = "POST /node_share_batch_set", skip_all)]
pub async fn node_share_batch_set(
    tx_round_sender: &State<Arc<flume::Sender<RoundData>>>,
    fsm_worker_metadata: &State<Arc<dyn FSMWorkerMetadata<LifecycleId = u64>>>,
    peer_state: &State<Arc<PeerState>>,
    request: Json<NodeShareBatchSetRequest>,
    tracing: Tracing,
    tracing_span: TracingSpan,
    remote_addr: SocketAddr,
) -> status::Custom<Value> {
    with_context(
        tracing.clone(),
        async move {
            let node_share_batch_request = request.0;

            // Deserialize and decrypt data.
            let batch_entries = match utils::serde_encrypt::deserialize_and_decrypt::<
                NodeTransmissionBatchEntries,
            >(
                peer_state.as_ref(),
                node_share_batch_request.sender_id,
                node_share_batch_request.encrypted_entries.as_ref(),
            ) {
                Ok(batch_entries) => batch_entries,
                Err(e) => {
                    return unexpected_err(
                        e,
                        Some("Failed to deserialize and decrypt data".into()),
                    )
                    .handle();
                }
            };
            trace!("Received batch of {} entries", batch_entries.entries.len());

            for entry in batch_entries.entries {
                if let Err(e) =
                    handle_node_share_set(tx_round_sender, fsm_worker_metadata, entry, remote_addr)
                        .await
                {
                    error!("Error handling node share set: {:?}", e);
                }
            }

            status::Custom(Status::Ok, json!(NodeShareSetResponse { success: true }))
        }
        .instrument(tracing_span.span().to_owned()),
    )
    .await
}

#[allow(clippy::too_many_arguments)]
#[post("/node_share_set", format = "json", data = "<request>")]
#[instrument(name = "POST /node_share_set", skip_all)]
pub async fn node_share_set(
    tx_round_sender: &State<Arc<flume::Sender<RoundData>>>,
    fsm_worker_metadata: &State<Arc<dyn FSMWorkerMetadata<LifecycleId = u64>>>,
    peer_state: &State<Arc<PeerState>>,
    request: Json<NodeShareSetRequest>,
    tracing: Tracing,
    tracing_span: TracingSpan,
    remote_addr: SocketAddr,
) -> status::Custom<Value> {
    with_context(tracing.clone(), async move {
        let node_share_request: NodeShareSetRequest = request.0;

        // Deserialize and decrypt data.
        let entry = match utils::serde_encrypt::deserialize_and_decrypt::<NodeTransmissionEntry>(
            peer_state.as_ref(),
            node_share_request.sender_id,
            node_share_request.encrypted_entry.as_ref(),
        ) {
            Ok(entry) => entry,
            Err(e) => {
                return unexpected_err(e, Some("Failed to deserialize and decrypt data".into()))
                    .handle();
            }
        };
        if let Err(e) = handle_node_share_set(tx_round_sender, fsm_worker_metadata, entry, remote_addr).await {
            return unexpected_err(e, Some("Error handling node share set".into())).handle();
        }

        status::Custom(Status::Ok, json!(NodeShareSetResponse {
            success: true,
        }))
    }.instrument(tracing_span.span().to_owned())).await
}

#[instrument(name = "handle_node_share_set", skip_all, fields(txn_prefix = entry.key))]
pub async fn handle_node_share_set(
    tx_round_sender: &Arc<flume::Sender<RoundData>>,
    fsm_worker_metadata: &Arc<dyn FSMWorkerMetadata<LifecycleId = u64>>,
    entry: NodeTransmissionEntry,
    remote_addr: SocketAddr,
) -> Result<()> {
    let parsed = parse_node_share_key(&entry.key)?;

    let operation_type_and_id = parsed.operation_type_and_id.clone();
    let round_number = parsed.round;

    let channel_id = format!("{}-{}", operation_type_and_id, round_number);
    let created = SystemTime::now();

    let round_share_set = RoundsShareSet {
        key: entry.key,
        value: entry.value,
        channel_id,
        from_index: entry.src_index,
        retry: 0,
        created,
    };

    // If the dkg_id is from an earlier lifecycle ID, then we need to update the metadata and abort
    // and restart any current epoch changes.
    if is_operation_epoch_change(&operation_type_and_id) {
        let parsed_txn_prefix = parse_epoch_change_operation_id(&parsed.operation_type_and_id)?;
        fsm_worker_metadata.compare_with_peer(parsed_txn_prefix.lifecycle_id);
    }

    let round_data = RoundData {
        command: RoundCommand::IncomingData,
        round_registration: None,
        round_share_set: Some(round_share_set),
    };

    let tx_round_sender = tx_round_sender.clone();

    tokio::spawn(async move {
        let r = tx_round_sender.send_async(round_data).await.map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeUnknownError,
                Some("Error pushing to bg message queue".into()),
            )
        });
        if let Err(e) = r {
            error!("Error sending message in handle_node_share_set: {:?}", e);
        }
    });

    Ok(())
}

#[post("/select_triple", format = "json", data = "<request>")]
#[instrument(name = "POST /select_triple", skip_all)]
pub async fn select_triple(
    peer_state: &State<Arc<PeerState>>,
    request: Json<TripleRequest>,
) -> status::Custom<Value> {
    let request = request.0;
    let (tx, rx) = flume::unbounded();

    let msg = BeaverMessage::RemoteRequestForStorageKey(request, tx);

    let result = peer_state
        .bm_tx
        .send_async(msg)
        .await
        .map_err(|e| unexpected_err(e, Some("Could not send request to beavermanager".into())));

    if let Err(e) = result {
        error!("Could not send request to beavermanager: {:?}", &e);
        return e.handle();
    }

    let triple_key = rx.recv_timeout(Duration::from_secs(10));

    match triple_key {
        Ok(triple_key) => {
            return status::Custom(Status::Ok, json!(triple_key));
        }
        Err(e) => {
            error!(
                "Could not receive valid response from beavermanager: {:?}",
                &e
            );
            return unexpected_err(
                format!(
                    "Could not receive valid response from beavermanager: {:?}  ",
                    e
                ),
                None,
            )
            .handle();
        }
    }
}
