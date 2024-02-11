use super::utils::virtual_node_collection::{new_virtual_node_collection, VirtualNodeCollection};
use crate::common::{
    self,
    interpolation::{interpolate_secret, load_secret_shares},
};
use futures::future::join_all;
use lit_node::{
    peers::peer_state::models::{PeerSocketAddress, PeerSocketAddressVec},
    tss::common::key_type::{DkgType, KeyType},
    utils::consensus::get_threshold_count,
};
use tokio::task::JoinHandle;
use tracing::info;
//const REFRESH_PUBKEY: &str = "c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

// The following tests show how compoments can be tested in isolation.
#[tokio::test]
#[doc = "Test that a DKG can be run on a set of virtual nodes."]
pub async fn ecdsa_initial_dkg() {
    common::init_test_config();
    initial_dkg(KeyType::EcdsaCaitSith).await;
}

// For a refresh the public key resolves to 0x0  (0x40000000..... uncompressed)
#[tokio::test]
pub async fn ecdsa_initial_dkg_and_refresh_dkg() {
    common::init_test_config();
    initial_dkg_and_refresh_dkg(KeyType::EcdsaCaitSith).await;
}

// For a reshare the public key resolves to the original public key
#[tokio::test]
pub async fn ecdsa_initial_dkg_and_reshare_add_one_dkg() {
    common::init_test_config();
    initial_dkg_and_reshare_add_one_dkg(KeyType::EcdsaCaitSith).await;
}

// The following tests show how compoments can be tested in isolation.
#[tokio::test]
#[doc = "Test that a DKG can be run on a set of virtual nodes."]
pub async fn bls_initial_dkg() {
    common::init_test_config();
    initial_dkg(KeyType::BLS).await;
}

#[tokio::test]
pub async fn bls_initial_dkg_and_refresh_dkg() {
    common::init_test_config();
    initial_dkg_and_refresh_dkg(KeyType::BLS).await;
}

#[tokio::test]
pub async fn bls_initial_dkg_and_reshare_add_one_dkg() {
    common::init_test_config();
    initial_dkg_and_reshare_add_one_dkg(KeyType::BLS).await;
}

fn gen_staker_addresses(vnc: &VirtualNodeCollection) -> (Vec<String>, Vec<(String, bool)>) {
    let mut staker_addresses = Vec::new();
    let mut all_staker_addresses = Vec::new();

    for _node in vnc.nodes.iter() {
        staker_addresses.push("0".to_string());
        all_staker_addresses.push(("0".to_string(), true));
    }

    tracing::info!("staker_addresses: {:?}", staker_addresses);

    (staker_addresses, all_staker_addresses)
}

pub async fn initial_dkg(key_type: KeyType) {
    common::init_test_config();
    info!("Starting dkg test {}", key_type.to_string());
    let num_nodes = 3;
    let threshold = get_threshold_count(num_nodes);
    let epoch = 0;
    let (vnc, _scenario) = new_virtual_node_collection(num_nodes).await;
    let (_staker_addresses, all_staker_addresses) = &gen_staker_addresses(&vnc);
    let pubkey = dkg(&vnc, key_type, epoch, None, None, None).await;
    let epoch = epoch + 1;
    info!("Generated {} pubkey: {:?}", key_type.to_string(), pubkey);
    let _initial_secret =
        interpolate_secret(key_type, all_staker_addresses, threshold, &pubkey, epoch).await;

    assert!(true);
}

pub async fn initial_dkg_and_refresh_dkg(key_type: KeyType) {
    common::init_test_config();
    let num_nodes = 3;
    let epoch = 1;
    let threshold = get_threshold_count(num_nodes);
    info!("Starting test: {}", key_type.to_string());

    let (vnc, _scenario) = new_virtual_node_collection(num_nodes).await;
    let (_staker_addresses, all_staker_addresses) = &gen_staker_addresses(&vnc);
    let pubkey = dkg(&vnc, key_type, epoch, None, None, None).await;
    info!("Generated {} pubkey: {:?}", key_type.to_string(), pubkey);
    let epoch = epoch + 1;
    let initial_secret =
        interpolate_secret(key_type, all_staker_addresses, threshold, &pubkey, epoch).await;
    let initial_shares = load_secret_shares(key_type, &pubkey, all_staker_addresses, epoch).await;

    let _result = dkg(&vnc, key_type, epoch, Some(pubkey.clone()), None, None).await;
    let epoch = epoch + 1;
    let refresh_secret =
        interpolate_secret(key_type, all_staker_addresses, threshold, &pubkey, epoch).await;
    let refresh_shares = load_secret_shares(key_type, &pubkey, all_staker_addresses, epoch).await;
    info!("Initial secret: {:?}", initial_secret);
    info!("Refresh secret: {:?}", refresh_secret);
    assert!(initial_secret == refresh_secret);

    info!("Initial shares: {:?}", initial_shares);
    info!("Refresh shares: {:?}", refresh_shares);

    // Refreshing updates the shares
    assert!(initial_shares
        .iter()
        .zip(refresh_shares)
        .all(|(a, b)| a != &b));
}

