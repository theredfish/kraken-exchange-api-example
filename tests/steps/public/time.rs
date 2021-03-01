use crate::context::ApiContext;
use chrono::DateTime;
use cucumber_rust::then;
use stonk::domain::time::*;

#[then("the Time response body does not contain any error")]
async fn check_error_response(test_ctx: &mut ApiContext) {
    let http_response = &test_ctx.response;

    let time_response: TimeResponse =
        serde_json::from_str(&http_response.data).expect("Cannot deserialize the http body");

    assert!(
        time_response.error.is_empty(),
        "Error field should be empty"
    );
}

#[then("the Time response body contains a valid response format")]
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
