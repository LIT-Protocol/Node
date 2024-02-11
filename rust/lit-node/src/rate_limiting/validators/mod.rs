use crate::error::Result;

use self::{ip::IpValidator, validator::Validator};

use super::models::{RateLimitDB, UserContext};

mod ip;
mod validator;

/// Validate the rate limiting UserContext against all of the registered validators.
pub(crate) async fn validate(
    rate_limit_db: &RateLimitDB,
    user_context: &UserContext,
) -> Result<()> {
    // Init all the validator plugins.
    let validators: Vec<Box<dyn Validator>> = vec![Box::new(IpValidator {
        rate_limit_db,
        threshold_wallets_per_ip_address: 5,
    })];

    // Validate all the plugins.
    for validator in validators {
        validator.validate(user_context).await?;
    }

    Ok(())
}
