use clap::builder::PossibleValue;
use clap::ValueEnum;
use lit_blockchain::contracts::release::{ReleasePlatform, ReleaseType};
use serde::{Deserialize, Serialize};
use sev_snp_utilities::CpuType;
use std::fmt;
use std::str::FromStr;

use crate::error::conversion_err;

// GuestType

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(unused)]
pub enum GuestType {
    Node = 0,
    Prov = 1,
    Build = 2,
    Custom = 3,
}

impl GuestType {
    pub fn eq_str(&self, other: &str) -> bool {
        format!("{self}").eq(other)
    }

    pub fn to_release_type(&self) -> ReleaseType {
        match self {
            GuestType::Node => ReleaseType::Node,
            GuestType::Prov => ReleaseType::Prov,
            GuestType::Build => ReleaseType::Build,
            GuestType::Custom => ReleaseType::Custom,
        }
    }
}

impl fmt::Display for GuestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuestType::Node => write!(f, "node"),
            GuestType::Prov => write!(f, "prov"),
            GuestType::Build => write!(f, "build"),
            GuestType::Custom => write!(f, "custom"),
        }
    }
}

impl FromStr for GuestType {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "node" => Ok(GuestType::Node),
            "prov" => Ok(GuestType::Prov),
            "build" => Ok(GuestType::Build),
            "custom" => Ok(GuestType::Custom),
            _ => Err(conversion_err(format!("unable to create GuestType from_str: {s}"), None)),
        }
    }
}

impl ValueEnum for GuestType {
    fn value_variants<'a>() -> &'a [Self] {
        &[GuestType::Node, GuestType::Prov, GuestType::Build]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            GuestType::Node => PossibleValue::new("node"),
            GuestType::Prov => PossibleValue::new("prov"),
            GuestType::Build => PossibleValue::new("build"),
            GuestType::Custom => PossibleValue::new("custom"),
        })
    }
}

impl From<ReleaseType> for GuestType {
    fn from(value: ReleaseType) -> Self {
        match value {
            ReleaseType::Node => GuestType::Node,
            ReleaseType::Prov => GuestType::Prov,
            ReleaseType::Build => GuestType::Build,
            ReleaseType::Custom => GuestType::Custom,
        }
    }
}

impl From<GuestType> for ReleaseType {
    fn from(val: GuestType) -> Self {
        match val {
            GuestType::Node => ReleaseType::Node,
            GuestType::Prov => ReleaseType::Prov,
            GuestType::Build => ReleaseType::Build,
            GuestType::Custom => ReleaseType::Custom,
        }
    }
}

// GuestCpuType

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub enum GuestCpuType {
    EPYCv4 = 0,
}

impl From<GuestCpuType> for ReleasePlatform {
    fn from(val: GuestCpuType) -> Self {
        match val {
            GuestCpuType::EPYCv4 => ReleasePlatform::MetalAmdSev,
        }
    }
}

impl From<GuestCpuType> for CpuType {
    fn from(val: GuestCpuType) -> Self {
        match val {
            GuestCpuType::EPYCv4 => CpuType::EpycV4,
        }
    }
}

impl fmt::Display for GuestCpuType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuestCpuType::EPYCv4 => write!(f, "EPYC-v4"),
        }
    }
}

impl FromStr for GuestCpuType {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EPYC-v4" => Ok(GuestCpuType::EPYCv4),
            _ => Err(conversion_err(format!("unable to create GuestCpuType from_str: {s}"), None)),
        }
    }
}

impl ValueEnum for GuestCpuType {
    fn value_variants<'a>() -> &'a [Self] {
        &[GuestCpuType::EPYCv4]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            GuestCpuType::EPYCv4 => PossibleValue::new("EPYC-v4"),
        })
    }
}

// GuestOsType

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[allow(unused)]
pub enum GuestOsType {
    Debian = 1,
}

impl fmt::Display for GuestOsType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuestOsType::Debian => write!(f, "debian"),
        }
    }
}

impl FromStr for GuestOsType {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "debian" => Ok(GuestOsType::Debian),
            _ => Err(conversion_err(format!("unable to create GuestOsType from_str: {s}"), None)),
        }
    }
}

impl ValueEnum for GuestOsType {
    fn value_variants<'a>() -> &'a [Self] {
        &[GuestOsType::Debian]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            GuestOsType::Debian => PossibleValue::new("debian"),
        })
    }
}
