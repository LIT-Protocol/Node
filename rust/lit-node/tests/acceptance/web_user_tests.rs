use std::sync::Arc;

use test_common::auth_sig::get_auth_sig_for_session_sig;
use test_common::auth_sig::get_session_sigs_for_pkp;
use test_common::lit_actions::assert_signed_action;
use test_common::lit_actions::execute_lit_action_session_sigs;
use test_common::lit_actions::sign_lit_action;
use test_common::lit_actions::HELLO_WORLD_LIT_ACTION_CODE;
use test_common::node_collection::get_network_pubkey;
use test_common::node_collection::hit_endpoints_with_json_body;
use test_common::node_collection::hit_endpoints_with_json_body_per_port;
use test_common::pkp::add_permitted_address_to_pkp_with_wallet;
use test_common::pkp::mint_next_pkp_with_wallet;
use test_common::testnet::actions::Actions;
use test_common::testnet::contracts::StakingContractConfig;
use test_common::testnet::node_config::CustomNodeRuntimeConfig;
use test_common::testnet::rate_limit_nfts::{
    create_payment_delegation_entry, fund_wallet, mint_rate_limit_nft, set_restrictions,
};
use test_common::testnet::Testnet;
use test_common::validator::ValidatorCollection;

#[allow(dead_code)]
use base64_light::base64_decode;
use blsful::Bls12381G2Impl;
use ethers::middleware::SignerMiddleware;
use ethers::signers::LocalWallet;
use ethers::signers::Signer;
use ethers::types::U256;
use lit_blockchain::contracts::rate_limit_nft::RateLimitNFT;
use lit_node::tss::common::curve_type::CurveType;
use rand_core::OsRng;
use sha2::{Digest, Sha256};
use test_common::auth_sig::{
    generate_authsig, get_auth_sig_with_rli_nft_resources, get_session_sigs_for_auth,
};
use test_common::new_node_collection;

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

//###### Use chain as in integration_tests.rs, but also hit node signing endpoints

// # Perform only an initial dkg, looking to the chain data for peers. After complete, advance the epoch to the new intial peer set.
// TODO:
// Request signatures:
// 1) during dkg (should fail, but not cause problems)
// 2) after dkg but before advance (should succeed)
// 3) after advance (should succeed)

// const NUMSTAKED: usize = 10;
// async fn test_jwt_signing_auth_sig(nc: &NodeCollection) {
//     // prepare
//     let (
//         access_control_conditions,
//         evm_contract_conditions,
//         sol_rpc_conditions,
//         unified_access_control_conditions,
//         chain,
//     ) = prepare_test_jwt_signing_parameters();
//
//     // Get auth sig for auth
//     let auth_sig = get_auth_sig_for_auth();
//
//     // Hash the access control conditions
//     let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
//         access_control_conditions: access_control_conditions.clone(),
//         evm_contract_conditions: evm_contract_conditions.clone(),
//         sol_rpc_conditions: sol_rpc_conditions.clone(),
//         unified_access_control_conditions: unified_access_control_conditions.clone(),
//     })
//     .unwrap();
//
//     // Retrieve a signed JWT
//     let signing_resp = retrieve_signed_jwt(
//         nc,
//         access_control_conditions,
//         evm_contract_conditions,
//         sol_rpc_conditions,
//         unified_access_control_conditions,
//         chain.clone(),
//         &auth_sig,
//     )
//     .await;
//
//     // Assert that the JWT is valid
//     assert_signed_jwt(nc, hashed_access_control_conditions, chain, signing_resp);
// }

// async fn test_jwt_signing_session_sigs(nc: &NodeCollection) {
//     // prepare
//     let (
//         access_control_conditions,
//         evm_contract_conditions,
//         sol_rpc_conditions,
//         unified_access_control_conditions,
//         chain,
//     ) = prepare_test_jwt_signing_parameters();
//
//     // Hash the access control conditions
//     let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
//         access_control_conditions: access_control_conditions.clone(),
//         evm_contract_conditions: evm_contract_conditions.clone(),
//         sol_rpc_conditions: sol_rpc_conditions.clone(),
//         unified_access_control_conditions: unified_access_control_conditions.clone(),
//     })
//     .unwrap();
//
//     // Get session sig for auth
//     let session_sigs = get_session_sigs_for_auth(
//         vec![LitResourceAbilityRequest {
//             resource: LitResourceAbilityRequestResource {
//                 resource: hashed_access_control_conditions.clone(),
//                 resource_prefix: "lit-accesscontrolcondition".to_string(),
//             },
//             ability: LitAbility::AccessControlConditionSigning.to_string(),
//         }],
//         &nc.node_info_arr
//             .iter()
//             .map(|pi| pi.addr.to_owned())
//             .collect::<Vec<String>>(),
//     );
//
//     // Retrieve a signed JWT
//     let signing_resp = retrieve_signed_jwt_session_sigs(
//         nc,
//         access_control_conditions,
//         evm_contract_conditions,
//         sol_rpc_conditions,
//         unified_access_control_conditions,
//         chain.clone(),
//         &session_sigs,
//     )
//     .await;
//
//     // Assert that the JWT is valid
//     assert_signed_jwt(nc, hashed_access_control_conditions, chain, signing_resp);
// }

// fn prepare_test_jwt_signing_parameters() -> (
//     Option<Vec<AccessControlConditionItem>>,
//     Option<Vec<EVMContractConditionItem>>,
//     Option<Vec<SolRpcConditionItem>>,
//     Option<Vec<UnifiedAccessControlConditionItem>>,
//     Option<String>,
// ) {
//     let chain = Some(CHAIN_LOCALCHAIN.to_string());
//     let access_control_conditions = Some(vec![
//         lit_node::models::AccessControlConditionItem::Condition(
//             models::JsonAccessControlCondition {
//                 contract_address: "".to_string(),
//                 chain: CHAIN_LOCALCHAIN.to_string(),
//                 standard_contract_type: "".to_string(),
//                 method: "eth_getBalance".to_string(),
//                 parameters: vec![":userAddress".to_string(), "latest".to_string()],
//                 return_value_test: models::JsonReturnValueTest {
//                     comparator: ">=".to_string(),
//                     value: "0".to_string(),
//                 },
//             },
//         ),
//     ]);

//     (access_control_conditions, None, None, None, chain)
// }

// async fn retrieve_signed_jwt(
//     _nc: &NodeCollection,
//     scenario: &Scenario,
//     access_control_conditions: Option<Vec<AccessControlConditionItem>>,
//     evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
//     sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
//     unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
//     chain: Option<String>,
//     auth_sig: &JsonAuthSig,
// ) -> Vec<String> {
//     let now: u64 = SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .expect("System time is before unix epoch.  Your computer clock is probably wrong")
//         .as_secs();

//     let sign_access_control_conditions_request = SigningAccessControlConditionRequest {
//         access_control_conditions,
//         evm_contract_conditions,
//         sol_rpc_conditions,
//         unified_access_control_conditions,
//         chain,
//         auth_sig: AuthSigItem::Single(auth_sig.to_owned()),
//         iat: now,
//         exp: now + 600,
//     };

