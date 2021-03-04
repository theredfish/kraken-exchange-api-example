/// Server's time.
#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    /// as unix timestamp
    pub unixtime: usize,
    /// as RFC 1123 time format
    pub rfc1123: String,
}
