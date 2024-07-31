use super::traits::*;
use elliptic_curve::{group::GroupEncoding, PrimeField};

impl CompressedPointBytes for jubjub::SubgroupPoint {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let mut repr = <jubjub::SubgroupPoint as GroupEncoding>::Repr::default();
        if bytes.len() != repr.len() {
            return None;
        }
        repr.copy_from_slice(bytes);
        Option::from(Self::from_bytes(&repr))
    }
}

impl BeBytes for jubjub::Scalar {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = <jubjub::Scalar as PrimeField>::Repr::default();
        repr.copy_from_slice(bytes);
        Option::from(Self::from_repr(repr))
    }
}

impl LeBytes for jubjub::Scalar {}