//     let signing_resp = hit_endpoints_with_json_body(
//         scenario,
//         "web/signing/access_control_condition".into(),
//         serde_json::to_string(&sign_access_control_conditions_request)
//             .unwrap()
//             .to_string(),
//     )
//     .await;
//     info!("signing resp: {:?}", signing_resp);

//     signing_resp
// }

// async fn retrieve_signed_jwt_session_sigs(
//     nc: &NodeCollection,
//     scenario: &Scenario,
//     access_control_conditions: Option<Vec<AccessControlConditionItem>>,
//     evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
//     sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
//     unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
//     chain: Option<String>,
//     session_sigs: &Vec<JsonAuthSig>,
// ) -> Vec<String> {
//     let mut json_body_vec = Vec::new();

//     let now: u64 = SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .expect("System time is before unix epoch.  Your computer clock is probably wrong")
//         .as_secs();

//     // Generate JSON body for each port
//     for (i, _port) in nc.portnames.iter().enumerate() {
//         let sign_access_control_conditions_request = SigningAccessControlConditionRequest {
//             access_control_conditions: access_control_conditions.clone(),
//             evm_contract_conditions: evm_contract_conditions.clone(),
//             sol_rpc_conditions: sol_rpc_conditions.clone(),
//             unified_access_control_conditions: unified_access_control_conditions.clone(),
//             chain: chain.clone(),
//             auth_sig: AuthSigItem::Single(session_sigs[i].to_owned()),
//             iat: now,
//             exp: now + 600,
//         };
//         let sign_access_control_conditions_request_payload =
//             serde_json::to_string(&sign_access_control_conditions_request).unwrap();
//         json_body_vec.push(sign_access_control_conditions_request_payload);
//     }

//     let signing_resp = hit_endpoints_with_json_body_per_port(
//         scenario,
//         "web/signing/access_control_condition".into(),
//         json_body_vec,
//     )
//     .await;
//     info!("signing resp: {:?}", signing_resp);

//     signing_resp
// }

// fn assert_signed_jwt(
//     _nc: &NodeCollection,
//     expected_hashed_access_control_conditions: String,
//     chain: Option<String>,
//     signing_resp: Vec<String>,
// ) {
//     assert_eq!(signing_resp.len(), NUMSTAKED);

//     let mut sig_shares: BTreeMap<usize, BlsfulSignatureShare<Bls12381G2Impl>> = BTreeMap::new();
//     let mut signed_message = "".to_string();
//     for resp in signing_resp {
//         let parsed_resp: SigningAccessControlConditionResponse =
//             serde_json::from_str(&resp).unwrap();
//         if signed_message.is_empty() {
//             signed_message = parsed_resp.unsigned_jwt;
//         }
//         let share_index = usize::try_from(parsed_resp.share_index).unwrap();
//         sig_shares.insert(share_index, parsed_resp.signature_share);
//     }

//     info!("shares: {:?}", sig_shares);

//     // TODO: Verify the network signature against the network public key.

//     // Verify the JWT claims match the original.
//     let jwt_claims_base64 = signed_message.split('.').nth(1).unwrap();
//     let payload_as_json = BASE64URL_NOPAD
//         .decode(jwt_claims_base64.as_bytes())
//         .unwrap();
//     let unsigned_jwt_claims = serde_json::from_slice::<JwtPayloadV2>(&payload_as_json).unwrap();

//     // Hash the access control conditions in the unsigned JWT.
//     let hashed_request_access_control_conditions =
//         hash_access_control_conditions(RequestConditions {
//             access_control_conditions: unsigned_jwt_claims.access_control_conditions,
//             evm_contract_conditions: unsigned_jwt_claims.evm_contract_conditions,
//             sol_rpc_conditions: unsigned_jwt_claims.sol_rpc_conditions,
//             unified_access_control_conditions: unsigned_jwt_claims
//                 .unified_access_control_conditions,
//         })
//         .unwrap();

//     assert_eq!(
//         hashed_request_access_control_conditions,
//         expected_hashed_access_control_conditions
//     );
//     assert_eq!(unsigned_jwt_claims.chain, chain);
// }

pub fn prepare_test_encryption_parameters() -> (
    String,
    String,
    Option<Vec<AccessControlConditionItem>>,
    Option<Vec<EVMContractConditionItem>>,
    Option<Vec<SolRpcConditionItem>>,
    Option<Vec<UnifiedAccessControlConditionItem>>,
    Option<String>,
) {
    let to_encrypt = "hello this is a test";

    // sha256 the plaintext
    let mut hasher = Sha256::new();
    hasher.update(to_encrypt.as_bytes());
    let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

    let chain = Some(CHAIN_LOCALCHAIN.to_string());
    let unified_access_control_conditions = Some(vec![
        lit_node::models::UnifiedAccessControlConditionItem::Condition(
            lit_node::models::UnifiedAccessControlCondition::JsonAccessControlCondition(
                models::JsonAccessControlCondition {
                    contract_address: "".to_string(),
                    chain: CHAIN_LOCALCHAIN.to_string(),
                    standard_contract_type: "".to_string(),
                    method: "eth_getBalance".to_string(),
                    parameters: vec![":userAddress".to_string(), "latest".to_string()],
                    return_value_test: models::JsonReturnValueTest {
                        comparator: ">=".to_string(),
                        value: "0".to_string(),
                    },
                },
            ),
        ),
    ]);

    (
        to_encrypt.into(),
        data_to_encrypt_hash,
        None,
        None,
        None,
        unified_access_control_conditions,
        chain,
    )
}

pub fn prepare_test_encryption_parameters_with_wallet_address(
    wallet_address: String,
) -> (
    String,
    String,
    Option<Vec<AccessControlConditionItem>>,
    Option<Vec<EVMContractConditionItem>>,
    Option<Vec<SolRpcConditionItem>>,
    Option<Vec<UnifiedAccessControlConditionItem>>,
    Option<String>,
) {
    let to_encrypt = "hello this is a test to decrypt with the provided wallet address";

    // sha256 the plaintext
    let mut hasher = Sha256::new();
    hasher.update(to_encrypt.as_bytes());
    let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

    let chain = Some(CHAIN_LOCALCHAIN.to_string());

    debug!(
        "Allow only the provided wallet_addres to decrypt- {:?}",
        wallet_address
    );

    let access_control_conditions = Some(vec![
        lit_node::models::AccessControlConditionItem::Condition(
            models::JsonAccessControlCondition {
                contract_address: "".to_string(),
                chain: CHAIN_LOCALCHAIN.to_string(),
                standard_contract_type: "".to_string(),
                method: "".to_string(),
                parameters: vec![":userAddress".to_string()],
                return_value_test: models::JsonReturnValueTest {
                    comparator: "=".to_string(),
                    value: wallet_address,
                },
            },
        ),
    ]);

    (
        to_encrypt.into(),
        data_to_encrypt_hash,
        access_control_conditions,
        None,
        None,
        None,
        chain,
    )
}

