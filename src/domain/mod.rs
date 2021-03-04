//! This module contains the data structures representing the domain under test.
//!
//! Generally this domain layer exists in applications following the
//! "Domain Driven Design" methodology. This domain is part of the ubiquitous
//! language shared by developers and domain experts.
//!
//! BDD tests can take advantage of this domain layer and share the same data
//! structures. In our case we define here the different structures that we need
//! in our tests and their serde::Deserialize implementation.

pub mod asset;
pub mod order;
pub mod time;

/// Response of API
#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResult<T> {
    /// array of error messages in the format of:
    /// + <char-severity code><string-error category>:<string-error type>[:<string-extra info>]
    /// + severity code can be E for error or W for warning
    pub error: Vec<String>,
    /// result of API call (may not be present if errors occur)
    pub result: Option<T>,
}
