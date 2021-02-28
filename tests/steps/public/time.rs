use crate::domain::time::*;
use crate::support::api::*;
use chrono::DateTime;
use cucumber_rust::{then, when};
use url::Url;

#[when("I access the server time from /0/public/Time")]
async fn get_http_request(test_ctx: &mut ApiContext) {
    let endpoint = String::from("/0/public/Time");
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

// TODO : parametrize the status code and add this in support
#[then("the http status code should be 200")]
async fn check_status_ok(test_ctx: &mut ApiContext) {
    let http_response = &test_ctx.response;

    assert_eq!(http_response.status, 200, "Status code should be 200.");
}

#[then("the response body does not contain any error")]
async fn check_error_response(test_ctx: &mut ApiContext) {
    let http_response = &test_ctx.response;

    let time_response: TimeResponse =
        serde_json::from_str(&http_response.data).expect("Cannot deserialize the http body");

    assert!(
        time_response.error.is_empty(),
        "Error field should be empty"
    );
}

#[then("the response body contains a valid response format")]
async fn check_body_format(test_ctx: &mut ApiContext) {
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
