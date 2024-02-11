pub const PKG_NAME: &str = "lit_logging_service";

use derive_more::Display;

pub use lit_core::error::*;
use lit_core::generate_pkg_constructors;
use lit_core_derive::{Description, ErrorCode};

#[derive(Debug, Display, Description, ErrorCode)]
#[allow(dead_code, clippy::enum_variant_names)]
pub(crate) enum EC {
    /// An unexpected fault in the logging service has occured.
    #[code(kind = Unexpected, http_status = 500)]
    LoggingServiceFault,
}

generate_pkg_constructors!(PKG_NAME, pub(crate), EC);
