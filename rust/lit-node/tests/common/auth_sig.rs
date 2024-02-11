use std::collections::BTreeMap;
use std::ops::Add;
use std::str::FromStr;

use chrono::{Duration, SecondsFormat};
use ed25519_dalek::Signer;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::signers::{LocalWallet, Wallet};
use ethers::types::H256;
use generic_array::GenericArray;
use lit_api_core::error::Unexpected;
use lit_blockchain::config::LitBlockchainConfig;
use lit_core::config::LitConfig;
use lit_node::auth::auth_material::{
    AuthSigItem, JsonAuthSig, AUTH_SIG_DERIVED_VIA_SESSION_SIG, AUTH_SIG_SESSION_SIG_ALGO,
};
use lit_node::error::Result;
use lit_node::models::auth::SessionKeySignedMessage;
use lit_node::models::auth::{LitResourceAbilityRequest, LitResourcePrefix};
use lit_node::utils::encoding;
use lit_node::utils::encoding::hex_to_bytes;
use log::info;
use serde_json::Value;
use siwe::Message;
use siwe_recap::Capability;
use std::ops::Sub;

use ethers::prelude::rand::rngs::OsRng as EthersOsRng;
use ethers::signers::Signer as WalletSigner;
use ethers::utils::to_checksum;

use tracing::error;

pub fn node_wallet(cfg: &LitConfig) -> Result<Wallet<SigningKey>> {
    let secret_key = SigningKey::from_bytes(GenericArray::from_slice(
        &hex_to_bytes(cfg.blockchain_wallet_private_key(None)?)
            .expect_or_err("Failed to hex encode node.private_key config var")?,
    ))
    .expect_or_err("Could not convert node.private_key config var to SigningKey")?;
    let chain_id = cfg.blockchain_chain_id()?;
    Ok(LocalWallet::from(secret_key).with_chain_id(chain_id)) // if you don't use this with_chain_id() function, you will get an error when you try to sign a txn.
}

pub async fn generate_authsig_item(wallet: &Wallet<SigningKey>) -> Result<AuthSigItem> {
    let auth_sig = generate_authsig(wallet).await?;
    Ok(AuthSigItem::Single(auth_sig))
}

/// handy function, but probably belongs elsewhere!
pub async fn generate_authsig(wallet: &Wallet<SigningKey>) -> Result<JsonAuthSig> {
    // let wallet = node_wallet(cfg)?;
    let chain_id = wallet.chain_id();
    // let chain_id = cfg.blockchain_chain_id()?;
    let address = to_checksum(&wallet.address(), None);
    let now = chrono::Utc::now();
    let issue_datetime = now.to_rfc3339_opts(SecondsFormat::Millis, true);
    let expiration_datetime = now
        .add(Duration::days(1))
        .to_rfc3339_opts(SecondsFormat::Millis, true);
    let message = format!(
        "localhost wants you to sign in with your Ethereum account:
{}

This is a key for a Lit Action Test.

URI: https://localhost/
Version: 1
Chain ID: {}
Nonce: 1LF00rraLO4f7ZSIt
Issued At: {}
Expiration Time: {}",
        address, chain_id, issue_datetime, expiration_datetime
    );

    // Sign the message
    let signature = wallet.sign_message(&message).await.unwrap();
    let auth_sig = JsonAuthSig::new(
        format!("0x{:}", &signature.to_string()),
        "web3.eth.personal.sign".to_string(),
        message,
        address.clone(),
        Option::None,
    );

    info!(
        "Successfully generated a SIWE compatible authSig for using wallet address: {}",
        address
    );
    // info!("AuthSig: {}", &auth_sig);

    Ok(auth_sig)
}

