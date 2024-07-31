use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("invalid key derive type: {0}")]
    InvalidKeyDeriveType(u8),
    #[error("mismatching curves or invalid share")]
    CurveMismatchOrInvalidShare,
    #[error("invalid curve")]
    CurveError,
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::InvalidKeyDeriveType(0)
    }
}

impl From<k256::elliptic_curve::Error> for Error {
    fn from(_: k256::elliptic_curve::Error) -> Self {
        Self::CurveError
    }
}
