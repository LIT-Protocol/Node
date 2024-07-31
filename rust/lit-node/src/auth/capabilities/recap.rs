use crate::auth::lit_resource::LitResource;
use crate::auth::resources::{LitResourceAbility, ResourceType};
use crate::error::{parser_err, parser_err_code, validation_err_code, Result, EC};
use crate::models::auth::LitAbility;
use iri_string::spec::UriSpec;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use siwe::Message;
use siwe_recap::{Capability, NotaBeneCollection, RESOURCE_PREFIX};
use ucan_capabilities_object::Ability;

use iri_string::types::{RiStr, RiString, UriStr};

use super::session_capability_object::SessionCapabilityObject;

// so extract_and_verify that's built into the siwe-recap crate only uses the last resource in the list:
// https://github.com/spruceid/siwe-recap/blob/main/src/capability.rs#L261
// this method just reimplements their extract_and_verify but it loops over allll the capabilities and returns an array instead of a single capability.
pub fn extract_and_verify_all_capabilities(
    siwe_message: &Message,
) -> Result<Vec<RecapSessionCapabilityObject>> {
    println!("Resources length: {}", siwe_message.resources.len());
    // extract the capabilities (all of them, not just the last)
    let capabilities = siwe_message
        .resources
        .iter()
        .filter(|u| u.as_str().starts_with(RESOURCE_PREFIX))
        .flat_map(Capability::try_from) // clippy said this is equivilant to map .map(Capability::try_from) then filtering .is_ok() and then map .unwrap(): https://rust-lang.github.io/rust-clippy/master/index.html#/result_filter_map
        .collect::<Vec<_>>();
    println!("Capabilities length: {}", capabilities.len());

    for capability in &capabilities {
        // i do not understand how this verifies it
        // but it's what the siwe-recap crate does
        let expected = capability.to_statement();
        if let Some(statement) = &siwe_message.statement {
            if !statement.ends_with(&expected) {
                return Err(parser_err_code(
                    format!(
                        "Incorrect statement for capability object: expected '{}', got '{}'",
                        expected, statement
                    ),
                    EC::NodeSIWECapabilityInvalid,
                    None,
                ));
            }
        }
    }

    Ok(capabilities
        .into_iter()
        .map(|c| RecapSessionCapabilityObject { inner: c })
        .collect())
}

#[derive(Debug)]
pub struct RecapSessionCapabilityObject {
    pub(self) inner: Capability<Value>,
}

impl RecapSessionCapabilityObject {
    fn as_uri_str(s: &str) -> Result<&RiStr<UriSpec>> {
        UriStr::new(s).map_err(|err| parser_err(err, Some("Unable to parse IRI string".into())))
    }

    fn as_recap_ability(s: &str) -> Result<Ability> {
        Ability::from_str(s)
            .map_err(|err| parser_err(err, Some("Unable to parse recap ability".into())))
    }

    pub fn recap_abilities(
        &self,
    ) -> &BTreeMap<RiString<UriSpec>, BTreeMap<Ability, NotaBeneCollection<serde_json::Value>>>
    {
        self.inner.abilities()
    }

    fn get_resource_key_to_match_against(
        &self,
        lit_resource: impl LitResource,
    ) -> Result<Box<RiString<UriSpec>>> {
        let lit_resource_prefix_with_wildcard =
            format!("{}://*", lit_resource.get_resource_prefix());
        let lit_resource_key = lit_resource.get_resource_key();
        let attenuated_resource_keys_to_match_against = vec![
            RecapSessionCapabilityObject::as_uri_str(lit_resource_prefix_with_wildcard.as_ref())?,
            RecapSessionCapabilityObject::as_uri_str(lit_resource_key.as_ref())?,
        ];

        for attenuated_resource_key_to_match_against in attenuated_resource_keys_to_match_against {
            if self
                .inner
                .abilities()
                .contains_key(attenuated_resource_key_to_match_against)
            {
                return Ok(Box::new(
                    attenuated_resource_key_to_match_against.to_owned(),
                ));
            }
        }

        Ok(Box::new(
            RecapSessionCapabilityObject::as_uri_str("lit-invalid://*")?.to_owned(),
        ))
    }
}

