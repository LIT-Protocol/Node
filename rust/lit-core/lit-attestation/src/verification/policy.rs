use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use lit_blockchain::contracts::release::ReleaseType;
use lit_core::config::envs::LitEnv;

use crate::attestation::AdminSignedType;
use crate::AttestationType;

/// Naming could be better but I wanted to be able to use `Policy` (below) in most cases vs. `DefaultPolicy`.
pub trait VerificationPolicy: Send + Sync + 'static {
    /// Policy name (for debug and logging purposes)
    fn name(&self) -> String;
    /// Require one of the returned AdminSignedType's
    fn require_signed(&self) -> Option<Vec<AdminSignedType>>;
    /// Only allow these AttestationType's
    fn allowed_attestation_types(&self) -> Option<Vec<AttestationType>>;
    /// Allow unsigned requests (i.e. no release build signature)
    fn allow_unsigned(&self) -> bool;
    /// Allow subnet
    fn allow_subnet(&self, our: &str, other: &str) -> bool;
    /// Allow environment
    fn allow_env(&self, our: &LitEnv, other: &LitEnv) -> bool;
    /// Allow release (guest) type
    fn allow_type(&self, our: &ReleaseType, other: &ReleaseType) -> bool;
    /// Require attestations originate from a secure faclity
    fn require_facility_service(&self) -> bool;
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Policy {
    #[default]
    Default,
    /// Used during the guest initrd process to obtain the initial passphrase.
    Init,
    /// Used during the guest initrd process for self verification.
    SelfVerify,
    /// Allow admin-signed requests only.
    Admin,
    /// Allow admin-signed or operator-signed requests only.
    AdminOrOperator,
    /// Used between cluster members.
    Cluster,
    /// Used between subnet members.
    Subnet,
    /// Used when nodes connect to eachother
    NodeConnect,
}

impl fmt::Display for Policy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Policy::Default => write!(f, "DEFAULT"),
            Policy::Init => write!(f, "INIT"),
            Policy::SelfVerify => write!(f, "SELF_VERIFY"),
            Policy::Admin => write!(f, "ADMIN"),
            Policy::AdminOrOperator => write!(f, "ADMIN_OR_OPERATOR"),
            Policy::NodeConnect => write!(f, "NODE_CONNECT"),
            Policy::Cluster => write!(f, "CLUSTER"),
            Policy::Subnet => write!(f, "SUBNET"),
        }
    }
}

impl VerificationPolicy for Policy {
    fn name(&self) -> String {
        self.to_string()
    }

    fn require_signed(&self) -> Option<Vec<AdminSignedType>> {
        match self {
            Policy::Admin => Some(vec![AdminSignedType::Admin]),
            Policy::AdminOrOperator => {
                Some(vec![AdminSignedType::Admin, AdminSignedType::Operator])
            }
            Policy::Subnet => Some(vec![AdminSignedType::Operator]),
            _ => None,
        }
    }

    fn allowed_attestation_types(&self) -> Option<Vec<AttestationType>> {
        match self {
            // TODO: Currently limiting admin signed.
            // - Eventually we will need to support admin via guest however.
            // - I think this will be FINE, I just want to verify once we cross that bridge.
            // - Presently 'require_signed' will validate signatures for any AttestationType.
            //   For AdminSigned this will then just return Ok(). So to add support for guest 'admin', if they
            //   also have an admin private key - it may be perfectly safe. Otherwise, we'll need to investigate.
            Policy::Admin | Policy::AdminOrOperator => Some(vec![AttestationType::AdminSigned]),
            Policy::NodeConnect => Some(vec![AttestationType::AmdSevSnp]),
            _ => None,
        }
    }

    fn allow_unsigned(&self) -> bool {
        matches!(self, Policy::Init | Policy::SelfVerify)
    }

    fn allow_subnet(&self, our: &str, other: &str) -> bool {
        match self {
            Policy::Admin => true,
            _ => our.eq(other),
        }
    }

    fn allow_env(&self, our: &LitEnv, other: &LitEnv) -> bool {
        // Prod only prods.
        if (LitEnv::Prod.eq(our) && !LitEnv::Prod.eq(other))
            || (LitEnv::Prod.eq(other) && !LitEnv::Prod.eq(our))
        {
            return false;
        }

        match self {
            Policy::Admin => true,
            _ => our.eq(other),
        }
    }

    fn allow_type(&self, our: &ReleaseType, other: &ReleaseType) -> bool {
        match self {
            Policy::Default | Policy::Admin | Policy::Init => true,
            _ => our.eq(other),
        }
    }

    fn require_facility_service(&self) -> bool {
        !matches!(self, Policy::NodeConnect | Policy::Admin | Policy::Init | Policy::SelfVerify)
    }
}
