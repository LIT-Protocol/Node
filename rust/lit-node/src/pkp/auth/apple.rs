use crate::error::{
    self, conversion_err, parser_err, unexpected_err, validation_err, Result, Unexpected,
};
use crate::models;
use crate::pkp::auth::constants::APPLE_JWT_AUTH_METHOD_TYPE_ID;
use crate::utils::encoding;
use chrono::{Duration, Utc};
use reqwest;
use rsa::PublicKey;
use rsa::{BigUint, RsaPublicKey};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str;

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use super::AuthMethodVerifier;

const APPLE_KEYS_FILE_PATH: &str = "./oauth/apple_keys.json";

pub struct AppleJwtAuthMethodVerifier {}

#[async_trait::async_trait]
impl AuthMethodVerifier for AppleJwtAuthMethodVerifier {
    async fn verify(&self, access_token: &str) -> error::Result<models::AuthMethodResponse> {
        debug!("Getting apple user id and app id from jwt");
        get_apple_auth_from_jwt(access_token).await
    }
}

pub async fn save_apple_keys() -> Result<Value> {
    trace!("Downloading Apple keys...");
    let url = "https://appleid.apple.com/auth/keys";
    let resp = reqwest::get(url)
        .await
        .map_err(|e| unexpected_err(e, Some("Unable to send request to apple".into())))?;
    let text = resp
        .text()
        .await
        .map_err(|e| unexpected_err(e, Some("Unable to get text from apple response".into())))?;

    let json: Value = serde_json::from_str(text.as_str())
        .map_err(|e| conversion_err(e, Some("Unable to convert apple response to json".into())))?;
    // get the keys out
    let mut top_level_map = json
        .as_object()
        .expect_or_err("Unable to convert apple response to object")
        .map_err(|e| conversion_err(e, None))?
        .clone();

    let now = SystemTime::now();
    let now_unix_time = now
        .duration_since(UNIX_EPOCH)
        .expect_or_err("Time went backwards")
        .map_err(|e| unexpected_err(e, None))?;

    // add latest timestamp
    top_level_map.insert(
        "last_retrieved_at".to_string(),
        Value::Number(now_unix_time.as_secs().into()),
    );

    // get the parent path so we can create the dir if it doesn't exist
    let path = Path::new(APPLE_KEYS_FILE_PATH);
    let parent_path = match path.parent() {
        Some(parent_path) => parent_path,
        None => {
            return Err(unexpected_err(
                "Unable to get parent path of apple keys file",
                None,
            ));
        }
    };

    // create the dir if it doesn't exist
    fs::create_dir_all(parent_path).map_err(|e| {
        unexpected_err(
            e,
            Some("Could not create directory for apple key files".into()),
        )
    })?;

    // write to the keys file
    let mut file = File::create(APPLE_KEYS_FILE_PATH)
        .map_err(|e| unexpected_err(e, Some("Unable to create apple keys file".into())))?;
    file.write_all(
        serde_json::to_string(&top_level_map)
            .map_err(|e| conversion_err(e, Some("Unable to convert apple keys to json".into())))?
            .as_bytes(),
    )
    .map_err(|e| unexpected_err(e, Some("Unable to write to apple keys file".into())))?;

    Ok(top_level_map["keys"].clone())
}

pub async fn get_apple_keys() -> Result<Value> {
    // check if oauth key file is present
    let contents = match fs::read_to_string(APPLE_KEYS_FILE_PATH) {
        Ok(contents) => contents,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                // get the key and save it
                error!("Apple keys file not found, downloading...");
                return save_apple_keys().await;
            } else {
                return Err(unexpected_err(
                    e,
                    Some("Unable to read apple keys file".into()),
                ));
            }
        }
    };

    // parse the json
    let json: Value = serde_json::from_str(&contents)
        .map_err(|e| conversion_err(e, Some("Unable to convert apple keys file to json".into())))?;
    let keys = json
        .as_object()
        .expect_or_err("Unable to convert apple response to object")
        .map_err(|e| conversion_err(e, None))?
        .clone();

    // parse last_retrieved_at and check if it's older than 1 hour
    let last_retrieved_at = match keys.get("last_retrieved_at") {
        Some(last_retrieved_at) => last_retrieved_at
            .as_i64()
            .expect_or_err("Could not convert last_retrieved_at to i64")
            .map_err(|e| conversion_err(e, None))?,
        None => {
            // get the keys again
            return save_apple_keys().await;
        }
    };
    let last_retrieved_at = chrono::DateTime::from_timestamp(last_retrieved_at, 0)
        .expect_or_err("failed to create timestamp opt")
        .map_err(|e| unexpected_err(e, None))?;

    let now = Utc::now();
    let one_hour_ago = now - Duration::hours(1);
    if last_retrieved_at < one_hour_ago {
        // get the keys again
        return save_apple_keys().await;
    }

    Ok(keys["keys"].clone())
}

