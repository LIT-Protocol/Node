use super::AuthMethodVerifier;
use crate::error::{self, conversion_err, io_err, parser_err, unexpected_err, validation_err};
use crate::models;
use crate::pkp::auth::constants::{
    STYTCH_JWT_AUTH_FACTOR_EMAIL_OTP, STYTCH_JWT_AUTH_FACTOR_SMS_OTP, STYTCH_JWT_AUTH_FACTOR_TOTP,
    STYTCH_JWT_AUTH_FACTOR_WHATS_APP_OTP, STYTCH_JWT_AUTH_METHOD_TYPE_ID,
};
use chrono::{Duration, Utc};
use lit_core::error::{Result, Unexpected};
use rsa::{BigUint, PublicKey, RsaPublicKey};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;

use std::str::{self};
use std::time::{SystemTime, UNIX_EPOCH};

const VERIFICATION_API_ADDR_TEST: &str = "https://test.stytch.com/v1/sessions/jwks/";
const VERIFICATION_API_ADDR_LIVE: &str = "https://api.stytch.com/v1/sessions/jwks/";

const AUTHORIZATION_KEYS_FILE_DIR_PATH: &str = "./stytch";
const VERIFICATION_PUBLIC_KEY: &str = "keys";

const CACHE_DURATION_HOURS: i64 = 1;

const JWT_ALG: &str = "alg";
const JWT_AUD: &str = "aud";
const JWT_EXP: &str = "exp";
const JWT_SUB: &str = "sub";
const JWT_LAST_RETRIEVED_AT: &str = "last_retrieved_at";
const JWT_RSA_ALG_VAL: &str = "RS256";

const SESSION_PROVIDER_CONTEXT: &str = "https://stytch.com/session";
const AUTH_FACTOR_CONTEXT: &str = "authentication_factors";

const STYTCH_AUTH_FACTOR_TYPE_SMS: &str = "sms";
const STYTCH_AUTH_FACTOR_TYPE_EMAIL: &str = "email";
const STYTCH_AUTH_FACTOR_TYPE_WHATS_APP: &str = "whatsApp";
const STYTCH_AUTH_FACTOR_TYPE_TOTP: &str = "totp";

const STYTCH_AUTH_FACTOR_SESSION_SMS: &str = "phone_number_factor";
const STYTCH_AUTH_FACTOR_SESSION_WHATS_APP: &str = "phone_number_factor";
const STYTCH_AUTH_FACTOR_SESSION_EMAIL: &str = "email_factor";
const STYTCH_AUTH_FACTOR_SESSION_TOTP: &str = "authenticator_app_factor";

const STYTCH_AUTH_FACTOR_USER_ID_SMS: &str = "phone_number";
const STYTCH_AUTH_FACTOR_USER_ID_WHATS_APP: &str = "phone_number";
const STYTCH_AUTH_FACTOR_USER_ID_EMAIL: &str = "email_address";
const STYTCH_AUTH_FACTOR_USER_ID_TOTP: &str = "totp_id";

pub struct StytchJWTAuthMethodVerifier {
    pub factor: Option<String>,
}

#[async_trait::async_trait]
impl AuthMethodVerifier for StytchJWTAuthMethodVerifier {
    async fn verify(&self, access_token: &str) -> Result<models::AuthMethodResponse> {
        return get_otp_auth_from_jwt(access_token, self.factor.clone()).await;
    }
}

pub async fn get_otp_auth_from_jwt(
    token: &str,
    factor: Option<String>,
) -> Result<models::AuthMethodResponse> {
    let auth_keys = get_auth_key(token).await;
    if let Err(e) = auth_keys {
        return Err(unexpected_err(
            e,
            Some("Unable to get Stytch auth keys".into()),
        ));
    }
    match auth_keys {
        Ok(keys) => {
            let keys = keys
                .as_array()
                .expect_or_err("Could not convert public key set to array aborting")?;
            parse_and_verify_otp_jwt(token, keys, factor).await
        }
        Err(e) => Err(e),
    }
}

