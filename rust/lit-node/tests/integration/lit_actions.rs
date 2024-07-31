/*
    const IPFS_CID: &str = "QmZuSixiCCkttPDvHKZdAw2gNby111rNEq1aHTozkqztSg";
    The source of this CID is :

    const go = async () => {
        // this is the string "Hello World" for testing
        const toSign = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100];
        // this requests a signature share from the Lit Node
        // the signature share will be automatically returned in the HTTP response from the node
        const sigShare = await LitActions.signEcdsa({
            toSign,
            publicKey, // <-- You should pass this in jsParam
            sigName: "sig1 Testttt",
        });
    };

    go();
*/
/*
     (async () => {
            const resp = await Lit.Actions.broadcastAndCollect({name: "some-name", value: "some-value"})
            Lit.Actions.setResponse({response: JSON.stringify(resp)})
        })()
*/

#[cfg(feature = "lit-actions")]
pub mod litactions {
    use std::collections::HashMap;

    use crate::acceptance::web_user_tests::prepare_test_encryption_parameters;
    use crate::common;
    use crate::common::auth_sig::{generate_authsig, generate_authsig_item};
    use crate::common::lit_actions::{
        assert_signed_action, execute_lit_action, prepare_sign_from_file_parameters,
        sign_lit_action,
    };
    use crate::common::node_collection::get_network_pubkey;
    use crate::common::pkp::mint_next_pkp;
    use crate::common::testnet::actions::Actions;
    use blsful::Bls12381G2Impl;
    use common::lit_actions::{
        sign_from_file_system, sign_hello_world, sign_using_child_lit_action,
    };
    use common::new_node_collection;
    use ethers::signers::LocalWallet;
    use ethers::utils::keccak256;
    use ipfs_hasher::IpfsHasher;
    use lit_core::utils::binary::bytes_to_hex;
    use lit_node::auth::auth_material::JsonAuthSig;
    use lit_node::auth::lit_resource::LitResource;
    use lit_node::auth::resources::AccessControlConditionResource;
    use lit_node::constants::CHAIN_LOCALCHAIN;
    use lit_node::models::{
        ControlConditionItem, EVMContractCondition, JsonAccessControlCondition,
        JsonReturnValueTest, JsonReturnValueTestV2, RequestConditions,
        UnifiedAccessControlCondition, UnifiedAccessControlConditionItem,
    };
    use lit_node::utils::web::hash_access_control_conditions;
    use lit_node::utils::web::EndpointVersion;
    use rand_core::OsRng;
    use regex::Regex;
    use serde_json::Value;
    use sha2::{Digest, Sha256};
    use test_case::test_case;
    use tracing::info;

    #[tokio::test]
    pub async fn lit_action_call_child_action() {
        common::init_test_config();
        info!("Starting test: lit_action_call_child_action");
        let num_nodes = 3;
        let (_testnet, validator_collection) = new_node_collection(num_nodes, false).await;

        let validation = sign_using_child_lit_action(&validator_collection)
            .await
            .unwrap();
        assert!(validation);
    }

    #[tokio::test]
    pub async fn lit_action_happy_path() {
        common::init_test_config();
        info!("Starting test: lit_action_happy_path");
        let num_nodes = 3;
        let (testnet, validator_collection) = new_node_collection(num_nodes, false).await;

        let validation = sign_hello_world(&testnet, &validator_collection)
            .await
            .unwrap();
        assert!(validation);
    }

    #[test_case("simple_broadcast_and_collect", false)]
    #[test_case("simple_get_rpc_url", false)]
    #[test_case("simple_run_once", false)]
    #[test_case("simple_sign_and_combine_ecdsa", false)]
    #[test_case("simple_run_once_and_collect_responses", false)]
    #[tokio::test]
    pub async fn lit_action_from_file(file_name: &str, assert_sig: bool) {
        common::init_test_config();
        info!("Starting test: lit_action_from_file: {}.js", file_name);
        let file_name = &format!("./tests/lit_action_scripts/{}.js", file_name);
        let num_nodes = 3;
        let (testnet, validator_collection) = new_node_collection(num_nodes, false).await;

        let validation =
            sign_from_file_system(&validator_collection, &testnet, file_name, assert_sig)
                .await
                .unwrap();

        info!("Validation: {:?}", validation);
        assert!(validation);
    }

