use std::time::SystemTime;

use chrono::Days;
use lit_core::error::Unexpected;

use crate::auth::auth_material::JsonAuthSig;
use crate::auth::validators::wallet_sig::validate_wallet_sig_by_chain;
use crate::error::{parser_err_code, Result, EC};
use crate::models::AuthMethodResponse;

use super::auth_method_verifier::AuthMethodVerifier;
use super::constants::WALLET_AUTH_METHOD_TYPE_ID;

/// This verifier validates that a wallet signature is valid.
/// Note that this only works for AuthSigs, not SessionSigs.
pub struct WalletAuthMethodVerifier {}

#[async_trait::async_trait]
impl AuthMethodVerifier for WalletAuthMethodVerifier {
    async fn verify(&self, access_token: &str) -> Result<AuthMethodResponse> {
        debug!("Verifying wallet auth method");
        verify_wallet_auth_method(access_token).await
    }
}

async fn verify_wallet_auth_method(access_token: &str) -> Result<AuthMethodResponse> {
    // Parse the access token into a JsonAuthSig using serde_json
    let wallet_sig: JsonAuthSig = serde_json::from_str(access_token).map_err(|e| {
        parser_err_code(
            e,
            EC::NodeWalletSignatureJSONError,
            Some("Unable to parse string into a JsonAuthSig".into()),
        )
    })?;

    // Validate the authSig
    validate_wallet_sig_by_chain(&wallet_sig, &None).await?;
    let exp = chrono::Utc::now()
        .checked_add_days(Days::new(7))
        .expect_or_err("Could not convert exp to timestamp")?;
    let exp = exp.timestamp();

    // Return the address as the auth method response
    Ok(AuthMethodResponse {
        user_id: wallet_sig.address,
        app_id: "lit".to_string(),
        auth_method_type: WALLET_AUTH_METHOD_TYPE_ID,
        last_retrieved_at: SystemTime::now(),
        expiration: exp,
        used_for_sign_session_key_request: false,
    })
}
