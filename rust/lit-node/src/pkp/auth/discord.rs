use crate::error::{self, conversion_err, parser_err, unexpected_err, Unexpected};
use crate::models;
use chrono::Days;
use reqwest;
use std::str;

use std::time::SystemTime;

use super::auth_method_verifier::AuthMethodVerifier;
use super::constants::DISCORD_AUTH_METHOD_TYPE_ID;

pub struct DiscordAuthMethodVerifier {}

#[async_trait::async_trait]
impl AuthMethodVerifier for DiscordAuthMethodVerifier {
    async fn verify(&self, access_token: &str) -> error::Result<models::AuthMethodResponse> {
        debug!("Getting discord user id and app id from access token");
        get_discord_auth_from_access_token(access_token).await
    }
}

pub async fn get_discord_auth_from_access_token(
    discord_access_token: &str,
) -> error::Result<models::AuthMethodResponse> {
    let url = "https://discord.com/api/oauth2/@me";
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", discord_access_token)
            .parse()
            .map_err(|e| {
                parser_err(
                    e,
                    Some("Failed to parse discord access token into header value".into()),
                )
            })?,
    );
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| unexpected_err(e, Some("Unable to get discord user".into())))?;

    let body = res
        .json::<serde_json::Value>()
        .await
        .map_err(|e| conversion_err(e, Some("Unable to get discord user".into())))?;
    let body = body
        .as_object()
        .expect_or_err("Body is not an object")
        .map_err(|e| conversion_err(e, Some("Unable to convert body to object".into())))?;
    let app_id = body
        .get("application")
        .expect_or_err("Couldn't not get application object from discord api result")
        .map_err(|e| parser_err(e, None))?
        .as_object()
        .expect_or_err("Couldn't convert application object to object")
        .map_err(|e| conversion_err(e, None))?
        .get("id")
        .expect_or_err("Couldn't get application id")
        .map_err(|e| parser_err(e, None))?
        .as_str()
        .expect_or_err("Couldn't convert application id to string")
        .map_err(|e| conversion_err(e, None))?;
    let discord_id = body
        .get("user")
        .expect_or_err("Couldn't get user object from discord api result")
        .map_err(|e| parser_err(e, None))?
        .as_object()
        .expect_or_err("Couldn't convert user object to object")
        .map_err(|e| conversion_err(e, None))?
        .get("id")
        .expect_or_err("Couldn't get user id")
        .map_err(|e| parser_err(e, None))?
        .as_str()
        .expect_or_err("Couldn't convert user id to string")
        .map_err(|e| conversion_err(e, None))?;

    let exp = chrono::Utc::now()
        .checked_add_days(Days::new(7))
        .expect_or_err("Could not generate expiration time for auth method")?
        .timestamp();

    Ok(models::AuthMethodResponse {
        user_id: discord_id.to_string(),
        app_id: app_id.to_string(),
        auth_method_type: DISCORD_AUTH_METHOD_TYPE_ID,
        last_retrieved_at: SystemTime::now(),
        expiration: exp,
        used_for_sign_session_key_request: false,
    })
}