    #[test_case("multiple_sign_and_combine", false)]
    #[tokio::test]
    pub async fn test_lit_actions_signing_from_file_with_response_check(
        file_name: &str,
        _assert_sig: bool,
    ) {
        common::init_test_config();
        info!(
            "Starting test: test_lit_actions_signing_from_file: {}.js",
            file_name
        );
        let file_with_path = &format!("./tests/lit_action_scripts/{}.js", file_name);
        let num_nodes = 3;

        let (testnet, validator_collection) = new_node_collection(num_nodes, false).await;
        let actions = validator_collection.actions();

        let wallet = testnet.deploy_account.signing_provider.signer();
        let auth_sig = generate_authsig_item(wallet)
            .await
            .expect("Could not create auth sig");
        let (lit_action_code, ipfs_id, js_params, auth_methods) =
            prepare_sign_from_file_parameters(validator_collection.actions(), file_with_path)
                .await
                .expect("Could not prepare sign from file parameters");

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
        .await
        .expect("Could not execute lit action");

        info!("execute_resp: {:?}", execute_resp);
        if file_name == "multiple_sign_and_combine" {
            for resp in execute_resp {
                info!("resp: {:?}", resp);
                let json_object: HashMap<String, serde_json::Value> =
                    serde_json::from_str(&resp).unwrap();
                let response = json_object.get("response").unwrap().as_str().unwrap();
                let response_obj: HashMap<String, serde_json::Value> =
                    serde_json::from_str(response).unwrap();
                // info!("Response object: {:?}", response_obj);
                let sig_names = vec!["sig1", "sig2"];
                for sig_name in sig_names {
                    let sig = response_obj.get(sig_name).unwrap().as_object().unwrap();
                    let r = sig.get("r").unwrap().as_str().unwrap();
                    let s = sig.get("s").unwrap().as_str().unwrap();
                    let v = sig.get("v").unwrap().as_u64().unwrap();
                    assert!(r.len() == 66);
                    assert!(s.len() == 64);
                    assert!(v == 0 || v == 1, "V recovery param must be 1 or 0");
                }
            }
        }
    }

