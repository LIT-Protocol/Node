use std::borrow::Cow;
use std::fmt;

use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

use derive_more::Display;

use lit_core_derive::{Description, ErrorCode};

use crate::error::Kind;
use crate::types::Description;

pub type ArcCode = Arc<dyn Code + Send + Sync>;

pub trait Code: Display + Debug + Description {
    fn code(&self) -> Cow<str>;
    fn kind(&self) -> Option<Kind>;
    fn http_status(&self) -> Option<u16>;
}

#[allow(dead_code)]
#[derive(Clone, Debug, Display, ErrorCode, Description)]
pub(crate) enum EC {
    /// A fatal error occured in the lit core system
    #[code(kind = Unexpected, http_status = 500)]
    CoreFatal,
    /// An unexpected internal server error occured.
    #[code(kind = Unexpected, http_status = 500)]
    CoreUnexpected,
}

/// Used mostly to reconstruct an error from a prior state (i.e. a PublicError from an API).
pub struct StaticCode {
    code: String,
    kind: Option<Kind>,
    http_status: Option<u16>,
    description: Option<String>,
}

impl StaticCode {
    pub fn new(
        code: String, kind: Option<Kind>, http_status: Option<u16>, description: Option<String>,
    ) -> Self {
        Self { code, kind, http_status, description }
    }
}

impl Display for StaticCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.code)?;
        Ok(())
    }
}

impl Debug for StaticCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.code)?;
        Ok(())
    }
}

impl Description for StaticCode {
    fn description(&self) -> Option<String> {
        self.description.clone()
    }
}

impl Code for StaticCode {
    fn code(&self) -> Cow<str> {
        Cow::from(self.code.clone())
    }

    fn kind(&self) -> Option<Kind> {
        self.kind.clone()
    }

    fn http_status(&self) -> Option<u16> {
        self.http_status
    }
}
