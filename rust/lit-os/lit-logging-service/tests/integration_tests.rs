use std::path::PathBuf;
use std::thread::JoinHandle;
use std::{env, thread, time};

use hyper::{body, Body, Client, Method, Request, StatusCode};
use hyperlocal::{UnixClientExt, Uri};
use serde_json::Value;

use tokio::runtime::Runtime;

use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_logging::service::types::{SubmitReq, SubmitResp};

const RESOURCES_TEST_DIR: &str = "resources/test";

#[tokio::test]
async fn test_attestation_success() {
    let socket_file = PathBuf::from("/tmp/lit-attestation-service_2.sock".to_string());
    let _ = start_service(socket_file.clone());

    let client = Client::unix();

    // Make request.
    let req_body = SubmitReq::new(vec![
        serde_json::from_str::<Value>("{\"test1\": true}").unwrap(),
        serde_json::from_str::<Value>("{\"test2\": true}").unwrap(),
    ]);
    let req_body_bytes = serde_json::to_vec(&req_body).unwrap();

    let req = Request::builder()
        .method(Method::POST)
        .header("X-Correlation-Id", "111122223333")
        .uri(Uri::new(&socket_file, "/submit"))
        .body(Body::from(req_body_bytes))
        .expect("Unable to build request");
    let resp = client.request(req).await.expect("failed to request attestation intention");

    // Assertions.
    assert_eq!(resp.status(), StatusCode::OK);
    let resp_bytes = body::to_bytes(resp.into_body()).await.unwrap();
    let submit_resp: SubmitResp = serde_json::from_slice(&resp_bytes).unwrap();

    assert_eq!(submit_resp.submitted, 2);
}

// Util
fn start_service(socket: PathBuf) -> JoinHandle<()> {
    setup_test_env();

    if socket.exists() {
        let _ = std::fs::remove_file(&socket);
    }

    let s = socket.clone();
    let j = thread::spawn(|| {
        let rt = Runtime::new().expect("failed to create tokio Runtime");
        rt.block_on(async move {
            lit_logging_service::start(false, Some(s))
                .await
                .expect("failed to start lit_logging_service");
        });
    });

    // Wait for startup.
    thread::sleep(time::Duration::from_millis(100));

    // Verify
    if !socket.exists() {
        panic!("failed to start service");
    }

    j
}

fn get_test_path(path: &str) -> PathBuf {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push(RESOURCES_TEST_DIR);
    test_path.push(path);
    test_path
}

fn setup_test_env() {
    env::set_var(ENV_LIT_CONFIG_FILE, get_test_path("config/config.toml"));
}
