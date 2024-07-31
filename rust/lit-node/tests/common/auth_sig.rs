use std::collections::BTreeMap;
use std::ops::Add;
use std::str::FromStr;

use anyhow::Result;
use blsful::{Bls12381G2Impl, Signature, SignatureShare};
use cait_sith::combine_signature_shares;
use chrono::{Duration, SecondsFormat};
use ed25519_dalek::Signer;
use elliptic_curve::ops::Reduce;
use elliptic_curve::point::AffineCoordinates;
use elliptic_curve::sec1::FromEncodedPoint;
use elliptic_curve::sec1::ToEncodedPoint;
use elliptic_curve::{Curve, CurveArithmetic};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::signers::{LocalWallet, Wallet};
use ethers::types::{Address, RecoveryMessage, H256};
use generic_array::GenericArray;
use jubjub::group::GroupEncoding;
use k256::ecdsa::{RecoveryId, VerifyingKey};
use k256::{EncodedPoint, Scalar, Secp256k1};
use lit_api_core::error::Unexpected;
use lit_blockchain::config::LitBlockchainConfig;
use lit_core::config::LitConfig;
use lit_node::auth::auth_material::{
    AuthSigItem, JsonAuthSig, AUTH_SIG_DERIVED_VIA_SESSION_SIG, AUTH_SIG_SESSION_SIG_ALGO,
};
use lit_node::models::auth::SessionKeySignedMessage;
use lit_node::models::auth::{LitResourceAbilityRequest, LitResourcePrefix};
use lit_node::models::JsonSignSessionKeyRequestV1;
use lit_node::models::{
    AuthMethod, JsonSDKHandshakeResponse, JsonSignSessionKeyRequest, JsonSignSessionKeyResponse,
    JsonSignSessionKeyResponseV1,
};
use lit_node::tss::common::curve_type::CurveType;
use lit_node::utils::encoding::{self, BeHex};
use lit_node::utils::encoding::{hex_to_bytes, CompressedPointHex};
use serde_json::Value;
use siwe::Message;
use siwe_recap::Capability;
use std::ops::Sub;
use tracing::info;

use ethers::prelude::rand::rngs::OsRng as EthersOsRng;
use ethers::signers::Signer as WalletSigner;
use ethers::utils::to_checksum;

use tracing::error;

use crate::common::pkp::SignatureRecidHex;
use crate::common::session_sigs::NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE;

