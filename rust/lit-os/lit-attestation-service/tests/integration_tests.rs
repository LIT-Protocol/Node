use hyper::{body, Body, Client, Method, Request, StatusCode};
use hyperlocal::{UnixClientExt, Uri};
use rand::Rng;
use serde_bytes_base64::Bytes;
use std::path::PathBuf;
use std::thread::JoinHandle;
use std::{env, thread, time};

use tokio::runtime::Runtime;
use uuid::Uuid;

/// Note that these tests are only intended to be run with AMD SEV SNP firmware support.
///
/// Note that the binary crate for the attestation service must be running before running
/// these integration tests.
use lit_api_core::error::PublicError;
use lit_attestation::attestation::ENV_ATTESTATION_TYPE_OVERRIDE;
use lit_attestation::service::types::{
    AttestationIntentReq, AttestationIntentResp, AttestationReq, AttestationResp,
};
use lit_attestation::{Attestation, AttestationType};
use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_os_core::config::ENV_BUILD_PRIV_KEY_PATH;

const RESOURCES_TEST_DIR: &str = "resources/test";

#[tokio::test]
async fn test_attestation_without_intent() {
    let socket_file = PathBuf::from("/tmp/lit-attestation-service_1.sock".to_string());
    let _ = start_service(socket_file.clone());

    let client = Client::unix();
    let nonce_for_test = generate_random_nonce();

    // Make Attestation request.
    let attestation = Attestation::new(AttestationType::AmdSevSnp, nonce_for_test)
        .await
        .expect("failed to create Attestation");

    let req_body = AttestationReq { attestation, session_id: Uuid::new_v4().to_string() };
    let req_body_bytes = serde_json::to_vec(&req_body).unwrap();

    let req = Request::builder()
        .method(Method::POST)
        .header("X-Correlation-Id", "111122223333")
        .uri(Uri::new(&socket_file, "/attestation/confirm"))
        .body(Body::from(req_body_bytes))
        .expect("Unable to build attestation request");
    let response = client.request(req).await.expect("failed to request attestation");

    // Assertions.
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let resp_body_bytes = body::to_bytes(response.into_body()).await.unwrap();
    let err_resp: PublicError = serde_json::from_slice(&resp_body_bytes).unwrap();
    assert_eq!(err_resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(*err_resp.message().unwrap(), "Missing attestation intent.");
}

#[tokio::test]
async fn test_attestation_success() {
    let socket_file = PathBuf::from("/tmp/lit-attestation-service_2.sock".to_string());
    let _ = start_service(socket_file.clone());

    let client = Client::unix();
    let nonce_for_test = generate_random_nonce();

    // Make Attestation intent request.
    let attestation_intent_req_body =
        AttestationIntentReq { noonce: Some(Bytes::from(nonce_for_test.clone())) };
    let attestation_intent_req_body_bytes =
        serde_json::to_vec(&attestation_intent_req_body).unwrap();

    let attestation_intent_req = Request::builder()
        .method(Method::POST)
        .header("X-Correlation-Id", "111122223333")
        .uri(Uri::new(&socket_file, "/attestation/intent"))
        .body(Body::from(attestation_intent_req_body_bytes))
        .expect("Unable to build attestation intent request");
    let attestation_intent_response = client
        .request(attestation_intent_req)
        .await
        .expect("failed to request attestation intention");

    // Assertions.
    assert_eq!(attestation_intent_response.status(), StatusCode::OK);
    let attestation_intent_resp_body_bytes =
        body::to_bytes(attestation_intent_response.into_body()).await.unwrap();
    let attestation_intent_resp: AttestationIntentResp =
        serde_json::from_slice(&attestation_intent_resp_body_bytes).unwrap();
    let attestation_intent = attestation_intent_resp.attestation;

    assert_eq!(*attestation_intent.typ(), lit_attestation::AttestationType::AmdSevSnp);
    assert_eq!(*attestation_intent.noonce(), nonce_for_test.clone());
    assert_eq!(attestation_intent.signatures().len(), 0);
    assert!(attestation_intent.report_raw().is_err());

    // Make Attestation request.
    let attestation_req_body = AttestationReq {
        attestation: attestation_intent,
        session_id: attestation_intent_resp.session_id,
    };
    let attestation_req_body_bytes = serde_json::to_vec(&attestation_req_body).unwrap();

    let attestation_req = Request::builder()
        .method(Method::POST)
        .header("X-Correlation-Id", "444455556666")
        .uri(Uri::new(&socket_file, "/attestation/confirm"))
        .body(Body::from(attestation_req_body_bytes))
        .expect("Unable to build attestation request");
    let attestation_response =
        client.request(attestation_req).await.expect("failed to request attestation");

    // Assertions.
    assert_eq!(attestation_response.status(), StatusCode::OK);
    let attestation_resp_body_bytes =
        body::to_bytes(attestation_response.into_body()).await.unwrap();
    let attestation_resp: AttestationResp =
        serde_json::from_slice(&attestation_resp_body_bytes).unwrap();
    let attestation = attestation_resp.attestation;

    assert_eq!(*attestation.typ(), lit_attestation::AttestationType::AmdSevSnp);
    assert_eq!(*attestation.noonce(), nonce_for_test.clone());
    assert_eq!(attestation.signatures().len(), 1);
    //assert!(attestation.report_raw().is_ok());
}

fn generate_random_nonce() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    vec![rng.gen()]
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
            lit_attestation_service::start(false, Some(s))
                .await
                .expect("failed to start lit_attestation_service");
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
    env::set_var(ENV_BUILD_PRIV_KEY_PATH, get_test_path("config/build.pem"));
    env::set_var(ENV_ATTESTATION_TYPE_OVERRIDE, "AMD_SEV_SNP");

    if !PathBuf::from("/dev/sev-guest").exists() {
        env::set_var("LIT_ATTESTATION_SERVICE_BYPASS_GENERATE", "1");
    }
}
