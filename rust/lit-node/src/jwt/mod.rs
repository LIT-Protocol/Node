use crate::error::Result;

use data_encoding::BASE64URL_NOPAD;
use lit_core::error::Unexpected;

use crate::models;

pub fn generate_unsigned_jwt(payload: &models::JwtPayload) -> Result<String> {
    let header = models::JwtHeader {
        alg: "BLS12-381".to_string(),
        typ: "JWT".to_string(),
    };

    let header_as_json = serde_json::to_string(&header).expect_or_err("Could not stringify")?;
    let header_as_base64 = BASE64URL_NOPAD.encode(header_as_json.as_bytes());

    let payload_as_json = serde_json::to_string(&payload).expect_or_err("Could not stringify")?;
    let payload_as_base64 = BASE64URL_NOPAD.encode(payload_as_json.as_bytes());

    let to_sign = format!("{}.{}", header_as_base64, payload_as_base64);

    Ok(to_sign)
}

pub fn generate_unsigned_jwt_v2(payload: &models::JwtPayloadV2) -> Result<String> {
    let header = models::JwtHeader {
        alg: "BLS12-381".to_string(),
        typ: "JWT".to_string(),
    };

    let header_as_json = serde_json::to_string(&header).expect_or_err("Could not stringify")?;
    let header_as_base64 = BASE64URL_NOPAD.encode(header_as_json.as_bytes());

    let payload_as_json = serde_json::to_string(&payload).expect_or_err("Could not stringify")?;
    let payload_as_base64 = BASE64URL_NOPAD.encode(payload_as_json.as_bytes());

    let to_sign = format!("{}.{}", header_as_base64, payload_as_base64);

    Ok(to_sign)
}

pub fn generate_unsigned_chain_data_jwt(
    payload: &models::JwtSignedChainDataPayload,
) -> Result<String> {
    let header = models::JwtHeader {
        alg: "BLS12-381".to_string(),
        typ: "JWT".to_string(),
    };

    let header_as_json = serde_json::to_string(&header).expect_or_err("Could not stringify")?;
    let header_as_base64 = BASE64URL_NOPAD.encode(header_as_json.as_bytes());

    let payload_as_json = serde_json::to_string(&payload).expect_or_err("Could not stringify")?;
    let payload_as_base64 = BASE64URL_NOPAD.encode(payload_as_json.as_bytes());

    let to_sign = format!("{}.{}", header_as_base64, payload_as_base64);

    Ok(to_sign)
}

// this happens on the client side because the client has to combine the sig shares
// pub fn encode_final_jwt(encoded_header_and_payload: String, signature: &SignatureShare) -> String {
//   let sig_as_base64 = base64::encode(signature.to_bytes());

//   let final_jwt = format!("{}.{}", encoded_header_and_payload, sig_as_base64);

//   return final_jwt;
// }
