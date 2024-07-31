use std::time::SystemTime;

use crate::error::validation_err_code;
use lit_core::config::LitConfig;
use std::str::FromStr;

use crate::auth::auth_material::JsonAuthSig;
use crate::auth::validators::wallet_sig::validate_wallet_sig_by_chain;
use crate::error::{parser_err_code, Result, EC};
use crate::models::AuthMethodResponse;

use super::auth_method_verifier::AuthMethodVerifier;
use super::constants::WALLET_AUTH_METHOD_TYPE_ID;

pub const DEFAULT_EXPIRATION_DAYS: i64 = 7;

/// This verifier validates that a wallet signature is valid.
/// Note that this only works for AuthSigs, not SessionSigs.
pub struct WalletAuthMethodVerifier<'a> {
    pub cfg: &'a LitConfig,
}

#[async_trait::async_trait]
impl AuthMethodVerifier for WalletAuthMethodVerifier<'_> {
    async fn verify(&self, access_token: &str) -> Result<AuthMethodResponse> {
        debug!("Verifying wallet auth method");
        verify_wallet_auth_method(access_token, self.cfg).await
    }
}

async fn verify_wallet_auth_method(
    access_token: &str,
    cfg: &LitConfig,
) -> Result<AuthMethodResponse> {
    // Parse the access token into a JsonAuthSig using serde_json
    let wallet_sig: JsonAuthSig = serde_json::from_str(access_token).map_err(|e| {
        parser_err_code(
            e,
            EC::NodeWalletSignatureJSONError,
            Some("Unable to parse string into a JsonAuthSig".into()),
        )
    })?;

    // Validate the authSig
    validate_wallet_sig_by_chain(&wallet_sig, &None, cfg).await?;

    // Return the address as the auth method response
    Ok(AuthMethodResponse {
        user_id: wallet_sig.address.clone(),
        app_id: "lit".to_string(),
        auth_method_type: WALLET_AUTH_METHOD_TYPE_ID,
        last_retrieved_at: SystemTime::now(),
        expiration: get_expiration_for_auth_sig(&wallet_sig)?,
        used_for_sign_session_key_request: false,
    })
}

fn get_expiration_for_auth_sig(auth_sig: &JsonAuthSig) -> Result<i64> {
    let exp = match siwe::Message::from_str(&auth_sig.signed_message) {
        Ok(parsed_siwe) => match parsed_siwe.expiration_time {
            Some(exp) => exp.as_ref().unix_timestamp(),
            None => {
                return Err(validation_err_code(
                    "Siwe message should contain expiration time",
                    EC::NodeUndefinedSiweExpiration,
                    None,
                ));
            }
        },
        Err(e) => {
            return Err(validation_err_code(
                e,
                EC::NodeSIWEMessageError,
                Some("Could not parse SIWE message when checking for expiration time".into()),
            ));
        }
    };

    Ok(exp)
}

#[cfg(test)]
mod tests {
    use crate::auth::auth_material::JsonAuthSig;
    use crate::pkp::auth::wallet::get_expiration_for_auth_sig;
    use crate::utils::encoding;
    use chrono::{Duration, Utc};
    use ethers::prelude::rand::rngs::OsRng as EthersOsRng;
    use ethers::signers::{LocalWallet, Signer};
    use ethers::types::H256;
    use siwe::Message;

    #[test]
    fn test_expiration_for_auth_sig() {
        let non_siwe_auth_sig = JsonAuthSig::new(
            "0x2bdede6164f56a601fc17a8a78327d28b54e87cf3fa20373fca1d73b804566736d76efe2dd79a4627870a50e66e1a9050ca333b6f98d9415d8bca424980611ca1c".into(),
            "def".to_string(),
            "ghi".to_string(),
            "jkl".to_string(),
            None,
        );
        let non_siwe_exp = (Utc::now() + Duration::days(7)).timestamp();

        let wallet = LocalWallet::new(&mut EthersOsRng);
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.fZ").to_string();
        let expiration = (Utc::now() + Duration::days(1)) // Sets 1 day expiration
            .format("%Y-%m-%dT%H:%M:%S%.fZ")
            .to_string();

        let siwe_message = Message {
            domain: "localhost:7470".parse().unwrap(),
            address: wallet.address().into(),
            statement: Some(r#"Some custom statement."#.into()),
            uri: "https://localhost/login".parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "JIsknRumpxsM9pqmc".into(),
            issued_at: now.parse().unwrap(),
            expiration_time: Some(expiration.parse().unwrap()),
            not_before: None,
            request_id: None,
            resources: vec![],
        };

        // Generate signature
        let sig = wallet
            .sign_hash(H256::from(siwe_message.eip191_hash().unwrap()))
            .expect("Could not sign hash");

        let siwe_auth_sig = JsonAuthSig::new(
            sig.to_string(),
            "def".to_string(),
            siwe_message.to_string(),
            encoding::bytes_to_hex(wallet.address()),
            None,
        );

        let siwe_exp = get_expiration_for_auth_sig(&siwe_auth_sig).unwrap();

        assert!(
            non_siwe_exp - siwe_exp > 500000,
            "Non-siwe expiration is not around 6 days higher than the Siwe expiration"
        ) // More than 6 day
    }
}
