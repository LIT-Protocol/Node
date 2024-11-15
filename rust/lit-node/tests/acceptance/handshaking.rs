use test_common::{new_node_collection, node_collection::handshake_returns_keys};
use tracing::info;

#[tokio::test]
pub async fn test_handshaking() {
    test_common::init_test_config();
    info!("Starting test: test_handshaking");
    let num_nodes = 3;
    let (_, validator_collection) = new_node_collection(num_nodes, false).await;

    // Assert that the handshake returns keys.
    assert!(handshake_returns_keys(validator_collection.actions()).await);
}
