use digest::{Digest, FixedOutput};
use elliptic_curve::{ops::Reduce, Curve, CurveArithmetic};
use k256::{
    ecdsa::{signature::PrehashSignature, Signature},
    FieldBytes, Scalar, Secp256k1,
};
use lit_core::utils::binary::bytes_to_hex;

pub fn scalar_hash(msg: &[u8]) -> <Secp256k1 as CurveArithmetic>::Scalar {
    let digest = <Signature as PrehashSignature>::Digest::new_with_prefix(msg);
    let m_bytes: FieldBytes = digest.finalize_fixed();
    <Scalar as Reduce<<Secp256k1 as Curve>::Uint>>::reduce_bytes(&m_bytes)
}

pub fn pubkey_to_checksum(sec1: Box<[u8]>) -> String {
    let public_key_hash = ethers::utils::keccak256(sec1);
    let public_key_hash = bytes_to_hex(&public_key_hash[12..]);
    let mut checksum = String::new();
    for c in public_key_hash.chars() {
        if let Some(v) = c.to_digit(16) {
            if v > 7 {
                checksum.push(c.to_ascii_uppercase());
            } else {
                checksum.push(c.to_ascii_lowercase());
            }
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::scalar_hash;
    use k256::SecretKey;

    #[test]
    fn should_create_sclar_from_hash() {
        let scalar = scalar_hash("this is a test".as_bytes());
        let _sk =
            SecretKey::from_bytes(&scalar.to_bytes()).expect("error while constructing privat key");
    }
}
