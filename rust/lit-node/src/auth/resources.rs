use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

use crate::error::{parser_err, Result};
use crate::models::auth::{LitAbility, LitResourcePrefix};

use super::lit_resource::LitResource;

#[derive(Debug, Clone)]
pub enum ResourceType {
    AccessControlCondition(AccessControlConditionResource),
    PKPNFT(PKPNFTResource),
    RateLimitIncreaseNFT(RateLimitIncreaseNFTResource),
    LitAction(LitActionResource),
}

pub(crate) fn get_resource_prefix_id_from_type(
    resource_type: &ResourceType,
) -> Result<(String, String)> {
    match resource_type {
        // TODO: code smell - refactor this
        ResourceType::AccessControlCondition(resource) => Ok((
            resource.get_resource_prefix().to_string(),
            resource.get_resource_id().to_owned(),
        )),
        ResourceType::PKPNFT(resource) => Ok((
            resource.get_resource_prefix().to_string(),
            resource.get_resource_id().to_owned(),
        )),
        ResourceType::RateLimitIncreaseNFT(resource) => Ok((
            resource.get_resource_prefix().to_string(),
            resource.get_resource_id().to_owned(),
        )),
        ResourceType::LitAction(resource) => Ok((
            resource.get_resource_prefix().to_string(),
            resource.get_resource_id().to_owned(),
        )),
    }
}

pub(crate) fn parse_resource_and_prefix<T>(
    resource_id: T,
    resource_prefix: T,
) -> Result<Arc<dyn LitResource>>
where
    T: AsRef<str>,
{
    let resource_prefix = resource_prefix.as_ref();
    let resource_id = resource_id.as_ref();

    match LitResourcePrefix::from_str(resource_prefix)
        .map_err(|e| parser_err("Unable to parse lit resource prefix", None))?
    {
        LitResourcePrefix::ACC => Ok(Arc::new(AccessControlConditionResource::new(
            resource_id.to_owned(),
        ))),
        LitResourcePrefix::PKP => Ok(Arc::new(PKPNFTResource::new(resource_id.to_owned()))),
        LitResourcePrefix::RLI => Ok(Arc::new(RateLimitIncreaseNFTResource::new(
            resource_id.to_owned(),
        ))),
        LitResourcePrefix::LA => Ok(Arc::new(LitActionResource::new(resource_id.to_owned()))),
    }
}

/// A `LitResourceAbility` specifies a LIT-specific ability that
/// is requested to be performed on a resource.
///
/// Since this struct can only be created from a LIT-specific resource
/// (eg. `AccessControlConditionResource` or `PKPNFTResource`) it is
/// guaranteed that the ability is compatible with the resource. For example,
/// a `PKPNFTResource` can only be used for signing, and an `AccessControlConditionResource`
/// can only be used for decryption or signing.
///
/// For example, to create a `LitResourceAbility` for a `PKPNFTResource` that
/// can be used for signing:
/// ```
/// let resource = PKPNFTResource::new("123".to_string());
/// let resource_ability = resource.sign_ability();
/// ```
#[derive(Debug, Clone)]
pub struct LitResourceAbility {
    resource: ResourceType,
    ability: LitAbility,
}

impl LitResourceAbility {
    pub fn get_resource(&self) -> &ResourceType {
        &self.resource
    }

    pub fn get_ability(&self) -> &LitAbility {
        &self.ability
    }
}

#[derive(Debug, Clone)]
pub struct AccessControlConditionResource {
    resource_id: String,
}

impl AccessControlConditionResource {
    pub fn new(resource_id: String) -> AccessControlConditionResource {
        AccessControlConditionResource { resource_id }
    }

    pub fn get_resource_id(&self) -> &String {
        &self.resource_id
    }

    pub fn decrypt_ability(&self) -> LitResourceAbility {
        LitResourceAbility {
            resource: ResourceType::AccessControlCondition(self.clone()),
            ability: LitAbility::AccessControlConditionDecryption,
        }
    }

