use crate::auth::auth_material::JsonAuthSig;

use crate::error::{conversion_err_code, validation_err_code, Result, EC};
use std::str::FromStr;

pub fn validate_solana_auth_sig(auth_sig: &JsonAuthSig) -> Result<bool> {
    let pk_bytes = bs58::decode(&auth_sig.address)
        .into_vec()
        .map_err(|e| conversion_err_code(e, EC::NodeAuthSigAddressConversionError, None))?;
    let pk_slice = <[u8; 32]>::try_from(&pk_bytes[..])
        .map_err(|e| conversion_err_code(e, EC::NodeAuthSigSessionKeyConversionError, None))?;
    let public_key = ed25519_dalek::VerifyingKey::from_bytes(&pk_slice)
        .map_err(|e| conversion_err_code(e, EC::NodeAuthSigSessionKeyConversionError, None))?;
    let message = auth_sig.signed_message.as_bytes();
    let signature = ed25519_dalek::Signature::from_str(&auth_sig.sig)
        .map_err(|e| validation_err_code(e, EC::NodeInvalidED25519AuthSig, None))?;
    Ok(public_key.verify_strict(message, &signature).is_ok())
}