pub async fn test_encryption_decryption_auth_sig(actions: &Actions) {
    // prepare
    let (
        to_encrypt,
        data_to_encrypt_hash,
        access_control_conditions,
        evm_contract_conditions,
        sol_rpc_conditions,
        unified_access_control_conditions,
        chain,
    ) = prepare_test_encryption_parameters();

    // Get auth sig for auth
    let wallet = LocalWallet::new(&mut OsRng);
    let auth_sig = generate_authsig(&wallet)
        .await
        .expect("Couldn't generate auth sig");

    // Encrypt.
    let network_pubkey = get_network_pubkey(actions).await;
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
    let ciphertext = lit_bls_wasm::encrypt_time_lock::<Bls12381G2Impl>(
        &network_pubkey,
        message_bytes.to_vec(),
        identity_param.clone(),
    )
    .expect("Unable to encrypt");
    info!("ciphertext: {:?}", ciphertext);

    // Retrieve decrypted key
    let decryption_resp = retrieve_decryption_key(
        actions,
        access_control_conditions,
        evm_contract_conditions,
        sol_rpc_conditions,
        unified_access_control_conditions,
        chain,
        data_to_encrypt_hash,
        &auth_sig,
    )
    .await;

    // Assert decryption
    assert_decrypted(
        actions.get_current_validator_count().await,
        &network_pubkey,
        identity_param,
        &to_encrypt,
        &ciphertext,
        decryption_resp,
    );
}

async fn test_encryption_decryption_session_sigs(validator_collection: &ValidatorCollection) {
    // prepare
    let (
        to_encrypt,
        data_to_encrypt_hash,
        access_control_conditions,
        evm_contract_conditions,
        sol_rpc_conditions,
        unified_access_control_conditions,
        chain,
    ) = prepare_test_encryption_parameters();

    // Get the resource key
    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: access_control_conditions.clone(),
        evm_contract_conditions: evm_contract_conditions.clone(),
        sol_rpc_conditions: sol_rpc_conditions.clone(),
        unified_access_control_conditions: unified_access_control_conditions.clone(),
    })
    .unwrap();

    // Get session sig for auth
    let session_sigs = get_session_sigs_for_auth(
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: format!(
                    "{}/{}",
                    hashed_access_control_conditions, data_to_encrypt_hash
                ),
                resource_prefix: "lit-accesscontrolcondition".to_string(),
            },
            ability: LitAbility::AccessControlConditionDecryption.to_string(),
        }],
        &validator_collection.addresses(),
        None,
        None,
    );

    // Encrypt.
    let network_pubkey = get_network_pubkey(validator_collection.actions()).await;
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
    let ciphertext = lit_bls_wasm::encrypt_time_lock::<Bls12381G2Impl>(
        &network_pubkey,
        message_bytes.to_vec(),
        identity_param.clone(),
    )
    .expect("Unable to encrypt");
    info!("ciphertext: {:?}", ciphertext);

    // Retrieve decrypted key
    let decryption_resp = retrieve_decryption_key_session_sigs(
        &validator_collection,
        access_control_conditions,
        evm_contract_conditions,
        sol_rpc_conditions,
        unified_access_control_conditions,
        chain,
        data_to_encrypt_hash,
        &session_sigs,
    )
    .await;

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
}

pub async fn retrieve_decryption_key(
    actions: &Actions,
    access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
    chain: Option<String>,
    data_to_encrypt_hash: String,
    auth_sig: &JsonAuthSig,
) -> Vec<String> {
    let epoch = actions.get_current_epoch().await;
    let epoch = epoch.as_u64();

    let payload = serde_json::to_string(&EncryptionSignRequest {
        access_control_conditions: access_control_conditions.clone(),
        evm_contract_conditions: evm_contract_conditions.clone(),
        sol_rpc_conditions: sol_rpc_conditions.clone(),
        unified_access_control_conditions: unified_access_control_conditions.clone(),
        chain: chain.clone(),
        data_to_encrypt_hash: data_to_encrypt_hash.clone(),
        auth_sig: AuthSigItem::Single(auth_sig.to_owned()),
        epoch,
    })
    .expect("Could not convert encryption_sign_request to string");
    info!("Sending payload {:?}", payload);

    let get_encryption_key_resp = hit_endpoints_with_json_body(
        actions,
        "web/encryption/sign/v1".into(),
        payload.to_string(),
    )
    .await;
    info!("get_encryption_key_resp: {:?}", get_encryption_key_resp);

    get_encryption_key_resp
}

pub async fn retrieve_decryption_key_session_sigs(
    validator_collection: &ValidatorCollection,
    access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
    chain: Option<String>,
    data_to_encrypt_hash: String,
    session_sigs: &Vec<JsonAuthSig>,
) -> Vec<String> {
    let mut json_body_vec = Vec::new();

    // Generate JSON body for each port
    for i in 0..validator_collection.size() {
        let encryption_sign_request = models::EncryptionSignRequest {
            access_control_conditions: access_control_conditions.clone(),
            evm_contract_conditions: evm_contract_conditions.clone(),
            sol_rpc_conditions: sol_rpc_conditions.clone(),
            unified_access_control_conditions: unified_access_control_conditions.clone(),
            chain: chain.clone(),
            data_to_encrypt_hash: data_to_encrypt_hash.clone(),
            auth_sig: AuthSigItem::Single(session_sigs[i].to_owned()),
            epoch: 0,
        };
        let encryption_sign_request_payload =
            serde_json::to_string(&encryption_sign_request).unwrap();
        json_body_vec.push(encryption_sign_request_payload);
    }

    let get_encryption_key_resp = hit_endpoints_with_json_body_per_port(
        validator_collection.actions(),
        "web/encryption/sign".into(),
        json_body_vec,
    )
    .await;
    info!("get_encryption_key_resp: {:?}", get_encryption_key_resp);

    get_encryption_key_resp
}

pub fn assert_decrypted(
    num_staked: u32,
    network_pubkey: &str,
    identity_param: Vec<u8>,
    expected_plaintext: &str,
    ciphertext: &str,
    decryption_resp: Vec<String>,
) {
    assert_eq!(decryption_resp.len(), num_staked as usize);

    // Use decryption shares to decrypt ciphertext and check that it matches the original
    let serialized_decryption_shares: Vec<String> = decryption_resp
        .iter()
        .map(|resp| {
            let parsed_resp: EncryptionSignResponse = serde_json::from_str(resp).unwrap();
            serde_json::to_string(&parsed_resp.signature_share).unwrap()
        })
        .collect();
    let decrypted = lit_bls_wasm::verify_and_decrypt::<Bls12381G2Impl>(
        network_pubkey,
        &identity_param,
        &base64_decode(ciphertext),
        &serialized_decryption_shares,
    )
    .expect("Unable to decrypt");
    assert_eq!(
        base64_decode(&decrypted),
        *expected_plaintext.as_bytes(),
        "Decrypted does not match expected plaintext"
    );
}

