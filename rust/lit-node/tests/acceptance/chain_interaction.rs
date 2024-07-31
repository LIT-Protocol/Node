use std::sync::Arc;

use crate::acceptance::web_user_tests::assert_decrypted;
use crate::acceptance::web_user_tests::retrieve_decryption_key;
use crate::common::auth_sig::get_auth_sig_for_session_sig;
use crate::common::auth_sig::get_session_sigs_for_pkp;
use crate::common::lit_actions::assert_signed_action;
use crate::common::lit_actions::execute_lit_action_session_sigs;
use crate::common::lit_actions::sign_lit_action;
use crate::common::lit_actions::HELLO_WORLD_LIT_ACTION_CODE;
use crate::common::node_collection::get_network_pubkey;
use crate::common::node_collection::hit_endpoints_with_json_body;
use crate::common::node_collection::hit_endpoints_with_json_body_per_port;
use crate::common::pkp::add_permitted_address_to_pkp_with_wallet;
use crate::common::pkp::mint_next_pkp_with_wallet;
use crate::common::testnet::actions::Actions;
use crate::common::testnet::contracts::StakingContractConfig;
use crate::common::testnet::node_config::CustomNodeRuntimeConfig;
use crate::common::testnet::rate_limit_nfts::create_payment_delegation_entry;
use crate::common::testnet::rate_limit_nfts::fund_wallet;
use crate::common::testnet::rate_limit_nfts::mint_rate_limit_nft;
use crate::common::testnet::Testnet;
use crate::common::validator::ValidatorCollection;
// use ethers::{prelude::*, solc::Solc};
use std::path::Path;

#[allow(dead_code)]
use super::super::common;
use base64_light::base64_decode;
use blsful::Bls12381G2Impl;
use common::auth_sig::{
    generate_authsig, get_auth_sig_with_rli_nft_resources, get_session_sigs_for_auth,
};
use common::new_node_collection;
use ethers::abi::Abi;
use ethers::contract::abigen;
use ethers::contract::Contract;
use ethers::contract::ContractFactory;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Middleware;
use ethers::signers::LocalWallet;
use ethers::signers::Signer;
use ethers::types::Bytes;
use ethers::types::TransactionRequest;
use ethers::types::U256;
use ethers::utils::keccak256;
use ethers::utils::to_checksum;
use lit_core::utils::binary::hex_to_bytes;
use lit_node::tss::common::curve_type::CurveType;
use rand_core::OsRng;
use sha2::{Digest, Sha256};

use lit_node::auth::auth_material::AuthSigItem;
use lit_node::auth::auth_material::JsonAuthSig;
use lit_node::auth::lit_resource::LitResource;
use lit_node::auth::resources::AccessControlConditionResource;
use lit_node::constants::CHAIN_LOCALCHAIN;
use lit_node::utils::encoding::bytes_to_hex;
use lit_node::utils::web::EndpointVersion;

use lit_node::models;
use lit_node::models::auth::LitAbility;
use lit_node::models::auth::LitResourceAbilityRequest;
use lit_node::models::auth::LitResourceAbilityRequestResource;
use lit_node::models::auth::LitResourcePrefix;
use lit_node::models::AccessControlConditionItem;
use lit_node::models::EVMContractConditionItem;
use lit_node::models::EncryptionSignRequest;
use lit_node::models::EncryptionSignResponse;

// use lit_node::models::JwtPayloadV2;
use lit_node::models::RequestConditions;
// use lit_node::models::SigningAccessControlConditionRequest;
// use lit_node::models::SigningAccessControlConditionResponse;
use lit_node::models::SolRpcConditionItem;
use lit_node::models::UnifiedAccessControlConditionItem;

use lit_node::utils::web::hash_access_control_conditions;

use tracing::{debug, info};

