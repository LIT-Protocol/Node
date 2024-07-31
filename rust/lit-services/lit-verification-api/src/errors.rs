use std::fmt::Debug;

use derive_more::Display;

pub use ::lit_core::error::*;
use lit_core::generate_pkg_constructors;
use lit_core_derive::{Description, ErrorCode};

pub const PKG_NAME: &str = "lit_verification_api";

#[derive(Debug, Display, Description, ErrorCode)]
#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum EC {
    #[code(kind=Unexpected, http_status = 500)]
    VerificationSystemFault,
    #[code(kind=Unexpected, http_status = 400)]
    VerificationBadRequestBody,
    #[code(kind=Unexpected, http_status = 500)]
    VerificationOTPRequestFailure,
    #[code(kind=Lock, http_status = 500)]
    VerificationCacheReadFailure,
    #[code(kind=Lock, http_status = 500)]
    VerificationCacheWriteFailure,
    #[code(kind=Unexpected, htttp_status = 400)]
    VerificationInvalidUserId,
    #[code(kind=Unexpected, http_status = 400)]
    VerificationUnknownRequestId,
    #[code(kind=Unexpected, http_status = 500)]
    VerificationEcdsaSystemFault,
    #[code(kind=Unexpected, http_status = 500)]
    VerificationEcdsaScalarGenerationError,
    #[code(kind=Unexpected, http_status = 500)]
    VerificationEcdsaKeyGenerationError,
    #[code(kind = Blockchain, http_status = 422)]
    VerificationBadInput,
    #[code(kind=Unexpected, http_status = 401)]
    VerificationUnauthorized,
    #[code(kind=HttpClient, http_status = 429)]
    VerificationRateLimitError,
    #[code(kind=HttpClient, http_status = 429)]
    VerificationClientAddress,
}

generate_pkg_constructors!(PKG_NAME, pub(crate), EC);
