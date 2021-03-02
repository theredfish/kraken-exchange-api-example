use base64::DecodeError;
use std::{fmt, time::SystemTimeError};
#[derive(Debug)]
pub enum ApiError {
    NonceError,
    DecodeError,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::NonceError => write!(
                f,
                "Cannot generate Nonce based on time elapsed since unix epoch."
            ),
            ApiError::DecodeError => write!(f, "Cannot decode secret."),
        }
    }
}

impl From<SystemTimeError> for ApiError {
    fn from(_: SystemTimeError) -> ApiError {
        ApiError::NonceError
    }
}

impl From<DecodeError> for ApiError {
    fn from(_: DecodeError) -> ApiError {
        ApiError::DecodeError
    }
}
