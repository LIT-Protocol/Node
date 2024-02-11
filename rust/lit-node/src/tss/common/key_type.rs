use serde::{Deserialize, Serialize};

use crate::error::{unexpected_err, Result};
use std::array::IntoIter;
use std::fmt::{self, Display};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DkgType {
    Standard = 1,
    RecoveryParty = 2,
}

impl DkgType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DkgType::Standard => "STANDARD",
            DkgType::RecoveryParty => "RECOVERY",
        }
    }

    pub fn is_to_be_backed_up(&self) -> bool {
        match self {
            DkgType::Standard => true,
            DkgType::RecoveryParty => false,
        }
    }
}

impl Display for DkgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    BLS = 1, // Could be further separated as G1 and G2.
    EcdsaCaitSith = 2,
}

impl KeyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            KeyType::BLS => "BLS",
            KeyType::EcdsaCaitSith => "ECDSA_CAIT_SITH",
        }
    }

    pub fn into_iter() -> IntoIter<KeyType, 2> {
        use KeyType::*;
        static KEYTYPE: [KeyType; 2] = [BLS, EcdsaCaitSith];
        KEYTYPE.into_iter()
    }

    fn invalid() -> Result<Self> {
        Err(unexpected_err(
            Error::new(ErrorKind::Other, "Not a valid key type"),
            None,
        ))
    }
}

impl Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for KeyType {
    type Err = crate::error::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "BLS" => Ok(KeyType::BLS),
            "ECDSA_CAIT_SITH" => Ok(KeyType::EcdsaCaitSith),
            _ => KeyType::invalid(),
        }
    }
}

impl From<KeyType> for ethers::types::U256 {
    fn from(key_type: KeyType) -> ethers::types::U256 {
        let key_type = key_type as u8;
        ethers::types::U256::from(key_type as u32)
    }
}

impl TryFrom<ethers::types::U256> for KeyType {
    type Error = crate::error::Error;
    fn try_from(key_type: ethers::types::U256) -> Result<Self> {
        let key_type = key_type.as_u32();
        let key_type = TryInto::<u8>::try_into(key_type);
        match key_type {
            Ok(1) => Ok(KeyType::BLS),
            Ok(2) => Ok(KeyType::EcdsaCaitSith),
            _ => KeyType::invalid(),
        }
    }
}

impl TryFrom<u8> for KeyType {
    type Error = crate::error::Error;
    fn try_from(byte: u8) -> Result<Self> {
        match byte {
            1 => Ok(KeyType::BLS),
            2 => Ok(KeyType::EcdsaCaitSith),
            _ => KeyType::invalid(),
        }
    }
}
