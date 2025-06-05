use std::collections::BTreeMap;

use anyhow::{bail, Result};
use indoc::{formatdoc, indoc};
use lit_actions_server::{init_v8, proto::*, unix, TestServer};
use pretty_assertions::assert_eq;
use rstest::*;
use temp_file::TempFile;
use tokio_stream::StreamExt as _;
use tonic::{Code, Request, Status};

// This client is only used for testing. The real implementation is provided by lit-node.
#[derive(Debug)]
struct TestClient {
    socket_file: TempFile,
    messages: gotham_store::GothamStore,
}

impl Drop for TestClient {
    fn drop(&mut self) {
        if !self.messages.is_empty() && !std::thread::panicking() {
            panic!(
                "GothamStore still contains {} type(s) to be inspected via `take`",
                self.messages.len()
            );
        }
    }
}

impl TestClient {
    fn new(socket_file: TempFile) -> Self {
        Self {
            socket_file,
            messages: Default::default(),
        }
    }

    async fn execute_js(
        &mut self,
        request: impl Into<ExecutionRequest>,
    ) -> Result<ExecutionResult> {
        let request = request.into();

        let (outbound_tx, outbound_rx) = flume::bounded(0);
        let channel = unix::connect_to_socket(self.socket_file.path()).await?;
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
                    self.messages.put(res.clone());
                    if !res.success {
                        bail!(res.error);
                    }
                    return Ok(res);
                }
                // Handle op requests
                Some(op) => {
                    let resp = if let Some(resp) = self.messages.try_take::<ErrorResponse>() {
                        resp.into()
                    } else {
                        self.handle_op(op)
                    };
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
            UnionResponse::SetResponse(req) => {
                self.messages.put(req);
                self.messages.take::<SetResponseResponse>().into()
            }
            UnionResponse::Print(req) => {
                self.messages.put(req);
                self.messages.take::<PrintResponse>().into()
            }
            UnionResponse::IncrementFetchCount(req) => {
                self.messages.put(req);
                self.messages.take::<IncrementFetchCountResponse>().into()
            }
            UnionResponse::PkpPermissionsGetPermitted(req) => {
                self.messages.put(req);
                self.messages
                    .take::<PkpPermissionsGetPermittedResponse>()
                    .into()
            }
            UnionResponse::PkpPermissionsGetPermittedAuthMethodScopes(req) => {
                self.messages.put(req);
                self.messages
                    .take::<PkpPermissionsGetPermittedAuthMethodScopesResponse>()
                    .into()
            }
            UnionResponse::PkpPermissionsIsPermitted(req) => {
                self.messages.put(req);
                self.messages
                    .take::<PkpPermissionsIsPermittedResponse>()
                    .into()
            }
            UnionResponse::PkpPermissionsIsPermittedAuthMethod(req) => {
                self.messages.put(req);
                self.messages
                    .take::<PkpPermissionsIsPermittedAuthMethodResponse>()
                    .into()
            }
            UnionResponse::PubkeyToTokenId(req) => {
                self.messages.put(req);
                self.messages.take::<PubkeyToTokenIdResponse>().into()
            }
            UnionResponse::SignEcdsa(req) => {
                self.messages.put(req);
                self.messages.take::<SignEcdsaResponse>().into()
            }
            UnionResponse::AesDecrypt(req) => {
                self.messages.put(req);
                self.messages.take::<AesDecryptResponse>().into()
            }
            UnionResponse::GetLatestNonce(req) => {
                self.messages.put(req);
                self.messages.take::<GetLatestNonceResponse>().into()
            }
            UnionResponse::CheckConditions(req) => {
                self.messages.put(req);
                self.messages.take::<CheckConditionsResponse>().into()
            }
            UnionResponse::ClaimKeyIdentifier(req) => {
                self.messages.put(req);
                self.messages.take::<ClaimKeyIdentifierResponse>().into()
            }
            UnionResponse::CallContract(req) => {
                self.messages.put(req);
                self.messages.take::<CallContractResponse>().into()
            }
            UnionResponse::CallChild(req) => {
                self.messages.put(req);
                self.messages.take::<CallChildResponse>().into()
            }
            UnionResponse::BroadcastAndCollect(req) => {
                self.messages.put(req);
                self.messages.take::<BroadcastAndCollectResponse>().into()
            }
            UnionResponse::DecryptAndCombine(req) => {
                self.messages.put(req);
                self.messages.take::<DecryptAndCombineResponse>().into()
            }
            UnionResponse::DecryptToSingleNode(req) => {
                self.messages.put(req);
                self.messages.take::<DecryptToSingleNodeResponse>().into()
            }
            UnionResponse::SignAndCombineEcdsa(req) => {
                self.messages.put(req);
                self.messages.take::<SignAndCombineEcdsaResponse>().into()
            }
            UnionResponse::GetRpcUrl(req) => {
                self.messages.put(req);
                self.messages.take::<GetRpcUrlResponse>().into()
            }
            UnionResponse::P2pBroadcast(req) => {
                self.messages.put(req);
                self.messages.take::<P2pBroadcastResponse>().into()
            }
            UnionResponse::P2pCollectFromLeader(req) => {
                self.messages.put(req);
                self.messages.take::<P2pCollectFromLeaderResponse>().into()
            }
            UnionResponse::IsLeader(req) => {
                self.messages.put(req);
                self.messages.take::<IsLeaderResponse>().into()
            }
            UnionResponse::EncryptBls(req) => {
                self.messages.put(req);
                self.messages.take::<EncryptBlsResponse>().into()
            }
            UnionResponse::Result(_) => unreachable!(), // handled in main loop
        }
    }

    fn respond_with<T: 'static>(&mut self, t: T) -> &mut Self {
        self.messages.put(t);
        self
    }

    fn received<T: 'static>(&mut self) -> T {
        self.messages.take::<T>()
    }
}

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

