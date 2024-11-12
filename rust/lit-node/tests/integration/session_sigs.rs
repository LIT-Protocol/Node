use crate::acceptance::web_user_tests::{
    assert_decrypted, prepare_test_encryption_parameters_with_wallet_address,
    retrieve_decryption_key, retrieve_decryption_key_session_sigs,
};
use anyhow::Result;
use blsful::Bls12381G2Impl;
use ethers::signers::LocalWallet;
use ethers::signers::Signer;
use ethers::types::U256;
use lit_node::auth::auth_material::AuthMaterialType;
use lit_node::auth::auth_material::AuthSigItem;
use lit_node::auth::lit_resource::LitResource;
use lit_node::auth::resources::AccessControlConditionResource;
use lit_node::models::auth::LitResourcePrefix;
use lit_node::models::auth::{
    LitAbility, LitResourceAbilityRequest, LitResourceAbilityRequestResource,
};
use lit_node::models::AuthMethod;
use lit_node::models::JsonSignSessionKeyResponseV1;
use lit_node::models::RequestConditions;
use lit_node::pkp::auth::get_user_wallet_auth_method_from_address;
use lit_node::pkp::auth::AuthMethodScope;
use lit_node::tss::dkg::curves::common::CurveType;
use lit_node::utils::encoding;
use lit_node::utils::web::hash_access_control_conditions;
use lit_node::utils::web::EndpointVersion;
use rand_core::OsRng;
use serde::Deserialize;
use test_common::auth_sig::get_auth_sig_for_session_sig_from_nodes;
use test_common::auth_sig::get_session_sigs_for_auth;
use test_common::auth_sig::get_session_sigs_for_pkp;
use test_common::lit_actions::HELLO_WORLD_LIT_ACTION_CODE;
use test_common::lit_actions::{
    assert_signed_action, execute_lit_action, execute_lit_action_session_sigs, sign_lit_action,
};
use test_common::node_collection::get_network_pubkey;
use test_common::pkp::add_permitted_action_to_pkp;
use test_common::pkp::{add_permitted_address_auth_method_to_pkp, add_permitted_address_to_pkp};
use test_common::{
    auth_sig::{generate_authsig, generate_authsig_item},
    session_sigs::{
        get_pkp_sign, init_test, mint_pkp, CUSTOM_AUTH_RESOURCE_VALID_PKP_SIGNING_LIT_ACTION_CODE,
        CUSTOM_AUTH_RESOURCE_VALID_SESSION_SIG_LIT_ACTION_CODE,
        INVALID_SESSION_SIG_LIT_ACTION_CODE, NO_AUTH_METHOD_PKP_SIGNING_LIT_ACTION_CODE,
        NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE, VALID_PKP_SIGNING_LIT_ACTION_CODE,
        VALID_SESSION_SIG_LIT_ACTION_CODE,
    },
};
use tracing::info;

#[derive(Deserialize)]
struct FailureResponse {
    success: bool,
    error: String,
}

#[doc = "Test that users can run a Lit Action before signing the sessionSig. Also test that you can't `signEcdsa` in that Lit Action."]
#[tokio::test]
async fn sign_session_sig_with_lit_actions() {
    let (testnet, validator_collection) = init_test().await;

    let wallet = testnet.deploy_account.signing_provider.signer();
    let auth_sig = generate_authsig_item(wallet).await.unwrap();

    let (eth_address, pubkey, _token_id) = mint_pkp(&validator_collection.actions()).await;

    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    info!("Starting test: only_sign_session_sig_with_lit_actions");

    let lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    let resource = "Threshold/Signing".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::PKP);

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &validator_collection.actions(),
        &auth_sig,
        false,
        &eth_address,
        &pubkey,
        session_pub_key.clone(),
        CurveType::BLS,
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        Some(lit_action_code),
        Some(serde_json::Value::Object(js_params.clone())),
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let responses = signing_resp.unwrap();
    for response in &responses {
        let response: JsonSignSessionKeyResponseV1 = serde_json::from_str(response).unwrap();
        assert_eq!(response.result, "success");

        let siwe_message = serde_json::to_string(&response.siwe_message).unwrap();
        assert!(siwe_message.contains("'Threshold': 'Signing' for 'lit-pkp://*'")); // should contain user defined resources
                                                                                    // TODO: Can add assertions for specific fields in the below resource like the actionIpfsIds should be: [QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz] & the userId should be: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
        assert!(siwe_message.contains("'Auth': 'Auth' for 'lit-resolvedauthcontext://*'"));
        // should contain resolved authContext resources
    }

    info!("Starting test: failed_ecdsa_sign_session_sig_with_lit_actions");

    let lit_action_code =
        data_encoding::BASE64.encode(INVALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    let signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &validator_collection.actions(),
        &auth_sig,
        false,
        &eth_address,
        &pubkey,
        session_pub_key,
        CurveType::BLS,
        vec![resource],
        vec![resource_prefix],
        Some(lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let responses = signing_resp.unwrap();
    for response in &responses {
        let response: FailureResponse = serde_json::from_str(response).unwrap();
        assert_eq!(response.success, false);
        assert!(response
            .error
            .contains("You can not sign without providing an auth_sig."));
    }
}

#[doc = "Test that non-permitted users can't run any random Lit Action they want before signing the sessionSig."]
#[tokio::test]
async fn only_permitted_lit_action_can_sign_session_sig() {
    let (_testnet, validator_collection) = init_test().await;

    let non_owner_wallet = LocalWallet::new(&mut OsRng);
    let auth_sig = generate_authsig_item(&non_owner_wallet).await.unwrap();

    let (eth_address, pubkey, _token_id) = mint_pkp(&validator_collection.actions()).await;

    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    info!("Starting test: only_permitted_lit_action_can_sign_session_sig");

    let lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes()); // This Lit Action hasn't been permitted so only the owner AuthSig can sign inside it

    let resource = "Threshold/Signing".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::PKP);

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &validator_collection.actions(),
        &auth_sig,
        false,
        &eth_address,
        &pubkey,
        session_pub_key.clone(),
        CurveType::BLS,
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        Some(lit_action_code),
        Some(serde_json::Value::Object(js_params.clone())),
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let responses = signing_resp.unwrap();
    for response in &responses {
        assert!(response.contains(
            "None of the AuthMethods, AuthSig or Lit Actions meet the requires scope [2]"
        ));
    }
}