pub async fn get_auth_key(token: &str) -> error::Result<Value> {
    let project_id = parse_key_for_public_key_context(token).map_err(|e| {
        parser_err(
            e,
            Some("Could not parse project id from stytch auth token".to_string()),
        )
    })?;
    let file_name = format!("{}.json", project_id.0);
    let contents = fs::read_to_string(format!(
        "{}/{}",
        AUTHORIZATION_KEYS_FILE_DIR_PATH, project_id.0
    ));
    if let Err(e) = contents {
        if e.kind() == std::io::ErrorKind::NotFound {
            trace!("Auth keys not found, downloading...");
            return save_auth_key(project_id.0.as_str(), project_id.1.as_str()).await;
        } else {
            return Err(io_err(
                e,
                Some(format!("Failed to read from file {}", file_name)),
            ));
        }
    }

    let contents = contents
        .map_err(|e| conversion_err(e, Some("Error while converting contents to string".into())))?;

    let json: Value = serde_json::from_str(&contents)
        .map_err(|e| conversion_err(e, Some("Unable to convert auth key file to JSON".into())))?;

    let key = json
        .as_object()
        .expect_or_err("Unable to convert Auth response to object")?
        .clone();

    let last_retrieved_at = key.get(JWT_LAST_RETRIEVED_AT);

    debug!("Checking expiration");
    match last_retrieved_at {
        Some(at) => {
            let last_retrieved_at = at
                .as_i64()
                .expect_or_err("Could not convert last_retrieved_at to i64")?;

            let last_retrieved_at = chrono::DateTime::from_timestamp(last_retrieved_at, 0)
                .expect_or_err("failed to create timestamp opt")?;

            let now = Utc::now();
            let one_hour_ago = now - Duration::hours(CACHE_DURATION_HOURS);
            if last_retrieved_at < one_hour_ago {
                debug!(
                    "Cache found to be expired, fetching public keys for project: {}",
                    project_id.0
                );
                // get the keys again
                return save_auth_key(token, project_id.1.as_str()).await;
            }
        }
        None => {
            // get the keys again
            return save_auth_key(token, project_id.1.as_str()).await;
        }
    }

    debug!(
        "Cache found to be valid returning keys from cache: {:?}",
        key
    );
    match key.get(VERIFICATION_PUBLIC_KEY) {
        Some(keys) => Ok(keys.clone()),
        None => Err(validation_err(
            "Could not find public key material in response".to_string(),
            None,
        )),
    }
}

pub async fn save_auth_key(project_id: &str, enviorment: &str) -> Result<Value> {
    trace!("Downloading stytch keys for project_id {}", project_id);
    let mut url = "".to_string();
    if enviorment == "test" {
        url = format!("{}{}", VERIFICATION_API_ADDR_TEST, project_id);
    } else if enviorment == "live" {
        url = format!("{}{}", VERIFICATION_API_ADDR_LIVE, project_id);
    }

    let resp = reqwest::get(url)
        .await
        .map_err(|e| unexpected_err(e, Some("Unable to send request to stytch".into())))?;
    let text = resp
        .text()
        .await
        .map_err(|e| unexpected_err(e, Some("Unable to get text from stytch response".into())))?;

    let json: Value = serde_json::from_str(text.as_str())
        .map_err(|e| conversion_err(e, Some("Unable to convert stytch response to json".into())))?;
    // get the keys out
    let mut top_level_map = json
        .as_object()
        .expect_or_err("Unable to convert stytch response to object")
        .map_err(|e| conversion_err(e, None))?
        .clone();
    trace!("stytch keys: {:?}", top_level_map);

    let now = SystemTime::now();
    let now_unix_time = now
        .duration_since(UNIX_EPOCH)
        .expect_or_err("Time went backwards")
        .map_err(|e| unexpected_err(e, None))?;

    // add latest timestamp
    top_level_map.insert(
        JWT_LAST_RETRIEVED_AT.to_string(),
        Value::Number(now_unix_time.as_secs().into()),
    );
    fs::create_dir_all(AUTHORIZATION_KEYS_FILE_DIR_PATH).map_err(|e| {
        unexpected_err(
            e,
            Some("Could not create directory for public key files".into()),
        )
    })?;

    let file_name = format!("{}.json", project_id);

    // write to the keys file
    let mut file = File::create(format!(
        "{}/{}",
        AUTHORIZATION_KEYS_FILE_DIR_PATH, file_name
    ))
    .map_err(|e| unexpected_err(e, Some("Unable to create stytch keys file".into())))?;
    file.write_all(
        serde_json::to_string(&top_level_map)
            .map_err(|e| conversion_err(e, Some("Unable to convert stytch keys to json".into())))?
            .as_bytes(),
    )
    .map_err(|e| unexpected_err(e, Some("Unable to write to stytch keys file".into())))?;

    match top_level_map.get(VERIFICATION_PUBLIC_KEY) {
        Some(keys) => Ok(keys.clone()),
        None => Err(validation_err(
            "Could not find public key material in response".to_string(),
            None,
        )),
    }
}

