use log::as_error;
use rocket::http::Status;
use rocket::response::status;
use serde_json::{json, Value};

use lit_core::error::PublicError;

use crate::context::TRACING;
use crate::error::Error;

pub trait ApiError {
    fn handle(self) -> status::Custom<Value>;
    fn handle_with_details(self, details: impl AsRef<str>) -> status::Custom<Value>;
    fn handle_with_logs(self, logs: impl AsRef<str>) -> status::Custom<Value>;
}

impl ApiError for Error {
    fn handle(self) -> status::Custom<Value> {
        handle_err(self)
    }

    fn handle_with_details(self, details: impl AsRef<str>) -> status::Custom<Value> {
        handle_err_with_details(self, Some(details.as_ref().to_string()))
    }

    fn handle_with_logs(self, logs: impl AsRef<str>) -> status::Custom<Value> {
        let mut status = handle_err(self);
        status.1["logs"] = logs.as_ref().into();
        status
    }
}

pub fn handle_err(err: Error) -> status::Custom<Value> {
    handle_err_with_details(err, None)
}

pub fn handle_err_with_details(err: Error, details: Option<String>) -> status::Custom<Value> {
    error!(error = as_error!(err); "Handling API error");

    let public = err_to_public_error(err.add_source_to_details(), details);
    let status = Status::from_code(public.status()).unwrap_or(Status::InternalServerError);

    match public.to_json() {
        Ok(json) => status::Custom(status, json),
        Err(e) => {
            error!(error = as_error!(e); "Failed to serialize error to json");

            let def_public = PublicError::default();
            match def_public.to_json() {
                Ok(json) => status::Custom(status, json),
                Err(_e) => {
                    // Also failed, but a double log would be a bit stupid.
                    status::Custom(
                        status,
                        json!({
                            "errorKind": def_public.error_kind(),
                            "message": def_public.message().unwrap()
                        }),
                    )
                }
            }
        }
    }
}

pub fn err_to_public_error(err: Error, details: Option<String>) -> PublicError {
    let mut public: PublicError = err.into();
    // Ignore error (as it's optional to exist).
    let mut correlation_id: Option<String> = None;
    let _ = TRACING.try_with(|v| {
        let _ = correlation_id.insert(v.correlation_id().clone());
    });
    if let Some(correlation_id) = correlation_id {
        public = public.with_correlation_id(correlation_id);
    }
    if let Some(details) = details {
        public = public.add_detail(details);
    }

    public
}
