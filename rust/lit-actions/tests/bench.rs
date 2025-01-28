use std::path::PathBuf;

use anyhow::{bail, Result};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lit_actions_server::{init_v8, proto::*, unix, TestServer};
use tokio_stream::StreamExt as _;
use tonic::Request;

// This client is only used for benchmarking. The real implementation is provided by lit-node.
#[derive(Debug)]
struct TestClient {
    socket_path: PathBuf,
}

impl TestClient {
    fn new(socket_path: impl Into<PathBuf>) -> Self {
        Self {
            socket_path: socket_path.into(),
        }
    }

    async fn execute_js(
        &mut self,
        request: impl Into<ExecutionRequest>,
    ) -> Result<ExecutionResult> {
        let request = request.into();

        let (outbound_tx, outbound_rx) = flume::bounded(0);
        let channel = unix::connect_to_socket(&self.socket_path).await?;
        let mut client = ActionClient::new(channel);

        let response = client
            .execute_js(Request::new(outbound_rx.into_stream()))
            .await?;
        let mut stream = response.into_inner();

        // Send initial execution request to server
        outbound_tx.send_async(request.into()).await?;

        // Handle responses from server
        while let Some(resp) = stream.try_next().await? {
            match resp.union {
                // Return final result from server
                Some(UnionResponse::Result(res)) => {
                    if !res.success {
                        bail!(res.error);
                    }
                    return Ok(res);
                }
                // Handle op requests
                Some(op) => {
                    let resp = self.handle_op(op);
                    outbound_tx.send_async(resp).await?;
                }
                // Ignore empty responses
                None => {}
            };
        }

        bail!("Server unexpectedly closed connection")
    }

    fn handle_op(&mut self, op: UnionResponse) -> ExecuteJsRequest {
        match op {
            UnionResponse::SetResponse(_) => SetResponseResponse {}.into(),
            UnionResponse::PubkeyToTokenId(_) => PubkeyToTokenIdResponse {
                token_id: "ignored".to_string(),
            }
            .into(),
            _ => unimplemented!("op not implemented"),
        }
    }
}

fn bench(c: &mut Criterion) {
    init_v8();

    let server = TestServer::start();
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let code = indoc::indoc! {r#"
        (async () => {
            const response = LitActions.pubkeyToTokenId({ publicKey: await "0x1234" })
            LitActions.setResponse({response})
        })()
    "#};

    c.bench_function("execute_js", |b| {
        b.to_async(&runtime).iter(|| async {
            TestClient::new(server.socket_path())
                .execute_js(black_box(code))
                .await
                .unwrap();
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