pub async fn parse_and_verify_otp_jwt(
    token: &str,
    keys: &Vec<Value>,
    factor: Option<String>,
) -> Result<models::AuthMethodResponse> {
    let parts: Vec<&str> = token.splitn(3, '.').collect();

    if parts.len() != 3 {
        return Err(validation_err("Invalid JWT parts length".to_string(), None));
    }

    let header_b64 = parts
        .first()
        .expect_or_err("Could not parse header from token")?;
    let payload_b64 = parts
        .get(1)
        .expect_or_err("Could not parse payload from token")?;
    let signature_b64 = parts
        .get(2)
        .expect_or_err("Could not parse signature from token")?;

    let header = data_encoding::BASE64URL_NOPAD
        .decode(header_b64.as_bytes())
        .map_err(|e| parser_err(e, Some("Unable to decode".into())))?;
    let header: Value = serde_json::from_str(
        str::from_utf8(&header)
            .map_err(|e| parser_err(e, Some("Unable to parse token header to string".into())))?,
    )
    .map_err(|e| conversion_err(e, Some("Unable to convert header to json".into())))?;
    let header = header
        .as_object()
        .expect_or_err("Could not convert header to object, invalid jwt")?;

    let signature = data_encoding::BASE64URL_NOPAD
        .decode(signature_b64.as_bytes())
        .map_err(|e| parser_err(e, Some("Unable to decode signature from payload".into())))?;

    let payload = data_encoding::BASE64URL_NOPAD
        .decode(payload_b64.as_bytes())
        .map_err(|e| parser_err(e, Some("Unable to decode token payload".into())))?;
    let payload: Value = serde_json::from_str(
        str::from_utf8(&payload)
            .map_err(|e| parser_err(e, Some("Unable to parse string".into())))?,
    )
    .map_err(|e| conversion_err(e, Some("Unable to convert to value".into())))?;

    let payload = payload
        .as_object()
        .expect_or_err("Could not convert payload to object, invalid jwt")?;

    let alg = header
        .get(JWT_ALG)
        .expect_or_err("Could not find alg in header")?;

    let alg = alg
        .as_str()
        .expect_or_err("Could not convert alg to string")?;

    if alg != JWT_RSA_ALG_VAL {
        return Err(validation_err(
            format!("Invalid JWT algorithm. Only {} supported", JWT_RSA_ALG_VAL),
            None,
        ));
    }
    #[cfg(not(test))]
    {
        let now = Utc::now();
        let jwt_exp = payload
            .get(JWT_EXP)
            .expect_or_err(format!("Can not find key {} in payload", JWT_EXP))?
            .as_i64()
            .expect_or_err(format!("could not convert {} to numeric", JWT_EXP))?;
        let jwt_exp = chrono::DateTime::from_timestamp(jwt_exp, 0)
            .expect_or_err("failed to create timestamp opt")
            .map_err(|e| unexpected_err(e, None))?;

        if now > jwt_exp {
            trace!(
                "JWT expired expiration time: {} and current time: {}",
                jwt_exp,
                now
            );
            return Err(validation_err("OTP token expired".to_string(), None));
        }
    }

    let mut hasher = Sha256::new();
    hasher.update(header_b64.as_bytes());
    hasher.update(b".");
    hasher.update(payload_b64.as_bytes());
    let signed_message = hasher.finalize();

    for key in keys {
        trace!("checking key: {}", key);
        let parsed_key = match key.as_object() {
            Some(key) => key,
            None => continue,
        };
        if parsed_key["alg"] != JWT_RSA_ALG_VAL {
            continue;
        }

        trace!("matched kid, checking signature...");
        let rsa_n = parsed_key["n"]
            .as_str()
            .expect_or_err("Could not parse n as string")
            .map_err(|e| {
                parser_err(
                    e,
                    Some("Could not parse public key materials from endpoint response.".into()),
                )
            })?
            .as_bytes();
        let rsa_e = parsed_key["e"]
            .as_str()
            .expect_or_err("Could not parse e as string")
            .map_err(|e| {
                parser_err(
                    e,
                    Some("Could not parse public key materials from endpoint response.".into()),
                )
            })?
            .as_bytes();
        let public_key = RsaPublicKey::new(
            BigUint::from_bytes_be(
                data_encoding::BASE64URL_NOPAD
                    .decode(rsa_n)
                    .map_err(|e| parser_err(e, Some("Unable to decode Stych key".into())))?
                    .as_slice(),
            ),
            BigUint::from_bytes_be(
                data_encoding::BASE64URL_NOPAD
                    .decode(rsa_e)
                    .map_err(|e| parser_err(e, Some("Unable to decode Stych key".into())))?
                    .as_slice(),
            ),
        )
        .map_err(|e| unexpected_err(e, Some("Unable to create public key".into())))?;

        trace!("parsed public key");

        match public_key.verify(
            rsa::padding::PaddingScheme::PKCS1v15Sign {
                hash: Some(rsa::Hash::SHA2_256),
            },
            &signed_message,
            &signature,
        ) {
            Ok(_) => {
                let user_id = match factor.clone() {
                    Some(auth_factor) => {
                        // if the factor is defined then
                        parse_auth_factor_from_token(payload, &auth_factor).map_err(|e| {
                            unexpected_err(e, Some("could not parse auth factor from token".into()))
                        })?
                    }
                    None => {
                        // if the factor is `None` we are using the `sub` claim of the token which is the stytch user id (uuidv4)
                        payload[JWT_SUB]
                            .as_str()
                            .expect_or_err("Could not parse user id from token")?
                            .to_string()
                    }
                };
                let mut app_id = payload[JWT_AUD]
                    .as_array()
                    .expect_or_err("Could not get app id from token")?
                    .first()
                    .expect_or_err("Could not get app id from token, not found in collection")?
                    .to_string();

                let jwt_exp = payload
                    .get(JWT_EXP)
                    .expect_or_err(format!("Can not find key {} in payload", JWT_EXP))?
                    .as_i64()
                    .expect_or_err(format!("could not convert {} to numeric", JWT_EXP))?;

                app_id = app_id.replace(['\"'], "");
                match factor.clone() {
                    Some(f) => match f.as_str() {
                        STYTCH_AUTH_FACTOR_TYPE_EMAIL => {
                            return Ok(models::AuthMethodResponse {
                                user_id: user_id.to_ascii_lowercase(),
                                app_id: app_id.to_ascii_lowercase(),
                                auth_method_type: STYTCH_JWT_AUTH_FACTOR_EMAIL_OTP,
                                last_retrieved_at: SystemTime::now(),
                                expiration: jwt_exp,
                                used_for_sign_session_key_request: false,
                            });
                        }
                        STYTCH_AUTH_FACTOR_TYPE_SMS => {
                            return Ok(models::AuthMethodResponse {
                                user_id: user_id.to_ascii_lowercase(),
                                app_id: app_id.to_ascii_lowercase(),
                                auth_method_type: STYTCH_JWT_AUTH_FACTOR_SMS_OTP,
                                last_retrieved_at: SystemTime::now(),
                                expiration: jwt_exp,
                                used_for_sign_session_key_request: false,
                            });
                        }

                        STYTCH_AUTH_FACTOR_TYPE_WHATS_APP => {
                            return Ok(models::AuthMethodResponse {
                                user_id: user_id.to_ascii_lowercase(),
                                app_id: app_id.to_ascii_lowercase(),
                                auth_method_type: STYTCH_JWT_AUTH_FACTOR_WHATS_APP_OTP,
                                last_retrieved_at: SystemTime::now(),
                                expiration: jwt_exp,
                                used_for_sign_session_key_request: false,
                            });
                        }

                        STYTCH_AUTH_FACTOR_TYPE_TOTP => {
                            return Ok(models::AuthMethodResponse {
                                user_id: user_id.to_ascii_lowercase(),
                                app_id: app_id.to_ascii_lowercase(),
                                auth_method_type: STYTCH_JWT_AUTH_FACTOR_TOTP,
                                last_retrieved_at: SystemTime::now(),
                                expiration: jwt_exp,
                                used_for_sign_session_key_request: false,
                            });
                        }

                        _ => {
                            return Err(unexpected_err("Non supported factor type", None));
                        }
                    },
                    None => {
                        return Ok(models::AuthMethodResponse {
                            user_id: user_id.to_ascii_lowercase(),
                            app_id: app_id.to_ascii_lowercase(),
                            auth_method_type: STYTCH_JWT_AUTH_METHOD_TYPE_ID,
                            last_retrieved_at: SystemTime::now(),
                            expiration: jwt_exp,
                            used_for_sign_session_key_request: false,
                        });
                    }
                }
            }
            Err(e) => {
                trace!("verify failed {:?}", e);
            }
        }
    }

    Err(validation_err("Invalid JWT".to_string(), None))
}