#[doc = "Custom Authentication: Test that permitted Lit Action is allowed to create a sessionSig & then sign with the PKP. To test this we use a Random wallet that doesn't own the PKP as the auth_method."]
#[tokio::test]
async fn sign_pkp_with_lit_action_session_sigs() {
    info!("Starting test: sign_pkp_with_lit_action_session_sigs");

    let (_testnet, validator_collection) = init_test().await;

    let (eth_address, pubkey, token_id) = mint_pkp(&validator_collection.actions()).await;

    let lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    assert!(add_permitted_action_to_pkp(
        &validator_collection.actions(),
        "QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz", // IPFS CID for `VALID_SESSION_SIG_LIT_ACTION_CODE`
        token_id,
        &[U256::from(AuthMethodScope::SignAnything as usize)]
    )
    .await
    .unwrap());

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs = get_session_sigs_for_pkp(
        &validator_collection.actions(),
        pubkey.clone(),
        ethers::types::H160(eth_address),
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::PKP.to_string(),
            },
            ability: LitAbility::PKPSigning.to_string(),
        }],
        &validator_collection.addresses(),
        non_owner_wallet,
        None,
        CurveType::BLS,
        Some(lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
    )
    .await
    .expect("Could not get session sigs");

    let pkp_signing_resp = get_pkp_sign(
        &validator_collection,
        Some(session_sigs),
        None,
        false,
        "Hello Lit".to_string(),
        pubkey,
    )
    .await;

    let signed_data = "[134,95,114,41,198,115,171,24,245,116,158,255,141,16,61,47,54,189,142,61,205,85,131,39,97,86,253,25,102,251,205,246]"; // Hello Lit encoded
    for resp in pkp_signing_resp.unwrap() {
        assert!(resp.contains(signed_data));
        assert!(resp.contains("\"success\":true"));
    }
}

