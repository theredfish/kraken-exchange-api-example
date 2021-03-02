use crate::context::{ApiContext, HttpResponse};
use cucumber_rust::{then, when};
use url::Url;

#[when(regex = r#"^User make GET http request to "(.*)"$"#)]
async fn http_get(test_ctx: &mut ApiContext, endpoint: String) {
    let endpoint_url = format!("{}{}", test_ctx.api_base_url, endpoint);

    Url::parse(&endpoint_url).expect(&format!(
        "Invalid URL format for the endpoint {}",
        endpoint_url
    ));

    let res = test_ctx
        .http_client
        .get(&endpoint_url)
        .send()
        .await
        .expect(&format!(
            "The http request builder is not valid for the endpoint {}",
            endpoint_url
        ));

    let status = res.status().as_u16();

    let data = res.text().await.expect(&format!(
        "Cannot retrieve http response body for the endpoint {}",
        endpoint
    ));

    test_ctx.response = HttpResponse { status, data };
}

#[then(regex = r#"^http response status code is "(.*)"$"#)]
async fn http_response_ok(test_ctx: &mut ApiContext, status_code: u16) {
    let http_response = &test_ctx.response;

    assert_eq!(
        http_response.status, status_code,
        "Status code should be 200."
    );
}