    pub fn signing_ability(&self) -> LitResourceAbility {
        LitResourceAbility {
            resource: ResourceType::AccessControlCondition(self.clone()),
            ability: LitAbility::AccessControlConditionSigning,
        }
    }
}

impl LitResource for AccessControlConditionResource {
    fn get_resource_id(&self) -> &String {
        self.get_resource_id()
    }

    fn get_resource_prefix(&self) -> LitResourcePrefix {
        LitResourcePrefix::ACC
    }
}

#[derive(Debug, Clone)]
pub struct PKPNFTResource {
    token_id: String,
}

impl PKPNFTResource {
    pub fn new(token_id: String) -> PKPNFTResource {
        PKPNFTResource { token_id }
    }

    pub fn get_resource_id(&self) -> &String {
        &self.token_id
    }

    pub fn signing_ability(&self) -> LitResourceAbility {
        LitResourceAbility {
            resource: ResourceType::PKPNFT(self.clone()),
            ability: LitAbility::PKPSigning,
        }
    }
}

impl LitResource for PKPNFTResource {
    fn get_resource_id(&self) -> &String {
        self.get_resource_id()
    }

    fn get_resource_prefix(&self) -> LitResourcePrefix {
        LitResourcePrefix::PKP
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitIncreaseNFTResource {
    token_id: String,
}

impl RateLimitIncreaseNFTResource {
    pub fn new(token_id: String) -> RateLimitIncreaseNFTResource {
        RateLimitIncreaseNFTResource { token_id }
    }

    pub fn get_resource_id(&self) -> &String {
        &self.token_id
    }

    pub fn signing_ability(&self) -> LitResourceAbility {
        LitResourceAbility {
            resource: ResourceType::RateLimitIncreaseNFT(self.clone()),
            ability: LitAbility::RateLimitIncreaseAuth,
        }
    }
}

impl LitResource for RateLimitIncreaseNFTResource {
    fn get_resource_id(&self) -> &String {
        self.get_resource_id()
    }

    fn get_resource_prefix(&self) -> LitResourcePrefix {
        LitResourcePrefix::RLI
    }
}

#[derive(Debug, Clone)]
pub struct LitActionResource {
    cid: String,
}

impl LitActionResource {
    pub fn new(cid: String) -> LitActionResource {
        LitActionResource { cid }
    }

    pub fn get_resource_id(&self) -> &String {
        &self.cid
    }

    pub fn execution_ability(&self) -> LitResourceAbility {
        LitResourceAbility {
            resource: ResourceType::LitAction(self.clone()),
            ability: LitAbility::LitActionExecution,
        }
    }
}

impl LitResource for LitActionResource {
    fn get_resource_id(&self) -> &String {
        self.get_resource_id()
    }

    fn get_resource_prefix(&self) -> LitResourcePrefix {
        LitResourcePrefix::LA
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        auth::resources::get_resource_prefix_id_from_type, models::auth::LitResourcePrefix,
    };

    use super::ResourceType;
    use crate::auth::resources::LitAbility;
    use crate::auth::resources::LitResource;

    struct TestCase {
        resource_type: ResourceType,
        expected_prefix: LitResourcePrefix,
        expected_id: String,
    }

    fn get_resource_prefix_id_from_type_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                resource_type: ResourceType::AccessControlCondition(
                    crate::auth::resources::AccessControlConditionResource::new("123".to_string()),
                ),
                expected_prefix: LitResourcePrefix::ACC,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_type: ResourceType::PKPNFT(crate::auth::resources::PKPNFTResource::new(
                    "123".to_string(),
                )),
                expected_prefix: LitResourcePrefix::PKP,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_type: ResourceType::RateLimitIncreaseNFT(
                    crate::auth::resources::RateLimitIncreaseNFTResource::new("123".to_string()),
                ),
                expected_prefix: LitResourcePrefix::RLI,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_type: ResourceType::LitAction(
                    crate::auth::resources::LitActionResource::new("123".to_string()),
                ),
                expected_prefix: LitResourcePrefix::LA,
                expected_id: "123".to_string(),
            },
        ]
    }

    #[test]
    fn test_get_resource_prefix_id_from_type() {
        let test_cases = get_resource_prefix_id_from_type_test_cases();

        for test_case in test_cases {
            let (prefix, id) = get_resource_prefix_id_from_type(&test_case.resource_type).unwrap();
            assert_eq!(prefix, test_case.expected_prefix.to_string());
            assert_eq!(id, test_case.expected_id);
        }
    }

    #[test]
    fn test_resource_methods() {
        let acc = crate::auth::resources::AccessControlConditionResource::new("123".to_string());
        let pkp = crate::auth::resources::PKPNFTResource::new("123".to_string());
        let rli = crate::auth::resources::RateLimitIncreaseNFTResource::new("123".to_string());
        let la = crate::auth::resources::LitActionResource::new("123".to_string());

        assert_eq!(acc.get_resource_id(), "123");
        assert_eq!(pkp.get_resource_id(), "123");
        assert_eq!(rli.get_resource_id(), "123");
        assert_eq!(la.get_resource_id(), "123");

        assert_eq!(acc.get_resource_prefix(), LitResourcePrefix::ACC);
        assert_eq!(pkp.get_resource_prefix(), LitResourcePrefix::PKP);
        assert_eq!(rli.get_resource_prefix(), LitResourcePrefix::RLI);
        assert_eq!(la.get_resource_prefix(), LitResourcePrefix::LA);

        assert_eq!(
            acc.decrypt_ability().get_ability().to_owned(),
            LitAbility::AccessControlConditionDecryption
        );
        assert_eq!(
            acc.signing_ability().get_ability().to_owned(),
            LitAbility::AccessControlConditionSigning
        );

        assert_eq!(
            pkp.signing_ability().get_ability().to_owned(),
            LitAbility::PKPSigning
        );

        assert_eq!(
            rli.signing_ability().get_ability().to_owned(),
            LitAbility::RateLimitIncreaseAuth
        );

        assert_eq!(
            la.execution_ability().get_ability().to_owned(),
            LitAbility::LitActionExecution
        );
    }
}

