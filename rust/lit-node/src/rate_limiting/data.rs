use std::collections::HashSet;
use std::time::SystemTime;

use crate::auth::auth_material::JsonAuthSig;
use crate::auth::capabilities::recap::extract_and_verify_all_capabilities;
use crate::auth::validators::auth_sig::CapabilityAuthSigValidator;
use crate::auth::validators::siwe::SiweValidator;
use crate::config::LitNodeConfig;
use crate::error::{blockchain_err_code, unexpected_err_code, Result, Unexpected, EC};
use crate::models::auth::{LitResourcePrefix, SessionKeySignedMessage};
use crate::rate_limiting::models::UsageEntries;
use crate::utils::encoding;
use chrono::{TimeZone, Utc};
use ethers::prelude::*;
use ethers::types::U256;
use futures::{pin_mut, select, FutureExt};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use lit_core::utils::binary::bytes_to_hex;
use sha2::{Digest, Sha256};
use tokio::sync::RwLock;
use tracing::debug;

use super::models::RateLimitNft;
use super::models::{PossiblyDelegatedRateLimitNft, RateLimitDB};

/// This function adds a usage entry to the usage map for the first NFT token in the provided list
/// that has not reached its quota.
///
/// NOTE: This function should be run assuming that there exists at least 1 NFT token in the list
/// that has not reached its quota. Otherwise, this function will simply be a no-op.
pub(super) async fn allocate_request_to_first_nft_with_quota(
    tokens: &Vec<PossiblyDelegatedRateLimitNft>,
    rate_limit_db: &RateLimitDB,
) -> f32 {
    let rate_limit_config = rate_limit_db
        .chain_data_config_manager
        .rate_limit_config
        .read()
        .await;
    let window_start = SystemTime::now() - rate_limit_config.default_window_duration_secs;
    let mut used_requests_per_second_from_tokens = 0.0;
    for token_with_delegate_info in tokens {
        let token = token_with_delegate_info.nft.clone();
        let nft_usage_map_readable = rate_limit_db.nft_usage_map.read().await;
        let nft_usage_map = nft_usage_map_readable.get(&token.id);
        match nft_usage_map {
            Some(nft_usage_map) => {
                // Determine whether the token has reached its quota.
                let mut timestamps = nft_usage_map.timestamps.write().await;
                drop_usage_entries_older_than_time(timestamps.as_mut(), window_start);

                // Get total possible requests per second granted by this NFT.
                let total_possible_requests_per_second =
                    token.requests_per_kilosecond.as_u32() as f32 / 1000.0;

                // Since we already dropped all the requests from before the window
                // we can just use the number of requests in the window as the number of existing requests
                let existing_requests_per_second = timestamps.len() as f32
                    / rate_limit_config.default_window_duration_secs.as_secs() as f32;

                // add the used requests to the total
                used_requests_per_second_from_tokens += existing_requests_per_second;

                // If the number of existing requests is greater than or equal to the total possible requests,
                // then the quota has been reached.
                if existing_requests_per_second >= total_possible_requests_per_second {
                    // Quota reached, try the next token.
                    continue;
                }

                // Quota not reached, add a usage entry to the usage map.
                debug!("in allocate_request_to_first_nft_with_quota and adding timestamp.  there were {} timestamps before adding this one.", timestamps.len());

                timestamps.push(SystemTime::now());
            }
            None => {
                // No usage map exists for this token, create one and add a usage entry.
                debug!("in allocate_request_to_first_nft_with_quota and creating new usage map");
                drop(nft_usage_map_readable);
                let mut nft_usage_map_writeable = rate_limit_db.nft_usage_map.write().await;
                let new_usage_entries = UsageEntries {
                    timestamps: RwLock::new(vec![SystemTime::now()]),
                };
                nft_usage_map_writeable.insert(token.id, new_usage_entries);
            }
        }
        if let Some(signature_hash_uses_key) = &token_with_delegate_info.signature_hash_uses_key {
            let existing_uses = rate_limit_db
                .delegation_uses_map
                .get(&signature_hash_uses_key.clone())
                .await
                .unwrap_or(0);
            rate_limit_db
                .delegation_uses_map
                .insert(signature_hash_uses_key.clone(), existing_uses + 1)
                .await;
        }
    }
    used_requests_per_second_from_tokens
}

