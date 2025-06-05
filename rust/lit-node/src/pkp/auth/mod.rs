use std::sync::Arc;

use crate::auth::auth_material::JsonAuthSig;
use crate::error::{self, unexpected_err_code, validation_err, validation_err_code, Error};
use crate::error::{unexpected_err, EC};
use crate::models;
use crate::utils::encoding;
use anyhow::Result;
use ethers::abi::AbiEncode;
use ethers::core::utils::to_checksum;
use ethers::prelude::*;
use ethers::types::Bytes;
use ethers::utils::keccak256;
use lit_blockchain::config::LitBlockchainConfig;
use lit_blockchain::contracts::pkp_permissions::{self, PKPPermissions};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;

use lit_core::error::Unexpected;
use lit_core::utils::ipfs::bytes_to_ipfs_cid;
use tracing::instrument;

mod apple;
pub mod auth_method_verifier;
pub mod constants;
mod discord;
mod google;
pub mod stytch;
pub mod wallet;
pub mod webauthn;

use self::constants::{
    APPLE_JWT_AUTH_METHOD_TYPE_ID, DISCORD_AUTH_METHOD_TYPE_ID, GOOGLE_AUTH_METHOD_TYPE_ID,
    GOOGLE_JWT_AUTH_METHOD_TYPE_ID, STYTCH_JWT_AUTH_FACTOR_EMAIL_OTP,
    STYTCH_JWT_AUTH_FACTOR_SMS_OTP, STYTCH_JWT_AUTH_FACTOR_TOTP,
    STYTCH_JWT_AUTH_FACTOR_WHATS_APP_OTP, STYTCH_JWT_AUTH_METHOD_TYPE_ID,
    WALLET_AUTH_METHOD_TYPE_ID, WEBAUTHN_AUTH_METHOD_TYPE_ID,
};

use self::apple::AppleJwtAuthMethodVerifier;
use self::discord::DiscordAuthMethodVerifier;
use self::google::{GoogleAuthMethodVerifier, GoogleJwtAuthMethodVerifier};
use self::stytch::StytchJWTAuthMethodVerifier;
use self::wallet::WalletAuthMethodVerifier;
use self::webauthn::WebauthnAuthMethodVerifier;
use auth_method_verifier::AuthMethodVerifier;
use lit_blockchain::resolver::rpc::RpcHealthcheckPoller;

pub const LIT_APP_ID: &str = "lit";

pub enum AuthMethodScope {
    SignAnything = 1,
    SignPersonalMessage = 2,
}

