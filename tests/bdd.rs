//! The entry point of BDD tests. Initializes the configuration, the test
//! context, and the test runners.

use stonk::config::Config;

mod context;
mod steps;

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[tokio::main]
async fn main() {
    use context::ApiContext;
    use cucumber_rust::WorldInit;

    // Run all scenarios even if previous failed. It allows us to get a full
    // report. The cli option allow us to pass options like --debug.
    // Usage example : cargo test --test bdd -- --debug
    ApiContext::init(&["./tests/features"]).cli().run().await;
}
