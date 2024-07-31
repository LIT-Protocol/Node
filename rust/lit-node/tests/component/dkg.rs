use super::utils::virtual_node_collection::VirtualNodeCollection;
use crate::common::{
    self,
    interpolation::{get_secret_and_shares, interpolate_secret},
};
use futures::future::join_all;
use lit_core::utils::binary::bytes_to_hex;
use lit_node::peers::peer_state::models::{SimplePeer, SimplePeerExt};
use lit_node::tss::common::{curve_type::CurveType, dkg_type::DkgType};
use test_case::test_case;
use tokio::task::JoinHandle;
use tracing::{error, info};

#[derive(Debug, Clone, PartialEq)]
enum DkgMode {
    Keygen,
    Reshare,
    Refresh,
}

// The following tests show how compoments can be tested in isolation.
#[test_case(CurveType::K256; "ECDSA Key generation")]
#[test_case(CurveType::BLS; "BLS Key generation")]
#[test_case(CurveType::Ed25519; "Ed25519 Key generation")]
#[test_case(CurveType::Ristretto25519; "Ristretto25519 Key generation")]
#[test_case(CurveType::Ed448; "Ed448 Key generation")]
#[test_case(CurveType::P256; "P256 Key generation")]
#[test_case(CurveType::P384; "P384 Key generation")]
#[test_case(CurveType::RedJubjub; "RedJubjub Key generation")]
#[tokio::test]
#[doc = "Test that a DKG can be run on a set of virtual nodes."]
pub async fn dkg_only(curve_type: CurveType) {
    common::init_test_config();
    initial_dkg(curve_type, 3).await;
}

#[test_case(CurveType::K256; "ECDSA Refresh")]
#[test_case(CurveType::BLS; "BLS Refresh")]
#[test_case(CurveType::Ed25519; "Ed25519 Refresh")]
#[test_case(CurveType::Ristretto25519; "Ristretto25519 Refresh")]
#[test_case(CurveType::Ed448; "Ed448 Refresh")]
#[test_case(CurveType::P256; "P256 Refresh")]
#[test_case(CurveType::P384; "P384 Refresh")]
#[test_case(CurveType::RedJubjub; "RedJubjub Refresh")]
#[tokio::test]
pub async fn dkg_and_refresh(curve_type: CurveType) {
    common::init_test_config();
    let num_nodes = 3;
    // initial setup
    let (mut vnc, pubkey, epoch, _current_peers) = initial_dkg(curve_type, num_nodes).await;
    // do a refresh
    let (next_epoch, _current_peers) = refresh_dkg(curve_type, &vnc, &pubkey, epoch).await;
    assert!(epoch + 1 == next_epoch);
    vnc.shutdown().await;
}

