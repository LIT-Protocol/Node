use crate::common::auth_sig::generate_authsig_item;
use crate::common::node_collection::hit_endpoints_with_json_body_join_all;
use crate::common::node_collection::NodeCollection;
use crate::common::testnet::scenario::Scenario;

use anyhow::Result;
use cait_sith::FullSignature;
use ethers::core::types::U256;
use ethers::types::Bytes;
use k256::ecdsa::{RecoveryId, Signature};
use k256::{Scalar, Secp256k1};
use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_core::utils::binary::hex_to_bytes;
use lit_node::utils::contract::decode_revert;
use lit_node::utils::encoding::UncompressedPointHex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use tracing::trace;

use lit_node::{
    models::JsonPKPSigningRequest,
    tss::{common::web::models::SignedMessageShare, ecdsa_cait_sith::protocols256k1::CsSigshare},
    utils::encoding::bytes_to_hex,
};
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignWithPKPReturn {
    success: bool,
    signed_data: Vec<u8>,
    signature_share: SignedMessageShare,
}

// copied from lit_ecdsa_wasm_combine
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignedDatak256 {
    pub sig_type: String,
    pub data_signed: k256::Scalar,
    pub signature_share: k256::Scalar,
    pub share_index: u32,
    pub big_r: k256::AffinePoint,
    pub public_key: k256::AffinePoint,
    pub sig_name: String,
}
// copied from lit_ecdsa_wasm_combine
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignatureRecidHex {
    pub r: String,
    pub s: String,
    pub recid: u8,
}

