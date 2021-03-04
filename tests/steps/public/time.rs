use crate::context::ApiContext;
use chrono::DateTime;
use cucumber_rust::then;
use stonk::domain::{time::*, ApiResult};

#[then("the Time response body contains a valid response format")]
async fn check_body_format(test_ctx: &mut ApiContext) {
    let body = &test_ctx.response.data;

    let res: ApiResult<Time> =
        serde_json::from_str(&body).expect("Cannot deserialize the http body");

    let time = res.result.unwrap_or_else(|| panic!("Missing time result."));

    // See rfc2822 - 3.3. Date and Time Specification
    // https://tools.ietf.org/html/rfc2822#page-14
    let date_time =
        DateTime::parse_from_rfc2822(&time.rfc1123).expect("Cannot parse rfc1123 field");

    // Ensure that we get the same timestamp from RFC1123 field compared with the value of the field unixtime.
    // RFC1123 has been converted to RFC2822 - it should be safe since it's the standard way
    // to represent date and time. RFC1123 can be seen as a parent of RFC2822.
    assert_eq!(
        date_time.timestamp(),
        time.unixtime as i64,
        "The parsed rfc1123 field does not give the same timestamp as the unixtime field"
    );

    serde_json::to_writer_pretty(std::io::stdout(), &time)
        .expect("Cannot report the Time result on the standard output.");
}
