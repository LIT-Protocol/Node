use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::sync::Arc;

use crate::error::{serializer_err, ArcCode, Error, Kind, Result, StaticCode, PKG_NAME};
use crate::types::Description;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicError {
    error_kind: Kind,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<String>,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    details: Vec<String>,
}

impl PublicError {
    pub fn with_correlation_id<S>(mut self, correlation_id: S) -> Self
    where
        S: Into<String>,
    {
        self.correlation_id = Some(correlation_id.into());
        self
    }

    pub fn add_detail<S>(mut self, details: S) -> Self
    where
        S: Into<String>,
    {
        self.details.insert(0, details.into());
        self
    }

    pub fn clear_details(mut self) -> Self {
        self.details.clear();
        self
    }

    pub fn to_json(&self) -> Result<Value> {
        serde_json::to_value(self)
            .map_err(|e| serializer_err(e, Some("failed to serialize PublicError".into())))
    }

    // Accessors
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn error_kind(&self) -> &Kind {
        &self.error_kind
    }

    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }
}

impl Default for PublicError {
    fn default() -> Self {
        Self {
            error_kind: Kind::Unexpected,
            error_code: None,
            status: 500,
            message: Some("An unexpected internal server error occured.".into()),
            correlation_id: None,
            details: Vec::new(),
        }
    }
}

impl fmt::Debug for PublicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct(format!("{PKG_NAME}::PublicError").as_str());

        builder.field("error_kind", &self.error_kind);

        if let Some(error_code) = &self.error_code {
            builder.field("error_code", error_code);
        }

        if let Some(message) = &self.message {
            builder.field("message", message);
        }

        if !self.details.is_empty() {
            builder.field("details", &self.details);
        }

        if let Some(correlation_id) = &self.correlation_id {
            builder.field("correlation_id", correlation_id);
        }

        builder.finish()
    }
}

impl From<Error> for PublicError {
    fn from(val: Error) -> Self {
        let real = val.concrete(true);

        let mut code: Option<String> = None;
        let mut status: Option<u16> = None;
        let mut message: Option<String> = None;
        if let Some(c) = real.code() {
            code = Some(c.code().to_string());
            status = c.http_status();
            message = c.description();
        }

        // Defaults
        if status.is_none() {
            status = Some(500);
        }
        if message.is_none() {
            message = real.kind().description();
        }

        // Extract details
        let mut details: Vec<String> = Vec::new();
        val.walk({
            let details = &mut details;
            move |c| {
                details.extend(c.inner.details.clone());

                false
            }
        });

        PublicError {
            error_kind: real.kind(),
            error_code: code,
            status: status.unwrap(),
            message,
            correlation_id: None,
            details,
        }
    }
}

impl From<PublicError> for Error {
    fn from(val: PublicError) -> Error {
        let mut code: Option<ArcCode> = None;
        if let Some(error_code) = val.error_code.as_ref() {
            code = Some(Arc::new(StaticCode::new(
                error_code.clone(),
                Some(val.error_kind.clone()),
                Some(val.status),
                val.message.clone(),
            )))
        }

        Error::new(
            Some(val.error_kind.clone()),
            "upstream",
            None,
            code,
            Some(format!("{:?}", &val)),
            None,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::error::*;

    #[test]
    fn error_to_public_test() {
        let err = sev_snp_err_code("real error", EC::CoreFatal, Some("message".into()));

        let public: PublicError = err.into();

        assert_eq!(public.error_kind, Kind::SevSnp);
        assert_eq!(public.error_code, Some("CoreFatal".into()));
        assert_eq!(public.status, 500);
        assert_eq!(public.message, Some("A fatal error occured in the lit core system".into()));
        assert_eq!(public.correlation_id, None);
        assert_eq!(public.details, Vec::<String>::new());

        let public = public.with_correlation_id("1234").add_detail("Something");

        assert_eq!(public.correlation_id, Some("1234".into()));
        assert_eq!(public.details, vec!["Something".to_string()]);
    }

    #[test]
    fn public_to_json_test() {
        let err = sev_snp_err_code("real error", EC::CoreFatal, Some("message".into()))
            .add_detail("Some juicy details")
            .add_detail("Some more");

        let public: PublicError = err.into();

        let json = public.to_json().unwrap().to_string();

        assert_eq!(json, "{\"details\":[\"Some juicy details\",\"Some more\"],\"errorCode\":\"CoreFatal\",\"errorKind\":\"SevSnp\",\"message\":\"A fatal error occured in the lit core system\",\"status\":500}");
    }

    #[test]
    fn public_add_detail_test() {
        let err = generic_err("first", None).add_detail("detail 1");
        let err = sev_snp_err(err, Some("sev-snp".into())).add_detail("detail 2");
        let err = err_code(err, EC::CoreFatal, Some("fatal-1".into())).add_detail("detail 3");
        let err = generic_err(err, Some("middle".into())).add_detail("detail 4");
        let err = err_code(err, EC::CoreFatal, Some("fatal-2".into()));
        let err = generic_err(err, Some("last".into()));

        let public = Into::<PublicError>::into(err).add_detail("final detail");

        assert_eq!(
            public.details,
            vec![
                "final detail".to_string(),
                "detail 4".to_string(),
                "detail 3".to_string(),
                "detail 2".to_string(),
                "detail 1".to_string()
            ]
        )
    }

    #[test]
    fn public_to_error_test() {
        let err = sev_snp_err_code("real error", EC::CoreFatal, Some("message".into()))
            .add_detail("Some juicy details")
            .add_detail("Some more");

        let public: PublicError = err.into();

        let new_err: Error = public.into();

        assert_eq!(format!("{:?}", new_err), "upstream::Error { kind: SevSnp, code: CoreFatal, source: \"lit_core::PublicError { error_kind: SevSnp, error_code: \\\"CoreFatal\\\", message: \\\"A fatal error occured in the lit core system\\\", details: [\\\"Some juicy details\\\", \\\"Some more\\\"] }\" }");
    }
}
