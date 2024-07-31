#![cfg_attr(target_arch = "wasm32", allow(unused))]

pub use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::Into;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::Debug;
use std::io;
pub use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub use code::*;
use lit_core_derive::Description;
pub use public::*;
pub use unexpected::*;

pub const PKG_NAME: &str = "lit_core";

pub mod code;
pub mod deserializer;
pub mod public;
pub mod serializer;
pub mod unexpected;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct Error {
    inner: Box<Inner>,
}

pub type BoxError = Box<dyn StdError + Send + Sync>;
pub type ArcBoxError = Arc<BoxError>;

#[derive(Clone)]
struct Inner {
    kind: Kind,
    pkg: String,
    msg: Option<String>,
    code: Option<ArcCode>,
    source: Option<ArcBoxError>,
    caller: Option<Caller>,
    details: Vec<String>,
    fields: Option<HashMap<String, Value>>,
}

impl Error {
    pub fn new<E, S>(
        kind: Option<Kind>, pkg: S, msg: Option<String>, code: Option<ArcCode>, source: Option<E>,
        caller: Option<&Location>,
    ) -> Self
    where
        E: Into<BoxError>,
        S: Into<String>,
    {
        // Formalise
        let pkg = pkg.into();

        // The Kind in the code is more of a default.
        let kind = kind.unwrap_or_else(|| {
            if let Some(code) = code.as_ref() {
                code.kind().unwrap_or(Kind::Unknown)
            } else {
                Kind::Unknown
            }
        });

        // Map the actual error
        let mut arc_source: Option<ArcBoxError> = None;
        if let Some(source) = source {
            arc_source = Some(Arc::new(source.into()));
        }

        // Extract caller information
        let our_caller = caller.map(Caller::from);

        Self {
            inner: Box::new(Inner {
                kind,
                pkg,
                msg,
                code,
                source: arc_source,
                caller: our_caller,
                details: Vec::new(),
                fields: None,
            }),
        }
    }

    /// Add some public details (sent to the client).
    pub fn add_detail<S>(mut self, detail: S) -> Self
    where
        S: Into<String>,
    {
        self.inner.details.push(detail.into());
        self
    }

    /// Adds the error msg as a public detail
    pub fn add_msg_to_details(mut self) -> Self {
        if let Some(msg) = self.inner.msg.as_ref() {
            self.inner.details.push(msg.clone())
        }

        self
    }

    /// Adds the error source as a public detail (useful if your source was a String)
    pub fn add_source_to_details(mut self) -> Self {
        if let Some(source) = self.inner.source.as_ref() {
            self.inner.details.push(source.to_string())
        }

        self
    }

    /// Clear all of the public details.
    pub fn clear_details(mut self) -> Self {
        self.inner.details.clear();
        self
    }

    /// Add some fields to the structured logs.
    pub fn add_field<K>(mut self, key: K, value: Value) -> Self
    where
        K: Into<String>,
    {
        let mut fields = match self.inner.fields.take() {
            Some(v) => v,
            None => HashMap::new(),
        };

        fields.insert(key.into(), value);
        self.inner.fields = Some(fields);
        self
    }

    /// Apply the fields (recursively) to the provided map).
    pub fn apply_fields(&self, fields: &mut Map<String, Value>) {
        self.walk({
            //let fields = &mut fields;
            move |c| {
                if let Some(cur_fields) = c.inner.fields.as_ref() {
                    for (k, v) in cur_fields.iter() {
                        fields.insert(k.clone(), v.clone());
                    }
                }
                false
            }
        });
    }

    fn set_caller(mut self, caller: Option<Caller>) -> Self {
        self.inner.caller = caller;
        self
    }

