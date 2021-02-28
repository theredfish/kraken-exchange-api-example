use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Time {
    pub unixtime: usize,
    pub rfc1123: String,
}

#[derive(Debug, Deserialize)]
pub struct TimeResponse {
    pub error: Vec<String>,
    pub result: Time,
}