async fn test_lit_action_session_sigs(
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
    curve_type: CurveType,
) {
    let wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
    let provider = validator_collection.actions().deployer_provider();
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
    fund_wallet(&provider, &wallet, "100000000000000000000").await;

    let pkp_info = mint_next_pkp_with_wallet(client.clone(), validator_collection.actions())
        .await
        .expect("Could not mint next pkp");
    let pubkey = pkp_info.0;
    let token_id = pkp_info.1;
    let pkp_eth_address = pkp_info.2;

    // println!("PKP ETH address: {}", hex::encode(pkp_eth_address.clone()));

    // add the PKP itself as a permitted address, so that our session sig from the PKP will be able to sign with it
    add_permitted_address_to_pkp_with_wallet(
        client.clone(),
        validator_collection.actions(),
        &hex::encode(pkp_eth_address),
        token_id,
        &[U256::from(1)],
    )
    .await
    .expect("Could not add permitted address to pkp");

    // Get session sig for auth
    let session_sigs = get_session_sigs_for_pkp(
        validator_collection.actions(),
        pubkey.clone(),
        pkp_eth_address,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: "lit-litaction".to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        &validator_collection.addresses(),
        wallet,
        None,
        curve_type,
        None,
        None,
        2,
    )
    .await
    .expect("Could not get session sigs");

    // run
    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        sign_lit_action(HELLO_WORLD_LIT_ACTION_CODE.to_string(), pubkey)
            .await
            .expect("Could not get lit action params");
    let execute_resp = execute_lit_action_session_sigs(
        validator_collection,
        lit_action_code,
        ipfs_id,
        js_params,
        auth_methods,
        &session_sigs,
        EndpointVersion::Initial,
        2,
    )
    .await
    .expect("Could not execute lit action");
    let action_result = assert_signed_action(validator_collection, execute_resp).await;

    assert!(action_result.is_ok());
    let action_result = action_result.unwrap();
    assert!(
        action_result == true,
        "The action should have returned true"
    );
}

#[tokio::test]
async fn test_everything_as_web_user() {
    test_common::init_test_config();
    // use initial_node_setup if you don't have a DKG result saved.
    let num_nodes = 6;

    let (testnet, validator_collection) = new_node_collection(num_nodes, false).await;

    // FIXME: Get this test working.
    // info!("Testing JWT signing with auth sigs");
    // test_jwt_signing_auth_sig(&nc).await;

    // FIXME: Get this test working.
    // info!("Testing JWT signing with session sigs");
    // test_jwt_signing_session_sigs(&nc).await;
    info!("Testing decryption with auth sigs");
    test_encryption_decryption_auth_sig(validator_collection.actions()).await;

    info!("Testing decryption with session sigs");
    test_encryption_decryption_session_sigs(&validator_collection).await;

    info!("Testing lit actions with ECDSA session sigs");
    test_lit_action_session_sigs(&testnet, &validator_collection, CurveType::K256).await;

    info!("Testing lit actions with BLS session sigs");
    test_lit_action_session_sigs(&testnet, &validator_collection, CurveType::BLS).await;
}

