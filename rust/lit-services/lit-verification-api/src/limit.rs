use log::error;
use rocket::{catch, Request};
use rocket_governor::{LimitError, Method, Quota, RocketGovernable};

use crate::errors::{timeout_err_code, EC};

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_second(Self::nonzero(4u32))
    }
}

#[catch(429)]
pub fn rocket_governor_catcher<'r>(request: &'r Request) -> &'r LimitError {
    let cached_res: &Result<(), LimitError> = request.local_cache(|| Err(LimitError::Error));
    let err = timeout_err_code(
        "Limit exceeded allowed requests per second".to_string(),
        EC::VerificationRateLimitError,
        Some("Rate limit exceeded".to_string()),
    );
    error!("rate limit error: {}", err);

    if let Err(limit_err) = cached_res {
        limit_err
    } else {
        &LimitError::Error
    }
}
