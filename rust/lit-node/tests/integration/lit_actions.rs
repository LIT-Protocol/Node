#[cfg(feature = "lit-actions")]
pub mod litactions {
    use blsful::Bls12381G2Impl;
    use ethers::signers::Wallet;
    use ipfs_hasher::IpfsHasher;
    use k256::ecdsa::SigningKey;
    use lit_core::utils::binary::bytes_to_hex;
    use lit_node::auth::auth_material::JsonAuthSig;
    use lit_node::auth::lit_resource::LitResource;
    use lit_node::auth::resources::AccessControlConditionResource;
    use lit_node::constants::CHAIN_LOCALCHAIN;
    use lit_node::models::SignedData;
    use lit_node::models::{
        ControlConditionItem, EVMContractCondition, JsonAccessControlCondition,
        JsonReturnValueTest, JsonReturnValueTestV2, RequestConditions,
        UnifiedAccessControlCondition, UnifiedAccessControlConditionItem,
    };
    use lit_node::utils::web::{hash_access_control_conditions, EndpointVersion};
    use serde_json::Value;
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use test_case::test_case;
    use test_common::auth_sig::{generate_authsig, generate_authsig_item};
    use test_common::init_test_config;
    use test_common::lit_actions::{
        execute_lit_action, generate_pkp_check_is_permitted_pkp_action, ActionReturn,
    };
    use test_common::pkp::SignedDatak256;
    use test_common::pkp::{mint_next_pkp, recombine_shares_using_wasm};
    use test_common::validator::ValidatorCollection;
    use test_common::{
        new_node_collection, node_collection::get_network_pubkey, testnet::actions::Actions,
    };
    use tracing::info;

    // Notes:
    // - The 2 tests inside test_pkp_permissions_is_cid_registered_and_can_it_sign, is covered by "sign_child_lit_action" & "fail_sign_non_hashed_message".
    // - The original encrypt test wasn't a good integration test - it attempted to compare against a known pubkey, but integration tests generate new keys each time.  encrypt & decrypt tests cover this functionality.
    #[test_case("broadcast_and_collect", &all_response_match, &standard_acc, true, "*", None)] /* Success */
    #[test_case("check_conditions_with_auth_sig", &all_response_match, &standard_acc, true, "true", None)] /* Success */
    #[test_case("check_conditions_without_auth_sig", &all_response_match, &standard_acc, false,  "true", None)] /* Success <<< BUT CHECK */
    #[test_case("current_ipfs_id_substitution", &all_response_match, &ipfs_acc, true, "hello this is a test", None)] /* Success */
    #[test_case("decrypt_and_combine_with_access_denied", &action_failed_with_error, &impossible_acc, true, "Access control conditions check failed", None)] /* Success */
    #[test_case("decrypt_and_combine_with_auth_sig", &all_response_match, &standard_acc, true, "hello this is a test", None)] /* Success */
    #[test_case("decrypt_and_combine_without_auth_sig", &all_response_match, &standard_acc, false, "*", None)]
    #[test_case("decrypt_to_single_node", &single_valid, &standard_acc, true, "hello this is a test", None)]
    #[test_case("get_rpc_url", &all_response_match, &standard_acc,true, "https://api.node.glif.io/rpc/v1", None)] /* local rpc config */
    #[test_case("multiple_sign_and_combine", &valid_sign_and_combine, &standard_acc, true, "", None)]
    #[test_case("run_once_and_collect_responses", &all_response_match, &standard_acc,true, "*", None)]
    #[test_case("run_once", &all_response_match, &standard_acc,true, "*", None)]
    #[test_case("sign_and_combine_ecdsa", &all_response_match, &standard_acc,true, "*", None)]
    #[test_case("sign_hello_world", &valid_sign_no_combine, &standard_acc, true, "", None)]
    #[test_case("sign_child_lit_action", &valid_sign_no_combine, &standard_acc, true, "", None)]
    #[test_case("fail_sign_non_hashed_message", &action_failed_with_error, &standard_acc, true, "Message length to be signed is not 32 bytes", None)]
    #[tokio::test]