#[tokio::test]
async fn test_rate_limit_nft() {
    test_common::init_test_config();
    let num_staked_and_joined_validators = 3;
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(num_staked_and_joined_validators)
        .custom_node_runtime_config(
            CustomNodeRuntimeConfig::builder()
                .enable_rate_limiting("true".into())
                .enable_rate_limiting_allocation("true".into())
                .build(),
        )
        .force_deploy_in_ci(true)
        .build()
        .await;

    let testnet_contracts = Testnet::setup_contracts(
        &mut testnet,
        Some(
            StakingContractConfig::builder()
                .epoch_length(U256::from(300))
                .build(),
        ),
    )
    .await
    .expect("Failed to setup contracts");

    let actions = testnet.actions(testnet_contracts.contracts());

    let validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(num_staked_and_joined_validators)
        .build(&testnet, &actions)
        .await
        .expect("Failed to build validator collection");

    // prepare
    let (
        to_encrypt,
        data_to_encrypt_hash,
        access_control_conditions,
        evm_contract_conditions,
        sol_rpc_conditions,
        unified_access_control_conditions,
        _chain,
    ) = prepare_test_encryption_parameters();

    // encrypt something
    let network_pubkey = get_network_pubkey(validator_collection.actions()).await;
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
    let ciphertext = lit_bls_wasm::encrypt_time_lock::<Bls12381G2Impl>(
        &network_pubkey,
        message_bytes.to_vec(),
        identity_param.clone(),
    )
    .expect("Unable to encrypt");
    info!("ciphertext: {:?}", ciphertext);

    // Get auth sig for auth
    let wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
    let auth_sig = generate_authsig(&wallet)
        .await
        .expect("Couldn't generate auth sig");

    // send requests until we hit the rate limit
    let mut count = 0;
    loop {
        info!("Sending request {}", count);
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key(
            validator_collection.actions(),
            access_control_conditions.clone(),
            evm_contract_conditions.clone(),
            sol_rpc_conditions.clone(),
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &auth_sig,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count += 1;
    }
    info!("We made {} requests before we hit the rate limit", count);

    // how many are we suposed to be able to make?
    let free_requests_per_rate_limit_window = validator_collection
        .actions()
        .contracts()
        .rate_limit_nft
        .free_requests_per_rate_limit_window()
        .await
        .unwrap();
    assert!(
        U256::from(count) == free_requests_per_rate_limit_window,
        "We should be able to make exactly {} requests before hitting the rate limit",
        free_requests_per_rate_limit_window
    );

    // 1. show that the user can mint a rate limit nft and it will work
    info!("Testing that the user can mint a rate limit NFT");
    let provider = testnet.provider.clone();
    fund_wallet(&provider, &wallet, "100000000000000000000").await;

    // mint a rate limit increase nft
    let rate_limit_nft_contract_address = validator_collection
        .actions()
        .contracts()
        .rate_limit_nft
        .address();
    let rate_limit_increase = 1;
    let _nft_id = mint_rate_limit_nft(
        rate_limit_nft_contract_address,
        &provider,
        &wallet,
        rate_limit_increase,
    )
    .await;

    let resources = vec![
        LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::ACC.to_string(),
            },
            ability: LitAbility::AccessControlConditionDecryption.to_string(),
        },
        LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        },
    ];

    // show that we can't use an inner authsig from a session as a top level authsig
    let auth_sig_for_session_sig_that_should_fail =
        get_auth_sig_for_session_sig("meow".to_string(), Some(wallet.clone()), &resources);
    // Retrieve decrypted key
    let decryption_resp = retrieve_decryption_key(
        validator_collection.actions(),
        access_control_conditions.clone(),
        evm_contract_conditions.clone(),
        sol_rpc_conditions.clone(),
        unified_access_control_conditions.clone(),
        Some(CHAIN_LOCALCHAIN.to_string()),
        data_to_encrypt_hash.clone(),
        &auth_sig_for_session_sig_that_should_fail,
    )
    .await;
    // info!("Responses from using session key: {:?}", decryption_resp);
    assert!(
        decryption_resp[0].contains("NodeInvalidAuthSig"),
        "We should not be able to use the inner sig from a session sig as a top level auth sig"
    );

    // send requests until we hit the rate limit
    let mut count_after = 0;
    loop {
        info!(
            "Sending request after rate limit nft minted {}",
            count_after
        );
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key(
            validator_collection.actions(),
            access_control_conditions.clone(),
            evm_contract_conditions.clone(),
            sol_rpc_conditions.clone(),
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &auth_sig,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after > 0,
        "We bought a rate limit nft, it should give us more requests"
    );
    let rate_limit_window_secs = validator_collection
        .actions()
        .contracts()
        .rate_limit_nft
        .default_rate_limit_window_seconds()
        .await
        .unwrap();
    let total_extra_permitted_requests = (rate_limit_window_secs.as_u32() as f32 / 1000.0
        * rate_limit_increase as f32)
        .floor() as u32;
    assert!(count_after == total_extra_permitted_requests, "we bought {} more requests per second, we should have been able to make exactly {} more requests after minting the nft", rate_limit_increase as f32 / 1000.0, total_extra_permitted_requests);

    info!("User minting own rate limit nft worked!");

    // 2. show that someone else can mint a rate limit NFT and delegate it to the user
    info!("Testing that someone else can mint a rate limit NFT and delegate it to the user");
    let rate_limit_holder_wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);

    fund_wallet(
        &provider,
        &rate_limit_holder_wallet,
        "100000000000000000000",
    )
    .await;

    // mint a rate limit increase nft
    let nft_id = mint_rate_limit_nft(
        rate_limit_nft_contract_address,
        &provider,
        &rate_limit_holder_wallet,
        rate_limit_increase,
    )
    .await;

    // delegate the nft to the user
    // to do this, we create a signature from the holder of the NFT
    // which is rate_limit_holder_wallet
    // and then we stick that signature into the capabilities section of the SIWE
    let delegation_signature = get_auth_sig_with_rli_nft_resources(
        rate_limit_holder_wallet.clone(),
        nft_id.to_string(),
        bytes_to_hex(wallet.address()),
    );
    info!("Delegation signature: {:?}", delegation_signature);
    // show that we cannot use the inner delegation signature as a top level auth sig
    let decryption_resp = retrieve_decryption_key(
        validator_collection.actions(),
        access_control_conditions.clone(),
        evm_contract_conditions.clone(),
        sol_rpc_conditions.clone(),
        unified_access_control_conditions.clone(),
        Some(CHAIN_LOCALCHAIN.to_string()),
        data_to_encrypt_hash.clone(),
        &delegation_signature,
    )
    .await;
    assert!(
        decryption_resp[0].contains("NodeInvalidAuthSig"),
        "We should not be able to use a delegation sig as a top level auth sig"
    );

    let session_sigs = get_session_sigs_for_auth(
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: format!(
                    "{}/{}",
                    hashed_access_control_conditions, data_to_encrypt_hash
                ),
                resource_prefix: "lit-accesscontrolcondition".to_string(),
            },
            ability: LitAbility::AccessControlConditionDecryption.to_string(),
        }],
        &validator_collection.addresses(),
        Some(wallet.clone()),
        Some(vec![delegation_signature.clone()]),
    );

    // send requests until we hit the rate limit
    let mut count_after = 0;
    loop {
        info!(
            "Sending request after rate limit nft minted {}",
            count_after
        );

        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            access_control_conditions.clone(),
            evm_contract_conditions.clone(),
            sol_rpc_conditions.clone(),
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT and delegating it before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after > 0,
        "We bought a rate limit nft, it should give us more requests"
    );
    assert!(
        count_after == 10,
        "we delegates 10 uses but were able to make {} more requests",
        count_after
    );

    info!("User using delegated rate limit nft worked!");

    // 3. do the delegation using the PaymentDelegation contract
    // let's start with a new wallet
    let payment_delegation_rate_limit_holder_wallet =
        LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
    fund_wallet(
        &provider,
        &payment_delegation_rate_limit_holder_wallet,
        "100000000000000000000",
    )
    .await;

    // mint another rate limit increase nft
    let _nft_id = mint_rate_limit_nft(
        rate_limit_nft_contract_address,
        &provider,
        &payment_delegation_rate_limit_holder_wallet,
        10,
    )
    .await;

    // delegate using the payment delegation contract
    let payment_delegation_contract_address = validator_collection
        .actions()
        .contracts()
        .payment_delegation
        .address();

    create_payment_delegation_entry(
        payment_delegation_contract_address,
        &provider,
        &payment_delegation_rate_limit_holder_wallet,
        wallet.address(),
        3,
        10,
    )
    .await;

    // make some requests and count how many we can make
    let mut count_after = 0;
    loop {
        info!(
            "Sending request after payment delegation entry {}",
            count_after
        );
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key(
            validator_collection.actions(),
            access_control_conditions.clone(),
            evm_contract_conditions.clone(),
            sol_rpc_conditions.clone(),
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &auth_sig,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT and delegating using the PaymentDelegation contract before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after >= 3,
        "We delegated a rate limit nft using the PaymentDelegation contract, it should give us more 10 requests but it gave us {}", count_after
    );

    // we have a 10 second period, so, after 10 seconds we should be able to make 10 more requests.  let's try it
    tokio::time::sleep(tokio::time::Duration::from_secs(11)).await;
    // make some requests and count how many we can make
    let mut count_after = 0;
    loop {
        info!(
            "Sending request after payment delegation entry {}",
            count_after
        );
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key(
            validator_collection.actions(),
            access_control_conditions.clone(),
            evm_contract_conditions.clone(),
            sol_rpc_conditions.clone(),
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &auth_sig,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT and delegating using the PaymentDelegation contract before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after >= 3,
        "We delegated a rate limit nft using the PaymentDelegation contract, it should give us more 10 requests, but it gave us {}", count_after
    );
}

#[tokio::test]
async fn test_rate_limit_nft_with_pkp() {
    test_common::init_test_config();
    let num_staked_and_joined_validators = 3;
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(num_staked_and_joined_validators)
        .custom_node_runtime_config(
            CustomNodeRuntimeConfig::builder()
                .enable_rate_limiting("true".into())
                .enable_rate_limiting_allocation("true".into())
                .build(),
        )
        .force_deploy_in_ci(true)
        .build()
        .await;

    let testnet_contracts = Testnet::setup_contracts(
        &mut testnet,
        Some(
            StakingContractConfig::builder()
                .epoch_length(U256::from(300))
                .build(),
        ),
    )
    .await
    .expect("Failed to setup contracts");

    let actions = testnet.actions(testnet_contracts.contracts());

    let validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(num_staked_and_joined_validators)
        .build(&testnet, &actions)
        .await
        .expect("Failed to build validator collection");

    let wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
    let provider = validator_collection.actions().deployer_provider();
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
    fund_wallet(&provider, &wallet, "100000000000000000000").await;

    let pkp_info = mint_next_pkp_with_wallet(client.clone(), validator_collection.actions())
        .await
        .expect("Could not mint next pkp");
    let pubkey = pkp_info.0;
    let token_id = pkp_info.1;
    let pkp_eth_address = pkp_info.2;

    // add the PKP itself as a permitted address, so that our session sig from the PKP will be able to sign with it
    add_permitted_address_to_pkp_with_wallet(
        client.clone(),
        validator_collection.actions(),
        &hex::encode(pkp_eth_address),
        token_id,
        &[U256::from(1)],
    )
    .await
    .expect("Could not add permitted address to pkp");

    // Get session sig for auth
    let session_sigs = get_session_sigs_for_pkp(
        validator_collection.actions(),
        pubkey.clone(),
        pkp_eth_address,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: "lit-litaction".to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        &validator_collection.addresses(),
        wallet.clone(),
        None,
        CurveType::BLS,
        None,
        None,
        2,
    )
    .await
    .expect("Could not get session sigs");

    let to_encrypt = "hello this is a test";

    // sha256 the plaintext
    let mut hasher = Sha256::new();
    hasher.update(to_encrypt.as_bytes());
    let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

    let chain = Some(CHAIN_LOCALCHAIN.to_string());
    let unified_access_control_conditions = Some(vec![
        lit_node::models::UnifiedAccessControlConditionItem::Condition(
            lit_node::models::UnifiedAccessControlCondition::JsonAccessControlCondition(
                models::JsonAccessControlCondition {
                    contract_address: "".to_string(),
                    chain: CHAIN_LOCALCHAIN.to_string(),
                    standard_contract_type: "".to_string(),
                    method: "".to_string(),
                    parameters: vec![":userAddress".to_string()],
                    return_value_test: models::JsonReturnValueTest {
                        comparator: "=".to_string(),
                        value: format!("0x{}", hex::encode(pkp_eth_address)),
                    },
                },
            ),
        ),
    ]);

    // encrypt something
    let network_pubkey = get_network_pubkey(validator_collection.actions()).await;
    let message_bytes = to_encrypt.as_bytes();
    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: None,
        evm_contract_conditions: None,
        sol_rpc_conditions: None,
        unified_access_control_conditions: unified_access_control_conditions.clone(),
    })
    .unwrap();
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

    // send requests until we hit the rate limit
    let mut count = 0;
    loop {
        info!("Sending request {}", count);
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count += 1;
    }
    info!("We made {} requests before we hit the rate limit", count);

    // how many are we suposed to be able to make?
    let free_requests_per_rate_limit_window = validator_collection
        .actions()
        .contracts()
        .rate_limit_nft
        .free_requests_per_rate_limit_window()
        .await
        .unwrap();
    assert!(
        U256::from(count) == free_requests_per_rate_limit_window,
        "We should be able to make exactly {} requests before hitting the rate limit",
        free_requests_per_rate_limit_window
    );

    // 1. show that the user can mint a rate limit nft and it will work
    info!("Testing that the user can mint a rate limit NFT");
    let provider = testnet.provider.clone();
    fund_wallet(&provider, &wallet, "100000000000000000000").await;

    // mint a rate limit increase nft
    let rate_limit_nft_contract_address = validator_collection
        .actions()
        .contracts()
        .rate_limit_nft
        .address();
    let rate_limit_increase = 1;
    let nft_id = mint_rate_limit_nft(
        rate_limit_nft_contract_address,
        &provider,
        &wallet,
        rate_limit_increase,
    )
    .await;
    // transfer rate limit NFT to the PKP
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let rate_limit_nft_contract =
        RateLimitNFT::new(rate_limit_nft_contract_address, Arc::new(client));
    let contract_call =
        rate_limit_nft_contract.transfer_from(wallet.address(), pkp_eth_address, nft_id);
    let transfer_tx = contract_call
        .send()
        .await
        .expect("Could not transfer rate limit NFT to PKP");
    let receipt = transfer_tx.await.expect("Could not get transfer receipt");

    let resources = vec![
        LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::ACC.to_string(),
            },
            ability: LitAbility::AccessControlConditionDecryption.to_string(),
        },
        LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        },
    ];

    // send requests until we hit the rate limit
    let mut count_after = 0;
    loop {
        info!(
            "Sending request after rate limit nft minted {}",
            count_after
        );
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after > 0,
        "We bought a rate limit nft, it should give us more requests"
    );
    let rate_limit_window_secs = validator_collection
        .actions()
        .contracts()
        .rate_limit_nft
        .default_rate_limit_window_seconds()
        .await
        .unwrap();
    let total_extra_permitted_requests = (rate_limit_window_secs.as_u32() as f32 / 1000.0
        * rate_limit_increase as f32)
        .floor() as u32;
    assert!(count_after == total_extra_permitted_requests, "we bought {} more requests per second, we should have been able to make exactly {} more requests after minting the nft", rate_limit_increase as f32 / 1000.0, total_extra_permitted_requests);

    info!("User minting own rate limit nft worked!");

    // 2. show that someone else can mint a rate limit NFT and delegate it to the user
    info!("Testing that someone else can mint a rate limit NFT and delegate it to the user");
    let rate_limit_holder_wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);

    fund_wallet(
        &provider,
        &rate_limit_holder_wallet,
        "100000000000000000000",
    )
    .await;

    // mint a rate limit increase nft
    let nft_id = mint_rate_limit_nft(
        rate_limit_nft_contract_address,
        &provider,
        &rate_limit_holder_wallet,
        rate_limit_increase,
    )
    .await;

    // delegate the nft to the user
    // to do this, we create a signature from the holder of the NFT
    // which is rate_limit_holder_wallet
    // and then we stick that signature into the capabilities section of the SIWE
    let delegation_signature = get_auth_sig_with_rli_nft_resources(
        rate_limit_holder_wallet.clone(),
        nft_id.to_string(),
        bytes_to_hex(pkp_eth_address),
    );
    info!("Delegation signature: {:?}", delegation_signature);
    // show that we cannot use the inner delegation signature as a top level auth sig
    let decryption_resp = retrieve_decryption_key(
        validator_collection.actions(),
        None,
        None,
        None,
        unified_access_control_conditions.clone(),
        Some(CHAIN_LOCALCHAIN.to_string()),
        data_to_encrypt_hash.clone(),
        &delegation_signature,
    )
    .await;
    assert!(
        decryption_resp[0].contains("NodeInvalidAuthSig"),
        "We should not be able to use a delegation sig as a top level auth sig"
    );
    let session_sigs = get_session_sigs_for_pkp(
        validator_collection.actions(),
        pubkey.clone(),
        pkp_eth_address,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: format!(
                    "{}/{}",
                    hashed_access_control_conditions, data_to_encrypt_hash
                ),
                resource_prefix: "lit-accesscontrolcondition".to_string(),
            },
            ability: LitAbility::AccessControlConditionDecryption.to_string(),
        }],
        &validator_collection.addresses(),
        wallet.clone(),
        Some(vec![delegation_signature.clone()]),
        CurveType::BLS,
        None,
        None,
        2,
    )
    .await
    .expect("Could not get session sigs");

    // send requests until we hit the rate limit
    let mut count_after = 0;
    loop {
        info!(
            "Sending request after rate limit nft minted {}",
            count_after
        );

        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT and delegating it before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after > 0,
        "We bought a rate limit nft, it should give us more requests"
    );
    assert!(
        count_after == 10,
        "we delegates 10 uses but were able to make {} more requests",
        count_after
    );

    info!("User using delegated rate limit nft worked!");

    // 3. do the delegation using the PaymentDelegation contract
    // let's start with a new wallet
    let payment_delegation_rate_limit_holder_wallet =
        LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
    fund_wallet(
        &provider,
        &payment_delegation_rate_limit_holder_wallet,
        "100000000000000000000",
    )
    .await;

    // mint another rate limit increase nft
    let _nft_id = mint_rate_limit_nft(
        rate_limit_nft_contract_address,
        &provider,
        &payment_delegation_rate_limit_holder_wallet,
        10,
    )
    .await;

    // delegate using the payment delegation contract
    let payment_delegation_contract_address = validator_collection
        .actions()
        .contracts()
        .payment_delegation
        .address();

    create_payment_delegation_entry(
        payment_delegation_contract_address,
        &provider,
        &payment_delegation_rate_limit_holder_wallet,
        pkp_eth_address,
        0,
        0,
    )
    .await;

    // try and make more requests - we should be able to make unlimited requests now so let's try 10
    // make some requests and count how many we can make
    let mut count_after = 0;
    loop {
        info!(
            "Sending request after payment delegation entry {}",
            count_after
        );
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || count_after > 10
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT and delegating using the PaymentDelegation contract with 0 restrictions before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after >= 10,
        "We delegated a rate limit nft using the PaymentDelegation contract, it should give us at least 10 more requests, but it gave us {}", count_after
    );

    // test with restrictions set to 0
    set_restrictions(
        payment_delegation_contract_address,
        &provider,
        &payment_delegation_rate_limit_holder_wallet,
        3,
        10,
    )
    .await;

    // make some requests and count how many we can make
    let mut count_after = 0;
    loop {
        info!(
            "Sending request after payment delegation entry {}",
            count_after
        );
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT and delegating using the PaymentDelegation contract before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after >= 3,
        "We delegated a rate limit nft using the PaymentDelegation contract, it should give us more 10 requests but it gave us {}", count_after
    );

    // we have a 10 second period, so, after 10 seconds we should be able to make 10 more requests.  let's try it
    tokio::time::sleep(tokio::time::Duration::from_secs(11)).await;
    // make some requests and count how many we can make
    let mut count_after = 0;
    loop {
        info!(
            "Sending request after payment delegation entry {}",
            count_after
        );
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT and delegating using the PaymentDelegation contract before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after >= 3,
        "We delegated a rate limit nft using the PaymentDelegation contract, it should give us more 3 requests, but it gave us {}", count_after
    );
}