pub async fn initial_dkg_and_reshare_add_one_dkg(key_type: KeyType) {
    common::init_test_config();
    info!("Starting test for {}", key_type.to_string());
    // Set up a 5 node network, but we're only going to use 4
    let epoch = 1;
    let num_nodes = 5;
    let (vnc, _scenario) = new_virtual_node_collection(num_nodes).await;
    let (_staker_addresses, mut all_staker_addresses) = gen_staker_addresses(&vnc);

    all_staker_addresses.pop(); // remove last staker address

    let threshold = get_threshold_count(num_nodes);
    info!(
        "Initial DKG with {} nodes and threshold of {}.",
        num_nodes, threshold
    );
    let first_peers = vnc
        .peers
        .iter()
        .take(num_nodes)
        .cloned()
        .collect::<Vec<PeerSocketAddress>>();
    info!("Using peers: {:?}", first_peers);
    let pubkey = dkg(&vnc, key_type, epoch, None, Some(first_peers.clone()), None).await;
    let epoch = epoch + 1;
    let initial_secret =
        interpolate_secret(key_type, &all_staker_addresses, threshold, &pubkey, epoch).await;
    info!("Generated {} pubkey: {:?}", key_type.to_string(), pubkey);
    let initial_shares = load_secret_shares(key_type, &pubkey, &all_staker_addresses, epoch).await;

    // reshare with 5 nodes
    let (_staker_addresses, all_staker_addresses) = &gen_staker_addresses(&vnc);

    let threshold = get_threshold_count(num_nodes);
    info!(
        "Reshare DKG with {} nodes and threshold of {}.",
        num_nodes, threshold
    );
    let _result = dkg(
        &vnc,
        key_type,
        epoch,
        Some(pubkey.clone()),
        Some(vnc.peers.clone()),
        Some(first_peers.clone()),
    )
    .await;
    let epoch = epoch + 1;
    let refresh_secret =
        interpolate_secret(key_type, all_staker_addresses, threshold, &pubkey, epoch).await;
    let refresh_shares = load_secret_shares(key_type, &pubkey, all_staker_addresses, epoch).await;
    info!(
        "Initial and refreshed secrets: {:?} / {:?}",
        initial_secret, refresh_secret
    );

    assert!(initial_secret == refresh_secret);

    // Refreshing updates the shares
    assert!(initial_shares
        .iter()
        .zip(refresh_shares.iter().take(initial_shares.len()))
        .all(|(a, b)| a != b));

    // .all(|(a, b)| a.index == b.index && a.private_share != b.private_share));
}

pub async fn dkg(
    vnc: &VirtualNodeCollection,
    key_type: KeyType,
    epoch: u64,
    pubkey: Option<String>,
    included_peers: Option<Vec<PeerSocketAddress>>,
    prior_peers: Option<Vec<PeerSocketAddress>>,
) -> String {
    // setup virtual nodes
    let all_peers = vnc.peers.clone();

    let included_peers = match included_peers {
        Some(peers) => peers,
        None => all_peers.clone(),
    };

    let mut v = Vec::new();

    // start keygen on all nodes
    for node in vnc.nodes.iter() {
        if !included_peers.contains_address(&node.addr) {
            continue;
        }

        let dkg_type = match pubkey.is_some() {
            true => "refresh",
            false => "keygen",
        };
        info!(
            "Starting {} on node: {}, {}",
            dkg_type, node.node_id, node.addr
        );
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let dkg_state = node
            .tss_state
            .get_dkg_state(key_type)
            .expect("failed to get dkg state");
        let peers = included_peers.clone();
        let prior_peers = prior_peers.clone();

        let pubkey = pubkey.clone();
        let jh: JoinHandle<String> = tokio::task::spawn(async move {
            if pubkey.is_none() {
                let r = dkg_state
                    .keygen("TEST_DKG_1_1.KEYTYPE", epoch, &peers, DkgType::Standard)
                    .await;
                info!("keygen result: {:?}", r);
                r.expect("error from dkg state keygen")
            } else if prior_peers.is_some() {
                info!(
                    "Doing reshare with prior peers -> next peers: {:?} -> {:?}",
                    prior_peers, peers
                );
                let prior_peers = prior_peers.expect("prior peers is None").clone();
                let r = dkg_state
                    .reshare(
                        &pubkey.expect("pubkey is None"),
                        "TEST_DKG_1_1.KEYTYPE",
                        epoch,
                        &prior_peers,
                        &peers,
                    )
                    .await;
                info!("reshare result: {:?}", r);
                r.expect("error from dkg state reshare").to_string()
            } else {
                info!("Doing refresh with peers: {:?}", peers);
                let r = dkg_state
                    .refresh(
                        &pubkey.expect("pubkey is None"),
                        "TEST_DKG_1_1.KEYTYPE",
                        epoch,
                        &peers,
                    )
                    .await;
                info!("refresh result: {:?}", r);
                r.expect("error from dkg state refresh").to_string()
            }
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
        vnc.nodes.len(),
        key_type,
        results
    );

    // return the pubkey value generated, or boolean as string for refresh.
    results[0]
        .as_ref()
        .expect("first result is not ok")
        .to_string()
}