impl SessionCapabilityObject for RecapSessionCapabilityObject {
    fn verify_capabilities_for_resource(
        &self,
        requested_lit_resource_ability: &LitResourceAbility,
    ) -> Result<()> {
        // Get the attenuations object.
        let attenuations = self.inner.abilities();

        let (recap_namespace, recap_ability) =
            get_recap_namespace_and_ability(requested_lit_resource_ability.get_ability())?;
        let recap_ability_to_check_for = RecapSessionCapabilityObject::as_recap_ability(
            format!("{}/{}", recap_namespace, recap_ability).as_ref(),
        )?;

        // Find an attenuated resource key to match against.
        let attenuated_resource_key = {
            match requested_lit_resource_ability.get_resource() {
                // TODO: code smell - refactor this
                ResourceType::AccessControlCondition(r) => {
                    self.get_resource_key_to_match_against(r.to_owned())?
                }
                ResourceType::PKPNFT(r) => self.get_resource_key_to_match_against(r.to_owned())?,
                ResourceType::RateLimitIncreaseNFT(r) => {
                    self.get_resource_key_to_match_against(r.to_owned())?
                }
                ResourceType::LitAction(r) => {
                    self.get_resource_key_to_match_against(r.to_owned())?
                }
            }
        };

        if !attenuations.contains_key(attenuated_resource_key.as_ref()) {
            // No attenuations specified for this resource.
            return Err(validation_err_code(
                "No capability object found for this resource".to_string(),
                EC::NodeSIWECapabilityInvalid,
                None,
            ));
        }

        // Check whether the exact Recap namespace/ability pair is present.
        let attenuated_recap_abilities = attenuations
            .get(attenuated_resource_key.as_ref())
            .ok_or_else(|| {
                validation_err_code(
                    "No attenuations found in capability object for this resource".to_string(),
                    EC::NodeSIWECapabilityInvalid,
                    None,
                )
            })?;

        for attenuated_recap_ability in attenuated_recap_abilities.keys() {
            // Return early if the attenuated recap ability is a wildcard.
            if *attenuated_recap_ability == RecapSessionCapabilityObject::as_recap_ability("*/*")? {
                return Ok(());
            }

            if *attenuated_recap_ability == recap_ability_to_check_for {
                return Ok(());
            }
        }

        Err(validation_err_code(
            "No attenuations found in capability object for this resource".to_string(),
            EC::NodeSIWECapabilityInvalid,
            None,
        ))
    }
}

/// `LitAbility` enums mapped into Recap-specific abilities.
pub(crate) enum LitRecapAbility {
    Decryption,
    Signing,
    Auth,
    Execution,
}

impl fmt::Display for LitRecapAbility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LitRecapAbility::Decryption => write!(f, "Decryption"),
            LitRecapAbility::Signing => write!(f, "Signing"),
            LitRecapAbility::Auth => write!(f, "Auth"),
            LitRecapAbility::Execution => write!(f, "Execution"),
        }
    }
}

/// `LitResource` enums mapped into Recap-specific namespaces.
pub(crate) enum LitRecapNamespace {
    Auth,
    Threshold,
}

impl fmt::Display for LitRecapNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LitRecapNamespace::Auth => write!(f, "Auth"),
            LitRecapNamespace::Threshold => write!(f, "Threshold"),
        }
    }
}