#[tokio::test]
async fn test_rate_limit_nft_with_pkp_and_no_allocation() {
    test_common::init_test_config();
    let num_staked_and_joined_validators = 3;
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(num_staked_and_joined_validators)
        .custom_node_runtime_config(
            CustomNodeRuntimeConfig::builder()
                .enable_rate_limiting("true".into())
                .enable_rate_limiting_allocation("false".into())
                .build(),
        )
        .force_deploy_in_ci(true)
        .build()
        .await;

    let testnet_contracts = Testnet::setup_contracts(
        &mut testnet,
        Some(
            StakingContractConfig::builder()
                .epoch_length(U256::from(300))
                .build(),
        ),
    )
    .await
    .expect("Failed to setup contracts");

    let actions = testnet.actions(testnet_contracts.contracts());

    let validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(num_staked_and_joined_validators)
        .build(&testnet, &actions)
        .await
        .expect("Failed to build validator collection");

    let wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
    let provider = validator_collection.actions().deployer_provider();
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
    fund_wallet(&provider, &wallet, "100000000000000000000").await;

    let pkp_info = mint_next_pkp_with_wallet(client.clone(), validator_collection.actions())
        .await
        .expect("Could not mint next pkp");
    let pubkey = pkp_info.0;
    let token_id = pkp_info.1;
    let pkp_eth_address = pkp_info.2;

    // add the PKP itself as a permitted address, so that our session sig from the PKP will be able to sign with it
    add_permitted_address_to_pkp_with_wallet(
        client.clone(),
        validator_collection.actions(),
        &hex::encode(pkp_eth_address),
        token_id,
        &[U256::from(1)],
    )
    .await
    .expect("Could not add permitted address to pkp");

    // Get session sig for auth
    let session_sigs = get_session_sigs_for_pkp(
        validator_collection.actions(),
        pubkey.clone(),
        pkp_eth_address,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: "lit-litaction".to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        &validator_collection.addresses(),
        wallet.clone(),
        None,
        CurveType::BLS,
        None,
        None,
        2,
    )
    .await
    .expect("Could not get session sigs");

    let to_encrypt = "hello this is a test";

    // sha256 the plaintext
    let mut hasher = Sha256::new();
    hasher.update(to_encrypt.as_bytes());
    let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

    let chain = Some(CHAIN_LOCALCHAIN.to_string());
    let unified_access_control_conditions = Some(vec![
        lit_node::models::UnifiedAccessControlConditionItem::Condition(
            lit_node::models::UnifiedAccessControlCondition::JsonAccessControlCondition(
                models::JsonAccessControlCondition {
                    contract_address: "".to_string(),
                    chain: CHAIN_LOCALCHAIN.to_string(),
                    standard_contract_type: "".to_string(),
                    method: "".to_string(),
                    parameters: vec![":userAddress".to_string()],
                    return_value_test: models::JsonReturnValueTest {
                        comparator: "=".to_string(),
                        value: format!("0x{}", hex::encode(pkp_eth_address)),
                    },
                },
            ),
        ),
    ]);

    // encrypt something
    let network_pubkey = get_network_pubkey(validator_collection.actions()).await;
    let message_bytes = to_encrypt.as_bytes();
    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: None,
        evm_contract_conditions: None,
        sol_rpc_conditions: None,
        unified_access_control_conditions: unified_access_control_conditions.clone(),
    })
    .unwrap();
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

    // send requests until we hit the rate limit
    let mut count = 0;
    loop {
        info!("Sending request {}", count);
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count += 1;
    }
    info!("We made {} requests before we hit the rate limit", count);

    // how many are we suposed to be able to make?
    let free_requests_per_rate_limit_window = validator_collection
        .actions()
        .contracts()
        .rate_limit_nft
        .free_requests_per_rate_limit_window()
        .await
        .unwrap();
    assert!(
        U256::from(count) == free_requests_per_rate_limit_window,
        "We should be able to make exactly {} requests before hitting the rate limit",
        free_requests_per_rate_limit_window
    );

    // 1. show that the user can mint a rate limit nft and it will work
    info!("Testing that the user can mint a rate limit NFT");
    let provider = testnet.provider.clone();
    fund_wallet(&provider, &wallet, "100000000000000000000").await;

    // mint a rate limit increase nft
    let rate_limit_nft_contract_address = validator_collection
        .actions()
        .contracts()
        .rate_limit_nft
        .address();
    let rate_limit_increase = 1;
    let nft_id = mint_rate_limit_nft(
        rate_limit_nft_contract_address,
        &provider,
        &wallet,
        rate_limit_increase,
    )
    .await;
    // transfer rate limit NFT to the PKP
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let rate_limit_nft_contract =
        RateLimitNFT::new(rate_limit_nft_contract_address, Arc::new(client));
    let contract_call =
        rate_limit_nft_contract.transfer_from(wallet.address(), pkp_eth_address, nft_id);
    let transfer_tx = contract_call
        .send()
        .await
        .expect("Could not transfer rate limit NFT to PKP");
    let receipt = transfer_tx.await.expect("Could not get transfer receipt");

    let resources = vec![
        LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::ACC.to_string(),
            },
            ability: LitAbility::AccessControlConditionDecryption.to_string(),
        },
        LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        },
    ];

    // send 10 more requests
    let mut count_after = 0;
    for _ in 0..10 {
        info!(
            "Sending request after rate limit nft minted {}",
            count_after
        );
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after == 10,
        "We bought a rate limit nft, it should give us at least 10 more requests, but it gave us {}", count_after
    );

    info!("User minting own rate limit nft worked!");

    // 2. show that someone else can mint a rate limit NFT and delegate it to the user
    info!("Testing that someone else can mint a rate limit NFT and delegate it to the user");
    let rate_limit_holder_wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);

    fund_wallet(
        &provider,
        &rate_limit_holder_wallet,
        "100000000000000000000",
    )
    .await;

    // mint a rate limit increase nft
    let nft_id = mint_rate_limit_nft(
        rate_limit_nft_contract_address,
        &provider,
        &rate_limit_holder_wallet,
        rate_limit_increase,
    )
    .await;

    // delegate the nft to the user
    // to do this, we create a signature from the holder of the NFT
    // which is rate_limit_holder_wallet
    // and then we stick that signature into the capabilities section of the SIWE
    let delegation_signature = get_auth_sig_with_rli_nft_resources(
        rate_limit_holder_wallet.clone(),
        nft_id.to_string(),
        bytes_to_hex(pkp_eth_address),
    );
    info!("Delegation signature: {:?}", delegation_signature);
    // show that we cannot use the inner delegation signature as a top level auth sig
    let decryption_resp = retrieve_decryption_key(
        validator_collection.actions(),
        None,
        None,
        None,
        unified_access_control_conditions.clone(),
        Some(CHAIN_LOCALCHAIN.to_string()),
        data_to_encrypt_hash.clone(),
        &delegation_signature,
    )
    .await;
    assert!(
        decryption_resp[0].contains("NodeInvalidAuthSig"),
        "We should not be able to use a delegation sig as a top level auth sig"
    );
    let session_sigs = get_session_sigs_for_pkp(
        validator_collection.actions(),
        pubkey.clone(),
        pkp_eth_address,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: format!(
                    "{}/{}",
                    hashed_access_control_conditions, data_to_encrypt_hash
                ),
                resource_prefix: "lit-accesscontrolcondition".to_string(),
            },
            ability: LitAbility::AccessControlConditionDecryption.to_string(),
        }],
        &validator_collection.addresses(),
        wallet.clone(),
        Some(vec![delegation_signature.clone()]),
        CurveType::BLS,
        None,
        None,
        2,
    )
    .await
    .expect("Could not get session sigs");

    // send requests until we hit the rate limit
    let mut count_after = 0;
    for _ in 0..10 {
        info!(
            "Sending request after rate limit nft minted {}",
            count_after
        );

        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT and delegating it before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after == 10,
        "We bought a rate limit nft, it should give us at least 10 more requests, but it gave us {}", count_after
    );

    info!("User using delegated rate limit nft worked!");

    // 3. do the delegation using the PaymentDelegation contract
    // let's start with a new wallet
    let payment_delegation_rate_limit_holder_wallet =
        LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
    fund_wallet(
        &provider,
        &payment_delegation_rate_limit_holder_wallet,
        "100000000000000000000",
    )
    .await;

    // mint another rate limit increase nft
    let _nft_id = mint_rate_limit_nft(
        rate_limit_nft_contract_address,
        &provider,
        &payment_delegation_rate_limit_holder_wallet,
        10,
    )
    .await;

    // delegate using the payment delegation contract
    let payment_delegation_contract_address = validator_collection
        .actions()
        .contracts()
        .payment_delegation
        .address();

    create_payment_delegation_entry(
        payment_delegation_contract_address,
        &provider,
        &payment_delegation_rate_limit_holder_wallet,
        pkp_eth_address,
        0,
        0,
    )
    .await;

    // try and make more requests - we should be able to make unlimited requests now so let's try 10
    // make some requests and count how many we can make
    let mut count_after = 0;
    for _ in 0..10 {
        info!(
            "Sending request after payment delegation entry {}",
            count_after
        );
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || count_after > 10
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT and delegating using the PaymentDelegation contract with 0 restrictions before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after == 10,
        "We delegated a rate limit nft using the PaymentDelegation contract, it should give us at least 10 more requests, but it gave us {}", count_after
    );

    // test with restrictions set to 0
    set_restrictions(
        payment_delegation_contract_address,
        &provider,
        &payment_delegation_rate_limit_holder_wallet,
        3,
        10,
    )
    .await;

    // make some requests and count how many we can make
    let mut count_after = 0;
    for _ in 0..10 {
        info!(
            "Sending request after payment delegation entry {}",
            count_after
        );
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT and delegating using the PaymentDelegation contract before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after == 10,
        "We delegated a rate limit nft using the PaymentDelegation contract, it should give us more 10 requests but it gave us {}", count_after
    );

    // we have a 10 second period, so, after 10 seconds we should be able to make 10 more requests.  let's try it
    tokio::time::sleep(tokio::time::Duration::from_secs(11)).await;
    // make some requests and count how many we can make
    let mut count_after = 0;
    for _ in 0..10 {
        info!(
            "Sending request after payment delegation entry {}",
            count_after
        );
        // Retrieve decrypted key
        let decryption_resp = retrieve_decryption_key_session_sigs(
            &validator_collection,
            None,
            None,
            None,
            unified_access_control_conditions.clone(),
            Some(CHAIN_LOCALCHAIN.to_string()),
            data_to_encrypt_hash.clone(),
            &session_sigs,
        )
        .await;
        info!("Responses: {:?}", decryption_resp);
        if decryption_resp[0].contains("Rate limit exceeded.")
            || decryption_resp[0].contains("error")
        {
            break;
        }

        count_after += 1;
    }
    info!(
        "We made {} requests after minting the NFT and delegating using the PaymentDelegation contract before we hit the rate limit again",
        count_after
    );
    assert!(
        count_after == 10,
        "We delegated a rate limit nft using the PaymentDelegation contract, it should give us at least 10 requests, but it gave us {}", count_after
    );
}
