use super::traits::*;

use elliptic_curve::group::GroupEncoding;

impl CompressedPointBytes for curve25519_dalek::RistrettoPoint {
    fn to_compressed(&self) -> Vec<u8> {
        self.compress().to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let arr = <[u8; 32]>::try_from(bytes).ok()?;
        Option::from(Self::from_bytes(&arr))
    }
}

impl CompressedPointBytes for curve25519_dalek::edwards::SubgroupPoint {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let arr = <[u8; 32]>::try_from(bytes).ok()?;
        Option::from(Self::from_bytes(&arr))
    }
}

impl CompressedPointBytes for ed25519_dalek::VerifyingKey {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let arr = <[u8; 32]>::try_from(bytes).ok()?;
        Self::from_bytes(&arr).ok()
    }
}

impl BeBytes for curve25519_dalek::Scalar {
    fn to_be_bytes(&self) -> Vec<u8> {
        let mut out = self.to_bytes().to_vec();
        out.reverse();
        out
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        let mut arr = [0u8; 32];
        for (i, byte) in bytes.iter().enumerate() {
            arr[31 - i] = *byte;
        }
        Option::from(Self::from_canonical_bytes(arr))
    }
}

impl LeBytes for curve25519_dalek::Scalar {
    fn to_le_bytes(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_le_bytes(bytes: &[u8]) -> Option<Self> {
        let arr = <[u8; 32]>::try_from(bytes).ok()?;
        Option::from(Self::from_canonical_bytes(arr))
    }
}

impl BeBytes for ed25519_dalek::SigningKey {
    fn to_be_bytes(&self) -> Vec<u8> {
        let mut out = self.to_bytes().to_vec();
        out.reverse();
        out
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 32 {
            return None;
        }
        let mut sk = ed25519_dalek::SecretKey::default();
        for (i, byte) in bytes.iter().enumerate() {
            sk[31 - i] = *byte;
        }
        Option::from(Self::from_bytes(&sk))
    }
}

impl LeBytes for ed25519_dalek::SigningKey {
    fn to_le_bytes(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_le_bytes(bytes: &[u8]) -> Option<Self> {
        let arr = ed25519_dalek::SecretKey::try_from(bytes).ok()?;
        Option::from(Self::from_bytes(&arr))
    }
}
