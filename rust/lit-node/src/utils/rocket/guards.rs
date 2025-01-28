use rocket::http::HeaderMap;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Value;

pub struct RequestHeaders<'r> {
    pub headers: HeaderMap<'r>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders<'r> {
    type Error = Value;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RequestHeaders {
            headers: request.headers().clone(),
        })
    }
}
