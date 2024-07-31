use blsful::Bls12381G2Impl;
use ethers::types::Address;
use rocket::time::OffsetDateTime;
use siwe::{Message, VerificationOpts};
use tracing::debug;

use crate::{
    auth::{
        auth_material::{siwe_hash_to_bls_session_hash, AuthMaterialType, JsonAuthSig},
        capabilities::recap::extract_and_verify_all_capabilities,
        capabilities::session_capability_object::SessionCapabilityObject,
    },
    error::{parser_err_code, validation_err_code, Result, EC},
    utils::encoding,
};

use super::auth_sig::{CapabilityAuthSigValidator, SessionSigAuthSigValidator};

pub(crate) struct SiweValidator;

impl SiweValidator {
    pub fn new() -> SiweValidator {
        SiweValidator {}
    }
}

impl SiweValidator {
    pub fn parse_siwe_message(&self, message: &str) -> Result<Message> {
        message.parse::<Message>().map_err(|e| {
            parser_err_code(
                e,
                EC::NodeSIWECapabilityInvalid,
                Some("Error parsing SIWE message on capability sig".into()),
            )
            .add_msg_to_details()
        })
    }
}

impl CapabilityAuthSigValidator for SiweValidator {
    /// Basic validation of the SIWE message and sig of the wallet sig.
    async fn validate_capability_ecdsa_auth_sig(&self, auth_sig: &JsonAuthSig) -> Result<()> {
        // Parse the SIWE message.
        let siwe_message = self.parse_siwe_message(&auth_sig.signed_message)?;

        // Parse the auth sig.
        let sig_as_array = encoding::hex_to_bytes(&auth_sig.sig)
            .map_err(|err| parser_err_code(err, EC::NodeSIWESigConversionError, None))?;
        let sig_as_array = sig_as_array.as_slice();

        // Verify SIWE message.
        let now = OffsetDateTime::now_utc();
        if let Err(e) = siwe_message
            .verify(
                sig_as_array,
                &VerificationOpts {
                    timestamp: Some(now),
                    ..Default::default()
                },
            )
            .await
        {
            return Err(validation_err_code(
                e,
                EC::NodeSIWEMessageError,
                Some("Wallet signature of SIWE is not valid".into()),
            )
            .add_msg_to_details());
        }

        // confirm that the address matches the address in the auth_sig
        let siwe_address = Address::from_slice(&siwe_message.address);
        let parsed_address = auth_sig.address.parse::<Address>().map_err(|err| {
            parser_err_code(
                err,
                EC::NodeSIWEMessageError,
                Some("Error parsing address in auth sig".into()),
            )
        })?;
        if siwe_address != parsed_address {
            return Err(validation_err_code(
                "The address in the SIWE message does not match the address in the auth sig",
                EC::NodeSIWEMessageError,
                None,
            ));
        }

        Ok(())
    }

    async fn validate_capability_bls_auth_sig(
        &self,
        auth_sig: &JsonAuthSig,
        bls_root_pubkey: &String,
    ) -> Result<()> {
        // Parse the SIWE message.
        let siwe_message = self.parse_siwe_message(&auth_sig.signed_message)?;

        // Verify SIWE message.
        if !siwe_message.valid_now() {
            return Err(validation_err_code(
                "The SIWE message is not valid now.  It has expired or is not yet valid",
                EC::NodeSIWEMessageError,
                None,
            ));
        }

        // for BLS, we don't just sign the raw SIWE message - we add a prefix then hash again.
        // we do this to namespace our signatures since we're using the BLS network key.
        // this is just a precaution to avoid signing the wrong data on the wrong code path
        let siwe_hash = match siwe_message.eip191_hash() {
            Ok(siwe_hash) => siwe_hash,
            Err(e) => {
                return Err(parser_err_code(
                    e,
                    EC::NodeSIWEMessageError,
                    Some("Error hashing SIWE message".into()),
                )
                .add_msg_to_details())
            }
        };
        let signed_data = siwe_hash_to_bls_session_hash(siwe_hash.into());

        let signature: blsful::Signature<Bls12381G2Impl> = serde_json::from_str(&auth_sig.sig)
            .map_err(|err| parser_err_code(err, EC::NodeSIWESigConversionError, None))?;

        let bls_root_key = blsful::PublicKey::<Bls12381G2Impl>::try_from(
            &hex::decode(bls_root_pubkey).expect("Failed to decode root key"),
        )
        .expect("Failed to convert bls public key from bytes");
        match signature.verify(&bls_root_key, signed_data) {
            Ok(_) => {}
            Err(e) => {
                return Err(validation_err_code(
                    e,
                    EC::NodeSIWEMessageError,
                    Some("BLS signature of session key via SIWE is not valid".into()),
                ));
            }
        }

        // we don't need to check that the address field matches the the recovered address, since the network created this message and set the address field, and the network verified their auth material etc during session creation.

        Ok(())
    }