fn parse_key_for_public_key_context(token: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = token.splitn(3, '.').collect();

    if parts.len() != 3 {
        return Err(validation_err("Invalid JWT parts length".to_string(), None));
    }

    let payload_b64 = parts
        .get(1)
        .expect_or_err("Could not parse payload from token")?;
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
        .expect_or_err("Could not convert payload to object, invalid jwt")?;

    let audiences = payload[JWT_AUD]
        .as_array()
        .expect_or_err("Could not parse audience from token")?;
    let aud = audiences.first();

    match aud {
        Some(res) => {
            debug!("found project id in token: {}", res);
            let project_id = res
                .as_str()
                .expect_or_err("Could not parse app id from token")?
                .to_string();
            if project_id.contains("test") {
                return Ok((project_id, "test".to_string()));
            } else if project_id.contains("live") {
                return Ok((project_id, "live".to_string()));
            }
        }
        None => {
            return Err(parser_err("Could not parse audience from token", None));
        }
    };

    Err(parser_err("Could not parse audience from token", None))
}

fn parse_auth_factor_from_token(
    token: &serde_json::Map<String, Value>,
    factor: &String,
) -> Result<String> {
    let factors = token[SESSION_PROVIDER_CONTEXT]
        .as_object()
        .expect_or_err("Could not find auth factors in token")?;
    let sessions = factors[AUTH_FACTOR_CONTEXT]
        .as_array()
        .expect_or_err("Could not convert sessions to collection")?;
    if sessions.is_empty() {
        return Err(unexpected_err("No valid sessions in token", None));
    }

    for session in sessions {
        let user_id = match session.as_object() {
            Some(auth_factor) => match factor.as_str() {
                STYTCH_AUTH_FACTOR_TYPE_SMS => {
                    match auth_factor.get(STYTCH_AUTH_FACTOR_SESSION_SMS) {
                        Some(phone_number_factor) => {
                            let user_id = phone_number_factor[STYTCH_AUTH_FACTOR_USER_ID_SMS]
                                .as_str()
                                .expect_or_err("Could not find user id in auth factor")?;
                            Ok(user_id.to_string())
                        }
                        None => Err(unexpected_err(
                            "Could not find factor in authentication factor",
                            None,
                        )),
                    }
                }
                STYTCH_AUTH_FACTOR_TYPE_WHATS_APP => {
                    match auth_factor.get(STYTCH_AUTH_FACTOR_SESSION_WHATS_APP) {
                        Some(whats_app_factor) => {
                            let user_id = whats_app_factor[STYTCH_AUTH_FACTOR_USER_ID_WHATS_APP]
                                .as_str()
                                .expect_or_err("Could not find user id in auth factor")?;
                            Ok(user_id.to_string())
                        }
                        None => Err(unexpected_err(
                            "Could not find factor in authentication factor",
                            None,
                        )),
                    }
                }
                STYTCH_AUTH_FACTOR_TYPE_TOTP => {
                    match auth_factor.get(STYTCH_AUTH_FACTOR_SESSION_TOTP) {
                        Some(auth_app_factor) => {
                            let user_id = auth_app_factor[STYTCH_AUTH_FACTOR_USER_ID_TOTP]
                                .as_str()
                                .expect_or_err("Could not find user id in auth factor")?;
                            Ok(user_id.to_string())
                        }
                        None => Err(unexpected_err(
                            "Could not find factor in authentication factor",
                            None,
                        )),
                    }
                }
                STYTCH_AUTH_FACTOR_TYPE_EMAIL => {
                    match auth_factor.get(STYTCH_AUTH_FACTOR_SESSION_EMAIL) {
                        Some(email_factor) => {
                            let user_id = email_factor[STYTCH_AUTH_FACTOR_USER_ID_EMAIL]
                                .as_str()
                                .expect_or_err("Could not find user id in auth factor")?;
                            Ok(user_id.to_string())
                        }
                        None => Err(unexpected_err(
                            "Could not find factor in authentication factor",
                            None,
                        )),
                    }
                }
                _ => Err(unexpected_err(
                    "Non supported factor, please use generic stytch option",
                    None,
                )),
            },
            None => {
                continue;
            }
        };

        return user_id;
    }

    Err(unexpected_err(
        "Could not find auth factor in session contexts",
        None,
    ))
}