    pub async fn lit_action_from_file(
        file_name: &str,
        fn_assertion: &dyn Fn(Vec<String>, serde_json::Map<String, Value>, &str) -> bool,
        fn_accs: &dyn Fn(&str, &Actions) -> Vec<UnifiedAccessControlConditionItem>,
        include_auth_sig: bool,
        value: &str,
        validator_collection: Option<&ValidatorCollection>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        init_test_config();
        info!("Starting test: {}.js", file_name);
        let file_with_path = &format!("./tests/lit_action_scripts/{}.js", file_name);
        let num_nodes = 3;
        let validator_collection = match validator_collection {
            Some(vc) => vc,
            None => &new_node_collection(num_nodes, false).await.1,
        };
        // let (_testnet, validator_collection) = new_node_collection(num_nodes, false).await;
        let actions = validator_collection.actions();
        let wallet = actions.deployer_signing_provider().signer().clone();
        let lit_action_code = std::fs::read_to_string(file_with_path).unwrap();
        //let lit_action_code = lit_action_code.replace("\n", "");

        // this isn't used for all actions, but it gets ignored when we pass it in for unrelated actions
        let (access_control_conditions, ciphertext, data_to_encrypt_hash, auth_sig) =
            get_encryption_decryption_test_params(
                wallet.clone(),
                actions,
                value,
                &lit_action_code,
                fn_accs,
            )
            .await;

        let (pubkey, _token_id, _ether_address) = mint_next_pkp(actions).await.unwrap();

        let lit_action_code = data_encoding::BASE64.encode(lit_action_code.as_bytes());
        // per above, there are more params than needed for some actions, but they are ignored
        let mut js_params = serde_json::Map::new();
        js_params.insert("publicKey".to_string(), pubkey.clone().into());
        js_params.insert("sigName".to_string(), "sig1".into());
        js_params.insert("ciphertext".to_string(), ciphertext.into());
        js_params.insert("dataToEncryptHash".to_string(), data_to_encrypt_hash.into());
        js_params.insert(
            "accessControlConditions".to_string(),
            serde_json::to_value(access_control_conditions.unwrap()).unwrap(),
        );
        if include_auth_sig {
            js_params.insert(
                "authSig".to_string(),
                serde_json::to_value(auth_sig).unwrap(),
            );
        };
        let params = js_params.clone();
        let js_params = Some(serde_json::Value::Object(js_params));

        let ipfs_id = None;
        let auth_methods = None;

        let auth_sig_item = generate_authsig_item(&wallet).await.unwrap();

        // run
        let execute_resp = execute_lit_action(
            &actions,
            Some(lit_action_code),
            ipfs_id,
            js_params,
            auth_methods,
            auth_sig_item,
            EndpointVersion::Initial,
        )
        .await;

        let success = fn_assertion(
            execute_resp.unwrap(),
            params, // this is the params that were passed into the lit action
            &format!("\"{}\"", value),
        );

        assert!(success);

        if success {
            Ok(())
        } else {
            Err("Test failed".into())
        }
    }

    // functions that check the response values

    #[doc = "Checks that all the 'response' values match and that each node provides a 'success' result of true"]
    fn all_response_match(
        execute_resp: Vec<String>,
        _params: serde_json::Map<String, Value>,
        value: &str,
    ) -> bool {
        let values = vec![value.to_string(); execute_resp.len()];
        count_matching_responses(execute_resp.len(), true, execute_resp, &values)
    }

    #[doc = "Checks that the 'response' values of a single node  match incoming data and that each node provides a 'success' result that is true"]
    fn single_valid(
        execute_resp: Vec<String>,
        _params: serde_json::Map<String, Value>,
        value: &str,
    ) -> bool {
        let values = vec![value.to_string(); execute_resp.len()];
        count_matching_responses(1, true, execute_resp, &values)
    }

