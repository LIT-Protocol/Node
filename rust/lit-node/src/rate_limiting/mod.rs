use crate::auth::auth_material::AuthSigItem;
use crate::auth::resources::LitResourceAbility;
use crate::config::LitNodeConfig;
use crate::error::Result;
use crate::utils::encoding;
use chrono::{DateTime, Utc};
use lit_core::config::LitConfig;
use std::net::SocketAddr;
use std::time::SystemTime;
use tracing::instrument;

use self::data::{
    allocate_request_to_first_nft_with_quota, calculate_requests_permitted_from_tokens,
    get_oldest_timestamp, get_owned_and_authorized_rate_limit_nfts,
};
use self::models::{RateLimitCheckReturn, RateLimitDB, UserContext};

mod data;
pub mod models;

#[allow(clippy::too_many_arguments)]
#[instrument(skip_all, ret)]
pub(crate) async fn check_rate_limit(
    user_context: &UserContext,
    auth_sig: &AuthSigItem,
    chain: Option<String>,
    rate_limit_db: &RateLimitDB,
    remote_addr: SocketAddr,
    requested_lit_resource_ability: &LitResourceAbility,
    cfg: &LitConfig,
    bls_root_pubkey: &String,
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

    // Gather owned + authorized RLI NFTs.
    let all_authorized_rate_limit_nfts = get_owned_and_authorized_rate_limit_nfts(
        single_auth_sig,
        user_address,
        rate_limit_db,
        cfg,
        bls_root_pubkey,
    )
    .await?;
    debug!(
        "All authorized rate limit NFTs: {:?}",
        all_authorized_rate_limit_nfts
    );

    if !all_authorized_rate_limit_nfts.is_empty() {
        if matches!(cfg.enable_rate_limiting_allocation(), Ok(false)) {
            // if there are any authorized NFTs, we're good
            debug!("Rate limiting allocation disabled, just checking if any NFTs have capacity");
            return Ok(RateLimitCheckReturn {
                rate_limit_exceeded: false,
                try_again_after: None,
            });
        }
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

        // sum up requests used by all RLI NFTs
        // if an NFT has capacity, it will be added to the used requests for that NFT
        let used_requests_per_second_from_tokens = allocate_request_to_first_nft_with_quota(
            &all_authorized_rate_limit_nfts,
            rate_limit_db,
        )
        .await;

        // used_requests_per_second_from_tokens doesn't count the existing request, so add 1 over the whole window
        let used_requests_per_second_from_tokens = used_requests_per_second_from_tokens
            + (1.0 / default_window_duration_secs.as_secs() as f32);

        if used_requests_per_second_from_tokens <= permitted_reqs_per_second_from_tokens {
            return Ok(RateLimitCheckReturn {
                rate_limit_exceeded: false,
                try_again_after: None,
            });
        }
    }

    // If we get here, the rate limit has been exceeded.
    // Get the oldest timestamp.  If one is present, then tell them to try again after that timestamp.
    // If one is not present, then there's no valid "try again after" so return none.
    let oldest_timestamp = get_oldest_timestamp(
        &user_address_str,
        rate_limit_db,
        &all_authorized_rate_limit_nfts,
    )
    .await;
    if let Some(oldest_timestamp) = oldest_timestamp {
        // set the try_again_datetime to the oldest timestamp + the window duration.
        // this tells them when the oldest request will fall out of the window, hence, when they will
        // have quota again.
        let try_again_datetime: DateTime<Utc> =
            (oldest_timestamp + default_window_duration_secs).into();
        trace!("Try again after: {}", try_again_datetime);

        return Ok(RateLimitCheckReturn {
            rate_limit_exceeded: true,
            try_again_after: Some(try_again_datetime.to_rfc3339()),
        });
    } else {
        return Ok(RateLimitCheckReturn {
            rate_limit_exceeded: true,
            try_again_after: None,
        });
    }
}
