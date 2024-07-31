use crate::pkp::auth::LIT_APP_ID;
use crate::siwe_db::db::{last_n_blocks, retrieve_and_store_blockhash};
use crate::siwe_db::rpc::{fetch_block_from_hash, fetch_latest_blockhash};
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::Bytes;
use lit_core::error::Unexpected;
use std::time::SystemTime;
use webauthn_rs_proto::PublicKeyCredential;

use crate::error::validation_err;
use crate::models;
use crate::{
    error::{parser_err, unexpected_err, validation_err_code, Result, EC},
    utils::encoding,
};
use crate::{pkp::auth::WEBAUTHN_AUTH_METHOD_TYPE_ID, utils::cose_keys::decode_cbor_cose_key};

use ethers::types::U256;
use ethers::utils::keccak256;
use futures::TryFutureExt;
use lit_blockchain::contracts::pkp_permissions::PKPPermissions;
use lit_core::config::LitConfig;
use std::sync::Arc;
use tracing::debug;
use webauthn_rs_proto::{CollectedClientData, RegisteredExtensions};

use webauthn_rs::prelude::{AuthenticationState, Base64UrlSafeData, ParsedAttestation};
use webauthn_rs_core::proto::Credential;

use crate::config::LitNodeConfig;
use crate::error::{conversion_err, unexpected_err_code};

use super::auth_method_verifier::AuthMethodVerifier;

pub struct WebauthnAuthMethodVerifier<M: Middleware> {
    config: Arc<LitConfig>,
    chain_name: String,
    pkp_permissions_contract: Arc<PKPPermissions<Provider<Http>>>,
    origin_domain: String,
    rpc_provider: Arc<M>,
    public_key: Option<String>,
}

#[async_trait::async_trait]
impl<M: Middleware + 'static> AuthMethodVerifier for WebauthnAuthMethodVerifier<M> {
    async fn verify(&self, access_token: &str) -> Result<models::AuthMethodResponse> {
        debug!("Verifying webauthn signature and credentials before using public keys.");

        // Deserialize the credential from the request body.
        let credential =
            serde_json::from_str::<PublicKeyCredential>(access_token).map_err(|e| {
                conversion_err(
                    e,
                    Some("Unable to deserialize credential from request body".into()),
                )
            })?;

        self.verify_webauthn_authentication(&credential).await
    }
}

impl<M: Middleware + 'static> WebauthnAuthMethodVerifier<M> {
    pub fn new(
        config: Arc<LitConfig>,
        chain_name: String,
        pkp_permissions_contract: Arc<PKPPermissions<Provider<Http>>>,
        origin_domain: String,
        rpc_provider: Arc<M>,
        public_key: Option<String>,
    ) -> Self {
        Self {
            config,
            chain_name,
            pkp_permissions_contract,
            origin_domain,
            rpc_provider,
            public_key,
        }
    }

    async fn verify_webauthn_authentication(
        &self,
        credential: &PublicKeyCredential,
    ) -> Result<models::AuthMethodResponse> {
        // Get the blockhash that the user signed against from within the request body.
        let challenge = get_challenge_from_credential(credential).map_err(|e| {
            unexpected_err(e, Some("Unable to get challenge from credential".into()))
        })?;
        let blockhash = format!("0x{}", encoding::bytes_to_hex(challenge.0.as_slice()));

        // Check whether this blockhash is a valid blockhash.
        let valid_blockhash = check_blockhash_challenge(
            blockhash.as_ref(),
            self.chain_name.clone(),
            self.rpc_provider.clone(),
            self.config.clone(),
        )
        .await
        .map_err(|e| {
            validation_err_code(
                e,
                EC::NodeInvalidWebAuthnChallenge,
                Some("Invalid blockhash used as challenge".into()),
            )
        })?;
        if !valid_blockhash {
            return Err(validation_err("Invalid blockhash used as challenge", None));
        }

        // Get AuthMethod Pubkey from contract given username provided in request
        debug!(
            "Credential Raw ID: {:?} ({:?})",
            credential.raw_id,
            credential.raw_id.to_string()
        );
        let auth_method_id = generate_auth_method_id(credential.raw_id.to_string());

        // Verify WebAuthn credential
        let verify_ok = verify_credential(
            self.config.clone(),
            &self.pkp_permissions_contract,
            credential,
            auth_method_id.clone(),
            self.origin_domain.as_str(),
            self.public_key.clone(),
        )
        .await?;

        if !verify_ok {
            return Err(validation_err("Invalid WebAuthn credential", None));
        }
        let exp = chrono::Utc::now()
            .checked_add_days(chrono::Days::new(7))
            .expect_or_err(
                "Could not add days to inital timestamp, aborting for lack of expiration time",
            )?
            .timestamp();

        Ok(models::AuthMethodResponse {
            user_id: credential.raw_id.to_string(),
            app_id: "lit".to_string(),
            auth_method_type: WEBAUTHN_AUTH_METHOD_TYPE_ID,
            last_retrieved_at: SystemTime::now(),
            expiration: exp,
            used_for_sign_session_key_request: false,
        })
    }
}

