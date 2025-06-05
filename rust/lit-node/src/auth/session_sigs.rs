use crate::config::LitNodeConfig;
use crate::error::{conversion_err_code, parser_err_code, validation_err_code, Result, EC};
use crate::models::auth::SessionKeySignedMessage;
use crate::siwe_db::utils::check_expiration_validity;
use lit_core::config::LitConfig;
use std::str::FromStr;
use std::sync::Arc;

use crate::{auth::auth_material::JsonAuthSig, utils::encoding};
use chrono::{DateTime, Utc};
use lit_api_core::config::LitApiConfig;

use super::auth_material::AuthMaterialType;
use super::lit_resource::LitResource;
use super::resources::{
    get_resource_prefix_id_from_type, parse_resource_and_prefix, LitResourceAbility,
};
use super::validators::auth_sig::SessionSigAuthSigValidator;
use super::validators::siwe::SiweValidator;

/// Extracts the resources that this session signature has requested to operate on.
///
/// Note: This function does not validate the outer session sig object.
pub(crate) fn extract_requested_resources_from_session_sig(
    session_sig: &JsonAuthSig,
) -> Result<Vec<Arc<dyn LitResource>>> {
    // Parse session key signed message
    let session_key_signed_message: SessionKeySignedMessage =
        serde_json::from_str(&session_sig.signed_message).map_err(|e| {
            validation_err_code(e, EC::NodeAuthSigSignedMessageConversionError, None)
        })?;

    // Parse each requested resource
    let mut requested_resources: Vec<Arc<dyn LitResource>> = Vec::new();
    for resource_ability_request in session_key_signed_message.resource_ability_requests.iter() {
        let requested_resource = &resource_ability_request.resource;
        requested_resources.push(parse_resource_and_prefix(
            &requested_resource.resource,
            &requested_resource.resource_prefix,
        )?);
    }

    Ok(requested_resources)
}

/// Extracts the first VALID inner auth sig in the session signature capabilities array.
///
/// Note: This function does not validate the outer session sig object.
///
/// TODO: Perhaps we should implement another function that extracts the first valid auth sig that ALSO
/// takes into account the requested resource ability and session capability object.
pub async fn extract_wallet_sig(
    session_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
) -> Result<JsonAuthSig> {
    // Parse session key signed message
    let session_key_signed_message: SessionKeySignedMessage =
        serde_json::from_str(&session_sig.signed_message).map_err(|e| {
            validation_err_code(e, EC::NodeAuthSigSignedMessageConversionError, None)
        })?;

    let valid_inner_auth_sig = validate_capabilities_array_basic(
        &session_key_signed_message.session_key,
        &session_key_signed_message.capabilities,
        bls_root_pubkey,
    )
    .await?;

    Ok(valid_inner_auth_sig.to_owned())
}

#[deprecated(
    note = "This function is deprecated and will be removed once the new session signature verification logic has stabilized. Use `validate_session_sig` instead."
)]
pub fn validate_and_extract_wallet_sig(session_sig: &JsonAuthSig) -> Result<JsonAuthSig> {
    // Validate the session signature
    check_ed25519_auth_sig(session_sig)?;

    // Parse session key signed message
    let session_key_signed_message: SessionKeySignedMessage =
        serde_json::from_str(&session_sig.signed_message).map_err(|e| {
            conversion_err_code(e, EC::NodeAuthSigSignedMessageConversionError, None)
        })?;

    let wallet_sig = session_key_signed_message
        .capabilities
        .last()
        .ok_or_else(|| {
            validation_err_code(
                "Empty capabilites array.",
                EC::NodeSIWECapabilityInvalid,
                None,
            )
            .add_source_to_details()
        })?;

    // parse it and make sure it signed the right key
    let signed_message = wallet_sig
        .signed_message
        .parse::<siwe::Message>()
        .map_err(|e| {
            parser_err_code(
                e,
                EC::NodeWalletSignatureJSONError,
                Some("Error parsing wallet sig signed message".into()),
            )
            .add_source_to_details()
        })?;
    // confirm that the signed session key matches the one we verified above
    let formatted_ed25519_pubkey = format!("lit:session:{}", session_sig.address);
    if signed_message.uri != formatted_ed25519_pubkey {
        return Err(parser_err_code(
            "Signed session key does not match the one we verified above",
            EC::NodeWalletSignatureJSONError,
            None,
        )
        .add_source_to_details());
    }

    Ok(wallet_sig.to_owned())
}