/// Mapper function to map `LitAbility` enums into Recap-specific namespaces and abilities.
pub(crate) fn get_recap_namespace_and_ability(
    lit_ability: &LitAbility,
) -> Result<(LitRecapNamespace, LitRecapAbility)> {
    match lit_ability {
        LitAbility::AccessControlConditionDecryption => {
            Ok((LitRecapNamespace::Threshold, LitRecapAbility::Decryption))
        }
        LitAbility::AccessControlConditionSigning => {
            Ok((LitRecapNamespace::Threshold, LitRecapAbility::Signing))
        }
        LitAbility::PKPSigning => Ok((LitRecapNamespace::Threshold, LitRecapAbility::Signing)),
        LitAbility::RateLimitIncreaseAuth => Ok((LitRecapNamespace::Auth, LitRecapAbility::Auth)),
        LitAbility::LitActionExecution => {
            Ok((LitRecapNamespace::Threshold, LitRecapAbility::Execution))
        }
    }
}

#[cfg(test)]
mod extract_and_verify_tests {
    use chrono::Duration;
    use serde_json::Value;
    use siwe::Message;
    use siwe_recap::Capability;
    use std::ops::{Add, Sub};

    use crate::auth::resources::AccessControlConditionResource;
    use crate::auth::resources::LitActionResource;
    use crate::models::auth::LitResourcePrefix;

    use super::extract_and_verify_all_capabilities;
    use crate::auth::capabilities::session_capability_object::SessionCapabilityObject;

    #[test]
    fn test_extract_and_verify_with_two_recaps_in_one_resource() {
        let now = chrono::Utc::now();
        let siwe_issued_at = now.sub(Duration::days(1));
        let siwe_expiration_time = now.add(Duration::days(7));
        let mut capabilities = Capability::<Value>::default();

        let resource = "Threshold/Decryption".to_string();
        let resource_prefix = format!("{}://*", LitResourcePrefix::ACC);
        let capabilities = capabilities.with_actions_convert(resource_prefix, [(resource, [])]);
        if let Err(e) = capabilities {
            panic!("Error: {:?}", e);
        }
        let capabilities = capabilities.unwrap();

        let resource = "*/*".to_string();
        let resource_prefix = format!("{}://*", LitResourcePrefix::LA);
        let capabilities = capabilities.with_actions_convert(resource_prefix, [(resource, [])]);
        if let Err(e) = capabilities {
            panic!("Error: {:?}", e);
        }
        let capabilities = capabilities.unwrap();

        let siwe_message: siwe::Message = capabilities
            .build_message(siwe::Message {
                domain: "example.com".parse().unwrap(),
                address: Default::default(),
                statement: None,
                uri: "lit:capability:delegation".parse().unwrap(),
                version: siwe::Version::V1,
                chain_id: 1,
                nonce: "mynonce1".into(),
                issued_at: siwe_issued_at
                    .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                    .parse()
                    .unwrap(),
                expiration_time: Some(
                    siwe_expiration_time
                        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                        .parse()
                        .unwrap(),
                ),
                not_before: None,
                request_id: None,
                resources: vec![],
            })
            .unwrap();

        let session_capability_objects = extract_and_verify_all_capabilities(&siwe_message);
        println!("{:?}", session_capability_objects);
        assert!(session_capability_objects.is_ok());
        let session_capability_objects = session_capability_objects.unwrap();
        assert_eq!(session_capability_objects.len(), 1);

        let session_capability_object = session_capability_objects.get(0).unwrap();
        let verify_res = session_capability_object.verify_capabilities_for_resource(
            &AccessControlConditionResource::new("blah".into()).decrypt_ability(),
        );
        assert!(verify_res.is_ok());

        let verify_res = session_capability_object.verify_capabilities_for_resource(
            &LitActionResource::new("blah".into()).execution_ability(),
        );
        assert!(verify_res.is_ok());
    }

