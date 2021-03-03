//! The domain definition for Time.
use serde::Deserialize;

/// A structure that represents the server time.
#[derive(Debug, Deserialize)]
pub struct Time {
    pub unixtime: usize,
    pub rfc1123: String,
}

/// A structure that represents the http response returned by the endpoint.
#[derive(Debug, Deserialize)]
pub struct TimeResponse {
    pub error: Vec<::serde_json::Value>,
    pub result: Time,
}
