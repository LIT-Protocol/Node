use crate::common::auth_sig::generate_authsig_item;
use crate::common::node_collection::get_current_validator_portnames;
use crate::common::node_collection::hit_endpoints_with_json_body_join_all;

use anyhow::Result;
use cait_sith::FullSignature;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::types::U256;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use ethers::types::Address;
use ethers::types::Bytes;
use k256::ecdsa::{RecoveryId, Signature};
use k256::{Scalar, Secp256k1};
use lit_blockchain::contracts::pkp_permissions::AuthMethod;
use lit_blockchain::contracts::pkp_permissions::PKPPermissions;
use lit_blockchain::contracts::pkpnft::PKPNFT;
use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_core::utils::binary::hex_to_bytes;
use lit_node::utils::consensus::get_threshold_count;
use lit_node::utils::contract::decode_revert;
use lit_node::utils::encoding::UncompressedPointHex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use std::sync::Arc;
use tracing::debug;
use tracing::trace;

use lit_node::{
    models::JsonPKPSigningRequest, p2p_comms::web::models::SignedMessageShare,
    tss::ecdsa_cait_sith::protocols256k1::CsSigshare, utils::encoding::bytes_to_hex,
};
use tracing::{error, info};

use super::testnet::actions::Actions;
use super::validator::ValidatorCollection;

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

#[doc = "Mint a new PKP with the default wallet and return the pubkey and token id"]
pub async fn mint_next_pkp(actions: &Actions) -> Result<(String, U256, [u8; 20])> {
    // this is happening under the deployer account, if nothing has changed it.
    let client = actions.deployer_signing_provider();

    let minted = mint_next_pkp_with_wallet(client, actions).await?;
    Ok((minted.0, minted.1, minted.2.into()))
}