// return the oldest timestamp from the nft usage map
pub(super) async fn get_oldest_timestamp(
    user_address: &String,
    rate_limit_db: &RateLimitDB,
    tokens: &Vec<PossiblyDelegatedRateLimitNft>,
) -> Option<SystemTime> {
    let usage_map = rate_limit_db.nft_usage_map.read().await;
    // get the oldest timestamp from all the NFTs
    let mut oldest_timestamp: Option<SystemTime> = None;
    for token_with_delegate_info in tokens {
        let token = token_with_delegate_info.nft.clone();
        let nft_usage_map = usage_map.get(&token.id);
        match nft_usage_map {
            Some(nft_usage_map) => {
                let timestamps = nft_usage_map.timestamps.read().await;
                if timestamps.is_empty() {
                    continue;
                }
                // at this point, we've dropped everything older than the rate limit window.  to timestamps[0] should be the oldest timestamp still in the window
                let oldest_timestamp_for_nft = timestamps[0];
                if let Some(existing_oldest_timestamp) = oldest_timestamp {
                    if oldest_timestamp_for_nft < existing_oldest_timestamp {
                        oldest_timestamp = Some(oldest_timestamp_for_nft);
                    }
                } else {
                    oldest_timestamp = Some(oldest_timestamp_for_nft);
                }
            }
            None => {
                // No usage map exists for this token, so we can skip it.
                continue;
            }
        }
    }
    oldest_timestamp
    // let usage_map = rate_limit_db.free_tier_wallet_usage_map.write().await;
    // let usage = usage_map.get(user_address)?;
    // let timestamps = usage.timestamps.write().await;
    // if timestamps.is_empty() {
    //     return None;
    // }
    // Some(timestamps[0])
}

