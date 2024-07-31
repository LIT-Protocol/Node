#[allow(unused_imports)]
pub use lit_blockchain::util::{string_to_eth_address, string_to_u256};
pub use lit_core::utils::ipfs::{bytes_to_ipfs_cid, ipfs_cid_to_bytes};

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
