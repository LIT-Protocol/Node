use lit_frost;

use super::curve_type::CurveType;

// Open questions:
// Do we assume just Cait-Sith for ECDSA & FROST for Schnorr, or do we need to support other algorithms for the same signature type?

pub enum SigningAlgorithm {
    Pairing,
    Ecdsa,
    Schnorr,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SigningScheme {
    Bls12381,
    EcdsaK256Sha256,
    SchnorrEd25519Sha512,
    SchnorrK256Sha256,
    SchnorrP256Sha256,
    SchnorrP384Sha384,
    SchnorrRistretto25519Sha512,
    SchnorrEd448Shake256,
    SchnorrRedJubjubBlake2b512,
    SchnorrK256Taproot,
}

impl TryFrom<SigningScheme> for lit_frost::Scheme {
    type Error = &'static str;
    fn try_from(value: SigningScheme) -> std::result::Result<lit_frost::Scheme, &'static str> {
        match value {
            SigningScheme::Bls12381 => Err("BLS12381 is not supported by FROST"),
            SigningScheme::EcdsaK256Sha256 => Err("ECDSA K256 SHA256 is not supported by FROST"),
            SigningScheme::SchnorrEd25519Sha512 => Ok(lit_frost::Scheme::Ed25519Sha512),
            SigningScheme::SchnorrK256Sha256 => Ok(lit_frost::Scheme::K256Sha256),
            SigningScheme::SchnorrP256Sha256 => Ok(lit_frost::Scheme::P256Sha256),
            SigningScheme::SchnorrP384Sha384 => Ok(lit_frost::Scheme::P384Sha384),
            SigningScheme::SchnorrRistretto25519Sha512 => {
                Ok(lit_frost::Scheme::Ristretto25519Sha512)
            }
            SigningScheme::SchnorrEd448Shake256 => Ok(lit_frost::Scheme::Ed448Shake256),
            SigningScheme::SchnorrRedJubjubBlake2b512 => Ok(lit_frost::Scheme::RedJubjubBlake2b512),
            SigningScheme::SchnorrK256Taproot => Ok(lit_frost::Scheme::K256Taproot),
        }
    }
}

impl SigningScheme {
    pub fn supports_algorithm(&self, algorithm: SigningAlgorithm) -> bool {
        // required to keep the matches aligned like this.
        matches!(
            (algorithm, self),
            (SigningAlgorithm::Pairing, SigningScheme::Bls12381)
                | (SigningAlgorithm::Ecdsa, SigningScheme::EcdsaK256Sha256)
                | (
                    SigningAlgorithm::Schnorr,
                    SigningScheme::SchnorrEd25519Sha512
                )
                | (SigningAlgorithm::Schnorr, SigningScheme::SchnorrK256Sha256)
                | (SigningAlgorithm::Schnorr, SigningScheme::SchnorrP256Sha256)
                | (SigningAlgorithm::Schnorr, SigningScheme::SchnorrP384Sha384)
                | (
                    SigningAlgorithm::Schnorr,
                    SigningScheme::SchnorrRistretto25519Sha512
                )
                | (
                    SigningAlgorithm::Schnorr,
                    SigningScheme::SchnorrEd448Shake256
                )
                | (
                    SigningAlgorithm::Schnorr,
                    SigningScheme::SchnorrRedJubjubBlake2b512
                )
                | (SigningAlgorithm::Schnorr, SigningScheme::SchnorrK256Taproot)
        )
    }

    pub fn curve_type(&self) -> CurveType {
        match self {
            SigningScheme::Bls12381 => CurveType::BLS,
            SigningScheme::EcdsaK256Sha256 => CurveType::K256,
            SigningScheme::SchnorrEd25519Sha512 => CurveType::Ed25519,
            SigningScheme::SchnorrK256Sha256 => CurveType::K256,
            SigningScheme::SchnorrP256Sha256 => CurveType::P256,
            SigningScheme::SchnorrP384Sha384 => CurveType::P384,
            SigningScheme::SchnorrRistretto25519Sha512 => CurveType::Ristretto25519,
            SigningScheme::SchnorrEd448Shake256 => CurveType::Ed448,
            SigningScheme::SchnorrRedJubjubBlake2b512 => CurveType::RedJubjub,
            SigningScheme::SchnorrK256Taproot => CurveType::K256,
        }
    }
}
