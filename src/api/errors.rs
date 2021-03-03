use base64::DecodeError;
use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwestError;
use std::{fmt, time::SystemTimeError};
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum ApiError {
    NonceError,
    DecodeError,
    HttpError,
    MalformedUrlError,
    InvalidHeaderError,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::NonceError => write!(
                f,
                "Cannot generate Nonce based on time elapsed since unix epoch."
            ),
            ApiError::DecodeError => write!(f, "Cannot decode secret."),
            ApiError::HttpError => write!(f, "An http error occured."),
            ApiError::MalformedUrlError => write!(f, "Malformed url. Verify your endpoint syntax."),
            ApiError::InvalidHeaderError => write!(f, "Invalid header for the http request."),
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

impl From<ReqwestError> for ApiError {
    fn from(_: ReqwestError) -> ApiError {
        ApiError::HttpError
    }
}

impl From<UrlParseError> for ApiError {
    fn from(_: UrlParseError) -> ApiError {
        ApiError::MalformedUrlError
    }
}

impl From<InvalidHeaderValue> for ApiError {
    fn from(_: InvalidHeaderValue) -> ApiError {
        ApiError::InvalidHeaderError
    }
}