    #[test]
    fn test_extract_and_verify_with_one_recap_but_two_resources() {
        let now = chrono::Utc::now();
        let siwe_issued_at = now.sub(Duration::days(1));
        let siwe_expiration_time = now.add(Duration::days(7));
        let mut capabilities = Capability::<Value>::default();

        let resource = "Threshold/Decryption".to_string();
        let resource_prefix = format!("{}://*", LitResourcePrefix::ACC);
        let capabilities = capabilities.with_actions_convert(resource_prefix, [(resource, [])]);
        if let Err(e) = capabilities {
            panic!("Error: {:?}", e);
        }
        let capabilities = capabilities.unwrap();

        let siwe_message: siwe::Message = capabilities
            .build_message(siwe::Message {
                domain: "example.com".parse().unwrap(),
                address: Default::default(),
                statement: None,
                uri: "lit:capability:delegation".parse().unwrap(),
                version: siwe::Version::V1,
                chain_id: 1,
                nonce: "mynonce1".into(),
                issued_at: siwe_issued_at
                    .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                    .parse()
                    .unwrap(),
                expiration_time: Some(
                    siwe_expiration_time
                        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                        .parse()
                        .unwrap(),
                ),
                not_before: None,
                request_id: None,
                resources: vec!["meow://woof".parse().unwrap()],
            })
            .unwrap();

        let session_capability_objects = extract_and_verify_all_capabilities(&siwe_message);
        println!("{:?}", session_capability_objects);
        assert!(session_capability_objects.is_ok());
        let session_capability_objects = session_capability_objects.unwrap();
        assert_eq!(session_capability_objects.len(), 1);

        let session_capability_object = session_capability_objects.get(0).unwrap();
        let verify_res = session_capability_object.verify_capabilities_for_resource(
            &AccessControlConditionResource::new("blah".into()).decrypt_ability(),
        );
        assert!(verify_res.is_ok());
    }

    #[test]
    fn test_extract_and_verify_no_caps() {
        let siwe_message = Message {
            domain: "example.com".parse().unwrap(),
            address: Default::default(),
            statement: None,
            uri: "did:key:example".parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "mynonce1".into(),
            issued_at: "2022-06-21T12:00:00.000Z".parse().unwrap(),
            expiration_time: None,
            not_before: None,
            request_id: None,
            resources: vec![],
        };

        let session_capability_objects = extract_and_verify_all_capabilities(&siwe_message);
        assert!(session_capability_objects.is_ok());
        let session_capability_objects = session_capability_objects.unwrap();
        assert!(session_capability_objects.is_empty());
    }
}

#[cfg(test)]
mod recap_tests {
    use siwe_recap::Capability;

    use crate::auth::resources::AccessControlConditionResource;

    use super::RecapSessionCapabilityObject;

    struct TestCase {
        recap_obj: RecapSessionCapabilityObject,
        lit_resource: AccessControlConditionResource,
        expected_resource_key: String,
    }

    fn test_get_resource_key_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                recap_obj: RecapSessionCapabilityObject {
                    inner: Capability::new()
                        .with_action_convert(
                            "lit-accesscontrolcondition://*",
                            "Threshold/Decryption",
                            [],
                        )
                        .unwrap()
                        .to_owned(),
                },
                lit_resource: AccessControlConditionResource::new("123".into()),
                expected_resource_key: "lit-accesscontrolcondition://*".to_string(),
            },
            TestCase {
                recap_obj: RecapSessionCapabilityObject {
                    inner: Capability::new()
                        .with_action_convert(
                            "lit-accesscontrolcondition://123",
                            "Threshold/Decryption",
                            [],
                        )
                        .unwrap()
                        .to_owned(),
                },
                lit_resource: AccessControlConditionResource::new("123".into()),
                expected_resource_key: "lit-accesscontrolcondition://123".to_string(),
            },
            TestCase {
                recap_obj: RecapSessionCapabilityObject {
                    inner: Capability::new()
                        .with_action_convert(
                            "lit-accesscontrolcondition://123",
                            "Threshold/Decryption",
                            [],
                        )
                        .unwrap()
                        .to_owned(),
                },
                lit_resource: AccessControlConditionResource::new("456".into()),
                expected_resource_key: "lit-invalid://*".to_string(),
            },
        ]
    }

    #[test]
    fn test_get_resource_key() {
        let test_cases = test_get_resource_key_test_cases();

        for test_case in test_cases {
            let resource_key = test_case
                .recap_obj
                .get_resource_key_to_match_against(test_case.lit_resource);
            assert!(resource_key.is_ok());
            let resource_key = resource_key.unwrap();

            assert_eq!(resource_key.to_string(), test_case.expected_resource_key);
        }
    }
}

