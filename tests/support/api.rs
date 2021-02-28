use cucumber_rust::{async_trait, World, WorldInit};
use std::convert::Infallible;

#[derive(Debug, Default)]
pub struct HttpResponse {
    pub status: u16,
    pub data: String,
}

#[derive(WorldInit, Debug)]
pub struct ApiContext {
    pub response: HttpResponse,
}

#[async_trait(?Send)]
impl World for ApiContext {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            response: HttpResponse::default(),
        })
    }
}
