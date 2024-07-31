use crate::{ecdsa::SigningPair, jwx::jws::Json};
use chrono::{DateTime, NaiveDateTime, Utc};
use k256::ecdsa::signature::Verifier;
use k256::ecdsa::Signature;
use lit_core::error::{self, Unexpected};
use lit_os_core::error::{conversion_err, parser_err, validation_err};
use log::trace;
use serde_json::Value;
use sha3::Digest;

use std::str::{self};

use super::models::{Jwt, JwtHeader, JwtPayload};

const SECP_ALG_VAL: &str = "secp256k1";

pub fn verify_token_from_str(token: String, key_shares: &SigningPair) -> error::Result<Jwt> {
    let parts: Vec<&str> = token.splitn(3, '.').collect();

    if parts.len() != 3 {
        return Err(validation_err("Invalid JWT parts length".to_string(), None));
    }

    let header_b64 = parts.first().expect_or_err("Could not parse header from token")?;
    let payload_b64 = parts.get(1).expect_or_err("Could not parse payload from token")?;
    let signature_b64 = parts.get(2).expect_or_err("Could not parse signature from token")?;

    let header = base64::decode_config(header_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| parser_err(e, Some("Unable to decode".into())))?;

    let header = &str::from_utf8(&header)
        .map_err(|e| parser_err(e, Some("Unable to parse header string".into())))?;

    let header: JwtHeader = serde_json::from_str(header)
        .map_err(|e| conversion_err(e, Some("Unable to convert to value".into())))?;

    let signature = base64::decode_config(signature_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| parser_err(e, Some("Unable to decode".into())))?;

    let payload = base64::decode_config(payload_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| parser_err(e, Some("Unable to decode".into())))?;

    let payload = &str::from_utf8(&payload)
        .map_err(|e| parser_err(e, Some("Unable to parse payload string".into())))?;

    let payload: JwtPayload = serde_json::from_str(payload)
        .map_err(|e| conversion_err(e, Some("Unable to convert to value".into())))?;

    if header.alg != SECP_ALG_VAL {
        return Err(validation_err(
            format!("Invalid JWT algorithm. Only {} supported", SECP_ALG_VAL),
            None,
        ));
    }

    let now = Utc::now();
    let jwt_exp = payload.exp;

    let jwt_exp = NaiveDateTime::from_timestamp_millis(jwt_exp)
        .expect_or_err("failed to create timestamp opt")?;

    if now.naive_utc() > jwt_exp {
        trace!("JWT expired expiration time: {} and current time: {}", jwt_exp, now);
        return Err(validation_err("OTP token expired".to_string(), None));
    }

    let message = payload.extra_data.clone(); // clone the value so we can return the converted struct later
    trace!("token message: {}", message);
    let signature: Value = serde_json::from_slice(signature.as_slice())
        .map_err(|e| conversion_err(e, Some("Error while decoding signature JSON".to_string())))?;

    trace!("signature value: {:?}", signature);

    let signature = Signature::from_value(signature)
        .map_err(|e| conversion_err(e, Some("Could not not convert signature".into())))?;

    let verifying_key = key_shares.signing_key.verifying_key();

    let mut hasher = sha3::Sha3_256::new();
    hasher.update(message.as_bytes());
    let hash = hasher.finalize();
    trace!("hashed message: {:?}", hash.as_slice());
    let verify_result = verifying_key.verify(&hash, &signature);
    match verify_result {
        Ok(_) => {
            let parts: Vec<&str> = message.splitn(2, '|').collect();
            let ts: &str = parts.get(1).expect_or_err("invalid message structure in token")?;

            let jwt_exp = DateTime::parse_from_rfc3339(ts).map_err(|e| {
                conversion_err(e, Some("Error while converting message date time".into()))
            })?;

            if now < jwt_exp {
                trace!("JWT expired expiration time: {} and current time: {}", jwt_exp, now);
                return Err(validation_err("OTP token expired".to_string(), None));
            }

            Ok(Jwt { header, payload, signature: None })
        }
        Err(e) => {
            Err(validation_err("Error while validating signature".to_string(), Some(e.to_string())))
        }
    }
}
