use std::fmt;
use std::str::FromStr;

use crate::error::conversion_err;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[allow(unused)]
pub enum LitEnv {
    #[default]
    Dev = 0,
    Staging = 1,
    Prod = 2,
}

impl LitEnv {
    pub fn eq_str(&self, other: &str) -> bool {
        format!("{self}").eq(other)
    }
}

impl TryFrom<u8> for LitEnv {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> crate::error::Result<Self> {
        match value {
            0 => Ok(LitEnv::Dev),
            1 => Ok(LitEnv::Staging),
            2 => Ok(LitEnv::Prod),
            _ => Err(conversion_err(format!("value: '{value}' cannot map to LitEnv"), None)),
        }
    }
}

impl fmt::Display for LitEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LitEnv::Dev => write!(f, "dev"),
            LitEnv::Staging => write!(f, "staging"),
            LitEnv::Prod => write!(f, "prod"),
        }
    }
}

impl FromStr for LitEnv {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(LitEnv::Dev),
            "staging" => Ok(LitEnv::Staging),
            "prod" => Ok(LitEnv::Prod),
            _ => Err(conversion_err(format!("unable to create LitEnv from_str: {s}"), None)),
        }
    }
}

impl ValueEnum for LitEnv {
    fn value_variants<'a>() -> &'a [Self] {
        &[LitEnv::Dev, LitEnv::Staging, LitEnv::Prod]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            LitEnv::Dev => PossibleValue::new("dev"),
            LitEnv::Staging => PossibleValue::new("staging"),
            LitEnv::Prod => PossibleValue::new("prod"),
        })
    }
}
