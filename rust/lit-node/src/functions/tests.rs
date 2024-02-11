use std::sync::Arc;

use indoc::{formatdoc, indoc};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::auth::auth_material::JsonAuthSig;
use crate::error::Result;
use crate::functions::bindings::RustJsComms;
use crate::functions::execute_js;
use crate::models;
use lit_core::config::LitConfig;

fn test_lit_config() -> Arc<LitConfig> {
    Arc::new(
        lit_core::config::LitConfigBuilder::default()
            .set_default("default.domain", "127.0.0.1")
            .set_default("default.port", 7470)
            .set_default("lit.env", "dev")
            .build()
            .unwrap(),
    )
}

async fn execute(code: &str) -> Result<RustJsComms> {
    execute_js(
        code.to_string(),
        None,
        JsonAuthSig::default(),
        None,
        models::DenoExecutionEnv {
            cfg: test_lit_config(),
            ..Default::default()
        },
        "some-request-id".to_string(),
        None,
        None,
    )
    .await
}

#[tokio::test]
async fn nop() {
    let code = "// Do nothing";

    let res = execute(code).await.unwrap();

    assert_eq!(res.claim_data.len(), 0);
    assert_eq!(res.node_address, "127.0.0.1:7470");
    assert_eq!(res.node_port, "7470");
    assert_eq!(res.signed_data.len(), 0);
    assert_eq!(res.decrypted_data.len(), 0);
    assert_eq!(res.claim_count, 0);
    assert_eq!(res.fetch_count, 0);
    assert_eq!(res.sign_count, 0);
    assert_eq!(res.lit_action_ipfs_id, None);
    assert_eq!(res.response, "");
    assert_eq!(res.logs, "");
    assert_eq!(res.error, None);
    assert_eq!(res.request_id, "some-request-id");
}

