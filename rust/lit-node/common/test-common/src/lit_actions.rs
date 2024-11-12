use super::node_collection::{
    hit_endpoints_with_json_body_join_all, hit_endpoints_with_json_body_per_port,
};
use super::testnet::actions::Actions;
use super::testnet::Testnet;
use super::validator::ValidatorCollection;
use crate::auth_sig::generate_authsig_item;
use crate::pkp::mint_next_pkp;
use anyhow::Result;
use cait_sith::combine_signature_shares;
use ethers::types::U256;

use k256::{Scalar, Secp256k1};
use lazy_static::lazy_static;
use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_node::auth::auth_material::{AuthSigItem, JsonAuthSig};
use lit_node::models::{AuthMethod, JsonExecutionRequest, SignedData};
use lit_node::p2p_comms::web::models::SignedMessageShare;
use lit_node::pkp::utils::pkp_permissions_get_permitted;
use lit_node::tss::common::curve_type::CurveType;
use lit_node::tss::ecdsa_cait_sith::protocols256k1::CsSigshare;
use lit_node::utils::web::EndpointVersion;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActionReturn {
    success: bool,
    signed_data: HashMap<String, SignedData>,
    response: String,
    logs: String,
}

pub const HELLO_WORLD_LIT_ACTION_CODE: &str = "const go = async () => {
  // this requests a signature share from the Lit Node
  // the signature share will be automatically returned in the response from the node
  // and combined into a full signature by the LitJsSdk for you to use on the client
  // all the params (toSign, publicKey, sigName) are passed in from the LitJsSdk.executeJs() function

  let utf8Encode = new TextEncoder();
  const toSign = utf8Encode.encode('This message is exactly 32 bytes');
  const sigShare = await Lit.Actions.signEcdsa({ toSign, publicKey, sigName });
};
go();";

const CALL_CHILD_LIT_ACTION_CODE: &str = "const go = async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    const _ = await Lit.Actions.call({ ipfsId: 'QmRwN9GKHvCn4Vk7biqtr6adjXMs7PzzYPCzNCRjPFiDjm', params: {
        toSign: Array.from(toSign),
        publicKey,
        sigName
    }});
  };
  go();";

lazy_static! {
    static ref CONTRACT_CALL_LIT_ACTION_CODE: String = r#"
            const go = async () => {
                // https://sepolia.etherscan.io/address/0xD2f13AeACd77bB8D0aD79c6dB5F081e358b481C2#code
                const toContract = "0xD2f13AeACd77bB8D0aD79c6dB5F081e358b481C2";

                const abi = [{"inputs":[],"stateMutability":"nonpayable","type":"constructor"},{"inputs":[{"internalType":"uint256","name":"a","type":"uint256"},{"internalType":"uint256","name":"b","type":"uint256"}],"name":"add","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"}];

                const contract = new ethers.Contract(toContract, abi);
                const rawTxn = await contract.populateTransaction.add(1,2);
                const txn = ethers.utils.serializeTransaction(rawTxn);
                
                const chain = "sepolia";

                const res = await LitActions.callContract({
                    chain,
                    txn
                });

                // decode response
                const decodedResult = contract.interface.decodeFunctionResult("add", res)[0].toString();

                Lit.Actions.setResponse({response: decodedResult});
            };
            go();
            "#.to_string();
}

async fn prepare_sign_hello_world_parameters(
    actions: &Actions,
) -> Result<(
    String,
    Option<String>,
    Option<serde_json::Value>,
    Option<Vec<AuthMethod>>,
)> {
    let lit_action_code = HELLO_WORLD_LIT_ACTION_CODE.to_string();

    let pkp_info = mint_next_pkp(actions).await?;
    let pubkey = pkp_info.0;

    sign_lit_action(lit_action_code, pubkey).await
}

