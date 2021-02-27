use chrono::DateTime;
use cucumber_rust::{async_trait, given, then, when, World, WorldInit};
use reqwest::{self, StatusCode};
use serde::Deserialize;
use serde_json::Value;
use std::{cell::RefCell, convert::Infallible};
use url::Url;

#[derive(Debug, Deserialize)]
struct Time {
    unixtime: usize,
    rfc1123: String,
}

#[derive(Debug, Deserialize)]
struct TimeResponse {
    error: Vec<String>,
    result: Time,
}

#[derive(Debug, Default)]
struct HttpResponse {
    status: u16,
    data: String,
}

#[derive(WorldInit, Debug)]
pub struct TestContext {
    response: HttpResponse,
}

#[async_trait(?Send)]
impl World for TestContext {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            response: HttpResponse::default(),
        })
    }
}

// TODO : param url
#[when("I access the server time from /0/public/Time")]
async fn get_http_request(test_ctx: &mut TestContext) {
    let endpoint = String::from("/0/public/Time");
    let api = String::from("https://api.kraken.com");
    let endpoint_url = format!("{}{}", api, endpoint);

    Url::parse(&endpoint_url).expect(&format!(
        "Invalid URL format for the endpoint {}",
        endpoint_url
    ));

    let res = reqwest::get(&endpoint_url).await.expect(&format!(
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

// TODO : parametrize the status code and add this in support
#[then("the http status code should be 200")]
fn check_status_ok(test_ctx: &mut TestContext) {
    let http_response = &test_ctx.response;

    assert_eq!(http_response.status, 200, "Status code should be 200.");
}

#[then("the response body does not contain any error")]
fn check_error_response(test_ctx: &mut TestContext) {
    let http_response = &test_ctx.response;

    let time_response: TimeResponse =
        serde_json::from_str(&http_response.data).expect("Cannot deserialize the http body");

    assert!(
        time_response.error.is_empty(),
        "Error field should be empty"
    );
}

#[then("the response body contains a valid response format")]
fn check_body_format(test_ctx: &mut TestContext) {
    let http_response = &test_ctx.response;

    let time_response: TimeResponse =
        serde_json::from_str(&http_response.data).expect("Cannot deserialize the http body");

    // See rfc2822 - 3.3. Date and Time Specification
    // https://tools.ietf.org/html/rfc2822#page-14
    let date_time = DateTime::parse_from_rfc2822(&time_response.result.rfc1123)
        .expect("Cannot parse rfc1123 field");

    // Ensure that we get the same timestamp from RFC1123 field compared with the value of the field unixtime.
    // RFC1123 has been converted to RFC2822 - it should be safe since it's the standard way
    // to represent date and time. RFC1123 can be seen as a parent of RFC2822.
    assert_eq!(
        date_time.timestamp(),
        time_response.result.unixtime as i64,
        "The parsed rfc1123 field does not give the same timestamp as the unixtime field"
    );
}

#[tokio::main]
async fn main() {
    let runner = TestContext::init(&["./tests/features/"]);
    runner.run_and_exit().await;
}