pub async fn parse_and_verify_apple_jwt(
    token: &str,
    apple_keys: &Vec<Value>,
) -> error::Result<models::AuthMethodResponse> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(validation_err("Invalid JWT parts length".to_string(), None));
    }
    let header_b64 = parts[0];
    let payload_b64 = parts[1];
    let signature_b64 = parts[2];

    // decode the header
    // trace!("Decoding JWT header: {}", header_b64);
    let header = data_encoding::BASE64URL_NOPAD
        .decode(header_b64.as_bytes())
        .map_err(|e| parser_err(e, Some("Unable to decode".into())))?;
    let header: Value = serde_json::from_str(
        str::from_utf8(&header)
            .map_err(|e| parser_err(e, Some("Unable to parse string".into())))?,
    )
    .map_err(|e| conversion_err(e, Some("Unable to convert to value".into())))?;
    let header = header
        .as_object()
        .expect_or_err("Invalid JWT header")
        .map_err(|e| validation_err(e, Some("Unable to convert to object".into())))?;

    // decode the payload
    // trace!("Decoding JWT payload: {}", payload_b64);
    let payload = data_encoding::BASE64URL_NOPAD
        .decode(payload_b64.as_bytes())
        .map_err(|e| parser_err(e, Some("Unable to decode".into())))?;
    let payload: Value = serde_json::from_str(
        str::from_utf8(&payload)
            .map_err(|e| parser_err(e, Some("Unable to parse string".into())))?,
    )
    .map_err(|e| conversion_err(e, Some("Unable to convert to value".into())))?;
    let payload = payload
        .as_object()
        .expect_or_err("Invalid JWT payload")
        .map_err(|e| validation_err(e, Some("Unable to convert to object".into())))?;

    // decode the signature
    // trace!("Decoding JWT signature: {}", signature_b64);
    let signature = data_encoding::BASE64URL_NOPAD
        .decode(signature_b64.as_bytes())
        .map_err(|e| parser_err(e, Some("Unable to decode".into())))?;

    if header["alg"] != "RS256" {
        return Err(validation_err(
            format!(
                "Invalid JWT algorithm.  Only RS256 supported.  Alg returned is {}",
                header["alg"]
            ),
            None,
        ));
    }

    trace!("Decoded apple JWT...");

    // bail if it's expired
    let now = Utc::now();
    let jwt_exp = payload["exp"]
        .as_i64()
        .expect_or_err("Apple JWT missing exp")
        .map_err(|e| validation_err(e, Some("Unable to convert to i64".into())))?;
    let jwt_exp = chrono::DateTime::from_timestamp(jwt_exp, 0)
        .expect_or_err("failed to create timestamp opt")
        .map_err(|e| unexpected_err(e, None))?;
    if now > jwt_exp {
        trace!(
            "JWT expired.  Expiration time: {} and current time: {}",
            jwt_exp,
            now
        );
        return Err(validation_err("Apple JWT expired".to_string(), None));
    }

    let mut hasher = Sha256::new();
    hasher.update(header_b64.as_bytes());
    hasher.update(b".");
    hasher.update(payload_b64.as_bytes());
    let signed_message = hasher.finalize();

    trace!(
        "Hashed apple JWT: {:?}",
        encoding::bytes_to_hex(signed_message)
    );

    let public_key_id = header["kid"]
        .as_str()
        .expect_or_err("Invalid JWT. Missing kid")
        .map_err(|e| validation_err(e, Some("Unable to convert to string".into())))?;
    trace!("Need to find public key with id: {}", public_key_id);

    for key in apple_keys {
        let parsed_apple_keys = match key.as_object() {
            Some(parsed_apple_keys) => parsed_apple_keys,
            None => continue,
        };
        trace!("checking against key: {:?}", parsed_apple_keys);

        if parsed_apple_keys["kid"] != public_key_id {
            continue;
        }

        trace!("kid matches, checking signature...");

        let public_key = RsaPublicKey::new(
            BigUint::from_bytes_be(
                &data_encoding::BASE64URL_NOPAD
                    .decode(
                        parsed_apple_keys["n"]
                            .as_str()
                            .expect_or_err("Invalid JWT. Missing n")
                            .map_err(|e| {
                                validation_err(e, Some("Unable to decode apple keys".into()))
                            })?
                            .as_bytes(),
                    )
                    .map_err(|e| parser_err(e, Some("Unable to decode apple keys".into())))?,
            ),
            BigUint::from_bytes_be(
                &data_encoding::BASE64URL_NOPAD
                    .decode(
                        parsed_apple_keys["e"]
                            .as_str()
                            .expect_or_err("Invalid JWT. Missing e")
                            .map_err(|e| {
                                validation_err(e, Some("Unable to decode apple keys".into()))
                            })?
                            .as_bytes(),
                    )
                    .map_err(|e| parser_err(e, Some("Unable to decode apple keys".into())))?,
            ),
        )
        .map_err(|e| unexpected_err(e, Some("Unable to create public key".into())))?;

        trace!("parsed pubkey");

        let verify_result = public_key.verify(
            rsa::padding::PaddingScheme::PKCS1v15Sign {
                hash: Some(rsa::Hash::SHA2_256),
            },
            &signed_message,
            &signature,
        );
        match verify_result {
            Ok(_) => {
                return Ok(models::AuthMethodResponse {
                    user_id: payload["sub"]
                        .as_str()
                        .expect_or_err("Invalid JWT. Missing sub")
                        .map_err(|e| validation_err(e, Some("Unable to convert to string".into())))?
                        .to_string(),
                    app_id: payload["aud"]
                        .as_str()
                        .expect_or_err("Invalid JWT. Missing aud")
                        .map_err(|e| validation_err(e, Some("Unable to convert to string".into())))?
                        .to_string(),
                    auth_method_type: APPLE_JWT_AUTH_METHOD_TYPE_ID,
                    last_retrieved_at: SystemTime::now(),
                    expiration: jwt_exp.timestamp(),
                    used_for_sign_session_key_request: false,
                });
            }
            Err(e) => {
                trace!("verify failed: {:?}", e);
            }
        }

        // es256 algorithm implementation.  apple docs say they use this but their actual JWTs are rsa.  keeping this for posterity in case we do indeed need to implement ES256 someday
        // let jwk_key = JwkEcKey::from_str(
        //     &serde_json::to_string(&parsed_apple_keys).map_err(|e| unexpected_err(e, None))?,
        // )
        // .map_err(|e| unexpected_err(e, None))?;
        // let public_key = PublicKey::from_jwk(&jwk_key).map_err(|e| unexpected_err(e, None))?;
        // let public_key = VerifyingKey::from(&public_key);
        // let sig = Signature::from_slice(&signature).map_err(|e| unexpected_err(e, None))?;

        // trace!("recovered key");

        // let verified = public_key.verify(&signed_message, &sig);

        // if let Err(e) = verified {
        //     trace!("Verification failed for apple with error: {}", e);
        // } else {
        //     return Ok(models::AuthMethodResponse {
        //         user_id: payload["sub"].as_str().unwrap().to_string(),
        //         app_id: payload["aud"].as_str().unwrap().to_string(),
        //         auth_method_type: APPLE_JWT_AUTH_METHOD_TYPE_ID,
        //         last_retrieved_at: SystemTime::now(),
        //         used_for_sign_session_key_request: false,
        //     });
        // }
    }

    Err(validation_err("Invalid JWT".to_string(), None))
}

pub async fn get_apple_auth_from_jwt(
    apple_access_token: &str,
) -> error::Result<models::AuthMethodResponse> {
    let apple_keys = get_apple_keys()
        .await
        .map_err(|e| unexpected_err(e, Some("Unable to get apple keys".into())))?;

    let apple_keys = apple_keys
        .as_array()
        .expect_or_err("apple_keys is not an array")
        .map_err(|e| validation_err(e, Some("Unable to parse apple keys".into())))?;

    trace!("Got apple keys: {:?}", apple_keys);

    parse_and_verify_apple_jwt(apple_access_token, apple_keys).await
}
