use crate::jwx::models::{Jwt, VerificationRequest, VerificationResponse};
use crate::jwx::verify::verify_token_from_str;
use crate::limit::RateLimitGuard;
use lit_api_core::context::{with_context, Tracing};
use lit_api_core::error::{ApiError, Unexpected};
use lit_core::error::Result;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_governor::RocketGovernor;
use serde_json::{json, Value};
use std::str::{self};

#[post("/api/otp/verify", format = "json", data = "<input>")]
pub(crate) async fn verify_jwt(
    _limitguard: RocketGovernor<'_, RateLimitGuard>, key_shares: &State<crate::ecdsa::SigningPair>,
    input: Json<VerificationRequest>, tracing: Tracing,
) -> status::Custom<Value> {
    with_context(tracing, async move {
        let token = &input.token;
        match verify_token_from_str(token.clone(), key_shares) {
            Ok(jwt) => match get_message_from_token(jwt) {
                Ok(user_id) => {
                    #[allow(clippy::needless_return)]
                    return status::Custom(
                        Status::Accepted,
                        json!(VerificationResponse { status: true, user_id }),
                    );
                }
                Err(e) => {
                    #[allow(clippy::needless_return)]
                    return e.handle();
                }
            },
            Err(e) => {
                #[allow(clippy::needless_return)]
                return e.handle();
            }
        };
    })
    .await
}

fn get_message_from_token(token: Jwt) -> Result<String> {
    let parts: Vec<&str> = token.payload.extra_data.splitn(2, '|').collect();
    let user_id = parts.first().expect_or_err("message in token not valid missing user id")?;

    Ok(user_id.to_string())
}
