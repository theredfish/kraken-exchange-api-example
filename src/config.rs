//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into the structure.
//!
//! dotenv allows environment variables to be augmented/overwritten by a
//! .env file.
//!
//! This file uses lazy_static to avoid multiple processing.

use dotenv::dotenv;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub api_base_url: String,
}

impl Config {
    /// Uses envy to inject dotenv and env vars into the Config struct
    pub fn new() -> Self {
        dotenv().ok();

        match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(error) => panic!("Configuration Error: {:#?}", error),
        }
    }
}
