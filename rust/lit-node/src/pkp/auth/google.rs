use crate::error::{
    self, conversion_err, parser_err, unexpected_err, validation_err, Result, Unexpected,
};
use crate::models;
use crate::pkp::auth::constants::{GOOGLE_AUTH_METHOD_TYPE_ID, GOOGLE_JWT_AUTH_METHOD_TYPE_ID};
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

const GOOGLE_KEYS_FILE_PATH: &str = "./oauth/google_keys.json";

pub struct GoogleAuthMethodVerifier {}

#[async_trait::async_trait]
impl AuthMethodVerifier for GoogleAuthMethodVerifier {
    async fn verify(&self, access_token: &str) -> error::Result<models::AuthMethodResponse> {
        debug!("Getting google user id and app id from access token");
        get_google_auth_from_access_token(access_token).await
    }
}

pub struct GoogleJwtAuthMethodVerifier {}

#[async_trait::async_trait]
impl AuthMethodVerifier for GoogleJwtAuthMethodVerifier {
    async fn verify(&self, access_token: &str) -> error::Result<models::AuthMethodResponse> {
        debug!("Getting google user id and app id from jwt");
        get_google_auth_from_jwt(access_token).await
    }
}

pub async fn save_google_keys() -> Result<Value> {
    trace!("Downloading Google keys...");
    let url = "https://www.googleapis.com/oauth2/v3/certs";
    let resp = reqwest::get(url)
        .await
        .map_err(|e| unexpected_err(e, Some("Unable to send request to google".into())))?;
    let text = resp
        .text()
        .await
        .map_err(|e| unexpected_err(e, Some("Unable to get text from google response".into())))?;

    let json: Value = serde_json::from_str(text.as_str())
        .map_err(|e| conversion_err(e, Some("Unable to convert google response to json".into())))?;
    // get the keys out
    let mut top_level_map = json
        .as_object()
        .expect_or_err("Unable to convert google response to object")
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
    let path = Path::new(GOOGLE_KEYS_FILE_PATH);
    let parent_path = match path.parent() {
        Some(parent_path) => parent_path,
        None => {
            return Err(unexpected_err(
                "Unable to get parent path of google keys file",
                None,
            ));
        }
    };

    // create the dir if it doesn't exist
    fs::create_dir_all(parent_path).map_err(|e| {
        unexpected_err(
            e,
            Some("Could not create directory for google key files".into()),
        )
    })?;

    // write to the keys file
    let mut file = File::create(GOOGLE_KEYS_FILE_PATH)
        .map_err(|e| unexpected_err(e, Some("Unable to create google keys file".into())))?;
    file.write_all(
        serde_json::to_string(&top_level_map)
            .map_err(|e| conversion_err(e, Some("Unable to convert google keys to json".into())))?
            .as_bytes(),
    )
    .map_err(|e| unexpected_err(e, Some("Unable to write to google keys file".into())))?;

    Ok(top_level_map["keys"].clone())
}

pub async fn get_google_keys() -> Result<Value> {
    // check if oauth key file is present
    let contents = match fs::read_to_string(GOOGLE_KEYS_FILE_PATH) {
        Ok(content) => content,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                // get the key and save it
                error!("Google keys file not found, downloading...");
                return save_google_keys().await;
            } else {
                return Err(unexpected_err(
                    e,
                    Some("Unable to read google keys file".into()),
                ));
            }
        }
    };

    // parse the json
    let json: Value = serde_json::from_str(&contents).map_err(|e| {
        conversion_err(e, Some("Unable to convert google keys file to json".into()))
    })?;
    let keys = json
        .as_object()
        .expect_or_err("Unable to convert google response to object")
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
            return save_google_keys().await;
        }
    };
    let last_retrieved_at = chrono::DateTime::from_timestamp(last_retrieved_at, 0)
        .expect_or_err("failed to create timestamp opt")
        .map_err(|e| unexpected_err(e, None))?;

    let now = Utc::now();
    let one_hour_ago = now - Duration::hours(1);
    if last_retrieved_at < one_hour_ago {
        // get the keys again
        return save_google_keys().await;
    }

    Ok(keys["keys"].clone())
}