#[doc = "Custom Authorization: Test that permitted Lit Action is allowed to create a sessionSig & only the other permitted Lit Action is allowed to sign with the PKP. To test this we use a Random wallet that doesn't own the PKP as the auth_method."]
#[tokio::test]
async fn sign_lit_actions_with_lit_action_session_sig() {
    info!("Starting test: sign_lit_actions_with_lit_action_session_sig");

    let (_testnet, validator_collection) = init_test().await;

    let (eth_address, pubkey, token_id) = mint_pkp(&validator_collection.actions()).await;

    let session_sig_lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    // For signing Session Key i.e. Personal Message
    assert!(add_permitted_action_to_pkp(
        &validator_collection.actions(),
        "QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz", // IPFS CID for `VALID_SESSION_SIG_LIT_ACTION_CODE`
        token_id,
        &[U256::from(AuthMethodScope::SignPersonalMessage as usize)]
    )
    .await
    .unwrap());

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs = get_session_sigs_for_pkp(
        &validator_collection.actions(),
        pubkey.clone(),
        ethers::types::H160(eth_address),
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        &validator_collection.addresses(),
        non_owner_wallet,
        None,
        CurveType::BLS,
        Some(session_sig_lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
    )
    .await
    .expect("Could not get session sigs");

    // For signing inside Lit Actions i.e. signing anything
    assert!(add_permitted_action_to_pkp(
        &validator_collection.actions(),
        "QmV9dkmhpDqABXZDup6GN6VNrtpXRWitCLCBdaaGDcxXan", // IPFS CID for `VALID_PKP_SIGNING_LIT_ACTION_CODE`
        token_id,
        &[U256::from(AuthMethodScope::SignAnything as usize)]
    )
    .await
    .unwrap());

    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        sign_lit_action(VALID_PKP_SIGNING_LIT_ACTION_CODE.to_string(), pubkey)
            .await
            .expect("Could not get lit action params");

    let execute_resp = execute_lit_action_session_sigs(
        &validator_collection,
        lit_action_code,
        ipfs_id, // None
        js_params,
        auth_methods, // None
        &session_sigs,
        EndpointVersion::V1,
        2,
    )
    .await
    .expect("Could not execute lit action");

    let action_result = assert_signed_action(&validator_collection, execute_resp).await;
    assert!(action_result.is_ok());

    let action_result = action_result.unwrap();
    assert!(
        action_result == true,
        "The action should have returned true"
    );
}

#[doc = "Can't sign without being permitted despite having run a Lit Action sessionSig."]
#[tokio::test]
async fn only_permitted_can_sign_with_lit_action_session_sig() {
    info!("Starting test: only_permitted_can_sign_with_lit_action_session_sig");

    let (_testnet, validator_collection) = init_test().await;

    let (eth_address, pubkey, token_id) = mint_pkp(&validator_collection.actions()).await;

    let session_sig_lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    // For signing Session Key i.e. Personal Message
    assert!(add_permitted_action_to_pkp(
        &validator_collection.actions(),
        "QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz", // IPFS CID for `VALID_SESSION_SIG_LIT_ACTION_CODE`
        token_id,
        &[U256::from(AuthMethodScope::SignPersonalMessage as usize)]
    )
    .await
    .unwrap());

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs = get_session_sigs_for_pkp(
        &validator_collection.actions(),
        pubkey.clone(),
        ethers::types::H160(eth_address),
        vec![
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::LA.to_string(),
                },
                ability: LitAbility::LitActionExecution.to_string(),
            },
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::PKP.to_string(),
                },
                ability: LitAbility::PKPSigning.to_string(),
            },
        ],
        &validator_collection.addresses(),
        non_owner_wallet,
        None,
        CurveType::BLS,
        Some(session_sig_lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
    )
    .await
    .expect("Could not get session sigs");

    let (lit_action_code, ipfs_id, js_params, auth_methods) = sign_lit_action(
        VALID_PKP_SIGNING_LIT_ACTION_CODE.to_string(),
        pubkey.clone(),
    )
    .await
    .expect("Could not get lit action params");

    let execute_resp = execute_lit_action_session_sigs(
        &validator_collection,
        lit_action_code,
        ipfs_id, // None
        js_params,
        auth_methods, // None
        &session_sigs,
        EndpointVersion::V1,
        2,
    )
    .await
    .expect("Could not execute lit action");

    for resp in execute_resp {
        let response: FailureResponse = serde_json::from_str(&resp).unwrap();
        assert_eq!(response.success, false);
        assert!(response.error.contains(
            "None of the AuthMethods, AuthSig or Lit Actions meet the requires scope [1]"
        ));
    }

    let pkp_signing_resp = get_pkp_sign(
        &validator_collection,
        Some(session_sigs),
        None,
        false,
        "Hello Lit".to_string(),
        pubkey,
    )
    .await;

    for resp in pkp_signing_resp.unwrap() {
        assert!(resp.contains(
            "None of the AuthMethods, AuthSig or Lit Actions meet the requires scope [1]"
        ));
    }
}

