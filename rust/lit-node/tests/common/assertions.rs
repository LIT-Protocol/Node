use crate::{
    acceptance::web_user_tests::test_encryption_decryption_auth_sig,
    common::{
        node_collection::{get_network_pubkey, NodeCollection},
        pkp::mint_next_pkp,
        testnet::scenario::Scenario,
    },
    integration::ecdsa::sign_with_hd_key,
};
use tracing::info;

/// This checker is intended to be used for checking the integrity of the network after notable network-wide
/// events such as epoch advancements.
#[derive(Debug)]
pub struct NetworkIntegrityChecker {
    initial_bls_pubkey: String,
    minted_pkp_pubkey: String,
}

impl NetworkIntegrityChecker {
    pub async fn new(initial_scenario: &Scenario) -> Self {
        let initial_bls_pubkey = get_network_pubkey(initial_scenario).await;

        // Mint a PKP that will be used later.
        let (pubkey, token_id) = mint_next_pkp(initial_scenario)
            .await
            .expect("Failed to mint key");
        info!("Generated new PKP: {:?} / token_id: {:?}", pubkey, token_id);

        Self {
            initial_bls_pubkey,
            minted_pkp_pubkey: pubkey,
        }
    }

    /// This function runs interpolation checks and performs decryption and signing operations on the network.
    pub async fn check(&self, latest_node_collection: &NodeCollection, latest_scenario: &Scenario) {
        // Pubkey check.
        info!("Running pubkey checks");
        let latest_bls_pubkey = get_network_pubkey(latest_scenario).await;
        assert_eq!(self.initial_bls_pubkey, latest_bls_pubkey);

        // Decryption check.
        info!("Running decryption checks");
        test_encryption_decryption_auth_sig(latest_node_collection, latest_scenario).await;

        // Signing operation.
        info!("Running signing checks");
        let validation = sign_with_hd_key(
            latest_node_collection,
            latest_scenario,
            self.minted_pkp_pubkey.clone(),
            false,
            false,
            1,
            None,
            true,
        )
        .await;
        assert!(validation);

        info!("Network integrity check passed");
    }
}
