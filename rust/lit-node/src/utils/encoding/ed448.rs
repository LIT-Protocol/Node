use super::traits::*;

use ed448_goldilocks::{EdwardsPoint, Scalar};
use elliptic_curve::{group::GroupEncoding, PrimeField};

impl CompressedPointBytes for EdwardsPoint {
    fn to_compressed(&self) -> Vec<u8> {
        self.compress().to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let mut repr = <EdwardsPoint as GroupEncoding>::Repr::default();
        if bytes.len() != repr.len() {
            return None;
        }
        repr.copy_from_slice(bytes);
        Option::from(Self::from_bytes(&repr))
    }
}

impl BeBytes for Scalar {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.to_bytes_rfc_8032().to_vec()
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = <Scalar as PrimeField>::Repr::default();
        repr.copy_from_slice(bytes);
        Option::from(Self::from_repr(repr))
    }
}

impl LeBytes for Scalar {}
