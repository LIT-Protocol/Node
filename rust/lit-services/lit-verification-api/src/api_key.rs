use log::trace;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use serde_json::{json, Value};

use crate::constants::API_HEADER_KEY;
use crate::utils::parse_api_key;

pub struct ApiKey {
    pub key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = Value;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one(API_HEADER_KEY) {
            Some(header) => match parse_api_key(&header.to_string()) {
                Ok(_) => {
                    trace!("Validated api key: {}", header.to_string());
                    return Outcome::Success(ApiKey { key: header.to_string() });
                }
                Err(_) => {
                    trace!("Error parsing api key found, aborting request");

                    return Outcome::Failure((
                        Status::Unauthorized,
                        json!({"error": "Invalid api-key"}),
                    ));
                }
            },
            None => {
                trace!("No api key found, aborting request");

                return Outcome::Failure((
                    Status::Unauthorized,
                    json!({"error": "Missing API key header"}),
                ));
            }
        }
    }
}
