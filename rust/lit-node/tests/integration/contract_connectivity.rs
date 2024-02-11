use crate::common;
use common::config::load_config;
use common::init_test_config;
use ethers::prelude::*;

use tracing::info;

#[tokio::test]
#[ignore]
pub async fn test_pubkey_router_connectivity() {
    init_test_config();

    let (cfg, resolver) = load_config();
    let contract = resolver
        .pub_key_router_contract_with_signer(&cfg)
        .await
        .unwrap();
    let owner = contract.address();
    info!("Pubkey Router address: {:?}", owner);
}

#[tokio::test]
#[ignore]
pub async fn test_pkp_permissions_connectivity() {
    init_test_config();

    let (cfg, resolver) = load_config();
    let contract = resolver
        .pkp_permissions_contract_with_signer(&cfg)
        .await
        .unwrap();
    let owner = contract.owner().call().await.unwrap();
    info!("Pubkey Router owner: {:?}", owner);
}

#[tokio::test]
#[ignore]
pub async fn lit_token_connectivity() {
    init_test_config();

    let (cfg, resolver) = load_config();
    let contract = resolver.lit_token_contract_with_signer(&cfg).await.unwrap();
    let result = contract.symbol().call().await.unwrap();
    info!("LitToken - symbol: {:?}", result);
}

#[tokio::test]
#[ignore]
pub async fn pkpnft_connectivity() {
    init_test_config();

    let (cfg, resolver) = load_config();
    let contract = resolver.pkp_nft_contract_with_signer(&cfg).await.unwrap();
    let result = contract.name().call().await.unwrap();
    info!("PKPNFT - Name: {:?}", result);
}

#[tokio::test]
#[ignore]
pub async fn rate_limit_connectivity() {
    init_test_config();

    let (cfg, resolver) = load_config();
    let contract = resolver
        .rate_limit_nft_contract_with_signer(&cfg)
        .await
        .unwrap();
    let token_id = U256::zero();
    let result = contract.is_expired(token_id).call().await.unwrap();
    info!("RateLimit - is token_id 0x0 expired: {:?}", result);
}

#[tokio::test]
#[ignore]
pub async fn staking_connectivity() {
    init_test_config();

    let (cfg, resolver) = load_config();
    let contract = resolver.staking_contract_with_signer(&cfg).await.unwrap();
    let address = H160::zero();
    let result = contract.is_active_validator(address).call().await.unwrap();
    info!("Staking - epoch: {:?}", result);
}
