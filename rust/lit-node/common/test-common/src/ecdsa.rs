use crate::node_collection::hit_ports_with_json_body_join_all;
use crate::pkp::{generate_data_to_send, sign_message_with_pkp};
use crate::testnet::actions::Actions;
use crate::testnet::Testnet;
use crate::validator::ValidatorCollection;
use crate::{
    new_node_collection,
    pkp::{add_permitted_address_to_pkp, mint_next_pkp},
};
use ethers::{types::U256, utils::keccak256};
use futures::future::join_all;
use lit_core::utils::binary::bytes_to_hex;
use tracing::{info, warn};

pub async fn simple_single_sign_with_hd_key(
    portnames: &Vec<String>,
    actions: &Actions,
    pubkey: &str,
) -> bool {
    sign_with_hd_key(
        portnames,
        actions,
        pubkey.to_string(),
        false,
        false,
        1,
        None,
        true,
    )
    .await
}

pub async fn new_node_collection_with_authd_pkp(
    num_nodes: usize,
    is_fault_test: bool,
) -> (Testnet, ValidatorCollection, String) {
    let (testnet, validator_collection, pubkey, _token_id) =
        new_node_collection_with_authd_pkp_with_token(num_nodes, is_fault_test).await;
    (testnet, validator_collection, pubkey)
}

pub async fn new_node_collection_with_authd_pkp_with_token(
    num_nodes: usize,
    is_fault_test: bool,
) -> (Testnet, ValidatorCollection, String, U256) {
    let (testnet, validator_collection) = new_node_collection(num_nodes, is_fault_test).await;

    let minted_key = mint_next_pkp(validator_collection.actions()).await;
    if let Err(e) = minted_key {
        panic!("Failed to mint key: {:?}", e);
    }
    assert!(minted_key.is_ok());
    let minted_key = minted_key.unwrap();
    let pubkey = minted_key.0;
    let token_id = minted_key.1;

    let address_bytes = testnet.node_accounts[0].staker_address.as_bytes();
    let address = &bytes_to_hex(address_bytes);

    let res = add_permitted_address_to_pkp(
        validator_collection.actions(),
        address,
        token_id,
        &[U256::from(1)],
    )
    .await;

    assert!(res.is_ok());
    (testnet, validator_collection, pubkey, token_id)
}

#[doc = "Mint a new key and sign with it.  This is a helper function for the test_pkp_hd_sign_generic_key test."]
pub async fn sign_with_hd_key(
    portnames: &Vec<String>,
    actions: &Actions,
    pubkey: String,
    concurrent_signing: bool,
    concurrent_randomization: bool,
    messages_to_sign: i32,
    message_to_sign: Option<String>,
    assert_inline: bool,
) -> bool {
    let mut validation = false;
    let mut future_validations = Vec::new();
    let expected_responses = portnames.len();
    let max_sleep_ms = 100; // a number between 1 and size of random number generator (currently a u8) ... creates concurrency when the rnd is above this value
    for i in 0..messages_to_sign {
        let to_sign = match message_to_sign.clone() {
            Some(m) => m,
            None => format!("test message #{}", i),
        };

        info!("Testing message #{}: {:?}", i, to_sign);

        if concurrent_signing {
            let data_to_send = generate_data_to_send(
                pubkey.clone(),
                keccak256(to_sign.as_bytes()).into(),
                actions,
            )
            .await
            .expect("Failed to generate PKP Signing Request.");
            let portnames = portnames.clone();
            let json_body = serde_json::to_string(&data_to_send).unwrap();
            let cmd = "/web/pkp/sign".to_string();

            let future_sign =
                tokio::spawn(hit_ports_with_json_body_join_all(portnames, cmd, json_body));
            future_validations.push(future_sign);
            if concurrent_randomization {
                let mut sleep_time = rand::random::<u8>() as u64;
                if sleep_time > max_sleep_ms {
                    sleep_time = 0;
                }
                actions.sleep_millis(sleep_time).await;
            }
        } else {
            validation = sign_message_with_pkp(actions, pubkey.clone(), to_sign)
                .await
                .unwrap();

            if assert_inline {
                assert!(validation);
            }
        }
    }

    if concurrent_signing {
        warn!("Waiting for concurrent signing to complete.");
        let validations = join_all(future_validations).await;
        for v in validations {
            let responses = v.unwrap();

            assert!(responses.is_ok());
            let responses = responses.unwrap();
            assert!(responses.len() == expected_responses);

            validation = true;
        }
    }

    validation
}