    #[allow(unused)]
    pub fn into_io(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self)
    }

    pub fn concrete(&self, use_first: bool) -> Self {
        let mut found = Some(self.clone());
        self.walk({
            let found = &mut found;
            move |c| {
                if c.inner.code.is_some() {
                    let _ = found.insert(c.clone());
                    !use_first
                } else {
                    false
                }
            }
        });

        found.unwrap()
    }

    pub fn walk<F>(&self, mut cb: F)
    where
        F: FnMut(&Self) -> bool,
    {
        if cb(self) {
            return;
        }

        if let Some(source) = self.source() {
            if let Some(source) = source.downcast_ref::<Error>() {
                Self::walk(source, cb);
            }
        }
    }

    // Accessors
    pub fn kind(&self) -> Kind {
        self.inner.kind.clone()
    }

    pub fn pkg(&self) -> &String {
        &self.inner.pkg
    }

    pub fn msg(&self) -> Option<&String> {
        self.inner.msg.as_ref()
    }

    pub fn code(&self) -> Option<ArcCode> {
        self.inner.code.clone()
    }

    pub fn is_kind(&self, kind: Kind, recursive: bool) -> bool {
        let mut found = Some(false);
        self.walk({
            let found = &mut found;
            move |c| {
                if c.kind() == kind {
                    let _ = found.insert(true);
                    true
                } else {
                    !recursive
                }
            }
        });

        found.unwrap()
    }

    pub fn is_code(&self, code: impl Code + 'static, recursive: bool) -> bool {
        let mut found = Some(false);
        self.walk({
            let found = &mut found;
            move |c| {
                if c.code().map(|cc| cc.code().eq(code.code().as_ref())).unwrap_or_else(|| false) {
                    let _ = found.insert(true);
                    true
                } else {
                    !recursive
                }
            }
        });

        found.unwrap()
    }

    pub fn has_code(&self) -> bool {
        let mut found = Some(false);
        self.walk({
            let found = &mut found;
            move |c| {
                if c.code().is_some() {
                    let _ = found.insert(true);
                    true
                } else {
                    false
                }
            }
        });

        found.unwrap()
    }

    pub fn as_error(&self) -> &(dyn StdError + 'static) {
        self
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct(format!("{}::Error", self.inner.pkg).as_str());

        builder.field("kind", &self.inner.kind);

        if let Some(code) = &self.inner.code {
            builder.field("code", code);
        }

        if let Some(ref msg) = self.inner.msg {
            builder.field("msg", msg);
        }

        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }

        if let Some(ref caller) = self.inner.caller {
            builder.field("caller", caller);
        }

        if let Some(ref fields) = self.inner.fields {
            builder.field("fields", fields);
        }

        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner.kind {
            Kind::Unknown => f.write_str("error")?,
            Kind::Unexpected => f.write_str("unexpected error")?,
            Kind::Generic => f.write_str("generic error")?,
            Kind::IPFS => f.write_str("IPFS error")?,
            Kind::Io => f.write_str("io error")?,
            Kind::Config => f.write_str("config error")?,
            Kind::Serializer => f.write_str("serializer error")?,
            Kind::Validation => f.write_str("validation error")?,
            Kind::Conversion => f.write_str("conversion error")?,
            Kind::Parser => f.write_str("parser error")?,
            Kind::Lock => f.write_str("lock error")?,
            Kind::Timeout => f.write_str("timeout error")?,
            Kind::Connect => f.write_str("connection error")?,
            Kind::MemoryLimit => f.write_str("memory limit exceeded")?,
            Kind::Blockchain => f.write_str("blockchain error")?,
            Kind::Attestation => f.write_str("attestation error")?,
            Kind::Certs => f.write_str("certificate error")?,
            Kind::HttpClient => f.write_str("HTTP client error")?,
            Kind::SevSnp => f.write_str("AMD SEV-SNP error")?,
        };

        if let Some(msg) = &self.inner.msg {
            write!(f, ": {msg}")?;
        }

        if let Some(e) = &self.inner.source {
            write!(f, ": {e}")?;
        }

        if let Some(ref fields) = self.inner.fields {
            write!(f, " (")?;
            let mut first = true;
            for (key, val) in fields {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;
                write!(f, "{}: {}", key, val)?;
            }
            write!(f, ")")?;
        }

        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &***e as _)
    }
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
struct Caller {
    file: String,
    line: u32,
    col: u32,
}

impl fmt::Debug for Caller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("") // Empty to take up less space.
            .field(
                "file",
                &format!("{}:{}:{}", &self.file, &self.line, &self.col),
            )
            .finish()
    }
}

