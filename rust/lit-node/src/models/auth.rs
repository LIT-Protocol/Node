use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use super::JsonAuthSig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionKeySignedMessage {
    pub session_key: String,
    pub resource_ability_requests: Vec<LitResourceAbilityRequest>,
    pub capabilities: Vec<JsonAuthSig>,
    pub issued_at: String,
    pub expiration: String,
    pub node_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LitResourceAbilityRequest {
    pub resource: LitResourceAbilityRequestResource,
    pub ability: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LitResourceAbilityRequestResource {
    /// The resource ID
    pub resource: String,
    /// The resource prefix
    pub resource_prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LitAbility {
    // Used by top level auth sigs
    AccessControlConditionDecryption,
    AccessControlConditionSigning,
    PKPSigning,
    RateLimitIncreaseAuth,
    LitActionExecution,
}

impl fmt::Display for LitAbility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LitAbility::AccessControlConditionDecryption => {
                write!(f, "access-control-condition-decryption")
            }
            LitAbility::AccessControlConditionSigning => {
                write!(f, "access-control-condition-signing")
            }
            LitAbility::PKPSigning => write!(f, "pkp-signing"),
            LitAbility::RateLimitIncreaseAuth => write!(f, "rate-limit-increase-auth"),
            LitAbility::LitActionExecution => write!(f, "lit-action-execution"),
        }
    }
}

// USER DEFINED
const LIT_RESOURCE_PREFIX_ACC: &str = "lit-accesscontrolcondition";
const LIT_RESOURCE_PREFIX_PKP: &str = "lit-pkp";
const LIT_RESOURCE_PREFIX_RLI: &str = "lit-ratelimitincrease";
const LIT_RESOURCE_PREFIX_LA: &str = "lit-litaction";

// NODE DEFINED
pub const LIT_RESOURCE_PREFIX_RAC: &str = "lit-resolvedauthcontext";
pub const LIT_RESOURCE_KEY_RAC: &str = "auth_context";

#[derive(PartialEq, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum LitResourcePrefix {
    ACC,
    PKP,
    RLI,
    LA,
}

impl fmt::Display for LitResourcePrefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LitResourcePrefix::ACC => write!(f, "{}", LIT_RESOURCE_PREFIX_ACC),
            LitResourcePrefix::PKP => write!(f, "{}", LIT_RESOURCE_PREFIX_PKP),
            LitResourcePrefix::RLI => write!(f, "{}", LIT_RESOURCE_PREFIX_RLI),
            LitResourcePrefix::LA => write!(f, "{}", LIT_RESOURCE_PREFIX_LA),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLitResourcePrefixError;

impl FromStr for LitResourcePrefix {
    type Err = ParseLitResourcePrefixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("://").collect();
        let uri_without_data = parts.first().unwrap_or(&s).to_owned();
        match s {
            LIT_RESOURCE_PREFIX_ACC => Ok(LitResourcePrefix::ACC),
            LIT_RESOURCE_PREFIX_PKP => Ok(LitResourcePrefix::PKP),
            LIT_RESOURCE_PREFIX_RLI => Ok(LitResourcePrefix::RLI),
            LIT_RESOURCE_PREFIX_LA => Ok(LitResourcePrefix::LA),
            _ => Err(ParseLitResourcePrefixError),
        }
    }
}
