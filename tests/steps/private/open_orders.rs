use crate::context::ApiContext;
use crate::CONFIG;
use cucumber_rust::{given, then};
use stonk::domain::{order::OpenOrders, ApiResult};

#[given("a User with an API key and 2FA configured")]
async fn check_api_key_2fa(_test_ctx: &mut ApiContext) {
    assert!(!CONFIG.api_key.is_empty());
    assert!(!CONFIG.api_secret.is_empty());
    assert!(CONFIG.totp_pwd.is_some());
}

#[then("the response contains a valid OpenOrders result")]
async fn check_body_format(test_ctx: &mut ApiContext) {
    let body = &test_ctx.response.data;

    let res: ApiResult<OpenOrders> =
        serde_json::from_str(&body).expect("Invalid OpenOrders format.");

    let open_orders = res.result.expect("Missing OpenOrders result.");

    serde_json::to_writer_pretty(std::io::stdout(), &open_orders)
        .expect("Cannot report the OpenOrders result on the standard output.");
}