#[cfg(test)]
mod session_capability_object_tests {

    use lit_core::error::Kind;
    use siwe_recap::Capability;

    use crate::{
        auth::{
            capabilities::{
                recap::{LitRecapAbility, LitRecapNamespace, RecapSessionCapabilityObject},
                session_capability_object::SessionCapabilityObject,
            },
            resources::AccessControlConditionResource,
        },
        error::EC,
    };

    #[test]
    fn test_verify_capabilities_for_resource_no_key() {
        let recap_obj = RecapSessionCapabilityObject {
            inner: Capability::new()
                .with_action_convert(
                    "lit-accesscontrolcondition://123",
                    format!(
                        "{}/{}",
                        LitRecapNamespace::Threshold,
                        LitRecapAbility::Decryption
                    )
                    .as_str(),
                    [],
                )
                .unwrap()
                .to_owned(),
        };

        let verify = recap_obj.verify_capabilities_for_resource(
            &AccessControlConditionResource::new("blah".into()).decrypt_ability(),
        );
        assert!(verify.is_err());
        let err = verify.unwrap_err();
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err.is_code(EC::NodeSIWECapabilityInvalid, false));
        assert!(err
            .to_string()
            .contains("No capability object found for this resource"));
    }

    #[test]
    fn test_verify_capabilities_for_resource_no_attenuations() {
        let recap_obj = RecapSessionCapabilityObject {
            inner: Capability::new()
                .with_action_convert(
                    "lit-accesscontrolcondition://123",
                    format!(
                        "{}/{}",
                        LitRecapNamespace::Threshold,
                        LitRecapAbility::Signing
                    )
                    .as_str(),
                    [],
                )
                .unwrap()
                .to_owned(),
        };

        let verify = recap_obj.verify_capabilities_for_resource(
            &AccessControlConditionResource::new("123".into()).decrypt_ability(),
        );
        assert!(verify.is_err());
        let err = verify.unwrap_err();
        assert!(err.is_kind(Kind::Validation, false));
        assert!(err.is_code(EC::NodeSIWECapabilityInvalid, false));
        assert!(err
            .to_string()
            .contains("No attenuations found in capability object for this resource"));
    }

    #[test]
    fn test_verify_capabilities_for_resource_exact_ability() {
        let recap_obj = RecapSessionCapabilityObject {
            inner: Capability::new()
                .with_action_convert(
                    "lit-accesscontrolcondition://123",
                    format!(
                        "{}/{}",
                        LitRecapNamespace::Threshold,
                        LitRecapAbility::Decryption
                    )
                    .as_str(),
                    [],
                )
                .unwrap()
                .to_owned(),
        };

        let verify = recap_obj.verify_capabilities_for_resource(
            &AccessControlConditionResource::new("123".into()).decrypt_ability(),
        );
        assert!(verify.is_ok());
    }

    #[test]
    fn test_verify_capabilities_for_resource_wildcard_ability() {
        let recap_obj = RecapSessionCapabilityObject {
            inner: Capability::new()
                .with_action_convert("lit-accesscontrolcondition://123", "*/*", [])
                .unwrap()
                .to_owned(),
        };

        let verify = recap_obj.verify_capabilities_for_resource(
            &AccessControlConditionResource::new("123".into()).decrypt_ability(),
        );
        assert!(verify.is_ok());
    }
}
