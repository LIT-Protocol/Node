use std::collections::HashMap;

use lit_api_core::config::LitApiConfig;
use lit_os_core::error::conversion_err_code;
use reqwest::{header, Client};
use serde_json::Value;

use crate::{
    config::LitAuthApiConfig,
    ecdsa::SigningPair,
    errors::{config_err, unexpected_err_code, EC},
    twillio::{
        endpoints::constants::{RESP_PROP_CHANNEL, RESP_PROP_STATUS, RESP_PROP_TO},
        models::TwillioVerificationCheckResponse,
    },
};
use crate::{
    errors::parser_err,
    jwx::{
        jws::Json as SignatureJson,
        models::{Jwt, JwtHeader, JwtPayload},
    },
};
use lit_core::{
    config::LitConfig,
    error::{Result, Unexpected},
};
use log::{info, trace};
use serde::Deserialize;

use crate::errors::conversion_err;
use chrono::{offset, Duration};
use digest::Digest;
use email_address_parser::EmailAddress;
use k256::ecdsa::signature::Signer;
use k256::ecdsa::Signature;

use self::constants::CALL_BACK_ROUTE;

use super::models::{EmailConfiguration, RequestOptions};

pub mod constants;
pub mod utils;
pub mod web;

pub async fn send_otp_request(
    to: &String, url: &str, channel: Option<String>, code: Option<String>,
    email_options: Option<EmailConfiguration>, custom_name: Option<String>, cfg: &LitConfig,
) -> Result<String> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().map_err(|e| parser_err(e, None))?,
    );
    let mut form_body: HashMap<&str, String> = HashMap::new();
    form_body.insert("To", to.to_string());

    if let Some(c) = channel {
        info!("Operation resolved to a start verification request channel: {}", c);
        form_body.insert("Channel", c);
    }

    if let Some(options) = email_options {
        trace!("Found email options configured, adding to form body");
        let serialized_options = serde_json::to_string(&options).map_err(|e| {
            conversion_err(e, Some("Could not serialize email options for form body".to_string()))
        })?;
        form_body.insert("ChannelConfiguration", serialized_options);
    }

    if let Some(name) = custom_name {
        trace!("Found custom message adding to form body");
        form_body.insert("CustomFriendlyName", name);
    }

    if let Some(c) = code {
        info!("Operation resolved to a check verification");
        form_body.insert("Code", c);
    }

    let acct_serv_key = cfg
        .twillio_acct_service_id()
        .map_err(|e| config_err(e, Some("Error while parsing configuration".to_string())))?;

    let acct_access_token = cfg
        .twillio_acct_access_token()
        .map_err(|e| config_err(e, Some("Error while parsing configuration".to_string())))?;

    let client = Client::new();
    let res = client
        .post(url)
        .basic_auth(acct_serv_key.to_string(), Some(acct_access_token.to_string()))
        .headers(headers)
        .form(&form_body)
        .send()
        .await
        .map_err(|e| unexpected_err_code(e, EC::VerificationOTPRequestFailure, None))?;

    let body = res
        .text()
        .await
        .map_err(|e| unexpected_err_code(e, EC::VerificationOTPRequestFailure, None))?;

    trace!("result from twillio request: {}", body);

    Ok(body)
}

pub async fn check_otp(cfg: &LitConfig, url: &str) -> Result<TwillioVerificationCheckResponse> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().map_err(|e| parser_err(e, None))?,
    );

    let acct_serv_key = cfg
        .twillio_acct_service_id()
        .map_err(|e| config_err(e, Some("Error while parsing configuration".to_string())))?;

    let acct_access_token = cfg
        .twillio_acct_access_token()
        .map_err(|e| config_err(e, Some("Error while parsing configuration".to_string())))?;

    let client = Client::new();
    let res = client
        .get(url)
        .basic_auth(acct_serv_key.to_string(), Some(acct_access_token.to_string()))
        .headers(headers)
        .send()
        .await
        .map_err(|e| unexpected_err_code(e, EC::VerificationOTPRequestFailure, None))?;

    let body = res
        .text()
        .await
        .map_err(|e| unexpected_err_code(e, EC::VerificationOTPRequestFailure, None))?;

    trace!("result from twillio request: {}", body);

    let body: Value =
        serde_json::from_str(&body).expect_or_err("Could not convert response to JSON")?;
    let body = body.as_object().expect_or_err("Unable to convert check response to object")?;
    let attempt_count =
        body.get("send_code_attempts").expect_or_err("Unable to parse check response")?;
    let attempt_count =
        attempt_count.as_array().expect_or_err("Unable to parse check response attempts")?.len();

    let channel = body.get(RESP_PROP_CHANNEL).expect_or_err("Unable to parse channel from body")?;
    let channel = channel.as_str().expect_or_err("Could not convert channel to string")?;

    let status = body.get(RESP_PROP_STATUS).expect_or_err("Unable to parse channel from body")?;
    let status = status.as_str().expect_or_err("Unable to parse status from body")?;

    let to = body.get(RESP_PROP_TO).expect_or_err("Unable to parse sender from body")?;
    let to = to.as_str().expect_or_err("Unable to parse sender from body")?;

    let resp = TwillioVerificationCheckResponse {
        send_code_attempts: attempt_count as u64,
        channel: channel.to_string(),
        to: to.to_string(),
        status: status.to_string(),
    };

    Ok(resp)
}

