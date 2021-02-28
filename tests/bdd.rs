mod domain;
mod steps;
mod support;

#[tokio::main]
async fn main() {
    use cucumber_rust::WorldInit;
    use support::api::ApiContext;

    ApiContext::init(&["./tests/features"]).run_and_exit().await;
}
