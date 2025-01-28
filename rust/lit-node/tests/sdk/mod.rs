use test_common::new_node_collection;
use tokio::fs;
use tokio::process::Command;
use tracing::info;

const SDKTESTSPATH: &str = "tests/sdk/";

#[tokio::test]
pub async fn test_sdk() {
    info!("Starting test: test_sdk");

    let path = fs::canonicalize(SDKTESTSPATH).await;

    let _pre_cleanup = Command::new("rm")
        .arg("-r")
        .arg("node_modules")
        .arg("package.json")
        .arg("yarn.lock")
        .current_dir(path.as_ref().unwrap())
        .status()
        .await
        .expect("Failed to remove existing dependency installations");

    let _yarn_install = Command::new("yarn")
        .current_dir(path.as_ref().unwrap())
        .arg("add")
        .arg("@lit-protocol/lit-node-client-nodejs@3.0.32")
        .arg("ethers")
        .arg("siwe@2.3.2")
        .status()
        .await
        .expect("Failed to install package with yarn");

    test_common::init_test_config();

    let num_nodes = 3;
    let (_testnet, _validator_collection) = new_node_collection(num_nodes, false).await;

    let test_script = Command::new("node")
        .arg("sdk_tests.mjs")
        .current_dir(path.as_ref().unwrap())
        .status()
        .await
        .expect("Failed SDK tests");

    let _cleanup = Command::new("rm")
        .arg("-r")
        .arg("node_modules")
        .arg("package.json")
        .arg("yarn.lock")
        .current_dir(path.unwrap())
        .status()
        .await
        .expect("Failed to remove dependency installations");

    assert!(test_script.success());
}
