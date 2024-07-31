pub const PKG_NAME: &str = "lit_attestation_service";

use derive_more::Display;

pub use lit_core::error::*;
use lit_core::generate_pkg_constructors;
use lit_core_derive::{Description, ErrorCode};

#[derive(Debug, Display, Description, ErrorCode)]
#[allow(dead_code, clippy::enum_variant_names)]
pub(crate) enum EC {
    /// System does not support the following function.
    #[code(kind = Validation, http_status = 400)]
    AttestationServiceUnsupportedFunction,
    /// Missing attestation intent.
    #[code(kind = Validation, http_status = 400)]
    AttestationServiceMissingIntent,
    /// Attestation is inconsistent with intent.
    #[code(kind = Validation, http_status = 400)]
    AttestationServiceInconsistentAttestation,
    /// Attestation type is not supported.
    #[code(kind = Validation, http_status = 400)]
    AttestationServiceUnsupportedType,
    /// Unable to sign attestation.
    #[code(kind = Attestation, http_status = 500)]
    AttestationServiceFailureSigning,
    /// Unable to generate attestation.
    #[code(kind = Attestation, http_status = 500)]
    AttestationServiceFailureGenerating,
}

generate_pkg_constructors!(PKG_NAME, pub(crate), EC);