use super::node_collection::{hit_endpoints_with_json_body, hit_endpoints_with_json_body_join_all};
use super::testnet::actions::Actions;

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
    resource_ability_requests: &Vec<LitResourceAbilityRequest>,
) -> JsonAuthSig {
    // Generate new wallet if one is not provided
    let wallet = auth_sig_wallet.unwrap_or(LocalWallet::new(&mut EthersOsRng));

    let mut capabilities = Capability::<Value>::default();

    for resource_ability in resource_ability_requests.iter() {
        let (resource, resource_prefix) = (
            "*/*".to_string(),
            format!("{}://*", resource_ability.resource.resource_prefix.clone()),
        );

        let _ = capabilities.with_actions_convert(resource_prefix, [(resource, [])]);
    }

    // Generate a SIWE message.
    let now = chrono::Utc::now();
    let siwe_issued_at = now.sub(Duration::days(1));
    let siwe_expiration_time = now.add(Duration::days(7));
    let siwe_message = capabilities
        .build_message(Message {
            domain: "localhost:3000".parse().unwrap(),
            address: wallet.address().into(),
            statement: Some(r#"Some custom statement. "#.into()),
            uri: format!("lit:session:{}", session_pub_key).parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "JIsknRumpxsM9pqmc".into(),
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
        .expect("Could not create SIWE");

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
    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    // Sign SIWE first.
    let auth_sig = get_auth_sig_for_session_sig(
        session_pub_key.clone(),
        auth_sig_wallet,
        &resource_ability_requests,
    );
    // info!("auth_sig: {:?}", auth_sig);

    // Generate message to sign by session key.
    let now = chrono::Utc::now();
    let session_sig_issued_at = now.sub(Duration::days(1));
    let session_sig_expiration_time = now.add(Duration::days(1));

    let mut session_sigs = vec![];

    let mut capabilities = vec![auth_sig.clone()];
    if let Some(additional_capabilities) = additional_capabilities {
        capabilities.extend(additional_capabilities);
    }

    // info!("node_addresses: {:?}", node_addresses);
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

pub async fn get_session_delegation_sig_for_pkp(
    actions: &Actions,
    auth_sig: &JsonAuthSig,
    pkp_pubkey: String,
    pkp_eth_address: Address,
    session_pub_key: String,
    curve_type: CurveType,
    resource_ability_requests: &Vec<LitResourceAbilityRequest>,
    code: Option<String>,
    js_params: Option<Value>,
    epoch: u64,
) -> Result<JsonAuthSig> {
    let pkp_pubkey_bytes = hex::decode(&pkp_pubkey).expect("Failed to decode pkp_pubkey");
    // create auth sig using /web/sign_session_key
    let eth_address = pkp_eth_address
        .as_bytes()
        .try_into()
        .expect("Expected an array of length 20");
    let eip55_eth_address = to_checksum(&pkp_eth_address, None);

    let mut resources = vec![];
    let mut resource_prefixes = vec![];

    for resource_ability_request in resource_ability_requests.iter() {
        let (resource, resource_prefix) = (
            "*/*".to_string(),
            format!(
                "{}://*",
                resource_ability_request.resource.resource_prefix.clone()
            ),
        );

        resources.push(resource);
        resource_prefixes.push(resource_prefix);
    }

    let responses = get_auth_sig_for_session_sig_from_nodes(
        actions,
        &AuthSigItem::Single(auth_sig.clone()),
        false,
        &eth_address,
        &pkp_pubkey,
        session_pub_key,
        curve_type,
        resources,
        resource_prefixes,
        code,
        js_params,
        epoch,
    )
    .await?;

    // "sessionSig" : {
    //     "curveType": curve_type.as_str().to_string(),
    //     "dataSigned": encoding::bytes_to_hex(to_sign),
    //     "signatureShare": result.0.signature_share,
    //     "shareIndex": result.0.share_index as u32,
    //     "bigr": result.0.big_r,
    //     "publicKey": encoding::bytes_to_hex(pkp_public_key).replace("0x", ""),
    //     "sigName": "sessionSig",
    //     "siweMessage": siwe_to_sign.to_string(),
    // }

    if curve_type == CurveType::K256 {
        // println!("responses: {:?}", responses);
        let parsed_responses: Vec<JsonSignSessionKeyResponse> = responses
            .iter()
            .map(|response| serde_json::from_str(response).unwrap())
            .collect();

        let affine_pubkey = k256::AffinePoint::from_encoded_point(
            &EncodedPoint::from_bytes(pkp_pubkey_bytes).unwrap(),
        )
        .unwrap();

        let one_response_with_share = parsed_responses
            .iter()
            .find(|r| r.signed_data.session_sig.signature_share.len() > 1)
            .expect("Could not find a response with a share");

        let msg_hash: Scalar = Scalar::from_be_hex(
            &one_response_with_share
                .signed_data
                .session_sig
                .data_signed
                .clone(),
        )
        .unwrap();

        let signed_message = one_response_with_share
            .signed_data
            .session_sig
            .siwe_message
            .clone();
        let big_r: k256::AffinePoint = k256::AffinePoint::from_compressed_hex(
            &one_response_with_share
                .signed_data
                .session_sig
                .bigr
                .replace("\"", ""),
        )
        .unwrap();

        let mut sig_shares_scalars: Vec<Scalar> = Vec::new();
        for s in &parsed_responses {
            if s.signed_data.session_sig.signature_share.len() > 1 {
                sig_shares_scalars.push(
                    serde_json::from_str(&s.signed_data.session_sig.signature_share).unwrap(),
                );
            }
        }

        let sig = combine_signature_shares::<Secp256k1>(
            sig_shares_scalars,
            affine_pubkey,
            big_r,
            msg_hash,
        )
        .await;
        let sig = sig.unwrap();

        let is_valid = sig.verify(&affine_pubkey, &msg_hash);

        if is_valid {
            info!("Cait-Sith Sig is valid");
        } else {
            info!("Cait-Sith Sig is not valid");
        }

        assert!(is_valid);

        let r = sig.big_r;
        let s = sig.s;

        let signature = k256::ecdsa::Signature::from_scalars(<<Secp256k1 as CurveArithmetic>::Scalar as Reduce<<Secp256k1 as Curve>::Uint>>::reduce_bytes(&r.x()), s,).expect("Couldn't create signature");
        // Convert our signature into a recoverable one
        let pubkey_0 = VerifyingKey::recover_from_prehash(
            &msg_hash.to_bytes(),
            &signature,
            RecoveryId::try_from(0).expect("Couldn't create recovery id"),
        )
        .expect("Couldn't recover pubkey for recovery id : 0");
        let pubkey_1 = VerifyingKey::recover_from_prehash(
            &msg_hash.to_bytes(),
            &signature,
            RecoveryId::try_from(1).expect("Couldn't create recovery id"),
        )
        .expect("Couldn't recover pubkey for recovery id : 1");

        let recid = if pubkey_0.to_encoded_point(false) == affine_pubkey.to_encoded_point(false) {
            0
        } else if pubkey_1.to_encoded_point(false) == affine_pubkey.to_encoded_point(false) {
            1
        } else {
            panic!("Neither recovery ID leads to the correct public key");
        };

        let sig_hex = SignatureRecidHex {
            r: hex::encode(r.to_bytes()),
            s: hex::encode(s.to_bytes()),
            recid: recid,
        };

        // println!("sig_hex: {:?}", sig_hex);

        // eth requires that we add 27 to the rec id.  this simply returns 27 if recid is 0, and 28 if recid is 1
        let eth_encoded_recid = match sig_hex.recid {
            0 => "1b",
            1 => "1c",
            _ => panic!("Invalid recid"),
        };

        let sig_hex_str = format!(
            "{}{}{}",
            sig_hex.r.split_at(2).1,
            sig_hex.s,
            eth_encoded_recid
        );

        // println!("sig_hex_str: {:?}", sig_hex_str);

        let ethers_signature = ethers::types::Signature::from_str(&sig_hex_str).unwrap();
        let _ = ethers_signature.verify(
            RecoveryMessage::Hash(H256::from_slice(msg_hash.to_bytes().as_slice())),
            pkp_eth_address,
        )?;

        return Ok(JsonAuthSig::new(
            sig_hex_str,
            "web3.eth.personal.sign".to_string(),
            signed_message,
            eip55_eth_address,
            None,
        ));
    } else if curve_type == CurveType::BLS {
        let parsed_responses: Vec<JsonSignSessionKeyResponseV1> = responses
            .iter()
            .map(|response| serde_json::from_str(response).unwrap())
            .collect();
        let one_response_with_share = parsed_responses[0].clone();
        // println!("one_response_with_share: {:?}", one_response_with_share);

        let shares = parsed_responses
            .iter()
            .map(|response| {
                let sig_share = response.signature_share.clone();
                sig_share
            })
            .collect::<Vec<SignatureShare<Bls12381G2Impl>>>();

        let signature = Signature::from_shares(&shares)?;

        let bls_root_key = blsful::PublicKey::<Bls12381G2Impl>::try_from(
            &hex::decode(&one_response_with_share.bls_root_pubkey)
                .expect("Failed to decode root key"),
        )
        .expect("Failed to convert bls public key from bytes");
        let _ = signature
            .verify(
                &bls_root_key,
                hex::decode(&one_response_with_share.data_signed)
                    .expect("Could not decode data_signed")
                    .as_slice(),
            )
            .expect("Failed to verify signature");

        let serialized_signature = match serde_json::to_string(&signature) {
            Ok(s) => s,
            Err(e) => panic!("Failed to serialize signature: {:?}", e),
        };

        return Ok(JsonAuthSig::new(
            serialized_signature,
            "lit.bls".to_string(),
            one_response_with_share.siwe_message.clone(),
            encoding::bytes_to_hex(eth_address.to_vec()),
            Some("LIT_BLS".to_string()),
        ));
    }

    panic!("Not a valid curve type");
}

/// Get session sigs that can be used for auth.
pub async fn get_session_sigs_for_pkp(
    actions: &Actions,
    pkp_pubkey: String,
    pkp_eth_address: Address,
    resource_ability_requests: Vec<LitResourceAbilityRequest>,
    node_addresses: &Vec<String>,
    auth_sig_wallet: Wallet<SigningKey>,
    additional_capabilities: Option<Vec<JsonAuthSig>>,
    curve_type: CurveType,
    code: Option<String>,
    js_params: Option<Value>,
    epoch: u64,
) -> Result<Vec<JsonAuthSig>> {
    // Generate ed25519 keypair for signing.
    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    let pkp_owner_auth_sig = generate_authsig(&auth_sig_wallet).await?;

    // Sign SIWE first.
    let delegation_auth_sig = get_session_delegation_sig_for_pkp(
        actions,
        &pkp_owner_auth_sig,
        pkp_pubkey,
        pkp_eth_address,
        session_pub_key.clone(),
        curve_type,
        &resource_ability_requests,
        code,
        js_params,
        epoch,
    )
    .await?;
    // info!("delegation_auth_sig: {:?}", delegation_auth_sig);

    // Generate message to sign by session key.
    let now = chrono::Utc::now();
    let session_sig_issued_at = now.sub(Duration::days(1));
    let session_sig_expiration_time = now.add(Duration::days(1));

    let mut session_sigs = vec![];

    let mut capabilities = vec![delegation_auth_sig.clone()];
    if let Some(additional_capabilities) = additional_capabilities {
        capabilities.extend(additional_capabilities);
    }

    // info!("node_addresses: {:?}", node_addresses);
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

    Ok(session_sigs)
}

pub async fn get_auth_sig_for_session_sig_from_nodes(
    actions: &Actions,
    auth_sig: &AuthSigItem,
    pass_auth_sig: bool,
    eth_address: &[u8; 20],
    pubkey: &str,
    session_pub_key: String,
    curve_type: CurveType,
    resources: Vec<String>,
    resource_prefixes: Vec<String>,
    code: Option<String>,
    js_params: Option<Value>,
    epoch: u64,
) -> Result<Vec<String>> {
    let results = hit_endpoints_with_json_body(
        actions,
        "/web/handshake".to_string(),
        r#"{"clientPublicKey":"blah"}"#.to_string(),
    )
    .await;

    // Get latest blockhash for the nonce
    let responses = results
        .iter()
        .map(|result| serde_json::from_str(result).unwrap())
        .collect::<Vec<JsonSDKHandshakeResponse>>();

    let latest_blockhash = &responses[0].latest_blockhash;

    let mut capabilities = Capability::<Value>::default();

    for (resource, resource_prefix) in resources.iter().zip(resource_prefixes.iter()) {
        let _ =
            capabilities.with_actions_convert(resource_prefix.clone(), [(resource.clone(), [])]);
    }

    // Generate a SIWE message.
    let now = chrono::Utc::now();
    let siwe_issued_at = now.sub(Duration::days(1));
    let siwe_expiration_time = now.add(Duration::days(7));
    let siwe_message = capabilities
        .build_message(Message {
            domain: "localhost:3000".parse().unwrap(),
            address: eth_address.clone(),
            statement: Some(r#"I am delegating to a session key"#.into()),
            uri: format!("lit:session:{}", session_pub_key).parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: latest_blockhash.to_string(),
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
        .expect("Could not create SIWE");

    let session_sig_lit_action_code = data_encoding::BASE64.encode(
        NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE
            .to_string()
            .as_bytes(),
    );
    let is_testing_without_auth_method = code == Some(session_sig_lit_action_code);

    match curve_type {
        CurveType::BLS => {
            let signing_request = JsonSignSessionKeyRequestV1 {
                auth_sig: if pass_auth_sig {
                    Some(auth_sig.clone())
                } else {
                    None
                },
                session_key: format!("lit:session:{}", session_pub_key).parse().unwrap(),
                auth_methods: if is_testing_without_auth_method {
                    vec![]
                } else {
                    vec![AuthMethod {
                        auth_method_type: 1,
                        access_token: serde_json::to_string(&auth_sig).unwrap(),
                    }]
                },
                pkp_public_key: Some(pubkey.to_string()),
                siwe_message: siwe_message.to_string(),
                curve_type: curve_type,
                code,
                lit_action_ipfs_id: None,
                js_params,
                epoch: epoch,
            };

            let json_body = serde_json::to_string(&signing_request).unwrap();
            let cmd = "web/sign_session_key/v1".to_string();

            hit_endpoints_with_json_body_join_all(actions, cmd, json_body).await
        }
        CurveType::K256 => {
            let signing_request = JsonSignSessionKeyRequest {
                auth_sig: if pass_auth_sig {
                    Some(auth_sig.clone())
                } else {
                    None
                },
                session_key: format!("lit:session:{}", session_pub_key).parse().unwrap(),
                auth_methods: if pass_auth_sig {
                    vec![]
                } else {
                    vec![AuthMethod {
                        auth_method_type: 1,
                        access_token: serde_json::to_string(&auth_sig).unwrap(),
                    }]
                },
                pkp_public_key: Some(pubkey.to_string()),
                siwe_message: siwe_message.to_string(),
                epoch: epoch,
            };

            let json_body = serde_json::to_string(&signing_request).unwrap();
            let cmd = "web/sign_session_key".to_string();

            hit_endpoints_with_json_body_join_all(actions, cmd, json_body).await
        }
        _ => panic!("Invalid curve type"),
    }
}