// For a reshare the public key resolves to the original public key
#[test_case( CurveType::K256, 3, [1,0].to_vec() ; "ECDSA add node, keep threshold")]
#[test_case( CurveType::BLS, 3, [1, 0].to_vec() ; "BLS add node, keep threshold")]
#[test_case(CurveType::Ed25519, 3, [1, 0].to_vec(); "Ed25519 add node, keep threshold")]
#[test_case(CurveType::Ristretto25519, 3, [1, 0].to_vec(); "Ristretto25519 add node, keep threshold")]
#[test_case(CurveType::Ed448, 3, [1, 0].to_vec(); "Ed448 add node, keep threshold")]
#[test_case(CurveType::P256, 3, [1, 0].to_vec(); "P256 add node, keep threshold")]
#[test_case(CurveType::P384, 3, [1, 0].to_vec(); "P384 add node, keep threshold")]
#[test_case(CurveType::RedJubjub, 3, [1, 0].to_vec(); "RedJubjub add node, keep threshold")]
// #[test_case( CurveType::K256, 4, [-2,0].to_vec() ; "ECDSA remove node, keep threshold")]
// #[test_case( CurveType::BLS, 4, [-2,0].to_vec() ; "BLS remove node, keep threshold")]
// #[test_case( CurveType::K256, 4, [-1,0].to_vec() ; "ECDSA first node, keep threshold")]
// #[test_case( CurveType::K256, 4, [1,0].to_vec() ; "ECDSA add node, change threshold")]
#[test_case( CurveType::K256, 5, [-1,0].to_vec() ; "ECDSA remove node, change threshold")]
#[test_case( CurveType::BLS, 5, [-1,0].to_vec() ; "BLS remove node, change threshold")]
// #[test_case( CurveType::K256, 3, [1,2,0].to_vec() ; "ECDSA add two nodes")]
// #[test_case( CurveType::K256, 6, [-2,-2,0].to_vec() ; "ECDSA remove two nodes")]
// #[test_case( CurveType::K256, 6, [4,4,-2,-2,0].to_vec() ; "ECDSA add two nodes and remove two nodes")]
// #[test_case( CurveType::K256, 5, [1,2,1,2,1,2,1,2,1,2,0].to_vec() ; "ECDSA add 10 nodes to 5 node network")]
// #[test_case( CurveType::K256, 10, [-1,-1,-5,0,-4,0,-3,0,-1,0,3,0,-1,5,5,0].to_vec() ; "ECDSA reshare batch: 10 r3 a1 a1 a1 a1 r3")]
// For a reshare the public key resolves to the original public key
// #[test_case( CurveType::K256, 5, [1,1,0,-4,0,-3,0,-1,0,4,3,0,1,5,5,0].to_vec() ; "ECDSA reshare batch: 5 a2 r1 r1 r1 a2 a3")]
#[tokio::test]
pub async fn dkg_and_reshare(curve_type: CurveType, num_nodes: usize, node_changes: Vec<i8>) {
    common::init_test_config();
    struct EpochHistory {
        epoch: u64,
        threshold: u16,
        peers: Vec<SimplePeer>,
    }

    let mut epoch_history: Vec<EpochHistory> = vec![];

    // initial setup
    let (mut vnc, pubkey, epoch, current_peers) = initial_dkg(curve_type, num_nodes).await;

    let history = EpochHistory {
        epoch: epoch,
        threshold: current_peers.threshold_for_set(),
        peers: current_peers.clone(),
    };
    epoch_history.push(history);

    for node_change in node_changes {
        let index = if node_change.abs() as usize > vnc.nodes.len() {
            vnc.nodes.len()
        } else {
            node_change.abs() as usize
        };

        if node_change > 0 {
            let _added = vnc.insert_node(index).await;
        } else if node_change < 0 {
            let _removed = vnc.remove_node(index).await;
        }

        if node_change == 0 {
            let existing_peers = epoch_history.iter().last().unwrap().peers.clone();
            let epoch = epoch_history.iter().last().unwrap().epoch;

            let (next_epoch, current_peers) =
                reshare_dkg(curve_type, &vnc, existing_peers, &pubkey, epoch).await;

            // save it!
            let history = EpochHistory {
                epoch: next_epoch,
                threshold: current_peers.threshold_for_set(),
                peers: current_peers.clone(),
            };
            epoch_history.push(history);
            assert!(epoch + 1 == next_epoch);
        }
    }

    for history in epoch_history {
        info!(
            "Epoch: {}, Threshold: {}, Peers [{}]: {:?}",
            history.epoch,
            history.threshold,
            history.peers.len(),
            history.peers.debug_addresses()
        );
    }

    vnc.shutdown().await;
}

pub async fn initial_dkg(
    curve_type: CurveType,
    num_nodes: usize,
) -> (VirtualNodeCollection, String, u64, Vec<SimplePeer>) {
    common::init_test_config();
    info!("Starting dkg test {}", curve_type.to_string());
    let epoch = 1;

    let vnc = VirtualNodeCollection::new(num_nodes).await;

    let peers = vnc.peers();

    let current_peers = vec![];
    let pubkey = dkg(&vnc, curve_type, epoch, None, current_peers).await;
    let epoch = epoch + 1;

    info!("Generated {} pubkey: {:?}", curve_type.to_string(), pubkey);
    let initial_secret = interpolate_secret(curve_type, &peers, &pubkey, epoch).await;
    let peers = vnc.peers();

    info!(
        "Initial interpolated secret: {:?}",
        bytes_to_hex(&initial_secret)
    );

    (vnc, pubkey, epoch, peers)
}

pub async fn refresh_dkg(
    curve_type: CurveType,
    vnc: &VirtualNodeCollection,
    pubkey: &String,
    epoch: u64,
) -> (u64, Vec<SimplePeer>) {
    let peers = vnc.peers();
    // get the secret & shares before doing the DKG (these keyshares aren't deleted, but for good form!)
    let (initial_secret, initial_shares) =
        get_secret_and_shares(curve_type, &pubkey, &peers, epoch).await;

    // the current/next peers are the same, which causes a refresh
    let current_peers = peers.clone();
    let _result = dkg(&vnc, curve_type, epoch, Some(pubkey.clone()), current_peers).await;
    let epoch = epoch + 1;

    let (refresh_secret, refresh_shares) =
        get_secret_and_shares(curve_type, &pubkey, &peers, epoch).await;

    let msg = format!(
        "Interpolated Secret (pre/post): {:?} / {:?}",
        bytes_to_hex(&initial_secret),
        bytes_to_hex(&refresh_secret)
    );
    match initial_secret == refresh_secret {
        true => info!("{}", msg),
        false => error!("{}", msg),
    }
    assert!(initial_secret == refresh_secret);

    assert!(initial_shares
        .iter()
        .zip(refresh_shares)
        .all(|(a, b)| a != &b));

    let peers = vnc.peers();
    (epoch, peers)
}