#[fixture]
fn client(server: TestServer) -> TestClient {
    // NB: Moving the socket file makes the client delete it once done without
    // having to deal with lifetimes all over the place.
    TestClient::new(server.socket_file)
}

#[rstest]
#[tokio::test]
async fn nop(mut client: TestClient) {
    let res = client.execute_js("// Do nothing").await.unwrap();

    assert_eq!(res.error, "");
    assert_eq!(res.success, true);

    assert_eq!(client.received::<ExecutionResult>(), res);
}

#[rstest]
#[tokio::test]
async fn console_log(mut client: TestClient) {
    client
        .respond_with(PrintResponse {})
        .execute_js(r#"console.log("Lit Actions!")"#)
        .await
        .unwrap();

    assert_eq!(client.received::<PrintRequest>().message, "Lit Actions!\n");
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn lit_namespace(mut client: TestClient) {
    let code = indoc! {r#"
        console.log(
            Object.keys(Lit.Actions).length > 0,
            Object.keys(Lit.Auth).length === 0,
            Object.keys(Lit.Headers).length === 0,

            Lit.Actions === LitActions,
            Lit.Actions === globalThis.LitActions,

            Lit.Auth === LitAuth,
            Lit.Auth === globalThis.LitAuth,

            Lit.Headers === LitHeaders,
            Lit.Headers === globalThis.LitHeaders,

            Lit === globalThis.Lit,
        )
    "#};

    client
        .respond_with(PrintResponse {})
        .execute_js(code)
        .await
        .unwrap();

    assert_eq!(
        client.received::<PrintRequest>().message,
        "true true true true true true true true true true\n"
    );
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn lit_namespace_protection(mut client: TestClient) {
    let code = indoc! {r#"
        "use strict";

        const errors = [];
        const run = (fn) => { try { fn() } catch(err) { errors.push(err) } };

        run(() => delete globalThis.Lit);
        run(() => delete globalThis.LitActions);
        run(() => delete globalThis.LitAuth);
        run(() => delete globalThis.LitHeaders);

        run(() => delete Lit.Actions);
        run(() => delete Lit.Actions.signEcdsa);
        run(() => delete Lit.Auth);
        run(() => delete Lit.Headers);

        run(() => Lit = {});
        run(() => LitActions = {});
        run(() => LitAuth = {});
        run(() => LitHeaders = {});

        run(() => Lit.Actions = {});
        run(() => Lit.Auth = {});
        run(() => Lit.Headers = {});

        console.log(errors.join('\n'))
    "#};

    client
        .respond_with(PrintResponse {})
        .execute_js(code)
        .await
        .unwrap();

    assert_eq!(
        client.received::<PrintRequest>().message.trim_end(),
        [
            "TypeError: Cannot delete property 'Lit' of #<Window>",
            "TypeError: Cannot delete property 'LitActions' of #<Window>",
            "TypeError: Cannot delete property 'LitAuth' of #<Window>",
            "TypeError: Cannot delete property 'LitHeaders' of #<Window>",
            "TypeError: Cannot delete property 'Actions' of #<Object>",
            "TypeError: Cannot delete property 'signEcdsa' of #<Object>",
            "TypeError: Cannot delete property 'Auth' of #<Object>",
            "TypeError: Cannot delete property 'Headers' of #<Object>",
            "TypeError: Cannot assign to read only property 'Lit' of object '#<Window>'",
            "TypeError: Cannot assign to read only property 'LitActions' of object '#<Window>'",
            "TypeError: Cannot assign to read only property 'LitAuth' of object '#<Window>'",
            "TypeError: Cannot assign to read only property 'LitHeaders' of object '#<Window>'",
            "TypeError: Cannot assign to read only property 'Actions' of object '#<Object>'",
            "TypeError: Cannot assign to read only property 'Auth' of object '#<Object>'",
            "TypeError: Cannot assign to read only property 'Headers' of object '#<Object>'",
        ]
        .join("\n")
    );
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn js_params(mut client: TestClient) {
    {
        let code = indoc! {r#"
            console.log(
                Hello === "hello",
                Hello === globalThis.Hello,
                Hello === this.Hello,

                WORLD.year === 2024,
                WORLD === globalThis.WORLD,
                WORLD === this.WORLD,
            )
        "#};

        client
            .respond_with(PrintResponse {})
            .execute_js(ExecutionRequest {
                code: code.into(),
                js_params: Some(b"{\"Hello\":\"hello\",\"WORLD\":{\"year\": 2024}}".into()),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(
            client.received::<PrintRequest>().message,
            "true true true true true true\n"
        );
        assert!(client.received::<ExecutionResult>().success);
    }

    // Check that the Lit namespace can't be modified
    {
        let res = client
            .execute_js(ExecutionRequest {
                code: "ignored".to_string(),
                js_params: Some(b"{\"Lit\":{\"Actions\": {}}}".into()),
                ..Default::default()
            })
            .await;

        assert_eq!(
            res.unwrap_err().to_string().lines().next().unwrap(),
            "Error building main worker: Error injecting params as globals: TypeError: Cannot assign to read only property 'Lit' of object '#<Window>'",
        );
        assert_eq!(client.received::<ExecutionResult>().success, false);
    }
}

#[rstest]
#[tokio::test]
async fn set_response(mut client: TestClient) {
    client
        .respond_with(SetResponseResponse {})
        .execute_js(r#"Lit.Actions.setResponse({response: "OK"})"#)
        .await
        .unwrap();

    assert_eq!(client.received::<SetResponseRequest>().response, "OK");
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn fetch(mut client: TestClient) {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let code = formatdoc! {r#"
        (async () => {{
            await fetch("{uri}")
        }})()
        "#,
        uri = &mock_server.uri()
    };

    client
        .respond_with(IncrementFetchCountResponse { fetch_count: 1 })
        .execute_js(code)
        .await
        .unwrap();

    assert_eq!(
        client.received::<IncrementFetchCountRequest>(),
        IncrementFetchCountRequest {}
    );
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn pkp_get_permitted(mut client: TestClient) {
    for method in [
        "getPermittedActions",
        "getPermittedAddresses",
        "getPermittedAuthMethods",
    ] {
        client
            .respond_with(PkpPermissionsGetPermittedResponse {
                resources: b"[]".into(), // ignored
            })
            .execute_js(format!(
                r#"(async () => {{ await LitActions.{method}({{tokenId: "0x1234"}}) }})()"#
            ))
            .await
            .unwrap();

        assert_eq!(
            client.received::<PkpPermissionsGetPermittedRequest>(),
            PkpPermissionsGetPermittedRequest {
                method: method.to_string(),
                token_id: "0x1234".to_string(),
            }
        );
        assert!(client.received::<ExecutionResult>().success);
    }

    // getPermittedAuthMethodScopes
    {
        client
            .respond_with(PkpPermissionsGetPermittedAuthMethodScopesResponse {
                scopes: vec![/* ignored */],
            })
            .execute_js(
                r#"(async () => { await LitActions.getPermittedAuthMethodScopes({tokenId: "0x1234", authMethodType: "5", userId: [1,2,3]}) })()"#,
            )
            .await
            .unwrap();

        assert_eq!(
            client.received::<PkpPermissionsGetPermittedAuthMethodScopesRequest>(),
            PkpPermissionsGetPermittedAuthMethodScopesRequest {
                token_id: "0x1234".to_string(),
                method: "5".to_string(),
                user_id: vec![1, 2, 3],
                max_scope_id: 100,
            }
        );
        assert!(client.received::<ExecutionResult>().success);
    }
}

#[rstest]
#[tokio::test]
async fn pkp_is_permitted(mut client: TestClient) {
    // isPermittedAction
    {
        client
            .respond_with(PkpPermissionsIsPermittedResponse { is_permitted: true })
            .execute_js(
                r#"(async () => { await LitActions.isPermittedAction({tokenId: "0x1234", ipfsId: "some-id"}) })()"#,
            )
            .await
            .unwrap();

        assert_eq!(
            client.received::<PkpPermissionsIsPermittedRequest>(),
            PkpPermissionsIsPermittedRequest {
                method: "isPermittedAction".to_string(),
                token_id: "0x1234".to_string(),
                params: b"[\"some-id\"]".into(),
            }
        );
        assert!(client.received::<ExecutionResult>().success);
    }

    // isPermittedAddress
    {
        client
            .respond_with(PkpPermissionsIsPermittedResponse { is_permitted: true })
            .execute_js(
                r#"(async () => { await LitActions.isPermittedAddress({tokenId: "0x1234", address: "some-address"}) })()"#,
            )
            .await
            .unwrap();

        assert_eq!(
            client.received::<PkpPermissionsIsPermittedRequest>(),
            PkpPermissionsIsPermittedRequest {
                method: "isPermittedAddress".to_string(),
                token_id: "0x1234".to_string(),
                params: b"[\"some-address\"]".into(),
            }
        );
        assert!(client.received::<ExecutionResult>().success);
    }

    // isPermittedAuthMethod
    {
        client
            .respond_with(PkpPermissionsIsPermittedAuthMethodResponse { is_permitted: true })
            .execute_js(
                r#"(async () => { await LitActions.isPermittedAuthMethod({tokenId: "0x1234", authMethodType: "5", userId: [1,2,3]}) })()"#,
            )
            .await
            .unwrap();

        assert_eq!(
            client.received::<PkpPermissionsIsPermittedAuthMethodRequest>(),
            PkpPermissionsIsPermittedAuthMethodRequest {
                token_id: "0x1234".to_string(),
                method: "5".to_string(),
                user_id: vec![1, 2, 3],
            }
        );
        assert!(client.received::<ExecutionResult>().success);
    }
}

#[rstest]
#[tokio::test]
async fn sign_ecdsa(mut client: TestClient) {
    // signEcdsa
    {
        client
            .respond_with(SignEcdsaResponse { success: "ignored".to_string() })
            .execute_js(
                r#"(async () => { await LitActions.signEcdsa({toSign: [1,2,3], publicKey: "some-key", sigName: "some-sig"}) })()"#,
            )
            .await
            .unwrap();

        assert_eq!(
            client.received::<SignEcdsaRequest>(),
            SignEcdsaRequest {
                to_sign: vec![1, 2, 3],
                public_key: "some-key".to_string(),
                sig_name: "some-sig".to_string(),
                eth_personal_sign: false,
            }
        );
        assert!(client.received::<ExecutionResult>().success);
    }

    // ethPersonalSignMessageEcdsa
    {
        client
            .respond_with(SignEcdsaResponse { success: "ignored".to_string() })
            .execute_js(
                r#"(async () => { await LitActions.ethPersonalSignMessageEcdsa({message: "some-message", publicKey: "some-key", sigName: "some-sig"}) })()"#,
            )
            .await
            .unwrap();

        assert_eq!(
            client.received::<SignEcdsaRequest>(),
            SignEcdsaRequest {
                to_sign: b"some-message".into(),
                public_key: "some-key".to_string(),
                sig_name: "some-sig".to_string(),
                eth_personal_sign: true,
            }
        );
        assert!(client.received::<ExecutionResult>().success);
    }

    // signEcdsa with invalid params
    {
        let res = client
            .execute_js(
                r#"(async () => { await LitActions.signEcdsa({toSign: [], publicKey: "some-key", sigName: "some-sig"}) })()"#,
            )
            .await;

        assert_eq!(
            res.unwrap_err().to_string().lines().next().unwrap(),
            "Uncaught (in promise) RangeError: toSign must not be empty"
        );
        assert!(!client.received::<ExecutionResult>().success);
    }
    {
        let res = client
            .execute_js(
                r#"(async () => { await LitActions.signEcdsa({toSign: [1,2,3], publicKey: " ", sigName: "some-sig"}) })()"#,
            )
            .await;

        assert_eq!(
            res.unwrap_err().to_string().lines().next().unwrap(),
            "Uncaught (in promise) RangeError: publicKey must not be blank"
        );
        assert!(!client.received::<ExecutionResult>().success);
    }
    {
        let res = client
            .execute_js(
                r#"(async () => { await LitActions.signEcdsa({toSign: [1,2,3], publicKey: "some-key"}) })()"#,
            )
            .await;

        assert_eq!(
            res.unwrap_err().to_string().lines().next().unwrap(),
            "Uncaught (in promise) RangeError: sigName must not be blank"
        );
        assert!(!client.received::<ExecutionResult>().success);
    }

    // catch signEcdsa op error
    {
        let code = indoc! {r#"
            (async () => {
                try {
                    await LitActions.signEcdsa({toSign: [1,2,3], publicKey: "some-key", sigName: "some-sig"})
                } catch (e) {
                    LitActions.setResponse({response: e.toString()})
                }
            })()
        "#};

        client
            .respond_with(ErrorResponse {
                error: "something went wrong".to_string(),
            })
            .respond_with(SetResponseResponse {})
            .execute_js(code)
            .await
            .unwrap();

        assert_eq!(
            client.received::<SetResponseRequest>(),
            SetResponseRequest {
                response: "Error: something went wrong".to_string(),
            }
        );
        assert!(client.received::<ExecutionResult>().success);
    }
}

#[rstest]
#[tokio::test]
async fn aes_decrypt(mut client: TestClient) {
    client
        .respond_with(AesDecryptResponse { plaintext: "ignored".to_string() })
        .execute_js(
            r#"(async () => { await LitActions.aesDecrypt({symmetricKey: [1,2,3], ciphertext: [4,5,6]}) })()"#,
        )
        .await
        .unwrap();

    assert_eq!(
        client.received::<AesDecryptRequest>(),
        AesDecryptRequest {
            symmetric_key: vec![1, 2, 3],
            ciphertext: vec![4, 5, 6],
        }
    );
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn check_conditions(mut client: TestClient) {
    client
        .respond_with(CheckConditionsResponse { success: true })
        .execute_js(
            r#"(async () => { await LitActions.checkConditions({conditions: [{conditionType: "some-type"}], authSig: {sig: "0x123"}, chain: "some-chain"}) })()"#,
        )
        .await
        .unwrap();

    assert_eq!(
        client.received::<CheckConditionsRequest>(),
        CheckConditionsRequest {
            conditions: b"[{\"conditionType\":\"some-type\"}]".into(),
            auth_sig: Some(b"{\"sig\":\"0x123\"}".into()),
            chain: Some("some-chain".to_string()),
        }
    );
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn get_latest_nonce(mut client: TestClient) {
    {
        client
            .respond_with(GetLatestNonceResponse {
                nonce: "ignored".to_string(),
            })
            .execute_js(
                r#"(async () => { await LitActions.getLatestNonce({address: "0x4ebde54ba404be158262fbf4901c276c400e4ae3", chain: "some-chain"}) })()"#,
            )
            .await
            .unwrap();

        assert_eq!(
            client.received::<GetLatestNonceRequest>(),
            GetLatestNonceRequest {
                address: "4ebde54ba404be158262fbf4901c276c400e4ae3".to_string(),
                chain: "some-chain".to_string(),
            }
        );
        assert!(client.received::<ExecutionResult>().success);
    }

    {
        let res = client
            .execute_js(
                r#"(async () => { await LitActions.getLatestNonce({address: "invalid", chain: "some-chain"}) })()"#,
            )
            .await;

        assert_eq!(
            res.unwrap_err().to_string().lines().next().unwrap(),
            "Uncaught (in promise) TypeError: invalid length 7, expected a (both 0x-prefixed or not) hex string or byte array containing 20 bytes",
        );
        assert_eq!(client.received::<ExecutionResult>().success, false);
    }
}

#[rstest]
#[tokio::test]
async fn pubkey_to_token_id(mut client: TestClient) {
    client
        .respond_with(PubkeyToTokenIdResponse {
            token_id: "ignored".to_string(),
        })
        .execute_js(r#"LitActions.pubkeyToTokenId({publicKey: "some-key"})"#)
        .await
        .unwrap();

    assert_eq!(
        client.received::<PubkeyToTokenIdRequest>(),
        PubkeyToTokenIdRequest {
            public_key: "some-key".to_string(),
        }
    );
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn call_child(mut client: TestClient) {
    {
        client
            .respond_with(CallChildResponse {
                response: "ignored".to_string(),
            })
            .execute_js(r#"(async () => { await LitActions.call({ipfsId: "some-id", params: {"foo": "bar"}}) })()"#)
            .await
            .unwrap();

        assert_eq!(
            client.received::<CallChildRequest>(),
            CallChildRequest {
                ipfs_id: "some-id".to_string(),
                params: Some(b"{\"foo\":\"bar\"}".into()),
            }
        );
        assert!(client.received::<ExecutionResult>().success);
    }

    // Omit params
    {
        client
            .respond_with(CallChildResponse {
                response: "ignored".to_string(),
            })
            .execute_js(r#"(async () => { await LitActions.call({ipfsId: "some-id"}) })()"#)
            .await
            .unwrap();

        assert_eq!(
            client.received::<CallChildRequest>(),
            CallChildRequest {
                ipfs_id: "some-id".to_string(),
                params: None,
            }
        );
        assert!(client.received::<ExecutionResult>().success);
    }
}

#[rstest]
#[tokio::test]
async fn call_contract(mut client: TestClient) {
    client
        .respond_with(CallContractResponse {
            result: "ignored".to_string(),
        })
        .execute_js(r#"(async () => { await LitActions.callContract({chain: "some-chain", txn: "some-txn"}) })()"#)
        .await
        .unwrap();

    assert_eq!(
        client.received::<CallContractRequest>(),
        CallContractRequest {
            chain: "some-chain".to_string(),
            txn: "some-txn".to_string(),
        }
    );
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn claim_key(mut client: TestClient) {
    client
        .respond_with(ClaimKeyIdentifierResponse {
            success: "ignored".to_string(),
        })
        .execute_js(r#"(async () => { await LitActions.claimKey({keyId: "some-id"}) })()"#)
        .await
        .unwrap();

    assert_eq!(
        client.received::<ClaimKeyIdentifierRequest>(),
        ClaimKeyIdentifierRequest {
            key_id: "some-id".to_string(),
        }
    );
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn jwt_decode(mut client: TestClient) {
    let code = indoc! {r#"
        const decoded = jwt.decode(token, { complete: true })
        LitActions.setResponse({response: decoded.payload.loggedInAs})
    "#};

    client
        .respond_with(SetResponseResponse {})
        .execute_js(ExecutionRequest {
            code: code.into(),
            js_params: Some(b"{\"token\":\"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJsb2dnZWRJbkFzIjoiYWRtaW4iLCJpYXQiOjE0MjI3Nzk2Mzh9.gzSraSYS8EXBxLN_oWnFSRgCzcmJmMjLiuyu5CSpyHI\"}".into()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(client.received::<SetResponseRequest>().response, "admin");
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn uint8arrays(mut client: TestClient) {
    let code = indoc! {r#"
        const hello = LitActions.uint8arrayFromString("hello")
        const world = LitActions.uint8arrayFromString("world", "utf8")
        console.log(LitActions.uint8arrayToString(hello), hello, LitActions.uint8arrayToString(world, "utf8"), world)
    "#};

    client
        .respond_with(PrintResponse {})
        .execute_js(code)
        .await
        .unwrap();

    assert_eq!(
        client.received::<PrintRequest>(),
        PrintRequest {
            message: "hello Uint8Array(5) [ 104, 101, 108, 108, 111 ] world Uint8Array(5) [ 119, 111, 114, 108, 100 ]\n".to_string(),
        }
    );
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn webcrypto(mut client: TestClient) {
    let code = indoc! {r#"
        (async () => {
            const data = new TextEncoder().encode("Hello, World!")
            const hashed = await crypto.subtle.digest("SHA-256", data)
            Lit.Actions.setResponse({response: hashed.byteLength.toString()})
        })()
    "#};

    client
        .respond_with(SetResponseResponse {})
        .execute_js(code)
        .await
        .unwrap();

    assert_eq!(client.received::<SetResponseRequest>().response, "32");
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn localstorage_shouldnt_panic(mut client: TestClient) {
    let code = indoc! {r#"
        localStorage.setItem("myApp", "Deno");
    "#};

    let res = client.execute_js(code).await;

    assert_eq!(
        res.unwrap_err().to_string().lines().next().unwrap(),
        "Uncaught NotSupported: LocalStorage is not supported in this context."
    );
    assert_eq!(client.received::<ExecutionResult>().success, false);
}

#[rstest]
#[tokio::test]
async fn web_worker_shouldnt_panic(mut client: TestClient) {
    let code = indoc! {r#"
        new Worker("file:///path/to/worker.js", {type: "module"});
    "#};

    let res = client.execute_js(code).await;

    assert_eq!(
        res.unwrap_err().to_string().lines().next().unwrap(),
        "Uncaught ReferenceError: Worker is not defined"
    );
    assert_eq!(client.received::<ExecutionResult>().success, false);
}

#[rstest]
#[tokio::test]
async fn async_await(mut client: TestClient) {
    let code = indoc! {r#"
        (async () => {
            const fulfilled = await Promise.all([
                LitActions.signEcdsa({toSign: [1,2,3], publicKey: "some-key", sigName: "some-sig"}),
                LitActions.aesDecrypt({symmetricKey: [1,2,3], ciphertext: [4,5,6]}),
                LitActions.setResponse({response: await "OK"})
            ])
            console.log(fulfilled)
        })()
    "#};

    for _ in 0..20 {
        client
            .respond_with(PrintResponse {})
            .respond_with(SignEcdsaResponse {
                success: "success".to_string(),
            })
            .respond_with(AesDecryptResponse {
                plaintext: "plaintext".to_string(),
            })
            .respond_with(SetResponseResponse {})
            .execute_js(code)
            .await
            .unwrap();

        assert_eq!(client.received::<SignEcdsaRequest>().sig_name, "some-sig");
        assert_eq!(
            client.received::<AesDecryptRequest>().ciphertext,
            vec![4, 5, 6]
        );
        assert_eq!(client.received::<SetResponseRequest>().response, "OK");
        assert_eq!(
            client.received::<PrintRequest>().message,
            "[ \"success\", \"plaintext\", null ]\n"
        );
        assert!(client.received::<ExecutionResult>().success);
    }
}

#[rstest]
#[tokio::test]
async fn reference_error(mut client: TestClient) {
    let res = client.execute_js("nonexisting_function()").await;

    assert_eq!(
        res.unwrap_err().to_string(),
        indoc! {r#"
            Uncaught ReferenceError: nonexisting_function is not defined
                at <user_provided_script>:1:1
        "#}
        .trim()
    );
    assert_eq!(client.received::<ExecutionResult>().success, false);
}

#[rstest]
#[tokio::test]
async fn throw_error(mut client: TestClient) {
    {
        let code = indoc! {r#"
            throw new Error("boom")
        "#};
        let res = client.execute_js(code).await;

        assert_eq!(
            res.unwrap_err().to_string(),
            indoc! {r#"
                Uncaught Error: boom
                    at <user_provided_script>:1:7
            "#}
            .trim(),
        );
        assert_eq!(client.received::<ExecutionResult>().success, false);
    }

    {
        let code = indoc! {r#"
            (async () => {
                throw new Error("boom")
            })()
        "#};
        let res = client.execute_js(code).await;

        assert_eq!(
            res.unwrap_err().to_string(),
            indoc! {r#"
                Uncaught (in promise) Error: boom
                    at <user_provided_script>:2:11
                    at <user_provided_script>:3:3
            "#}
            .trim(),
        );
        assert_eq!(client.received::<ExecutionResult>().success, false);
    }
}

#[rstest]
#[tokio::test]
async fn timeout(mut client: TestClient) {
    let code = indoc! {r#"
        while (true) {}
    "#};

    let res = client
        .execute_js(ExecutionRequest {
            code: code.into(),
            timeout: Some(500),
            ..Default::default()
        })
        .await;
    let status = res.unwrap_err().downcast::<Status>().unwrap();

    assert_eq!(status.code(), Code::DeadlineExceeded);
    assert_eq!(
        status.message(),
        "Your function exceeded the maximum runtime of 500ms and was terminated."
    );
}

#[rstest]
#[tokio::test]
async fn oom(mut client: TestClient) {
    let code = indoc! {r#"
        let s = ""
        while (true) {
            s += "Hello"
        }
    "#};

    let res = client
        .execute_js(ExecutionRequest {
            code: code.into(),
            memory_limit: Some(100),
            ..Default::default()
        })
        .await;
    let status = res.unwrap_err().downcast::<Status>().unwrap();

    assert_eq!(status.code(), Code::ResourceExhausted);
    assert_eq!(
        status.message(),
        "Your function exceeded the maximum memory of 100 MB and was terminated."
    );
}

#[tokio::test]
async fn server_down() {
    let mut client_without_server = TestClient::new(temp_file::empty());

    let res = client_without_server.execute_js("ignored").await;

    assert_eq!(res.unwrap_err().to_string(), "transport error");
}

// You can run this test with `cargo test -- --ignored`, but it will fail due to
// SIGTRAP (UB of panicking in V8's C code), which can't be handled by #[should_panic].
#[rstest]
#[tokio::test]
#[ignore]
async fn panic_in_op(mut client: TestClient) {
    client
        .execute_js(r#"LitTest.op_panic("boom")"#)
        .await
        .unwrap();
}

// This includes an example for every Deno permission class except --allow-net,
// which is the only permission allowed (see fetch test).
// You can test what's allowed and denied by default via `deno repl --no-prompt`.
#[rstest]
#[tokio::test]
async fn deno_permissions(mut client: TestClient) {
    let tests = BTreeMap::from([
        (
            r#"Deno.readFileSync("test.txt")"#,
            r#"Uncaught NotCapable: Requires read access to "test.txt", run again with the --allow-read flag"#,
        ),
        (
            r#"Deno.makeTempDirSync()"#,
            r#"Uncaught NotCapable: Requires write access to <TMP>, run again with the --allow-write flag"#,
        ),
        (
            r#"Deno.env.get("SHELL")"#,
            r#"Uncaught NotCapable: Requires env access to "SHELL", run again with the --allow-env flag"#,
        ),
        (
            r#"Deno.hostname()"#,
            r#"Uncaught NotCapable: Requires sys access to "hostname", run again with the --allow-sys flag"#,
        ),
        (
            r#"Deno.kill(1234)"#,
            r#"Uncaught NotCapable: Requires run access, run again with the --allow-run flag"#,
        ),
        (
            r#"Deno.dlopen("test.dll", {})"#,
            r#"Uncaught NotCapable: Requires ffi access to "test.dll", run again with the --allow-ffi flag"#,
        ),
    ]);

    for (code, expected_err) in tests {
        let res = client.execute_js(code).await;
        assert_eq!(
            res.unwrap_err().to_string().lines().next().unwrap(),
            expected_err
        );
        assert_eq!(client.received::<ExecutionResult>().success, false);
    }
}

// Make sure we don't expose any of these versions:
// > Deno.version
// { deno: "1.41.3", v8: "12.3.219.9", typescript: "5.3.3" }
#[rstest]
#[tokio::test]
async fn deno_version(mut client: TestClient) {
    client
        .respond_with(PrintResponse {})
        .execute_js(r#"console.log(Deno.version)"#)
        .await
        .unwrap();

    assert_eq!(client.received::<PrintRequest>().message, "undefined\n");
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn deno_exit(mut client: TestClient) {
    for code in ["Deno.exit(1)", "Deno.exit(0)", "Deno.exit()"] {
        let res = client.execute_js(code).await;

        assert_eq!(
            res.unwrap_err().to_string().lines().next().unwrap(),
            "Uncaught PermissionDenied: 'Deno.exit' is not allowed in this context."
        );
        assert_eq!(client.received::<ExecutionResult>().success, false);
    }
}

#[rstest]
#[tokio::test]
async fn broadcast_and_collect(mut client: TestClient) {
    let code = indoc! {r#"
        (async () => {
            const resp = await Lit.Actions.broadcastAndCollect({name: "some-name", value: "some-value"})
            Lit.Actions.setResponse({response: JSON.stringify(resp)})
        })()
    "#};

    client
        .respond_with(BroadcastAndCollectResponse {
            name: "some-name".to_string(),
            values: vec!["some-response".to_string()],
        })
        .respond_with(SetResponseResponse {})
        .execute_js(code)
        .await
        .unwrap();

    assert_eq!(
        client.received::<BroadcastAndCollectRequest>(),
        BroadcastAndCollectRequest {
            name: "some-name".to_string(),
            value: "some-value".to_string(),
        }
    );
    assert_eq!(
        client.received::<SetResponseRequest>().response,
        "[\"some-response\"]"
    );
    assert!(client.received::<ExecutionResult>().success);
}

#[rstest]
#[tokio::test]
async fn wasm(mut client: TestClient) {
    // A simple add function in WebAssembly text format compiled using:
    // wat2wasm add.wat --output=- | xxd -i
    //
    // (module
    //   (func (export "add") (param $a i32) (param $b i32) (result i32)
    //     local.get $a
    //     local.get $b
    //     i32.add
    //   )
    // )
    //
    // Source: https://docs.deno.com/runtime/reference/wasm/
    let code = indoc! {r#"
        const wasmCode = new Uint8Array([
            0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60,
            0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01,
            0x03, 0x61, 0x64, 0x64, 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20,
            0x00, 0x20, 0x01, 0x6a, 0x0b
        ]);
        const wasmModule = new WebAssembly.Module(wasmCode);
        const wasmInstance = new WebAssembly.Instance(wasmModule);
        const { add } = wasmInstance.exports;
        console.log(add(123, 456));
    "#};

    client
        .respond_with(PrintResponse {})
        .execute_js(code)
        .await
        .unwrap();

    assert_eq!(client.received::<PrintRequest>().message, "579\n");
    assert!(client.received::<ExecutionResult>().success);
}
