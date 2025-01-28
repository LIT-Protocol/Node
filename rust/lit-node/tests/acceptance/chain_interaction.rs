use std::sync::Arc;

use chrono::{Duration, SecondsFormat};
use ethers::core::k256::ecdsa::SigningKey;
use std::ops::Add;

use crate::acceptance::web_user_tests::assert_decrypted;
use crate::acceptance::web_user_tests::retrieve_decryption_key;

use test_common::auth_sig::generate_authsig;
use test_common::node_collection::get_network_pubkey;
use test_common::testnet::rate_limit_nfts::fund_wallet;
use test_common::testnet::Testnet;
use test_common::validator::ValidatorCollection;

#[allow(dead_code)]
use blsful::Bls12381G2Impl;
use ethers::contract::abigen;
use ethers::contract::Contract;
use ethers::contract::ContractFactory;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Middleware;
use ethers::signers::{LocalWallet, Signer, Wallet};
use ethers::types::Bytes;
use ethers::types::TransactionRequest;
use ethers::types::U256;
use ethers::utils::keccak256;
use ethers::utils::to_checksum;
use lit_core::utils::binary::hex_to_bytes;
use lit_node::tss::common::curve_type::CurveType;
use rand_core::OsRng;
use sha2::{Digest, Sha256};
use test_common::new_node_collection;

use lit_node::auth::auth_material::AuthSigItem;
use lit_node::auth::auth_material::{AuthMaterialType, JsonAuthSig};
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

use lit_node::auth::auth_material::{
    AUTH_SIG_DERIVED_VIA_CONTRACT_SIG, AUTH_SIG_DERIVED_VIA_CONTRACT_SIG_SHA256,
};

#[derive(Clone, Copy)]
enum HashType {
    Keccak256,
    Sha256,
}

impl HashType {
    fn hash(&self, message: &[u8]) -> [u8; 32] {
        match self {
            HashType::Keccak256 => {
                let hash = ethers::utils::hash_message(message).0;
                println!("Test Keccak256 hash: {:?}", hash);
                hash
            }
            HashType::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(message);
                hasher.finalize().into()
            }
        }
    }

    fn get_auth_sig_type(&self) -> String {
        match self {
            HashType::Keccak256 => AUTH_SIG_DERIVED_VIA_CONTRACT_SIG.to_string(),
            HashType::Sha256 => AUTH_SIG_DERIVED_VIA_CONTRACT_SIG_SHA256.to_string(),
        }
    }
}

