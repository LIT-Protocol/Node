use crate::error::{conversion_err, validation_err, Result};
use libaes::Cipher;

use crate::utils::encoding;

pub async fn aes_decrypt(symmetric_key: Vec<u8>, ciphertext_with_iv: Vec<u8>) -> Result<String> {
    // Create a new 128-bit cipher
    let cipher_bytes = symmetric_key
        .try_into()
        .map_err(|e| conversion_err("Could not convert symmetric key to length 32", None))?;
    let cipher = Cipher::new_256(&cipher_bytes);

    if ciphertext_with_iv.len() < 16 {
        return Err(validation_err("Ciphertext is too short", None));
    }

    let iv = &ciphertext_with_iv[0..16];
    let ciphertext = &ciphertext_with_iv[16..];
    // Decryption
    let decrypted = cipher.cbc_decrypt(iv, ciphertext);

    Ok(encoding::bytes_to_hex(decrypted))
}