pub async fn sign_lit_action(
    lit_action_code: String,
    pubkey: String,
) -> Result<(
    String,
    Option<String>,
    Option<serde_json::Value>,
    Option<Vec<AuthMethod>>,
)> {
    let lit_action_code = data_encoding::BASE64.encode(lit_action_code.as_bytes());
    // let auth_sig = generate_authsig_item(&node_collection.config_files[0]).await;

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.into());
    js_params.insert("sigName".to_string(), "sig1".into());

    Ok((
        lit_action_code,
        None,
        Some(serde_json::Value::Object(js_params)),
        None,
    ))
}

pub async fn sign_using_child_lit_action(
    validator_collection: &ValidatorCollection,
) -> Result<bool> {
    let actions = validator_collection.actions();
    let signing_provider = actions.deployer_signing_provider();
    let wallet = signing_provider.signer();
    let auth_sig = generate_authsig_item(wallet).await?;

    let lit_action_code = CALL_CHILD_LIT_ACTION_CODE.to_string();

    let pkp_info = mint_next_pkp(actions).await?;
    let pubkey = pkp_info.0;

    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        sign_lit_action(lit_action_code, pubkey).await.unwrap();

    // run
    let execute_resp = execute_lit_action(
        actions,
        Some(lit_action_code),
        ipfs_id,
        js_params,
        auth_methods,
        auth_sig,
        EndpointVersion::Initial,
    )
    .await?;

    // assert
    let result = assert_signed_action(validator_collection, execute_resp).await?;

    Ok(result)
}

pub async fn sign_hello_world_with_auth_sig_item(
    validator_collection: &ValidatorCollection,
    auth_sig: AuthSigItem,
) -> Result<bool> {
    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        prepare_sign_hello_world_parameters(validator_collection.actions()).await?;

    // run
    let execute_resp = execute_lit_action(
        validator_collection.actions(),
        Some(lit_action_code),
        ipfs_id,
        js_params,
        auth_methods,
        auth_sig,
        EndpointVersion::Initial,
    )
    .await?;

    // assert
    let result = assert_signed_action(validator_collection, execute_resp).await?;

    Ok(result)
}

pub async fn sign_hello_world(
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
) -> Result<bool> {
    let wallet = testnet.deploy_account.signing_provider.signer();
    let auth_sig = generate_authsig_item(wallet).await?;

    sign_hello_world_with_auth_sig_item(validator_collection, auth_sig).await
}

pub async fn sign_hello_world_session_sigs(
    validator_collection: &ValidatorCollection,
    actions: &Actions,
    session_sigs: &Vec<JsonAuthSig>,
) -> Result<bool> {
    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        prepare_sign_hello_world_parameters(actions).await?;

    let epoch = actions.get_current_epoch().await.as_u64();
    // run
    let execute_resp = execute_lit_action_session_sigs(
        validator_collection,
        lit_action_code,
        ipfs_id,
        js_params,
        auth_methods,
        session_sigs,
        EndpointVersion::Initial,
        epoch,
    )
    .await?;

    // assert
    let result = assert_signed_action(validator_collection, execute_resp).await?;

    Ok(result)
}

pub async fn prepare_sign_from_file_parameters(
    actions: &Actions,
    file_name: &str,
) -> Result<(
    String,
    Option<String>,
    Option<serde_json::Value>,
    Option<Vec<AuthMethod>>,
)> {
    info!("Attempting to run lit action from file: {}", file_name);
    let lit_action_code = std::fs::read_to_string(file_name).unwrap();

    let pkp_info = mint_next_pkp(actions).await?;
    let pubkey = pkp_info.0;

    Ok(sign_lit_action(lit_action_code, pubkey).await.unwrap())
}

