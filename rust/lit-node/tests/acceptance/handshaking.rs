use crate::common::{self, node_collection::handshake_returns_keys};
use common::new_node_collection;
use tracing::info;

#[tokio::test]
pub async fn test_handshaking() {
    common::init_test_config();
    info!("Starting test: test_handshaking");
    let num_nodes = 3;
    let (_node_collection, scenario) =
        new_node_collection(num_nodes, num_nodes, None, true, false).await;

    // Assert that the handshake returns keys.
    assert!(handshake_returns_keys(&scenario).await);
}
