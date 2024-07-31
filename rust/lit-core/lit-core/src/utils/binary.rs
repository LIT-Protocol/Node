use crate::error::{conversion_err, validation_err, Result};

pub fn remove_0x_prefix<S>(input: S) -> String
where
    S: AsRef<str>,
{
    let input = input.as_ref();
    if let Some(val) = input.strip_prefix("0x") {
        val.to_string()
    } else {
        input.to_string()
    }
}

pub fn hex_to_bytes<S>(hex_string: S) -> Result<Vec<u8>>
where
    S: AsRef<str>,
{
    let mut raw_str = remove_0x_prefix(hex_string);
    if raw_str.len() % 2 == 1 {
        // if length is odd, add a zero at the front
        raw_str.insert(0, '0');
    }
    hex::decode(raw_str)
        .map_err(|e| conversion_err(e, Some("failed to decode hex from str".into())))
}

pub fn bytes_to_hex<B>(vec: B) -> String
where
    B: AsRef<[u8]>,
{
    vec.as_ref().iter().map(|b| format!("{b:02x}")).collect::<String>()
}

pub fn bytes_to_decimal<B>(vec: B) -> String
where
    B: AsRef<[u8]>,
{
    vec.as_ref().iter().map(|b| format!("{b:0>2}")).collect::<String>()
}

pub fn bytes_to_zero_padded_32<B>(val: B) -> Result<[u8; 32]>
where
    B: AsRef<[u8]>,
{
    let val = val.as_ref();
    let len = val.len();
    if len > 32 {
        return Err(validation_err(format!("given val has a len ('{len}') > 32"), None));
    }

    let mut res = [0; 32];
    res[..len].copy_from_slice(&val[..len]);

    Ok(res)
}

pub fn bytes_zero_padded_32_from_str<S>(val: S) -> Result<[u8; 32]>
where
    S: AsRef<str>,
{
    bytes_to_zero_padded_32(val.as_ref().as_bytes())
}

pub fn bytes_32_are_zeros(val: &[u8; 32]) -> bool {
    val == &[0; 32]
}

// bincode is little endian encoding, see
// https://docs.rs/bincode/1.3.2/bincode/config/trait.Options.html#options
// but SecretKey.reveal() gives big endian hex
// and all other bls implementations specify bigendian.
// Also see
// https://safenetforum.org/t/simple-web-based-tool-for-bls-keys/32339/37
// so to deserialize a big endian bytes using bincode
// we must convert to little endian bytes
pub fn big_endian_bytes_to_bincode_bytes(mut v: Vec<u8>) -> Vec<u8> {
    v.reverse();
    v
}

pub fn bincode_bytes_to_big_endian_bytes(mut v: Vec<u8>) -> Vec<u8> {
    v.reverse();
    v
}
