//! The entry point of BDD tests. Initializes the configuration, the test
//! context, and the test runners.

use lazy_static;

mod config;
mod domain;
mod steps;
mod support;

#[tokio::main]
async fn main() {
    use cucumber_rust::WorldInit;
    use support::api::ApiContext;

    // Make sure the config is initialized at the beginning instead on the
    // first call.
    lazy_static::initialize(&config::CONFIG);

    ApiContext::init(&["./tests/features"]).run_and_exit().await;
}