fn generate_auth_method_id(credential_raw_id: String) -> Bytes {
    Bytes::from(keccak256(
        format!("{}:{}", credential_raw_id, LIT_APP_ID).into_bytes(),
    ))
}

/// WebAuthn Verification.
///
/// Normally, the credential information is stored server-side. However,
/// here we are storing the credential public key in the contract, so we
/// need to first check that the credential public key in the assertion
/// matches the one in the contract before we resume the rest of the
/// remaining WebAuthn verification process that is relevant.
async fn verify_credential(
    config: Arc<LitConfig>,
    pkp_permissions_contract: &PKPPermissions<Provider<Http>>,
    credential: &PublicKeyCredential,
    auth_method_id: Bytes,
    client_origin: &str,
    public_key: Option<String>,
) -> Result<bool> {
    // initalize with default
    let cbor_encoded_cose_key = match public_key {
        Some(pk) => {
            debug!("Using provided credential public key from request");
            ethers::types::Bytes::from_iter(pk.as_bytes())
        }
        None => {
            debug!("querying chain state for credential public key");
            // Get the credential public key (in COSE format) from the contract.
            let query = pkp_permissions_contract
                .get_user_pubkey_for_auth_method(U256::from(3), auth_method_id);

            query
                .call()
                .map_err(|e| {
                    unexpected_err_code(
                        e,
                        EC::NodeRpcError,
                        Some("Failed to query contract".into()),
                    )
                })
                .await?
        }
    };
    debug!("cbor_encoded_cose_key: {:?}", cbor_encoded_cose_key);

    let allowed_localhost = url::Url::parse("http://localhost:3000")
        .map_err(|e| parser_err(e, Some("Unable to parse localhost URL".into())))?;

    let webauthn_verification = webauthn_rs_core::core::WebauthnCore::new_unsafe_experts_only(
        client_origin,
        client_origin,
        config.webauthn_allowed_origins()?,
        None,
        None,
        None,
    );

    // Decode credentialPublicKey CBOR to COSE
    let (cose_key, public_key_hex) = decode_cbor_cose_key(cbor_encoded_cose_key).map_err(|e| {
        parser_err(
            e,
            Some("Unable to decode credential public key CBOR".into()),
        )
    })?;

    // Here we construct a Credential object from the credential public key
    // obtained from the contract. This object will be used to verify against
    // the assertion signature and payload from the client request. Authenticity
    // comes from verifying chain data (credential public key) against data provided
    // by the client (assertion signature and payload).

    let restored_credential = Credential {
        cred_id: credential.raw_id.clone(),
        cred: cose_key,
        counter: 0,
        transports: None,
        user_verified: false,
        backup_eligible: true,
        backup_state: false,
        registration_policy: webauthn_rs_proto::UserVerificationPolicy::Required,
        extensions: RegisteredExtensions::none(),
        attestation: ParsedAttestation::default(),
        attestation_format: webauthn_rs_core::AttestationFormat::None, // unused during verification.
    };

    let client_challenge = get_challenge_from_credential(credential)
        .map_err(|e| parser_err(e, Some("Unable to get challenge from credential".into())))?;

    let authentication_state = AuthenticationState {
        credentials: vec![restored_credential],
        policy: webauthn_rs_proto::UserVerificationPolicy::Preferred,
        challenge: client_challenge,
        appid: Some(client_origin.to_string()),
        allow_backup_eligible_upgrade: true,
    };

    let auth_res = webauthn_verification
        .authenticate_credential(credential, &authentication_state)
        .map_err(|e| conversion_err(e, Some("Unable to authenticate credential".into())))?;

    Ok(auth_res.user_verified())
}

fn get_challenge_from_credential(credential: &PublicKeyCredential) -> Result<Base64UrlSafeData> {
    let client_data = serde_json::from_slice::<CollectedClientData>(
        credential.response.client_data_json.as_ref(),
    )
    .map_err(|e| conversion_err(e, Some("Unable to parse client data JSON".into())))?;
    Ok(client_data.challenge)
}

#[allow(dead_code)]
fn get_rp_id_from_credential(credential: &PublicKeyCredential) -> Result<String> {
    let client_data = serde_json::from_slice::<CollectedClientData>(
        credential.response.client_data_json.as_ref(),
    )
    .map_err(|e| conversion_err(e, Some("Unable to parse client data JSON".into())))?;

    match client_data.origin.domain() {
        Some(s) => Ok(s.to_string()),
        None => {
            info!("Unable to parse domain from client data origin, using default app id");
            Ok("lit".to_string())
        }
    }
}