#[tokio::test]
async fn test_generate_authsig() {
    super::init_test_config();

    std::env::set_var("LIT_CONFIG_FILE", "./config/test/lit_sig_cfg.toml");
    let cfg = lit_node::config::load_cfg().expect("failed to load LitConfig");
    let loaded_config = &cfg.load_full();
    let wallet = node_wallet(loaded_config).expect("failed to get node wallet");
    let auth_sig = generate_authsig(&wallet).await;

    assert!(auth_sig.is_ok());

    let auth_sig = auth_sig.unwrap();

    // check the SIWE message format.
    let siwe_message = auth_sig.signed_message;
    info!("siwe_message: {}", &siwe_message);
    let message = Message::from_str(&siwe_message);

    if message.is_err() {
        let err = message.err().unwrap();
        error!("Error: {:?}", err);
        assert!(false);
    } else {
        assert!(message.is_ok());
    }
}

// Get an auth sig that can be used for generating session sigs.
pub fn get_auth_sig_with_rli_nft_resources(
    wallet: Wallet<SigningKey>,
    nft_id: String,
    wallet_being_delegated_to: String,
) -> JsonAuthSig {
    let mut notabene = BTreeMap::new();
    notabene.insert("nft_id".to_string(), Value::from(vec![Value::from(nft_id)]));
    notabene.insert("uses".to_string(), Value::from("10".to_string()));
    notabene.insert(
        "delegate_to".to_string(),
        Value::from(vec![Value::from(wallet_being_delegated_to)]),
    );
    let now = chrono::Utc::now();
    let siwe_issued_at = now.sub(Duration::days(1));
    let siwe_expiration_time = now.add(Duration::days(7));
    let mut capabilities = Capability::<Value>::default();
    let resource = "Auth/Auth".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::RLI);
    let capabilities = capabilities.with_actions_convert(resource_prefix, [(resource, [notabene])]);
    if let Err(e) = capabilities {
        panic!("Error: {:?}", e);
    }
    let capabilities = capabilities.unwrap();
    let siwe_message: siwe::Message = capabilities
        .build_message(siwe::Message {
            domain: "example.com".parse().unwrap(),
            address: wallet.address().into(),
            statement: None,
            uri: "lit:capability:delegation".parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "mynonce1".into(),
            issued_at: siwe_issued_at
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                .parse()
                .unwrap(),
            expiration_time: Some(
                siwe_expiration_time
                    .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                    .parse()
                    .unwrap(),
            ),
            not_before: None,
            request_id: None,
            resources: vec![],
        })
        .unwrap();

    // // Generate a SIWE message.

    // let siwe_message = Message {
    //     domain: "localhost:3000".parse().unwrap(),
    //     address: wallet.address().into(),
    //     statement: Some(r#"Some custom statement. I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-accesscontrolcondition://*'. (2) '*': '*' for 'lit-litaction://*'."#.into()),
    //     uri: format!("lit:session:{}", session_pub_key).parse().unwrap(),
    //     version: siwe::Version::V1,
    //     chain_id: 1,
    //     nonce: "JIsknRumpxsM9pqmc".into(),
    //     issued_at: siwe_issued_at.to_rfc3339_opts(chrono::SecondsFormat::Millis, true).parse().unwrap(),
    //     expiration_time: Some(siwe_expiration_time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true).parse().unwrap()),
    //     not_before: None,
    //     request_id: None,
    //     // NOTE: resources contains wildcard capabilities for all ACCs and lit actions. When we
    //     // incorporate SDK tests on the node side here, we can start generating
    //     // more specific capabilities.
    //     resources: vec!["urn:recap:eyJhdHQiOnsibGl0LWFjY2Vzc2NvbnRyb2xjb25kaXRpb246Ly8qIjp7IiovKiI6W3t9XX0sImxpdC1saXRhY3Rpb246Ly8qIjp7IiovKiI6W3t9XX19LCJwcmYiOltdfQ==".parse().unwrap()],
    // };

    // Sign SIWE message with local wallet.
    let sig = wallet.sign_hash(H256::from(siwe_message.eip191_hash().unwrap()));
    JsonAuthSig::new(
        sig.expect("Could not parse sig").to_string(),
        "web3.eth.personal.sign".to_string(),
        siwe_message.to_string(),
        encoding::bytes_to_hex(wallet.address()),
        None,
    )
}

