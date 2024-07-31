use serde::{Deserialize, Serialize};

use crate::error::{unexpected_err, Result};
use std::array::IntoIter;
use std::fmt::{self, Display};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurveType {
    BLS = 1, // Could be further separated as G1 and G2.
    K256 = 2,
    Ed25519 = 3,        // Ed25519 with the SHA-512 hash function
    Ed448 = 4,          // Ed448 with the SHAKE-256 hash function
    Ristretto25519 = 5, // Ristretto25519 with the SHA-512 hash function
    P256 = 6, // NistP256 with SHA-256 hash function  <<< this is a duplicate of the Cait Sith Key.
    P384 = 7, // NistP384 with SHA-384 hash function
    RedJubjub = 8, // RedJubjub
}

impl CurveType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CurveType::BLS => "BLS",
            CurveType::K256 => "K256",
            CurveType::P256 => "P256",
            CurveType::P384 => "P384",
            CurveType::Ed25519 => "Ed25519",
            CurveType::Ed448 => "Ed448",
            CurveType::Ristretto25519 => "Ristretto25519",
            CurveType::RedJubjub => "RedJubjub",
        }
    }

    pub fn into_iter() -> IntoIter<CurveType, 2> {
        use CurveType::*;
        static KEYTYPE: [CurveType; 2] = [BLS, K256];
        KEYTYPE.into_iter()
    }

    pub fn scalar_len(&self) -> usize {
        match self {
            Self::BLS => 32,
            Self::K256 => 32,
            Self::Ed25519 => 32,
            Self::Ed448 => 57,
            Self::Ristretto25519 => 32,
            Self::P256 => 32,
            Self::P384 => 48,
            Self::RedJubjub => 32,
        }
    }

    pub fn compressed_point_len(&self) -> usize {
        match self {
            Self::BLS => 48,
            Self::K256 => 33,
            Self::Ed25519 => 32,
            Self::Ed448 => 57,
            Self::Ristretto25519 => 32,
            Self::P256 => 33,
            Self::P384 => 49,
            Self::RedJubjub => 32,
        }
    }

    fn invalid() -> Result<Self> {
        Err(unexpected_err(
            Error::new(ErrorKind::Other, "Not a valid key type"),
            None,
        ))
    }
}

impl Display for CurveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for CurveType {
    type Err = crate::error::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "BLS" => Ok(CurveType::BLS),
            "ECDSA_CAIT_SITH" => Ok(CurveType::K256),
            "K256" => Ok(CurveType::K256),
            "ED25519" => Ok(CurveType::Ed25519),
            "ED448" => Ok(CurveType::Ed448),
            "RISTRETTO25519" => Ok(CurveType::Ristretto25519),
            "P256" => Ok(CurveType::P256),
            "P384" => Ok(CurveType::P384),
            "REDJUBJUB" => Ok(CurveType::RedJubjub),
            _ => CurveType::invalid(),
        }
    }
}

impl From<CurveType> for ethers::types::U256 {
    fn from(curve_type: CurveType) -> ethers::types::U256 {
        let curve_type = curve_type as u8;
        ethers::types::U256::from(curve_type as u32)
    }
}

impl TryFrom<ethers::types::U256> for CurveType {
    type Error = crate::error::Error;
    fn try_from(curve_type: ethers::types::U256) -> Result<Self> {
        let curve_type = curve_type.as_u32();
        let curve_type = TryInto::<u8>::try_into(curve_type);
        match curve_type {
            Ok(1) => Ok(CurveType::BLS),
            Ok(2) => Ok(CurveType::K256),
            Ok(3) => Ok(CurveType::Ed25519),
            Ok(4) => Ok(CurveType::Ed448),
            Ok(5) => Ok(CurveType::Ristretto25519),
            Ok(6) => Ok(CurveType::P256),
            Ok(7) => Ok(CurveType::P384),
            Ok(8) => Ok(CurveType::RedJubjub),
            _ => CurveType::invalid(),
        }
    }
}

impl TryFrom<u8> for CurveType {
    type Error = crate::error::Error;
    fn try_from(byte: u8) -> Result<Self> {
        match byte {
            1 => Ok(CurveType::BLS),
            2 => Ok(CurveType::K256),
            3 => Ok(CurveType::Ed25519),
            4 => Ok(CurveType::Ed448),
            5 => Ok(CurveType::Ristretto25519),
            6 => Ok(CurveType::P256),
            7 => Ok(CurveType::P384),
            8 => Ok(CurveType::RedJubjub),
            _ => CurveType::invalid(),
        }
    }
}

impl From<CurveType> for u8 {
    fn from(curve_type: CurveType) -> Self {
        match curve_type {
            CurveType::BLS => 1,
            CurveType::K256 => 2,
            CurveType::Ed25519 => 3,
            CurveType::Ed448 => 4,
            CurveType::Ristretto25519 => 5,
            CurveType::P256 => 6,
            CurveType::P384 => 7,
            CurveType::RedJubjub => 8,
        }
    }
}
