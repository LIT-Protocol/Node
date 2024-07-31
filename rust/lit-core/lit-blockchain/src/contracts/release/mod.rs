use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::{conversion_err, Result};

// Also update ReleaseRegister.sol
pub const RELEASE_OPTION_RO: u8 = 1 << 1;
pub const RELEASE_OPTION_USERS: u8 = 1 << 2;
pub const RELEASE_OPTION_SSH: u8 = 1 << 3;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(unused)]
pub enum ReleaseStatus {
    Null = 0,
    Pending = 1,
    Active = 2,
    Disabled = 3,
}

impl TryFrom<u8> for ReleaseStatus {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(ReleaseStatus::Null),
            1 => Ok(ReleaseStatus::Pending),
            2 => Ok(ReleaseStatus::Active),
            3 => Ok(ReleaseStatus::Disabled),
            _ => {
                return Err(conversion_err(
                    format!("value: '{}' cannot map to ReleaseStatus", value),
                    None,
                ));
            }
        }
    }
}

impl FromStr for ReleaseStatus {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "null" => Ok(ReleaseStatus::Null),
            "pending" => Ok(ReleaseStatus::Pending),
            "active" => Ok(ReleaseStatus::Active),
            "disabled" => Ok(ReleaseStatus::Disabled),
            _ => {
                Err(conversion_err(format!("unable to create ReleaseStatus from_str: {}", s), None))
            }
        }
    }
}

impl fmt::Display for ReleaseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReleaseStatus::Null => write!(f, "null"),
            ReleaseStatus::Pending => write!(f, "pending"),
            ReleaseStatus::Active => write!(f, "active"),
            ReleaseStatus::Disabled => write!(f, "disabled"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(unused)]
pub enum ReleaseType {
    Node = 0,
    Prov = 1,
    Build = 2,
    Custom = 3,
}

impl FromStr for ReleaseType {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "node" => Ok(ReleaseType::Node),
            "prov" => Ok(ReleaseType::Prov),
            "build" => Ok(ReleaseType::Build),
            "custom" => Ok(ReleaseType::Custom),
            _ => Err(conversion_err(format!("unable to create ReleaseType from_str: {}", s), None)),
        }
    }
}

impl TryFrom<u8> for ReleaseType {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(ReleaseType::Node),
            1 => Ok(ReleaseType::Prov),
            2 => Ok(ReleaseType::Build),
            3 => Ok(ReleaseType::Custom),
            _ => {
                return Err(conversion_err(
                    format!("value: '{}' cannot map to ReleaseType", value),
                    None,
                ));
            }
        }
    }
}

impl fmt::Display for ReleaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReleaseType::Node => write!(f, "node"),
            ReleaseType::Prov => write!(f, "prov"),
            ReleaseType::Build => write!(f, "build"),
            ReleaseType::Custom => write!(f, "custom"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(unused)]
pub enum ReleasePlatform {
    MetalAmdSev = 0,
}

impl TryFrom<u8> for ReleasePlatform {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(ReleasePlatform::MetalAmdSev),
            _ => {
                return Err(conversion_err(
                    format!("value: '{}' cannot map to ReleasePlatform", value),
                    None,
                ));
            }
        }
    }
}

impl FromStr for ReleasePlatform {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "metal_mmd_sev" => Ok(ReleasePlatform::MetalAmdSev),
            _ => Err(conversion_err(
                format!("unable to create ReleasePlatform from_str: {}", s),
                None,
            )),
        }
    }
}

impl fmt::Display for ReleasePlatform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReleasePlatform::MetalAmdSev => write!(f, "metal_mmd_sev"),
        }
    }
}
