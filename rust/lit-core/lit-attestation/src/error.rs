use std::fmt::Debug;

use derive_more::Display;

pub use lit_core::error::*;
use lit_core::generate_pkg_constructors;
use lit_core_derive::{Description, ErrorCode};

pub const PKG_NAME: &str = "lit_attestation";

#[derive(Debug, Display, Description, ErrorCode)]
#[allow(dead_code, clippy::enum_variant_names)]
pub(crate) enum EC {
    /// A fault error occured in the attestation system
    #[code(kind = Unexpected, http_status = 500)]
    AttestationFault,
    /// The verification of the attestation has failed.
    #[code(kind = Validation, http_status = 403)]
    AttestationVerifyFailed,
    /// The verification of the attestation policy has failed.
    #[code(kind = Validation, http_status = 403)]
    AttestationPolicyVerifyFailed,
    /// The verification of the attestation report has failed.
    #[code(kind = Validation, http_status = 403)]
    AttestationReportVerifyFailed,
    /// The verification of the attestation report hash has failed.
    #[code(kind = Validation, http_status = 403)]
    AttestationReportVerifyHashFailed,
    /// The parsing of the attestation report has failed.
    #[code(kind = Parser, http_status = 403)]
    AttestationReportParseFailed,
    /// The generation of an attestation report has failed.
    #[code(kind = Attestation, http_status = 500)]
    AttestationReportGenerateFailed,
    /// The verification of the attestation signatures has failed.
    #[code(kind = Validation, http_status = 403)]
    AttestationSignatureVerifyFailed,
    /// The verification of an auth header on a request has failed.
    #[code(kind = Validation, http_status = 403)]
    AttestationRequestAuthVerifyFailed,
    /// The auth header of a request is missing the body hash data.
    #[code(kind = Validation, http_status = 403)]
    AttestationRequestAuthBodyHashMissing,
    /// The auth header of a request has an invalid body hash.
    #[code(kind = Validation, http_status = 403)]
    AttestationRequestAuthBodyHashInvalid,
}

generate_pkg_constructors!(PKG_NAME, pub(crate), EC);