pub async fn start_otp_request<T>(options: &RequestOptions, cfg: &LitConfig) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    let serv_id = cfg
        .twillio_verify_service_id()
        .map_err(|e| config_err(e, Some("Error while parsing configuration".to_string())))?;

    let url = build_req_url(&serv_id, &options.code);

    trace!("Formed verification url: {}", url);
    trace!("resolving channel for user_id {}", options.user_id.to_string());
    let channel = resolve_otp_channel_type(&options.user_id).map_err(|e| {
        conversion_err_code(
            e,
            EC::VerificationBadRequestBody,
            Some("Unable to parse otp transport".into()),
        )
    })?;
    trace!("channel resolved to: {}", channel);
    let body_str = match &options.code {
        Some(c) => {
            trace!("code found sending code verification request");
            send_otp_request(
                &options.user_id,
                url.as_str(),
                Some(channel),
                Some(c.to_string()),
                options.email_configuration.to_owned(),
                options.custom_name.to_owned(),
                cfg,
            )
            .await
            .map_err(|e| unexpected_err_code(e, EC::VerificationOTPRequestFailure, None))?
        }
        None => {
            trace!("code not found, sending start verification request");
            send_otp_request(
                &options.user_id,
                url.as_str(),
                Some(channel),
                None,
                options.email_configuration.to_owned(),
                options.custom_name.to_owned(),
                cfg,
            )
            .await
            .map_err(|e| unexpected_err_code(e, EC::VerificationOTPRequestFailure, None))?
        }
    };

    trace!("verification response recieved: {}", body_str);
    let body: T = serde_json::from_str(&body_str).map_err(|e| {
        conversion_err(e, Some("Could not convert request body to response type".into()))
    })?;

    Ok(body)
}

// tries to parse the otp transport as an email, then as a phone #.
//will return an error if it cannot parse the phone # only
pub fn resolve_otp_channel_type(otp: &String) -> Result<String> {
    let email = EmailAddress::parse(otp, None);
    if email.is_some() {
        Ok("email".to_string())
    } else {
        phonenumber::parse(None, otp).map_err(|e| {
            conversion_err(e, Some("Could not parse user otp for sending request".to_string()))
        })?;

        Ok("sms".to_string())
    }
}

pub fn build_jwt(id: &str, key_shares: &SigningPair) -> Result<String> {
    let time_stamp = offset::Utc::now();
    let data: String = format!("{}|{}", id, time_stamp.to_rfc3339());
    let pl = JwtPayload {
        iss: "LIT-Protocol".to_string(),
        sub: "LIT-OTP".to_string(),
        iat: time_stamp.timestamp_millis(),
        exp: (time_stamp + Duration::minutes(30)).timestamp_millis(),
        org_id: "LIT".to_string(),
        role: "user".to_string(),
        extra_data: data.to_string(),
    };
    trace!("hashing content for signing {}", data);
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(data.as_bytes());
    let digest = hasher.finalize();
    info!("hashed message: {:?} message length: {}", digest, digest.len());
    let signed_token: Signature = key_shares.signing_key.sign(&digest);

    let sig_json = signed_token.to_json().map_err(|e| conversion_err(e, None))?;

    let signature = serde_json::to_string(&sig_json).map_err(|e| conversion_err(e, None))?;
    trace!("converted signature: {}", signature);

    let jwt: Jwt = Jwt {
        header: JwtHeader { alg: "secp256k1".to_string(), typ: "JWT".to_string() },
        payload: pl,
        signature: Some(signature),
    };
    let token_str = jwt.as_base64().expect_or_err("Couldnt not convert JWT to base64 string")?;

    Ok(token_str)
}

fn build_req_url(serv_id: &String, code: &Option<String>) -> String {
    match code {
        Some(_) => {
            trace!("resolving url for verification check");
            format!(
                "https://verify.twilio.com/v2/Services/{serv_id}/VerificationCheck",
                serv_id = serv_id
            )
        }
        None => {
            trace!("resolving url for start verification");
            format!(
                "https://verify.twilio.com/v2/Services/{serv_id}/Verifications",
                serv_id = serv_id
            )
        }
    }
}

fn build_status_url(cfg: &LitConfig, request_id: &str) -> Result<String> {
    let host = cfg.api_domain().map_err(|e| config_err(e, None))?;
    let port = cfg.api_http_port().map_err(|e| config_err(e, None))?;
    Ok(format!("{}:{}{}{}", host, port, CALL_BACK_ROUTE, request_id))
}