    #[doc = "Checks that the 'response' values of a single node against known encryption shares and that each node provides a 'success' result that is true"]
    fn valid_encryption(
        execute_resp: Vec<String>,
        params: serde_json::Map<String, Value>,
        _value: &str,
    ) -> bool {
        let mut values: Vec<String> = Vec::new();
        // a bit silly, but we're going to re-encrypt the data, recreating the identity params based on the nodes' hashed
        // values and check that the encryption shares match...

        for resp in &execute_resp {
            let result = serde_json::from_str::<serde_json::Value>(resp).unwrap();

            let public_key =
                serde_json::from_value::<String>(params.get("publicKey").unwrap().clone()).unwrap();

            let message = "Hello World".as_bytes().to_vec();

            let data_to_encrypt_hash = result
                .get("data_to_encrypt_hash")
                .unwrap()
                .as_str()
                .unwrap();

            let access_control_conditions =
                serde_json::from_value::<
                    Option<Vec<ControlConditionItem<UnifiedAccessControlCondition>>>,
                >(params.get("accessControlConditions").unwrap().clone())
                .unwrap();

            let conditions = access_control_conditions.unwrap();

            let identity = lit_node::functions::action_client::Client::get_identity_param(
                &conditions,
                &data_to_encrypt_hash,
            )
            .unwrap();

            let value =
                lit_bls_wasm::encrypt_time_lock::<Bls12381G2Impl>(&public_key, message, identity)
                    .unwrap();
            values.push(value);
        }

        // map values to s_values

        count_matching_responses(execute_resp.len(), true, execute_resp, &values)
    }

    #[doc = "Validate multiple signatures"]
    fn valid_sign_and_combine(
        execute_resp: Vec<String>,
        _params: serde_json::Map<String, Value>,
        _value: &str,
    ) -> bool {
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
        true
    }

    #[doc = "Validate single signature"]
    fn valid_sign_no_combine(
        execute_resp: Vec<String>,
        _params: serde_json::Map<String, Value>,
        _value: &str,
    ) -> bool {
        // collect the shares into a struct and a set of string that can be used to recombine using the WASM module.
        // currently designed to handle just a single siganture.
        let mut shares = vec![];
        for resp in execute_resp {
            let json_object: HashMap<String, serde_json::Value> =
                serde_json::from_str(&resp).unwrap();
            info!("json_object: {:?}", json_object);
            // let signed_data = json_object.get("signedData").unwrap().as_str().unwrap();

            let sig1 = json_object.get("signedData");

            if sig1.is_none() {
                error!("Could not find signedData in response: {:?}", json_object);
                return false;
            }

            let sig1 = sig1.unwrap().get("sig1").unwrap().clone();
            let la_signed_data: Result<SignedData, serde_json::Error> =
                serde_json::from_value(sig1);

            if la_signed_data.is_err() {
                error!("Could not parse result: {:?}", la_signed_data.err());
                continue;
            }

            let la_signed_data = la_signed_data.unwrap();
            if la_signed_data.data_signed.contains("fail") {
                info!("Failed share returned - normal if node not selected.");
            } else {
                let signed_data: SignedDatak256 = SignedDatak256 {
                    sig_type: la_signed_data.sig_type,
                    data_signed: serde_json::from_str(&la_signed_data.data_signed).unwrap(),
                    signature_share: serde_json::from_str(&la_signed_data.signature_share).unwrap(),
                    share_index: la_signed_data.share_index,
                    big_r: serde_json::from_str(&la_signed_data.big_r).unwrap(),
                    public_key: serde_json::from_str(&la_signed_data.public_key).unwrap(),
                    sig_name: la_signed_data.sig_name,
                };
                info!("signed_data: {:?}", signed_data);
                shares.push(serde_json::to_string(&signed_data).unwrap());
            }
        }

        let (signature, recovery_id) =
            recombine_shares_using_wasm(shares).expect("Failed to recombine shares");

        info!("Signature: {:?}", signature);
        info!("Recovery ID: {:?}", recovery_id);
        true
    }

    #[doc = "Counts the number of responses that match the expected values and returns true if the count matches the expected quantity"]
    fn count_matching_responses(
        qty: usize,
        is_success: bool,
        execute_resp: Vec<String>,
        values: &Vec<String>,
    ) -> bool {
        let results: Vec<ActionReturn> = execute_resp
            .iter()
            .map(|r| serde_json::from_str::<ActionReturn>(r).unwrap())
            .collect();

        let mut valid = 0;
        for (i, result) in results.iter().enumerate() {
            if result.success != is_success {
                error!(
                    "Bad success: {} - Looking for {}",
                    result.success, is_success
                );
                return false;
            }
            if result.response != values[i] && values[i] != "\"*\"" {
                error!(
                    "Bad response: |{}| - Looking for {}",
                    result.response, values[i]
                );
            } else {
                valid += 1;
            }
        }
        qty == valid
    }

