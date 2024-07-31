use ethers::{
    providers::{Http, Provider},
    types::{Bytes, U256},
};
use futures::TryFutureExt;
use lit_blockchain::contracts::pkp_permissions::PKPPermissions;

use crate::error::{unexpected_err_code, Result, EC};

// Retrieve the PKP pubkey using the auth method ID. Right now we only use the first token ID returned.
pub async fn get_pkp_pubkey(
    pkp_permissions_contract: &PKPPermissions<Provider<Http>>,
    auth_method_type: u32,
    auth_method_id: Bytes,
) -> Result<Bytes> {
    // Get PKP token ID for auth method.
    let query = pkp_permissions_contract
        .get_token_ids_for_auth_method(U256::from(auth_method_type), auth_method_id.clone());
    let token_ids_for_auth_method = query
        .call()
        .map_err(|e| {
            unexpected_err_code(e, EC::NodeRpcError, Some("Failed to query contract".into()))
        })
        .await?;
    let pkp_token_id = token_ids_for_auth_method.first();
    debug!("Retrieved token IDs {:?}", token_ids_for_auth_method);

    let pkp_token_id = match pkp_token_id {
        Some(pkp_token_id) => pkp_token_id,
        None => {
            return Err(unexpected_err_code(
                format!(
                    "Could not find PKP token IDs for auth method id {}",
                    auth_method_id.clone()
                ),
                EC::NodePKPTokenIdNotFound,
                None,
            ));
        }
    };

    // Get PKP public key.
    let query = pkp_permissions_contract.get_pubkey(pkp_token_id.to_owned());
    let pkp_public_key = query
        .call()
        .map_err(|e| {
            unexpected_err_code(e, EC::NodeRpcError, Some("Failed to query contract".into()))
        })
        .await?;

    Ok(pkp_public_key)
}