async fn test_encryption_decryption_eip1271(
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
    hash_type: HashType,
) {
    // setup our wallet
    let wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
    let provider = testnet.provider.clone();
    fund_wallet(&provider, &wallet, "100000000000000000000").await;
    let actions = validator_collection.actions();

    // deploy the eip1271 contract
    let contract_name = "EIP1271";
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

    info!("EIP1271 deployed to {:?}", contract_address);

    abigen!(EIP1271, "./tests/acceptance/contracts/EIP1271/EIP1271.json");
    let contract = EIP1271::new(contract_address, Arc::new(client.clone()));

    // validate that the contract works
    let valid_result = "0x1626ba7e";
    let invalid_result = "ffffffff";
    let valid_result_bytes: Bytes = hex_to_bytes(valid_result).unwrap().into();
    let invalid_result_bytes: Bytes = hex_to_bytes(invalid_result).unwrap().into();

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

    // Encrypt
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

    info!("1. FAIL: Should not decrypt since we passing a random _hash signed by the user which could be available on-chain instead of a valid unexpired SIWE message");
    let message_to_sign = "Random message signed by the owner";
    let hashed_message = hash_type.hash(message_to_sign.as_bytes());

    let signature = wallet
        .sign_hash(ethers::types::TxHash(hashed_message))
        .unwrap();
    let sig_bytes: Bytes = signature.to_vec().into();

    let is_valid = contract
        .is_valid_signature(hashed_message.into(), sig_bytes.clone())
        .call()
        .await
        .unwrap();
    let is_valid_bytes: Bytes = is_valid.into();
    assert_eq!(is_valid_bytes, valid_result_bytes);

    info!("1.1. isValidSignature() succeeded on-chain but the nodes won't validate this as it's not a valid unexpired SIWE message. So any random message or `_hash` signed by the user won't be permitted by the nodes");
    // create a random EIP1271 authsig
    let auth_sig = JsonAuthSig::new_with_type(
        format!("0x{:}", &signature.to_string()),
        hash_type.get_auth_sig_type(),
        message_to_sign.to_string(),
        to_checksum(&contract_address, None),
        None,
        AuthMaterialType::ContractSig,
        None,
    );
    info!("1.2. Random auth_sig: {:?}", auth_sig);
    let decryption_resp = retrieve_decryption_key(
        actions,
        access_control_conditions.clone(),
        None,
        None,
        None,
        Some(chain.clone()),
        data_to_encrypt_hash.clone(),
        &auth_sig,
    )
    .await;

    for response in &decryption_resp {
        println!("response- {:?}", response);
        assert!(response.contains("NodeInvalidAuthSig"));
        assert!(response.contains("Parse error on SIWE"));
    }

    info!("2. FAIL: Should not decrypt even though we are passing a valid unexpired SIWE message from another user which fails the EIP1271 contract check");
    // create a EIP1271 authsig with a different wallet
    let non_permitted_wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
    let siwe_message = get_siwe_message(&non_permitted_wallet);
    let siwe_message_hash = hash_type.hash(siwe_message.as_bytes());

    // User signs the hash of the SIWE message NOT the original SIWE message since `isValidSignature()` verifies the `_signature` against the `_hash`
    let siwe_signature = non_permitted_wallet
        .sign_hash(ethers::types::TxHash(siwe_message_hash))
        .unwrap();

    // validate that the contract works not for the a non-permitted wallet's SIWE hash signature
    let siwe_sig_bytes: Bytes = siwe_signature.to_vec().into();
    let is_valid = contract
        .is_valid_signature(siwe_message_hash.into(), siwe_sig_bytes.clone())
        .call()
        .await
        .unwrap();
    let is_valid_bytes: Bytes = is_valid.into();
    assert_eq!(is_valid_bytes, invalid_result_bytes);

    info!("2.1. SIWE sig NOT validated on-chain since it's from a non-permitted wallet as per the EIP1271 contract.");
    // create a valid SIWE EIP1271 authsig
    let auth_sig = JsonAuthSig::new_with_type(
        format!("0x{:}", &siwe_signature.to_string()),
        hash_type.get_auth_sig_type(),
        siwe_message,
        to_checksum(&contract_address, None),
        None,
        AuthMaterialType::ContractSig,
        None,
    );

    info!("2.2. Non-permitted SIWE auth_sig: {:?}", auth_sig);
    let decryption_resp = retrieve_decryption_key(
        actions,
        access_control_conditions.clone(),
        None,
        None,
        None,
        Some(chain.clone()),
        data_to_encrypt_hash.clone(),
        &auth_sig,
    )
    .await;

    for response in &decryption_resp {
        println!("response- {:?}", response);
        assert!(response.contains("Access control failed for Smart contract"));
        assert!(response.contains("EIP1271 Authsig failed"));
        assert!(response.contains("Return value was ffffffff."));
    }

    info!("3. PASS: Should decrypt since we passing a 'unhashed' valid SIWE message as the _hash");
    let siwe_message = get_siwe_message(&wallet);
    let siwe_message_hash = hash_type.hash(siwe_message.as_bytes());

    // User signs the hash of the SIWE message NOT the original SIWE message since `isValidSignature()` verifies the `_signature` against the `_hash`
    let siwe_signature = wallet
        .sign_hash(ethers::types::TxHash(siwe_message_hash))
        .unwrap();

    // validate that the contract works for the SIWE hash signature
    let siwe_sig_bytes: Bytes = siwe_signature.to_vec().into();
    let is_valid = contract
        .is_valid_signature(siwe_message_hash.into(), siwe_sig_bytes.clone())
        .call()
        .await
        .unwrap();
    let is_valid_bytes: Bytes = is_valid.into();
    assert_eq!(is_valid_bytes, valid_result_bytes);

    info!("3.1. SIWE sig validated on-chain since it's from the owner wallet");
    // create a valid SIWE EIP1271 authsig
    let auth_sig = JsonAuthSig::new_with_type(
        format!("0x{:}", &siwe_signature.to_string()),
        hash_type.get_auth_sig_type(),
        siwe_message,
        to_checksum(&contract_address, None),
        None,
        AuthMaterialType::ContractSig,
        None,
    );

    info!("3.2. Valid SIWE auth_sig: {:?}", auth_sig);
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
    println!("decryption_resp: {:?}", decryption_resp);

    assert_decrypted(
        validator_collection.size().try_into().unwrap(),
        &network_pubkey,
        identity_param,
        to_encrypt,
        &ciphertext,
        decryption_resp,
    );
}

async fn test_encryption_decryption_eip1271_keccak256(
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
) {
    info!("Testing decryption with eip1271_keccka256");
    test_encryption_decryption_eip1271(testnet, validator_collection, HashType::Keccak256).await;
}

async fn test_encryption_decryption_eip1271_sha256(
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
) {
    info!("Testing decryption with eip1271_sha256");
    test_encryption_decryption_eip1271(testnet, validator_collection, HashType::Sha256).await;
}

#[tokio::test]
async fn test_chain_interaction() {
    test_common::init_test_config();
    let num_nodes = 6;
    let (testnet, validator_collection) = new_node_collection(num_nodes, false).await;

    test_encryption_decryption_eip1271_keccak256(&testnet, &validator_collection).await;
    test_encryption_decryption_eip1271_sha256(&testnet, &validator_collection).await;
}

fn get_siwe_message(wallet: &Wallet<SigningKey>) -> String {
    let chain_id = wallet.chain_id();
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

    message
}