    #[doc = "Checks for an invalid authsig."]
    fn action_failed_with_error(
        execute_resp: Vec<String>,
        _params: serde_json::Map<String, Value>,
        value: &str,
    ) -> bool {
        let results: Vec<ActionReturn> = execute_resp
            .iter()
            .map(|r| serde_json::from_str::<ActionReturn>(r).unwrap())
            .collect();

        let value = &value.to_string().replace("\"", "");
        let mut result_count = results.len();
        let mut err_count = 0;
        for (i, result) in results.iter().enumerate() {
            if result.success {
                error!("Success returned: {} - Looking for false, though this may be a non selected node.", result.success);
                // return false;
                result_count -= 1;
            }
            if result.error.contains(value) {
                err_count += 1;
            }
        }
        err_count == result_count
    }

    fn standard_acc(
        _lit_action_code: &str,
        _actions: &Actions,
    ) -> Vec<UnifiedAccessControlConditionItem> {
        vec![UnifiedAccessControlConditionItem::Condition(
            UnifiedAccessControlCondition::JsonAccessControlCondition(JsonAccessControlCondition {
                contract_address: "".to_string(),
                chain: CHAIN_LOCALCHAIN.to_string(),
                standard_contract_type: "".to_string(),
                method: "eth_getBalance".to_string(),
                parameters: vec![":userAddress".to_string(), "latest".to_string()],
                return_value_test: JsonReturnValueTest {
                    comparator: ">=".to_string(),
                    value: "0".to_string(),
                },
            }),
        )]
    }

    // create a condition that will always fail.  we check that they have greater than the max UINT256 value which is impossible
    fn impossible_acc(
        _lit_action_code: &str,
        _actions: &Actions,
    ) -> Vec<UnifiedAccessControlConditionItem> {
        vec![
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
        ]
    }

    fn ipfs_acc(
        lit_action_code: &str,
        _actions: &Actions,
    ) -> Vec<UnifiedAccessControlConditionItem> {
        let ipfs_hasher = IpfsHasher::default();
        let cid = ipfs_hasher.compute(lit_action_code.as_bytes());
        let derived_ipfs_id = cid;

        // create a condition that checks if the current IPFS ID is equal to the derived IPFS ID
        vec![UnifiedAccessControlConditionItem::Condition(
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
        )]
    }

    fn evm_acc(
        _lit_action_code: &str,
        actions: &Actions,
    ) -> Vec<UnifiedAccessControlConditionItem> {
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
        vec![UnifiedAccessControlConditionItem::Condition(
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
        )]
    }

    pub async fn get_encryption_decryption_test_params(
        wallet: Wallet<SigningKey>,
        actions: &Actions,
        to_encrypt: &str,
        lit_actions_code: &str,
        fn_accs: &dyn Fn(&str, &Actions) -> Vec<UnifiedAccessControlConditionItem>,
    ) -> (
        Option<Vec<ControlConditionItem<UnifiedAccessControlCondition>>>,
        String,
        String,
        JsonAuthSig,
    ) {
        let unified_access_control_conditions = Some(fn_accs(lit_actions_code, actions));
        let mut hasher = Sha256::new();
        hasher.update(to_encrypt.as_bytes());
        let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

        // Get auth sig for auth
        // let wallet = LocalWallet::new(&mut OsRng);
        let auth_sig = generate_authsig(&wallet)
            .await
            .expect("Couldn't generate auth sig");

        // Encrypt.
        let network_pubkey = get_network_pubkey(actions).await;
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

    #[tokio::test]
    #[ignore]
    async fn test_pkp_is_permissions_is_permitted_action() {
        init_test_config();
        let num_nodes = 3;

        let (_testnet, validator_collection) = new_node_collection(num_nodes, false).await;
        const IPFS_CID: &str = "QmZuSixiCCkttPDvHKZdAw2gNby111rNEq1aHTozkqztSg";

        let res = generate_pkp_check_is_permitted_pkp_action(IPFS_CID, &validator_collection).await;

        //res
        assert!(res.is_ok());
        assert!(res.unwrap());
    }
}