#[instrument(skip_all)]
pub async fn verify_auth_method(
    auth_method: &models::AuthMethod,
    config: Arc<LitConfig>,
    pkp_permissions_contract: Arc<PKPPermissions<Provider<Http>>>,
    webauthn_origin_domain: Option<String>,
) -> error::Result<models::AuthMethodResponse> {
    let auth_method_response = match auth_method.auth_method_type {
        WALLET_AUTH_METHOD_TYPE_ID => {
            let verifier = WalletAuthMethodVerifier { cfg: &config };
            verifier.verify(&auth_method.access_token).await?
        }
        GOOGLE_AUTH_METHOD_TYPE_ID => {
            let verifier = GoogleAuthMethodVerifier {};
            verifier.verify(&auth_method.access_token).await?
        }
        GOOGLE_JWT_AUTH_METHOD_TYPE_ID => {
            let verifier = GoogleJwtAuthMethodVerifier {};
            verifier.verify(&auth_method.access_token).await?
        }
        DISCORD_AUTH_METHOD_TYPE_ID => {
            let verifier = DiscordAuthMethodVerifier {};
            verifier.verify(&auth_method.access_token).await?
        }
        WEBAUTHN_AUTH_METHOD_TYPE_ID => {
            let chain_name = "ethereum".to_string();
            let rpc_url =
                lit_blockchain::resolver::rpc::ENDPOINT_MANAGER.rpc_url(chain_name.clone())?;
            let rpc_provider =
                Arc::new(Provider::<Http>::try_from(rpc_url).map_err(|e| {
                    unexpected_err(e, Some("failed to create provider".to_string()))
                })?);
            let webauthn_origin_domain = webauthn_origin_domain
                .expect_or_err("Webauthn origin domain is not set")
                .map_err(|e| validation_err(e, None))?;

            let verifier = WebauthnAuthMethodVerifier::new(
                config,
                chain_name.clone(),
                pkp_permissions_contract,
                webauthn_origin_domain,
                rpc_provider.clone(),
                None,
            );
            verifier.verify(&auth_method.access_token).await?
        }
        STYTCH_JWT_AUTH_METHOD_TYPE_ID => {
            let verifier = StytchJWTAuthMethodVerifier { factor: None };
            verifier.verify(&auth_method.access_token).await?
        }
        APPLE_JWT_AUTH_METHOD_TYPE_ID => {
            let verifier = AppleJwtAuthMethodVerifier {};
            verifier.verify(&auth_method.access_token).await?
        }

        STYTCH_JWT_AUTH_FACTOR_EMAIL_OTP => {
            let verifier = StytchJWTAuthMethodVerifier {
                factor: Some("email".to_string()),
            };
            verifier.verify(&auth_method.access_token).await?
        }

        STYTCH_JWT_AUTH_FACTOR_SMS_OTP => {
            let verifier = StytchJWTAuthMethodVerifier {
                factor: Some("sms".to_string()),
            };
            verifier.verify(&auth_method.access_token).await?
        }

        STYTCH_JWT_AUTH_FACTOR_WHATS_APP_OTP => {
            let verifier = StytchJWTAuthMethodVerifier {
                factor: Some("whatsApp".to_string()),
            };
            verifier.verify(&auth_method.access_token).await?
        }

        STYTCH_JWT_AUTH_FACTOR_TOTP => {
            let verifier = StytchJWTAuthMethodVerifier {
                factor: Some("totp".to_string()),
            };
            verifier.verify(&auth_method.access_token).await?
        }
        _ => {
            return Err(validation_err(
                format!(
                    "Unsupported auth method type: {}",
                    auth_method.auth_method_type
                ),
                None,
            ))
        }
    };
    Ok(auth_method_response)
}

#[instrument(skip_all)]
pub async fn verify_auth_method_for_claim(
    auth_method: &models::AuthMethod,
    config: Arc<LitConfig>,
    pkp_permissions_contract: Arc<PKPPermissions<Provider<Http>>>,
    webauthn_origin_domain: Option<String>,
    webauthn_credential_public_key: Option<String>,
) -> error::Result<models::AuthMethodResponse> {
    let chain_name = config
        .blockchain_chain_name()
        .map_err(|e| unexpected_err(e, Some("Unable to get chain name".into())))?;

    let rpc_url = lit_blockchain::resolver::rpc::ENDPOINT_MANAGER.rpc_url(chain_name.clone())?;
    let rpc_provider = Arc::new(
        Provider::<Http>::try_from(rpc_url)
            .map_err(|e| unexpected_err(e, Some("failed to create provider".to_string())))?,
    );

    let auth_method_response = match auth_method.auth_method_type {
        WALLET_AUTH_METHOD_TYPE_ID => {
            let verifier = WalletAuthMethodVerifier { cfg: &config };
            verifier.verify(&auth_method.access_token).await?
        }
        GOOGLE_AUTH_METHOD_TYPE_ID => {
            let verifier = GoogleAuthMethodVerifier {};
            verifier.verify(&auth_method.access_token).await?
        }
        GOOGLE_JWT_AUTH_METHOD_TYPE_ID => {
            let verifier = GoogleJwtAuthMethodVerifier {};
            verifier.verify(&auth_method.access_token).await?
        }
        DISCORD_AUTH_METHOD_TYPE_ID => {
            let verifier = DiscordAuthMethodVerifier {};
            verifier.verify(&auth_method.access_token).await?
        }
        WEBAUTHN_AUTH_METHOD_TYPE_ID => {
            let webauthn_origin_domain = webauthn_origin_domain
                .expect_or_err("Webauthn origin domain is not set")
                .map_err(|e| validation_err(e, None))?;

            let verifier = WebauthnAuthMethodVerifier::new(
                config,
                chain_name.clone(),
                pkp_permissions_contract,
                webauthn_origin_domain,
                rpc_provider.clone(),
                webauthn_credential_public_key,
            );
            verifier.verify(&auth_method.access_token).await?
        }

        APPLE_JWT_AUTH_METHOD_TYPE_ID => {
            let verifier = AppleJwtAuthMethodVerifier {};
            verifier.verify(&auth_method.access_token).await?
        }

        STYTCH_JWT_AUTH_METHOD_TYPE_ID => {
            let verifier = StytchJWTAuthMethodVerifier { factor: None };
            verifier.verify(&auth_method.access_token).await?
        }

        STYTCH_JWT_AUTH_FACTOR_EMAIL_OTP => {
            let verifier = StytchJWTAuthMethodVerifier {
                factor: Some("email".to_string()),
            };
            verifier.verify(&auth_method.access_token).await?
        }

        STYTCH_JWT_AUTH_FACTOR_SMS_OTP => {
            let verifier = StytchJWTAuthMethodVerifier {
                factor: Some("sms".to_string()),
            };
            verifier.verify(&auth_method.access_token).await?
        }

        STYTCH_JWT_AUTH_FACTOR_WHATS_APP_OTP => {
            let verifier = StytchJWTAuthMethodVerifier {
                factor: Some("whatsApp".to_string()),
            };
            verifier.verify(&auth_method.access_token).await?
        }

        STYTCH_JWT_AUTH_FACTOR_TOTP => {
            let verifier = StytchJWTAuthMethodVerifier {
                factor: Some("totp".to_string()),
            };
            verifier.verify(&auth_method.access_token).await?
        }

        _ => {
            return Err(validation_err(
                format!(
                    "Unsupported auth method type: {}",
                    auth_method.auth_method_type
                ),
                None,
            ))
        }
    };
    Ok(auth_method_response)
}