#[cfg(test)]
mod tests {
    use crate::pkp::auth::auth_method_verifier::AuthMethodVerifier;
    use crate::pkp::auth::stytch::StytchJWTAuthMethodVerifier;

    #[tokio::test]
    async fn should_verify_stytch_no_auth_factor() {
        // token from test project, not used in any production context
        const TEST_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6Imp3ay10ZXN0LWVlZGI5YjU1LTRjNWItNGE4Ni1iODVmLWVjOTlkZmMwY2EyNyIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsicHJvamVjdC10ZXN0LTI3OTA2MzVkLWJhMzEtNDcwNS1hYWY0LTIxM2JiMGYwNjgxMiJdLCJleHAiOjE3MjAwOTgwODQsImh0dHBzOi8vc3R5dGNoLmNvbS9zZXNzaW9uIjp7ImlkIjoic2Vzc2lvbi10ZXN0LWUzYzhiMmIyLTRjYjctNDI1MS1iZGZiLTNjM2I4YzZjOTEzOCIsInN0YXJ0ZWRfYXQiOiIyMDI0LTA3LTA0VDEyOjU2OjI0WiIsImxhc3RfYWNjZXNzZWRfYXQiOiIyMDI0LTA3LTA0VDEyOjU2OjI0WiIsImV4cGlyZXNfYXQiOiIyMDI0LTA3LTExVDEyOjU2OjI0WiIsImF0dHJpYnV0ZXMiOnsidXNlcl9hZ2VudCI6IiIsImlwX2FkZHJlc3MiOiIifSwiYXV0aGVudGljYXRpb25fZmFjdG9ycyI6W3sidHlwZSI6Im90cCIsImRlbGl2ZXJ5X21ldGhvZCI6ImVtYWlsIiwibGFzdF9hdXRoZW50aWNhdGVkX2F0IjoiMjAyNC0wNy0wNFQxMjo1NjoyNFoiLCJlbWFpbF9mYWN0b3IiOnsiZW1haWxfaWQiOiJlbWFpbC10ZXN0LWUyOTZjMjZmLTMwYzQtNDljZS1hOTdhLTRmNTM0YTRhNjk3MSIsImVtYWlsX2FkZHJlc3MiOiJqb3NoQGxpdHByb3RvY29sLmNvbSJ9fV19LCJpYXQiOjE3MjAwOTc3ODQsImlzcyI6InN0eXRjaC5jb20vcHJvamVjdC10ZXN0LTI3OTA2MzVkLWJhMzEtNDcwNS1hYWY0LTIxM2JiMGYwNjgxMiIsIm5iZiI6MTcyMDA5Nzc4NCwic3ViIjoidXNlci10ZXN0LTk3MWYyYjAxLWRjYWMtNGNhOC04OGQ4LTFlMTI0MDViMTc5MyJ9.DKFizX3EUPFvf7BYw17gsY0I3k3EfIORQg_VpKsKSWa0X-yqGEyll6cP2MxjyTmh5N_0XZ1po-7EAWZWYYZ9tJ-XH-st3pdjmB_-R1JoTxlFgpoQw0joaqfQlTMxC8oCnTgi6MAFgDJY-37KU1ddg-JrAib-6GUrKDyos0SI6a5D_i1iYIiF7EbF2q8RTUbJQnbxSZs-fO9Xcl9qDXZztFQcL7y7qOSH5XzoqNhE2X0UqYYETWRFi3YUgQZ8e1RDxX1Ik2sjE5xWCUiWJ91OzEXWBHzGKLSkLQ-jJUKrA3l-5UzYuay5dorKz1w4bQVlKOXZljC6Zpwx0QYGB50paQ";
        let verifier = StytchJWTAuthMethodVerifier { factor: None };
        let res = verifier.verify(TEST_TOKEN).await;
        let res = res.unwrap();
        println!("response {:?}", res);
    }

