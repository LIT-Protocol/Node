use core::panic;
use super::utils::virtual_node_collection::new_virtual_node_collection;
use crate::common::{self, interpolation};
use crate::component::dkg::dkg;
use blsful::Bls12381G2Impl;
use lit_core::utils::binary::{bytes_to_hex, hex_to_bytes};
use lit_node::tss::common::key_type::KeyType;
use lit_node::utils::consensus::get_threshold_count;
use tracing::info;

#[tokio::test]
#[doc = "Test that signs a Session Key using a set of virtual nodes."]
pub async fn sign_with_pubkey() {
    common::init_test_config();
    info!("Starting test: ecdsa_dkg");
    let num_nodes = 3;
    let threshold = get_threshold_count(num_nodes);
    let (vnc, _scenario) = new_virtual_node_collection(num_nodes).await;

    let message_to_sign = "Hello LIT Network!".to_string();
    let message_bytes = message_to_sign.clone().into_bytes();
    let pubkey = dkg(&vnc, KeyType::BLS, None, None, None).await;
    let public_key = hex_to_bytes(&pubkey).unwrap();
    let mut signature_shares = Vec::new();

    info!("Signing message '{}' with each node's secret key share.", message_to_sign);

    for  node in vnc.nodes {
        let cipher_state = match node.tss_state.get_cipher_state(KeyType::BLS) {
            Ok(cipher_state) => cipher_state,
            Err(e) => {
                panic!("error from get_cipher_state: {:?}", e);
            }
        };

        // Sign the message using the blsful secret key share.
        let (signature_share, _share_index) = match cipher_state.sign_with_pubkey(&message_bytes.clone(), public_key.clone()).await {
            Ok(signature_share) => signature_share,
            Err(e) => {
                panic!("error from sign: {:?}", e);
            }
        };

     
        signature_shares.push(signature_share);
        

    }
    let sig = blsful::Signature::from_shares(&signature_shares);
    assert!(sig.is_ok());
    
    let sig = sig.unwrap();


    // let all_staker_addresses = vnc.nodes.iter().map(|node| (bytes_to_hex(&node.staker_address.as_bytes()), true)).collect::<Vec<(String, bool)>>();
    // let secret = interpolation::interpolate_bls_secret(&all_staker_addresses, threshold, &pubkey).await;
    
    // let pk = blsful::PublicKey::from_shares(&secret);
    // sig.verify(pk, message_bytes);

}