pub async fn sign_from_file_system_with_auth_sig_item(
    validator_collection: &ValidatorCollection,
    file_name: &str,
    auth_sig: AuthSigItem,
    assert_sig: bool,
) -> Result<bool> {
    let path = std::path::Path::new(file_name);

    if !path.exists() {
        error!("File does not exist: {}", file_name);
        return Ok(false);
    }

    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        prepare_sign_from_file_parameters(validator_collection.actions(), file_name).await?;

    info!("lit_action_code: {}", lit_action_code);
    // run
    let execute_resp = execute_lit_action(
        validator_collection.actions(),
        Some(lit_action_code),
        ipfs_id,
        js_params,
        auth_methods,
        auth_sig,
        EndpointVersion::Initial,
    )
    .await?;

    info!("execute_resp: {:?}", execute_resp);

    if !assert_sig {
        return Ok(true);
    };

    // assert
    let result = assert_signed_action(validator_collection, execute_resp).await?;

    Ok(result)
}

pub async fn sign_from_file_system(
    validator_collection: &ValidatorCollection,
    testnet: &Testnet,
    file_name: &str,
    assert_sig: bool,
) -> Result<bool> {
    let wallet = testnet.deploy_account.signing_provider.signer();
    let auth_sig = generate_authsig_item(wallet).await?;
    sign_from_file_system_with_auth_sig_item(validator_collection, file_name, auth_sig, assert_sig)
        .await
}

pub async fn execute_lit_action_session_sigs(
    validator_collection: &ValidatorCollection,
    lit_action_code: String,
    ipfs_id: Option<String>,
    js_params: Option<serde_json::Value>,
    auth_methods: Option<Vec<AuthMethod>>,
    session_sigs: &Vec<JsonAuthSig>,
    endpoint_version: EndpointVersion,
    epoch: u64,
) -> Result<Vec<String>> {
    let mut json_body_vec = Vec::new();

    // Generate JSON body for each port
    for i in 0..validator_collection.size() {
        let execute_request = JsonExecutionRequest {
            auth_sig: AuthSigItem::Single(session_sigs[i].to_owned()),
            code: Some(lit_action_code.clone()),
            ipfs_id: ipfs_id.clone(),
            js_params: Some(js_params.clone().unwrap_or_default()),
            auth_methods: auth_methods.clone(),
            epoch,
        };
        let json_body = serde_json::to_string(&execute_request).unwrap();
        json_body_vec.push(json_body);
    }

    let cmd = match endpoint_version {
        EndpointVersion::Initial => "/web/execute",
        EndpointVersion::V1 => "web/execute/v1",
    };

    Ok(hit_endpoints_with_json_body_per_port(
        validator_collection.actions(),
        cmd.to_string(),
        json_body_vec,
    )
    .await)
}

pub async fn execute_lit_action(
    actions: &Actions,
    lit_action_code: Option<String>,
    ipfs_id: Option<String>,
    js_params: Option<serde_json::Value>,
    auth_methods: Option<Vec<AuthMethod>>,
    auth_sig_item: AuthSigItem,
    endpoint_version: EndpointVersion,
) -> Result<Vec<String>> {
    let epoch = actions.get_current_epoch().await.as_u64();

    let execute_request = JsonExecutionRequest {
        auth_sig: auth_sig_item,
        code: lit_action_code,
        ipfs_id,
        js_params,
        auth_methods,
        epoch,
    };

    let json_body = serde_json::to_string(&execute_request).unwrap();

    let cmd = match endpoint_version {
        EndpointVersion::Initial => "/web/execute",
        EndpointVersion::V1 => "web/execute/v1",
    };

    hit_endpoints_with_json_body_join_all(actions, cmd.to_string(), json_body).await
}