#[doc = "Custom Authorization: Return Custom Auth Resource in executeJs. Test that permitted Lit Action is allowed to create a sessionSig & only the other permitted Lit Action is allowed to sign with the PKP. To test this we use a Random wallet that doesn't own the PKP as the auth_method."]
#[tokio::test]
async fn sign_lit_actions_with_custom_auth_resource_lit_action_session_sig() {
    info!("Starting test: sign_lit_actions_with_custom_auth_resource_lit_action_session_sig");

    let (_testnet, validator_collection) = init_test().await;

    let (eth_address, pubkey, token_id) = mint_pkp(&validator_collection.actions()).await;

    let session_sig_lit_action_code = data_encoding::BASE64.encode(
        CUSTOM_AUTH_RESOURCE_VALID_SESSION_SIG_LIT_ACTION_CODE
            .to_string()
            .as_bytes(),
    );

    // For signing Session Key i.e. Personal Message
    assert!(add_permitted_action_to_pkp(
        &validator_collection.actions(),
        "QmRxUzYX52zEko9nvvtkdA6k8jU36enwwTVgW9ZwbdsUHY", // IPFS CID for `CUSTOM_AUTH_RESOURCE_VALID_SESSION_SIG_LIT_ACTION_CODE`
        token_id,
        &[U256::from(AuthMethodScope::SignPersonalMessage as usize)]
    )
    .await
    .unwrap());

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs = get_session_sigs_for_pkp(
        &validator_collection.actions(),
        pubkey.clone(),
        ethers::types::H160(eth_address),
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        &validator_collection.addresses(),
        non_owner_wallet,
        None,
        CurveType::BLS,
        Some(session_sig_lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
    )
    .await
    .expect("Could not get session sigs");

    // For signing inside Lit Actions i.e. signing anything
    assert!(add_permitted_action_to_pkp(
        &validator_collection.actions(),
        "QmPJ36XTrrzhfXSuvgBLo7HwZpUiNmeBex6rv9GLYXopM5", // IPFS CID for `CUSTOM_AUTH_RESOURCE_VALID_PKP_SIGNING_LIT_ACTION_CODE`
        token_id,
        &[U256::from(AuthMethodScope::SignAnything as usize)]
    )
    .await
    .unwrap());

    let (lit_action_code, ipfs_id, js_params, auth_methods) = sign_lit_action(
        CUSTOM_AUTH_RESOURCE_VALID_PKP_SIGNING_LIT_ACTION_CODE.to_string(),
        pubkey,
    )
    .await
    .expect("Could not get lit action params");

    let execute_resp = execute_lit_action_session_sigs(
        &validator_collection,
        lit_action_code,
        ipfs_id, // None
        js_params,
        auth_methods, // None
        &session_sigs,
        EndpointVersion::V1,
        2,
    )
    .await
    .expect("Could not execute lit action");

    let action_result = assert_signed_action(&validator_collection, execute_resp).await;
    assert!(action_result.is_ok());

    let action_result = action_result.unwrap();
    assert!(
        action_result == true,
        "The action should have returned true"
    );
}

#[doc = "Custom Authorization: Create sessionSig with Custom Auth without providing any AuthMethods. Test pkpSign is allowed to it."]
#[tokio::test]
async fn sign_pkp_with_no_auth_method_lit_action_session_sig() {
    info!("Starting test: sign_pkp_with_no_auth_method_lit_action_session_sig");

    let (_testnet, validator_collection) = init_test().await;

    let (eth_address, pubkey, token_id) = mint_pkp(&validator_collection.actions()).await;

    let session_sig_lit_action_code = data_encoding::BASE64.encode(
        NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE
            .to_string()
            .as_bytes(),
    );

    assert!(add_permitted_action_to_pkp(
        &validator_collection.actions(),
        "QmWLP9ojXrHJrFHnvMJv12HScFoz7R8kcYAECjtcpaJM2Y", // IPFS CID for `NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE`
        token_id,
        &[U256::from(AuthMethodScope::SignAnything as usize)]
    )
    .await
    .unwrap());

    let mut js_params = serde_json::Map::new();
    js_params.insert("customAccessToken".to_string(), "lit".to_string().into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs = get_session_sigs_for_pkp(
        &validator_collection.actions(),
        pubkey.clone(),
        ethers::types::H160(eth_address),
        vec![
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::LA.to_string(),
                },
                ability: LitAbility::LitActionExecution.to_string(),
            },
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::PKP.to_string(),
                },
                ability: LitAbility::PKPSigning.to_string(),
            },
        ],
        &validator_collection.addresses(),
        non_owner_wallet,
        None,
        CurveType::BLS,
        Some(session_sig_lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
    )
    .await
    .expect("Could not get session sigs");

    let pkp_signing_resp = get_pkp_sign(
        &validator_collection,
        Some(session_sigs),
        None,
        false,
        "Hello Lit".to_string(),
        pubkey,
    )
    .await;

    let signed_data = "[134,95,114,41,198,115,171,24,245,116,158,255,141,16,61,47,54,189,142,61,205,85,131,39,97,86,253,25,102,251,205,246]"; // Hello Lit encoded
    for resp in pkp_signing_resp.unwrap() {
        assert!(resp.contains(signed_data));
        assert!(resp.contains("\"success\":true"));
    }
}

