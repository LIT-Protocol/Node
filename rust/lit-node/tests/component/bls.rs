use crate::component::{dkg::dkg, utils::virtual_node_collection::VirtualNodeCollection};
use core::panic;
use lit_core::utils::binary::hex_to_bytes;
use lit_node::tss::common::curve_type::CurveType;
use tracing::info;

#[tokio::test]
#[doc = "Test that signs a message using BLS in a set of virtual nodes."]
pub async fn sign_with_pubkey() {
    crate::common::init_test_config();
    info!("Starting test: BLS Sign with Pubkey.");
    let num_nodes = 3;
    let mut vnc = VirtualNodeCollection::new(num_nodes).await;

    let message_to_sign = "Hello LIT Network!".to_string();
    let message_bytes = message_to_sign.clone().into_bytes();
    let epoch = 1;
    let pubkey = dkg(&vnc, CurveType::BLS, epoch, None, vec![]).await;
    let epoch = 2;
    vnc.update_cdm_epoch(epoch).await;
    let mut signature_shares = Vec::new();

    info!(
        "Signing message '{}' with each node's secret key share.",
        message_to_sign
    );
    let public_key = hex_to_bytes(&pubkey).unwrap();

    for node in vnc.nodes {
        let cipher_state = match node.tss_state.get_cipher_state(CurveType::BLS) {
            Ok(cipher_state) => cipher_state,
            Err(e) => {
                panic!("error from get_cipher_state: {:?}", e);
            }
        };

        // Sign the message using the blsful secret key share.
        let (signature_share, _share_index) = match cipher_state
            .sign_with_pubkey(&message_bytes.clone(), public_key.clone(), None)
            .await
        {
            Ok(signature_share) => signature_share,
            Err(e) => {
                panic!("error from sign: {:?}", e);
            }
        };

        signature_shares.push(signature_share);
    }
    let sig = blsful::Signature::from_shares(&signature_shares);
    assert!(sig.is_ok());

    let _sig = sig.unwrap();

    // let all_staker_addresses = vnc.nodes.iter().map(|node| (bytes_to_hex(&node.staker_address.as_bytes()), true)).collect::<Vec<(String, bool)>>();
    // let secret = interpolation::interpolate_bls_secret(&all_staker_addresses, threshold, &pubkey).await;

    // let pk = blsful::PublicKey::from_shares(&secret);
    // sig.verify(pk, message_bytes);
}
