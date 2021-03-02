use crate::context::ApiContext;
use cucumber_rust::then;
use stonk::domain::asset_pair::*;

#[then("the AssetPairs response body contains a valid asset pair response format")]
async fn asset_pair_check_body_format(test_ctx: &mut ApiContext) {
    let http_response = &test_ctx.response;
    let asset_pair_response: AssetPairResponse =
        serde_json::from_str(&http_response.data).expect("Cannot deserialize the http body");

    let asset_pair = asset_pair_response.result.asset_pair;

    assert!(
        asset_pair.altname.is_some(),
        "Failed to deserialize the asset pair."
    );

    println!("AssetPair : {:#?}", asset_pair);
}

#[then("the AssetPairs response body does not contain any error")]
async fn asset_pair_check_error_response(test_ctx: &mut ApiContext) {
    let http_response = &test_ctx.response;
    let asset_pair_response: AssetPairResponse =
        serde_json::from_str(&http_response.data).expect("Cannot deserialize the http body");

    assert!(
        asset_pair_response.error.is_empty(),
        "The error field should be empty"
    );
}

#[then("the AssetPairs response body contains fees information")]
async fn check_fees_body_format(test_ctx: &mut ApiContext) {
    let http_response = &test_ctx.response;
    let asset_pair_response: AssetPairResponse =
        serde_json::from_str(&http_response.data).expect("Cannot deserialize the http body");

    let asset_pair = asset_pair_response.result.asset_pair;

    assert!(asset_pair.fees.is_some(), "fees should have a value.");
    assert!(
        asset_pair.fees_maker.is_some(),
        "fees_maker should have a value.",
    );
    assert!(
        asset_pair.fee_volume_currency.is_some(),
        "fee_volume_currency should have a value.",
    );

    // A quick test on another filterable field that is exclusive with "fees".
    assert!(asset_pair.margin_call.is_none());
}