    #[tokio::test]
    async fn should_verify_stytch_email_auth_factor() {
        // token from test project, not used in any production context
        const TEST_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6Imp3ay10ZXN0LWVlZGI5YjU1LTRjNWItNGE4Ni1iODVmLWVjOTlkZmMwY2EyNyIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsicHJvamVjdC10ZXN0LTI3OTA2MzVkLWJhMzEtNDcwNS1hYWY0LTIxM2JiMGYwNjgxMiJdLCJleHAiOjE3MjAwOTgwODQsImh0dHBzOi8vc3R5dGNoLmNvbS9zZXNzaW9uIjp7ImlkIjoic2Vzc2lvbi10ZXN0LWUzYzhiMmIyLTRjYjctNDI1MS1iZGZiLTNjM2I4YzZjOTEzOCIsInN0YXJ0ZWRfYXQiOiIyMDI0LTA3LTA0VDEyOjU2OjI0WiIsImxhc3RfYWNjZXNzZWRfYXQiOiIyMDI0LTA3LTA0VDEyOjU2OjI0WiIsImV4cGlyZXNfYXQiOiIyMDI0LTA3LTExVDEyOjU2OjI0WiIsImF0dHJpYnV0ZXMiOnsidXNlcl9hZ2VudCI6IiIsImlwX2FkZHJlc3MiOiIifSwiYXV0aGVudGljYXRpb25fZmFjdG9ycyI6W3sidHlwZSI6Im90cCIsImRlbGl2ZXJ5X21ldGhvZCI6ImVtYWlsIiwibGFzdF9hdXRoZW50aWNhdGVkX2F0IjoiMjAyNC0wNy0wNFQxMjo1NjoyNFoiLCJlbWFpbF9mYWN0b3IiOnsiZW1haWxfaWQiOiJlbWFpbC10ZXN0LWUyOTZjMjZmLTMwYzQtNDljZS1hOTdhLTRmNTM0YTRhNjk3MSIsImVtYWlsX2FkZHJlc3MiOiJqb3NoQGxpdHByb3RvY29sLmNvbSJ9fV19LCJpYXQiOjE3MjAwOTc3ODQsImlzcyI6InN0eXRjaC5jb20vcHJvamVjdC10ZXN0LTI3OTA2MzVkLWJhMzEtNDcwNS1hYWY0LTIxM2JiMGYwNjgxMiIsIm5iZiI6MTcyMDA5Nzc4NCwic3ViIjoidXNlci10ZXN0LTk3MWYyYjAxLWRjYWMtNGNhOC04OGQ4LTFlMTI0MDViMTc5MyJ9.DKFizX3EUPFvf7BYw17gsY0I3k3EfIORQg_VpKsKSWa0X-yqGEyll6cP2MxjyTmh5N_0XZ1po-7EAWZWYYZ9tJ-XH-st3pdjmB_-R1JoTxlFgpoQw0joaqfQlTMxC8oCnTgi6MAFgDJY-37KU1ddg-JrAib-6GUrKDyos0SI6a5D_i1iYIiF7EbF2q8RTUbJQnbxSZs-fO9Xcl9qDXZztFQcL7y7qOSH5XzoqNhE2X0UqYYETWRFi3YUgQZ8e1RDxX1Ik2sjE5xWCUiWJ91OzEXWBHzGKLSkLQ-jJUKrA3l-5UzYuay5dorKz1w4bQVlKOXZljC6Zpwx0QYGB50paQ";
        let verifier = StytchJWTAuthMethodVerifier {
            factor: Some("email".to_string()),
        };
        let res = verifier.verify(TEST_TOKEN).await;
        let res = res.unwrap();
        println!("response {:?}", res);
    }