pub async fn assert_signed_action(
    validator_collection: &ValidatorCollection,
    execute_resp: Vec<String>,
) -> Result<bool> {
    let num_nodes = validator_collection.size();
    let threshold = validator_collection.threshold();
    assert_eq!(execute_resp.len(), num_nodes);

    let mut sig_response: Vec<CsSigshare> = Vec::new();
    let mut ecdsa_message_share: Vec<SignedMessageShare> = Vec::new();

    info!("execute_resp: {:?}", &execute_resp);

    for r in execute_resp {
        let result: std::result::Result<ActionReturn, serde_json::Error> =
            serde_json::from_str::<ActionReturn>(&r);
        if result.is_err() {
            error!("Error parsing response as ActionReturn: {}", &r);
            return Ok(false);
        }

        let result = result.unwrap();
        info!("Result: {:?}", &result);

        let signed_data = result.signed_data.iter().last().unwrap().1;
        info!("Signed Data: {:?}", &signed_data);

        if signed_data.signature_share.len() > 1 {
            let ecdsa_msg_share = SignedMessageShare {
                digest: signed_data.data_signed.clone(),
                result: result.success.to_string(),
                signature_share: signed_data.signature_share.clone(),
                big_r: signed_data.big_r.clone(),
                public_key: signed_data.public_key.clone(),
                share_index: signed_data.share_index as usize,
                sig_type: signed_data.sig_type.clone(),
            };

            info!("Ecdsa msg share: {:?}", &ecdsa_msg_share);

            ecdsa_message_share.push(ecdsa_msg_share);

            let sig_share = CsSigshare {
                share: serde_json::from_str(&signed_data.signature_share).unwrap_or(Scalar::ZERO),
                public_key: serde_json::from_str(&signed_data.public_key)
                    .unwrap_or(k256::AffinePoint::IDENTITY),
                presignature_big_r: serde_json::from_str(&signed_data.big_r)
                    .unwrap_or(k256::AffinePoint::IDENTITY),
                msg_hash: serde_json::from_str(&signed_data.data_signed).unwrap_or(Scalar::ZERO),
            };

            sig_response.push(sig_share);
        }
    }

    assert!(
        sig_response.len() >= threshold,
        "Not enough sig shares.  Got: {} but expected {}",
        sig_response.len(),
        threshold
    );

    info!("Sig sig_response: {:?}", &sig_response);

    if ecdsa_message_share[0].sig_type == CurveType::K256.to_string() {
        info!("Cait-Sith");
        let public_key = sig_response[0].public_key;
        let presignature_big_r = sig_response[0].presignature_big_r;
        let msg_hash = sig_response[0].msg_hash;

        let mut sig_shares: Vec<Scalar> = Vec::new();
        for s in sig_response {
            sig_shares.push(s.share);
        }

        let sig = combine_signature_shares::<Secp256k1>(
            sig_shares,
            public_key,
            presignature_big_r,
            msg_hash,
        )
        .await;
        let sig = sig.unwrap();

        let is_valid = sig.verify(&public_key, &msg_hash);

        if is_valid {
            info!("Cait-Sith Sig is valid");
        } else {
            info!("Cait-Sith Sig is not valid");
        }

        assert!(is_valid);
    } else {
        panic!("Unsupported curve type");
    }
    Ok(true)
}

