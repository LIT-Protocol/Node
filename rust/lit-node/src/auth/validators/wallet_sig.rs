use crate::auth::validators::cosmos::validate_cosmos_auth_sig;
use crate::auth::validators::solana::validate_solana_auth_sig;
use crate::error::{
    conversion_err, parser_err, parser_err_code, unexpected_err, validation_err,
    validation_err_code, Result, EC,
};
use crate::utils::siwe::validate_siwe;
use std::str::FromStr;

use std::convert::TryInto;

use crate::auth::auth_material::JsonAuthSig;
use crate::config::LitNodeConfig as _;
use crate::utils::encoding;
use ethers::{
    types::{Address, Signature},
    utils::keccak256,
};
use libsecp256k1::PublicKey;
use lit_core::config::LitConfig;
use siwe::Message;

const REGEX: &str = r"\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d(\.\d+)?(([+-]\d\d:\d\d)|Z)?$";

const INVALID_SIWE_URIS_FOR_AUTH: [&str; 2] = ["lit:session", "lit:capability:delegation"];

/// Validate wallet signature by chain.
pub(crate) async fn validate_wallet_sig_by_chain(
    auth_sig: &JsonAuthSig,
    chain: &Option<String>,
    cfg: &LitConfig,
) -> Result<()> {
    let enable_siwe_validation = matches!(cfg.enable_siwe_validation(), Ok(true));
    match chain {
        Some(chain) => match chain.as_str() {
            "solana" | "solanaDevnet" | "solanaTestnet" => {
                debug!("Checking solana signature");
                let validate = validate_solana_auth_sig(auth_sig)?;
                if !validate {
                    return Err(validation_err_code(
                        "Signature is not valid",
                        EC::NodeInvalidAuthSig,
                        None,
                    ));
                }
                Ok(())
            }
            "cosmos" | "kyve" | "cheqd" | "cheqdTestnet" | "cheqdMainnet" | "juno" => {
                debug!("Checking cosmos signature");
                let validate = validate_cosmos_auth_sig(auth_sig, chain)?;
                if !validate {
                    return Err(validation_err_code(
                        "Signature is not valid",
                        EC::NodeInvalidAuthSig,
                        None,
                    ));
                }
                Ok(())
            }
            _ => validate_wallet_sig(auth_sig, enable_siwe_validation).await,
        },
        None => validate_wallet_sig(auth_sig, enable_siwe_validation).await,
    }
}

/// Validate EVM-compatible wallet signature.
async fn validate_wallet_sig(auth_sig: &JsonAuthSig, enable_siwe_validation: bool) -> Result<()> {
    let sig = Signature::from_str(&auth_sig.sig).map_err(|e| {
        parser_err_code(
            e,
            EC::NodeAuthSigSignatureConversionError,
            Some("Error parsing the signature".into()),
        )
    })?;

    let presented_address = Address::from_str(&auth_sig.address)
        .map_err(|e| validation_err_code(e, EC::NodeAuthSigAddressConversionError, None))?;

    sig.verify(auth_sig.signed_message.clone(), presented_address)
        .map_err(|e| validation_err(e, Some("Invalid signature verification".into())))?;

    let message = validate_siwe_message(auth_sig, enable_siwe_validation)?;

    let sig_as_array = encoding::hex_to_bytes(&auth_sig.sig)
        .map_err(|e| parser_err_code(e, EC::NodeSIWESigConversionError, None))?
        .try_into()
        .map_err(|_| conversion_err("Could not convert into fixed size slice", None))?;

    // Verify the signature against the hashed message this is delegated to the EIP1271 contract for derived_via = "EIP1271"
    let verification_result = message
        .verify_eip191(&sig_as_array)
        .map_err(|err| validation_err(err, Some("Error verifying SIWE signature".into())))?;
    debug!(
        "SIWE verification result: {:?}",
        encoding::bytes_to_hex(&verification_result)
    );

    // Convert compressed pubkey to uncompressed
    let mut pubkey_fixed_length: [u8; 33] = [0; 33];
    pubkey_fixed_length.copy_from_slice(&verification_result);
    let pubkey = PublicKey::parse_compressed(&pubkey_fixed_length)
        .map_err(|e| parser_err(e, Some("Error parsing pubkey".into())))?;

    // Convert verification_result public key to eth address
    let pubkey_hash = keccak256(&pubkey.serialize()[1..]);
    let mut pubkey_hash_fixed_length: [u8; 20] = [0; 20];
    pubkey_hash_fixed_length.copy_from_slice(&pubkey_hash[12..]);
    let siwe_recovered_address = Address::from_slice(&pubkey_hash_fixed_length);

    // Verify the address only when the Optional param is available
    if siwe_recovered_address != presented_address {
        return Err(unexpected_err(
            "Recovered address does not match presented address",
            None,
        ));
    }

    Ok(())
}

