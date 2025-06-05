use std::collections::BTreeMap;

use indoc::{formatdoc, indoc};
use lit_actions_server::{init_v8, TestServer};
use moka::future::Cache;
use pretty_assertions::assert_eq;
use rstest::*;
use serde_json::json;
use std::sync::Arc;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::action_client::{Client, ClientBuilder, ExecutionOptions, ExecutionState};
use crate::models;

#[ctor::ctor]
fn init() {
    // Set RUST_LOG to get logs during testing
    pretty_env_logger::init();

    lit_core::utils::unix::raise_fd_limit();

    init_v8();
}

#[fixture]
fn server() -> TestServer {
    TestServer::start()
}

#[rstest]
#[tokio::test]
async fn nop(server: TestServer) {
    let mut client = Client::new(server.socket_path());

    let res = client.execute_js("// Do nothing").await.unwrap();

    assert_eq!(res, ExecutionState::default());
    assert_eq!(client.request_id(), "");
}

#[rstest]
#[tokio::test]
async fn console_log(server: TestServer) {
    let mut client = Client::new(server.socket_path());

    let code = indoc! {r#"
        console.log("It")
        console.info("works")
        console.error("!")
    "#};

    let res = client.execute_js(code).await.unwrap();

    assert_eq!(res.logs, "It\nworks\n!\n");
    assert_eq!(client.logs(), "It\nworks\n!\n");
}

#[rstest]
#[tokio::test]
async fn console_log_length_limit(server: TestServer) {
    let mut client = ClientBuilder::default()
        .socket_path(server.socket_path())
        .max_console_log_length(3)
        .build()
        .unwrap();

    let res = client
        .execute_js(r#"console.log("ok"); console.log("too long")"#)
        .await;

    assert_eq!(
        res.unwrap_err().to_string().lines().next().unwrap(),
        "unexpected error: Uncaught Error: Console.log is printing something that is too long. Max length for all logs in a single request is 3 bytes"
    );
    assert_eq!(client.logs(), "ok\n");
}

#[rstest]
#[tokio::test]
async fn set_response(server: TestServer) {
    let mut client = Client::new(server.socket_path());

    let res = client
        .execute_js(r#"LitActions.setResponse({response: "Not OK"})"#)
        .await
        .unwrap();
    assert_eq!(res.response, "Not OK");

    let res = client
        .execute_js(r#"Lit.Actions.setResponse({response: "OK"})"#)
        .await
        .unwrap();
    assert_eq!(res.response, "OK");
}

#[rstest]
#[tokio::test]
async fn set_response_length_limit(server: TestServer) {
    let mut client = ClientBuilder::default()
        .socket_path(server.socket_path())
        .max_response_length(5)
        .build()
        .unwrap();

    let res = client
        .execute_js(r#"LitActions.setResponse({response: "Too long"})"#)
        .await;

    assert_eq!(
        res.unwrap_err().to_string().lines().next().unwrap(),
        "unexpected error: Uncaught Error: Response is too long. Max length is 5 bytes"
    );
}

#[rstest]
#[tokio::test]
async fn js_params(server: TestServer) {
    let mut client = Client::new(server.socket_path());

    let code = indoc! {r#"
        LitActions.setResponse({response: hello})
        console.log(WORLD)
    "#};

    let res = client
        .execute_js(ExecutionOptions {
            code: Arc::new(code.to_string()),
            globals: Some(json!({"hello": "hello", "WORLD": [2, 0, 2, 3]})),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(res.response, "hello");
    assert_eq!(res.logs, "[ 2, 0, 2, 3 ]\n");
}

#[rstest]
#[tokio::test]
async fn auth_context(server: TestServer) {
    let mut client = Client::new(server.socket_path());

    let code = indoc! {r#"
        LitActions.setResponse({response: JSON.stringify(LitAuth)})
        console.log(Lit.Auth.actionIpfsIds)
    "#};

    let res = client
        .execute_js(ExecutionOptions {
            code: Arc::new(code.to_string()),
            action_ipfs_id: Some("some-ipfs-id".to_string()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(
        res.response,
        r#"{"actionIpfsIds":["some-ipfs-id"],"actionIpfsIdStack":["some-ipfs-id"],"authSigAddress":null,"authMethodContexts":[],"resources":[],"customAuthResource":""}"#
    );
    assert_eq!(res.logs, "[ \"some-ipfs-id\" ]\n");
}

#[rstest]
#[tokio::test]
async fn http_headers(server: TestServer) {
    let mut client = ClientBuilder::default()
        .socket_path(server.socket_path())
        .http_headers(BTreeMap::from([
            ("user-agent".to_string(), "lit_node".to_string()),
            ("X-Request-ID".to_string(), "some-id".to_string()),
            ("accept".to_string(), "text/plain, text/html".to_string()),
        ]))
        .build()
        .unwrap();

    // See https://docs.deno.com/deploy/api/runtime-headers
    let code = indoc! {r#"
        console.log(Lit.Headers)
        console.assert(LitHeaders.has("user-agent"), "LitHeaders.has")
        console.assert(LitHeaders.get("x-request-id") === "some-id", "LitHeaders.get")
        console.assert(Array.from(LitHeaders.keys()).length === 3, "LitHeaders.keys")
    "#};

    let res = client.execute_js(code).await.unwrap();

    assert_eq!(
        res.logs,
        indoc! {r#"
            Headers {
              accept: "text/plain, text/html",
              "user-agent": "lit_node",
              "x-request-id": "some-id"
            }
        "#},
    );
}

#[rstest]
#[tokio::test]
async fn fetch(server: TestServer) {
    let mut client = Client::new(server.socket_path());

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
            const temp = json.properties.periods[0].temperature
            LitActions.setResponse({{response: temp.toString()}})
            const text = await fetch("{uri}/forecast").then(r => r.text())
            console.log(text)
        }})()
        "#,
        uri = &mock_server.uri()
    };

    let res = client.execute_js(code).await.unwrap();

    assert_eq!(res.response, "92");
    assert_eq!(
        res.logs,
        "{\"properties\":{\"periods\":[{\"temperature\":92}]}}\n"
    );
    assert_eq!(res.fetch_count, 2);
}

#[rstest]
#[tokio::test]
async fn fetch_invocation_limit(server: TestServer) {
    let mut client = ClientBuilder::default()
        .socket_path(server.socket_path())
        .max_fetch_count(1)
        .build()
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let code = formatdoc! {r#"
        (async () => {{
            await fetch("{uri}")
            await fetch("{uri}")
        }})()
        "#,
        uri = &mock_server.uri()
    };

    let res = client.execute_js(code).await;

    assert_eq!(
        res.unwrap_err().to_string().lines().next().unwrap(),
        "unexpected error: Uncaught (in promise) Error: You may not send more than 1 HTTP requests per session and you have attempted to exceed that limit."
    );
}

#[rstest]
#[tokio::test]
// note about this test: the actionIpfsIds is broken, and only contains the current IPFS CID of the running action.
// We are retaining this for backwards compatibility in case existing apps depend on it.
// The actionIpfsIdStack is fixed, which contains the full stack of IPFS CIDs of the running actions.
// This test is used to verify that both the actionIpfsIds and actionIpfsIdStack is correctly populated.
async fn call_child(server: TestServer) {
    // Note: When updating any child code below, run this test again to get the new IPFS hash from the error message.
    let child1_ipfs_id = "QmTKj5hVSPx6RFWQMFVUwxinQ2QMurrikFtNciJCsxx8Tg";
    let child1_code = indoc! {r#"
        (async () => {
            console.log("child action #1, actionIpfsIds:", LitAuth.actionIpfsIds.join(","), ", actionIpfsIdStack:", LitAuth.actionIpfsIdStack.join(","))
            if (typeof ipfsId === "string") {
                await LitActions.call({ipfsId, params: {Link}})
            }
        })()
    "#};
    let child2_ipfs_id = "QmRz81aWn9ctxdUxeqMPm7dg5s8wMrwwLK8pTD92793dzV";
    let child2_code = indoc! {r#"
        (async () => {
            console.log("child action #2, actionIpfsIds:", LitAuth.actionIpfsIds.join(","), ", actionIpfsIdStack:", LitAuth.actionIpfsIdStack.join(","))
            LitActions.setResponse({response: "child"})
            await fetch(Link)
        })()
    "#};

    let ipfs_cache: Cache<String, Arc<String>> = Cache::new(4096);

    // Start fake IPFS server returning child code
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/ipfs/{child1_ipfs_id}")))
        .respond_with(ResponseTemplate::new(200).set_body_string(child1_code))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!("/ipfs/{child2_ipfs_id}")))
        .respond_with(ResponseTemplate::new(200).set_body_string(child2_code))
        .mount(&mock_server)
        .await;

    let env = models::DenoExecutionEnv {
        cfg: std::sync::Arc::new(
            lit_core::config::LitConfigBuilder::default()
                .set_override("ipfs.gateway", format!("{}/ipfs/{{}}", &mock_server.uri()))
                .set_override("lit.env", "dev")
                .build()
                .unwrap(),
        ),
        ipfs_cache: Some(ipfs_cache),
        ..Default::default()
    };

    let mut client = ClientBuilder::default()
        .socket_path(server.socket_path())
        .js_env(env.clone())
        .max_fetch_count(3)
        .build()
        .unwrap();

    let code = formatdoc! {r#"
        (async () => {{
            LitActions.setResponse({{response: "parent"}})
            await fetch("{uri}")
            console.log("start")
            await LitActions.call({{
                ipfsId: "{child2_ipfs_id}",
                params: {{ Link: "{uri}" }}
            }})
            await LitActions.call({{
                ipfsId: "{child1_ipfs_id}",
                params: {{ Link: "{uri}", ipfsId: "{child2_ipfs_id}" }}
            }})
           await LitActions.call({{
                ipfsId: "{child1_ipfs_id}",
                params: {{ Link: "{uri}", ipfsId: "{child1_ipfs_id}" }}
            }})
            console.log("end")
        }})()
        "#,
        uri = &mock_server.uri()
    };

    let res = client.execute_js(code.clone()).await.unwrap();

    assert_eq!(
        res,
        ExecutionState {
            response: "child".to_string(),
            logs: [
                "start",
                // Call #2
                &format!("child action #2, actionIpfsIds: {child2_ipfs_id} , actionIpfsIdStack: {child2_ipfs_id}"),
                // Call #1, which calls #2
                &format!("child action #1, actionIpfsIds: {child1_ipfs_id} , actionIpfsIdStack: {child1_ipfs_id}"),
                &format!("child action #2, actionIpfsIds: {child2_ipfs_id} , actionIpfsIdStack: {child1_ipfs_id},{child2_ipfs_id}"),
                // Call #1, which calls itself
                &format!("child action #1, actionIpfsIds: {child1_ipfs_id} , actionIpfsIdStack: {child1_ipfs_id}"),
                &format!("child action #1, actionIpfsIds: {child1_ipfs_id} , actionIpfsIdStack: {child1_ipfs_id},{child1_ipfs_id}"),
                "end",
            ]
            .join("\n")
                + "\n",
            fetch_count: 3,
            ops_count: 18,
            ..Default::default()
        }
    );

    // Test recursion limit using the same code
    {
        let mut client = ClientBuilder::default()
            .socket_path(server.socket_path())
            .js_env(env)
            .max_call_depth(1)
            .build()
            .unwrap();

        let res = client.execute_js(code).await;

        assert_eq!(
            res.unwrap_err().to_string().lines().next().unwrap(),
            "unexpected error: Uncaught (in promise) Error: Uncaught (in promise) Error: The recursion limit of a child action is 1 and you have attempted to exceed that limit."
        );
    }
}

#[rstest]
#[tokio::test]
async fn catch_op_errors(server: TestServer) {
    let mut client = ClientBuilder::default()
        .socket_path(server.socket_path())
        .max_fetch_count(0)
        .max_response_length(5)
        .build()
        .unwrap();

    let code = indoc! {r#"
        (async () => {
            try {
                await LitActions.signEcdsa({toSign: [1,2,3], publicKey: "some-key", sigName: "some-sig"})
            } catch (e) {
                console.error(e.toString())
            }
            try {
                await fetch("https://www.litprotocol.com")
            } catch (e) {
                console.error(e.toString())
            }
            try {
                LitActions.setResponse({response: "Too long"})
            } catch (e) {
                console.error(e.toString())
            }
        })()
    "#};

    let res = client.execute_js(code).await.unwrap();

    assert_eq!(
        res.logs.trim(),
        [
            "Error: No TSS state found",
            "Error: You may not send more than 0 HTTP requests per session and you have attempted to exceed that limit.",
            "Error: Response is too long. Max length is 5 bytes",
        ]
        .join("\n")
    );

    {
        let mut client = ClientBuilder::default()
            .socket_path(server.socket_path())
            .max_console_log_length(5)
            .build()
            .unwrap();

        let code = indoc! {r#"
            try {
                console.log("Too")
                console.log("long")
            } catch (e) {
                LitActions.setResponse({response: e.toString()})
            }
        "#};

        let res = client.execute_js(code).await.unwrap();

        assert_eq!(res.logs, "Too\n");
        assert_eq!(res.response, "Error: Console.log is printing something that is too long. Max length for all logs in a single request is 5 bytes");
    }
}

#[rstest]
#[tokio::test]
async fn timeout(server: TestServer) {
    let mut client = ClientBuilder::default()
        .socket_path(server.socket_path())
        .timeout_ms(500)
        .build()
        .unwrap();

    let code = indoc! {r#"
        while (true) {}
    "#};

    let res = client.execute_js(code).await;

    assert_eq!(
        res.unwrap_err().to_string(),
        "timeout error: Your function exceeded the maximum runtime of 500ms and was terminated."
    );
}

#[rstest]
#[tokio::test]
async fn oom(server: TestServer) {
    let mut client = ClientBuilder::default()
        .socket_path(server.socket_path())
        .memory_limit_mb(100)
        .build()
        .unwrap();

    let code = indoc! {r#"
        let s = ""
        while (true) {
            s += "Hello"
        }
    "#};

    let res = client.execute_js(code).await;

    assert_eq!(
        res.unwrap_err().to_string(),
        "memory limit exceeded: Your function exceeded the maximum memory of 100 MB and was terminated."
    );
}

#[tokio::test]
async fn server_down() {
    let socket = temp_file::empty();
    let mut client_without_server = ClientBuilder::default()
        .socket_path(socket.path())
        .max_retries(1) // speed up test
        .build()
        .unwrap();

    let res = client_without_server.execute_js("ignored").await;

    assert_eq!(res.unwrap_err().kind(), lit_core::error::Kind::Connect);
}

#[rstest]
#[tokio::test]
async fn max_code_length(server: TestServer) {
    let mut client = ClientBuilder::default()
        .socket_path(server.socket_path())
        .max_code_length(10)
        .build()
        .unwrap();

    let res = client.execute_js(r#"const n = 1000"#).await;

    assert_eq!(
        res.unwrap_err().to_string().lines().next().unwrap(),
        "unexpected error: Code payload is too large (14 bytes). Max length is 10 bytes."
    );
}
