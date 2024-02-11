use hyper::{Body, Response, StatusCode};
use log::{as_error, error};
use serde_json::json;

use crate::error::{err_to_public_error, Error};

pub trait ApiError {
    fn handle(self) -> Response<Body>;
    fn handle_with_details(self, details: String) -> Response<Body>;
}

impl ApiError for Error {
    fn handle(self) -> Response<Body> {
        handle_err(self)
    }

    fn handle_with_details(self, details: String) -> Response<Body> {
        handle_err_with_details(self, Some(details))
    }
}

pub fn handle_err(err: Error) -> Response<Body> {
    handle_err_with_details(err, None)
}

pub fn handle_err_with_details(err: Error, details: Option<String>) -> Response<Body> {
    error!(error = as_error!(err); "Handling API error");

    let public = err_to_public_error(err, details);
    let status = StatusCode::from_u16(public.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let public_json = public.to_json();
    match public_json {
        Ok(json) => {
            let err_resp = Response::builder().status(status).body(Body::from(json.to_string()));
            if let Ok(resp) = err_resp {
                return resp;
            }
        }
        Err(e) => error!(error = as_error!(e); "Failed to serialize error to json"),
    }

    // For cases where the above fails to serialize.
    Response::builder()
        .status(status)
        .body(Body::from(
            json!({
                "errorKind": public.error_kind(),
                "message": public.message().unwrap(),
            })
            .to_string(),
        ))
        .expect("Unable to build default error response. This should never occur.")
    // This should never panic.
}
