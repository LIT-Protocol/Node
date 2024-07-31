use std::sync::Arc;

use serde::{de::Deserializer, Deserialize};
use serde_json::Value;

use crate::error::{ArcCode, BoxError, Caller, Error, Kind, StaticCode};

impl<'de> Deserialize<'de> for Error {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct EncodedError {
            pkg: String,
            kind: Kind,
            code: Option<String>,
            msg: Option<String>,
            source: Value,
            caller: Option<Caller>,
        }

        Deserialize::deserialize(deserializer).map(|e: EncodedError| {
            Error::new(
                Some(e.kind),
                e.pkg,
                e.msg,
                e.code.map(|code| Arc::new(StaticCode::new(code, None, None, None)) as ArcCode),
                // Source can be an Error, a String, or None
                match serde_json::from_value::<Error>(e.source.clone()) {
                    Ok(err) => Some(BoxError::from(err)),
                    _ => e.source.as_str().map(BoxError::from),
                },
                None,
            )
            .set_caller(e.caller) // Set caller after the fact since we can't contruct a Location
        })
    }
}

impl<'de> Deserialize<'de> for Caller {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct EncodedCaller {
            file: String,
        }

        Deserialize::deserialize(deserializer).map(|c: EncodedCaller| {
            let mut parts = c.file.splitn(3, ':');
            let file = parts.next().unwrap_or_default().to_string();
            let line = parts.next().unwrap_or_default().parse().unwrap_or_default();
            let col = parts.next().unwrap_or_default().parse().unwrap_or_default();

            Caller { file, line, col }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_from_json_test() {
        let json = "{\"pkg\":\"lit_core\",\"kind\":\"Unexpected\",\"code\":\"CoreFatal\",\"msg\":\"fatal-1\",\"source\":{\"pkg\":\"lit_core\",\"kind\":\"SevSnp\",\"msg\":\"sev-snp\",\"source\":{\"pkg\":\"lit_core\",\"kind\":\"Generic\",\"source\":\"first\",\"caller\":{\"file\":\"lit-core/src/error/serializer.rs:73:19\"}},\"caller\":{\"file\":\"lit-core/src/error/serializer.rs:74:19\"}},\"caller\":{\"file\":\"lit-core/src/error/serializer.rs:75:19\"}}";
        let err: Error = serde_json::from_str(json).unwrap();

        assert_eq!(err.pkg(), "lit_core");
        assert_eq!(err.kind(), Kind::Unexpected);
        assert_eq!(err.code().unwrap().code(), "CoreFatal");
        assert_eq!(err.msg(), Some(&"fatal-1".to_string()));

        // Use debug format to check private properties
        assert_eq!(
            format!("{err:?}"),
            r#"lit_core::Error { kind: Unexpected, code: CoreFatal, msg: "fatal-1", source: lit_core::Error { kind: SevSnp, msg: "sev-snp", source: lit_core::Error { kind: Generic, source: "first", caller:  { file: "lit-core/src/error/serializer.rs:73:19" } }, caller:  { file: "lit-core/src/error/serializer.rs:74:19" } }, caller:  { file: "lit-core/src/error/serializer.rs:75:19" } }"#
        );
    }
}