#[doc = "Custom Authorization: Create sessionSig with Custom Auth without providing any AuthMethods. Use it to executeJs and sign within that Lit Action."]
#[tokio::test]
async fn sign_lit_actions_with_no_auth_method_lit_action_session_sig() {
    info!("Starting test: sign_lit_actions_with_no_auth_method_lit_action_session_sig");

    let (_testnet, validator_collection) = init_test().await;

    let (eth_address, pubkey, token_id) = mint_pkp(&validator_collection.actions()).await;

    let session_sig_lit_action_code = data_encoding::BASE64.encode(
        NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE
            .to_string()
            .as_bytes(),
    );

    // For signing Session Key i.e. Personal Message
    assert!(add_permitted_action_to_pkp(
        &validator_collection.actions(),
        "QmWLP9ojXrHJrFHnvMJv12HScFoz7R8kcYAECjtcpaJM2Y", // IPFS CID for `NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE`
        token_id,
        &[U256::from(AuthMethodScope::SignPersonalMessage as usize)]
    )
    .await
    .unwrap());

    let mut js_params = serde_json::Map::new();
    js_params.insert("customAccessToken".to_string(), "lit".to_string().into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs = get_session_sigs_for_pkp(
        &validator_collection.actions(),
        pubkey.clone(),
        ethers::types::H160(eth_address),
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        &validator_collection.addresses(),
        non_owner_wallet,
        None,
        CurveType::BLS,
        Some(session_sig_lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
    )
    .await
    .expect("Could not get session sigs");

    // For signing inside Lit Actions i.e. signing anything
    assert!(add_permitted_action_to_pkp(
        &validator_collection.actions(),
        "QmY9Z2zAMhNG3r2gxiid39WdtJs5WswdpzWrZ8CkV9sWgC", // IPFS CID for `NO_AUTH_METHOD_PKP_SIGNING_LIT_ACTION_CODE`
        token_id,
        &[U256::from(AuthMethodScope::SignAnything as usize)]
    )
    .await
    .unwrap());

    let (lit_action_code, ipfs_id, js_params, auth_methods) = sign_lit_action(
        NO_AUTH_METHOD_PKP_SIGNING_LIT_ACTION_CODE.to_string(),
        pubkey,
    )
    .await
    .expect("Could not get lit action params");

    let execute_resp = execute_lit_action_session_sigs(
        &validator_collection,
        lit_action_code,
        ipfs_id, // None
        js_params,
        auth_methods, // None
        &session_sigs,
        EndpointVersion::V1,
        2,
    )
    .await
    .expect("Could not execute lit action");

    let action_result = assert_signed_action(&validator_collection, execute_resp).await;
    assert!(action_result.is_ok());

    let action_result = action_result.unwrap();
    assert!(
        action_result == true,
        "The action should have returned true"
    );
}

#[doc = "Test pkpSign with EOA sessionSig."]
#[tokio::test]
async fn sign_pkp_with_eoa_session_sigs() {
    info!("Starting test: sign_pkp_with_eoa_session_sigs");

    let (testnet, validator_collection) = init_test().await;
    let wallet = testnet.deploy_account.signing_provider.signer();

    let (_eth_address, pubkey, _token_id) = mint_pkp(&validator_collection.actions()).await;

    let session_sigs = get_session_sigs_for_auth(
        vec![
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::PKP.to_string(),
                },
                ability: LitAbility::PKPSigning.to_string(),
            },
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::LA.to_string(),
                },
                ability: LitAbility::LitActionExecution.to_string(),
            },
        ],
        &validator_collection.addresses(),
        Some(wallet.clone()),
        None,
    );

    let pkp_signing_resp = get_pkp_sign(
        &validator_collection,
        Some(session_sigs),
        None,
        false,
        "Hello Lit".to_string(),
        pubkey,
    )
    .await;

    let signed_data = "[134,95,114,41,198,115,171,24,245,116,158,255,141,16,61,47,54,189,142,61,205,85,131,39,97,86,253,25,102,251,205,246]"; // Hello Lit encoded
    for resp in pkp_signing_resp.unwrap() {
        assert!(resp.contains(signed_data));
        assert!(resp.contains("\"success\":true"));
    }
}

#[doc = "Test executeJs with EOA sessionSig."]
#[tokio::test]
async fn execute_js_with_eoa_session_sigs() {
    info!("Starting test: execute_js_with_eoa_session_sigs");

    let (testnet, validator_collection) = init_test().await;
    let wallet = testnet.deploy_account.signing_provider.signer();

    let (_eth_address, pubkey, _token_id) = mint_pkp(&validator_collection.actions()).await;

    let session_sigs = get_session_sigs_for_auth(
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        &validator_collection.addresses(),
        Some(wallet.clone()),
        None,
    );

    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        sign_lit_action(HELLO_WORLD_LIT_ACTION_CODE.to_string(), pubkey)
            .await
            .expect("Could not get lit action params");

    let execute_resp = execute_lit_action_session_sigs(
        &validator_collection,
        lit_action_code,
        ipfs_id, // None
        js_params,
        auth_methods, // None
        &session_sigs,
        EndpointVersion::V1,
        2,
    )
    .await
    .expect("Could not execute lit action");

    let action_result = assert_signed_action(&validator_collection, execute_resp).await;
    assert!(action_result.is_ok());

    let action_result = action_result.unwrap();
    assert!(
        action_result == true,
        "The action should have returned true"
    );
}