#[doc = "Mint a new PKP with a specific wallet and return the pubkey and token id"]
pub async fn mint_next_pkp_with_wallet(
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    actions: &Actions,
) -> Result<(String, U256, Address)> {
    let key_type: U256 = U256::from(2);

    let pkpnft_address = actions.contracts().pkpnft.address();
    let pkpnft = PKPNFT::new(pkpnft_address, Arc::new(client));

    info!("Minting a new PKP from the test harness.");
    let mint_cost = pkpnft.mint_cost().call().await?;
    info!("Mint cost: {:}", mint_cost);

    let cc = pkpnft.mint_next(key_type).value(mint_cost);

    let tx = cc.send().await;
    if tx.is_err() {
        let e = tx.unwrap_err();
        info!(
            "Decoded error: {}",
            decode_revert(&e, actions.contracts().pkpnft.abi()).to_string()
        );
        error!("Error sending mint PKP: {:?}", e);
        return Err(anyhow::anyhow!("Error minting PKP - sending tx"));
    }
    let tx = tx.unwrap();

    let tr = tx.await;
    if tr.is_err() {
        error!("Error waiting for mint PKP: {:?}", tr.unwrap_err());
        return Err(anyhow::anyhow!("Error minting PKP - waiting for tx"));
    }
    let tr = tr.unwrap();
    if tr.is_none() {
        error!("Error minting PKP: No transaction receipt?");
        return Err(anyhow::anyhow!("Error minting PKP - no tx receipt"));
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

    let r = actions
        .contracts()
        .pubkey_router
        .get_pubkey(token_id)
        .call()
        .await?;
    let pubkey = bytes_to_hex(r);

    let eth_address = pkpnft.get_eth_address(token_id).call().await?;

    info!(
        "Minted PKP with token id: {} / pubkey : {} / eth address: {:?}",
        token_id, &pubkey, eth_address
    );

    Ok((pubkey, token_id, eth_address))
}

#[doc = "Transfer a PKP"]
pub async fn transfer_pkp_with_wallet(
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    actions: &Actions,
    to_address: Address,
    token_id: U256,
) -> Result<()> {
    info!(
        "Transferring PKP with token id: {} to address: {}",
        token_id, to_address
    );
    let pkpnft_address = actions.contracts().pkpnft.address();
    let pkpnft = PKPNFT::new(pkpnft_address, client.clone());

    let cc = pkpnft.transfer_from(client.clone().address(), to_address, token_id);
    let tx = cc.send().await;
    if tx.is_err() {
        let e = tx.unwrap_err();
        info!(
            "Decoded error: {}",
            decode_revert(&e, actions.contracts().pkpnft.abi()).to_string()
        );
        error!("Error sending transfer PKP: {:?}", e);
        return Err(anyhow::anyhow!("Error transferring PKP - sending tx"));
    }
    let tx = tx.unwrap();

    let tr = tx.await;
    if tr.is_err() {
        error!("Error waiting for transfer PKP: {:?}", tr.unwrap_err());
        return Err(anyhow::anyhow!("Error transferring PKP - waiting for tx"));
    }
    let tr = tr.unwrap();
    if tr.is_none() {
        error!("Error transferring PKP: No transaction receipt?");
        return Err(anyhow::anyhow!("Error transferring PKP - no tx receipt"));
    }

    Ok(())
}

#[doc = "Grant an action permission to use a PKP"]
pub async fn add_permitted_action_to_pkp(
    actions: &Actions,
    ipfs_cid: &str,
    token_id: U256,
    scopes: &[U256],
) -> Result<bool> {
    info!("ipfs_cid to permit: {}", ipfs_cid);

    let pacc = actions.contracts().pkp_permissions.add_permitted_action(
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

#[doc = "Grant a Address Authmethod permission to use a PKP"]
pub async fn add_permitted_address_auth_method_to_pkp(
    actions: &Actions,
    address_token: Vec<u8>,
    token_id: U256,
    scopes: &[U256],
) -> Result<bool> {
    let address_auth_method = AuthMethod {
        auth_method_type: U256::from(1),
        id: Bytes::from(address_token),
        user_pubkey: Bytes::from(vec![]),
    };
    debug!("Address Auth method to permit: {:?}", address_auth_method);

    let paam = actions
        .contracts()
        .pkp_permissions
        .add_permitted_auth_method(token_id, address_auth_method, scopes.to_vec());

    let tx = paam.send().await;
    if tx.is_err() {
        error!(
            "Error adding address authMethod to pkp: {:?}",
            tx.unwrap_err()
        );
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tx = tx.unwrap();

    let tr = tx.await;
    if tr.is_err() {
        error!(
            "Error adding address authMethod to pkp: {:?}",
            tr.unwrap_err()
        );
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tr = tr.unwrap();
    if tr.is_none() {
        error!("Error adding address authMethod to pkp: No transaction receipt?");
        return Err(anyhow::anyhow!("Error minting PKP"));
    }

    Ok(true)
}

#[doc = "Grant an address permission to use a PKP"]
pub async fn add_permitted_address_to_pkp(
    actions: &Actions,
    address: &str,
    token_id: U256,
    scopes: &[U256],
) -> Result<bool> {
    // this is happening under the deployer account, if nothing has changed it.
    let client = actions.deployer_signing_provider();

    add_permitted_address_to_pkp_with_wallet(client, actions, address, token_id, scopes).await
}

#[doc = "Grant an address permission to use a PKP"]
pub async fn add_permitted_address_to_pkp_with_wallet(
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    actions: &Actions,
    address: &str,
    token_id: U256,
    scopes: &[U256],
) -> Result<bool> {
    info!(
        "add_permitted_address_to_pkp_with_wallet() - adding address: {}",
        address
    );
    let addr = ethers::core::types::Address::from_str(address);
    assert!(addr.is_ok());
    let addr = addr.unwrap();
    let pkp_permissions_address = actions.contracts().pkp_permissions.address();
    let pkp_permissions = PKPPermissions::new(pkp_permissions_address, client.clone());
    let pacc = pkp_permissions.add_permitted_address(token_id, addr, scopes.to_vec());

    let tx = pacc.send().await;
    if tx.is_err() {
        error!("Error adding address to pkp: {:?}", tx.unwrap_err());
        return Err(anyhow::anyhow!(
            "Error adding address to PKP - couldn't send tx"
        ));
    }
    let tx = tx.unwrap();

    let tr = tx.await;
    if tr.is_err() {
        error!("Error adding address to pkp: {:?}", tr.unwrap_err());
        return Err(anyhow::anyhow!(
            "Error adding address to PKP - waiting for tx"
        ));
    }
    let tr = tr.unwrap();
    if tr.is_none() {
        error!("Error adding address to pkp: No transaction receipt?");
        return Err(anyhow::anyhow!(
            "Error adding address to PKP - no tx receipt"
        ));
    }

    let pa = pkp_permissions
        .get_permitted_addresses(token_id)
        .call()
        .await?;
    info!("permitted addresses: {:?}", pa);
    if !pa.contains(&addr) {
        return Err(anyhow::anyhow!("Address not added to permitted list"));
    }

    Ok(true)
}

#[doc = "Sign a 'hello world' message with a PKP"]
pub async fn sign_hello_world_with_pkp(actions: &Actions, pubkey: String) -> Result<bool> {
    let to_sign = "hello world".to_string();
    sign_message_with_pkp(actions, pubkey, to_sign).await
}

#[doc = "Sign bytes with a PKP"]
pub async fn sign_bytes_with_pkp(
    actions: &Actions,
    pubkey: String,
    to_sign: Vec<u8>,
) -> Result<(bool, Signature, RecoveryId)> {
    let auth_sig = generate_authsig_item(actions.deployer_signing_provider().signer()).await?;
    let epoch = actions.get_current_epoch().await.as_u64();
    let data_to_send = JsonPKPSigningRequest {
        auth_sig,
        to_sign,
        pubkey,
        auth_methods: None,
        epoch,
    };
    let result = sign_with_pkp_request(actions, data_to_send).await?;
    Ok(result)
}

#[doc = "Sign a message with a PKP"]
pub async fn sign_message_with_pkp(
    actions: &Actions,
    pubkey: String,
    to_sign: String,
) -> Result<bool> {
    let data_to_send =
        generate_data_to_send(pubkey, ethers::utils::keccak256(to_sign).into(), actions).await?;
    let result = sign_with_pkp_request(actions, data_to_send).await?;
    Ok(result.0)
}
pub async fn generate_data_to_send(
    pubkey: String,
    to_sign: Vec<u8>,
    actions: &Actions,
) -> Result<JsonPKPSigningRequest> {
    let epoch = actions.get_current_epoch().await.as_u64();
    generate_data_to_send_with_epoch(pubkey, to_sign, actions, epoch).await
}

pub async fn generate_data_to_send_with_epoch(
    pubkey: String,
    to_sign: Vec<u8>,
    actions: &Actions,
    epoch: u64,
) -> Result<JsonPKPSigningRequest> {
    let auth_sig = generate_authsig_item(actions.deployer_signing_provider().signer()).await?;
    let data_to_send = JsonPKPSigningRequest {
        auth_sig,
        to_sign,
        pubkey,
        auth_methods: None,
        epoch,
    };
    Ok(data_to_send)
}

pub async fn sign_message_with_hd_key(
    actions: &Actions,
    pubkey: String,
    to_sign: String,
) -> Result<bool> {
    let auth_sig = generate_authsig_item(actions.deployer_signing_provider().signer()).await?;
    let epoch = actions.get_current_epoch().await.as_u64();
    let data_to_send = JsonPKPSigningRequest {
        auth_sig,
        to_sign: to_sign.into_bytes(),
        pubkey,
        auth_methods: None,
        epoch,
    };
    let result = sign_with_pkp_request(actions, data_to_send).await?;
    Ok(result.0)
}

pub async fn send_signing_requests(
    actions: &Actions,
    data_to_send: JsonPKPSigningRequest,
) -> Result<Vec<String>> {
    // Send out our signature request to all the nodes.
    let json_body = serde_json::to_string(&data_to_send).unwrap();
    let cmd = "/web/pkp/sign".to_string();
    let endpoint_responses = hit_endpoints_with_json_body_join_all(actions, cmd, json_body).await?;

    Ok(endpoint_responses)
}

pub async fn sign_with_pkp_request(
    actions: &Actions,
    data_to_send: JsonPKPSigningRequest,
) -> Result<(bool, Signature, RecoveryId)> {
    // Remember, for ECDSA signatures we need 100% participation (API responses) from the deterministic subset,
    // which has the size of `get_threshold_count(validator_set)`.
    let expected_responses =
        get_threshold_count(get_current_validator_portnames(actions).await.len());

    let endpoint_responses = send_signing_requests(actions, data_to_send.clone()).await?;

    trace!("endpoint_responses: {:?}", endpoint_responses);

    assert!(endpoint_responses.len() >= expected_responses);

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
pub fn decode_endpoint_responses(
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
    validator_collection: &ValidatorCollection,
) -> Result<Vec<Value>> {
    let config_file = validator_collection.config_files()[0].clone();
    tracing::info!("config_file: {}", &config_file);
    std::env::set_var(ENV_LIT_CONFIG_FILE, &config_file);

    let cfg = lit_node::config::load_cfg().expect("failed to load LitConfig");
    let loaded_config = &cfg.load_full();

    let (_result, token_id) =
        generate_pkp_check_is_permitted_address(address, validator_collection.actions()).await;

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
    actions: &Actions,
) -> (Result<bool>, U256) {
    let minted_key = mint_next_pkp(actions).await;
    assert!(minted_key.is_ok());

    let minted_key = minted_key.unwrap();
    let token_id = minted_key.1;

    let res = add_permitted_address_to_pkp(actions, address, token_id, &[]).await;

    assert!(res.is_ok());
    (res, token_id)
}