#[doc = "Mint a new PKP and return the pubkey and token id"]
pub async fn mint_next_pkp(scenario: &Scenario) -> Result<(String, U256)> {
    let key_type: U256 = U256::from(2);

    info!("Minting a new PKP from the test harness.");
    let mint_cost = scenario.contracts.pkpnft.mint_cost().call().await?;
    info!("Mint cost: {:}", mint_cost);

    // this is happening under the deployer account, if nothing has changed it.
    let cc = scenario
        .contracts
        .pkpnft
        .mint_next(key_type)
        .value(mint_cost);

    let tx = cc.send().await;
    if tx.is_err() {
        let e = tx.unwrap_err();
        info!(
            "Decoded error: {}",
            decode_revert(&e, scenario.contracts.pkpnft.abi()).to_string()
        );
        error!("Error sending mint PKP: {:?}", e);
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tx = tx.unwrap();

    let tr = tx.await;
    if tr.is_err() {
        error!("Error waiting for mint PKP: {:?}", tr.unwrap_err());
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tr = tr.unwrap();
    if tr.is_none() {
        error!("Error minting PKP: No transaction receipt?");
        return Err(anyhow::anyhow!("Error minting PKP"));
    }

    let receipt = tr.unwrap();

    for log in &receipt.logs {
        tracing::trace!("log: {:?}", &log);
        for topic in &log.topics {
            tracing::trace!("topic: {:?}", &topic);
        }
    }

    let token_id = receipt.logs[0].topics[1];
    let token_id = U256::from(token_id.as_bytes());

    let r = scenario
        .contracts
        .pubkey_router
        .get_pubkey(token_id)
        .call()
        .await?;
    let pubkey = bytes_to_hex(r);

    info!(
        "Minted PKP with token id: {} / pubkey : {}",
        token_id, &pubkey
    );

    Ok((pubkey, token_id))
}

#[doc = "Grant an action permission to use a PKP"]
pub async fn add_permitted_action_to_pkp(
    scenario: &Scenario,
    ipfs_cid: &str,
    token_id: U256,
    scopes: &[U256],
) -> Result<bool> {
    info!("ipfs_cid to permit: {}", ipfs_cid);

    let pacc = scenario.contracts.pkp_permissions.add_permitted_action(
        token_id,
        Bytes::from(bs58::decode(ipfs_cid).into_vec().unwrap()),
        scopes.to_vec(),
    );

    let tx = pacc.send().await;
    if tx.is_err() {
        error!("Error adding action to pkp: {:?}", tx.unwrap_err());
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tx = tx.unwrap();

    let tr = tx.await;
    if tr.is_err() {
        error!("Error adding action to pkp: {:?}", tr.unwrap_err());
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tr = tr.unwrap();
    if tr.is_none() {
        error!("Error adding action to pkp: No transaction receipt?");
        return Err(anyhow::anyhow!("Error minting PKP"));
    }

    Ok(true)
}

#[doc = "Grant an address permission to use a PKP"]
pub async fn add_permitted_address_to_pkp(
    scenario: &Scenario,
    address: &str,
    token_id: U256,
    scopes: &[U256],
) -> Result<bool> {
    info!("address to permit: {}", address);
    let addr = ethers::core::types::Address::from_str(address);
    assert!(addr.is_ok());
    let addr = addr.unwrap();

    let pacc =
        scenario
            .contracts
            .pkp_permissions
            .add_permitted_address(token_id, addr, scopes.to_vec());

    let tx = pacc.send().await;
    if tx.is_err() {
        error!("Error adding address to pkp: {:?}", tx.unwrap_err());
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tx = tx.unwrap();

    let tr = tx.await;
    if tr.is_err() {
        error!("Error adding address to pkp: {:?}", tr.unwrap_err());
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tr = tr.unwrap();
    if tr.is_none() {
        error!("Error adding address to pkp: No transaction receipt?");
        return Err(anyhow::anyhow!("Error minting PKP"));
    }

    let pa = scenario
        .contracts
        .pkp_permissions
        .get_permitted_addresses(token_id)
        .call()
        .await?;
    info!("permitted addresses: {:?}", pa);

    Ok(true)
}

#[doc = "Sign a 'hello world' message with a PKP"]
pub async fn sign_hello_world_with_pkp(
    node_collection: &NodeCollection,
    scenario: &Scenario,
    pubkey: String,
) -> Result<bool> {
    let to_sign = "hello world".to_string();
    sign_message_with_pkp(node_collection, scenario, pubkey, to_sign).await
}

#[doc = "Sign bytes with a PKP"]
pub async fn sign_bytes_with_pkp(
    node_collection: &NodeCollection,
    scenario: &Scenario,
    pubkey: String,
    to_sign: Vec<u8>,
) -> Result<(bool, Signature, RecoveryId)> {
    let auth_sig =
        generate_authsig_item(scenario.testnet.deploy_account.signing_provider.signer()).await?;
    let data_to_send = JsonPKPSigningRequest {
        auth_sig,
        to_sign,
        pubkey,
        auth_methods: None,
    };
    let result = sign_with_pkp_request(node_collection, scenario, data_to_send).await?;
    Ok(result)
}

#[doc = "Sign a message with a PKP"]
pub async fn sign_message_with_pkp(
    node_collection: &NodeCollection,
    scenario: &Scenario,
    pubkey: String,
    to_sign: String,
) -> Result<bool> {
    let data_to_send =
        generate_data_to_send(pubkey, ethers::utils::keccak256(to_sign).into(), scenario).await?;
    let result = sign_with_pkp_request(node_collection, scenario, data_to_send).await?;
    Ok(result.0)
}

pub async fn generate_data_to_send(
    pubkey: String,
    to_sign: Vec<u8>,
    scenario: &Scenario,
) -> Result<JsonPKPSigningRequest> {
    let auth_sig =
        generate_authsig_item(scenario.testnet.deploy_account.signing_provider.signer()).await?;
    let data_to_send = JsonPKPSigningRequest {
        auth_sig,
        to_sign,
        pubkey,
        auth_methods: None,
    };
    Ok(data_to_send)
}

pub async fn sign_message_with_hd_key(
    node_collection: &NodeCollection,
    scenario: &Scenario,
    pubkey: String,
    to_sign: String,
) -> Result<bool> {
    let auth_sig =
        generate_authsig_item(scenario.testnet.deploy_account.signing_provider.signer()).await?;

    let data_to_send = JsonPKPSigningRequest {
        auth_sig,
        to_sign: to_sign.into_bytes(),
        pubkey,
        auth_methods: None,
    };
    let result = sign_with_pkp_request(node_collection, scenario, data_to_send).await?;
    Ok(result.0)
}

pub async fn send_signing_requests(
    _node_collection: &NodeCollection,
    scenario: &Scenario,
    data_to_send: JsonPKPSigningRequest,
) -> Result<Vec<String>> {
    // Send out our signature request to all the nodes.
    let json_body = serde_json::to_string(&data_to_send).unwrap();
    let cmd = "/web/pkp/sign".to_string();
    let endpoint_responses =
        hit_endpoints_with_json_body_join_all(scenario, cmd, json_body).await?;

    // check to make sure we received all answers.
    // removed to account for nodes that drop during tests.
    // assert_eq!(endpoint_responses.len(), num_nodes);

    Ok(endpoint_responses)
}

pub async fn sign_with_pkp_request(
    node_collection: &NodeCollection,
    scenario: &Scenario,
    data_to_send: JsonPKPSigningRequest,
) -> Result<(bool, Signature, RecoveryId)> {
    let endpoint_responses =
        send_signing_requests(node_collection, scenario, data_to_send.clone()).await?;

    trace!("endpoint_responses: {:?}", endpoint_responses);

    let json_responses: Vec<Value> = endpoint_responses
        .iter()
        .map(|x| serde_json::from_str(x).expect("Could not parse JSON response"))
        .collect();

    trace!("json_responses: {:?}", json_responses);

    if json_responses.iter().any(|x| {
        x.as_object()
            .expect("Could not convert JSON response to object")
            .contains_key("errorCode")
    }) {
        // return default sig
        let sig = Signature::from_scalars(Scalar::from(1_u32), Scalar::from(1_u32)).unwrap();
        trace!(
            "Could not sign with PKP, returning empty sig and false validation: {:?}",
            json_responses
        );
        return Ok((false, sig, RecoveryId::from_byte(0).unwrap()));
    }

    // collect the shares into a struct and a set of string that can be used to recombine using the WASM module.
    let (shares, cs_sig_shares, public_key, presignature_big_r, msg_hash) =
        decode_endpoint_responses(endpoint_responses);

    let (signature, recovery_id) = recombine_shares_using_wasm(shares).unwrap();

    // Check on the public key value from DKG and based on the signature + recovery id.
    let dkg_public_key = k256::ecdsa::VerifyingKey::from_affine(public_key).unwrap();
    let recovered_key = k256::ecdsa::VerifyingKey::recover_from_prehash(
        &data_to_send.to_sign.clone(),
        &signature,
        recovery_id,
    )?;
    let pkp_requested_to_be_used =
        k256::ecdsa::VerifyingKey::from_uncompressed_hex(&data_to_send.pubkey.clone()).unwrap();

    info!(
        "dkg'd pk: {:?}",
        bytes_to_hex(&dkg_public_key.to_encoded_point(false))
    );
    info!(
        "recovered pk: {:?}",
        bytes_to_hex(&recovered_key.to_encoded_point(false))
    );
    info!(
        "pkp requested to be used: {:?}",
        &data_to_send.pubkey.clone()
    );

    assert!(dkg_public_key.to_encoded_point(false) == recovered_key.to_encoded_point(false));
    assert!(
        dkg_public_key.to_encoded_point(false) == pkp_requested_to_be_used.to_encoded_point(false)
    );

    let (is_valid, _sig) = verify_ecdsa_using_cait_sith_shares(
        public_key,
        presignature_big_r,
        msg_hash,
        cs_sig_shares,
    )
    .await;
    Ok((is_valid, signature, recovery_id))
}

#[doc = "Verify a signature using a set of Cait-Sith shares"]
async fn verify_ecdsa_using_cait_sith_shares(
    public_key: k256::AffinePoint,
    presignature_big_r: k256::AffinePoint,
    msg_hash: Scalar,
    sig_response: Vec<CsSigshare>,
) -> (bool, FullSignature<Secp256k1>) {
    let shares = sig_response
        .into_iter()
        .map(|x| x.share)
        .collect::<Vec<_>>();
    let sig = cait_sith::combine_signature_shares::<Secp256k1>(
        shares,
        public_key,
        presignature_big_r,
        msg_hash,
    )
    .await;
    let sig = sig.unwrap();
    let is_valid = sig.verify(&public_key, &msg_hash);
    info!("Cait-Sith Sig is valid: {}", is_valid);
    (is_valid, sig)
}

#[doc = "Recombine a set of shares using code imported from the Lit ECDSA WASM module."]
fn recombine_shares_using_wasm(shares: Vec<String>) -> Result<(Signature, RecoveryId)> {
    // use the WASM module
    tracing::trace!("shares for wasm combine: {:?}", &shares);
    let result = lit_ecdsa_wasm_combine::combiners::k256_cait_sith::combine_signature(shares);
    tracing::trace!("result: {:?}", &result);
    let wasm_sig = serde_json::from_str::<SignatureRecidHex>(&result).unwrap();
    info!("wasm_sig: {:?}", &wasm_sig);
    let sig = format!("{}{}", &wasm_sig.r[2..], wasm_sig.s);
    let sig_slice = hex_to_bytes(sig).unwrap();
    let signature = k256::ecdsa::Signature::from_slice(&sig_slice)?;
    let recovery_id = k256::ecdsa::RecoveryId::try_from(wasm_sig.recid)?;
    Ok((signature, recovery_id))
}

#[doc = "Decode the responses from the nodes into a set of string based shares and a set of Cait-Sith shares."]
fn decode_endpoint_responses(
    endpoint_responses: Vec<String>,
) -> (
    Vec<String>,
    Vec<CsSigshare>,
    k256::AffinePoint,
    k256::AffinePoint,
    Scalar,
) {
    let mut cs_sig_shares: Vec<CsSigshare> = Vec::new();
    let mut shares: Vec<String> = Vec::new();

    for r in endpoint_responses {
        let result = serde_json::from_str::<SignWithPKPReturn>(&r);
        if result.is_err() {
            panic!("Error parsing response as SignWithPKPReturn: {}", &r);
        }

        let result = result.unwrap();
        tracing::trace!("Result: {:?}", &result);
        if result.signature_share.result == "success" {
            let ecdsa_msg_share = result.signature_share.clone();

            let sig_share = CsSigshare {
                share: serde_json::from_str(&ecdsa_msg_share.signature_share)
                    .unwrap_or(Scalar::ZERO),
                public_key: serde_json::from_str(&ecdsa_msg_share.public_key)
                    .unwrap_or(k256::AffinePoint::IDENTITY),
                presignature_big_r: serde_json::from_str(&ecdsa_msg_share.big_r)
                    .unwrap_or(k256::AffinePoint::IDENTITY),
                msg_hash: serde_json::from_str(&ecdsa_msg_share.digest).unwrap_or(Scalar::ZERO),
            };

            cs_sig_shares.push(sig_share);

            // doing this in a structured way to emulate the way it would be consumed by the SDK.
            let signed_data = SignedDatak256 {
                sig_type: result.signature_share.sig_type,
                data_signed: serde_json::from_str(&ecdsa_msg_share.digest).unwrap_or(Scalar::ZERO),
                signature_share: serde_json::from_str(&ecdsa_msg_share.signature_share)
                    .unwrap_or(Scalar::ZERO),
                share_index: result.signature_share.share_index as u32,
                big_r: serde_json::from_str(&ecdsa_msg_share.big_r)
                    .unwrap_or(k256::AffinePoint::IDENTITY),
                public_key: serde_json::from_str(&ecdsa_msg_share.public_key)
                    .unwrap_or(k256::AffinePoint::IDENTITY),
                sig_name: "CS".to_string(),
            };

            shares.push(serde_json::to_string(&signed_data).unwrap());
        }
    }

    let public_key = cs_sig_shares[0].public_key;
    let presignature_big_r = cs_sig_shares[0].presignature_big_r;
    let msg_hash = cs_sig_shares[0].msg_hash;

    (
        shares,
        cs_sig_shares,
        public_key,
        presignature_big_r,
        msg_hash,
    )
}

#[doc = "Generate a PKP and get addresss  permitted to use it"]
pub async fn generate_pkp_check_get_permitted_address(
    address: &str,
    node_collection: &NodeCollection,
    scenario: &Scenario,
) -> Result<Vec<Value>> {
    let config_file = node_collection.config_files[0].clone(); //  format!("../../{}",node_collection.config_files[0]);
    tracing::info!("config_file: {}", &config_file);
    std::env::set_var(ENV_LIT_CONFIG_FILE, &config_file);

    let cfg = lit_node::config::load_cfg().expect("failed to load LitConfig");
    let loaded_config = &cfg.load_full();

    let (_result, token_id) = generate_pkp_check_is_permitted_address(address, scenario).await;

    let res = lit_node::pkp::utils::pkp_permissions_get_permitted(
        String::from("getPermittedAddresses"),
        loaded_config.as_ref(),
        token_id.to_string(),
    )
    .await;

    assert!(res.is_ok());
    res.map_err(|e| anyhow::anyhow!(e))
}

#[doc = "Generate a PKP and add a permitted address to use it"]
pub async fn generate_pkp_check_is_permitted_address(
    address: &str,
    scenario: &Scenario,
) -> (Result<bool>, U256) {
    let minted_key = mint_next_pkp(scenario).await;
    assert!(minted_key.is_ok());

    let minted_key = minted_key.unwrap();
    let token_id = minted_key.1;

    let res = add_permitted_address_to_pkp(scenario, address, token_id, &[]).await;

    assert!(res.is_ok());
    (res, token_id)
}