#[doc = "Custom Decryption: Decrypt with PKP Wallet that run a Lit Action to create a sessionSig. This shows that the authentication can happen via PKP & the ACC is just the PKP Wallet Address."]
#[tokio::test]
async fn decrypt_with_lit_action_session_sig() {
    info!("Starting test: decrypt_with_lit_action_session_sig");

    let (_testnet, validator_collection) = init_test().await;

    let (eth_address, pubkey, token_id) = mint_pkp(&validator_collection.actions()).await;

    let lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    assert!(add_permitted_action_to_pkp(
        &validator_collection.actions(),
        "QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz", // IPFS CID for `VALID_SESSION_SIG_LIT_ACTION_CODE`
        token_id,
        &[U256::from(AuthMethodScope::SignAnything as usize)]
    )
    .await
    .unwrap());

    let (
        to_encrypt,
        data_to_encrypt_hash,
        access_control_conditions,
        evm_contract_conditions,
        sol_rpc_conditions,
        unified_access_control_conditions,
        chain,
    ) = prepare_test_encryption_parameters_with_wallet_address(encoding::bytes_to_hex(
        eth_address.to_vec(),
    ));

    let network_pubkey = get_network_pubkey(&validator_collection.actions()).await;
    let message_bytes = to_encrypt.as_bytes();
    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: access_control_conditions.clone(),
        evm_contract_conditions: evm_contract_conditions.clone(),
        sol_rpc_conditions: sol_rpc_conditions.clone(),
        unified_access_control_conditions: unified_access_control_conditions.clone(),
    })
    .unwrap();
    let identity_param = AccessControlConditionResource::new(format!(
        "{}/{}",
        hashed_access_control_conditions, data_to_encrypt_hash
    ))
    .get_resource_key()
    .into_bytes();

    // Encrypt
    let ciphertext = lit_bls_wasm::encrypt_time_lock::<Bls12381G2Impl>(
        &network_pubkey,
        message_bytes.to_vec(),
        identity_param.clone(),
    )
    .expect("Unable to encrypt");
    info!("ciphertext: {:?}", ciphertext);

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs = get_session_sigs_for_pkp(
        &validator_collection.actions(),
        pubkey.clone(),
        ethers::types::H160(eth_address.clone()),
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                // resource: "*".to_string(),
                resource: format!(
                    "{}/{}",
                    hashed_access_control_conditions, data_to_encrypt_hash
                ),
                resource_prefix: LitResourcePrefix::ACC.to_string(),
            },
            ability: LitAbility::AccessControlConditionDecryption.to_string(),
        }],
        &validator_collection.addresses(),
        non_owner_wallet.clone(),
        None,
        CurveType::BLS,
        Some(lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
    )
    .await
    .expect("Could not get session sigs");

    // Retrieve decrypted key
    let decryption_resp = retrieve_decryption_key_session_sigs(
        &validator_collection,
        access_control_conditions.clone(),
        evm_contract_conditions.clone(),
        sol_rpc_conditions.clone(),
        unified_access_control_conditions.clone(),
        chain.clone(),
        data_to_encrypt_hash.clone(),
        &session_sigs,
    )
    .await;

    // Assert decryption
    assert_decrypted(
        validator_collection
            .actions()
            .get_current_validator_count()
            .await,
        &network_pubkey,
        identity_param,
        &to_encrypt,
        &ciphertext,
        decryption_resp,
    );

    // Decryption fails for any other address
    let auth_sig = generate_authsig(&non_owner_wallet).await.unwrap();

    let unauthorized_decryption_resp = retrieve_decryption_key(
        &validator_collection.actions(),
        access_control_conditions,
        evm_contract_conditions,
        sol_rpc_conditions,
        unified_access_control_conditions,
        chain,
        data_to_encrypt_hash,
        &auth_sig,
    )
    .await;

    assert!(unauthorized_decryption_resp[0]
        .contains("NodeAccessControlConditionsReturnedNotAuthorized"));
}

#[doc = "V1 endpoints should accept AuthSig."]
#[tokio::test]
async fn test_v1_endpoints_api_constraints() {
    info!("Starting test: test_v1_endpoints_api_constraints");

    let (testnet, validator_collection) = init_test().await;

    let wallet = testnet.deploy_account.signing_provider.signer();
    let auth_sig = generate_authsig_item(wallet).await.unwrap();

    let (eth_address, pubkey, _token_id) = mint_pkp(&validator_collection.actions()).await;

    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    let resource = "*/*".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::LA);

    info!("Starting test: Can't provide Authsig/SessionSig to sign_session_key");
    let session_sig_resp = get_auth_sig_for_session_sig_from_nodes(
        &validator_collection.actions(),
        &auth_sig,
        true,
        &eth_address,
        &pubkey,
        session_pub_key.clone(),
        CurveType::BLS,
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        None,
        None,
        2, // Hardcoded as at other places in the tests
    )
    .await
    .expect("Could not get SessionKey signed");

    assert!(session_sig_resp[0].contains("NodeInvalidAuthSigForSessionKey"));

    info!("Starting test: Can't provide Authsig to pkp_sign");
    let pkp_signing_resp = get_pkp_sign(
        &validator_collection,
        None,
        Some(auth_sig.clone()),
        false,
        "Hello Lit".to_string(),
        pubkey.clone(),
    )
    .await
    .expect("Could not get PKP sign");

    assert!(pkp_signing_resp[0].contains("NodeCannotProvideAuthSigForEndpoint"));

    info!("Starting test: Can't provide AuthMethod to pkp_sign");
    let pkp_signing_resp = get_pkp_sign(
        &validator_collection,
        None,
        Some(auth_sig.clone()),
        true,
        "Hello Lit".to_string(),
        pubkey.clone(),
    )
    .await
    .expect("Could not get PKP sign");

    assert!(pkp_signing_resp[0].contains("NodeCannotProvideAuthMethodForEndpoint"));

    info!("Starting test: Can't provide Authsig to execute_js");
    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        sign_lit_action(HELLO_WORLD_LIT_ACTION_CODE.to_string(), pubkey.clone())
            .await
            .expect("Could not get lit action params");

    let execute_resp = execute_lit_action(
        &validator_collection.actions(),
        Some(lit_action_code),
        ipfs_id, // None
        js_params,
        auth_methods, // None
        auth_sig.clone(),
        EndpointVersion::V1,
    )
    .await
    .expect("Could not execute lit action");

    assert!(execute_resp[0].contains("NodeCannotProvideAuthSigForEndpoint"));

    info!("Starting test: Can't provide AuthMethod to execute_js");
    let (lit_action_code, ipfs_id, js_params, _auth_methods) =
        sign_lit_action(HELLO_WORLD_LIT_ACTION_CODE.to_string(), pubkey)
            .await
            .expect("Could not get lit action params");

    let auth_methods = Some(vec![AuthMethod {
        auth_method_type: 1,
        access_token: serde_json::to_string(&auth_sig).unwrap(),
    }]);

    let execute_resp = execute_lit_action(
        &validator_collection.actions(),
        Some(lit_action_code),
        ipfs_id, // None
        js_params,
        auth_methods,
        auth_sig,
        EndpointVersion::V1,
    )
    .await
    .expect("Could not execute lit action");

    assert!(execute_resp[0].contains("NodeCannotProvideAuthMethodForEndpoint"));
}

