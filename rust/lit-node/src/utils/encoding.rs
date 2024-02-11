pub use lit_blockchain::util::{string_to_eth_address, string_to_u256};
pub use lit_core::utils::binary::{
    big_endian_bytes_to_bincode_bytes, bincode_bytes_to_big_endian_bytes, bytes_to_hex,
    bytes_to_zero_padded_32, hex_to_bytes,
};
pub use lit_core::utils::ipfs::{bytes_to_ipfs_cid, ipfs_cid_to_bytes};

use elliptic_curve::{
    group::GroupEncoding,
    sec1::{EncodedPoint, FromEncodedPoint, ToEncodedPoint},
    PrimeField,
};

/// A trait for handling points in uncompressed form.
pub trait UncompressedPointBytes: Sized {
    /// Convert the point to uncompressed bytes.
    fn to_uncompressed(&self) -> Vec<u8>;

    /// Convert the point from uncompressed bytes.
    fn from_uncompressed(bytes: &[u8]) -> Option<Self>;
}

/// A trait for handling points in compressed form.
pub trait CompressedPointBytes: Sized {
    /// Convert the point to compressed bytes.
    fn to_compressed(&self) -> Vec<u8>;

    /// Convert the point from compressed bytes.
    fn from_compressed(bytes: &[u8]) -> Option<Self>;
}

pub trait UncompressedPointHex: Sized {
    /// Convert the point to uncompressed hex.
    fn to_uncompressed_hex(&self) -> String;

    /// Convert the point from uncompressed hex.
    fn from_uncompressed_hex(hex: &str) -> Option<Self>;
}

pub trait CompressedPointHex: Sized {
    /// Convert the point to compressed hex.
    fn to_compressed_hex(&self) -> String;

    /// Convert the point from compressed hex.
    fn from_compressed_hex(hex: &str) -> Option<Self>;
}

pub trait BeBytes: Sized {
    fn to_be_bytes(&self) -> Vec<u8>;

    fn from_be_bytes(bytes: &[u8]) -> Option<Self>;
}

pub trait LeBytes: BeBytes {
    fn to_le_bytes(&self) -> Vec<u8> {
        let mut out = self.to_be_bytes();
        out.reverse();
        out
    }

    fn from_le_bytes(bytes: &[u8]) -> Option<Self> {
        let mut bytes = bytes.to_vec();
        bytes.reverse();
        Self::from_be_bytes(&bytes)
    }
}

pub trait BeHex: Sized {
    fn to_be_hex(&self) -> String;

    fn from_be_hex(hex: &str) -> Option<Self>;
}

pub trait LeHex: Sized {
    fn to_le_hex(&self) -> String;

    fn from_le_hex(hex: &str) -> Option<Self>;
}

impl<B: BeBytes> BeHex for B {
    fn to_be_hex(&self) -> String {
        hex::encode(self.to_be_bytes())
    }

    fn from_be_hex(hex: &str) -> Option<Self> {
        let bytes = hex::decode(hex).ok()?;
        Self::from_be_bytes(&bytes)
    }
}

impl<B: LeBytes> LeHex for B {
    fn to_le_hex(&self) -> String {
        hex::encode(self.to_le_bytes())
    }

    fn from_le_hex(hex: &str) -> Option<Self> {
        let bytes = hex::decode(hex).ok()?;
        Self::from_le_bytes(&bytes)
    }
}

impl<P: UncompressedPointBytes> UncompressedPointHex for P {
    fn to_uncompressed_hex(&self) -> String {
        hex::encode(self.to_uncompressed())
    }

    fn from_uncompressed_hex(hex: &str) -> Option<Self> {
        let bytes = hex::decode(hex).ok()?;
        Self::from_uncompressed(&bytes)
    }
}

impl<P: CompressedPointBytes> CompressedPointHex for P {
    fn to_compressed_hex(&self) -> String {
        hex::encode(self.to_compressed())
    }

    fn from_compressed_hex(hex: &str) -> Option<Self> {
        let bytes = hex::decode(hex).ok()?;
        Self::from_compressed(&bytes)
    }
}

