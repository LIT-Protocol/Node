use std::collections::HashSet;

use lit_core::error::Unexpected;

use crate::error::{validation_err, Result};
use crate::rate_limiting::models::{RateLimitDB, UserContext};

use super::validator::Validator;

pub(super) struct IpValidator<'a> {
    pub rate_limit_db: &'a RateLimitDB,
    pub threshold_wallets_per_ip_address: usize,
}

#[async_trait::async_trait]
impl<'a> Validator for IpValidator<'a> {
    async fn validate(&self, user_context: &UserContext) -> Result<()> {
        // Get user IP address
        let ip_address = match &user_context.ip_address {
            Some(ip_address) => ip_address,
            None => {
                warn!("IP address not found in user context");
                return Ok(());
            }
        };
        let user_address = user_context
            .user_address
            .expect_or_err("Missing user address")?;

        // Read wallet addresses from map.
        match self
            .rate_limit_db
            .ip_to_wallet_addresses_map
            .get(ip_address)
        {
            Some(wallet_addresses) => {
                // Check if the number of wallet addresses for this IP address exceeds the threshold.
                if wallet_addresses.len() > self.threshold_wallets_per_ip_address {
                    error!(
                        "Too many wallets associated with IP address: {}",
                        ip_address
                    );
                    return Err(validation_err(
                        "Too many wallets associated with IP address",
                        None,
                    ));
                }

                // Validation successful, now add the wallet address to the map.
                let mut new_wallet_addresses = wallet_addresses.clone();
                new_wallet_addresses.insert(user_address);
                self.rate_limit_db
                    .ip_to_wallet_addresses_map
                    .insert(ip_address.clone(), new_wallet_addresses)
                    .await;
            }
            None => {
                // Validation successful, now add the wallet address to the map.
                let mut ip_wallet_addresses = HashSet::new();
                ip_wallet_addresses.insert(user_address);
                self.rate_limit_db
                    .ip_to_wallet_addresses_map
                    .insert(ip_address.clone(), ip_wallet_addresses)
                    .await;
            }
        };

        Ok(())
    }
}