pub fn serialize_auth_context_for_checking_against_contract_data(
    auth_method: &models::AuthMethodResponse,
) -> error::Result<Vec<u8>> {
    if auth_method.auth_method_type == DISCORD_AUTH_METHOD_TYPE_ID
        || auth_method.auth_method_type == GOOGLE_AUTH_METHOD_TYPE_ID
        || auth_method.auth_method_type == GOOGLE_JWT_AUTH_METHOD_TYPE_ID
        || auth_method.auth_method_type == WEBAUTHN_AUTH_METHOD_TYPE_ID
        || auth_method.auth_method_type == APPLE_JWT_AUTH_METHOD_TYPE_ID
        || auth_method.auth_method_type == STYTCH_JWT_AUTH_METHOD_TYPE_ID
        || auth_method.auth_method_type == WALLET_AUTH_METHOD_TYPE_ID
        || auth_method.auth_method_type == STYTCH_JWT_AUTH_FACTOR_EMAIL_OTP
        || auth_method.auth_method_type == STYTCH_JWT_AUTH_FACTOR_SMS_OTP
        || auth_method.auth_method_type == STYTCH_JWT_AUTH_FACTOR_WHATS_APP_OTP
        || auth_method.auth_method_type == STYTCH_JWT_AUTH_FACTOR_TOTP
    {
        let serialized = format!("{}:{}", auth_method.user_id, auth_method.app_id);
        trace!("Serializing auth context: {}", serialized);
        let as_bytes = serialized.as_bytes().to_vec();

        // hash with keccack256
        let hashed = keccak256(as_bytes);
        Ok(hashed.to_vec())
    } else {
        Err(validation_err(
            format!(
                "Unsupported auth method type: {}",
                auth_method.auth_method_type
            ),
            None,
        ))
    }
}

pub fn get_user_wallet_auth_method_from_address(address: &str) -> error::Result<Vec<u8>> {
    let serialized = format!("{}:{}", address, LIT_APP_ID);
    let as_bytes = serialized.as_bytes().to_vec();
    let hashed = keccak256(as_bytes);

    Ok(hashed.to_vec())
}

