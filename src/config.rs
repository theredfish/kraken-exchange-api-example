//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into the structure.
//!
//! dotenv allows environment variables to be augmented/overwritten by a
//! .env file.

use dotenv::dotenv;

/// The configuration loaded from the .env or system environment variables.
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Config {
    /// The base API url
    pub api_base_url: String,
    /// The public API key
    pub api_key: String,
    /// The private API key
    pub api_secret: String,
    /// The one time password or 2FA password
    pub totp_pwd: Option<String>,
}

impl Config {
    /// Uses envy to inject dotenv and env vars into the Config struct
    pub fn new() -> Self {
        dotenv().ok();

        match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(error) => panic!("Configuration Error: {}", error),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
