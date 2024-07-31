use derive_more::Display;
#[cfg(feature = "server")]
pub use handler::*;
pub use lit_core::error::*;
use lit_core::generate_pkg_constructors;
use lit_core_derive::{Description, ErrorCode};

#[cfg(feature = "server")]
pub mod handler;

pub const PKG_NAME: &str = "lit_api_core";

#[derive(Debug, Display, Description, ErrorCode)]
#[allow(dead_code, clippy::enum_variant_names)]
pub(crate) enum EC {
    /// An unexpected error has occured
    #[code(kind = Unexpected, http_status = 400)]
    CoreApiUnexpected,
    /// The API path was not found.
    #[code(kind = Validation, http_status = 404)]
    CoreApiRouteNotFound,
    /// Missing required header: X-Correlation-Id / X-Request-Id
    #[code(kind = Validation, http_status = 400)]
    CoreApiMissingRequiredXRequestIdHeader,
    /// Missing required header: X-Lit-Sdk-Version
    #[code(kind = Validation, http_status = 400)]
    CoreApiInvalidXSDKVersionHeader,
    /// The HTTP client has timed out
    #[code(kind = Timeout, http_status = 500)]
    CoreApiHttpClientTimeout,
    /// The HTTP client has failed to connect
    #[code(kind = Connect, http_status = 500)]
    CoreApiHttpClientConnectFailed,
}

generate_pkg_constructors!(PKG_NAME);
