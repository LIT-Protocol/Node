use crate::component::dkg::reshare_dkg;
use crate::component::{dkg::dkg, utils::virtual_node_collection::VirtualNodeCollection};
use cait_sith::protocol::Participant;
use ethers::utils::keccak256;
use futures::future::join_all;
use lit_node::peers::peer_state::models::SimplePeerExt;
use lit_node::peers::utils::derministic_subset::DeterministicSubset;
use lit_node::tss::common::dkg_type::DkgType;
use lit_node::tss::ecdsa_cait_sith::models::ProtocolTransactionParams;
use lit_node::utils::consensus::get_threshold_count;
use lit_node::{
    tasks::beaver_manager::models::BeaverTriplePair,
    tss::{
        common::{curve_type::CurveType, tss_state::TssState},
        ecdsa_cait_sith::{protocols256k1::CsSigshare, CsEcdsaState},
    },
};
use test_case::test_case;
use tokio::task::{JoinError, JoinHandle};
use tracing::info;

#[tokio::test]
#[doc = "Test that signs a message using a set of virtual nodes."]
pub async fn sign_with_pubkey() {
    let node_count = 3;
    do_sign_with_pubkey(node_count, 0).await;
}

#[doc = "Test that signs a message using a set of virtual nodes, with a triple that has different index values."]
#[test_case( 5, -1 ; "Start with 5 nodes, remove first node")]
#[test_case( 3, 1 ; "Start with 3 nodes, insert first node")]
#[test_case( 5, -3 ; "Start with 5 nodes, remove 3rd node")]
#[tokio::test]
pub async fn sign_with_pubkey_and_different_indexed_triple(
    node_count: usize,
    index_to_change: i16,
) {
    do_sign_with_pubkey(node_count, index_to_change).await;
}

pub async fn do_sign_with_pubkey(num_nodes: usize, node_change: i16) {
    test_common::init_test_config();
    info!("Starting test: ecdsa_dkg");
    let epoch = 1;
    let threshold = get_threshold_count(num_nodes);
    let mut vnc = VirtualNodeCollection::new(num_nodes).await;

    vnc.update_cdm_epoch(epoch + 1).await;
    let root_pubkey1 = dkg(&vnc, CurveType::K256, epoch, None, vec![]).await;
    let root_pubkey2 = dkg(&vnc, CurveType::K256, epoch, None, vec![]).await;
    info!(
        "Generated ECDSA root_pubkey1/root_pubkey2: {:?} / {:?}",
        root_pubkey1, root_pubkey2
    );
    let epoch = epoch + 1;

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
    let triple_shares = generate_beaver_triples(&vnc, txn_prefix.clone()).await;
    info!(
        "Generated Beaver Triples in {} seconds",
        start.elapsed().as_secs()
    );

    let peers_for_triples = vnc.peers();
    info!("Peers for triples: {:?}", vnc.peers().debug_addresses());

    let triple_shares = triple_shares
        .into_iter()
        .map(|tp| tp.unwrap())
        .collect::<Vec<_>>();

    let (peers, epoch) = match node_change {
        0 => (vnc.peers(), epoch),
        _ => {
            let current_peers = vnc.peers().clone();
            info!("Current peers: {:?}", current_peers.debug_addresses());

            if node_change < 0 {
                let node_change = node_change.abs() as usize;
                vnc.remove_node(node_change).await;
            } else {
                let node_change = node_change as usize;
                vnc.insert_node(node_change).await;
            }
            let pubkey = root_pubkey1.clone();
            let (_epoch, _peers) =
                reshare_dkg(CurveType::K256, &vnc, current_peers.clone(), &pubkey, epoch).await;

            let pubkey = root_pubkey2.clone();
            let (_next_epoch, peers) =
                reshare_dkg(CurveType::K256, &vnc, current_peers, &pubkey, epoch).await;
            info!("New Current peers: {:?}", vnc.peers().debug_addresses());
            vnc.update_cdm_epoch(epoch + 1).await;
            (peers, epoch + 1)
        }
    };

    let message = keccak256("Hello world!".as_bytes());
    let key_id = "id".as_bytes();

    // if we're increasing the node size, chose a subset from the "old" set of nodes, so that we guarantee every node has a triple share
    let ds = match node_change > 0 {
        true => {
            let mut union_peers = Vec::new();
            for peer in &peers {
                if peers_for_triples.contains_address(&peer.socket_address) {
                    union_peers.push(peer.clone());
                }
            }
            DeterministicSubset::new_from_peer_sets(union_peers.clone())
        }
        false => DeterministicSubset::new_from_peer_sets(peers.clone()),
    };

    let signing_peers = ds.get_subset(&message, request_id, threshold).unwrap();

    //TEST  let signing_peers be the first two peers
    info!(
        "peers\n  all: {:?}\nSubset:{:?}",
        peers.debug_addresses(),
        signing_peers.debug_addresses()
    );
    let mut v = Vec::new();

    let hd_root_keys = vec![root_pubkey1.clone(), root_pubkey2.clone()];

    let all_signing_peers = signing_peers.clone();

    for triple_share in triple_shares {
        let peer = signing_peers
            .iter()
            .filter(|p| p.key_hash == triple_share.staker_hash)
            .last();

        if peer.is_none() {
            continue;
        }

        let peer = peer.unwrap();
        // let triple_share = triple_shares.iter().filter(|p|p.staker_hash == peer.key_hash).last().unwrap();
        let triple_share_ids = triple_share.indices_from_peers(all_signing_peers.clone());
        info!("Triple share Ids: {:?}", triple_share_ids);
        let _triple_share_ids = triple_share_ids
            .iter()
            .map(|p| Participant::from(*p as u32))
            .collect::<Vec<_>>();

        info!(
            "Key peer index {:} / staker address {:}",
            peer.share_index, peer.staker_address
        );

        let node = vnc.node_by_staker_address(peer.staker_address).unwrap();
        let cs_ecdsa_state = get_cs_ecdsa_state(node.tss_state.clone());
        let mut txn_params = ProtocolTransactionParams::from_config(
            cs_ecdsa_state
                .protocol_params_with_options(
                    txn_prefix,
                    Some(all_signing_peers.clone()),
                    Some(threshold as u16),
                )
                .await
                .unwrap()
                .into(),
        );

        txn_params.update_triple_info(&triple_share);

        let hd_root_keys = hd_root_keys.clone();
        let jh: JoinHandle<CsSigshare> = tokio::task::spawn(async move {
            let sig_share = cs_ecdsa_state
                .generate_hd_key_signature_share_from_key_id(
                    &txn_params,
                    &message,
                    key_id,
                    hd_root_keys,
                    &triple_share,
                    epoch,
                )
                .await;
            sig_share.expect("error from cs_ecdsa state sig_share")
        });
        v.push(jh);
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
        let cs_ecdsa_state = get_cs_ecdsa_state(node.tss_state.clone());
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

fn get_cs_ecdsa_state(state: TssState) -> CsEcdsaState {
    CsEcdsaState {
        state,
        dkg_type: DkgType::Standard,
    }
}
