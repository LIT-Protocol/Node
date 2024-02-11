use super::utils::virtual_node_collection::new_virtual_node_collection;
use crate::common;
use crate::component::{dkg::dkg, utils::virtual_node_collection::VirtualNodeCollection};
use ethers::utils::keccak256;
use futures::future::join_all;
use lit_node::peers::utils::derministic_subset::DeterministicSubset;
use lit_node::tss::common::key_type::DkgType;
use lit_node::tss::ecdsa_cait_sith::models::ProtocolTransactionParams;
use lit_node::utils::consensus::get_threshold_count;
use lit_node::{
    tasks::beaver_manager::models::BeaverTriplePair,
    tss::{
        common::{key_type::KeyType, tss_state::TssState},
        ecdsa_cait_sith::{protocols256k1::CsSigshare, CsEcdsaState},
    },
};
use reqwest::Client;
use tokio::task::{JoinError, JoinHandle};
use tracing::info;

#[tokio::test]
#[doc = "Test that signs a Session Key using a set of virtual nodes."]
pub async fn sign_with_pubkey() {
    common::init_test_config();
    info!("Starting test: ecdsa_dkg");
    let epoch = 1;
    let num_nodes = 6;
    let threshold = get_threshold_count(num_nodes);
    let (mut vnc, _scenario) = new_virtual_node_collection(num_nodes).await;
    vnc.set_epoch(epoch + 1).await;
    let root_pubkey1 = dkg(&vnc, KeyType::EcdsaCaitSith, epoch, None, None, None).await;
    let root_pubkey2 = dkg(&vnc, KeyType::EcdsaCaitSith, epoch, None, None, None).await;
    info!(
        "Generated ECDSA root_pubkey1/root_pubkey2: {:?} / {:?}",
        root_pubkey1, root_pubkey2
    );

    // Arguments for generate_hd_key_signature_share_from_key_id:
    let txn = "LIT1";
    let request_id = "lit_9489d2c30aa7b".as_bytes(); // Random
    let txn_prefix = &format!(
        "{}_full_{}",
        txn,
        String::from_utf8(request_id.into()).unwrap(),
    );

    // generate BTs
    info!("Generating Beaver Triples");
    let start = std::time::Instant::now();
    let shares = generate_beaver_triples(&vnc, txn_prefix.clone()).await;
    info!(
        "Generated Beaver Triples in {} seconds",
        start.elapsed().as_secs()
    );

    let message = keccak256("Hello world!".as_bytes());
    let key_id = "id".as_bytes();
    let signing_peers = DeterministicSubset::new_from_peer_sets(vnc.peers.clone())
        .get_subset(&message, request_id, threshold)
        .unwrap();

    //TEST  let signing_peers be the first two peers
    info!("peers  all/subset: /n {:?}/n{:?}", vnc.peers, signing_peers);
    let mut v = Vec::new();
    for (index, share) in shares.into_iter().enumerate() {
        if signing_peers.contains(&vnc.peers[index]) {
            let hd_root_keys = vec![root_pubkey1.clone(), root_pubkey2.clone()];
            let cs_ecdsa_state = get_cs_ecdsa_state(
                vnc.nodes[index].tss_state.clone(),
                vnc.nodes[index].tss_state.http_client.clone(),
            );
            let triple_pair = share.unwrap();

            let txn_params = ProtocolTransactionParams::from_config(
                cs_ecdsa_state
                    .protocol_params_with_options(
                        txn_prefix,
                        Some(signing_peers.clone()),
                        Some(threshold),
                    )
                    .await
                    .unwrap(),
            );

            let jh: JoinHandle<CsSigshare> = tokio::task::spawn(async move {
                let sig_share = cs_ecdsa_state
                    .generate_hd_key_signature_share_from_key_id(
                        txn_params,
                        &message,
                        key_id,
                        hd_root_keys,
                        triple_pair,
                    )
                    .await;
                sig_share.expect("error from cs_ecdsa state sig_share")
            });
            v.push(jh);
        }
    }

    // wait for all nodes to complete
    let results = join_all(v).await;
    for result in &results {
        assert!(result.is_ok());
    }

    // Public Key should be the same
    assert_eq!(
        results[0].as_ref().unwrap().public_key,
        results[1].as_ref().unwrap().public_key
    );
    // assert_eq!(
    //     results[1].as_ref().unwrap().public_key,
    //     results[2].as_ref().unwrap().public_key
    // );

    // Shares should be different
    assert_ne!(
        results[0].as_ref().unwrap().share,
        results[1].as_ref().unwrap().share
    );
    // assert_ne!(
    //     results[1].as_ref().unwrap().share,
    //     r    esults[2].as_ref().unwrap().share
    // );
}

async fn generate_beaver_triples(
    vnc: &VirtualNodeCollection,
    txn_prefix: String,
) -> Vec<std::result::Result<BeaverTriplePair, JoinError>> {
    let mut v = Vec::new();
    for node in vnc.nodes.iter() {
        let cs_ecdsa_state =
            get_cs_ecdsa_state(node.tss_state.clone(), node.tss_state.http_client.clone());
        let txn_prefix_temp = txn_prefix.clone();

        let jh: JoinHandle<BeaverTriplePair> = tokio::task::spawn(async move {
            let r = cs_ecdsa_state
                .generate_triple_pair(None, txn_prefix_temp)
                .await;
            // info!("triple generation result: {:?}", r);
            r.expect("error from cs_ecdsa state triple generation")
        });
        v.push(jh);
    }

    // wait for all nodes to complete
    let results = join_all(v).await;
    for result in &results {
        assert!(result.is_ok());
    }

    results
}

fn get_cs_ecdsa_state(state: TssState, http_client: Client) -> CsEcdsaState {
    CsEcdsaState {
        state,
        http_client,
        dkg_type: DkgType::Standard,
    }
}
