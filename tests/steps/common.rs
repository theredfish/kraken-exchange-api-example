use std::collections::HashMap;

use crate::context::{ApiContext, HttpResponse};
use cucumber_rust::{then, when};
use stonk::api::Api;

#[when(regex = r#"^User make GET http request to "(.*)"$"#)]
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

async fn handle_http_request(
    api: &Api,
    endpoint: String,
    data: HashMap<String, String>,
) -> HttpResponse {
    if endpoint.contains("private") {
        let res = api.private_call(&endpoint, data).await.expect(&format!(
            "Error during http request to private endpoint {}",
            endpoint
        ));

        let status = res.status().as_u16();

        let data = res.text().await.expect(&format!(
            "Cannot retrieve http response body for the endpoint {}",
            endpoint
        ));

        return HttpResponse { status, data };
    } else if endpoint.contains("public") {
        let res = api.public_call(&endpoint).await.expect(&format!(
            "Error during http request to public endpoint {}",
            endpoint
        ));

        let status = res.status().as_u16();

        let data = res.text().await.expect(&format!(
            "Cannot retrieve http response body for the endpoint {}",
            endpoint
        ));

        return HttpResponse { status, data };
    } else {
        panic!(format!(
            "Private or public endpoint expected. Found : {}",
            endpoint
        ));
    }
}