#[instrument(name = "check_pkp_auth", skip_all)]
#[allow(clippy::too_many_arguments)]
pub async fn check_pkp_auth(
    ipfs_id_option: Option<String>,
    auth_sig: Option<JsonAuthSig>,
    pkp_pubkey: String,
    auth_context: models::AuthContext,
    cfg: &LitConfig,
    required_scopes: &[usize],
    bls_root_pubkey: &String,
) -> Result<bool, Error> {
    use std::io::{Error, ErrorKind};

    debug!("auth_context- {:?}", auth_context);

    debug!(
        "Checking PKP for ipfs_id {:?} and pkp_pubkey {:?} for scopes {:?}",
        ipfs_id_option, pkp_pubkey, required_scopes
    );

    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;

    let token_id = U256::from(&keccak256(encoding::hex_to_bytes(&pkp_pubkey)?));

    trace!("token_id: {}", token_id.encode_hex());

    let pkp_permissions_contract = resolver.pkp_permissions_contract(cfg).await?;
    let pkp_nft_contract = resolver.pkp_nft_contract(cfg).await?;

    let permitted_auth_methods: Vec<pkp_permissions::AuthMethod> = pkp_permissions_contract
        .get_permitted_auth_methods(token_id)
        .call()
        .await
        .map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeUnknownError,
                Some("Error getting permitted auth methods".to_string()),
            )
        })?;

    debug!("Permitted Auth Methods- {:?}", permitted_auth_methods);

    let owner_address = pkp_nft_contract
        .owner_of(token_id)
        .call()
        .await
        .map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeContractResolverConversionFailed,
                Some("Error getting owner of PKP".to_string()),
            )
        })?;
    debug!("Owner Address: {:?}", owner_address);

    // check if any of the AuthMethods provided are valid
    for auth_method in auth_context.auth_method_contexts {
        debug!("Checking auth method: {:?}", auth_method);
        let auth_method_type = U256::from(auth_method.auth_method_type);
        let serialized_user_id = serialize_auth_context_for_checking_against_contract_data(
            &auth_method,
        )
        .map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeContractResolverConversionFailed,
                Some("Error serializing auth context".into()),
            )
        })?;
        let serialized_user_id = Bytes::from(serialized_user_id);

        debug!("Checking if permitted auth methods contains for auth_method_type: {:?}, serialized_user_id: {:?}, token_id: {:?}",
        auth_method_type, encoding::bytes_to_hex(&serialized_user_id), token_id.encode_hex());

        let auth_method_is_permitted = permitted_auth_methods.iter().any(|permitted_auth_method| {
            permitted_auth_method.auth_method_type == auth_method_type
                && permitted_auth_method.id == serialized_user_id
        });
        debug!("Is Auth method permitted? {:?}", auth_method_is_permitted);

        match auth_method_is_permitted {
            true => {
                let has_scopes = check_scopes(
                    required_scopes,
                    pkp_permissions_contract.clone(),
                    token_id,
                    auth_method_type,
                    serialized_user_id,
                )
                .await?;

                if has_scopes {
                    return Ok(true);
                }
            }
            false => {
                debug!(
                    "AuthMethod not permitted for token id: {:?}- {:?}",
                    token_id.encode_hex(),
                    auth_method
                );
            }
        };

        let owner_string_address = format!("0x{}", hex::encode(owner_address.as_bytes()));

        // Wallet address
        if auth_method_type == U256::from(1) {
            debug!("Checking for Eth Wallet AuthMethod");

            let user_wallet_address = encoding::string_to_eth_address(auth_method.user_id.clone())?;

            let user_wallet_address_string = to_checksum(&user_wallet_address, None); // Because the address is the auth_method.user_id may not be in the checked sum format

            match is_any_user_address_format_permitted(
                user_wallet_address_string,
                &owner_address,
                required_scopes,
                &permitted_auth_methods,
                pkp_permissions_contract.clone(),
                token_id,
            )
            .await?
            {
                true => return Ok(true),
                false => debug!("User address not PKP owner and not permitted either"),
            };
        }
    }

    // check if any of the Lit actions in AuthContext are valid
    for ipfs_id in auth_context.action_ipfs_id_stack {
        let lit_action_auth_method_type = U256::from(2); // AuthMethodType::Action
        let ipfs_id_bytes = encoding::ipfs_cid_to_bytes(ipfs_id.clone())?;

        debug!(
            "Checking if permitted lit actions contains lit action with token_id {} and ipfs_id_bytes {}",
            token_id.encode_hex(),
            ipfs_id_bytes.clone().encode_hex()
        );

        let auth_method_is_permitted = permitted_auth_methods.iter().any(|permitted_auth_method| {
            permitted_auth_method.auth_method_type == lit_action_auth_method_type // AuthMethodType::Action
                && permitted_auth_method.id == ipfs_id_bytes.to_vec()
        });

        match auth_method_is_permitted {
            true => {
                let has_scopes = check_scopes(
                    required_scopes,
                    pkp_permissions_contract.clone(),
                    token_id,
                    lit_action_auth_method_type,
                    Bytes::from(ipfs_id_bytes.to_vec()),
                )
                .await?;

                if has_scopes {
                    return Ok(true);
                }
            }
            false => {
                debug!(
                    "Lit Action not permitted for token id: {:?}- {:?}",
                    token_id.encode_hex(),
                    ipfs_id
                );
            }
        };
    }

    #[cfg(feature = "lit-actions")]
    if let Some(ipfs_id) = ipfs_id_option {
        let lit_action_auth_method_type = U256::from(2); // AuthMethodType::Action
        let ipfs_id_bytes = encoding::ipfs_cid_to_bytes(ipfs_id.clone())?;

        debug!(
            "Checking if permitted auth methods contains lit action with token_id {} and ipfs_id_bytes {}",
            token_id.encode_hex(),
            ipfs_id_bytes.clone().encode_hex()
        );

        let auth_method_is_permitted = permitted_auth_methods.iter().any(|permitted_auth_method| {
            permitted_auth_method.auth_method_type == lit_action_auth_method_type // AuthMethodType::Action
                && permitted_auth_method.id == ipfs_id_bytes.to_vec()
        });

        match auth_method_is_permitted {
            true => {
                let has_scopes = check_scopes(
                    required_scopes,
                    pkp_permissions_contract.clone(),
                    token_id,
                    lit_action_auth_method_type,
                    Bytes::from(ipfs_id_bytes.to_vec()),
                )
                .await?;

                if has_scopes {
                    return Ok(true);
                }
            }
            false => {
                debug!(
                    "Lit Action not permitted for token id: {:?}- {:?}",
                    token_id.encode_hex(),
                    ipfs_id
                );
            }
        };
    }

    if let Some(auth_sig) = auth_sig {
        let user_wallet_address_string = auth_sig.user_address(bls_root_pubkey).await?; // checked sum

        debug!(
            "Checking if permitted auth methods contains address for token_id {} and auth_sig.address {:?}",
            token_id.encode_hex(),
            user_wallet_address_string
        );

        match is_any_user_address_format_permitted(
            user_wallet_address_string,
            &owner_address,
            required_scopes,
            &permitted_auth_methods,
            pkp_permissions_contract.clone(),
            token_id,
        )
        .await?
        {
            true => return Ok(true),
            false => debug!("User address not PKP owner and not permitted either"),
        };

        debug!(
            "AuthSig not permitted for token id: {:?}- {:?}",
            token_id.encode_hex(),
            auth_sig
        );
    }

    return Err(validation_err_code(
        Error::new(
            ErrorKind::Other,
            format!(
                "None of the AuthMethods, AuthSig or Lit Actions meet the requires scope {:?}.",
                required_scopes
            ),
        ),
        EC::NodeAuthSigScopeTooLimited,
        None,
    ));
}