/// Get an auth sig that can be used for generating session sigs.
pub fn get_auth_sig_for_session_sig(
    session_pub_key: String,
    auth_sig_wallet: Option<Wallet<SigningKey>>,
) -> JsonAuthSig {
    // Generate new wallet if one is not provided
    let wallet = auth_sig_wallet.unwrap_or(LocalWallet::new(&mut EthersOsRng));

    // Generate a SIWE message.
    let now = chrono::Utc::now();
    let siwe_issued_at = now.sub(Duration::days(1));
    let siwe_expiration_time = now.add(Duration::days(7));
    let siwe_message = Message {
        domain: "localhost:3000".parse().unwrap(),
        address: wallet.address().into(),
        statement: Some(r#"Some custom statement. I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-accesscontrolcondition://*'. (2) '*': '*' for 'lit-litaction://*'."#.into()),
        uri: format!("lit:session:{}", session_pub_key).parse().unwrap(),
        version: siwe::Version::V1,
        chain_id: 1,
        nonce: "JIsknRumpxsM9pqmc".into(),
        issued_at: siwe_issued_at.to_rfc3339_opts(chrono::SecondsFormat::Millis, true).parse().unwrap(),
        expiration_time: Some(siwe_expiration_time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true).parse().unwrap()),
        not_before: None,
        request_id: None,
        // NOTE: resources contains wildcard capabilities for all ACCs and lit actions. When we 
        // incorporate SDK tests on the node side here, we can start generating 
        // more specific capabilities.
        resources: vec!["urn:recap:eyJhdHQiOnsibGl0LWFjY2Vzc2NvbnRyb2xjb25kaXRpb246Ly8qIjp7IiovKiI6W3t9XX0sImxpdC1saXRhY3Rpb246Ly8qIjp7IiovKiI6W3t9XX19LCJwcmYiOltdfQ==".parse().unwrap()],
    };

    // Sign SIWE message with local wallet.
    let sig = wallet.sign_hash(H256::from(siwe_message.eip191_hash().unwrap()));
    JsonAuthSig::new(
        sig.expect("Could not parse sig").to_string(),
        "web3.eth.personal.sign".to_string(),
        siwe_message.to_string(),
        encoding::bytes_to_hex(wallet.address()),
        None,
    )
}

/// Get session sigs that can be used for auth.
pub fn get_session_sigs_for_auth(
    resource_ability_requests: Vec<LitResourceAbilityRequest>,
    node_addresses: &Vec<String>,
    auth_sig_wallet: Option<Wallet<SigningKey>>,
    additional_capabilities: Option<Vec<JsonAuthSig>>,
) -> Vec<JsonAuthSig> {
    // Generate ed25519 keypair for signing.
    // let csprng = OsRng {};
    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    // Sign SIWE first.
    let auth_sig = get_auth_sig_for_session_sig(session_pub_key.clone(), auth_sig_wallet);
    info!("auth_sig: {:?}", auth_sig);

    // Generate message to sign by session key.
    let now = chrono::Utc::now();
    let session_sig_issued_at = now.sub(Duration::days(1));
    let session_sig_expiration_time = now.add(Duration::days(1));

    let mut session_sigs = vec![];

    let mut capabilities = vec![auth_sig.clone()];
    if let Some(additional_capabilities) = additional_capabilities {
        capabilities.extend(additional_capabilities);
    }

    info!("node_addresses: {:?}", node_addresses);
    for node_address in node_addresses {
        let session_key_signed_message = SessionKeySignedMessage {
            session_key: session_pub_key.clone(),
            resource_ability_requests: resource_ability_requests.clone(),
            capabilities: capabilities.clone(),
            issued_at: session_sig_issued_at.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            expiration: session_sig_expiration_time
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            node_address: node_address.to_owned(),
        };

        // serialize to JSON string
        let message = serde_json::to_string(&session_key_signed_message).unwrap();

        // Sign message with session key.
        let signature = signing_key.sign(message.as_bytes());

        session_sigs.push(JsonAuthSig::new(
            signature.to_string(),
            AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
            message,
            session_pub_key.clone(),
            Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
        ));
    }

    session_sigs
}