impl UncompressedPointBytes for k256::ProjectivePoint {
    fn to_uncompressed(&self) -> Vec<u8> {
        self.to_encoded_point(false).to_bytes().to_vec()
    }

    fn from_uncompressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<k256::Secp256k1>::from_bytes(bytes).ok()?;
        Option::from(Self::from_encoded_point(&pt))
    }
}

impl UncompressedPointBytes for k256::AffinePoint {
    fn to_uncompressed(&self) -> Vec<u8> {
        self.to_encoded_point(false).to_bytes().to_vec()
    }

    fn from_uncompressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<k256::Secp256k1>::from_bytes(bytes).ok()?;
        Option::from(Self::from_encoded_point(&pt))
    }
}

impl UncompressedPointBytes for k256::ecdsa::VerifyingKey {
    fn to_uncompressed(&self) -> Vec<u8> {
        self.to_encoded_point(false).to_bytes().to_vec()
    }

    fn from_uncompressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<k256::Secp256k1>::from_bytes(bytes).ok()?;
        Self::from_encoded_point(&pt).ok()
    }
}

impl UncompressedPointBytes for k256::schnorr::VerifyingKey {
    fn to_uncompressed(&self) -> Vec<u8> {
        self.as_affine().to_encoded_point(false).to_bytes().to_vec()
    }

    fn from_uncompressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<k256::Secp256k1>::from_bytes(bytes).ok()?;
        Self::from_bytes(pt.compress().as_bytes()).ok()
    }
}

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

impl CompressedPointBytes for curve25519_dalek::RistrettoPoint {
    fn to_compressed(&self) -> Vec<u8> {
        self.compress().to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let arr = <[u8; 32]>::try_from(bytes).ok()?;
        Option::from(Self::from_bytes(&arr))
    }
}

impl CompressedPointBytes for curve25519_dalek::EdwardsPoint {
    fn to_compressed(&self) -> Vec<u8> {
        self.compress().to_bytes().to_vec()
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

impl CompressedPointBytes for k256::ProjectivePoint {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_encoded_point(true).to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<k256::Secp256k1>::from_bytes(bytes).ok()?;
        Option::from(Self::from_encoded_point(&pt))
    }
}

impl CompressedPointBytes for k256::AffinePoint {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_encoded_point(true).to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<k256::Secp256k1>::from_bytes(bytes).ok()?;
        Option::from(Self::from_encoded_point(&pt))
    }
}

impl CompressedPointBytes for k256::ecdsa::VerifyingKey {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_encoded_point(true).to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<k256::Secp256k1>::from_bytes(bytes).ok()?;
        Self::from_encoded_point(&pt).ok()
    }
}

impl CompressedPointBytes for k256::schnorr::VerifyingKey {
    fn to_compressed(&self) -> Vec<u8> {
        self.as_affine().to_encoded_point(true).to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let pt = EncodedPoint::<k256::Secp256k1>::from_bytes(bytes).ok()?;
        Self::from_bytes(pt.compress().as_bytes()).ok()
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

impl BeBytes for k256::Scalar {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = k256::FieldBytes::default();
        repr.copy_from_slice(bytes);
        Option::from(Self::from_repr(repr))
    }
}

impl LeBytes for k256::Scalar {}

impl BeBytes for k256::NonZeroScalar {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = k256::FieldBytes::default();
        repr.copy_from_slice(bytes);
        Option::from(Self::from_repr(repr))
    }
}

impl LeBytes for k256::NonZeroScalar {}

impl BeBytes for k256::ecdsa::SigningKey {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.as_nonzero_scalar().to_be_bytes()
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = k256::FieldBytes::default();
        repr.copy_from_slice(bytes);
        Self::from_bytes(&repr).ok()
    }
}

impl LeBytes for k256::ecdsa::SigningKey {}

impl BeBytes for k256::schnorr::SigningKey {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.as_nonzero_scalar().to_be_bytes()
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        Self::from_bytes(bytes).ok()
    }
}

impl LeBytes for k256::schnorr::SigningKey {}

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
