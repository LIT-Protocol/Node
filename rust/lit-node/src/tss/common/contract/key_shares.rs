use crate::error::{generic_err_code, unexpected_err, EC};
use ethers::prelude::*;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use lit_core::error::Result;
use lit_core::utils::binary::bytes_to_hex;
use std::sync::Arc;

pub async fn get_first_key(cfg: &LitConfig) -> Result<String> {
    let resolver =
        Arc::new(ContractResolver::try_from(cfg).expect("failed to load ContractResolver"));

    let pkpnft = resolver.pkp_nft_contract(cfg).await?;
    let total_supply = pkpnft.total_supply().call().await;
    let total_supply = total_supply.unwrap_or(U256::from(0));

    if total_supply == U256::zero() {
        return Err(generic_err_code(
            "No keys in the system, please generate and route a key first".to_string(),
            EC::NodeNoKeyGenError,
            None,
        ));
    }

    let token_id = pkpnft
        .token_by_index(U256::zero())
        .call()
        .await
        .unwrap_or(U256::zero());

    if total_supply == U256::zero() {
        return Err(generic_err_code(
            "The first key the system is empty, please generate and route a key first".to_string(),
            EC::NodeNoKeyGenError,
            None,
        ));
    }

    let pubkey = pkpnft
        .get_pubkey(token_id)
        .call()
        .await
        .map_err(|e| unexpected_err(e, Some("Failed to get pubkey for token_id".into())))?;

    let public_key = bytes_to_hex(&pubkey);

    Ok(public_key)
}