#[cfg(test)]
mod parse_resource_and_prefix_tests {
    use crate::{auth::resources::parse_resource_and_prefix, models::auth::LitResourcePrefix};

    struct TestCase {
        resource_id: String,
        resource_prefix: String,
        expected_prefix: LitResourcePrefix,
        expected_id: String,
    }

    fn parse_resource_and_prefix_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                resource_id: "123".to_string(),
                resource_prefix: "lit-accesscontrolcondition".to_string(),
                expected_prefix: LitResourcePrefix::ACC,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_id: "123".to_string(),
                resource_prefix: "lit-pkp".to_string(),
                expected_prefix: LitResourcePrefix::PKP,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_id: "123".to_string(),
                resource_prefix: "lit-ratelimitincrease".to_string(),
                expected_prefix: LitResourcePrefix::RLI,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_id: "123".to_string(),
                resource_prefix: "lit-litaction".to_string(),
                expected_prefix: LitResourcePrefix::LA,
                expected_id: "123".to_string(),
            },
        ]
    }

    #[test]
    fn test_parse_resource_and_prefix() {
        let test_cases = parse_resource_and_prefix_test_cases();

        for test_case in test_cases {
            let lit_resource =
                parse_resource_and_prefix(&test_case.resource_id, &test_case.resource_prefix)
                    .unwrap();
            assert_eq!(
                lit_resource.get_resource_prefix(),
                test_case.expected_prefix
            );
            assert_eq!(
                lit_resource.get_resource_id().to_owned(),
                test_case.expected_id
            );
        }
    }
}
