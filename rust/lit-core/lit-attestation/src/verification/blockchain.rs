use ethers::abi::AbiEncode;
use ethers::prelude::{Bytes, Http, Provider};
use lit_blockchain::contracts::release_register::Release;
use log::{as_serde, debug};
use serde_json::Value;
use std::sync::Arc;

use lit_blockchain::contracts::staking::Staking;
use lit_blockchain::contracts::staking_balances::StakingBalances;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_blockchain::util::ether::public_key_to_address;
use lit_blockchain::ReleaseRegister;
use lit_core::config::LitConfig;
use lit_core::utils::binary::bytes_to_hex;

use crate::error::{blockchain_err, config_err, Result};
use crate::verification::cache::{CacheValue, CACHE};

pub(crate) async fn unwrap_or_create_contract_resolver(
    cfg: &LitConfig, resolver: Option<&ContractResolver>, subnet_id: Option<&String>, label: &str,
) -> Result<ContractResolver> {
    let resolver = match resolver {
        Some(resolver) => resolver.clone(),
        None => ContractResolver::try_from(cfg).map_err(|e| {
            config_err(e, Some(format!("{label}: failed to load contract resolver")))
        })?,
    };

    if let Some(subnet_id) = subnet_id {
        if !resolver.subnet_id().eq(subnet_id) {
            return Ok(resolver.for_subnet(subnet_id.clone()));
        }
    }

    Ok(resolver)
}

// Release

pub(crate) async fn get_release_register_contract(
    cfg: &LitConfig, resolver: &ContractResolver, label: &str,
) -> Result<ReleaseRegister<Provider<Http>>> {
    resolver.release_register_contract(cfg).await.map_err(|e| {
        blockchain_err(e, Some(format!("{label}: failed to resolve ReleaseRegister contract")))
    })
}

pub(crate) async fn get_release(
    contract: &ReleaseRegister<Provider<Http>>, subnet_id: &str, release_id: [u8; 32], label: &str,
) -> Result<Release> {
    let cache_key = format!("rr:rel:{}:{}", subnet_id, bytes_to_hex(release_id));

    if let Some(release) = CACHE.get(&cache_key) {
        if let Some(release) = release.as_release() {
            return Ok(release.clone());
        }
    }

    let release = contract.get_release(release_id).call().await.map_err(|e| {
        blockchain_err(e, Some(format!("{label}: failed to call ReleaseRegister.get_release")))
            .add_field("release_id", Value::String(bytes_to_hex(release_id)))
    })?;

    CACHE.insert(cache_key, Arc::new(release.clone())).await;

    Ok(release)
}

pub(crate) async fn release_has_allowed_admin_signing_public_key(
    contract: &ReleaseRegister<Provider<Http>>, subnet_id: &str, public_key: Vec<u8>, label: &str,
) -> Result<bool> {
    let cache_key = format!("rr:aspk:{}:{}", subnet_id, bytes_to_hex(&public_key));

    if let Some(has) = CACHE.get(&cache_key) {
        if let Some(has) = has.as_bool() {
            return Ok(*has);
        }
    }

    let public_key_bytes = Bytes::from(public_key.clone());

    let valid =
        contract.has_allowed_admin_signing_public_key(public_key_bytes).call().await.map_err(
            |e| {
                blockchain_err(
                    e,
                    Some(format!(
                        "{label}: failed to call ReleaseRegister.has_admin_signing_public_key"
                    )),
                )
                .add_field("public_key", Value::String(bytes_to_hex(public_key)))
            },
        )?;

    CACHE.insert(cache_key, Arc::new(valid)).await;

    Ok(valid)
}

pub(crate) async fn release_has_allowed_author_key_digest(
    contract: &ReleaseRegister<Provider<Http>>, subnet_id: &str, author_key_digest: Vec<u8>,
    label: &str,
) -> Result<bool> {
    let cache_key = format!("rr:aakd:{}:{}", subnet_id, bytes_to_hex(&author_key_digest));

    if let Some(has) = CACHE.get(&cache_key) {
        if let Some(has) = has.as_bool() {
            return Ok(*has);
        }
    }

    let author_key_digest_bytes = Bytes::from(author_key_digest.clone());

    let valid =
        contract.has_allowed_author_key_digest(author_key_digest_bytes).call().await.map_err(
            |e| {
                blockchain_err(
                    e,
                    Some(format!(
                        "{label}: failed to call ReleaseRegister.has_allowed_author_key_digest"
                    )),
                )
                .add_field("author_key_digest", Value::String(bytes_to_hex(&author_key_digest)))
            },
        )?;

    CACHE.insert(cache_key, Arc::new(valid)).await;

    Ok(valid)
}

