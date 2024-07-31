use ethers::prelude::*;
use lit_core::utils::binary::hex_to_bytes;

use crate::error::{conversion_err, Result};

pub mod ether;
pub mod release;

pub fn string_to_eth_address<S>(address: S) -> Result<Address>
where
    S: AsRef<str>,
{
    let bytes = hex_to_bytes(address.as_ref())?;
    if bytes.len() != Address::len_bytes() {
        return Err(conversion_err(
            format!(
                "eth address has incorrect length, expected {}, got {}",
                Address::len_bytes(),
                bytes.len()
            ),
            None,
        ));
    }
    Ok(Address::from_slice(bytes.as_slice()))
}

pub fn string_to_u256<S>(input_str: S) -> Result<U256>
where
    S: AsRef<str>,
{
    let input_str = input_str.as_ref();
    if let Some(stripped) = input_str.strip_prefix("0x") {
        U256::from_str_radix(stripped, 16).map_err(|e| conversion_err(e, None))
    } else {
        Ok(U256::from_dec_str(input_str).map_err(|e| conversion_err(e, None))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_eth_address() {
        assert_eq!(
            string_to_eth_address("0123456789abcdef0123456789abcdef01234567").unwrap().to_string(),
            "0x0123…4567"
        );
        assert_eq!(
            string_to_eth_address("0000").unwrap_err().to_string(),
            "conversion error: eth address has incorrect length, expected 20, got 2"
        );
        assert_eq!(
            string_to_eth_address("not-hex").unwrap_err().to_string(),
            "conversion error: failed to decode hex from str: Invalid character 'n' at position 1"
        );
    }
}
