use crate::{
    ecdsa::{sign_with_hd_key, simple_single_sign_with_hd_key},
    web_user_tests::test_encryption_decryption_auth_sig,
    {node_collection::get_network_pubkey, pkp::mint_next_pkp},
};
use tracing::{debug, info};

use super::{testnet::actions::Actions, validator::ValidatorCollection};

/// This checker is intended to be used for checking the integrity of the network after notable network-wide
/// events such as epoch advancements.
#[derive(Debug)]
pub struct NetworkIntegrityChecker {
    initial_bls_pubkey: String,
    minted_pkp_pubkey: String,
}

impl NetworkIntegrityChecker {
    pub async fn new(actions: &Actions) -> Self {
        let initial_bls_pubkey = get_network_pubkey(actions).await;

        // Mint a PKP that will be used later.
        let (pubkey, token_id, _) = mint_next_pkp(actions).await.expect("Failed to mint key");
        info!("Generated new PKP: {:?} / token_id: {:?}", pubkey, token_id);

        Self {
            initial_bls_pubkey,
            minted_pkp_pubkey: pubkey,
        }
    }

    /// This function runs interpolation checks and performs decryption and signing operations on the network.
    pub async fn check(&self, validator_collection: &ValidatorCollection) {
        // Pubkey check.
        info!("Running pubkey checks");
        let latest_bls_pubkey = get_network_pubkey(validator_collection.actions()).await;
        assert_eq!(self.initial_bls_pubkey, latest_bls_pubkey);

        // Decryption check.
        info!("Running decryption checks");
        test_encryption_decryption_auth_sig(validator_collection.actions()).await;

        // Signing operation.
        info!("Running signing checks");
        assert!(
            simple_single_sign_with_hd_key(
                &validator_collection
                    .ports()
                    .iter()
                    .map(|p| p.to_string())
                    .collect(),
                validator_collection.actions(),
                &self.minted_pkp_pubkey
            )
            .await,
            "ECDSA Signing failed!"
        );

        info!("Network integrity check passed");
    }

    #[deprecated(
        note = "This should be removed once all nodes have updated to the new code that supports using BTs across boundaries."
    )]
    /// This function runs interpolation checks and performs decryption and signing operations on the network.
    /// The signing operations are only asserted against when the beaver triples are completely drained.
    /// Instead of explicitly checking the logs of each deterministic subset of nodes - which is slightly complicated -
    /// we simply retry the operation up to a maximum number of times in an attempt to drain the triples.
    pub async fn check_with_drained_beaver_triples(
        &self,
        validator_collection: &ValidatorCollection,
    ) {
        const MAX_TRIES: usize = 5;

        // Pubkey check.
        info!("Running pubkey checks");
        let latest_bls_pubkey = get_network_pubkey(validator_collection.actions()).await;
        assert_eq!(self.initial_bls_pubkey, latest_bls_pubkey);

        // Decryption check.
        info!("Running decryption checks");
        test_encryption_decryption_auth_sig(validator_collection.actions()).await;

        // Signing check.
        info!("Running signing checks");
        for idx in 0..MAX_TRIES {
            if sign_with_hd_key(
                &validator_collection
                    .ports()
                    .iter()
                    .map(|p| p.to_string())
                    .collect(),
                validator_collection.actions(),
                self.minted_pkp_pubkey.clone(),
                false,
                false,
                1,
                None,
                false, // No need to assert the validity of these signing operations - we just want to drain the beaver triples.
            )
            .await
            {
                break;
            }
            debug!("Failed {:?} try to sign with HD key (possibly due to bad, uncleared triples being used) - retrying...", idx + 1);
        }

        info!("Network integrity check passed");
    }
}
