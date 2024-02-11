use std::fmt::Debug;

use derive_more::Display;

pub use lit_core::error::*;
use lit_core::generate_pkg_constructors;
use lit_core_derive::{Description, ErrorCode};

pub const PKG_NAME: &str = "lit_logging";

#[derive(Debug, Display, Description, ErrorCode)]
#[allow(dead_code, clippy::enum_variant_names)]
pub(crate) enum EC {
    /// A fault error occured in the logging system
    #[code(kind = Unexpected, http_status = 500)]
    LoggingFault,
}

generate_pkg_constructors!(PKG_NAME, pub(crate), EC);