async fn test_encryption_decryption_eip1271(
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
) {
    // setup our wallet
    let wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
    let provider = testnet.provider.clone();
    fund_wallet(&provider, &wallet, "100000000000000000000").await;
    let actions = validator_collection.actions();

    // deploy the eip1271 contract
    let contract_name = "EIP1271";
    // let abi = include_bytes!("contracts/EIP1271/EIP1271.abi");
    // let abi_str = std::str::from_utf8(abi).expect("ABI should be valid UTF-8");
    // let abi_struct: Abi = serde_json::from_str(abi_str).unwrap();
    let deploy_txn = include_str!("contracts/EIP1271/EIP1271_deploytxn.hex");
    let deploy_txn = hex_to_bytes(deploy_txn).unwrap();
    let deploy_txn: Bytes = deploy_txn.into();
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    let tx = TransactionRequest::new()
        .data(deploy_txn)
        .chain_id(testnet.chain_id)
        .from(wallet.address());

    let pending = client.send_transaction(tx, None).await.unwrap();
    let receipt = pending.await.unwrap().unwrap();
    let contract_address = receipt.contract_address.unwrap();

    println!("EIP1271 deployed to {:?}", contract_address);

    abigen!(EIP1271, "./tests/acceptance/contracts/EIP1271/EIP1271.json");
    let contract = EIP1271::new(contract_address, Arc::new(client.clone()));

    // load the contract
    // let contract = Contract::new(contract_address, abi_struct, Arc::new(client.clone()));

    // create a signed message
    let message_to_sign = "this is a test message";
    // hash message
    let hashed_message = keccak256(message_to_sign.as_bytes());

    let signature = wallet
        .sign_hash(ethers::types::TxHash(hashed_message))
        .unwrap();
    let sig_bytes: Bytes = signature.to_vec().into();
    // validate that the contract works
    let valid_result = "0x1626ba7e";
    let valid_result_bytes: Bytes = hex_to_bytes(valid_result).unwrap().into();

    let is_valid = contract
        .is_valid_signature(hashed_message, sig_bytes.clone())
        .call()
        .await
        .unwrap();
    let is_valid_bytes: Bytes = is_valid.into();
    assert_eq!(is_valid_bytes, valid_result_bytes);

    // okay now let's try encrypting with this condition and then decrypting
    let chain = CHAIN_LOCALCHAIN.to_string();
    let access_control_conditions = Some(vec![
        lit_node::models::AccessControlConditionItem::Condition(
            models::JsonAccessControlCondition {
                contract_address: "".to_string(),
                chain: chain.clone(),
                standard_contract_type: "".to_string(),
                method: "".to_string(),
                parameters: vec![":userAddress".to_string()],
                return_value_test: models::JsonReturnValueTest {
                    comparator: "=".to_string(),
                    value: format!("0x{}", bytes_to_hex(contract_address.as_bytes())),
                },
            },
        ),
    ]);

    // Get the resource key
    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: access_control_conditions.clone(),
        evm_contract_conditions: None,
        sol_rpc_conditions: None,
        unified_access_control_conditions: None,
    })
    .unwrap();

    // Encrypt.
    let to_encrypt = "super secret message";
    let mut hasher = Sha256::new();
    hasher.update(to_encrypt.as_bytes());
    let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());
    let network_pubkey = get_network_pubkey(validator_collection.actions()).await;
    let message_bytes = to_encrypt.as_bytes();
    let identity_param = AccessControlConditionResource::new(format!(
        "{}/{}",
        hashed_access_control_conditions, data_to_encrypt_hash
    ))
    .get_resource_key()
    .into_bytes();
    let ciphertext = lit_bls_wasm::encrypt_time_lock::<Bls12381G2Impl>(
        &network_pubkey,
        message_bytes.to_vec(),
        identity_param.clone(),
    )
    .expect("Unable to encrypt");
    info!("ciphertext: {:?}", ciphertext);

    // okay, now try to decrypt

    // create the EIP1271 authsig
    let auth_sig = JsonAuthSig::new(
        format!("0x{}", bytes_to_hex(sig_bytes)),
        "EIP1271".to_string(),
        format!("0x{}", bytes_to_hex(hashed_message)),
        to_checksum(&contract_address, None),
        None,
    );

    // Retrieve decrypted key
    let decryption_resp = retrieve_decryption_key(
        actions,
        access_control_conditions,
        None,
        None,
        None,
        Some(chain.clone()),
        data_to_encrypt_hash,
        &auth_sig,
    )
    .await;

    assert_decrypted(
        validator_collection.size().try_into().unwrap(),
        &network_pubkey,
        identity_param,
        to_encrypt,
        &ciphertext,
        decryption_resp,
    );
}

#[tokio::test]
async fn test_chain_interaction() {
    common::init_test_config();
    // use initial_node_setup if you don't have a DKG result saved.
    let num_nodes = 6;

    let (testnet, validator_collection) = new_node_collection(num_nodes, false).await;
    info!("Testing decryption with eip1271");
    test_encryption_decryption_eip1271(&testnet, &validator_collection).await;
}
