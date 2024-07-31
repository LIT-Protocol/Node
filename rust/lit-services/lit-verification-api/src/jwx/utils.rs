use elliptic_curve::{ops::Reduce, Curve};
use k256::Scalar as FE;
use k256::{FieldBytes, Secp256k1};
use lit_core::error::Result;
use rand::AsByteSliceMut;

pub fn hex_to_scalar(hex_val: &str) -> Result<FE> {
    let mut hex_val = hex_val.to_string();
    if hex_val.len() % 2 == 1 {
        // if length is odd, add a zero at the front
        hex_val.insert(0, '0');
    }

    let mut slice = hex::decode(hex_val).unwrap();
    let slice = slice.as_byte_slice_mut();
    let bytes = FieldBytes::from_mut_slice(slice);

    Ok(<FE as Reduce<<Secp256k1 as Curve>::Uint>>::reduce_bytes(bytes))
}
