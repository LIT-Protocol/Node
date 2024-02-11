use crate::error::{Caller, Error};
use crate::utils::debug::unescape_debug_output;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

/// NB: This is intended for structured logging and not to send as a HTTP response object.

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut len = 2;
        if self.inner.code.is_some() {
            len += 1
        };
        if self.inner.msg.is_some() {
            len += 1
        };
        if self.inner.source.is_some() {
            len += 1
        };
        if self.inner.caller.is_some() {
            len += 1
        };

        let mut state = serializer.serialize_struct("Error", len)?;
        state.serialize_field("pkg", &self.inner.pkg)?;
        state.serialize_field("kind", &self.inner.kind)?;

        if let Some(code) = &self.inner.code {
            state.serialize_field("code", code.code().as_ref())?;
        }

        if let Some(ref msg) = self.inner.msg {
            state.serialize_field("msg", msg)?;
        }

        if let Some(ref source) = self.inner.source {
            if let Some(source) = source.downcast_ref::<Error>() {
                state.serialize_field("source", source)?;
            } else {
                state.serialize_field("source", &unescape_debug_output(format!("{source:?}")))?;
            }
        }

        if let Some(ref caller) = self.inner.caller {
            state.serialize_field("caller", caller)?;
        }

        state.end()
    }
}

impl Serialize for Caller {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Caller", 1)?;
        state.serialize_field("file", &format!("{}:{}:{}", &self.file, &self.line, &self.col))?;

        state.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{err_code, generic_err, sev_snp_err, EC};

    #[test]
    fn error_to_json_test() {
        let err = generic_err("first", None);
        let err = sev_snp_err(err, Some("sev-snp".into()));
        let err = err_code(err, EC::CoreFatal, Some("fatal-1".into()));

        let json = serde_json::to_string(&err).unwrap();

        assert_eq!(json, "{\"pkg\":\"lit_core\",\"kind\":\"Unexpected\",\"code\":\"CoreFatal\",\"msg\":\"fatal-1\",\"source\":{\"pkg\":\"lit_core\",\"kind\":\"SevSnp\",\"msg\":\"sev-snp\",\"source\":{\"pkg\":\"lit_core\",\"kind\":\"Generic\",\"source\":\"first\",\"caller\":{\"file\":\"lit-core/src/error/serializer.rs:73:19\"}},\"caller\":{\"file\":\"lit-core/src/error/serializer.rs:74:19\"}},\"caller\":{\"file\":\"lit-core/src/error/serializer.rs:75:19\"}}");
    }
}
