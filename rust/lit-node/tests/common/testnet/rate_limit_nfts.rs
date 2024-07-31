use std::{sync::Arc, time::Duration};

use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider, ProviderError},
    signers::{Signer, Wallet},
    types::{H160, U256},
};
use k256::ecdsa::SigningKey;
use lit_blockchain::contracts::{
    payment_delegation::{PaymentDelegation, PaymentDelegationErrors, Restriction},
    rate_limit_nft::{RateLimitNFT, RateLimitNFTErrors},
};
use lit_core::utils::binary::bytes_to_hex;
use lit_node::utils::contract::decode_revert;
use tracing::{error, info};

pub async fn fund_wallet(provider: &Provider<Http>, wallet: &Wallet<SigningKey>, amount: &str) {
    let res: Result<(), ProviderError> = provider
        .request(
            "anvil_setBalance",
            [
                format!("0x{}", bytes_to_hex(wallet.address())),
                amount.to_string(),
            ],
        )
        .await;
    if let Err(e) = res {
        panic!("Couldn't set balance: {:?}", e);
    }

    // info!("User balance set using anvil_setBalance");
}

pub async fn mint_rate_limit_nft(
    rate_limit_nft_contract_address: H160,
    provider: &Provider<Http>,
    wallet: &Wallet<SigningKey>,
    rate_limit_increase: u32,
) -> U256 {
    // 1. get cost
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let rate_limit_nft_contract =
        RateLimitNFT::new(rate_limit_nft_contract_address, Arc::new(client));
    // we would like 0.001 requests per second more
    // aka 1 request per 1000 seconds more
    // this is the minimum amount we can mint
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("System time is before unix epoch.  Your computer clock is probably wrong")
        .as_secs();
    let expires_at = (now / 86400 + 1) * 86400;
    let cost = rate_limit_nft_contract
        .calculate_cost(U256::from(rate_limit_increase), U256::from(expires_at))
        .await;
    if let Err(e) = cost {
        let error_with_type: Option<RateLimitNFTErrors> = e.decode_contract_revert();
        panic!("Couldn't get cost: {:?}", error_with_type);
    }
    let cost = cost.unwrap();
    info!("Cost of rate limit NFT is {}", cost);

    // 2. mint the nft
    let tx = rate_limit_nft_contract
        .mint(U256::from(expires_at))
        .value(cost);
    let pending_tx = tx.send().await;
    if let Err(e) = pending_tx {
        error!("Problem minting nft: {:?}", e);
        let error_with_type: Option<RateLimitNFTErrors> = e.decode_contract_revert();
        if let Some(error) = error_with_type {
            panic!("Couldn't mint nft - contract revert: {:?}", error);
        }
        // ok if we're here then decoding failed.  try to decode not as a contract revert
        let decoded = decode_revert(&e, rate_limit_nft_contract.abi());
        panic!("Couldn't mint nft: {:?}", decoded);
    }
    let pending_tx = pending_tx.unwrap().interval(Duration::from_millis(100));
    let receipt = pending_tx.await.unwrap().expect("No receipt from txn");
    info!("Rate limit NFT minting receipt: {:?}", receipt);

    // 3. get the id
    let log = &receipt.logs[0];
    // tokenId is the 4th topic
    U256::from_big_endian(&log.topics[3][..])
}

pub async fn create_payment_delegation_entry(
    payment_delegation_contract_address: H160,
    provider: &Provider<Http>,
    wallet: &Wallet<SigningKey>,
    user_we_are_paying_for: H160,
    requests_per_period: u32,
    period_seconds: u32,
) {
    // 1. get cost
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let payment_delegation_contract =
        PaymentDelegation::new(payment_delegation_contract_address, Arc::new(client));

    // add restrictions
    let tx = payment_delegation_contract.set_restriction(Restriction {
        requests_per_period: U256::from(requests_per_period),
        period_seconds: U256::from(period_seconds),
    });
    let pending_tx = tx.send().await;
    if let Err(e) = pending_tx {
        error!("Problem creating payment delegation restriction: {:?}", e);
        let error_with_type: Option<PaymentDelegationErrors> = e.decode_contract_revert();
        if let Some(error) = error_with_type {
            panic!(
                "Couldn't create payment delegation restriction - contract revert: {:?}",
                error
            );
        }
        // ok if we're here then decoding failed.  try to decode not as a contract revert
        let decoded = decode_revert(&e, payment_delegation_contract.abi());
        panic!(
            "Couldn't create payment delegation restriction: {:?}",
            decoded
        );
    }
    let pending_tx = pending_tx.unwrap().interval(Duration::from_millis(100));
    let receipt = pending_tx.await.unwrap().expect("No receipt from txn");
    info!(
        "Payment delegation restriction setting receipt: {:?}",
        receipt
    );

    // do the delegation
    let tx = payment_delegation_contract.delegate_payments(user_we_are_paying_for);
    let pending_tx = tx.send().await;
    if let Err(e) = pending_tx {
        error!("Problem creating payment delegation: {:?}", e);
        let error_with_type: Option<PaymentDelegationErrors> = e.decode_contract_revert();
        if let Some(error) = error_with_type {
            panic!(
                "Couldn't create payment delegation - contract revert: {:?}",
                error
            );
        }
        // ok if we're here then decoding failed.  try to decode not as a contract revert
        let decoded = decode_revert(&e, payment_delegation_contract.abi());
        panic!("Couldn't create payment delegation: {:?}", decoded);
    }
    let pending_tx = pending_tx.unwrap().interval(Duration::from_millis(100));
    let receipt = pending_tx.await.unwrap().expect("No receipt from txn");
    info!("Payment delegation entry creation receipt: {:?}", receipt);
}