/// Validate that the session signature is valid.
///
/// This function returns the first valid auth sig in the session signature capabilities.
pub(crate) async fn validate_session_sig(
    session_sig: &JsonAuthSig,
    requested_lit_resource_ability: &LitResourceAbility,
    chain: &Option<String>,
    cfg: &LitConfig,
    bls_root_pubkey: &String,
) -> Result<JsonAuthSig> {
    let session_pubkey = session_sig.address.clone();

    // Verify the session key signature
    check_ed25519_auth_sig(session_sig)?;

    // Parse session key signed message
    let session_key_signed_message: SessionKeySignedMessage =
        serde_json::from_str(&session_sig.signed_message).map_err(|e| {
            conversion_err_code(e, EC::NodeAuthSigSignedMessageConversionError, None)
        })?;

    // Validate that the session signed message contains the correctly authorized resourceAbilityRequests.
    validate_session_sig_resource_ability_requests(
        &session_key_signed_message,
        requested_lit_resource_ability,
    )?;

    // Validate time related parameters
    validate_session_sig_time_parameters(
        &session_key_signed_message.issued_at,
        &session_key_signed_message.expiration,
    )?;

    // Validate that node_address matches our node address
    let port = cfg.external_port()?;
    let domain_name = cfg.api_domain()?;
    let our_node_addr = format!("{}:{}", domain_name, port);
    if our_node_addr != prepare_domain_name(&session_key_signed_message.node_address) {
        return Err(validation_err_code(
            format!(
                "Session key node_address {} does not match our node address {}",
                session_key_signed_message.node_address, our_node_addr
            ),
            EC::NodeNotAuthorized,
            None,
        )
        .add_source_to_details());
    }

    // Lastly, check that the resource id is authed in the capabilities
    let valid_auth_sig = validate_capabilities_array(
        &session_pubkey,
        &session_key_signed_message.capabilities,
        requested_lit_resource_ability,
        bls_root_pubkey,
    )
    .await
    .map_err(|e| {
        validation_err_code(
            e,
            EC::NodeResourceIdNotFound,
            Some("Resource id not found in auth_sig capabilities".into()),
        )
        .add_msg_to_details()
    })?;

    Ok(valid_auth_sig.to_owned())
}

/// Strips https:// and http:// from the domain name and replaces localhost with
/// 127.0.0.1
fn prepare_domain_name<T>(domain_name: T) -> String
where
    T: AsRef<str>,
{
    let domain_name = domain_name.as_ref();
    let domain_name = domain_name.trim();
    let domain_name = domain_name.trim_start_matches("https://");
    let domain_name = domain_name.trim_start_matches("http://");

    // Replace `localhost` with `127.0.0.1`
    domain_name.replace("localhost", "127.0.0.1")
}