pub async fn reshare_dkg(
    curve_type: CurveType,
    vnc: &VirtualNodeCollection,
    current_peers: Vec<SimplePeer>,
    pubkey: &String,
    epoch: u64,
) -> (u64, Vec<SimplePeer>) {
    // get the SimplePe before doing the DKG (these keyshares aren't deleted, but for good form!)
    let (initial_secret, _initial_shares) =
        get_secret_and_shares(curve_type, &pubkey, &current_peers, epoch).await;

    // the current/next peers are the same, which causes a refresh
    let _result = dkg(&vnc, curve_type, epoch, Some(pubkey.clone()), current_peers).await;
    let epoch = epoch + 1;

    let (reshare_secret, _reshare_shares) =
        get_secret_and_shares(curve_type, &pubkey, &vnc.peers(), epoch).await;

    let msg = format!(
        "Interpolated Secret (pre/post): {:?} / {:?}",
        bytes_to_hex(&initial_secret),
        bytes_to_hex(&reshare_secret)
    );
    match initial_secret == reshare_secret {
        true => info!("{}", msg),
        false => error!("{}", msg),
    }
    assert!(initial_secret == reshare_secret);

    // assert!(initial_shares
    //     .iter()
    //     .zip(refresh_shares)
    //     .all(|(a, b)| a != &b));

    let peers = vnc.peers();
    (epoch, peers)
}

pub async fn dkg(
    vnc: &VirtualNodeCollection,
    curve_type: CurveType,
    epoch: u64,
    pubkey: Option<String>,
    current_peers: Vec<SimplePeer>,
) -> String {
    // setup virtual nodes
    let next_peers = vnc.peers();
    let dkg_type = match current_peers.len() == 0 {
        true => DkgMode::Keygen,
        false => match current_peers == next_peers {
            true => DkgMode::Refresh,
            false => DkgMode::Reshare,
        },
    };

    info!("Starting {:?} on virtual nodes. ", dkg_type,);
    info!("DKG Included peers: {:?}", next_peers.debug_addresses());

    let mut v = Vec::new();
    let pubkey = pubkey.unwrap_or("".to_string());
    let current_epoch = epoch;
    let dkg_id = "TEST_DKG_1_1.KEYTYPE";
    // start keygen on all nodes
    for node in vnc.nodes.iter() {
        // this is a representation of what happens - but is not exhaustive

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let dkg_state = node
            .tss_state
            .get_dkg_state(curve_type)
            .expect("failed to get dkg state");

        let pubkey = pubkey.clone();
        let dkg_type = dkg_type.clone();
        let current_peers = current_peers.clone();
        let next_peers = next_peers.clone();

        let jh: JoinHandle<String> = tokio::task::spawn(async move {
            let r = match dkg_type {
                DkgMode::Keygen => {
                    let r = dkg_state
                        .keygen(dkg_id, epoch, &next_peers, DkgType::Standard)
                        .await;
                    info!("keygen result: {:?}", r);
                    r.expect("error from dkg state keygen")
                }
                DkgMode::Reshare => {
                    let r = dkg_state
                        .reshare(&pubkey, dkg_id, current_epoch, &current_peers, &next_peers)
                        .await;
                    info!("reshare result: {:?}", r);
                    r.expect("error from dkg state reshare").to_string()
                }
                DkgMode::Refresh => {
                    let r = dkg_state
                        .refresh(&pubkey, dkg_id, current_epoch, &next_peers)
                        .await;
                    info!("refresh result: {:?}", r);
                    r.expect("error from dkg state refresh").to_string()
                }
            };
            r
        });
        v.push(jh);
    }

    // wait for all nodes to complete
    let results = join_all(v).await;
    for result in &results {
        assert!(result.is_ok()); // weak test, but better than nothing!
    }

    // Public keys should be the same for all the nodes
    for index in 0..(results.len() - 1) {
        assert_eq!(
            results[index].as_ref().unwrap(),
            results[index + 1].as_ref().unwrap()
        );
    }

    info!(
        "Finished DKG for {:?} nodes with key type {:?} with results : {:?}",
        next_peers.len(),
        curve_type,
        results
    );

    // return the pubkey value generated, or boolean as string for refresh.
    results[0]
        .as_ref()
        .expect("first result is not ok")
        .to_string()
}