#[tokio::test]
async fn console_log() {
    let code = indoc! {r#"
        console.log("It")
        console.info("works")
        console.error("!")
    "#};

    let res = execute(code).await.unwrap();

    assert_eq!(res.logs, "It\nworks\n!\n");
}

#[tokio::test]
async fn set_response() {
    let res = execute(r#"LitActions.setResponse({response: "Not OK"})"#)
        .await
        .unwrap();
    assert_eq!(res.response, "Not OK");

    let res = execute(r#"Lit.Actions.setResponse({response: "OK"})"#)
        .await
        .unwrap();
    assert_eq!(res.response, "OK");
}

#[tokio::test]
async fn js_params() {
    let code = indoc! {r#"
        LitActions.setResponse({response: hello})
        console.log(WORLD)
    "#};

    let res = execute_js(
        code.to_string(),
        None,
        JsonAuthSig::default(),
        Some(json!({"hello": "hello", "WORLD": [2, 0, 2, 3]})),
        models::DenoExecutionEnv {
            cfg: test_lit_config(),
            ..Default::default()
        },
        "some-request-id".to_string(),
        None,
        None,
    )
    .await
    .unwrap();

    assert_eq!(res.response, "hello");
    assert_eq!(res.logs, "[ 2, 0, 2, 3 ]\n");
}

#[tokio::test]
async fn auth_context() {
    let code = indoc! {r#"
        LitActions.setResponse({response: JSON.stringify(LitAuth)})
        console.log(Lit.Auth.actionIpfsIds)
    "#};

    let res = execute_js(
        code.to_string(),
        None,
        JsonAuthSig::default(),
        Some(json!({"hello": "hello", "WORLD": [2, 0, 2, 3]})),
        models::DenoExecutionEnv {
            cfg: test_lit_config(),
            auth_context: models::AuthContext {
                action_ipfs_ids: vec!["some-ipfs-id".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        "some-request-id".to_string(),
        None,
        None,
    )
    .await
    .unwrap();

    assert_eq!(
        res.response,
        r#"{"actionIpfsIds":["some-ipfs-id"],"authSigAddress":null,"authMethodContexts":[],"resources":[]}"#
    );
    assert_eq!(res.logs, "[ \"some-ipfs-id\" ]\n");
}

#[tokio::test]
async fn jwt_decode() {
    let code = indoc! {r#"
        const decoded = jwt.decode(token, { complete: true })
        LitActions.setResponse({response: decoded.payload.loggedInAs})
    "#};

    let res = execute_js(
        code.to_string(),
        None,
        JsonAuthSig::default(),
        Some(json!({
            "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJsb2dnZWRJbkFzIjoiYWRtaW4iLCJpYXQiOjE0MjI3Nzk2Mzh9.gzSraSYS8EXBxLN_oWnFSRgCzcmJmMjLiuyu5CSpyHI",
        })),
        models::DenoExecutionEnv {
            cfg: test_lit_config(),
            ..Default::default()
        },
        "some-request-id".to_string(),
        None,
        None,
    )
    .await
    .unwrap();

    assert_eq!(res.response, "admin");
}

#[tokio::test]
async fn uint8arrays() {
    let code = indoc! {r#"
        const hello = LitActions.uint8arrayFromString("hello")
        const world = LitActions.uint8arrayFromString("world", "utf8")
        console.log(LitActions.uint8arrayToString(hello), hello)
        console.log(LitActions.uint8arrayToString(world, "utf8"), world)
    "#};

    let res = execute(code).await.unwrap();

    assert_eq!(
        res.logs.trim_end(),
        [
            "hello Uint8Array(5) [ 104, 101, 108, 108, 111 ]",
            "world Uint8Array(5) [ 119, 111, 114, 108, 100 ]"
        ]
        .join("\n")
    );
}

#[tokio::test]
async fn reference_error() {
    let code = "nonexisting_function()";

    let res = execute(code).await;

    assert_eq!(
        res.unwrap_err().to_string(),
        "unexpected error: ReferenceError: nonexisting_function is not defined\n    at <user_provided_script>:1:1"
    );
}

#[tokio::test]
async fn throw_error() {
    let code = indoc! {r#"
        throw new Error("boom")
    "#};
    let res = execute(code).await;
    assert_eq!(
        res.unwrap_err().to_string(),
        "unexpected error: Error: boom\n    at <user_provided_script>:1:7"
    );

    let code = indoc! {r#"
        (async () => {
            throw new Error("boom")
        })()
    "#};
    let res = execute(code).await;
    assert_eq!(
        res.unwrap_err().to_string(),
        "unexpected error: Error: boom\n    at <user_provided_script>:2:11\n    at <user_provided_script>:3:3"
    );
}

#[tokio::test]
async fn async_await() {
    let code = indoc! {r#"
        (async () => {
            const response = await "OK"
            LitActions.setResponse({response})
        })()
    "#};

    let res = execute(code).await.unwrap();

    assert_eq!(res.response, "OK");
}

#[tokio::test]
async fn fetch() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/forecast"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "properties": {
                "periods": [
                    { "temperature": 92 }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let code = formatdoc! {r#"
            (async () => {{
                const json = await fetch("{uri}/forecast").then(r => r.json())
                const temp = json.properties.periods[0].temperature;
                LitActions.setResponse({{response: temp.toString()}})
                const text = await fetch("{uri}/forecast").then(r => r.text())
                console.log(text)
            }})()
        "#,
        uri = &mock_server.uri()
    };

    let res = execute(&code).await.unwrap();

    assert_eq!(res.response, "92");
    assert_eq!(
        res.logs,
        "{\"properties\":{\"periods\":[{\"temperature\":92}]}}\n"
    );
    assert_eq!(res.fetch_count, 2);
}

#[tokio::test]
async fn localstorage_shouldnt_panic() {
    let code = indoc! {r#"
        localStorage.setItem("myApp", "Deno");
    "#};

    let res = execute(code).await;
    assert!(res.unwrap_err().to_string().starts_with(
        "unexpected error: NotSupported: LocalStorage is not supported in this context."
    ));
}

#[tokio::test]
async fn webcrypto() {
    let code = indoc! {r#"
        const go = async () => {
            const data = new TextEncoder().encode("Hello, World!");
            const hashed = await crypto.subtle.digest("SHA-256", data);
            Lit.Actions.setResponse({response: hashed.byteLength.toString()});
        }
        go();
    "#};

    let res = execute(code).await.unwrap();

    assert_eq!(res.response, "32");
}

#[tokio::test]
async fn timeout() {
    let code = indoc! {r#"
        while (true) {}
    "#};

    let res = execute_js(
        code.to_string(),
        None,
        JsonAuthSig::default(),
        None,
        models::DenoExecutionEnv {
            cfg: test_lit_config(),
            ..Default::default()
        },
        "some-request-id".to_string(),
        Some(1000),
        Some(100),
    )
    .await;

    assert_eq!(
        res.unwrap_err().to_string(),
        "timeout error: Your function exceeded the maximum runtime of 1000ms and was terminated."
    );
}

#[tokio::test]
async fn oom() {
    let code = indoc! {r#"
        let s = ""
        while (true) {
            s += "Hello"
        }
    "#};

    let res = execute_js(
        code.to_string(),
        None,
        JsonAuthSig::default(),
        None,
        models::DenoExecutionEnv {
            cfg: test_lit_config(),
            ..Default::default()
        },
        "some-request-id".to_string(),
        Some(2000),
        Some(100),
    )
    .await;

    assert_eq!(
        res.unwrap_err().to_string(),
        "memory limit exceeded: Your function exceeded the maximum memory of 100 MB and was terminated."
    );
}

#[tokio::test]
#[ignore]
async fn call_contract() {
    let code = indoc! {r#"
    const go = async () => {
        // https://sepolia.etherscan.io/address/0xD2f13AeACd77bB8D0aD79c6dB5F081e358b481C2#code
        const toContract = "0xD2f13AeACd77bB8D0aD79c6dB5F081e358b481C2";

        const abi = [{"inputs":[],"stateMutability":"nonpayable","type":"constructor"},{"inputs":[{"internalType":"uint256","name":"a","type":"uint256"},{"internalType":"uint256","name":"b","type":"uint256"}],"name":"add","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"}];

        const contract = new ethers.Contract(toContract, abi);
        const rawTxn = await contract.populateTransaction.add(1,2);
        const txn = ethers.utils.serializeTransaction(rawTxn);
        
        const chain = "sepolia";

        const res = await Lit.Actions.callContract({
            chain,
            txn
        });

        // decode response
        const decodedResult = contract.interface.decodeFunctionResult("add", res)[0].toString();

        Lit.Actions.setResponse({response: decodedResult});
    };
    go();
    "#};

    let res = execute(code).await.unwrap();

    assert_eq!(res.response, "3");
}