pub async fn generate_pkp_check_get_permitted_pkp_action(
    ipfs_cid: &str,
    validator_collection: &ValidatorCollection,
) -> Result<(String, Vec<Value>)> {
    let config_file = validator_collection.config_files()[0].clone();

    std::env::set_var(ENV_LIT_CONFIG_FILE, config_file);

    let cfg = lit_node::config::load_cfg().expect("failed to load LitConfig");
    let loaded_config = &cfg.load_full();

    let pkp_info = mint_next_pkp(validator_collection.actions()).await?;
    let token_id = pkp_info.1;
    let pkp_pubkey = pkp_info.0;

    let res = crate::pkp::add_permitted_action_to_pkp(
        validator_collection.actions(),
        ipfs_cid,
        token_id,
        &[U256::from(1)],
    )
    .await;

    assert!(res.is_ok());

    let res = lit_node::pkp::utils::pkp_permissions_get_permitted(
        String::from("getPermittedActions"),
        loaded_config.as_ref(),
        token_id.to_string(),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Error getting permitted actions: {:?}", e));

    assert!(res.is_ok());
    Ok((pkp_pubkey, res.unwrap()))
}

pub async fn generate_pkp_check_is_permitted_pkp_action(
    ipfs_cid: &str,
    validator_collection: &ValidatorCollection,
) -> Result<bool> {
    let config_file = validator_collection.config_files()[0].clone();

    std::env::set_var(ENV_LIT_CONFIG_FILE, config_file);

    let cfg = lit_node::config::load_cfg().expect("failed to load LitConfig");
    let loaded_config = &cfg.load_full();

    let pkp_info = mint_next_pkp(validator_collection.actions()).await?;
    let token_id = pkp_info.1;

    let res = crate::pkp::add_permitted_action_to_pkp(
        validator_collection.actions(),
        ipfs_cid,
        token_id,
        &[U256::from(1)],
    )
    .await;
    assert!(res.is_ok());

    let res = lit_node::pkp::utils::pkp_permissions_is_permitted(
        token_id.to_string(),
        loaded_config.as_ref(),
        String::from("isPermittedAction"),
        [Value::from(ipfs_cid)].to_vec(),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Error getting permitted actions: {:?}", e));

    assert!(res.is_ok());
    res
}

pub async fn generate_pkp_check_get_permitted_address(
    address: &str,
    validator_collection: &ValidatorCollection,
) -> Result<Vec<Value>> {
    let config_file = validator_collection.config_files()[0].clone();
    std::env::set_var(ENV_LIT_CONFIG_FILE, config_file);

    let cfg = lit_node::config::load_cfg().expect("failed to load LitConfig");
    let loaded_config = &cfg.load_full();

    let minted_key = mint_next_pkp(validator_collection.actions()).await;
    assert!(minted_key.is_ok());
    let (_pubkey, token_id, _) = minted_key.unwrap();

    let _add_res = crate::pkp::add_permitted_address_to_pkp(
        validator_collection.actions(),
        address,
        token_id,
        &[U256::from(1)],
    )
    .await;

    let res = pkp_permissions_get_permitted(
        String::from("getPermittedAddresses"),
        loaded_config.as_ref(),
        token_id.to_string(),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Error getting permitted addresses: {:?}", e));

    assert!(res.is_ok());
    res
}

pub async fn generate_pkp_check_is_permitted_address(
    param: &str,
    validator_collection: &ValidatorCollection,
) -> Result<bool> {
    let config_file = validator_collection.config_files()[0].clone();
    std::env::set_var(ENV_LIT_CONFIG_FILE, config_file);

    let cfg = lit_node::config::load_cfg().expect("failed to load LitConfig");
    let _loaded_config = &cfg.load_full();

    let minted_key = mint_next_pkp(validator_collection.actions()).await;
    assert!(minted_key.is_ok());

    let minted_key = minted_key.unwrap();
    let token_id = minted_key.1;

    let res = crate::pkp::add_permitted_address_to_pkp(
        validator_collection.actions(),
        param,
        token_id,
        &[U256::from(1)],
    )
    .await;

    assert!(res.is_ok());
    res
}

#[cfg(feature = "lit-actions")]
pub async fn add_permitted_action_to_pkp(
    ipfs_cid: &str,
    token_id: U256,
    actions: &Actions,
) -> Result<bool> {
    use ethers::types::Bytes;

    let ipfs_cid = bs58::decode(ipfs_cid).into_vec()?;
    let ipfs_cid = Bytes::from(ipfs_cid);

    info!(
        "adding cid permission to tokenId: {} cid: {}",
        token_id, ipfs_cid
    );

    let mut scopes = Vec::new();
    scopes.push(token_id);

    let pacc = actions
        .contracts()
        .pkp_permissions
        .add_permitted_action(token_id, ipfs_cid, scopes);

    let tx = pacc.send().await;
    if tx.is_err() {
        error!("Error minting PKP: {:?}", tx.unwrap_err());
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tx = tx.unwrap();

    let tr = tx.await;
    if tr.is_err() {
        error!("Error minting PKP: {:?}", tr.unwrap_err());
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tr = tr.unwrap();
    if tr.is_none() {
        error!("Error minting PKP: No transaction receipt?");
        return Err(anyhow::anyhow!("Error minting PKP"));
    }

    Ok(true)
}