/// Validate that the session signature time related parameters are valid
fn validate_session_sig_time_parameters<T>(issued_at_str: T, expiration_str: T) -> Result<()>
where
    T: AsRef<str>,
{
    let grace_period_seconds = 60;
    // Validate that issued_at is not in the future (within grace period if so)
    let issued_at = DateTime::parse_from_rfc3339(issued_at_str.as_ref()).map_err(|e| {
        parser_err_code(
            e,
            EC::NodeParserError,
            Some("Could not parse issued_at of session key signed message".into()),
        )
        .add_msg_to_details()
    })?;

    let now = Utc::now();
    if issued_at.timestamp() > now.timestamp() + grace_period_seconds {
        return Err(validation_err_code(
            format!(
                "Session key issued_at {} is in the future beyond the grace period of {} seconds (now is {})",
                issued_at, grace_period_seconds, now
            ),
            EC::NodeIatOutsideGracePeriod,
            None,
        )
        .add_source_to_details());
    }

    let expiration = DateTime::parse_from_rfc3339(expiration_str.as_ref()).map_err(|e| {
        parser_err_code(
            e,
            EC::NodeParserError,
            Some("Could not parse expiration of session key signed message".into()),
        )
        .add_msg_to_details()
    })?;

    // Validate that expires_at is in the future of issue_at irrespective of the user's time drift
    if expiration < issued_at {
        return Err(validation_err_code(
            format!(
                "Session key expiration {} is in behind issue_at which is {}",
                expiration, issued_at
            ),
            EC::NodeExpWrongOrTooLarge,
            None,
        )
        .add_source_to_details());
    }

    // Validate that expires_at is in the future (within the grace period if so)
    if expiration.timestamp() < now.timestamp() - grace_period_seconds {
        return Err(validation_err_code(
            format!(
                "Session key expiration {} is in the past beyond the grace period of {} seconds (now is {})",
                expiration, grace_period_seconds, now
            ),
            EC::NodeExpWrongOrTooLarge,
            None,
        )
        .add_source_to_details());
    }

    check_expiration_validity(issued_at.into(), expiration.into())?;

    Ok(())
}

/// Validate that the signed message specifies a resource in the `resourceAbilityRequests` that is equal
/// or wider than `requested_hashed_resource_id`, ie. `requested_hashed_resource_id` or `*`.
fn validate_session_sig_resource_ability_requests(
    session_key_signed_message: &SessionKeySignedMessage,
    requested_lit_resource_ability: &LitResourceAbility,
) -> Result<()> {
    let (requested_resource_prefix, requested_resource_id) =
        get_resource_prefix_id_from_type(requested_lit_resource_ability.get_resource())?;

    trace!(
        "Looking for resource {:?} in resourceAbilityRequests",
        requested_lit_resource_ability
    );
    trace!(
        "resourceAbilityRequests: {:?}",
        session_key_signed_message.resource_ability_requests
    );

    // Find if corresponding resource in the resourceAbilityRequests
    for authorized_resource_ability_request in &session_key_signed_message.resource_ability_requests
    {
        let authorized_resource_prefix = authorized_resource_ability_request
            .resource
            .resource_prefix
            .to_string();
        let authorized_resource_id = authorized_resource_ability_request
            .resource
            .resource
            .to_string();

        if authorized_resource_prefix == requested_resource_prefix
            && (authorized_resource_id == "*" || authorized_resource_id == requested_resource_id)
        {
            return Ok(());
        }
    }

    Err(validation_err_code(
        format!(
            "Could not find requested resource {:?} in the resourceAbilityRequests",
            requested_lit_resource_ability.get_resource()
        ),
        EC::NodeResourceIdNotFound,
        None,
    ))
}

/// Validate that the session signature capabilities are valid. Currently we
/// are using the `SiweValidator` to validate each capability auth sig.
///
/// This function returns the first valid capability auth sig.
async fn validate_capabilities_array<'a>(
    session_pubkey: &str,
    capabilities: &'a Vec<JsonAuthSig>,
    requested_lit_resource_ability: &LitResourceAbility,
    bls_root_pubkey: &String,
) -> Result<&'a JsonAuthSig> {
    if capabilities.is_empty() {
        return Err(validation_err_code(
            "Empty capabilites array.",
            EC::NodeSIWECapabilityInvalid,
            None,
        )
        .add_source_to_details());
    }

    // Init validator
    let auth_sig_validator: Arc<dyn SessionSigAuthSigValidator> = Arc::new(SiweValidator::new());

    // Validate each capability
    for inner_auth_sig in capabilities {
        let validation_res = auth_sig_validator
            .validate_auth_sig(
                inner_auth_sig,
                session_pubkey,
                requested_lit_resource_ability,
                bls_root_pubkey,
            )
            .await;
        match validation_res {
            Ok(_) => return Ok(inner_auth_sig),
            Err(e) => {
                debug!("When searching for delegation auth sig for a session sig, the capability auth sig validation failed.  This may not be a problem, since we check all capabilities and another one might be valid.  Printing the error here just in case: {:?}", e);
            }
        }
    }

    Err(validation_err_code(
        "Could not find valid capability.",
        EC::NodeSIWECapabilityInvalid,
        None,
    ))
}

