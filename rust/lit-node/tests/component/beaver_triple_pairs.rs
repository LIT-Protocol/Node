use super::utils::virtual_node_collection::{new_virtual_node_collection, VirtualNodeCollection};
use crate::common::{self};
use futures::future::join_all;
use lit_node::{
    tasks::beaver_manager::{
        models::generate_hash, models::BeaverManager, models::BeaverTriplePair,
    },
    tss::common::storage::write_beaver_triple_to_disk,
};
use tokio::task::JoinHandle;

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
#[ignore]
async fn test_generate_triple_pairs() {
    let num_nodes = 10;
    let num_pairs = 1;
    let safe_triple = true;
    generate_triple_pairs(num_nodes, num_pairs, safe_triple).await;
}

pub async fn generate_triple_pairs(num_nodes: usize, num_pairs: usize, safe_triple: bool) {
    common::init_test_config();
    let (mut vnc, _scenario) = new_virtual_node_collection(num_nodes).await;
    match safe_triple {
        true => generate_safe_triple_pairs(&vnc, num_pairs).await,
        false => generate_unsafe_triple_pairs(&vnc, num_pairs).await,
    }
    vnc.shutdown().await;
}

async fn generate_safe_triple_pairs(vnc: &VirtualNodeCollection, num_pairs: usize) {
    for _count in 0..num_pairs {
        let triple_pairs = generate_beaver_triple_pair(vnc, "test_pair".to_string()).await;
        for (share_index, triple_pair) in triple_pairs.iter().enumerate() {
            let triple_key = BeaverManager::triple_key_from_triple_pair(&triple_pairs[0]);
            let pubkey = format!("{}", triple_key);
            let result =
                write_beaver_triple_to_disk(&pubkey, share_index as u16, triple_pair).await;
            if result.is_err() {
                panic!("Error writing beaver triple to disk");
            }
        }
    }
}

async fn generate_unsafe_triple_pairs(vnc: &VirtualNodeCollection, num_pairs: usize) {
    let triple_pairs = generate_beaver_triple_pair(vnc, "test_pair".to_string()).await;
    let mut triple_key = BeaverManager::triple_key_from_triple_pair(&triple_pairs[0]);
    for _count in 0..num_pairs {
        triple_key = generate_hash(triple_key);
        let pubkey = format!("{}", triple_key);
        for (share_index, triple_pair) in triple_pairs.iter().enumerate() {
            let result =
                write_beaver_triple_to_disk(&pubkey, share_index as u16, triple_pair).await;
            if result.is_err() {
                panic!("Error writing beaver triple to disk");
            }
        }
    }
}

async fn generate_beaver_triple_pair(
    vnc: &VirtualNodeCollection,
    txn_prefix: String,
) -> Vec<BeaverTriplePair> {
    let mut v = Vec::new();

    for node in vnc.nodes.iter() {
        let ecdsa_state = node.ecdsa_state().await;

        let txn_prefix = txn_prefix.clone();

        let jh: JoinHandle<BeaverTriplePair> = tokio::spawn(async move {
            let r = ecdsa_state.generate_triple_pair(None, txn_prefix).await;
            r.expect("error from dkg state keygen")
        });
        v.push(jh);
    }

    let start = std::time::Instant::now();
    let r = join_all(v).await;

    tracing::info!("time to generate triple pairs: {:?}", start.elapsed());
    // collect all the results
    let mut triple_pairs = Vec::new();
    for result in r {
        triple_pairs.push(result.unwrap());
    }

    triple_pairs
}

// compiler reports unused so commenting out for now - but leaving in case we want to use this in the future
// async fn generate_beaver_triple(vnc: &VirtualNodeCollection, txn_prefix: String) -> bool {
//     let mut v = Vec::new();

//     for node in vnc.nodes.iter() {
//         let ecdsa_state = node.ecdsa_state().await;
//         let txn_prefix = txn_prefix.clone();
//         let txn_params = ProtocolTransactionParams::from_config(
//             ecdsa_state.protocol_params(&txn_prefix).await.unwrap(),
//         );
//         let node_id = node.node_id;
//         // Spawn a future onto the runtime
//         let jh = tokio::spawn(async move {
//             tracing::warn!("Generating triple node: {}", node_id);
//             let r = ecdsa_state.triples(txn_params).await;
//             r.expect("error from dkg state keygen")
//         });

//         v.push(jh);
//     }

//     // wait for all nodes to complete
//     let start = std::time::Instant::now();
//     let _results = join_all(v).await;
//     tracing::info!(
//         "Time to generate a triple with {} nodes: {:?}",
//         vnc.nodes.len(),
//         start.elapsed()
//     );

//     true
// }