    #[test_case("simple_decrypt_to_single_node", false)]
    #[test_case("simple_decrypt_and_combine_with_auth_sig", false)]
    #[test_case("simple_encrypt", false)]
    #[test_case("check_conditions_with_auth_sig", false)]
    #[tokio::test]
    pub async fn lit_action_from_file_with_encryption_with_auth_sig(
        file_name: &str,
        _assert_sig: bool,
    ) {
        common::init_test_config();
        info!(
            "Starting test: lit_action_from_file_with_encryption_with_auth_sig: {}.js",
            file_name
        );
        let file_with_path = &format!("./tests/lit_action_scripts/{}.js", file_name);
        let num_nodes = 3;

        let (_testnet, validator_collection) = new_node_collection(num_nodes, false).await;
        let actions = validator_collection.actions();
        let (access_control_conditions, ciphertext, data_to_encrypt_hash, auth_sig) =
            get_encryption_decryption_test_params(actions).await;

        info!("access_control_conditions: {:?}", access_control_conditions);
        info!("ciphertext: {:?}", ciphertext);
        info!("data_to_encrypt_hash: {:?}", data_to_encrypt_hash);
        info!("auth_sig: {:?}", auth_sig);

        let lit_action_code = std::fs::read_to_string(file_with_path).unwrap();
        let pkp_info = mint_next_pkp(actions).await.unwrap();
        let pubkey = pkp_info.0;

        let lit_action_code = lit_action_code.replace(
            "[access_control_conditions]",
            &serde_json::to_string(&access_control_conditions.unwrap()).unwrap(),
        );
        let lit_action_code = lit_action_code.replace("[ciphertext]", &ciphertext);
        let lit_action_code =
            lit_action_code.replace("[data_to_encrypt_hash]", &data_to_encrypt_hash);
        let lit_action_code =
            lit_action_code.replace("[auth_sig]", &serde_json::to_string(&auth_sig).unwrap());

        info!("Lit action code: {:?}", lit_action_code);

        let (lit_action_code, ipfs_id, js_params, auth_methods) =
            sign_lit_action(lit_action_code, pubkey).await.unwrap();
        let signing_provider = actions.deployer_signing_provider();
        let wallet = signing_provider.signer();
        let auth_sig = generate_authsig_item(wallet).await.unwrap();

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
        .await;

        info!("execute_resp: {:?}", execute_resp);

        // this needs to be refactored into some kind of callback that we pass into the test_case macro, and we need to write these for all the test cases
        if file_name == "check_conditions_with_auth_sig" {
            for resp in execute_resp.unwrap() {
                assert_eq!(
                resp,
                "{\"success\":true,\"signedData\":{},\"decryptedData\":{},\"claimData\":{},\"response\":\"true\",\"logs\":\"\"}",
            );
            }
        } else if file_name == "simple_decrypt_and_combine_with_auth_sig" {
            let mut matches = 0;
            for resp in execute_resp.unwrap() {
                matches = matches + match resp.as_str() {
                    "{\"success\":true,\"signedData\":{},\"decryptedData\":{},\"claimData\":{},\"response\":\"\\\"hello this is a test\\\"\",\"logs\":\"\"}" => 1,
                    _ =>  { assert!(false, "Unexpected response: {}", resp); 0 },
                }
            }
            assert_eq!(
                matches, num_nodes,
                "Expected {} matches, got {}",
                num_nodes, matches
            );
        } else if file_name == "simple_decrypt_to_single_node" {
            let mut matches = 0;
            for resp in execute_resp.unwrap() {
                matches = matches + match resp.as_str() {
                    "{\"success\":true,\"signedData\":{},\"decryptedData\":{},\"claimData\":{},\"response\":\"\\\"hello this is a test\\\"\",\"logs\":\"\"}" => 1,
                    "{\"success\":true,\"signedData\":{},\"decryptedData\":{},\"claimData\":{},\"response\":\"\\\"\\\"\",\"logs\":\"\"}" => 0,
                    _ =>  { assert!(false, "Unexpected response: {}", resp); 0 },
                }
            }
            assert_eq!(matches, 1, "Expected 1 match, got {}", matches);
        } else if file_name == "simple_encrypt" {
            let regex = Regex::new(r#""response":"(\w|/|\+)+""#).unwrap();
            for resp in execute_resp.unwrap() {
                assert!(regex.is_match(&resp));
            }
        }

        // assert!(validation);
    }

    #[test_case("simple_decrypt_and_combine_without_auth_sig", false)]
    #[test_case("check_conditions_without_auth_sig", false)]
    #[test_case("simple_encrypt_and_decrypt", false)]
    #[tokio::test]
    pub async fn lit_action_from_file_with_encryption_without_auth_sig(
        file_name: &str,
        _assert_sig: bool,
    ) {
        common::init_test_config();
        info!(
            "Starting test: lit_action_from_file_with_encryption_without_auth_sig: {}.js",
            file_name
        );
        let file_with_path = &format!("./tests/lit_action_scripts/{}.js", file_name);
        let num_nodes = 3;

        let (_testnet, validator_collection) = new_node_collection(num_nodes, false).await;
        let actions = validator_collection.actions();
        let (access_control_conditions, ciphertext, data_to_encrypt_hash, auth_sig) =
            get_encryption_decryption_test_params(actions).await;

        info!("access_control_conditions: {:?}", access_control_conditions);
        info!("ciphertext: {:?}", ciphertext);
        info!("data_to_encrypt_hash: {:?}", data_to_encrypt_hash);
        info!("auth_sig: {:?}", auth_sig);

        let lit_action_code = std::fs::read_to_string(file_with_path).unwrap();
        let pkp_info = mint_next_pkp(actions).await.unwrap();
        let pubkey = pkp_info.0;

        let lit_action_code = lit_action_code.replace(
            "[access_control_conditions]",
            &serde_json::to_string(&access_control_conditions.unwrap()).unwrap(),
        );
        let lit_action_code = lit_action_code.replace("[ciphertext]", &ciphertext);
        let lit_action_code =
            lit_action_code.replace("[data_to_encrypt_hash]", &data_to_encrypt_hash);

        info!("Lit action code: {:?}", lit_action_code);

        let (lit_action_code, ipfs_id, js_params, auth_methods) =
            sign_lit_action(lit_action_code, pubkey).await.unwrap();
        let signing_provider = actions.deployer_signing_provider();
        let wallet = signing_provider.signer();
        let auth_sig = generate_authsig_item(wallet).await.unwrap();

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
        .await;

        info!("execute_resp: {:?}", execute_resp);
        if file_name == "check_conditions_without_auth_sig" {
            for resp in execute_resp.unwrap() {
                info!("resp: {:?}", resp);
                assert_eq!(
                resp,
                "{\"success\":true,\"signedData\":{},\"decryptedData\":{},\"claimData\":{},\"response\":\"true\",\"logs\":\"\"}",
            );
            }
        } else if file_name == "simple_decrypt_and_combine_without_auth_sig" {
            let mut matches = 0;
            for resp in execute_resp.unwrap() {
                matches = matches + match resp.as_str() {
                    "{\"success\":true,\"signedData\":{},\"decryptedData\":{},\"claimData\":{},\"response\":\"\\\"hello this is a test\\\"\",\"logs\":\"\"}" => 1,
                    "{\"success\":true,\"signedData\":{},\"decryptedData\":{},\"claimData\":{},\"response\":\"\\\"\\\"\",\"logs\":\"\"}" => 0,
                    _ =>  { assert!(false, "Unexpected response: {}", resp); 0 },
                }
            }
            assert_eq!(
                matches, num_nodes,
                "Expected {} match, got {}",
                num_nodes, matches
            );
        } else if file_name == "simple_encrypt_and_decrypt" {
            let regex =
                Regex::new(r#""response":"this is a secret that was encrypted with lit""#).unwrap();
            for resp in execute_resp.unwrap() {
                assert!(regex.is_match(&resp));
            }
        }

        // assert!(validation);
    }

    #[test_case("decrypt_and_combine_with_access_denied", false)]
    #[tokio::test]
    pub async fn lit_action_from_file_with_access_denied(file_name: &str, _assert_sig: bool) {
        common::init_test_config();
        info!(
            "Starting test: lit_action_from_file_with_access_denied: {}.js",
            file_name
        );
        let file_with_path = &format!("./tests/lit_action_scripts/{}.js", file_name);
        let num_nodes = 3;

        let (_testnet, validator_collection) = new_node_collection(num_nodes, false).await;
        let actions = validator_collection.actions();

        // create a condition that will always fail.  we check that they have greater than the max UINT256 value which is impossible
        let access_control_conditions = vec![
            UnifiedAccessControlConditionItem::Condition(
                UnifiedAccessControlCondition::JsonAccessControlCondition(
                    JsonAccessControlCondition {
                        contract_address: "".to_string(),
                        chain: CHAIN_LOCALCHAIN.to_string(),
                        standard_contract_type: "".to_string(),
                        method: "eth_getBalance".to_string(),
                        parameters: vec![":userAddress".to_string(), "latest".to_string()],
                        return_value_test: JsonReturnValueTest {
                            comparator: ">".to_string(),
                            value: "115792089237316195423570985008687907853269984665640564039457584007913129639935".to_string(),
                        },
                    },
                ),
            ),
        ];

        let to_encrypt = "this is a super secret message";
        let (ciphertext, data_to_encrypt_hash) =
            encrypt_with_bls_key(actions, to_encrypt, access_control_conditions.clone()).await;

        info!("access_control_conditions: {:?}", access_control_conditions);
        info!("ciphertext: {:?}", ciphertext);
        info!("data_to_encrypt_hash: {:?}", data_to_encrypt_hash);

        let lit_action_code = std::fs::read_to_string(file_with_path).unwrap();
        let pkp_info = mint_next_pkp(actions).await.unwrap();
        let pubkey = pkp_info.0;

        let lit_action_code = lit_action_code.replace(
            "[access_control_conditions]",
            &serde_json::to_string(&access_control_conditions).unwrap(),
        );
        let lit_action_code = lit_action_code.replace("[ciphertext]", &ciphertext);
        let lit_action_code =
            lit_action_code.replace("[data_to_encrypt_hash]", &data_to_encrypt_hash);

        info!("Lit action code: {:?}", lit_action_code);

        let (lit_action_code, ipfs_id, js_params, auth_methods) =
            sign_lit_action(lit_action_code, pubkey).await.unwrap();
        let signing_provider = actions.deployer_signing_provider();
        let wallet = signing_provider.signer();
        let auth_sig = generate_authsig_item(wallet).await.unwrap();

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
        .await;

        info!("execute_resp: {:?}", execute_resp);
        if file_name == "decrypt_and_combine_with_access_denied" {
            #[derive(serde::Deserialize)]
            struct ErrorResponse {
                success: bool,
                error: String,
                logs: String,
            }

            let resp: ErrorResponse =
                serde_json::from_str(execute_resp.unwrap().first().unwrap()).unwrap();

            assert!(!resp.success);
            assert!(resp.error.starts_with("Uncaught (in promise) Error: Access control conditions check failed.  Check that you are allowed to decrypt this item."));
            assert_eq!(resp.logs, "Checking access...\n");
        }

        // assert!(validation);
    }

    #[test_case("current_ipfs_id_substitution", false)]
    #[tokio::test]
    pub async fn test_current_ipfs_id_substitution(file_name: &str, _assert_sig: bool) {
        common::init_test_config();
        info!(
            "Starting test: test_current_ipfs_id_substitution: {}.js",
            file_name
        );

        let num_nodes = 3;
        let (_testnet, validator_collection) = new_node_collection(num_nodes, false).await;
        let actions = validator_collection.actions();

        let file_with_path = &format!("./tests/lit_action_scripts/{}.js", file_name);
        let lit_action_code = std::fs::read_to_string(file_with_path).unwrap();
        // calculate IPFS ID of lit action code
        let ipfs_hasher = IpfsHasher::default();
        let cid = ipfs_hasher.compute(lit_action_code.as_bytes());
        let derived_ipfs_id = cid;

        // create a condition that checks if the current IPFS ID is equal to the derived IPFS ID
        let unified_access_control_conditions = vec![UnifiedAccessControlConditionItem::Condition(
            UnifiedAccessControlCondition::JsonAccessControlCondition(JsonAccessControlCondition {
                contract_address: "".to_string(),
                chain: CHAIN_LOCALCHAIN.to_string(),
                standard_contract_type: "".to_string(),
                method: "".to_string(),
                parameters: vec![":currentActionIpfsId".to_string()],
                return_value_test: JsonReturnValueTest {
                    comparator: "=".to_string(),
                    value: derived_ipfs_id,
                },
            }),
        )];

        let to_encrypt = "this is a super secret message";
        let (ciphertext, data_to_encrypt_hash) = encrypt_with_bls_key(
            actions,
            to_encrypt,
            unified_access_control_conditions.clone(),
        )
        .await;

        info!("ciphertext: {:?}", ciphertext);
        info!("data_to_encrypt_hash: {:?}", data_to_encrypt_hash);

        // let pkp_info = mint_next_pkp(actions).await.unwrap();
        // let pubkey = pkp_info.0;

        let signing_provider = actions.deployer_signing_provider();
        let wallet = signing_provider.signer();
        let auth_sig = generate_authsig_item(wallet).await.unwrap();

        // run
        let lit_action_code_base64 = data_encoding::BASE64.encode(lit_action_code.as_bytes());
        let execute_resp = execute_lit_action(
            validator_collection.actions(),
            Some(lit_action_code_base64),
            None,
            Some(serde_json::json!({
                "ciphertext": ciphertext,
                "dataToEncryptHash": data_to_encrypt_hash,
                "accessControlConditions": unified_access_control_conditions,
            })),
            None,
            auth_sig,
            EndpointVersion::Initial,
        )
        .await;

        info!("execute_resp: {:?}", execute_resp);
        if file_name == "current_ipfs_id_substitution" {
            for resp in execute_resp.unwrap() {
                info!("resp: {:?}", resp);
                assert_eq!(
                resp,
                "{\"success\":true,\"signedData\":{},\"decryptedData\":{},\"claimData\":{},\"response\":\"this is a super secret message\",\"logs\":\"\"}",
            );
            }
        }

        // assert!(validation);
    }

    #[test_case("decrypt_and_combine_with_jsparams", false)]
    #[tokio::test]
    pub async fn test_lit_action_evm_contract_conditions(file_name: &str, _assert_sig: bool) {
        common::init_test_config();
        info!(
            "Starting test: test_lit_action_evm_contract_conditions: {}.js",
            file_name
        );
        let file_with_path = &format!("./tests/lit_action_scripts/{}.js", file_name);
        let num_nodes = 3;

        let (_testnet, validator_collection) = new_node_collection(num_nodes, false).await;
        let actions = validator_collection.actions();
        let staking_contract_address =
            format!("0x{}", bytes_to_hex(actions.contracts().staking.address()));
        let abi = r#"
        {
            "inputs": [],
            "name": "getValidatorsInCurrentEpochLength",
            "outputs": [
              {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          }
        "#;

        let eth_abi: ethabi::Function = serde_json::from_str(abi).unwrap();

        // create a condition that will use EVM contract conditions
        let access_control_conditions = vec![UnifiedAccessControlConditionItem::Condition(
            UnifiedAccessControlCondition::EVMContractCondition(EVMContractCondition {
                contract_address: staking_contract_address,
                chain: CHAIN_LOCALCHAIN.to_string(),
                function_name: "getValidatorsInCurrentEpochLength".to_string(),
                function_params: vec![],
                function_abi: eth_abi,
                return_value_test: JsonReturnValueTestV2 {
                    key: "".to_string(),
                    comparator: "=".to_string(),
                    value: "3".to_string(),
                },
            }),
        )];

        let to_encrypt = "this is a super secret message";
        let (ciphertext, data_to_encrypt_hash) =
            encrypt_with_bls_key(actions, to_encrypt, access_control_conditions.clone()).await;

        info!("access_control_conditions: {:?}", access_control_conditions);
        info!("ciphertext: {:?}", ciphertext);
        info!("data_to_encrypt_hash: {:?}", data_to_encrypt_hash);

        let lit_action_code = std::fs::read_to_string(file_with_path).unwrap();
        info!("Lit action code: {:?}", lit_action_code);
        let lit_action_code = data_encoding::BASE64.encode(lit_action_code.as_bytes());

        let signing_provider = actions.deployer_signing_provider();
        let wallet = signing_provider.signer();
        let auth_sig = generate_authsig_item(wallet).await.unwrap();

        // create js params wiht access control conditions and ciphertext and datatoencrypthash

        let mut js_params = serde_json::Map::new();
        js_params.insert(
            "accessControlConditions".to_string(),
            serde_json::to_value(access_control_conditions).unwrap(),
        );
        js_params.insert("ciphertext".to_string(), ciphertext.into());
        js_params.insert("dataToEncryptHash".to_string(), data_to_encrypt_hash.into());

        // run
        let execute_resp = execute_lit_action(
            validator_collection.actions(),
            Some(lit_action_code),
            None,
            Some(serde_json::Value::Object(js_params)),
            None,
            auth_sig,
            EndpointVersion::Initial,
        )
        .await;

        info!("execute_resp: {:?}", execute_resp);
        if file_name == "simple_decrypt_and_combine_without_auth_sig" {
            for resp in execute_resp.unwrap() {
                info!("resp: {:?}", resp);
                assert_eq!(
                resp,
                "{\"success\":true,\"signedData\":{},\"decryptedData\":{},\"claimData\":{},\"response\":\"\\\"this is a super secret message\\\"\",\"logs\":\"\"}",
            );
            }
        }

        // assert!(validation);
    }

    pub async fn get_encryption_decryption_test_params(
        actions: &Actions,
    ) -> (
        Option<Vec<ControlConditionItem<UnifiedAccessControlCondition>>>,
        String,
        String,
        JsonAuthSig,
    ) {
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

        info!("Identity parameter: {:?}", identity_param);

        let ciphertext = lit_bls_wasm::encrypt_time_lock::<Bls12381G2Impl>(
            &network_pubkey,
            message_bytes.to_vec(),
            identity_param.clone(),
        )
        .expect("Unable to encrypt");
        info!("ciphertext: {:?}", ciphertext);

        (
            unified_access_control_conditions,
            ciphertext,
            data_to_encrypt_hash,
            auth_sig,
        )
    }

    async fn encrypt_with_bls_key(
        actions: &Actions,
        to_encrypt: &str,
        unified_access_control_conditions: Vec<UnifiedAccessControlConditionItem>,
    ) -> (String, String) {
        // Encrypt.
        let mut hasher = Sha256::new();
        hasher.update(to_encrypt.as_bytes());
        let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

        let network_pubkey = get_network_pubkey(actions).await;
        let message_bytes = to_encrypt.as_bytes();
        let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
            access_control_conditions: None,
            evm_contract_conditions: None,
            sol_rpc_conditions: None,
            unified_access_control_conditions: Some(unified_access_control_conditions),
        })
        .unwrap();
        let identity_param = AccessControlConditionResource::new(format!(
            "{}/{}",
            hashed_access_control_conditions, data_to_encrypt_hash
        ))
        .get_resource_key()
        .into_bytes();

        info!("Identity parameter: {:?}", identity_param);

        let ciphertext = lit_bls_wasm::encrypt_time_lock::<Bls12381G2Impl>(
            &network_pubkey,
            message_bytes.to_vec(),
            identity_param.clone(),
        )
        .expect("Unable to encrypt");
        info!("ciphertext: {:?}", ciphertext);

        (ciphertext, data_to_encrypt_hash)
    }

    #[tokio::test]
    async fn test_pkp_permissions_is_cid_registered_and_can_it_sign() {
        common::init_test_config();
        let num_nodes = 3;

        let (_testnet, validator_collection) = new_node_collection(num_nodes, false).await;
        const IPFS_CID: &str = "QmRwN9GKHvCn4Vk7biqtr6adjXMs7PzzYPCzNCRjPFiDjm";

        let res = common::lit_actions::generate_pkp_check_get_permitted_pkp_action(
            IPFS_CID,
            &validator_collection,
        )
        .await;

        assert!(res.is_ok());

        let res = res.unwrap();
        let (pkp_pubkey, res) = res;

        assert!(!res.is_empty());
        assert!(res[0] == IPFS_CID);
        info!("get permitted action result: {:?}", res);

        // test signing with it, using a diff key to send the txn (one that doesnt own the PKP)
        let mut js_params = serde_json::Map::new();
        js_params.insert("publicKey".to_string(), pkp_pubkey.clone().into());
        js_params.insert("sigName".to_string(), "sig1".into());
        js_params.insert(
            "toSign".to_string(),
            serde_json::Value::Array(
                keccak256("Hello World".as_bytes())
                    .map(|f| serde_json::Value::Number(f.into()))
                    .to_vec(),
            ),
        );
        let wallet = _testnet.node_accounts[2].signing_provider.signer();
        let auth_sig = generate_authsig_item(wallet)
            .await
            .expect("Could not generate auth sig");
        let res = execute_lit_action(
            validator_collection.actions(),
            None,
            Some(IPFS_CID.to_string()),
            Some(serde_json::Value::Object(js_params)),
            None,
            auth_sig,
            EndpointVersion::Initial,
        )
        .await;
        assert!(res.is_ok());
        let res = res.unwrap();

        let asserted = assert_signed_action(&validator_collection, res).await;
        assert!(asserted.is_ok());
        let asserted = asserted.unwrap();
        assert!(asserted);

        // test that the error when trying to sign something smaller than 32 bytes works
        let mut js_params = serde_json::Map::new();
        js_params.insert("publicKey".to_string(), pkp_pubkey.into());
        js_params.insert("sigName".to_string(), "sig1".into());
        js_params.insert("toSign".to_string(), serde_json::json!([1, 2, 3, 4]));
        let wallet = _testnet.node_accounts[2].signing_provider.signer();
        let auth_sig = generate_authsig_item(wallet)
            .await
            .expect("Could not generate auth sig");
        let res = execute_lit_action(
            validator_collection.actions(),
            None,
            Some(IPFS_CID.to_string()),
            Some(serde_json::Value::Object(js_params)),
            None,
            auth_sig,
            EndpointVersion::Initial,
        )
        .await;
        assert!(res.is_ok());
        let res = res.unwrap();
        let json_responses: Vec<Value> = res
            .iter()
            .map(|x| serde_json::from_str(x).expect("Could not parse JSON response"))
            .collect();
        let mut error_counts = 0;
        // check all responses and that we got 2 errors
        for response in json_responses {
            if !response.is_object() || !response.as_object().unwrap().contains_key("error") {
                continue;
            }
            let err_str = response["error"].to_string();
            if err_str.contains("Message length to be signed is not 32 bytes.") {
                error_counts += 1;
            }
        }
        assert!(error_counts == 2);
    }

    #[tokio::test]
    #[ignore]
    async fn test_pkp_is_permissions_is_permitted_action() {
        common::init_test_config();
        let num_nodes = 3;

        let (_testnet, validator_collection) = new_node_collection(num_nodes, false).await;
        const IPFS_CID: &str = "QmZuSixiCCkttPDvHKZdAw2gNby111rNEq1aHTozkqztSg";

        let res = common::lit_actions::generate_pkp_check_is_permitted_pkp_action(
            IPFS_CID,
            &validator_collection,
        )
        .await;

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res);
        info!("is permitted action result: {:?}", res);
    }
}