pub async fn parse_and_verify_google_jwt(
    token: &str,
    google_keys: &Vec<Value>,
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
        .expect_or_err("header is not an object")
        .map_err(|e| parser_err(e, Some("Unable to parse header".into())))?;

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
        .expect_or_err("payload is not an object")
        .map_err(|e| parser_err(e, Some("Unable to parse payload".into())))?;

    // decode the signature
    // trace!("Decoding JWT signature: {}", signature_b64);
    let signature = data_encoding::BASE64URL_NOPAD
        .decode(signature_b64.as_bytes())
        .map_err(|e| parser_err(e, Some("Unable to decode".into())))?;

    if header["alg"] != "RS256" {
        return Err(validation_err(
            "Invalid JWT algorithm.  Only rs256 supported".to_string(),
            None,
        ));
    }

    trace!("Decoded google JWT...");

    // bail if it's expired
    let now = Utc::now();
    let jwt_exp = payload["exp"]
        .as_i64()
        .expect_or_err("Could not convert exp to i64")
        .map_err(|e| parser_err(e, None))?;
    let jwt_exp = chrono::DateTime::from_timestamp(jwt_exp, 0)
        .expect_or_err("failed to create timestamp opt")
        .map_err(|e| unexpected_err(e, None))?;
    if now > jwt_exp {
        trace!(
            "JWT expired.  Expiration time: {} and current time: {}",
            jwt_exp,
            now
        );
        return Err(validation_err("Google JWT expired".to_string(), None));
    }

    let mut hasher = Sha256::new();
    hasher.update(header_b64.as_bytes());
    hasher.update(b".");
    hasher.update(payload_b64.as_bytes());
    let signed_message = hasher.finalize();

    trace!(
        "Hashed google JWT: {:?}",
        encoding::bytes_to_hex(signed_message)
    );

    let public_key_id = header["kid"]
        .as_str()
        .expect_or_err("Could not parse kid to string")
        .map_err(|e| parser_err(e, None))?;
    trace!("Need to find public key with id: {}", public_key_id);

    for key in google_keys {
        let parsed_google_keys = key
            .as_object()
            .expect_or_err("key is not an object")
            .map_err(|e| parser_err(e, Some("Unable to parse key".into())))?;
        trace!("checking against key: {:?}", parsed_google_keys);

        if parsed_google_keys["kid"] != public_key_id {
            continue;
        }

        trace!("kid matches, checking signature...");

        let public_key = RsaPublicKey::new(
            BigUint::from_bytes_be(
                &data_encoding::BASE64URL_NOPAD
                    .decode(
                        parsed_google_keys["n"]
                            .as_str()
                            .expect_or_err("Could not parse n to string")
                            .map_err(|e| parser_err(e, None))?
                            .as_bytes(),
                    )
                    .map_err(|e| parser_err(e, Some("Unable to decode google keys".into())))?,
            ),
            BigUint::from_bytes_be(
                &data_encoding::BASE64URL_NOPAD
                    .decode(
                        parsed_google_keys["e"]
                            .as_str()
                            .expect_or_err("Could not parse e to string")
                            .map_err(|e| parser_err(e, None))?
                            .as_bytes(),
                    )
                    .map_err(|e| parser_err(e, Some("Unable to decode google keys".into())))?,
            ),
        )
        .map_err(|e| unexpected_err(e, Some("Unable to create public key".into())))?;

        trace!("parsed pubkey");

        match public_key.verify(
            rsa::padding::PaddingScheme::PKCS1v15Sign {
                hash: Some(rsa::Hash::SHA2_256),
            },
            &signed_message,
            &signature,
        ) {
            Ok(_) => {
                return Ok(models::AuthMethodResponse {
                    user_id: payload["sub"]
                        .as_str()
                        .expect_or_err("Could not parse sub to string")
                        .map_err(|e| parser_err(e, None))?
                        .to_string(),
                    app_id: payload["aud"]
                        .as_str()
                        .expect_or_err("Could not parse aud to string")
                        .map_err(|e| parser_err(e, None))?
                        .to_string(),
                    auth_method_type: GOOGLE_JWT_AUTH_METHOD_TYPE_ID,
                    last_retrieved_at: SystemTime::now(),
                    expiration: jwt_exp.timestamp(),
                    used_for_sign_session_key_request: false,
                });
            }
            Err(e) => {
                trace!("verify failed: {:?}", e);
            }
        }
    }

    Err(validation_err("Invalid JWT".to_string(), None))
}

