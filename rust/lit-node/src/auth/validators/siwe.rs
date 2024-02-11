use rocket::time::OffsetDateTime;
use siwe::{Message, VerificationOpts};

use crate::{
    auth::{
        auth_material::JsonAuthSig,
        capabilities::{extract_and_verify, session_capability_object::SessionCapabilityObject},
    },
    error::{parser_err_code, validation_err_code, Result, EC},
    utils::encoding,
};

use super::auth_sig::SessionSigAuthSigValidator;

pub(crate) struct SiweValidator;

impl SiweValidator {
    pub fn new() -> SiweValidator {
        SiweValidator {}
    }
}

impl SiweValidator {
    pub(self) fn parse_siwe_message(&self, message: &str) -> Result<Message> {
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

    async fn validate_auth_sig(
        &self,
        auth_sig: &crate::auth::auth_material::JsonAuthSig,
        session_pubkey: &str,
        requested_lit_resource_ability: &crate::auth::resources::LitResourceAbility,
    ) -> Result<()> {
        // Parse the SIWE message.
        let siwe_message = self.parse_siwe_message(&auth_sig.signed_message)?;

        self.validate_auth_sig_basic(auth_sig, session_pubkey)
            .await?;

        let session_capability_object = extract_and_verify(siwe_message).map_err(|err| {
            parser_err_code(
                err,
                EC::NodeSIWECapabilityInvalid,
                Some("Unable to extract and verify session capability object".into()),
            )
        })?;
        if let Err(e) = session_capability_object
            .verify_capabilities_for_resource(requested_lit_resource_ability)
        {
            return Err(validation_err_code(
                e,
                EC::NodeSIWECapabilityInvalid,
                Some("Unable to verify SIWE capability object".into()),
            ));
        }

        Ok(())
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
            .validate_auth_sig(&auth_sig, "0xdeadbeef", &requested_lit_resource_ability)
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
            .validate_auth_sig(&auth_sig, "0xdeadbeef", &requested_lit_resource_ability)
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
            .validate_auth_sig(&auth_sig, "0xdeadbeef", &requested_lit_resource_ability)
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
            .validate_auth_sig(&auth_sig, "0xdeadbeef", &requested_lit_resource_ability)
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
            )
            .await;
        assert!(validate.is_err());
        let err = validate.unwrap_err();
        assert!(err.is_code(EC::NodeSIWECapabilityInvalid, false));
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err
            .to_string()
            .contains("Unable to verify SIWE capability object"));
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
            )
            .await;
        assert!(validate.is_ok());
    }
}