/// we accept the last 10 eth block hashes as the challenge
/// this checks that the challenge is one of those 10 hashes.
/// This gives them 120 seconds of grace period because eth block time is 12s.
pub async fn check_blockhash_challenge<M: Middleware + 'static>(
    blockhash_to_check: &str,
    chain_name: String,
    rpc_provider: Arc<M>,
    cfg: Arc<LitConfig>,
) -> Result<bool> {
    // get port from cfg
    let port = cfg.external_port()?;
    let last_10_blocks = match last_n_blocks(port, 10) {
        Ok(b) => {
            if b.is_empty() {
                // grab the last block,
                let block = match fetch_latest_blockhash(rpc_provider.clone()).await {
                    Ok(b) => b,
                    Err(e) => return Err(e),
                };
                vec![block]
            } else {
                b
            }
        }
        Err(e) => {
            // grab the last block,
            error!("Error getting last 10 blocks from DB: {:?}", e);
            let block = match fetch_latest_blockhash(rpc_provider.clone()).await {
                Ok(b) => b,
                Err(e) => return Err(e),
            };
            vec![block]
        }
    };
    if last_10_blocks
        .iter()
        .any(|bh| bh.blockhash == blockhash_to_check)
    {
        return Ok(true);
    }

    let latest_stored_block = last_10_blocks.first().expect("This array isn't empty and we grabbed the first element and it produced this error.  That makes no sense.");

    // maybe the blockhash just got mined, and we haven't seen it yet.
    let retrieved = match retrieve_and_store_blockhash(
        blockhash_to_check.to_owned(),
        port,
        rpc_provider.clone(),
    )
    .await
    {
        Ok(b) => b,
        Err(e) => {
            error!("Error retrieving blockhash from DB: {:?}", e);
            // seems like the DB is broken, let's get the block manually
            fetch_block_from_hash(blockhash_to_check, rpc_provider).await?
        }
    };
    println!("retrieved blockhash: {:?}", retrieved.blockhash);
    println!(
        "latest stored blockhash: {:?}",
        latest_stored_block.blockhash
    );
    println!("retrieved block number: {:?}", retrieved.block_number);
    println!(
        "latest stored block number: {:?}",
        latest_stored_block.block_number
    );
    if retrieved.blockhash == blockhash_to_check
        && retrieved.block_number > latest_stored_block.block_number - 10
    {
        return Ok(true);
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::check_blockhash_challenge;
    use crate::{constants::CHAIN_ETHEREUM, utils::encoding};
    use chrono::Utc;
    use ethers::{
        providers::Provider,
        types::{Block, H256, U256, U64},
    };

    #[tokio::test]
    async fn test_check_blockhash_challenge_failure() {
        let (provider, mock) = Provider::mocked();
        let block_hash = "0x70dd3646979bc3d49af8ad6320d2b03149a72863f8e08f254e54fa8954f59143";
        let block_number = U64::from(100);

        let block: Block<H256> = Block {
            hash: Some(H256::from(
                encoding::bytes_to_zero_padded_32(encoding::hex_to_bytes(block_hash).unwrap())
                    .unwrap(),
            )),
            timestamp: U256::from(Utc::now().timestamp()),
            number: Some(block_number),
            ..Default::default()
        };

        // NOTE: Requests are retrieved back-to-front (WTF right?)
        assert!(mock.push(block.clone()).is_ok());
        assert!(mock.push(block.clone()).is_ok());
        assert!(mock.push(block_number).is_ok());

        let lit_config = std::sync::Arc::new(
            lit_core::config::LitConfigBuilder::default()
                .set_default("default.domain", "127.0.0.1")
                .set_default("default.port", 7470)
                .set_default("lit.env", "dev")
                .build()
                .unwrap(),
        );

        let checked = check_blockhash_challenge(
            "0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae",
            CHAIN_ETHEREUM.to_owned(),
            Arc::new(provider),
            lit_config,
        )
        .await
        .expect("failed to check blockhash challenge");
        assert!(!checked);
    }

    #[tokio::test]
    async fn test_check_blockhash_challenge_success() {
        let (provider, mock) = Provider::mocked();
        let block_hash = "0x70dd3646979bc3d49af8ad6320d2b03149a72863f8e08f254e54fa8954f59143";
        let block_number = U64::from(1000000000000 as u64);
        let block: Block<H256> = Block {
            hash: Some(H256::from(
                encoding::bytes_to_zero_padded_32(encoding::hex_to_bytes(block_hash).unwrap())
                    .unwrap(),
            )),
            timestamp: U256::from(Utc::now().timestamp()),
            number: Some(block_number),
            ..Default::default()
        };

        // NOTE: Requests are retrieved back-to-front (WTF right?)
        assert!(mock.push(block.clone()).is_ok());
        assert!(mock.push(block.clone()).is_ok());
        assert!(mock.push(block_number).is_ok());

        let lit_config = std::sync::Arc::new(
            lit_core::config::LitConfigBuilder::default()
                .set_default("default.domain", "127.0.0.1")
                .set_default("default.port", 7470)
                .set_default("lit.env", "dev")
                .build()
                .unwrap(),
        );

        let checked = check_blockhash_challenge(
            block_hash,
            CHAIN_ETHEREUM.to_owned(),
            Arc::new(provider),
            lit_config,
        )
        .await
        .expect("failed to check blockhash challenge");
        assert!(checked);
    }
}
