use crate::auth_sig::get_session_sigs_for_pkp;
use crate::lit_actions::assert_signed_action;
use crate::lit_actions::execute_lit_action_session_sigs;
use crate::lit_actions::sign_lit_action;
use crate::lit_actions::HELLO_WORLD_LIT_ACTION_CODE;
use crate::node_collection::get_network_pubkey;
use crate::node_collection::hit_endpoints_with_json_body;
use crate::node_collection::hit_endpoints_with_json_body_per_port;
use crate::pkp::add_permitted_address_to_pkp_with_wallet;
use crate::pkp::mint_next_pkp_with_wallet;
use crate::testnet::actions::Actions;
use crate::testnet::rate_limit_nfts::fund_wallet;
use crate::testnet::Testnet;
use crate::validator::ValidatorCollection;
use std::sync::Arc;

use crate::auth_sig::generate_authsig;
use base64_light::base64_decode;
use blsful::Bls12381G2Impl;
use ethers::middleware::SignerMiddleware;
use ethers::signers::LocalWallet;
use ethers::signers::Signer;
use ethers::types::U256;
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
use lit_node::models::AccessControlConditionItem;
use lit_node::models::EVMContractConditionItem;
use lit_node::models::EncryptionSignRequest;
use lit_node::models::EncryptionSignResponse;
use lit_node::models::RequestConditions;
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

// async fn test_encryption_decryption_session_sigs(
//     validator_collection: &ValidatorCollection,
//     endpoint_version: EndpointVersion,
// ) {
//     // prepare
//     let (
//         to_encrypt,
//         data_to_encrypt_hash,
//         access_control_conditions,
//         evm_contract_conditions,
//         sol_rpc_conditions,
//         unified_access_control_conditions,
//         chain,
//     ) = prepare_test_encryption_parameters();

//     // Get the resource key
//     let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
//         access_control_conditions: access_control_conditions.clone(),
//         evm_contract_conditions: evm_contract_conditions.clone(),
//         sol_rpc_conditions: sol_rpc_conditions.clone(),
//         unified_access_control_conditions: unified_access_control_conditions.clone(),
//     })
//     .unwrap();

//     // Get session sig for auth
//     let session_sigs = get_session_sigs_for_auth(
//         vec![LitResourceAbilityRequest {
//             resource: LitResourceAbilityRequestResource {
//                 resource: format!(
//                     "{}/{}",
//                     hashed_access_control_conditions, data_to_encrypt_hash
//                 ),
//                 resource_prefix: "lit-accesscontrolcondition".to_string(),
//             },
//             ability: LitAbility::AccessControlConditionDecryption.to_string(),
//         }],
//         &validator_collection.addresses(),
//         None,
//         None,
//         Some(U256::MAX), // max_price
//     );

//     // Encrypt.
//     let network_pubkey = get_network_pubkey(validator_collection.actions()).await;
//     let message_bytes = to_encrypt.as_bytes();
//     let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
//         access_control_conditions: access_control_conditions.clone(),
//         evm_contract_conditions: evm_contract_conditions.clone(),
//         sol_rpc_conditions: sol_rpc_conditions.clone(),
//         unified_access_control_conditions: unified_access_control_conditions.clone(),
//     })
//     .unwrap();
//     let identity_param = AccessControlConditionResource::new(format!(
//         "{}/{}",
//         hashed_access_control_conditions, data_to_encrypt_hash
//     ))
//     .get_resource_key()
//     .into_bytes();
//     let ciphertext = lit_bls_wasm::encrypt_time_lock::<Bls12381G2Impl>(
//         &network_pubkey,
//         message_bytes.to_vec(),
//         identity_param.clone(),
//     )
//     .expect("Unable to encrypt");
//     info!("ciphertext: {:?}", ciphertext);

//     // Retrieve decrypted key
//     let decryption_resp = retrieve_decryption_key_session_sigs_with_version(
//         &validator_collection,
//         access_control_conditions,
//         evm_contract_conditions,
//         sol_rpc_conditions,
//         unified_access_control_conditions,
//         chain,
//         data_to_encrypt_hash,
//         &session_sigs,
//         endpoint_version,
//     )
//     .await;

//     assert_decrypted(
//         validator_collection
//             .actions()
//             .get_current_validator_count()
//             .await,
//         &network_pubkey,
//         identity_param,
//         &to_encrypt,
//         &ciphertext,
//         decryption_resp,
//     );
// }

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

    let get_encryption_key_resp =
        hit_endpoints_with_json_body(actions, "web/encryption/sign".into(), payload.to_string())
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
    retrieve_decryption_key_session_sigs_with_version(
        validator_collection,
        access_control_conditions,
        evm_contract_conditions,
        sol_rpc_conditions,
        unified_access_control_conditions,
        chain,
        data_to_encrypt_hash,
        session_sigs,
        EndpointVersion::Initial,
    )
    .await
}

pub async fn retrieve_decryption_key_session_sigs_with_version(
    validator_collection: &ValidatorCollection,
    access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
    chain: Option<String>,
    data_to_encrypt_hash: String,
    session_sigs: &Vec<JsonAuthSig>,
    endpoint_version: EndpointVersion,
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
        format!("web/encryption/sign{}", endpoint_version.as_str()),
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
    endpoint_version: EndpointVersion,
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
        endpoint_version,
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
