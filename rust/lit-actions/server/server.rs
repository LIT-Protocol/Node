use std::path::PathBuf;
use std::pin::Pin;

use anyhow::Result;
use deno_core::error::CoreError;
use deno_core::futures::TryFutureExt as _;
use deno_lib::util::result::any_and_jserrorbox_downcast_ref;
use deno_runtime::tokio_util::create_and_run_current_thread;
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
                        create_and_run_current_thread(with_context(tracer.clone(), async move {
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
                                                error: format_error(&err),
                                                request_id: tracer.correlation_id().to_string(),
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

fn format_error(err: &anyhow::Error) -> String {
    if let Some(CoreError::Js(js_err)) = any_and_jserrorbox_downcast_ref::<CoreError>(err) {
        deno_runtime::fmt_errors::format_js_error(js_err)
    } else {
        format!("{err:#}")
    }
}

pub async fn start_server<P, S>(socket_path: P, shutdown_signal: Option<S>) -> Result<()>
where
    P: Into<PathBuf>,
    S: std::future::Future<Output = ()>,
{
    unix::start_server(Server.into_service(), socket_path, shutdown_signal).await
}

pub struct TestServer {
    pub socket_file: TempFile,
}

impl TestServer {
    pub fn start() -> Self {
        let socket_file = temp_file::empty();
        let socket_path = socket_file.path().to_path_buf();

        std::thread::spawn(|| {
            create_and_run_current_thread(async move {
                let signal = async {
                    let _ = tokio::signal::ctrl_c().await;
                };
                start_server(socket_path, Some(signal))
                    .await
                    .expect("failed to start action server")
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