    async fn validate_capability_auth_sig(
        &self,
        auth_sig: &JsonAuthSig,
        bls_root_pubkey: &String,
    ) -> Result<()> {
        match auth_sig.auth_material_type() {
            AuthMaterialType::WalletSig => {
                self.validate_capability_ecdsa_auth_sig(auth_sig).await?
            }
            AuthMaterialType::BLSNetworkSig => {
                self.validate_capability_bls_auth_sig(auth_sig, bls_root_pubkey)
                    .await?
            }
            _ => {
                return Err(validation_err_code(
                    format!("The auth sig material type {:?} is not supported when calling validate_auth_sig() within session sig validation.  Only WalletSig and BLSNetworkSig are supported.", auth_sig.auth_material_type()),
                    EC::NodeSIWEMessageError,
                    None,
                ));
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl SessionSigAuthSigValidator for SiweValidator {
    /// Basic validation of the SIWE message and sig of the wallet sig.
    async fn validate_auth_sig_basic(
        &self,
        auth_sig: &JsonAuthSig,
        session_pubkey: &str,
    ) -> Result<()> {
        // Parse the SIWE message.
        let siwe_message = self.parse_siwe_message(&auth_sig.signed_message)?;

        // Parse the auth sig.
        let sig_as_array = encoding::hex_to_bytes(&auth_sig.sig)
            .map_err(|err| parser_err_code(err, EC::NodeSIWESigConversionError, None))?;
        let sig_as_array = sig_as_array.as_slice();

        // Verify SIWE message.
        let now = OffsetDateTime::now_utc();
        if let Err(e) = siwe_message
            .verify(
                sig_as_array,
                &VerificationOpts {
                    timestamp: Some(now),
                    ..Default::default()
                },
            )
            .await
        {
            return Err(validation_err_code(
                e,
                EC::NodeSIWEMessageError,
                Some("Wallet signature of session key via SIWE is not valid".into()),
            )
            .add_msg_to_details());
        }
        // Validate that the session public key is signed in the SIWE message.
        let signed_uri = siwe_message.uri.to_string();
        let correct_uri = format!("lit:session:{}", session_pubkey);
        if signed_uri != correct_uri {
            return Err(validation_err_code(
                "The session pubkey in the auth sig is not signed in the wallet-signed SIWE message",
                EC::NodeSIWEMessageError,
                None
            ).add_source_to_details());
        }

        Ok(())
    }

    async fn validate_bls_auth_sig_basic(
        &self,
        auth_sig: &JsonAuthSig,
        session_pubkey: &str,
        bls_root_pubkey: &String,
    ) -> Result<()> {
        // Parse the SIWE message.
        let siwe_message = self.parse_siwe_message(&auth_sig.signed_message)?;

        // Verify SIWE message.
        if !siwe_message.valid_now() {
            return Err(validation_err_code(
                "The SIWE message is not valid now.  It has expired or is not yet valid",
                EC::NodeSIWEMessageError,
                None,
            ));
        }

        // for BLS, we don't just sign the raw SIWE message - we add a prefix then hash again.
        // we do this to namespace our signatures since we're using the BLS network key.
        // this is just a precaution to avoid signing the wrong data on the wrong code path
        let siwe_hash = match siwe_message.eip191_hash() {
            Ok(siwe_hash) => siwe_hash,
            Err(e) => {
                return Err(parser_err_code(
                    e,
                    EC::NodeSIWEMessageError,
                    Some("Error hashing SIWE message".into()),
                )
                .add_msg_to_details())
            }
        };
        let signed_data = siwe_hash_to_bls_session_hash(siwe_hash.into());

        let signature: blsful::Signature<Bls12381G2Impl> = serde_json::from_str(&auth_sig.sig)
            .map_err(|err| parser_err_code(err, EC::NodeSIWESigConversionError, None))?;

        let bls_root_key = blsful::PublicKey::<Bls12381G2Impl>::try_from(
            &hex::decode(bls_root_pubkey).expect("Failed to decode root key"),
        )
        .expect("Failed to convert bls public key from bytes");
        match signature.verify(&bls_root_key, signed_data) {
            Ok(_) => {}
            Err(e) => {
                return Err(validation_err_code(
                    e,
                    EC::NodeSIWEMessageError,
                    Some("BLS signature of session key via SIWE is not valid".into()),
                ));
            }
        }

        // Validate that the session public key is signed in the SIWE message.
        let signed_uri = siwe_message.uri.to_string();
        let correct_uri = format!("lit:session:{}", session_pubkey);
        if signed_uri != correct_uri {
            return Err(validation_err_code(
                format!("The session pubkey in the auth sig is not signed in the wallet-signed SIWE message.  The correct URI should be {} but the signed URI was {}", correct_uri, signed_uri),
                EC::NodeSIWEMessageError,
                None
            ).add_source_to_details());
        }

        Ok(())
    }

    async fn validate_auth_sig(
        &self,
        auth_sig: &crate::auth::auth_material::JsonAuthSig,
        session_pubkey: &str,
        requested_lit_resource_ability: &crate::auth::resources::LitResourceAbility,
        bls_root_pubkey: &String,
    ) -> Result<()> {
        // Parse the SIWE message.
        let siwe_message = self.parse_siwe_message(&auth_sig.signed_message)?;

        match auth_sig.auth_material_type() {
            AuthMaterialType::WalletSig => {
                self.validate_auth_sig_basic(auth_sig, session_pubkey)
                    .await?
            }
            AuthMaterialType::BLSNetworkSig => {
                self.validate_bls_auth_sig_basic(auth_sig, session_pubkey, bls_root_pubkey)
                    .await?
            }
            _ => {
                return Err(validation_err_code(
                    format!("The auth sig material type {:?} is not supported when calling validate_auth_sig() within session sig validation.  Only WalletSig and BLSNetworkSig are supported.", auth_sig.auth_material_type()),
                    EC::NodeSIWEMessageError,
                    None,
                ));
            }
        }

        let session_capability_objects = extract_and_verify_all_capabilities(&siwe_message)
            .map_err(|err| {
                parser_err_code(
                    err,
                    EC::NodeSIWECapabilityInvalid,
                    Some("Unable to extract and verify session capability object".into()),
                )
            })?;
        let mut errors = vec![];
        for session_capability_object in &session_capability_objects {
            if let Err(e) = session_capability_object
                .verify_capabilities_for_resource(requested_lit_resource_ability)
            {
                debug!("Failed to verify capability object - this may not be an issue, since we check all of them, and only need 1 to pass: {}", e);
                errors.push(e);
            } else {
                return Ok(()); // success, one of the capabilities matched
            }
        }

        Err(validation_err_code(
            "Unable to verify any of the SIWE capability objects".to_string(),
            EC::NodeSIWECapabilityInvalid,
            Some(
                errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use ethers::prelude::rand::rngs::OsRng;
    use ethers::signers::{LocalWallet, Signer};
    use ethers::types::H256;
    use lit_core::error::Kind;
    use siwe::Message;

    use crate::{
        auth::auth_material::JsonAuthSig, auth::resources::AccessControlConditionResource,
        error::EC, utils::encoding,
    };

    use super::SiweValidator;
    use crate::auth::validators::auth_sig::SessionSigAuthSigValidator;

    #[tokio::test]
    async fn test_validate_parse_siwe_error() {
        let validator = SiweValidator::new();
        let auth_sig = JsonAuthSig::new(
            "0xdeadbeef".into(),
            "siwe".into(),
            "0xdeadbeef".into(),
            "0xdeadbeef".into(),
            None,
        );
        let requested_lit_resource_ability =
            AccessControlConditionResource::new("blah".into()).decrypt_ability();

        let validate = validator
            .validate_auth_sig(
                &auth_sig,
                "0xdeadbeef",
                &requested_lit_resource_ability,
                &"".to_string(),
            )
            .await;
        assert!(validate.is_err());

        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeSIWECapabilityInvalid, false));
        assert!(err.is_kind(Kind::Parser, false));
        assert!(err
            .to_string()
            .contains("Error parsing SIWE message on capability sig"));
    }

    #[tokio::test]
    async fn test_validate_parse_sig_error() {
        let validator = SiweValidator::new();
        let auth_sig = JsonAuthSig::new(
            "0xZZZ".into(),
            "siwe".into(),
            Message {
                domain: "example.com".parse().unwrap(),
                address: Default::default(),
                statement: Some("Some custom statement.".into()),
                uri: "did:key:example".parse().unwrap(),
                version: siwe::Version::V1,
                chain_id: 1,
                nonce: "mynonce1".into(),
                issued_at: "2022-06-21T12:00:00.000Z".parse().unwrap(),
                expiration_time: None,
                not_before: None,
                request_id: None,
                resources: vec![],
            }
            .to_string(),
            "0xdeadbeef".into(),
            None,
        );
        let requested_lit_resource_ability =
            AccessControlConditionResource::new("blah".into()).decrypt_ability();

        let validate = validator
            .validate_auth_sig(
                &auth_sig,
                "0xdeadbeef",
                &requested_lit_resource_ability,
                &"".to_string(),
            )
            .await;
        assert!(validate.is_err());

        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeSIWESigConversionError, false));
        assert!(err.is_kind(Kind::Parser, false));
    }

    #[tokio::test]
    async fn test_verify_siwe_error() {
        let validator = SiweValidator::new();
        let auth_sig = JsonAuthSig::new(
            "0x7d302b5095c43fe5291f3a50ece1bdc80767ef833944e709b04be43cc6c5f7846f96547590c638ce2c8ed19cb0d40ece449f1c32eb18da8ca430141d6ca5f36c1b".into(),
            "siwe".into(),
            Message {
                domain: "localhost:3000".parse().unwrap(),
                address: encoding::hex_to_bytes("0x3c3CA2BFFfffE532aed2923A34D6c1F9307F8076").unwrap().try_into().unwrap(),
                statement: Some("Lit Protocol PKP session signature".into()),
                uri: "lit:session:e76233cdd5483d674020cee626bdecfee6cf9d02b2bffa31b75b91c0ec04a09f".parse().unwrap(),
                version: siwe::Version::V1,
                chain_id: 1,
                nonce: "JIsknRumpxsM9pqmc".into(),
                issued_at: "2023-05-01T15:41:08.640Z".parse().unwrap(),
                expiration_time: Some("2023-05-02T15:41:08.603Z".parse().unwrap()),
                not_before: None,
                request_id: None,
                resources: vec!["urn:recap:eyJhdHQiOnsibGl0LWFjY2Vzc2NvbnRyb2xjb25kaXRpb246Ly81MjRhNjk3YTQxMGE0MTdmYjk1YTlmNTJkNTdjYmE1ZmE3Yzg3YjNhY2QzYjQwOGNmMTQ1NjBmYTUyNjkxMjUxIjp7IiovKiI6W3t9XX19LCJwcmYiOltdfQo=".parse().unwrap()],
            }.to_string(),
            "0xdeadbeef".into(),
            None,
        );
        let requested_lit_resource_ability =
            AccessControlConditionResource::new("blah".into()).decrypt_ability();

        let validate = validator
            .validate_auth_sig(
                &auth_sig,
                "0xdeadbeef",
                &requested_lit_resource_ability,
                &"".to_string(),
            )
            .await;
        assert!(validate.is_err());

        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeSIWEMessageError, false));
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err
            .to_string()
            .contains("Wallet signature of session key via SIWE is not valid"));
    }

    #[tokio::test]
    async fn test_validate_signed_uri_error() {
        // Need an expiration time that is dynamic and into the future,
        // can only be achieved if I sign with an ephemeral key here.
        let wallet = LocalWallet::new(&mut OsRng);

        let siwe_message = Message {
            domain: "localhost:3000".parse().unwrap(),
            address: wallet.address().into(),
            statement: Some("Lit Protocol PKP session signature".into()),
            uri: "lit:session:e76233cdd5483d674020cee626bdecfee6cf9d02b2bffa31b75b91c0ec04a09f".parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "JIsknRumpxsM9pqmc".into(),
            issued_at: "2023-05-01T15:41:08.640Z".parse().unwrap(),
            expiration_time: Some("2030-05-01T15:41:08.640Z".parse().unwrap()),
            not_before: None,
            request_id: None,
            resources: vec!["urn:recap:eyJhdHQiOnsibGl0LWFjY2Vzc2NvbnRyb2xjb25kaXRpb246Ly81MjRhNjk3YTQxMGE0MTdmYjk1YTlmNTJkNTdjYmE1ZmE3Yzg3YjNhY2QzYjQwOGNmMTQ1NjBmYTUyNjkxMjUxIjp7IiovKiI6W3t9XX19LCJwcmYiOltdfQo=".parse().unwrap()],
        };
        let to_sign = siwe_message.eip191_hash().unwrap();
        let sig: ethers::core::types::Signature = wallet
            .sign_hash(H256::from(to_sign))
            .expect("Could not sign hash");
        let auth_sig = JsonAuthSig::new(
            sig.to_string(),
            "siwe".into(),
            siwe_message.to_string(),
            wallet.address().to_string(),
            None,
        );
        let requested_lit_resource_ability =
            AccessControlConditionResource::new("blah".into()).decrypt_ability();

        let validator = SiweValidator::new();
        let validate = validator
            .validate_auth_sig(
                &auth_sig,
                "0xdeadbeef",
                &requested_lit_resource_ability,
                &"".to_string(),
            )
            .await;
        assert!(validate.is_err());
        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeSIWEMessageError, false));
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err.to_string().contains(
            "The session pubkey in the auth sig is not signed in the wallet-signed SIWE message"
        ));
    }

    #[tokio::test]
    async fn test_extract_error() {
        // Need an expiration time that is dynamic and into the future,
        // can only be achieved if I sign with an ephemeral key here.
        let wallet = LocalWallet::new(&mut OsRng);

        let siwe_message = Message {
            domain: "localhost:3000".parse().unwrap(),
            address: wallet.address().into(),
            statement: Some("Lit Protocol PKP session signature".into()),
            uri: "lit:session:e76233cdd5483d674020cee626bdecfee6cf9d02b2bffa31b75b91c0ec04a09f".parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "JIsknRumpxsM9pqmc".into(),
            issued_at: "2023-05-01T15:41:08.640Z".parse().unwrap(),
            expiration_time: Some("2030-05-01T15:41:08.640Z".parse().unwrap()),
            not_before: None,
            request_id: None,
            resources: vec!["urn:recap:eyJhdHQiOnsibGl0LWFjY2Vzc2NvbnRyb2xjb25kaXRpb246Ly81MjRhNjk3YTQxMGE0MTdmYjk1YTlmNTJkNTdjYmE1ZmE3Yzg3YjNhY2QzYjQwOGNmMTQ1NjBmYTUyNjkxMjUxIjp7IiovKiI6W3t9XX19LCJwcmYiOltdfQo=".parse().unwrap()],
        };
        let to_sign = siwe_message.eip191_hash().unwrap();
        let sig: ethers::core::types::Signature = wallet
            .sign_hash(H256::from(to_sign))
            .expect("Could not sign hash");
        let auth_sig = JsonAuthSig::new(
            sig.to_string(),
            "siwe".into(),
            siwe_message.to_string(),
            wallet.address().to_string(),
            None,
        );
        let requested_lit_resource_ability =
            AccessControlConditionResource::new("blah".into()).decrypt_ability();

        let validator = SiweValidator::new();
        let validate = validator
            .validate_auth_sig(
                &auth_sig,
                "e76233cdd5483d674020cee626bdecfee6cf9d02b2bffa31b75b91c0ec04a09f",
                &requested_lit_resource_ability,
                &"".to_string(),
            )
            .await;
        assert!(validate.is_err());
        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeSIWECapabilityInvalid, false));
        assert!(err.is_kind(Kind::Parser, false));
        assert!(err
            .to_string()
            .contains("Unable to extract and verify session capability object"));
    }

    #[tokio::test]
    async fn test_verify_capabilities_error() {
        // Need an expiration time that is dynamic and into the future,
        // can only be achieved if I sign with an ephemeral key here.
        let wallet = LocalWallet::new(&mut OsRng);

        let siwe_message = Message {
            domain: "localhost:3000".parse().unwrap(),
            address: wallet.address().into(),
            statement: Some(r#"Some custom statement. I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-accesscontrolcondition://524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251'."#.into()),
            uri: "lit:session:e76233cdd5483d674020cee626bdecfee6cf9d02b2bffa31b75b91c0ec04a09f".parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "JIsknRumpxsM9pqmc".into(),
            issued_at: "2023-05-01T15:41:08.640Z".parse().unwrap(),
            expiration_time: Some("2030-05-01T15:41:08.640Z".parse().unwrap()),
            not_before: None,
            request_id: None,
            resources: vec!["urn:recap:eyJhdHQiOnsibGl0LWFjY2Vzc2NvbnRyb2xjb25kaXRpb246Ly81MjRhNjk3YTQxMGE0MTdmYjk1YTlmNTJkNTdjYmE1ZmE3Yzg3YjNhY2QzYjQwOGNmMTQ1NjBmYTUyNjkxMjUxIjp7IiovKiI6W3t9XX19LCJwcmYiOltdfQo=".parse().unwrap()],
        };
        let to_sign = siwe_message.eip191_hash().unwrap();
        let sig: ethers::core::types::Signature = wallet
            .sign_hash(H256::from(to_sign))
            .expect("Could not sign hash");
        let auth_sig = JsonAuthSig::new(
            sig.to_string(),
            "siwe".into(),
            siwe_message.to_string(),
            wallet.address().to_string(),
            None,
        );
        let requested_lit_resource_ability =
            AccessControlConditionResource::new("blah".into()).decrypt_ability();

        let validator = SiweValidator::new();
        let validate = validator
            .validate_auth_sig(
                &auth_sig,
                "e76233cdd5483d674020cee626bdecfee6cf9d02b2bffa31b75b91c0ec04a09f",
                &requested_lit_resource_ability,
                &"".to_string(),
            )
            .await;
        assert!(validate.is_err());
        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeSIWECapabilityInvalid, false));
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err
            .to_string()
            .contains("Unable to verify any of the SIWE capability objects"));
    }

    #[tokio::test]
    async fn test_validate_success() {
        // Need an expiration time that is dynamic and into the future,
        // can only be achieved if I sign with an ephemeral key here.
        let wallet = LocalWallet::new(&mut OsRng);
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.fZ").to_string();
        let expiration = (Utc::now() + Duration::days(1))
            .format("%Y-%m-%dT%H:%M:%S%.fZ")
            .to_string();

        let siwe_message = Message {
            domain: "localhost:3000".parse().unwrap(),
            address: wallet.address().into(),
            statement: Some(r#"Some custom statement. I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-accesscontrolcondition://524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251'."#.into()),
            uri: "lit:session:e76233cdd5483d674020cee626bdecfee6cf9d02b2bffa31b75b91c0ec04a09f".parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "JIsknRumpxsM9pqmc".into(),
            issued_at: now.parse().unwrap(),
            expiration_time: Some(expiration.parse().unwrap()),
            not_before: None,
            request_id: None,
            resources: vec!["urn:recap:eyJhdHQiOnsibGl0LWFjY2Vzc2NvbnRyb2xjb25kaXRpb246Ly81MjRhNjk3YTQxMGE0MTdmYjk1YTlmNTJkNTdjYmE1ZmE3Yzg3YjNhY2QzYjQwOGNmMTQ1NjBmYTUyNjkxMjUxIjp7IiovKiI6W3t9XX19LCJwcmYiOltdfQo=".parse().unwrap()],
        };
        let to_sign = siwe_message.eip191_hash().unwrap();
        let sig: ethers::core::types::Signature = wallet
            .sign_hash(H256::from(to_sign))
            .expect("Could not sign hash");
        let auth_sig = JsonAuthSig::new(
            sig.to_string(),
            "siwe".into(),
            siwe_message.to_string(),
            wallet.address().to_string(),
            None,
        );
        let requested_lit_resource_ability = AccessControlConditionResource::new(
            "524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251".into(),
        )
        .decrypt_ability();

        let validator = SiweValidator::new();
        let validate = validator
            .validate_auth_sig(
                &auth_sig,
                "e76233cdd5483d674020cee626bdecfee6cf9d02b2bffa31b75b91c0ec04a09f",
                &requested_lit_resource_ability,
                &"".to_string(),
            )
            .await;
        assert!(validate.is_ok());
    }
}