async fn validate_capabilities_array_basic<'a>(
    session_pubkey: &str,
    capabilities: &'a Vec<JsonAuthSig>,
    bls_root_pubkey: &String,
) -> Result<&'a JsonAuthSig> {
    if capabilities.is_empty() {
        return Err(validation_err_code(
            "Empty capabilites array.",
            EC::NodeSIWECapabilityInvalid,
            None,
        )
        .add_source_to_details());
    }

    // Init validator
    let auth_sig_validator: Arc<dyn SessionSigAuthSigValidator> = Arc::new(SiweValidator::new());

    // Validate each capability
    for inner_auth_sig in capabilities {
        let validation_res = match inner_auth_sig.auth_material_type() {
            AuthMaterialType::WalletSig => {
                auth_sig_validator
                    .validate_auth_sig_basic(inner_auth_sig, session_pubkey)
                    .await
            }
            AuthMaterialType::BLSNetworkSig => {
                auth_sig_validator
                    .validate_bls_auth_sig_basic(inner_auth_sig, session_pubkey, bls_root_pubkey)
                    .await
            }
            _ => {
                return Err(validation_err_code(
                    "The auth sig material type is not supported when calling validate_capabilities_array_basic() within session sig validation.  Only WalletSig and BLSNetworkSig are supported.",
                    EC::NodeSIWEMessageError,
                    None,
                ));
            }
        };

        match validation_res {
            Ok(_) => return Ok(inner_auth_sig),
            Err(e) => {
                warn!("Capability auth sig validation failed: {:?}", e);
            }
        }
    }

    Err(validation_err_code(
        "Could not find valid capability.",
        EC::NodeSIWECapabilityInvalid,
        None,
    ))
}

/// Verifies the signature against the signed message payload.
pub fn check_ed25519_auth_sig(auth_sig: &JsonAuthSig) -> Result<()> {
    let pk_bytes = encoding::hex_to_bytes(&auth_sig.address)
        .map_err(|e| conversion_err_code(e, EC::NodeAuthSigAddressConversionError, None))?;
    let pk_slice = <[u8; 32]>::try_from(&pk_bytes[..])
        .map_err(|e| conversion_err_code(e, EC::NodeAuthSigAddressConversionError, None))?;
    let public_key = ed25519_dalek::VerifyingKey::from_bytes(&pk_slice)
        .map_err(|e| validation_err_code(e, EC::NodeAuthSigSessionKeyConversionError, None))?;
    let message = auth_sig.signed_message.as_bytes();
    let signature = ed25519_dalek::Signature::from_str(&auth_sig.sig)
        .map_err(|e| validation_err_code(e, EC::NodeAuthSigSignatureConversionError, None))?;
    public_key.verify_strict(message, &signature).map_err(|e| {
        validation_err_code(
            e,
            EC::NodeInvalidAuthSigSessionKeySignature,
            Some("Session key signature is not valid".into()),
        )
        .add_source_to_details()
    })
}

#[cfg(test)]
mod validate_session_sig_tests {
    use chrono::{Duration, Utc};
    use ethers::signers::{LocalWallet, Signer as WalletSigner};
    use ethers::types::H256;
    use lit_core::error::Kind;

    use ed25519_dalek::{Signature, Signer};
    use ethers::prelude::rand::rngs::OsRng as EthersOsRng;
    use rand::rngs::OsRng;
    use siwe::Message;

    use crate::auth::auth_material::JsonAuthSig;
    use crate::auth::lit_resource::LitResource;
    use crate::auth::resources::AccessControlConditionResource;
    use crate::auth::session_sigs::validate_session_sig;
    use crate::error::EC;
    use crate::models::auth::{
        LitAbility, LitResourceAbilityRequest, LitResourceAbilityRequestResource,
        SessionKeySignedMessage,
    };
    use crate::tests::common::{get_test_config, get_test_config_with_key};
    use crate::utils::encoding;

