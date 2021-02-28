use crate::config::CONFIG;
use cucumber_rust::{async_trait, World, WorldInit};
use reqwest::Client;
use std::convert::Infallible;

#[derive(Debug, Default)]
pub struct HttpResponse {
    pub status: u16,
    pub data: String,
}

#[derive(WorldInit, Debug)]
pub struct ApiContext {
    pub api_base_url: String,
    pub http_client: Client,
    pub response: HttpResponse,
}

#[async_trait(?Send)]
impl World for ApiContext {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            api_base_url: String::from(&CONFIG.api_base_url),
            http_client: Client::new(),
            response: HttpResponse::default(),
        })
    }
}
