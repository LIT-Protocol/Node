use crate::p2p_comms::web::grpc_transmissions::chatter::{
    chatter_service_server::ChatterService, chatter_service_server::ChatterServiceServer,
    NodeShareRequestProto, NodeShareResponseProto,
};
use crate::p2p_comms::web::internal::handle_node_share_set;
use crate::tasks::batch_transmissions::INTERNAL_CHATTER_PORT_OFFSET;
use crate::tss::common::models::NodeTransmissionEntry;
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use crate::tss::common::tss_state::TssState;
use crate::utils;
use lit_api_core::config::LitApiConfig;
use lit_core::config::LitConfig;
use std::fs;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tonic::transport::{Server, ServerTlsConfig};
use tonic::{self, Code, Status};
use xor_name::XorName;

#[allow(clippy::unwrap_used)]
pub mod chatter {
    tonic::include_proto!("chatter");
}

#[derive(Clone)]
pub struct ChatterServer {
    pub tss_state: Arc<TssState>,
    pub fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64>>,
}

#[tonic::async_trait]
impl ChatterService for ChatterServer {
    async fn send_direct_grpc(
        &self,
        request: tonic::Request<NodeShareRequestProto>,
    ) -> tonic::Result<tonic::Response<NodeShareResponseProto>, tonic::Status> {
        let peer_state = self.tss_state.peer_state.clone();
        let tx_round_sender = self.tss_state.tx_round_manager.clone();
        let sender = request.remote_addr();
        let remote_addr = match sender {
            Some(remote_addr) => remote_addr,
            None => {
                error!("Could not get remote address from sender: {:?}", sender);
                return Err(Status::new(
                    Code::Internal,
                    "Could not get remote address".to_string(),
                ));
            }
        };
        let req = request.into_inner();

        let entry = match utils::serde_encrypt::deserialize_and_decrypt::<NodeTransmissionEntry>(
            peer_state.as_ref(),
            XorName::from_content(req.sender_id.as_bytes()),
            req.encrypted_entry.as_ref(),
        ) {
            Ok(entry) => entry,
            Err(e) => {
                error!("Error deserializing and decrypting entry: {:?}", e);
                return Err(Status::new(
                    Code::Internal,
                    format!("Error deserializing and decrypting entry: {:?}", e),
                ));
            }
        };
        if let Err(e) = handle_node_share_set(
            &tx_round_sender,
            &self.fsm_worker_metadata,
            entry,
            remote_addr,
        )
        .await
        {
            error!("Error handling node share set: {:?}", e);
            return Err(Status::new(
                Code::Internal,
                format!("Error handling node share set: {:?}", e),
            ));
        }
        let reply = NodeShareResponseProto { success: true };
        Ok(tonic::Response::new(reply))
    }
}

pub async fn launch_chatter_server(
    tss_state: Arc<TssState>,
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64>>,
    cfg: Arc<LitConfig>,
    mut quit_rx: broadcast::Receiver<bool>,
    mut file_rx: mpsc::Receiver<bool>,
) {
    let (controller_tx, mut controller_rx) = broadcast::channel(1);
    let chatter_server = ChatterServer {
        tss_state: tss_state.clone(),
        fsm_worker_metadata: fsm_worker_metadata.clone(),
    };
    let self_addr = format!(
        "0.0.0.0:{}",
        chatter_server.tss_state.port + INTERNAL_CHATTER_PORT_OFFSET as u32
    );
    let server_url = self_addr.parse().expect("failed to parse server url");

    let mut builder = match cfg.https_enabled() {
        true => match (cfg.tls_certs(), cfg.tls_key()) {
            (Some(cert_path), Some(key_path)) => match (fs::read(cert_path), fs::read(key_path)) {
                (Ok(cert), Ok(key)) => {
                    let tls_config = ServerTlsConfig::new()
                        .identity(tonic::transport::Identity::from_pem(cert, key));
                    Server::builder()
                        .tls_config(tls_config)
                        .expect("failed to create server")
                }
                _ => {
                    error!("Failed to read tls cert or key");
                    Server::builder()
                }
            },
            _ => {
                error!("tls cert or key not set in config");
                Server::builder()
            }
        },
        false => Server::builder(),
    };
    let chatter_service = builder
        .add_service(ChatterServiceServer::new(chatter_server.clone()))
        .serve_with_shutdown(server_url, async move {
            controller_rx.recv().await.ok();
        });

    info!("Starting grpc chatter server on {}", server_url);
    tokio::spawn(chatter_service);

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
                let _ = controller_tx.send(true);
                break;
            }
            _ = file_rx.recv() => {
                info!("Reloading grpc chatter server due to tls file change");
                let tss_state = tss_state.clone();
                let fsm_worker_metadata = fsm_worker_metadata.clone();
                let cfg = cfg.clone();

                let _ = controller_tx.send(true);
                let self_addr = format!(
                    "0.0.0.0:{}",
                    tss_state.port + INTERNAL_CHATTER_PORT_OFFSET as u32
                );
                let server_url = self_addr.parse().expect("failed to parse server url");

                let mut builder = match cfg.https_enabled() {
                    true => {
                        match (cfg.tls_certs(), cfg.tls_key()) {
                            (Some(cert_path), Some(key_path)) => {
                                match (fs::read(cert_path), fs::read(key_path)) {
                                    (Ok(cert), Ok(key)) => {
                                    let tls_config = ServerTlsConfig::new()
                                        .identity(tonic::transport::Identity::from_pem(cert, key));
                                    Server::builder()
                                        .tls_config(tls_config)
                                        .expect("failed to create server")
                                    }
                                    _ => {
                                        error!("Failed to read tls cert or key");
                                        Server::builder()
                                    }
                                }
                            }
                            _ => {
                                error!("tls cert or key not set in config");
                                Server::builder()
                            }

                        }
                    }
                    false => Server::builder(),
                };
                let mut rx = controller_tx.subscribe();
                let chatter_service = builder
                    .add_service(ChatterServiceServer::new(chatter_server.clone()))
                    .serve_with_shutdown(server_url, async move {
                        rx.recv().await.ok();
                    });

                info!("Restarting grpc chatter server on {}", server_url);
                tokio::spawn(chatter_service);
            }
        }
    }
}
