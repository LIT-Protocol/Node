use super::utils::virtual_node_collection::new_virtual_node_collection;
use crate::common;

use lit_node::tss::common::key_type::KeyType;

use std::time::Instant;
use tracing::info;
pub struct TestRun {
    pub network_size: u32,
    pub bls_dkg: u128,
    pub ecdsa_dkg_cs: u128,
}

#[tokio::test]
#[ignore]
#[doc = "Run a series of benchmarks to compare the time taken to run DKG with BLS and ECDSA curves - the DKG currently uses the Gennaro DKG implementation for both curves"]
async fn dkg_benchmarks() {
    common::init_test_config();

    let test_sizes: Vec<usize> = vec![3, 5, 7, 10];
    let mut test_runs: Vec<TestRun> = Vec::new();
    let epoch = 1;

    for test_size in test_sizes {
        info!("Starting test with network size {}", test_size);
        let (vnc, _scenario) = new_virtual_node_collection(test_size).await;
        let start = Instant::now();
        super::dkg::dkg(&vnc, KeyType::EcdsaCaitSith, epoch, None, None, None).await;
        let ecdsa_dkg_cs = start.elapsed().as_millis();

        let start = Instant::now();
        let bls_dkg = start.elapsed().as_millis();

        let test_run = TestRun {
            network_size: test_size as u32,
            bls_dkg,
            ecdsa_dkg_cs,
        };

        test_runs.push(test_run);
    }

    println!("\n\nTest Complete. Benchmark Results:\n");
    for test_run in test_runs {
        println!(
            "Test run with network size {} took {:.2}s for BLS DKG, and {:.2}s for CS ECDSA DKG",
            test_run.network_size,
            test_run.bls_dkg as f64 / 1000.0,
            test_run.ecdsa_dkg_cs as f64 / 1000.0
        );
    }
}