// Staking

pub(crate) async fn get_staking_contract(
    cfg: &LitConfig, resolver: &ContractResolver, label: &str,
) -> Result<Staking<Provider<Http>>> {
    resolver.staking_contract(cfg).await.map_err(|e| {
        blockchain_err(e, Some(format!("{label}: failed to resolve Staking contract")))
    })
}

pub(crate) async fn get_staking_balances_contract(
    cfg: &LitConfig, resolver: &ContractResolver, label: &str,
) -> Result<StakingBalances<Provider<Http>>> {
    resolver.staking_balances_contract(cfg).await.map_err(|e| {
        blockchain_err(e, Some(format!("{label}: failed to resolve Staking Balances contract")))
    })
}

pub(crate) async fn staking_has_allowed_node_signing_public_key(
    contract: &Staking<Provider<Http>>, balances_contract: &StakingBalances<Provider<Http>>,
    subnet_id: &str, public_key: &[u8], label: &str,
) -> Result<bool> {
    let cache_key = format!("s:anspk:{}:{}", subnet_id, bytes_to_hex(public_key));

    if let Some(has) = CACHE.get(&cache_key) {
        if let Some(has) = has.as_bool() {
            return Ok(*has);
        }
    }

    let node_address = public_key_to_address(public_key)?;

    let staking_address =
        contract.node_address_to_staker_address(node_address).call().await.map_err(|e| {
            blockchain_err(
                e,
                Some(format!("{label}: failed to call Staking.node_address_to_staker_address")),
            )
            .add_field("public_key", Value::String(bytes_to_hex(public_key)))
            .add_field("node_address", Value::String(node_address.encode_hex()))
        })?;

    if staking_address.is_zero() {
        debug!(
            subnet_id = as_serde!(subnet_id),
            public_key = as_serde!(bytes_to_hex(public_key)),
            node_address = as_serde!(node_address.encode_hex()),
            staking_address = as_serde!(staking_address.encode_hex());
            "failed to verify attestation by node signing key - staking contract does not recognise the node address"
        );

        CACHE.insert(cache_key, Arc::new(false)).await;

        return Ok(false);
    }

    let validator = contract.validators(staking_address).call().await.map_err(|e| {
        blockchain_err(e, Some(format!("{label}: failed to call Staking.validators")))
            .add_field("public_key", Value::String(bytes_to_hex(public_key)))
            .add_field("node_address", Value::String(node_address.encode_hex()))
            .add_field("staking_address", Value::String(staking_address.encode_hex()))
    })?;
    let validator_ip = validator.ip;
    let validator_balance =
        balances_contract.balance_of(staking_address).call().await.map_err(|e| {
            blockchain_err(e, Some(format!("{label}: failed to call StakingBalances.balance_of")))
                .add_field("public_key", Value::String(bytes_to_hex(public_key)))
                .add_field("node_address", Value::String(node_address.encode_hex()))
                .add_field("staking_address", Value::String(staking_address.encode_hex()))
        })?;
    let valid = validator_ip != 0 && !validator_balance.is_zero();
    if !valid {
        debug!(
            subnet_id = as_serde!(subnet_id),
            public_key = as_serde!(bytes_to_hex(public_key)),
            node_address = as_serde!(node_address.encode_hex()),
            staking_address = as_serde!(staking_address.encode_hex()),
            validator_valid = as_serde!(validator_ip != 0),
            validator_balance = as_serde!(validator_balance.to_string());
            "failed to verify attestation by node signing key - staking address not valid validator or has zero balance"
        );
    }

    CACHE.insert(cache_key, Arc::new(valid)).await;

    Ok(valid)
}