#[doc = "Initial signSessionKey: AuthSig with permitted Address and permitted AuthMethods (Address with lit suffixed)"]
#[tokio::test]
async fn sign_session_key_auth_sig() {
    info!("Starting test: sign_session_key_auth_sig");

    let (_testnet, validator_collection) = init_test().await;

    let (eth_address, pubkey, token_id) = mint_pkp(&validator_collection.actions()).await;

    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    // Resources don't matter here
    let resource = "*/*".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::PKP);

    info!("Permit bare address");
    let address_to_permit_wallet = LocalWallet::new(&mut OsRng);
    let address_to_permit_auth_sig = generate_authsig_item(&address_to_permit_wallet)
        .await
        .unwrap();

    let is_permitted_address_to_permit_wallet = add_permitted_address_to_pkp(
        &validator_collection.actions(),
        &hex::encode(address_to_permit_wallet.address().as_bytes()),
        token_id,
        &[U256::from(AuthMethodScope::SignPersonalMessage as usize)],
    )
    .await;
    assert!(is_permitted_address_to_permit_wallet.unwrap());

    let address_to_permit_signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &validator_collection.actions(),
        &address_to_permit_auth_sig,
        true,
        &eth_address,
        &pubkey,
        session_pub_key.clone(),
        CurveType::K256,
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        None,
        None,
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let address_to_permit_signing_resp = address_to_permit_signing_resp.unwrap();
    for response in &address_to_permit_signing_resp {
        assert!(response.contains("success"));
    }

    info!("Permit lower case suffixed AuthMethod");
    let lowercase_auth_method_to_permit_wallet = LocalWallet::new(&mut OsRng);
    let lowercase_auth_method_to_permit_auth_sig =
        generate_authsig_item(&lowercase_auth_method_to_permit_wallet)
            .await
            .unwrap();

    let lowercase_auth_method_to_permit_wallet_address = &format!(
        "0x{}",
        hex::encode(lowercase_auth_method_to_permit_wallet.address().as_bytes())
    ); // Need to add "0x" explicitly

    let is_permitted_lowercase_auth_method_to_permit_wallet =
        add_permitted_address_auth_method_to_pkp(
            &validator_collection.actions(),
            get_user_wallet_auth_method_from_address(
                lowercase_auth_method_to_permit_wallet_address,
            )
            .unwrap(),
            token_id,
            &[U256::from(AuthMethodScope::SignPersonalMessage as usize)],
        )
        .await;
    assert!(is_permitted_lowercase_auth_method_to_permit_wallet.unwrap());

    let lowercase_auth_method_to_permit_signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &validator_collection.actions(),
        &lowercase_auth_method_to_permit_auth_sig,
        true,
        &eth_address,
        &pubkey,
        session_pub_key.clone(),
        CurveType::K256,
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        None,
        None,
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let lowercase_auth_method_to_permit_signing_resp =
        lowercase_auth_method_to_permit_signing_resp.unwrap();
    for response in &lowercase_auth_method_to_permit_signing_resp {
        assert!(response.contains("success"));
    }

    info!("Permit checked sum suffixed AuthMethod");
    let checkedsum_auth_method_to_permit_wallet = LocalWallet::new(&mut OsRng);
    let checkedsum_auth_method_to_permit_auth_sig =
        generate_authsig_item(&checkedsum_auth_method_to_permit_wallet)
            .await
            .unwrap();

    let checkedsum_address =
        get_wallet_address_from_auth_sig_item(&checkedsum_auth_method_to_permit_auth_sig);
    assert!(checkedsum_address.is_ok());

    let is_permitted_checkedsum_auth_method_to_permit_wallet =
        add_permitted_address_auth_method_to_pkp(
            &validator_collection.actions(),
            get_user_wallet_auth_method_from_address(&checkedsum_address.unwrap()).unwrap(),
            token_id,
            &[U256::from(AuthMethodScope::SignPersonalMessage as usize)],
        )
        .await;
    assert!(is_permitted_checkedsum_auth_method_to_permit_wallet.unwrap());

    let checkedsum_auth_method_to_permit_signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &validator_collection.actions(),
        &checkedsum_auth_method_to_permit_auth_sig,
        true,
        &eth_address,
        &pubkey,
        session_pub_key.clone(),
        CurveType::K256,
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        None,
        None,
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let checkedsum_auth_method_to_permit_signing_resp =
        checkedsum_auth_method_to_permit_signing_resp.unwrap();
    for response in &checkedsum_auth_method_to_permit_signing_resp {
        assert!(response.contains("success"));
    }
}

