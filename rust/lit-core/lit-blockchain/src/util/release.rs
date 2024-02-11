use lit_core::utils::binary::{bytes_to_hex, bytes_to_zero_padded_32, hex_to_bytes};

use crate::error::{validation_err, Result};

pub const RELEASE_ID_STR_LEN: usize = 48;
pub const BUILD_ID_STR_LEN: usize = 8;
pub const SUBNET_ID_STR_LEN: usize = 40;

pub fn release_id_bin_from_string<S>(val: S) -> Result<[u8; 32]>
where
    S: AsRef<str>,
{
    let bytes = hex_to_bytes(val.as_ref())?;
    bytes_to_zero_padded_32(&bytes[..])
}

pub fn validate_release_id_len<S>(val: S) -> Result<()>
where
    S: AsRef<str>,
{
    if val.as_ref().len() != RELEASE_ID_STR_LEN {
        return Err(validation_err(
            format!("release_id is invalid (lem: {})", val.as_ref().len()),
            None,
        ));
    }

    Ok(())
}

pub fn build_id_from_release_id<S>(val: S) -> Result<String>
where
    S: AsRef<str>,
{
    let val = val.as_ref();

    validate_release_id_len(val)?;

    Ok(val[..BUILD_ID_STR_LEN].to_string())
}

pub fn subnet_id_from_release_id<S>(val: S) -> Result<String>
where
    S: AsRef<str>,
{
    let val = val.as_ref();
    validate_release_id_len(val)?;

    Ok(val[BUILD_ID_STR_LEN..(SUBNET_ID_STR_LEN + BUILD_ID_STR_LEN)].to_string())
}

pub fn release_id_to_string(val: &[u8; 32]) -> String {
    bytes_to_hex(&val[..(RELEASE_ID_STR_LEN / 2)])
}
