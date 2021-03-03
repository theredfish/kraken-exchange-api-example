use crate::CONFIG;
use cucumber_rust::{async_trait, World, WorldInit};
use reqwest::{Client, Response};
use std::convert::Infallible;
use std::convert::TryFrom;
use stonk::api::Api;

/// A http response structure to share between steps of a scenario.
#[derive(Debug, Default)]
pub struct HttpResponse {
    pub status: u16,
    pub data: String,
}

/// A mutable context for BDD steps in the case of REST API testing.
#[derive(WorldInit)]
pub struct ApiContext {
    pub api: Api,
    pub response: HttpResponse,
}

#[async_trait(?Send)]
impl World for ApiContext {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            api: Api::new(
                CONFIG.api_base_url.to_owned(),
                CONFIG.api_key.to_owned(),
                CONFIG.api_secret.to_owned(),
                CONFIG.totp_pwd.to_owned(),
            ),
            response: HttpResponse::default(),
        })
    }
}