#[doc = "Initial signSessionKey: AuthMethod with permitted Address and permitted AuthMethods (Address with lit suffixed)"]
#[tokio::test]
async fn sign_session_key_auth_method() {
    info!("Starting test: sign_session_key_auth_method");

    let (_testnet, validator_collection) = init_test().await;

    let (eth_address, pubkey, token_id) = mint_pkp(&validator_collection.actions()).await;

    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    // Resources don't matter here
    let resource = "*/*".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::PKP);

    // Permit address_to_permit_wallet address
    info!("Permit bare address");
    let address_to_permit_wallet = LocalWallet::new(&mut OsRng);
    let address_to_permit_auth_sig = generate_authsig_item(&address_to_permit_wallet)
        .await
        .unwrap();

    let is_permitted_address_to_permit_wallet = add_permitted_address_to_pkp(
        &validator_collection.actions(),
        &hex::encode(address_to_permit_wallet.address().as_bytes()),
        token_id,
        &[U256::from(AuthMethodScope::SignPersonalMessage as usize)],
    )
    .await;
    assert!(is_permitted_address_to_permit_wallet.unwrap());

    let address_to_permit_signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &validator_collection.actions(),
        &address_to_permit_auth_sig,
        false,
        &eth_address,
        &pubkey,
        session_pub_key.clone(),
        CurveType::K256,
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        None,
        None,
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let address_to_permit_signing_resp = address_to_permit_signing_resp.unwrap();
    for response in &address_to_permit_signing_resp {
        assert!(response.contains("success"));
    }

    info!("Permit lower case suffixed address");
    let lowercase_auth_method_to_permit_wallet = LocalWallet::new(&mut OsRng);
    let lowercase_auth_method_to_permit_auth_sig =
        generate_authsig_item(&lowercase_auth_method_to_permit_wallet)
            .await
            .unwrap();

    let lowercase_auth_method_to_permit_wallet_address = &format!(
        "0x{}",
        hex::encode(lowercase_auth_method_to_permit_wallet.address().as_bytes())
    ); // Need to add "0x" explicitly

    let is_permitted_lowercase_auth_method_to_permit_wallet =
        add_permitted_address_auth_method_to_pkp(
            &validator_collection.actions(),
            get_user_wallet_auth_method_from_address(
                lowercase_auth_method_to_permit_wallet_address,
            )
            .unwrap(),
            token_id,
            &[U256::from(AuthMethodScope::SignPersonalMessage as usize)],
        )
        .await;
    assert!(is_permitted_lowercase_auth_method_to_permit_wallet.unwrap());

    let lowercase_auth_method_to_permit_signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &validator_collection.actions(),
        &lowercase_auth_method_to_permit_auth_sig,
        false,
        &eth_address,
        &pubkey,
        session_pub_key.clone(),
        CurveType::K256,
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        None,
        None,
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let lowercase_auth_method_to_permit_signing_resp =
        lowercase_auth_method_to_permit_signing_resp.unwrap();
    for response in &lowercase_auth_method_to_permit_signing_resp {
        assert!(response.contains("success"));
    }

    info!("Permit checked sum suffixed address");
    let checkedsum_auth_method_to_permit_wallet = LocalWallet::new(&mut OsRng);
    let checkedsum_auth_method_to_permit_auth_sig =
        generate_authsig_item(&checkedsum_auth_method_to_permit_wallet)
            .await
            .unwrap();

    let checkedsum_address =
        get_wallet_address_from_auth_sig_item(&checkedsum_auth_method_to_permit_auth_sig);
    assert!(checkedsum_address.is_ok());

    let is_permitted_checkedsum_auth_method_to_permit_wallet =
        add_permitted_address_auth_method_to_pkp(
            &validator_collection.actions(),
            // get_user_wallet_auth_method_from_address(lowercase_auth_method_to_permit_wallet_address).unwrap(),
            get_user_wallet_auth_method_from_address(&checkedsum_address.unwrap()).unwrap(),
            token_id,
            &[U256::from(AuthMethodScope::SignPersonalMessage as usize)],
        )
        .await;
    assert!(is_permitted_checkedsum_auth_method_to_permit_wallet.unwrap());

    let checkedsum_auth_method_to_permit_signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &validator_collection.actions(),
        &checkedsum_auth_method_to_permit_auth_sig,
        false,
        &eth_address,
        &pubkey,
        session_pub_key.clone(),
        CurveType::K256,
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        None,
        None,
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let checkedsum_auth_method_to_permit_signing_resp =
        checkedsum_auth_method_to_permit_signing_resp.unwrap();
    for response in &checkedsum_auth_method_to_permit_signing_resp {
        assert!(response.contains("success"));
    }
}

fn get_wallet_address_from_auth_sig_item(auth_sig_item: &AuthSigItem) -> Result<String> {
    match auth_sig_item {
        AuthSigItem::Single(json_auth_sig) => match json_auth_sig.auth_material_type() {
            AuthMaterialType::WalletSig => Ok(json_auth_sig.address.clone()),
            _ => Err(anyhow::anyhow!("Can only pass Wallet Sig")),
        },
        AuthSigItem::Multiple(_) => Err(anyhow::anyhow!("Can't pass multiple AuthSigs")),
    }
}
