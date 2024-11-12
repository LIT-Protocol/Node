use super::utils::virtual_node_collection::VirtualNodeCollection;
use futures::future::join_all;
use lit_node::{
    tasks::{
        beaver_manager::{models::BeaverManager, models::BeaverTriplePair},
        utils::generate_hash,
    },
    tss::common::storage::write_beaver_triple_to_disk,
};
use tokio::task::JoinHandle;

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
#[ignore]
async fn test_generate_triple_pairs() {
    let num_nodes = 3;
    let num_pairs = 1;
    let safe_triple = true;
    generate_triple_pairs(num_nodes, num_pairs, safe_triple).await;
}

#[tokio::test]
async fn test_generate_triple() {
    test_common::init_test_config();
    let num_nodes = 3;
    let vnc = VirtualNodeCollection::new(num_nodes).await;
    let txn_prefix = "test".to_string();
    generate_beaver_triple(&vnc, txn_prefix).await;
}

pub async fn generate_triple_pairs(num_nodes: usize, num_pairs: usize, safe_triple: bool) {
    test_common::init_test_config();
    tracing::trace!("starting test: generate_triple_pairs");
    let mut vnc = VirtualNodeCollection::new(num_nodes).await;
    match safe_triple {
        true => generate_safe_triple_pairs(&vnc, num_pairs).await,
        false => generate_unsafe_triple_pairs(&vnc, num_pairs).await,
    }
    vnc.shutdown().await;
}

async fn generate_safe_triple_pairs(vnc: &VirtualNodeCollection, num_pairs: usize) {
    for _count in 0..num_pairs {
        let triple_pairs = generate_beaver_triple_pair(vnc, "test_pair".to_string()).await;
        let staker_address = "0x0";
        for (share_index, triple_pair) in triple_pairs.iter().enumerate() {
            let triple_key = BeaverManager::triple_key_from_triple_pair(&triple_pairs[0]);
            let pubkey = format!("{}", triple_key);
            let result = write_beaver_triple_to_disk(
                &pubkey,
                share_index as u16,
                staker_address,
                triple_pair,
            )
            .await;
            if result.is_err() {
                panic!("Error writing beaver triple to disk");
            }
        }
    }
}

async fn generate_unsafe_triple_pairs(vnc: &VirtualNodeCollection, num_pairs: usize) {
    let triple_pairs = generate_beaver_triple_pair(vnc, "test_pair".to_string()).await;
    let mut triple_key = BeaverManager::triple_key_from_triple_pair(&triple_pairs[0]);
    let staker_address = "0x0"; // this function is about to be deprecated
    for _count in 0..num_pairs {
        triple_key = generate_hash(triple_key);
        let pubkey = format!("{}", triple_key);
        for (share_index, triple_pair) in triple_pairs.iter().enumerate() {
            let result = write_beaver_triple_to_disk(
                &pubkey,
                share_index as u16,
                staker_address,
                triple_pair,
            )
            .await;
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

async fn generate_beaver_triple(vnc: &VirtualNodeCollection, txn_prefix: String) -> bool {
    let mut v = Vec::new();

    for node in vnc.nodes.iter() {
        let ecdsa_state = node.ecdsa_state().await;
        let txn_prefix = txn_prefix.clone();
        // let jh: JoinHandle<BeaverTriplePair> = tokio::spawn(async move {
        let jh: JoinHandle<_> = tokio::spawn(async move {
            let txn_params =
                lit_node::tss::ecdsa_cait_sith::models::ProtocolTransactionParams::from_config(
                    ecdsa_state
                        .protocol_params_with_options(&txn_prefix, None, None)
                        .await
                        .unwrap()
                        .into(),
                );

            let r = ecdsa_state.triples(&txn_params).await;
            // let r = ecdsa_state.generate_triple_pair(None, txn_prefix).await;
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

    true
}