// We need this due to an issue in the SDK which allows user to permit the following 3 formats:
// - Bare user address
// - User address suffixed with ":lit"
//  - User address is lower case
//  - User address is checked sum i.e. mixed case
async fn is_any_user_address_format_permitted(
    user_wallet_address_string: String,
    owner_address: &H160,
    required_scopes: &[usize],
    permitted_auth_methods: &Vec<pkp_permissions::AuthMethod>,
    pkp_permissions_contract: PKPPermissions<Provider<Http>>,
    token_id: U256,
) -> Result<bool, Error> {
    debug!(
        "user_wallet_address_string- {:?}",
        user_wallet_address_string
    );
    let user_wallet_address = encoding::string_to_eth_address(user_wallet_address_string.clone())?; // lower case

    // is this address the owner of the PKP?  if so, we don't need to check for scopes.
    // also, this won't show up as an auth method
    // the PKP owner has root access
    if owner_address == &user_wallet_address {
        return Ok(true);
    }

    let wallet_address_bytes = Bytes::from(user_wallet_address.as_bytes().to_vec());

    // Have to check for the encoded authMethod as the permitted AuthMethod on-chain may contain ":lit" suffix
    // Check for both lowercase & checkedsum (mixedcase) as the permitted address hash will be different for both
    let lowercase_wallet_auth_method_with_app_id =
        get_user_wallet_auth_method_from_address(&user_wallet_address_string.to_lowercase())?;
    let lowercase_wallet_auth_method_with_app_id_bytes =
        Bytes::from(lowercase_wallet_auth_method_with_app_id);

    let checkedsum_wallet_auth_method_with_app_id =
        get_user_wallet_auth_method_from_address(&user_wallet_address_string)?;
    let checkedsum_wallet_auth_method_with_app_id_bytes =
        Bytes::from(checkedsum_wallet_auth_method_with_app_id);

    let address_auth_method_type = U256::from(1); // AuthMethodType::Address

    for auth_method in permitted_auth_methods {
        debug!("Checking auth method: {:?}", auth_method);
        // if the auth method type is 2 aka an IPFS cid, print this too
        if auth_method.auth_method_type == U256::from(2) {
            // encode as a base58 ipfs cid
            trace!(
                "IPFS cid of auth method: {:?}",
                bytes_to_ipfs_cid(auth_method.id.clone())
            );
        }
        let is_address_permitted_auth_method_type =
            auth_method.auth_method_type == address_auth_method_type;
        if !is_address_permitted_auth_method_type {
            continue;
        }

        if auth_method.id == wallet_address_bytes
            || auth_method.id == lowercase_wallet_auth_method_with_app_id_bytes
            || auth_method.id == checkedsum_wallet_auth_method_with_app_id_bytes
        {
            let has_scopes = check_scopes(
                required_scopes,
                pkp_permissions_contract.clone(),
                token_id,
                address_auth_method_type,
                auth_method.id.clone(),
            )
            .await?;

            if has_scopes {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

async fn check_scopes(
    required_scopes: &[usize],
    contract: PKPPermissions<Provider<Http>>,
    token_id: U256,
    auth_method_type: U256,
    serialized_user_id: Bytes,
) -> Result<bool, Error> {
    // When no scope is required, return immediately.
    if required_scopes.is_empty() {
        return Ok(true);
    }

    // this returns an array with 32 entries, with each entry being a bool indicating if the scope is permitted
    let permitted_scopes = contract
        .get_permitted_auth_method_scopes(
            token_id,
            auth_method_type,
            serialized_user_id.clone(),
            U256::from(32),
        )
        .call()
        .await
        .map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeContractResolverConversionFailed,
                Some("Error getting permitted auth method scopes".to_string()),
            )
        })?;
    debug!(
        "permitted_scopes from the chain for the auth method: {:?}",
        permitted_scopes
    );

    let all_scopes_permitted = required_scopes.iter().all(|scope| {
        let permitted_scope = permitted_scopes.get(*scope).unwrap_or(&false);

        // the weird || here is to allow the SignPersonalMessage scope (2) to be used if the SignAnything scope (1) is also permitted, since if they can sign anything, they can sign a personal message.  So even if (2) is required, but not present, we can still sign if (1) is present
        *permitted_scope
            || (*scope == AuthMethodScope::SignPersonalMessage as usize
                && *permitted_scopes
                    .get(AuthMethodScope::SignAnything as usize)
                    .unwrap_or(&false))
    });

    Ok(all_scopes_permitted)
}

#[cfg(test)]
mod tests {
    use crate::models::{AuthContextCache, AuthContextCacheExpiry, AuthMethodResponse};
    use std::time::SystemTime;

    #[tokio::test]
    async fn should_remove_expired_entries_with_10_sec_ellapse() {
        let auth_context_cache = std::sync::Arc::new(AuthContextCache {
            auth_contexts: moka::future::Cache::builder()
                .max_capacity(100_000)
                .expire_after(AuthContextCacheExpiry)
                .build(),
        });
        let mut initial_auth_responses: Vec<AuthMethodResponse> = vec![];
        for i in 0..10 {
            initial_auth_responses.push(AuthMethodResponse {
                user_id: "abcd123".to_string(),
                app_id: "456xyz".to_string(),
                auth_method_type: i,
                last_retrieved_at: SystemTime::now(),
                expiration: 8,
                used_for_sign_session_key_request: true,
            });
        }

        for response in initial_auth_responses.iter() {
            auth_context_cache
                .auth_contexts
                .insert(
                    format!("{}-{}", response.user_id, response.auth_method_type),
                    response.clone(),
                )
                .await;
        }

        tokio::time::sleep(std::time::Duration::from_secs(11)).await;
        assert_eq!(auth_context_cache.auth_contexts.weighted_size(), 0);
    }
}
