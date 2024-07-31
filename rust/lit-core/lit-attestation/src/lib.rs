pub mod attestation;
pub mod config;
pub mod error;
#[cfg(feature = "kdf")]
pub mod kdf;
pub mod request;
#[cfg(feature = "service")]
pub mod service;
pub mod utils;
pub mod verification;

#[cfg(feature = "generate")]
pub use attestation::TryGenerate;
pub use attestation::{AdminSignedAttestation, AmdSevSnpAttestation, Attestation, AttestationType};
pub use error::{Error, Result};
pub use request::AttestedRequest;
pub use verification::verify_full;
