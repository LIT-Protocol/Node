use crate::{
    new_node_collection,
    node_collection::{hit_endpoints_with_json_body, hit_endpoints_with_json_body_per_port},
    pkp::mint_next_pkp,
    testnet::actions::Actions,
    Testnet, ValidatorCollection,
};
use anyhow::Result;
use ethers::core::types::U256;
use ethers::utils::keccak256;
use lit_node::auth::auth_material::AuthSigItem;
use lit_node::auth::auth_material::JsonAuthSig;
use lit_node::models::AuthMethod;
use lit_node::models::JsonPKPSigningRequest;
use serde::{Deserialize, Serialize};

pub const INVALID_SESSION_SIG_LIT_ACTION_CODE: &str = r#"(async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    const sigShare = await LitActions.signEcdsa({ toSign, publicKey, sigName });
})();"#;

pub const VALID_SESSION_SIG_LIT_ACTION_CODE: &str = r#"
    // Works with an AuthSig AuthMethod
    if (Lit.Auth.authMethodContexts.some(e => e.authMethodType === 1)) {
        LitActions.setResponse({response:"true"});
    } else {
        LitActions.setResponse({response:"false"});
    }
"#;

pub const CUSTOM_AUTH_RESOURCE_VALID_SESSION_SIG_LIT_ACTION_CODE: &str = r#"
    if (Lit.Auth.authMethodContexts.some(e => e.authMethodType === 1)) {
        // Adds the Custom Auth Resource in the SessionSigs
        LitActions.setResponse({response:"(true, 'Anything your want to use in executeJs')"});
    } else {
        LitActions.setResponse({response:"false"});
    }
"#;

pub const VALID_PKP_SIGNING_LIT_ACTION_CODE: &str = r#"(async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    console.log("Lit.Auth", Lit.Auth);

    // Signs only when the sessionSig was created by the below Custom Lit Action Authentication
    if (Lit.Auth.actionIpfsIds.includes("QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz")) {
        const sigShare = await LitActions.signEcdsa({ toSign, publicKey, sigName });
    }
})();
"#;

pub const CUSTOM_AUTH_RESOURCE_VALID_PKP_SIGNING_LIT_ACTION_CODE: &str = r#"(async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    console.log("Lit.Auth-", Lit.Auth);
    const isValidCustomAuthResource = Lit.Auth.customAuthResource === "(true, 'Anything your want to use in executeJs')";
    console.log("isValidCustomAuthResource-", isValidCustomAuthResource);

    // Checks the custom auth resource returned in the SessionSigs
    if (Lit.Auth.actionIpfsIds.includes("QmRxUzYX52zEko9nvvtkdA6k8jU36enwwTVgW9ZwbdsUHY") && isValidCustomAuthResource) {
        console.log("Custom Authorization Successful!");
        const sigShare = await LitActions.signEcdsa({ toSign, publicKey, sigName });
    }
})();
"#;

pub const NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE: &str = r#"
    if (customAccessToken === 'lit') {
        LitActions.setResponse({response:"true"});
    }
"#;

pub const NO_AUTH_METHOD_PKP_SIGNING_LIT_ACTION_CODE: &str = r#"(async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    console.log("Lit.Auth", Lit.Auth);
    // Signs only when the sessionSig was created by the below Custom Lit Action Authentication
    if (Lit.Auth.actionIpfsIds.includes("QmWLP9ojXrHJrFHnvMJv12HScFoz7R8kcYAECjtcpaJM2Y")) {
        const sigShare = await LitActions.signEcdsa({ toSign, publicKey, sigName });
    }
})();
"#;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigningResponse {
    pub success: bool,
    pub signed_data: JsonSignSessionKeyResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyResponse {
    pub session_sig: JsonSignSessionKeyResponseShare,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyResponseShare {
    pub signature_share: String,
    pub share_index: u32,
    pub sig_type: String,
    pub siwe_message: String,
    pub data_signed: String,
    pub bigr: String,
    pub public_key: String,
    pub sig_name: String,
}

pub async fn init_test() -> (Testnet, ValidatorCollection) {
    crate::init_test_config();

    let num_nodes = 3;
    new_node_collection(num_nodes, false).await
}

pub async fn mint_pkp(actions: &Actions) -> ([u8; 20], String, U256) {
    let pkp_info = mint_next_pkp(actions).await.unwrap();
    let pubkey = pkp_info.0;
    let token_id = pkp_info.1;
    let eth_address = pkp_info.2;

    (eth_address, pubkey, token_id)
}

pub async fn get_pkp_sign(
    validator_collection: &ValidatorCollection,
    session_sigs: Option<Vec<JsonAuthSig>>,
    auth_sig: Option<AuthSigItem>,
    pass_as_auth_method: bool,
    to_sign: String,
    pubkey: String,
) -> Result<Vec<String>> {
    let cmd = "/web/pkp/sign/v1".to_string();

    if let Some(session_sigs) = session_sigs {
        let mut json_body_vec = Vec::new();

        // Generate JSON body for each port
        for i in 0..validator_collection.size() {
            let data_to_send = JsonPKPSigningRequest {
                auth_sig: AuthSigItem::Single(session_sigs[i].clone()),
                to_sign: keccak256(to_sign.as_bytes()).into(),
                pubkey: pubkey.clone(),
                auth_methods: None,
                epoch: 2, // Hardcoded as at other places in the tests
            };

            let json_body = serde_json::to_string(&data_to_send).unwrap();
            json_body_vec.push(json_body);
        }

        return Ok(hit_endpoints_with_json_body_per_port(
            validator_collection.actions(),
            cmd,
            json_body_vec,
        )
        .await);
    }

    if let Some(auth_sig) = auth_sig {
        let mut auth_methods = None;
        if pass_as_auth_method {
            auth_methods = Some(vec![AuthMethod {
                auth_method_type: 1,
                access_token: serde_json::to_string(&auth_sig).unwrap(),
            }]);
        }

        let data_to_send = JsonPKPSigningRequest {
            auth_sig,
            to_sign: keccak256(to_sign.as_bytes()).into(),
            pubkey: pubkey.clone(),
            auth_methods,
            epoch: 2, // Hardcoded as at other places in the tests
        };

        let json_body = serde_json::to_string(&data_to_send).unwrap();

        return Ok(
            hit_endpoints_with_json_body(validator_collection.actions(), cmd, json_body).await,
        );
    }

    Err(anyhow::anyhow!("Provide either an AuthSig or SessionSigs"))
}