    #[tokio::test]
    async fn test_validate_invalid_session_sig() {
        let lit_config = get_test_config();

        let session_sig = JsonAuthSig::new(
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
            "jkl".to_string(),
            None,
        );

        let requested_lit_resource_ability =
            AccessControlConditionResource::new("blah".into()).decrypt_ability();

        let validate = validate_session_sig(
            &session_sig,
            &requested_lit_resource_ability,
            &None,
            &lit_config,
            &"".to_string(),
        )
        .await;
        assert!(validate.is_err());
        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeAuthSigAddressConversionError, false));
        assert!(err.is_kind(Kind::Conversion, false));
    }

    #[tokio::test]
    async fn test_validate_invalid_session_sig_verify() {
        let lit_config = get_test_config();

        // Generate ed25519 keypair for signing.
        let mut csprng = OsRng {};
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let message: &[u8] = b"This is a test of the tsunami alert system.";
        let signature: Signature = signing_key.sign(message);

        let session_sig = JsonAuthSig::new(
            signature.to_string(),
            "ghi".to_string(),
            "abc".to_string(),
            "e76233cdd5483d674020cee626bdecfee6cf9d02b2bffa31b75b91c0ec04a09f".to_string(),
            None,
        );

        let requested_lit_resource_ability =
            AccessControlConditionResource::new("blah".into()).decrypt_ability();

        let validate = validate_session_sig(
            &session_sig,
            &requested_lit_resource_ability,
            &None,
            &lit_config,
            &"".to_string(),
        )
        .await;
        assert!(validate.is_err());
        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeInvalidAuthSigSessionKeySignature, false));
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err
            .to_string()
            .contains("Session key signature is not valid"));
    }

    #[tokio::test]
    async fn test_validate_invalid_signed_message() {
        let lit_config = get_test_config();

        // Generate ed25519 keypair for signing.
        let mut csprng = OsRng {};
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let message = "This is a test of the tsunami alert system.".to_string();
        let signature: Signature = signing_key.sign(message.as_bytes());

        let session_sig = JsonAuthSig::new(
            signature.to_string(),
            "ghi".to_string(),
            message,
            encoding::bytes_to_hex(verifying_key.to_bytes()),
            None,
        );

        let requested_lit_resource_ability =
            AccessControlConditionResource::new("blah".into()).decrypt_ability();

        let validate = validate_session_sig(
            &session_sig,
            &requested_lit_resource_ability,
            &None,
            &lit_config,
            &"".to_string(),
        )
        .await;
        assert!(validate.is_err());
        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeAuthSigSignedMessageConversionError, false));
        assert!(err.is_kind(Kind::Conversion, false));
    }

    #[tokio::test]
    async fn test_validate_invalid_resource_ability_requests() {
        let lit_config = get_test_config();

        // Generate ed25519 keypair for signing.
        let mut csprng = OsRng {};
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let session_key_signed_message = SessionKeySignedMessage {
            session_key: encoding::bytes_to_hex(verifying_key.to_bytes()),
            resource_ability_requests: vec![],
            capabilities: vec![],
            issued_at: "2021-01-01T00:00:00Z".to_string(),
            expiration: "2021-01-01T00:00:00Z".to_string(),
            node_address: "abc".to_string(),
        };

        // serialize to JSON string
        let message = serde_json::to_string(&session_key_signed_message).unwrap();
        let signature: Signature = signing_key.sign(message.as_bytes());

        let session_sig = JsonAuthSig::new(
            signature.to_string(),
            "ghi".to_string(),
            message,
            encoding::bytes_to_hex(verifying_key.to_bytes()),
            None,
        );

        let requested_lit_resource_ability =
            AccessControlConditionResource::new("blah".into()).decrypt_ability();

        let validate = validate_session_sig(
            &session_sig,
            &requested_lit_resource_ability,
            &None,
            &lit_config,
            &"".to_string(),
        )
        .await;
        assert!(validate.is_err());
        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeResourceIdNotFound, false));
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err
            .to_string()
            .contains("Could not find requested resource"));
    }

    #[tokio::test]
    async fn test_validate_invalid_time_parameters() {
        let lit_config = get_test_config();

        // Generate ed25519 keypair for signing.
        let mut csprng = OsRng {};
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let session_key_signed_message = SessionKeySignedMessage {
            session_key: encoding::bytes_to_hex(verifying_key.to_bytes()),
            resource_ability_requests: vec![LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "123".into(),
                    resource_prefix: "lit-accesscontrolcondition".into(),
                },
                ability: LitAbility::AccessControlConditionDecryption.to_string(),
            }],
            capabilities: vec![],
            issued_at: "2021-01-01T00:00:00Z".to_string(),
            expiration: "2021-01-01T00:00:00Z".to_string(),
            node_address: "abc".to_string(),
        };

        // serialize to JSON string
        let message = serde_json::to_string(&session_key_signed_message).unwrap();
        let signature: Signature = signing_key.sign(message.as_bytes());

        let session_sig = JsonAuthSig::new(
            signature.to_string(),
            "ghi".to_string(),
            message,
            encoding::bytes_to_hex(verifying_key.to_bytes()),
            None,
        );

        let requested_lit_resource_ability =
            AccessControlConditionResource::new("123".into()).decrypt_ability();

        let validate = validate_session_sig(
            &session_sig,
            &requested_lit_resource_ability,
            &None,
            &lit_config,
            &"".to_string(),
        )
        .await;
        assert!(validate.is_err());
        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeExpWrongOrTooLarge, false));
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err
            .to_string()
            .contains("is in the past beyond the grace period"));
    }

    #[tokio::test]
    async fn test_validate_invalid_node_address() {
        let lit_config = get_test_config_with_key(Some("node".into()));

        // Generate ed25519 keypair for signing.
        let mut csprng = OsRng {};
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.fZ");
        let expiration = (Utc::now() + Duration::days(1)).format("%Y-%m-%dT%H:%M:%S%.fZ");

        let session_key_signed_message = SessionKeySignedMessage {
            session_key: encoding::bytes_to_hex(verifying_key.to_bytes()),
            resource_ability_requests: vec![LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "123".into(),
                    resource_prefix: "lit-accesscontrolcondition".into(),
                },
                ability: LitAbility::AccessControlConditionDecryption.to_string(),
            }],
            capabilities: vec![],
            issued_at: now.to_string(),
            expiration: expiration.to_string(),
            node_address: "abc".to_string(),
        };

        // serialize to JSON string
        let message = serde_json::to_string(&session_key_signed_message).unwrap();
        let signature: Signature = signing_key.sign(message.as_bytes());

        let session_sig = JsonAuthSig::new(
            signature.to_string(),
            "ghi".to_string(),
            message,
            encoding::bytes_to_hex(verifying_key.to_bytes()),
            None,
        );

        let requested_lit_resource_ability =
            AccessControlConditionResource::new("123".into()).decrypt_ability();

        let validate = validate_session_sig(
            &session_sig,
            &requested_lit_resource_ability,
            &None,
            &lit_config,
            &"".to_string(),
        )
        .await;
        assert!(validate.is_err());
        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeNotAuthorized, false));
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err.to_string().contains("does not match our node address"));
    }

    #[tokio::test]
    async fn test_validate_invalid_capabilities_array() {
        let lit_config = get_test_config_with_key(Some("node".into()));

        // Generate ed25519 keypair for signing.
        let mut csprng = OsRng {};
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.fZ");
        let expiration = (Utc::now() + Duration::days(1)).format("%Y-%m-%dT%H:%M:%S%.fZ");

        let session_key_signed_message = SessionKeySignedMessage {
            session_key: encoding::bytes_to_hex(verifying_key.to_bytes()),
            resource_ability_requests: vec![LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "123".into(),
                    resource_prefix: "lit-accesscontrolcondition".into(),
                },
                ability: LitAbility::AccessControlConditionDecryption.to_string(),
            }],
            capabilities: vec![],
            issued_at: now.to_string(),
            expiration: expiration.to_string(),
            node_address: "localhost:7470".to_string(),
        };

        // serialize to JSON string
        let message = serde_json::to_string(&session_key_signed_message).unwrap();
        let signature: Signature = signing_key.sign(message.as_bytes());

        let session_sig = JsonAuthSig::new(
            signature.to_string(),
            "ghi".to_string(),
            message,
            encoding::bytes_to_hex(verifying_key.to_bytes()),
            None,
        );

        let requested_lit_resource_ability =
            AccessControlConditionResource::new("123".into()).decrypt_ability();

        let validate = validate_session_sig(
            &session_sig,
            &requested_lit_resource_ability,
            &None,
            &lit_config,
            &"".to_string(),
        )
        .await;
        assert!(validate.is_err());
        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeResourceIdNotFound, false));
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err
            .to_string()
            .contains("Resource id not found in auth_sig capabilities"));
    }

    #[tokio::test]
    async fn test_validate_success() {
        let lit_config = get_test_config_with_key(Some("node".into()));

        // Generate ed25519 keypair for signing.
        let mut csprng = OsRng {};
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

        // Generate local wallet for emulating EOA.
        let wallet = LocalWallet::new(&mut EthersOsRng);

        let lit_resource = AccessControlConditionResource::new(
            "524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251".into(),
        );

        let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.fZ").to_string();
        let expiration = (Utc::now() + Duration::days(1))
            .format("%Y-%m-%dT%H:%M:%S%.fZ")
            .to_string();

        let siwe_message = Message {
            domain: "localhost:7470".parse().unwrap(),
            address: wallet.address().into(),
            statement: Some(r#"Some custom statement. I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-accesscontrolcondition://524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251'."#.into()),
            uri: format!("lit:session:{}", session_pub_key).parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "JIsknRumpxsM9pqmc".into(),
            issued_at: now.parse().unwrap(),
            expiration_time: Some(expiration.parse().unwrap()),
            not_before: None,
            request_id: None,
            resources: vec!["urn:recap:eyJhdHQiOnsibGl0LWFjY2Vzc2NvbnRyb2xjb25kaXRpb246Ly81MjRhNjk3YTQxMGE0MTdmYjk1YTlmNTJkNTdjYmE1ZmE3Yzg3YjNhY2QzYjQwOGNmMTQ1NjBmYTUyNjkxMjUxIjp7IiovKiI6W3t9XX19LCJwcmYiOltdfQo=".parse().unwrap()],
        };

        // Sign SIWE message with local wallet.
        let sig = wallet
            .sign_hash(H256::from(siwe_message.eip191_hash().unwrap()))
            .expect("Could not sign hash");
        let auth_sig = JsonAuthSig::new(
            sig.to_string(),
            "ghi".to_string(),
            siwe_message.to_string(),
            wallet.address().to_string(),
            None,
        );

        let session_key_signed_message = SessionKeySignedMessage {
            session_key: session_pub_key.clone(),
            resource_ability_requests: vec![LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: lit_resource.get_resource_id().to_owned(),
                    resource_prefix: lit_resource.get_resource_prefix().to_string(),
                },
                ability: LitAbility::AccessControlConditionDecryption.to_string(),
            }],
            capabilities: vec![auth_sig.clone()],
            issued_at: now,
            expiration,
            node_address: "localhost:7470".to_string(),
        };

        // serialize to JSON string
        let message = serde_json::to_string(&session_key_signed_message).unwrap();
        let signature: Signature = signing_key.sign(message.as_bytes());

        let session_sig = JsonAuthSig::new(
            signature.to_string(),
            "ghi".to_string(),
            message,
            session_pub_key,
            None,
        );

        let requested_lit_resource_ability = lit_resource.decrypt_ability();

        let validate = validate_session_sig(
            &session_sig,
            &requested_lit_resource_ability,
            &None,
            &lit_config,
            &"".to_string(),
        )
        .await;

        match validate {
            Ok(valid_auth_sig) => {
                assert_eq!(valid_auth_sig, auth_sig);
            }
            Err(e) => {
                println!("Failed to validate session sig: {:?}", e);
                assert!(false);
                return;
            }
        }
    }
}