pub async fn get_google_auth_from_jwt(
    google_access_token: &str,
) -> error::Result<models::AuthMethodResponse> {
    let google_keys = get_google_keys()
        .await
        .map_err(|e| unexpected_err(e, Some("Unable to get google keys".into())))?;
    let google_keys = google_keys
        .as_array()
        .expect_or_err("google keys is not an array")
        .map_err(|e| parser_err(e, Some("Unable to parse google keys".into())))?;

    trace!("Got google keys: {:?}", google_keys);

    parse_and_verify_google_jwt(google_access_token, google_keys).await
}

pub async fn get_google_auth_from_access_token(
    google_access_token: &str,
) -> error::Result<models::AuthMethodResponse> {
    let url = format! {"https://www.googleapis.com/oauth2/v1/tokeninfo?access_token={:}", google_access_token};
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        "application/json"
            .parse()
            .map_err(|e| unexpected_err(e, Some("Unable to parse accept header".into())))?,
    );
    let client = reqwest::Client::new();
    let res = client.get(url).headers(headers).send().await.map_err(|e| {
        unexpected_err(
            e,
            Some("Unable to get google auth from access token".into()),
        )
    })?;
    let body_val = res
        .json::<serde_json::Value>()
        .await
        .map_err(|e| conversion_err(e, Some("Unable to deserialize body".into())))?;
    trace!("Got google auth from access token: {:?}", body_val);

    /* body looks like this:
         {
    Oct 21 23:35:24 serrano lit_node   issued_to: "372111829464-50hbc7gica37noi4r02goi7uiu0drfqc.apps.googleusercontent.com",
    Oct 21 23:35:24 serrano lit_node   audience: "372111829464-50hbc7gica37noi4r02goi7uiu0drfqc.apps.googleusercontent.com",
    Oct 21 23:35:24 serrano lit_node   user_id: "112805338087936768277",
    Oct 21 23:35:24 serrano lit_node   scope: "https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email open...",
    Oct 21 23:35:24 serrano lit_node   expires_in: 1577,
    Oct 21 23:35:24 serrano lit_node   email: "baptiste.greve@gmail.com",
    Oct 21 23:35:24 serrano lit_node   verified_email: true,
    Oct 21 23:35:24 serrano lit_node   access_type: "online"
    Oct 21 23:35:24 serrano lit_node } */
    let body = body_val
        .as_object()
        .expect_or_err("body is not an object")
        .map_err(|e| parser_err(e, Some("Unable to parse body".into())))?;

    if body.get("error").is_some() {
        let error_desc = body
            .get("error_description")
            .unwrap_or(&Value::String("".to_string()))
            .as_str()
            .expect_or_err("error_description is not a string")
            .map_err(|e| parser_err(e, None))?;
        return Err(unexpected_err(
            format!("Error from google: {}", body_val),
            None,
        ));
    }
    let app_id = body
        .get("issued_to")
        .expect_or_err("Couldn't get issued_to")
        .map_err(|e| parser_err(e, None))?
        .as_str()
        .expect_or_err("Couldn't convert issued_to id to string")
        .map_err(|e| conversion_err(e, None))?;
    let user_id = body
        .get("user_id")
        .expect_or_err("Couldn't get user id")
        .map_err(|e| parser_err(e, None))?
        .as_str()
        .expect_or_err("Couldn't convert user id to string")
        .map_err(|e| conversion_err(e, None))?;
    let expires_in = body
        .get("expires_in")
        .expect_or_err("Couldn't get expires_in")
        .map_err(|e| parser_err(e, None))?
        .as_i64()
        .expect_or_err("Could not convert expires_in to i64")
        .map_err(|e| parser_err(e, None))?;
    let expiration = Utc::now() + Duration::seconds(expires_in);

    Ok(models::AuthMethodResponse {
        user_id: user_id.to_string(),
        app_id: app_id.to_string(),
        auth_method_type: GOOGLE_AUTH_METHOD_TYPE_ID,
        last_retrieved_at: SystemTime::now(),
        expiration: expiration.timestamp(),
        used_for_sign_session_key_request: false,
    })
}
