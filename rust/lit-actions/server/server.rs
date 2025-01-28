use std::path::PathBuf;
use std::pin::Pin;

use anyhow::Result;
use deno_core::futures::TryFutureExt as _;
use lit_actions_grpc::{proto::*, unix};
use lit_api_core::context::{with_context, Tracer, Tracing};
use temp_file::TempFile;
use tokio_stream::{Stream, StreamExt as _};
use tonic::{Request, Response, Status};
use tracing::{debug, error};

use crate::tracing::FromGrpcRequest as _;

#[derive(Default)]
pub struct Server;

impl Server {
    fn into_service(self) -> ActionServer<Self> {
        ActionServer::new(self)
            // Let lit_node enforce size limits
            .max_decoding_message_size(usize::MAX)
    }
}

#[tonic::async_trait]
impl Action for Server {
    type ExecuteJsStream =
        Pin<Box<dyn Stream<Item = Result<ExecuteJsResponse, Status>> + Send + 'static>>;

    async fn execute_js(
        &self,
        request: Request<tonic::Streaming<ExecuteJsRequest>>,
    ) -> Result<Response<Self::ExecuteJsStream>, Status> {
        let tracer = Tracing::from_grpc_request(&request);

        debug!("{:?}", request.metadata());

        let mut stream = request.into_inner();
        let (inbound_tx, inbound_rx) = flume::bounded(0);
        let (outbound_tx, outbound_rx) = flume::bounded(0);

        // Put incoming requests into channel
        tokio::spawn(with_context(tracer.clone(), async move {
            while let Ok(Some(req)) = stream.try_next().await {
                let _ = inbound_tx
                    .send_async(req)
                    .inspect_err(|e| error!("failed to forward request: {e:#}"))
                    .await;
            }
        }));

        // Handle initial execution request, forward ops requests to the runtime
        tokio::spawn(with_context(tracer.clone(), async move {
            let req = inbound_rx
                .recv_async()
                .inspect_err(|e| error!("failed to receive request: {e:#}"))
                .await
                .unwrap_or_default();

            let outbound_tx = outbound_tx.clone();
            let inbound_rx = inbound_rx.clone();

            #[allow(clippy::single_match)]
            match req.union {
                Some(UnionRequest::Execute(req)) => {
                    debug!("{:?}", DebugExecutionRequest::from(&req));

                    std::thread::spawn(move || {
                        let rt = tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .expect("failed to build Tokio runtime to execute JS");

                        rt.block_on(with_context(tracer.clone(), async move {
                            let res = crate::runtime::execute_js(
                                req.code,
                                req.js_params.and_then(|v| serde_json::from_slice(&v).ok()),
                                req.auth_context
                                    .and_then(|v| serde_json::from_slice(&v).ok()),
                                req.http_headers,
                                req.timeout,
                                req.memory_limit.map(|limit| limit as usize),
                                outbound_tx.clone(),
                                inbound_rx.clone(),
                            )
                            .await;
                            let _ = outbound_tx
                                .send_async(match res {
                                    Ok(()) => Ok(ExecutionResult {
                                        success: true,
                                        request_id: tracer.correlation_id().to_string(),
                                        ..Default::default()
                                    }
                                    .into()),
                                    Err(err) => {
                                        // Return Tonic error as-is, otherwise return ExecutionResult
                                        if let Some(status) = err.downcast_ref::<Status>() {
                                            error!("{status:#}");
                                            Err(status.clone())
                                        } else {
                                            Ok(ExecutionResult {
                                                success: false,
                                                error: if let Some(js_err) =
                                                    err.downcast_ref::<deno_core::error::JsError>()
                                                {
                                                    deno_runtime::fmt_errors::format_js_error(
                                                        js_err,
                                                    )
                                                } else {
                                                    format!("{err:#}")
                                                },
                                                ..Default::default()
                                            }
                                            .into())
                                        }
                                    }
                                })
                                .inspect_err(|e| error!("failed to send execution result: {e:#}"))
                                .await;
                        }));
                    });
                }
                _ => {} // Ignore empty requests
            }
        }));

        Ok(Response::new(Box::pin(outbound_rx.into_stream())))
    }
}

pub async fn start_server(socket_path: impl Into<PathBuf>) -> Result<()> {
    unix::start_server(
        Server.into_service(),
        socket_path,
        None::<std::future::Ready<()>>,
    )
    .await
}

pub async fn start_server_with_shutdown<P, S>(socket_path: P, signal: S) -> Result<()>
where
    P: Into<PathBuf>,
    S: std::future::Future<Output = ()>,
{
    unix::start_server(Server.into_service(), socket_path, Some(signal)).await
}

pub struct TestServer {
    pub socket_file: TempFile,
}

impl TestServer {
    pub fn start() -> Self {
        let socket_file = temp_file::empty();
        let socket_path = socket_file.path().to_path_buf();

        std::thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().expect("failed to create runtime");
            rt.block_on(async move {
                start_server(socket_path)
                    .await
                    .expect("failed to start action server");
            });
        });

        // Wait for startup
        std::thread::sleep(std::time::Duration::from_millis(100));

        Self { socket_file }
    }

    pub fn socket_path(&self) -> PathBuf {
        self.socket_file.path().to_path_buf()
    }
}
