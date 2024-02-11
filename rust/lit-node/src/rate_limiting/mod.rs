use crate::auth::auth_material::AuthSigItem;
use crate::auth::resources::LitResourceAbility;
use crate::config::LitNodeConfig;
use crate::error::Result;
use crate::rate_limiting::validators::validate;
use crate::utils::encoding;
use chrono::{DateTime, Utc};
use lit_core::config::LitConfig;
use std::net::SocketAddr;
use std::time::SystemTime;
use tracing::instrument;

use self::data::{
    allocate_request_to_first_nft_with_quota, calculate_requests_permitted_from_tokens,
    check_free_tier_reached, get_free_tier_rate_limit_requests_per_second, get_oldest_timestamp,
    get_owned_and_authorized_rate_limit_nfts, get_requests_per_second,
};
use self::models::{RateLimitCheckReturn, RateLimitDB, UserContext};
// use self::validators::validate;

mod data;
pub mod models;
mod validators;

#[instrument(skip_all, ret)]
pub(crate) async fn check_rate_limit(
    user_context: &UserContext,
    auth_sig: &AuthSigItem,
    chain: Option<String>,
    rate_limit_db: &RateLimitDB,
    remote_addr: SocketAddr,
    requested_lit_resource_ability: &LitResourceAbility,
    cfg: &LitConfig,
) -> Result<RateLimitCheckReturn> {
    // should we even check the rate limit?
    if matches!(cfg.enable_rate_limiting(), Ok(false)) {
        debug!("Rate limiting disabled, skipping checks");
        return Ok(RateLimitCheckReturn {
            rate_limit_exceeded: false,
            try_again_after: None,
        });
    }

    let default_window_duration_secs = {
        let rate_limit_config = rate_limit_db
            .chain_data_config_manager
            .rate_limit_config
            .read()
            .await;
        rate_limit_config.default_window_duration_secs
    };

    let user_address = match user_context.user_address {
        Some(user_address) => user_address,
        None => {
            // If user context does not have EVM-compatible user address, reject request.
            error!(
                "User context does not have EVM-compatible user address and must use an RLI NFT."
            );
            let now = SystemTime::now();
            let try_again_datetime: DateTime<Utc> = (now + default_window_duration_secs).into();
            return Ok(RateLimitCheckReturn {
                rate_limit_exceeded: true,
                try_again_after: Some(try_again_datetime.to_rfc3339()),
            });
        }
    };
    let user_address_str = encoding::bytes_to_hex(user_address);

    // 1. Check against the free rate limit tier.
    let existing_requests_per_second =
        get_requests_per_second(rate_limit_db, &user_address_str).await;
    debug!(
        "Existing requests per second: {}",
        existing_requests_per_second
    );
    let free_tier_rate_limit_requests_per_second =
        get_free_tier_rate_limit_requests_per_second(rate_limit_db).await;
    // NOTE: This call to check free tier reached adds a new entry to the usage map if the free tier is not breached.
    // This is also why we get the existing_requests_per_second above, prior to making this call.
    let free_tier_reached = check_free_tier_reached(
        rate_limit_db,
        &user_address_str,
        free_tier_rate_limit_requests_per_second,
    )
    .await;
    debug!("Free tier reached: {}", free_tier_reached);

    // Check against all validators - the only one right now is the user IP address.
    // this checks if they're trying to scam us by sending a lot of NFTs from different IPs
    // you can think of validate_sucess == true as meaning "this user is not scamming us"
    let validate_success = match validate(rate_limit_db, user_context).await {
        Ok(_) => true,
        Err(e) => {
            warn!("Failed to validate rate limit: {:?}", e);
            false
        }
    };
    // let validate_success = true;

    // if the user didn't hit the free tier, and isn't scamming us, we should just return.
    // they're good to go and we don't need to do any more checks
    if !free_tier_reached && validate_success {
        return Ok(RateLimitCheckReturn {
            rate_limit_exceeded: false,
            try_again_after: None,
        });
    }

    let single_auth_sig = match auth_sig {
        AuthSigItem::Single(single_auth_sig) => single_auth_sig,
        AuthSigItem::Multiple(_) => {
            error!("MultiAuthSig not supported for rate limiting");
            return Ok(RateLimitCheckReturn {
                rate_limit_exceeded: true,
                try_again_after: None,
            });
        }
    };

    // they are over the free tier rate limit
    // Gather owned + authorized RLI NFTs.
    let all_authorized_rate_limit_nfts = get_owned_and_authorized_rate_limit_nfts(
        single_auth_sig,
        user_address,
        &user_context.authorized_rate_limit_nft_ids,
        rate_limit_db,
        cfg,
    )
    .await?;
    debug!(
        "All authorized rate limit NFTs: {:?}",
        all_authorized_rate_limit_nfts
    );

    // Separate validation logic depending if the user has any authorized RLI NFTs.
    if !all_authorized_rate_limit_nfts.is_empty() {
        // Free tier has been reached - check against RLI NFT quota.
        let permitted_reqs_per_second_from_tokens = calculate_requests_permitted_from_tokens(
            &all_authorized_rate_limit_nfts,
            rate_limit_db,
        )
        .await?;
        debug!(
            "Permitted requests per second from tokens: {}",
            permitted_reqs_per_second_from_tokens
        );

        // sum up requests used by all RLI NFTs and add to existing_requests_per_second
        let used_requests_per_second_from_tokens = allocate_request_to_first_nft_with_quota(
            &all_authorized_rate_limit_nfts,
            rate_limit_db,
        )
        .await;

        // used_requests_per_second_from_tokens doesn't count the existing request, so add 1 over the whole window
        let used_requests_per_second_from_tokens = used_requests_per_second_from_tokens
            + (1.0 / default_window_duration_secs.as_secs() as f32);

        // Add free_tier_rate_limit_requests_per_second because the RLI nft stacks with the free tier limit
        // add 1 to account for the current request
        if existing_requests_per_second + used_requests_per_second_from_tokens
            <= permitted_reqs_per_second_from_tokens + free_tier_rate_limit_requests_per_second
        {
            return Ok(RateLimitCheckReturn {
                rate_limit_exceeded: false,
                try_again_after: None,
            });
        }
    }

    // If we get here, the (free or paid) rate limit tier has been exceeded.
    // Get the oldest timestamp.
    let oldest_timestamp = get_oldest_timestamp(&user_address_str, rate_limit_db)
        .await
        .unwrap_or(SystemTime::now());

    let try_again_datetime: DateTime<Utc> =
        (oldest_timestamp + default_window_duration_secs).into();
    trace!("Try again after: {}", try_again_datetime);

    return Ok(RateLimitCheckReturn {
        rate_limit_exceeded: true,
        try_again_after: Some(try_again_datetime.to_rfc3339()),
    });
}