#[cfg(test)]
mod extract_requested_resources_tests {
    use std::sync::Arc;

    use crate::{
        auth::{
            auth_material::JsonAuthSig,
            lit_resource::LitResource,
            resources::{AccessControlConditionResource, RateLimitIncreaseNFTResource},
            session_sigs::extract_requested_resources_from_session_sig,
        },
        models::auth::{
            LitAbility, LitResourceAbilityRequest, LitResourceAbilityRequestResource,
            SessionKeySignedMessage,
        },
    };

    struct TestCase {
        session_sig: JsonAuthSig,
        expected_resources: Vec<Arc<dyn LitResource>>,
    }

    fn get_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                session_sig: JsonAuthSig::new(
                    "sig".into(),
                    "ghi".into(),
                    serde_json::to_string(&SessionKeySignedMessage {
                        session_key: "pub_key".into(),
                        resource_ability_requests: vec![],
                        capabilities: vec![],
                        issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                        expiration: "2024-01-01T00:00:00Z".to_string(),
                        node_address: "localhost:7470".to_string(),
                    }).unwrap(),
                    "pub_key".into(),
                    None,
                ),
                expected_resources: vec![],
            },
            TestCase {
                session_sig: JsonAuthSig::new(
                    "sig".into(),
                    "ghi".into(),
                    serde_json::to_string(&SessionKeySignedMessage {
                        session_key: "pub_key".into(),
                        resource_ability_requests: vec![LitResourceAbilityRequest {
                            resource: LitResourceAbilityRequestResource {
                                resource: "524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251".into(),
                                resource_prefix: "lit-accesscontrolcondition".into(),
                            },
                            ability: LitAbility::AccessControlConditionDecryption.to_string(),
                        }],
                        capabilities: vec![],
                        issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                        expiration: "2024-01-01T00:00:00Z".to_string(),
                        node_address: "localhost:7470".to_string(),
                    }).unwrap(),
                    "pub_key".into(),
                    None,
                ),
                expected_resources: vec![
                    Arc::new(AccessControlConditionResource::new(
                        "524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251".into(),
                    )),
                ],
            },
            TestCase {
                session_sig: JsonAuthSig::new(
                    "sig".into(),
                    "ghi".into(),
                    serde_json::to_string(&SessionKeySignedMessage {
                        session_key: "pub_key".into(),
                        resource_ability_requests: vec![LitResourceAbilityRequest {
                            resource: LitResourceAbilityRequestResource {
                                resource: "524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251".into(),
                                resource_prefix: "lit-accesscontrolcondition".into(),
                            },
                            ability: LitAbility::AccessControlConditionDecryption.to_string(),
                        }, LitResourceAbilityRequest {
                            resource: LitResourceAbilityRequestResource {
                                resource: "123".into(),
                                resource_prefix: "lit-ratelimitincrease".into(),
                            },
                            ability: LitAbility::RateLimitIncreaseAuth.to_string(),
                        }],
                        capabilities: vec![],
                        issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                        expiration: "2024-01-01T00:00:00Z".to_string(),
                        node_address: "localhost:7470".to_string(),
                    }).unwrap(),
                    "pub_key".into(),
                    None,
                ),
                expected_resources: vec![
                    Arc::new(AccessControlConditionResource::new(
                        "524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251".into(),
                    )),
                    Arc::new(RateLimitIncreaseNFTResource::new(
                        "123".into(),
                    )),
                ],
            },
        ]
    }

    #[test]
    fn test_extract_requested_resources_from_session_sig() {
        for test_case in get_test_cases() {
            let resources = extract_requested_resources_from_session_sig(&test_case.session_sig);
            assert!(resources.is_ok());
            let resources: Vec<Arc<dyn LitResource>> = resources.unwrap();
            for (i, expected_resource) in test_case.expected_resources.iter().enumerate() {
                assert_eq!(
                    expected_resource.get_resource_id(),
                    resources[i].get_resource_id()
                );
                assert_eq!(
                    expected_resource.get_resource_prefix(),
                    resources[i].get_resource_prefix()
                );
            }
        }
    }
}
