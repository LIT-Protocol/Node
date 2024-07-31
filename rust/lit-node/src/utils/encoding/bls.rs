use super::traits::*;

use elliptic_curve::PrimeField;

impl CompressedPointBytes for blsful::inner_types::G1Projective {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_compressed().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 48 {
            return None;
        }
        let bytes: [u8; 48] = bytes.try_into().ok()?;
        Option::from(Self::from_compressed(&bytes))
    }
}

impl BeBytes for bulletproofs::blstrs_plus::Scalar {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = <bulletproofs::blstrs_plus::Scalar as PrimeField>::Repr::default();
        repr.copy_from_slice(bytes);
        Option::from(Self::from_repr(repr))
    }
}

impl LeBytes for bulletproofs::blstrs_plus::Scalar {}
