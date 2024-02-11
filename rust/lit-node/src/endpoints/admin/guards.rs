use crate::auth::auth_material::JsonAuthSig;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::serde::json::{serde_json::json, Value};

pub struct AdminAuthSig {
    pub auth_sig: JsonAuthSig,
}

/// The AdminAuthSig request guard is used to check for the existence of the x-auth-sig header.
/// If it is not present, the request is rejected as unauthorized.
/// If it is present, it is decoded from base64, deserialized as JSON and returned as a JsonAuthSig struct.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminAuthSig {
    type Error = Value;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_sig = match request.headers().get_one("x-auth-sig") {
            Some(auth_sig) => auth_sig,
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    json!({"error": "Missing x-auth-sig header"}),
                ));
            }
        };

        // Decode base64.
        let decoded_auth_sig = match data_encoding::BASE64.decode(auth_sig.as_bytes()) {
            Ok(decoded_auth_sig) => decoded_auth_sig,
            Err(e) => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    json!({"error": "Unable to decode base64", "reason": e.to_string()}),
                ));
            }
        };

        // Deserialize JSON.
        let deserialized_auth_sig = match serde_json::from_slice::<JsonAuthSig>(&decoded_auth_sig) {
            Ok(deserialized_auth_sig) => deserialized_auth_sig,
            Err(e) => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    json!({"error": "Unable to deserialize JSON", "reason": e.to_string()}),
                ));
            }
        };

        return Outcome::Success(AdminAuthSig {
            auth_sig: deserialized_auth_sig,
        });
    }
}
