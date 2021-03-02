use crate::CONFIG;
use cucumber_rust::{async_trait, World, WorldInit};
use reqwest::Client;
use std::convert::Infallible;

/// A http response structure to share between steps of a scenario. Generally
/// the first step (Given) will initialize the http request and handle the
/// the response. The subsequent steps will have access to this structure
/// through a mutable context.
#[derive(Debug, Default)]
pub struct HttpResponse {
    pub status: u16,
    pub data: String,
}

/// A mutable context for BDD steps in the case of REST API testing.
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
