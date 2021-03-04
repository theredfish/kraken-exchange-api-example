//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into the structure.
//!
//! dotenv allows environment variables to be augmented/overwritten by a
//! .env file.

use dotenv::dotenv;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub api_base_url: String,
    pub api_key: String,
    pub api_secret: String,
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