/// Gets all NFTs authorized via a capabilities signature
pub async fn get_authorized_rate_limit_nfts_via_delegation_signature(
    user_address: Address,
    session_sig: &JsonAuthSig,
    rate_limit_db: &RateLimitDB,
    cfg: &LitConfig,
    bls_root_pubkey: &String,
) -> Result<Vec<PossiblyDelegatedRateLimitNft>> {
    trace!("Getting authorized rate limit NFTs via delegation signature");
    let session_key_signed_message: std::result::Result<
        SessionKeySignedMessage,
        serde_json::Error,
    > = serde_json::from_str(&session_sig.signed_message);
    let session_key_signed_message = match session_key_signed_message {
        Ok(session_key_signed_message) => session_key_signed_message,
        Err(e) => {
            error!("Error parsing session sig in get_authorized_rate_limit_nfts.  If the user submitted a regular auth sig and not a session sig, you can ignore this.  {:?}", e);
            return Ok(vec![]);
        }
    };

    // loop over capabilities and find any for lit-ratelimitincrease://*
    let mut authorized_rate_limit_nfts: Vec<PossiblyDelegatedRateLimitNft> = Vec::new();
    for capability_auth_sig in session_key_signed_message.capabilities.iter() {
        // parse and validate the auth sig
        let siwe_validator = SiweValidator::new();
        let signed_message =
            siwe_validator.parse_siwe_message(&capability_auth_sig.signed_message)?;
        siwe_validator
            .validate_capability_auth_sig(capability_auth_sig, bls_root_pubkey)
            .await?;

        let capabilities_res = extract_and_verify_all_capabilities(&signed_message);
        let capabilities = match capabilities_res {
            Ok(capabilities) => capabilities,
            Err(e) => {
                error!("Error extracting and verifying capabilities: {:?}", e);
                continue;
            }
        };

        // hash the signature of this delegation, as it will be the "uses" key
        // this will ensure that a delegation can only be used for "uses" number of times
        let mut hasher = Sha256::new();
        hasher.update(&capability_auth_sig.sig);
        let delegation_signature_key = hasher.finalize().to_vec();
        for capability in capabilities {
            for ability in capability.recap_abilities().iter() {
                if ability.0.scheme_str() == LitResourcePrefix::RLI.to_string() {
                    // loop over the restrictions
                    for inner_ability in ability.1 {
                        if inner_ability.0.clone().into_inner() == *"Auth/Auth".to_string() {
                            // loop over the restrictions
                            for map in inner_ability.1.clone().into_iter() {
                                // possible restrictions:
                                // "nft_id" = the nft id to scope this to
                                // "uses" = the number of uses to scope this to
                                // "delegate_to" = the addresses to delegate this to

                                // skip this one if the user isn't allowed to use it
                                if let Some(delegate_to) = map.get("delegate_to") {
                                    // loop over all items in delegate_to array and check if the user is in there
                                    let delegate_to_arr = delegate_to
                                        .as_array()
                                        .expect("delegate_to is not an array");
                                    let user_address_str =
                                        bytes_to_hex(user_address).to_ascii_lowercase();
                                    let user_is_delegate =
                                        delegate_to_arr.iter().any(|x| {
                                            x.as_str()
                                        .expect("Could not convert delegate_to member to string")
                                        .to_ascii_lowercase()
                                        == user_address_str
                                        });
                                    debug!(
                                        "User is delegate: {} and delegate_to_arr: {:?}",
                                        user_is_delegate, delegate_to_arr,
                                    );
                                    if !user_is_delegate {
                                        continue;
                                    }
                                }

                                // skip this one if the user has already gone over the max uses
                                if let Some(uses) = map.get("uses") {
                                    let max_uses = uses
                                        .as_str()
                                        .expect(
                                            "Could not convert uses in delegation recap to string",
                                        )
                                        .parse::<u64>()
                                        .expect(
                                            "Could not convert uses in delegation recap to u64",
                                        );

                                    let already_used = rate_limit_db
                                        .delegation_uses_map
                                        .get(&delegation_signature_key)
                                        .await
                                        .unwrap_or(0);
                                    debug!(
                                        "User uses: {} and max uses: {}",
                                        already_used, max_uses,
                                    );
                                    if already_used as u64 >= max_uses {
                                        continue;
                                    }
                                }

                                let nfts;
                                if let Some(nft_id) = map.get("nft_id") {
                                    let nft_id_arr = nft_id
                                        .as_array()
                                        .unwrap_or(&Vec::new())
                                        .iter()
                                        .filter_map(|x| x.as_str())
                                        .flat_map(|x| U256::from_dec_str(x).ok())
                                        .collect::<Vec<U256>>();
                                    debug!("nft_id_arr: {:?}", nft_id_arr);
                                    nfts = fetch_rate_limit_nfts_by_ids(
                                        &nft_id_arr,
                                        rate_limit_db,
                                        cfg,
                                    )
                                    .await?
                                    .iter()
                                    .map(|x| PossiblyDelegatedRateLimitNft {
                                        nft: x.clone(),
                                        signature_hash_uses_key: Some(
                                            delegation_signature_key.clone(),
                                        ),
                                    })
                                    .collect::<Vec<PossiblyDelegatedRateLimitNft>>();
                                } else {
                                    // all NFTs they hold are usable
                                    nfts = get_rate_limit_nfts_by_owner(
                                        Address::from(signed_message.address),
                                        rate_limit_db,
                                        cfg,
                                    )
                                    .await?
                                    .iter()
                                    .map(|x| PossiblyDelegatedRateLimitNft {
                                        nft: x.clone().nft,
                                        signature_hash_uses_key: Some(
                                            delegation_signature_key.clone(),
                                        ),
                                    })
                                    .collect::<Vec<PossiblyDelegatedRateLimitNft>>();
                                }

                                authorized_rate_limit_nfts.extend(nfts);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(authorized_rate_limit_nfts)
}

pub async fn get_authorized_rate_limit_nfts_via_payer_db(
    user_address: Address,
    rate_limit_db: &RateLimitDB,
    cfg: &LitConfig,
) -> Result<Vec<PossiblyDelegatedRateLimitNft>> {
    trace!("Getting authorized rate limit NFTs via payer DB");
    // find all payers in the payer db that have delegated to this user
    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;
    let payment_delegation_contract = resolver.payment_delegation_contract(cfg).await?;
    let returned_tuple = payment_delegation_contract
        .get_payers_and_restrictions(vec![user_address])
        .call()
        .await
        .map_err(|e| blockchain_err_code(e, EC::NodeBlockchainError, None))?;
    let payers = returned_tuple.0.clone();
    let restrictions = returned_tuple.1.clone();
    if payers.is_empty() {
        debug!("No payers returned for wallet {}.  This should never happen, because the function should always return n elements for n users", bytes_to_hex(user_address));
        return Ok(vec![]);
    }

    // payers is of type address[][] because we support multiple payers for a single user
    // restrictions is of type string[][] because each payer should have a corresponding restriction
    // we need to check again that we got at least 1 payer to continue
    let payers = payers[0].clone();
    let restrictions = restrictions[0].clone();
    if payers.is_empty() {
        debug!(
            "No payers for wallet {}.  This means there are just no payer entries for a user.",
            bytes_to_hex(user_address)
        );
        return Ok(vec![]);
    }

    if payers.len() != restrictions.len() {
        return Err(unexpected_err_code(
            format!(
                "Payers and restrictions arrays are not the same length.  Payers length is {} and restrictions length is {}",
                payers.len(), restrictions.len()
            ),
            EC::NodeBlockchainError,
            None,
        ));
    }

    // loop over payers, and then collect the NFTs each payer has
    let mut authorized_rate_limit_nfts: Vec<PossiblyDelegatedRateLimitNft> = Vec::new();
    for (i, payer) in payers.iter().enumerate() {
        let mut delegation_uses_key = None;
        if matches!(cfg.enable_rate_limiting_allocation(), Ok(true)) {
            let restriction = match restrictions.get(i) {
                Some(restriction) => restriction,
                None => {
                    return Err(unexpected_err_code(
                        format!(
                            "No restriction found for payer {}.  This should never happen.",
                            bytes_to_hex(payer)
                        ),
                        EC::NodeBlockchainError,
                        None,
                    ))
                }
            };

            // we need to create a hash of the current period start, the user address, and the payer address.
            // this will be used to restrict the number of requests a user can make in a given period
            // find the current start for the period.  the global start is jan 1st, 1970.
            // we consider than "n" periods have elapsed since the global start, where n is the number of periods that have elapsed since the global start
            let now_seconds = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| {
                    unexpected_err_code(
                        e,
                        EC::NodeUnknownError,
                        Some("There was an error getting the system time".into()),
                    )
                })?
                .as_secs();
            if restriction.period_seconds > U256::from(u64::MAX) {
                return Err(unexpected_err_code(
                format!("Period seconds is greater than u64 max value.  This should never happen.  You may have accidently set the period seconds in the restriction to a giant number.  The number is: {}", restriction.period_seconds),
                EC::NodeBlockchainError,
                None,
            ));
            }
            // this function will panic if the number is greater than a u64, and we check above.  so this is safe
            let period_seconds = restriction.period_seconds.as_u64();
            let period_start = now_seconds / period_seconds * period_seconds;

            let mut hasher = Sha256::new();
            hasher.update(
                [
                    user_address.as_bytes(),
                    payer.as_bytes(),
                    &period_start.to_le_bytes(),
                ]
                .concat(),
            );
            let delegation_uses_key_vec = hasher.finalize().to_vec();

            // by default, they are both zero, so only check restrictions if at least 1 value is nonzero
            // which means they set a restriction
            if !restriction.requests_per_period.is_zero() && !restriction.period_seconds.is_zero() {
                // this payer has restrictions - check them
                let max_uses = match restriction.requests_per_period < U256::from(u64::MAX) {
                true => restriction.requests_per_period.as_u64(),
                false => {
                    return Err(unexpected_err_code(
                        format!(
                            "Could not convert requests per period to u64.  The number is too large.  The number is: {}",
                            restriction.requests_per_period
                        ),
                        EC::NodeBlockchainError,
                        None,
                    ))
                }
            };

                let already_used = rate_limit_db
                    .delegation_uses_map
                    .get(&delegation_uses_key_vec)
                    .await
                    .unwrap_or(0);
                debug!(
                    "Checking if payer should pay for user - User uses: {} and max uses: {}",
                    already_used, max_uses,
                );
                if already_used as u64 >= max_uses {
                    continue;
                }
            }
            delegation_uses_key = Some(delegation_uses_key_vec);
        }

        // all NFTs they hold are usable, and they haven't passed the restriction limit, so add them to the list
        let nfts = get_rate_limit_nfts_by_owner(*payer, rate_limit_db, cfg)
            .await?
            .iter()
            .map(|x| PossiblyDelegatedRateLimitNft {
                nft: x.clone().nft,
                signature_hash_uses_key: delegation_uses_key.clone(),
            })
            .collect::<Vec<PossiblyDelegatedRateLimitNft>>();
        authorized_rate_limit_nfts.extend(nfts);
        if matches!(cfg.enable_rate_limiting_allocation(), Ok(false)) {
            // early exit - we just need 1 of these
            if !authorized_rate_limit_nfts.is_empty() {
                return Ok(authorized_rate_limit_nfts);
            }
        }
    }

    Ok(authorized_rate_limit_nfts)
}

/// This function gets the owned and authorized rate limit NFTs for the given wallet address.
/// This includes all NFTs delegated to the user via the payer contract DB.
pub(super) async fn get_owned_and_authorized_rate_limit_nfts(
    session_sig: &JsonAuthSig,
    user_address: Address,
    rate_limit_db: &RateLimitDB,
    cfg: &LitConfig,
    bls_root_pubkey: &String,
) -> Result<Vec<PossiblyDelegatedRateLimitNft>> {
    let mut all_authorized_nfts = HashSet::new();

    let sig_future = get_authorized_rate_limit_nfts_via_delegation_signature(
        user_address,
        session_sig,
        rate_limit_db,
        cfg,
        bls_root_pubkey,
    )
    .fuse();

    let owned_future = get_rate_limit_nfts_by_owner(user_address, rate_limit_db, cfg).fuse();

    let payer_db_future =
        get_authorized_rate_limit_nfts_via_payer_db(user_address, rate_limit_db, cfg).fuse();

    pin_mut!(sig_future, owned_future, payer_db_future);

    let early_exit_enabled = matches!(cfg.enable_rate_limiting_allocation(), Ok(false));

    loop {
        select! {
            nfts_res = sig_future => {
                all_authorized_nfts.extend(nfts_res?);
                if early_exit_enabled && !all_authorized_nfts.is_empty() {
                    return Ok(all_authorized_nfts.into_iter().collect());
                }
            },
            nfts_res = owned_future => {
                all_authorized_nfts.extend(nfts_res?);
                if early_exit_enabled && !all_authorized_nfts.is_empty() {
                    return Ok(all_authorized_nfts.into_iter().collect());
                }
            },
            nfts_res = payer_db_future => {
                all_authorized_nfts.extend(nfts_res?);
                if early_exit_enabled && !all_authorized_nfts.is_empty() {
                    return Ok(all_authorized_nfts.into_iter().collect());
                }
            },
            complete => {
                break;
            }
        }
    }

    Ok(all_authorized_nfts.into_iter().collect())
}

/// This function fetches rate limit NFT data per the given NFT IDs.
///
/// This function only fetches from cache and can tolerate some stale data here. We only
/// fetch from chain when the cache key is missing.
///
/// This function skips past all expired NFTs.
pub(super) async fn fetch_rate_limit_nfts_by_ids(
    token_ids: &Vec<U256>,
    rate_limit_db: &RateLimitDB,
    cfg: &LitConfig,
) -> Result<Vec<RateLimitNft>> {
    let mut nfts = Vec::new();
    for token_id in token_ids {
        // Try fetch from cache
        if let Some(nft) = rate_limit_db.nft_cache.get(token_id).await {
            let token_expiry = nft.expires_at.as_u64();
            let token_expires_at = Utc
                .timestamp_opt(token_expiry as i64, 0)
                .expect_or_err("failed to get timestamp")?;
            if token_expires_at <= Utc::now() {
                continue;
            }
            nfts.push(nft.clone());
            if matches!(cfg.enable_rate_limiting_allocation(), Ok(false)) {
                // early exit - we just need 1 of these
                return Ok(nfts);
            }
        } else {
            // Fetch from chain
            let nft = fetch_rate_limit_nft(token_id, cfg).await?;
            if let Some(nft) = nft {
                let token_expiry = nft.expires_at.as_u64();
                let token_expires_at = Utc
                    .timestamp_opt(token_expiry as i64, 0)
                    .expect_or_err("failed to get timestamp")?;
                if token_expires_at <= Utc::now() {
                    continue;
                }
                nfts.push(nft.clone());
                // insert into cache
                rate_limit_db.nft_cache.insert(*token_id, nft).await;
                if matches!(cfg.enable_rate_limiting_allocation(), Ok(false)) {
                    // early exit - we just need 1 of these
                    return Ok(nfts);
                }
            }
        }
    }
    Ok(nfts)
}

/// This function fetches rate limit NFT data from chain per the given NFT ID.
///
/// This function returns None if the NFT is expired.
async fn fetch_rate_limit_nft(token_id: &U256, cfg: &LitConfig) -> Result<Option<RateLimitNft>> {
    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;

    let contract = resolver.rate_limit_nft_contract(cfg).await?;
    debug!("Getting token info from contract {:?}", contract);
    let token_info = contract
        .capacity(*token_id)
        .call()
        .await
        .map_err(|e| blockchain_err_code(e, EC::NodeBlockchainError, None))?;
    debug!("Got token info {:?}", token_info);

    // get token expiry
    let token_expiry = token_info.expires_at.as_u64();

    let token_expires_at = Utc
        .timestamp_opt(token_expiry as i64, 0)
        .expect_or_err("failed to get timestamp")?;
    if token_expires_at <= Utc::now() {
        return Ok(None);
    }

    // get token owner
    let token_owner = contract
        .owner_of(*token_id)
        .call()
        .await
        .map_err(|e| blockchain_err_code(e, EC::NodeBlockchainError, None))?;

    Ok(Some(RateLimitNft {
        id: *token_id,
        requests_per_kilosecond: token_info.requests_per_kilosecond,
        expires_at: token_info.expires_at,
        owner: token_owner,
    }))
}

// Simply get all the rate limit NFTs that this user owns
pub(super) async fn get_rate_limit_nfts_by_owner(
    owner: Address,
    rate_limit_db: &RateLimitDB,
    cfg: &LitConfig,
) -> Result<Vec<PossiblyDelegatedRateLimitNft>> {
    let owner_address_string = encoding::bytes_to_hex(owner);

    debug!("get_rate_limit_nfts_by_owner for owner: {}", owner);
    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;

    let contract = resolver.rate_limit_nft_contract(cfg).await?;

    debug!("Getting token balance from contract {:?}", contract);
    let token_balance = (contract
        .balance_of(owner)
        .call()
        .await
        .map_err(|e| blockchain_err_code(e, EC::NodeBlockchainError, None))?)
    .as_u64();
    if token_balance == 0 {
        return Ok(vec![]);
    }

    debug!("got token balance of {}", token_balance);
    let mut nft_ids = vec![];
    let mut nfts: Vec<PossiblyDelegatedRateLimitNft> = Vec::new();
    let now = Utc::now();
    for i in 0..token_balance {
        let token_id = contract
            .token_of_owner_by_index(owner, U256([i, 0, 0, 0]))
            .call()
            .await
            .map_err(|e| blockchain_err_code(e, EC::NodeBlockchainError, None))?;
        debug!("Got token id {}", token_id);
        nft_ids.push(token_id);
        let nft_infos = fetch_rate_limit_nfts_by_ids(&vec![token_id], rate_limit_db, cfg).await?;
        if nft_infos.is_empty() {
            // this can happen if the NFT is expired.  skip it.
            continue;
        }
        let nft = nft_infos[0].clone();
        nfts.push(PossiblyDelegatedRateLimitNft {
            nft,
            signature_hash_uses_key: None,
        });

        if !nfts.is_empty() && matches!(cfg.enable_rate_limiting_allocation(), Ok(false)) {
            // early exit - we just need 1 of these
            return Ok(nfts);
        }
    }

    Ok(nfts)
}

/// This function calculates the total requests permitted per second for a given array of
/// rate limit NFTs.
pub(super) async fn calculate_requests_permitted_from_tokens(
    tokens: &Vec<PossiblyDelegatedRateLimitNft>,
    rate_limit_db: &RateLimitDB,
) -> Result<f32> {
    debug!("Calculating requests permitted from tokens: {:?}", tokens);
    // Filter out expired tokens.
    let now = Utc::now();
    let mut valid_tokens = vec![];
    for token_with_delegate_info in tokens {
        let token = &token_with_delegate_info.nft;
        // only count unexpired ones
        let token_info_u64 = token.expires_at.as_u64();

        let expires_at = Utc
            .timestamp_opt(token_info_u64 as i64, 0)
            .expect_or_err("failed to get timestamp")?;
        if expires_at > now {
            valid_tokens.push(token);
        }
    }

    debug!("Looping over {} valid tokens", valid_tokens.len());
    let mut total_possible_requests_per_second = 0.0;
    for token in valid_tokens.iter() {
        total_possible_requests_per_second +=
            token.requests_per_kilosecond.as_u32() as f32 / 1000.0;
    }
    debug!(
        "total_possible_requests_per_second: {}",
        total_possible_requests_per_second
    );

    Ok(total_possible_requests_per_second)
}

/// This function starts at the newest timestamp and go back until it finds one that is older than
/// the provided drop_before_time, after which it drops from the beginning to that index.
fn drop_usage_entries_older_than_time(
    timestamps: &mut Vec<SystemTime>,
    drop_before_time: SystemTime,
) {
    let mut drop_before_index: Option<usize> = None;
    for (i, timestamp) in timestamps.iter().rev().enumerate() {
        if *timestamp < drop_before_time {
            drop_before_index = Some(i);
            break;
        }
    }
    if let Some(drop_before_index) = drop_before_index {
        timestamps.drain(0..drop_before_index + 1);
    }
}

#[cfg(test)]
mod calculate_requests_permitted_from_token_tests {
    use std::{sync::Arc, time::SystemTime};

    use ethers::types::{Address, U256};
    use lit_core::config::ReloadableLitConfig;
    use tokio::sync::RwLock;

    use crate::rate_limiting::models::PossiblyDelegatedRateLimitNft;
    use crate::rate_limiting::models::RateLimitNft;
    use crate::{
        config::chain::ChainDataConfigManager,
        rate_limiting::models::{RateLimitDB, UsageEntries},
        tests::common::get_test_config,
    };
    use tracing::info;

    use super::calculate_requests_permitted_from_tokens;

    struct TestCase {
        tokens: Vec<RateLimitNft>,
        rate_limit_db: RateLimitDB,
        expected_output: f32,
    }

    #[tokio::test]
    async fn test_calculate_requests_permitted_from_tokens() {
        let test_cases = get_test_cases().await;
        for test_case in test_cases {
            info!(
                "Checking test case tokens: {:?}, expected_output: {}",
                test_case.tokens, test_case.expected_output
            );
            let permitted_requests = calculate_requests_permitted_from_tokens(
                &test_case
                    .tokens
                    .iter()
                    .map(|x| PossiblyDelegatedRateLimitNft {
                        nft: x.clone(),
                        signature_hash_uses_key: None,
                    })
                    .collect(),
                &test_case.rate_limit_db,
            )
            .await;
            assert!(permitted_requests.is_ok());
            let permitted_requests = permitted_requests.unwrap();
            assert_eq!(permitted_requests, test_case.expected_output);
        }
    }

    async fn get_test_cases() -> Vec<TestCase> {
        let reloadable_cfg = ReloadableLitConfig::new(|| Ok(get_test_config())).unwrap();
        let cdm = Arc::new(ChainDataConfigManager::new(reloadable_cfg).await);
        let now = SystemTime::now();

        vec![
            // No tokens.
            TestCase {
                tokens: vec![],
                rate_limit_db: {
                    let rate_limit_db =
                        RateLimitDB::default_with_chain_data_config_manager(cdm.clone());
                    {
                        let mut usage_map = rate_limit_db.nft_usage_map.write().await;
                        usage_map.insert(
                            U256::from(0),
                            UsageEntries {
                                timestamps: RwLock::new(vec![now]),
                            },
                        );
                    }
                    rate_limit_db
                },
                expected_output: 0.0,
            },
            // Expired tokens.
            TestCase {
                tokens: vec![RateLimitNft {
                    id: U256::from(0),
                    requests_per_kilosecond: U256::from(100),
                    expires_at: U256::from(0),
                    owner: Address::zero(),
                }],
                rate_limit_db: {
                    let rate_limit_db =
                        RateLimitDB::default_with_chain_data_config_manager(cdm.clone());
                    {
                        let mut usage_map = rate_limit_db.nft_usage_map.write().await;
                        usage_map.insert(
                            U256::from(0),
                            UsageEntries {
                                timestamps: RwLock::new(vec![now]),
                            },
                        );
                    }
                    rate_limit_db
                },
                expected_output: 0.0,
            },
            // Valid token at the minimum requests per second.
            TestCase {
                tokens: vec![RateLimitNft {
                    id: U256::from(0),
                    requests_per_kilosecond: U256::from(1),
                    expires_at: U256::from(1985576402),
                    owner: Address::zero(),
                }],
                rate_limit_db: {
                    let rate_limit_db =
                        RateLimitDB::default_with_chain_data_config_manager(cdm.clone());
                    {
                        let mut usage_map = rate_limit_db.nft_usage_map.write().await;
                        usage_map.insert(
                            U256::from(0),
                            UsageEntries {
                                timestamps: RwLock::new(vec![now]),
                            },
                        );
                    }
                    rate_limit_db
                },
                expected_output: 0.001,
            },
            // Valid token with requests remaining.
            TestCase {
                tokens: vec![RateLimitNft {
                    id: U256::from(0),
                    requests_per_kilosecond: U256::from(200),
                    expires_at: U256::from(1985576402),
                    owner: Address::zero(),
                }],
                rate_limit_db: {
                    let rate_limit_db =
                        RateLimitDB::default_with_chain_data_config_manager(cdm.clone());
                    {
                        let mut usage_map = rate_limit_db.nft_usage_map.write().await;
                        usage_map.insert(
                            U256::from(0),
                            UsageEntries {
                                timestamps: RwLock::new(vec![now]),
                            },
                        );
                    }
                    rate_limit_db
                },
                expected_output: 0.2,
            },
            // Valid token with expired timestamps.
            TestCase {
                tokens: vec![RateLimitNft {
                    id: U256::from(0),
                    requests_per_kilosecond: U256::from(200),
                    expires_at: U256::from(1985576402),
                    owner: Address::zero(),
                }],
                rate_limit_db: {
                    let rate_limit_db =
                        RateLimitDB::default_with_chain_data_config_manager(cdm.clone());
                    {
                        let mut usage_map = rate_limit_db.nft_usage_map.write().await;
                        usage_map.insert(
                            U256::from(0),
                            UsageEntries {
                                timestamps: RwLock::new(vec![SystemTime::UNIX_EPOCH]),
                            },
                        );
                    }
                    rate_limit_db
                },
                expected_output: 0.2,
            },
        ]
    }
}

#[cfg(test)]
mod get_oldest_timestamp_tests {
    use std::{sync::Arc, time::SystemTime};

    use ethers::types::{Address, U256};
    use lit_core::config::ReloadableLitConfig;
    use tokio::sync::RwLock;

    use crate::{
        config::chain::ChainDataConfigManager,
        rate_limiting::models::{
            PossiblyDelegatedRateLimitNft, RateLimitDB, RateLimitNft, UsageEntries,
        },
        tests::common::get_test_config,
    };

    use super::get_oldest_timestamp;

    struct TestCase {
        user_address: String,
        rate_limit_db: RateLimitDB,
        expected_output: Option<SystemTime>,
    }

    #[tokio::test]
    async fn test_get_oldest_timestamp() {
        let test_cases = get_test_cases().await;
        for test_case in test_cases {
            let oldest_timestamp = get_oldest_timestamp(
                &test_case.user_address,
                &test_case.rate_limit_db,
                &vec![PossiblyDelegatedRateLimitNft {
                    nft: RateLimitNft {
                        id: U256::from(555),
                        requests_per_kilosecond: U256::from(100),
                        expires_at: U256::from(0),
                        owner: Address::zero(),
                    },
                    signature_hash_uses_key: None,
                }],
            )
            .await;
            assert_eq!(oldest_timestamp, test_case.expected_output);
        }
    }

    async fn get_test_cases() -> Vec<TestCase> {
        let reloadable_cfg = ReloadableLitConfig::new(|| Ok(get_test_config())).unwrap();
        let cdm = Arc::new(ChainDataConfigManager::new(reloadable_cfg).await);
        let now = SystemTime::now();

        vec![
            TestCase {
                user_address: "123".into(),
                rate_limit_db: RateLimitDB::default_with_chain_data_config_manager(cdm.clone()),
                expected_output: None,
            },
            TestCase {
                user_address: "123".into(),
                rate_limit_db: {
                    let rate_limit_db =
                        RateLimitDB::default_with_chain_data_config_manager(cdm.clone());
                    rate_limit_db
                },
                expected_output: None,
            },
            TestCase {
                user_address: "123".into(),
                rate_limit_db: {
                    let rate_limit_db =
                        RateLimitDB::default_with_chain_data_config_manager(cdm.clone());
                    {
                        let mut usage_map = rate_limit_db.nft_usage_map.write().await;
                        usage_map.insert(
                            U256::from(555),
                            UsageEntries {
                                timestamps: RwLock::new(vec![now]),
                            },
                        );
                    }
                    rate_limit_db
                },
                expected_output: Some(now),
            },
        ]
    }
}

#[cfg(test)]
mod allocate_request_to_nft_tests {
    use std::{sync::Arc, time::SystemTime};

    use ethers::types::{Address, U256};
    use lit_core::config::ReloadableLitConfig;
    use tokio::sync::RwLock;

    use crate::{
        config::chain::ChainDataConfigManager,
        rate_limiting::models::{
            PossiblyDelegatedRateLimitNft, RateLimitDB, RateLimitNft, UsageEntries,
        },
        tests::common::get_test_config,
    };

    use super::allocate_request_to_first_nft_with_quota;

    struct TestCase {
        tokens: Vec<RateLimitNft>,
        rate_limit_db: RateLimitDB,
        expected_token_id_mutated: U256,
        expected_timestamps_length: usize,
    }

    #[tokio::test]
    async fn test_allocate_request_to_nft() {
        let test_cases = get_test_cases().await;
        for test_case in test_cases {
            allocate_request_to_first_nft_with_quota(
                &test_case
                    .tokens
                    .iter()
                    .map(|x| PossiblyDelegatedRateLimitNft {
                        nft: x.clone(),
                        signature_hash_uses_key: None,
                    })
                    .collect(),
                &test_case.rate_limit_db,
            )
            .await;
            let usage_map = test_case.rate_limit_db.nft_usage_map.read().await;
            let timestamps = usage_map
                .get(&test_case.expected_token_id_mutated)
                .unwrap()
                .timestamps
                .read()
                .await;
            assert_eq!(timestamps.len(), test_case.expected_timestamps_length);
        }
    }

    async fn get_test_cases() -> Vec<TestCase> {
        let reloadable_cfg = ReloadableLitConfig::new(|| Ok(get_test_config())).unwrap();
        let cdm = Arc::new(ChainDataConfigManager::new(reloadable_cfg).await);
        let now = SystemTime::now();

        vec![
            // 1 NFT with no usage entries.
            TestCase {
                tokens: vec![RateLimitNft {
                    id: U256::from(0),
                    requests_per_kilosecond: U256::from(100),
                    expires_at: U256::from(0),
                    owner: Address::zero(),
                }],
                rate_limit_db: RateLimitDB::default_with_chain_data_config_manager(cdm.clone()),
                expected_token_id_mutated: U256::from(0),
                expected_timestamps_length: 1,
            },
            // 1 NFT with usage entries below its quota.
            TestCase {
                tokens: vec![RateLimitNft {
                    id: U256::from(0),
                    requests_per_kilosecond: U256::from(100),
                    expires_at: U256::from(0),
                    owner: Address::zero(),
                }],
                rate_limit_db: {
                    let rate_limit_db =
                        RateLimitDB::default_with_chain_data_config_manager(cdm.clone());
                    {
                        let mut usage_map = rate_limit_db.nft_usage_map.write().await;
                        usage_map.insert(
                            U256::from(0),
                            UsageEntries {
                                timestamps: RwLock::new(vec![now]),
                            },
                        );
                    }
                    rate_limit_db
                },
                expected_token_id_mutated: U256::from(0),
                expected_timestamps_length: 2,
            },
            // 2 NFTs, with the first one having usage entries below its quota.
            TestCase {
                tokens: vec![
                    RateLimitNft {
                        id: U256::from(0),
                        requests_per_kilosecond: U256::from(100),
                        expires_at: U256::from(0),
                        owner: Address::zero(),
                    },
                    RateLimitNft {
                        id: U256::from(1),
                        requests_per_kilosecond: U256::from(100),
                        expires_at: U256::from(0),
                        owner: Address::zero(),
                    },
                ],
                rate_limit_db: {
                    let rate_limit_db =
                        RateLimitDB::default_with_chain_data_config_manager(cdm.clone());
                    {
                        let mut usage_map = rate_limit_db.nft_usage_map.write().await;
                        usage_map.insert(
                            U256::from(0),
                            UsageEntries {
                                timestamps: RwLock::new(vec![now]),
                            },
                        );
                    }
                    rate_limit_db
                },
                expected_token_id_mutated: U256::from(0),
                expected_timestamps_length: 2,
            },
            // 2 NFTs, with the first one having usage entries at its quota.
            TestCase {
                tokens: vec![
                    RateLimitNft {
                        id: U256::from(0),
                        requests_per_kilosecond: U256::from(0),
                        expires_at: U256::from(0),
                        owner: Address::zero(),
                    },
                    RateLimitNft {
                        id: U256::from(1),
                        requests_per_kilosecond: U256::from(100),
                        expires_at: U256::from(0),
                        owner: Address::zero(),
                    },
                ],
                rate_limit_db: {
                    let rate_limit_db =
                        RateLimitDB::default_with_chain_data_config_manager(cdm.clone());
                    {
                        let mut usage_map = rate_limit_db.nft_usage_map.write().await;
                        usage_map.insert(
                            U256::from(0),
                            UsageEntries {
                                timestamps: RwLock::new(vec![now]),
                            },
                        );
                    }
                    rate_limit_db
                },
                expected_token_id_mutated: U256::from(1),
                expected_timestamps_length: 1,
            },
        ]
    }
}
