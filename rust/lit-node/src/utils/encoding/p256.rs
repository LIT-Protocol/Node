use super::traits::*;

use elliptic_curve::{
    sec1::{EncodedPoint, FromEncodedPoint, ToEncodedPoint},
    PrimeField,
};

impl UncompressedPointBytes for p256::ProjectivePoint {
    fn to_uncompressed(&self) -> Vec<u8> {
        self.to_encoded_point(false).to_bytes().to_vec()
    }

    fn from_uncompressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<p256::NistP256>::from_bytes(bytes).ok()?;
        Option::from(Self::from_encoded_point(&pt))
    }
}

impl UncompressedPointBytes for p256::AffinePoint {
    fn to_uncompressed(&self) -> Vec<u8> {
        self.to_encoded_point(false).to_bytes().to_vec()
    }

    fn from_uncompressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<p256::NistP256>::from_bytes(bytes).ok()?;
        Option::from(Self::from_encoded_point(&pt))
    }
}

impl UncompressedPointBytes for p256::ecdsa::VerifyingKey {
    fn to_uncompressed(&self) -> Vec<u8> {
        self.to_encoded_point(false).to_bytes().to_vec()
    }

    fn from_uncompressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<p256::NistP256>::from_bytes(bytes).ok()?;
        Self::from_encoded_point(&pt).ok()
    }
}

impl CompressedPointBytes for p256::ProjectivePoint {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_encoded_point(true).to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<p256::NistP256>::from_bytes(bytes).ok()?;
        Option::from(Self::from_encoded_point(&pt))
    }
}

impl CompressedPointBytes for p256::AffinePoint {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_encoded_point(true).to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<p256::NistP256>::from_bytes(bytes).ok()?;
        Option::from(Self::from_encoded_point(&pt))
    }
}

impl CompressedPointBytes for p256::ecdsa::VerifyingKey {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_encoded_point(true).to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<p256::NistP256>::from_bytes(bytes).ok()?;
        Self::from_encoded_point(&pt).ok()
    }
}

impl BeBytes for p256::Scalar {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = p256::FieldBytes::default();
        repr.copy_from_slice(bytes);
        Option::from(Self::from_repr(repr))
    }
}

impl LeBytes for p256::Scalar {}

impl BeBytes for p256::NonZeroScalar {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = p256::FieldBytes::default();
        repr.copy_from_slice(bytes);
        Option::from(Self::from_repr(repr))
    }
}

impl LeBytes for p256::NonZeroScalar {}

impl BeBytes for p256::ecdsa::SigningKey {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.as_nonzero_scalar().to_be_bytes()
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = p256::FieldBytes::default();
        repr.copy_from_slice(bytes);
        Self::from_bytes(&repr).ok()
    }
}

impl LeBytes for p256::ecdsa::SigningKey {}
