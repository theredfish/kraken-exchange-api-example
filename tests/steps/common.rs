use std::collections::HashMap;

use crate::context::{ApiContext, HttpResponse};
use cucumber_rust::{then, when};
use serde_json::Value;
use stonk::api::Api;
use stonk::domain::ApiResult;

#[when(regex = r#"^a User make an http request to "(.*)"$"#)]
async fn http_get(test_ctx: &mut ApiContext, endpoint: String) {
    let api = &test_ctx.api;
    test_ctx.response = handle_http_request(api, endpoint, HashMap::new()).await;
}

#[then(regex = r#"^http response status code is "(.*)"$"#)]
async fn http_response_ok(test_ctx: &mut ApiContext, status_code: u16) {
    let http_response = &test_ctx.response;

    assert_eq!(
        http_response.status, status_code,
        "Status code should be 200."
    );
}

#[then("the response body does not contain any error")]
async fn check_error_response(test_ctx: &mut ApiContext) {
    let body = &test_ctx.response.data;

    let res: ApiResult<Value> =
        serde_json::from_str(&body).expect("Cannot deserialize the http body");

    assert!(
        res.error.is_empty(),
        "The HTTP response shouldn't contain errors"
    );
}

async fn handle_http_request(
    api: &Api,
    endpoint: String,
    data: HashMap<String, String>,
) -> HttpResponse {
    if endpoint.contains("private") {
        let res = api
            .private_call(&endpoint, data)
            .await
            .unwrap_or_else(|_| panic!("Error during http request to private endpoint"));

        let status = res.status().as_u16();

        let data = res
            .text()
            .await
            .unwrap_or_else(|_| panic!("Cannot retrieve http response body"));

        return HttpResponse { status, data };
    } else if endpoint.contains("public") {
        let res = api
            .public_call(&endpoint)
            .await
            .unwrap_or_else(|_| panic!("Error during http request to public endpoint"));

        let status = res.status().as_u16();

        let data = res
            .text()
            .await
            .unwrap_or_else(|_| panic!("Cannot retrieve http response body"));

        return HttpResponse { status, data };
    } else {
        panic!("Private or public endpoint expected.");
    }
}