impl From<&Location<'_>> for Caller {
    fn from(src: &Location) -> Self {
        Self { file: src.file().to_string(), line: src.line(), col: src.column() }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Description, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Kind {
    /// An error has occured.
    Unknown,
    /// An unexpected has occured.
    Unexpected,
    /// An error has occured.
    Generic,
    /// An IPFS related error has occured.
    IPFS,
    /// An IO related error has occured.
    Io,
    /// An error related to configuration has occured.
    Config,
    /// An error related to validation has occured.
    Validation,
    /// An error related to type conversion has occured.
    Conversion,
    /// An error related to a parsing has occured.
    Parser,
    /// An error related to a serialization has occured.
    Serializer,
    /// An error related to a lock has occured.
    Lock,
    /// An timeout error has occured.
    Timeout,
    /// A connection error has occured.
    Connect,
    /// A memory limit was exceeded.
    MemoryLimit,
    /// A blockchan related error has occured.
    Blockchain,
    /// A TEE attestation related error has occured.
    Attestation,
    /// An error related to a certificate has occured.
    Certs,
    /// An error related to HTTP/HTTPS client connections.
    HttpClient,
    /// An error related to a the AMD SEV-SNP subsystems has occured.
    SevSnp,
}

// constructors

#[macro_export]
macro_rules! generate_pkg_constructors {
    ($(#[$attr:meta])* $pkg:expr) => (
        generate_pkg_constructors!($pkg, pub(crate));
    );
    ($(#[$attr:meta])* $pkg:expr, $vis:vis) => (
        generate_pkg_constructors!($pkg, $vis, impl Code + Send + Sync + 'static);
    );
    ($(#[$attr:meta])* $pkg:expr, $vis:vis, $code:ty) => (
        use std::panic::Location;

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn err_pkg_name() -> String {
            $pkg.into()
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(None, $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn generic_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Generic), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn generic_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Generic), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn unexpected_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Unexpected), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn unexpected_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Unexpected), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn ipfs_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::IPFS), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn ipfs_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::IPFS), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn io_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Io), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn io_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Io), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn config_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Config), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn config_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Config), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn validation_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Validation), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn validation_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Validation), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn conversion_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Conversion), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn conversion_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Conversion), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn parser_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Parser), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn parser_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Parser), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn serializer_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Serializer), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn serializer_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Serializer), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn lock_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Lock), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn lock_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Lock), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn timeout_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Timeout), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn timeout_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Timeout), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn memory_limit_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::MemoryLimit), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn memory_limit_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::MemoryLimit), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn blockchain_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Blockchain), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn blockchain_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Blockchain), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn attestation_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Attestation), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn attestation_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Attestation), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn certs_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Certs), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn certs_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Certs), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn http_client_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::HttpClient), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn http_client_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::HttpClient), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn connect_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Connect), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn connect_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::Connect), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn sev_snp_err<E: Into<BoxError>>(e: E, msg: Option<String>) -> Error {
            Error::new(Some(Kind::SevSnp), $pkg, msg, None, Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn sev_snp_err_code<E: Into<BoxError>>(e: E, code: $code, msg: Option<String>) -> Error {
            Error::new(Some(Kind::SevSnp), $pkg, msg, Some(Arc::new(code)), Some(e), Some(Location::caller()))
        }

        $(#[$attr])*
        #[allow(dead_code)]
        #[track_caller]
        #[inline]
        $vis fn map_io_err<E: Into<BoxError>>(e: E) -> Error {
            io_err(e, None)
        }
    )
}

generate_pkg_constructors!(PKG_NAME);

#[cfg(test)]
mod tests {
    use crate::error::Code;
    use crate::error::{err_code, generic_err, sev_snp_err, Kind, EC};
    use crate::types::description::Description;

    #[test]
    fn error_to_display_test() {
        let err = generic_err("first", None);
        let err = sev_snp_err(err, Some("sev-snp".into()));

        let err = format!("{}", err);

        assert_eq!(err, "AMD SEV-SNP error: sev-snp: generic error: first");
    }

    #[test]
    fn error_to_debug_test() {
        let err = generic_err("first", None);
        let err = sev_snp_err(err, Some("sev-snp".into()));
        let err = err_code(err, EC::CoreFatal, Some("fatal-1".into()));

        let err = format!("{:?}", err);

        assert_eq!(err, "lit_core::Error { kind: Unexpected, code: CoreFatal, msg: \"fatal-1\", source: lit_core::Error { kind: SevSnp, msg: \"sev-snp\", source: lit_core::Error { kind: Generic, source: \"first\", caller:  { file: \"lit-core/src/error/mod.rs:772:19\" } }, caller:  { file: \"lit-core/src/error/mod.rs:773:19\" } }, caller:  { file: \"lit-core/src/error/mod.rs:774:19\" } }");
    }

    #[test]
    fn ec_description_test() {
        let code = EC::CoreFatal;

        assert_eq!(code.description(), Some("A fatal error occured in the lit core system".into()));
        assert_eq!(code.kind(), Some(Kind::Unexpected));
        assert_eq!(code.http_status(), Some(500));
    }

    #[test]
    fn error_concrete_test() {
        let err = generic_err("first", None);
        let err = sev_snp_err(err, Some("sev-snp".into()));
        let err = err_code(err, EC::CoreFatal, Some("fatal-1".into()));
        let err = generic_err(err, Some("middle".into()));
        let err = err_code(err, EC::CoreFatal, Some("fatal-2".into()));
        let err = generic_err(err, Some("last".into()));

        let first_coded = err.concrete(true);

        assert_eq!(first_coded.inner.msg, Some("fatal-1".into()));

        let last_coded = err.concrete(false);

        assert_eq!(last_coded.inner.msg, Some("fatal-2".into()));
    }

    #[test]
    fn is_kind_test() {
        let err = generic_err("first", None);
        let err = sev_snp_err(err, Some("sev-snp".into()));
        let err = generic_err(err, Some("last".into()));

        assert_eq!(err.is_kind(Kind::Generic, false), true);
        assert_eq!(err.is_kind(Kind::SevSnp, false), false);
        assert_eq!(err.is_kind(Kind::SevSnp, true), true);
    }

    #[test]
    fn is_code_test() {
        let err = generic_err("first", None);
        let err = err_code(err, EC::CoreFatal, Some("fatal-1".into()));
        let err = sev_snp_err(err, Some("sev-snp".into()));

        assert_eq!(err.is_code(EC::CoreFatal, false), false);
        assert_eq!(err.is_code(EC::CoreFatal, true), true);
    }

    #[test]
    fn has_code_test() {
        let err = generic_err("first", None);
        let err = generic_err(err, Some("second".into()));

        assert_eq!(err.has_code(), false);

        let err = err_code(err, EC::CoreFatal, Some("fatal-1".into()));
        let err = sev_snp_err(err, Some("sev-snp".into()));

        assert_eq!(err.has_code(), true);
    }
}