    #[tokio::test]
    async fn should_verify_stytch_sms_auth_factor() {
        // token from test project, not used in any production context
        const TEST_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6Imp3ay10ZXN0LWVlZGI5YjU1LTRjNWItNGE4Ni1iODVmLWVjOTlkZmMwY2EyNyIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsicHJvamVjdC10ZXN0LTI3OTA2MzVkLWJhMzEtNDcwNS1hYWY0LTIxM2JiMGYwNjgxMiJdLCJleHAiOjE3MjAwOTg3MDUsImh0dHBzOi8vc3R5dGNoLmNvbS9zZXNzaW9uIjp7ImlkIjoic2Vzc2lvbi10ZXN0LWVjOGU3ZWRiLTc5YzctNDg3Mi05NjQ3LTlkMzY1YzlkYTEzOSIsInN0YXJ0ZWRfYXQiOiIyMDI0LTA3LTA0VDEzOjA2OjQ1WiIsImxhc3RfYWNjZXNzZWRfYXQiOiIyMDI0LTA3LTA0VDEzOjA2OjQ1WiIsImV4cGlyZXNfYXQiOiIyMDI0LTA3LTExVDEzOjA2OjQ1WiIsImF0dHJpYnV0ZXMiOnsidXNlcl9hZ2VudCI6IiIsImlwX2FkZHJlc3MiOiIifSwiYXV0aGVudGljYXRpb25fZmFjdG9ycyI6W3sidHlwZSI6Im90cCIsImRlbGl2ZXJ5X21ldGhvZCI6InNtcyIsImxhc3RfYXV0aGVudGljYXRlZF9hdCI6IjIwMjQtMDctMDRUMTM6MDY6NDVaIiwicGhvbmVfbnVtYmVyX2ZhY3RvciI6eyJwaG9uZV9pZCI6InBob25lLW51bWJlci10ZXN0LWNmYTJjYTFlLTVmNzMtNGM0NS1hOWU2LWNjYWE5MWZkYjE3ZCIsInBob25lX251bWJlciI6IisxMjAxNDA3MjA3MyJ9fV19LCJpYXQiOjE3MjAwOTg0MDUsImlzcyI6InN0eXRjaC5jb20vcHJvamVjdC10ZXN0LTI3OTA2MzVkLWJhMzEtNDcwNS1hYWY0LTIxM2JiMGYwNjgxMiIsIm5iZiI6MTcyMDA5ODQwNSwic3ViIjoidXNlci10ZXN0LWFhMGUwZmFhLTFjZTgtNGY4Yy05MGJhLWU2OTZmMWY4OTFlZiJ9.ms0b45kuOTKRePK2hLNZQO73-2VujpcaKBEHYVDGewsNTWCdLW0eqqpGSep8CMyDQtdnfcnffJ3wH_r0FP4vLHsls7YtuF9hR2fPK73G0pTkzebobpSYWTVyUs_bOewgRniQIiUXfvZQ1Dy21t4vIhUj_tkFrMrguskWFSPhPY_z8BPwQjCll-l0WPGrQF5rDzP3tuJwrjmL9mIm6hoIfIxYfBlO-c8x22ZIwxVUiiAltFSlgiILOLFz7byV5UcroEqUm7MYJm7HgiXzGJ2QY3QiRNKsl7x745Hj3dLz2G2ZTTvA-nta_6yFhCiGThmjgJtvO6ksd9KMes5O937Q4g";
        let verifier = StytchJWTAuthMethodVerifier {
            factor: Some("sms".to_string()),
        };
        let res = verifier.verify(TEST_TOKEN).await;
        let res = res.unwrap();
        println!("response {:?}", res);
    }
}