/// Validate the SIWE message and returns the SIWE Message
pub(crate) fn validate_siwe_message(
    auth_sig: &JsonAuthSig,
    enable_siwe_validation: bool,
) -> Result<Message> {
    let message: Message = auth_sig
        .signed_message
        .parse()
        .map_err(|e| parser_err(e, Some("Parse error on SIWE".into())).add_msg_to_details())?;

    // Do not let delegation or session signatures be used as an auth sig here
    if INVALID_SIWE_URIS_FOR_AUTH
        .iter()
        .any(|uri| message.uri.to_string().starts_with(uri))
    {
        return Err(validation_err_code(
            format!(
                "Invalid URI for top level auth sig: {}",
                message.uri.as_str()
            ),
            EC::NodeInvalidAuthSig,
            None,
        ));
    }

    // Validate time-related parameters
    if enable_siwe_validation {
        validate_siwe(&message)
            .map_err(|e| validation_err(e, Some("SIWE validation failed".into())))?;
    }

    Ok(message)
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use ethers::prelude::rand::rngs::OsRng as EthersOsRng;
    use ethers::signers::{LocalWallet, Signer as WalletSigner};
    use ethers::types::H256;
    use lit_core::error::Kind;
    use siwe::Message;

    use super::validate_wallet_sig;
    use crate::utils::encoding;
    use crate::{auth::auth_material::JsonAuthSig, error::EC};

    #[tokio::test]
    async fn test_validate_invalid_sig() {
        let auth_sig = JsonAuthSig::new(
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
            "jkl".to_string(),
            None,
        );

        let result = validate_wallet_sig(&auth_sig, true).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.is_code(EC::NodeAuthSigSignatureConversionError, false));
        assert!(err.is_kind(Kind::Parser, false));
        assert!(err.to_string().contains("Error parsing the signature"));
    }

    #[tokio::test]
    async fn test_validate_invalid_address() {
        let auth_sig = JsonAuthSig::new(
            "0x2bdede6164f56a601fc17a8a78327d28b54e87cf3fa20373fca1d73b804566736d76efe2dd79a4627870a50e66e1a9050ca333b6f98d9415d8bca424980611ca1c".into(),
            "def".to_string(),
            "ghi".to_string(),
            "jkl".to_string(),
            None,
        );

        let result = validate_wallet_sig(&auth_sig, true).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.is_code(EC::NodeAuthSigAddressConversionError, false));
        assert!(err.is_kind(Kind::Validation, false));
    }

    #[tokio::test]
    async fn test_validate_invalid_signature() {
        let auth_sig = JsonAuthSig::new(
            "0x2bdede6164f56a601fc17a8a78327d28b54e87cf3fa20373fca1d73b804566736d76efe2dd79a4627870a50e66e1a9050ca333b6f98d9415d8bca424980611ca1c".into(),
            "def".to_string(),
            "ghi".to_string(),
            "0x2bdede6164f56a601fc17a8a78327d28b54e87cf".into(),
            None,
        );

        let result = validate_wallet_sig(&auth_sig, true).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err.to_string().contains("Invalid signature verification"));
    }

    #[tokio::test]
    async fn test_validate_invalid_expiration() {
        // Generate local wallet for emulating EOA.
        let wallet = LocalWallet::new(&mut EthersOsRng);

        let siwe_message = Message {
            domain: "localhost:7470".parse().unwrap(),
            address: wallet.address().into(),
            statement: Some(r#"Some custom statement."#.into()),
            uri: "https://localhost/login".parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "JIsknRumpxsM9pqmc".into(),
            issued_at: "2023-05-01T15:41:08.640Z".parse().unwrap(),
            expiration_time: Some("2023-06-01T15:41:08.640Z".parse().unwrap()),
            not_before: None,
            request_id: None,
            resources: vec![],
        };

        // Generate signature
        let sig = wallet
            .sign_hash(H256::from(siwe_message.eip191_hash().unwrap()))
            .expect("Could not sign hash");

        let auth_sig = JsonAuthSig::new(
            sig.to_string(),
            "def".to_string(),
            siwe_message.to_string(),
            encoding::bytes_to_hex(wallet.address()),
            None,
        );

        let result = validate_wallet_sig(&auth_sig, true).await;
        let err = result.unwrap_err();
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err.to_string().contains("Session key expiration 2023-06-01T15:41:08.640Z is in the past beyond the grace period of 60 seconds"));

        // Try again, this time without SIWE validation
        let result = validate_wallet_sig(&auth_sig, false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_valid_siwe() {
        // Generate local wallet for emulating EOA.
        let wallet = LocalWallet::new(&mut EthersOsRng);
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.fZ").to_string();
        let expiration = (Utc::now() + Duration::days(1))
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

        let auth_sig = JsonAuthSig::new(
            sig.to_string(),
            "def".to_string(),
            siwe_message.to_string(),
            encoding::bytes_to_hex(wallet.address()),
            None,
        );

        let result = validate_wallet_sig(&auth_sig, true).await;
        assert!(result.is_ok());
    }
}
