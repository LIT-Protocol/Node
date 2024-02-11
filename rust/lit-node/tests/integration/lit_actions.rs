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

#[cfg(feature = "lit-actions")]
pub mod litactions {
    use crate::common;
    use crate::common::auth_sig::generate_authsig_item;
    use crate::common::lit_actions::{assert_lit_action, execute_lit_action};
    use common::lit_actions::{sign_from_file_system, sign_hello_world};
    use common::new_node_collection;
    use ethers::utils::keccak256;
    use serde_json::Value;
    use tracing::info;

    #[tokio::test]
    pub async fn lit_action_happy_path() {
        common::init_test_config();
        info!("Starting test: lit_action_happy_path");
        let num_nodes = 3;
        let (node_collection, scenario) =
            new_node_collection(num_nodes, num_nodes, None, true, false).await;

        let validation = sign_hello_world(&node_collection, &scenario).await.unwrap();
        drop(node_collection);
        assert!(validation);
    }

    #[tokio::test]
    #[ignore]
    pub async fn lit_action_from_file() {
        common::init_test_config();
        info!("Starting test: lit_action_from_file");
        let num_nodes = 3;
        let (node_collection, scenario) =
            new_node_collection(num_nodes, num_nodes, None, true, false).await;
        let file_name = "./tests/sample_lit_action.js";
        let validation = sign_from_file_system(&node_collection, &scenario, file_name)
            .await
            .unwrap();
        drop(node_collection);
        assert!(validation);
    }

    #[tokio::test]
    async fn test_pkp_permissions_is_cid_registered_and_can_it_sign() {
        common::init_test_config();
        let num_staked = 3;
        let num_nodes = 3;

        let (nc, s) = new_node_collection(num_staked, num_nodes, None, true, false).await;
        const IPFS_CID: &str = "QmRwN9GKHvCn4Vk7biqtr6adjXMs7PzzYPCzNCRjPFiDjm";

        let res =
            common::lit_actions::generate_pkp_check_get_permitted_pkp_action(IPFS_CID, &nc, &s)
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
        let wallet = s.testnet.node_accounts[2].signing_provider.signer();
        let auth_sig = generate_authsig_item(wallet)
            .await
            .expect("Could not generate auth sig");
        let res = execute_lit_action(
            &nc,
            None,
            Some(IPFS_CID.to_string()),
            Some(serde_json::Value::Object(js_params)),
            None,
            auth_sig,
        )
        .await;
        assert!(res.is_ok());
        let res = res.unwrap();

        let asserted = assert_lit_action(&nc, res).await;
        assert!(asserted.is_ok());
        let asserted = asserted.unwrap();
        assert!(asserted);

        // test that the error when trying to sign something smaller than 32 bytes works
        let mut js_params = serde_json::Map::new();
        js_params.insert("publicKey".to_string(), pkp_pubkey.into());
        js_params.insert("sigName".to_string(), "sig1".into());
        js_params.insert("toSign".to_string(), serde_json::json!([1, 2, 3, 4]));
        let wallet = s.testnet.node_accounts[2].signing_provider.signer();
        let auth_sig = generate_authsig_item(wallet)
            .await
            .expect("Could not generate auth sig");
        let res = execute_lit_action(
            &nc,
            None,
            Some(IPFS_CID.to_string()),
            Some(serde_json::Value::Object(js_params)),
            None,
            auth_sig,
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
        // info!("json_responses: {:?}", json_responses);

        drop(nc);
    }

    #[tokio::test]
    #[ignore]
    async fn test_pkp_is_permissions_is_permitted_action() {
        common::init_test_config();
        let num_staked = 3;
        let num_nodes = 3;

        let (nc, s) = new_node_collection(num_staked, num_nodes, None, true, false).await;
        const IPFS_CID: &str = "QmZuSixiCCkttPDvHKZdAw2gNby111rNEq1aHTozkqztSg";

        let res =
            common::lit_actions::generate_pkp_check_is_permitted_pkp_action(IPFS_CID, &nc, &s)
                .await;

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res);
        info!("is permitted action result: {:?}", res);

        drop(nc);
    }
}
