use crate::context::ApiContext;
use cucumber_rust::then;
use std::collections::HashMap;
use stonk::domain::{asset::AssetPair, ApiResult};

#[then("the AssetPairs response body contains a valid asset pair response format")]
async fn asset_pair_check_body_format(test_ctx: &mut ApiContext) {
    let body = &test_ctx.response.data;

    let res: ApiResult<HashMap<String, AssetPair>> =
        serde_json::from_str(&body).expect("Invalid AssetPairs format");

    let asset_pairs: HashMap<String, AssetPair> = res.result.expect("Missing asset pairs result.");

    serde_json::to_writer_pretty(std::io::stdout(), &asset_pairs)
        .expect("Cannot report the AssetPairs result on the standard output.");
}

#[then("the AssetPairs response body contains a valid asset pair response format with margin")]
async fn asset_pair_check_body_format_margin(test_ctx: &mut ApiContext) {
    let body = &test_ctx.response.data;

    let res: ApiResult<HashMap<String, AssetPair>> =
        serde_json::from_str(&body).expect("Invalid AssetPairs format");

    let asset_pairs: HashMap<String, AssetPair> = res.result.expect("Missing asset pairs result.");
    for (_, asset_pair) in asset_pairs.iter() {
        assert!(
            asset_pair.margin_stop.is_some(),
            "AssetPair should have margin_stop"
        );
    }

    serde_json::to_writer_pretty(std::io::stdout(), &asset_pairs)
        .expect("Cannot report the AssetPairs result on the standard output.");
}

#[then("the AssetPairs response body contains a valid asset pair response format with fees")]
async fn asset_pair_check_body_format_fees(test_ctx: &mut ApiContext) {
    let body = &test_ctx.response.data;

    let res: ApiResult<HashMap<String, AssetPair>> =
        serde_json::from_str(&body).expect("Invalid AssetPairs format");

    let asset_pairs: HashMap<String, AssetPair> = res.result.expect("Missing asset pairs result.");
    for (_, asset_pair) in asset_pairs.iter() {
        assert!(asset_pair.fees.is_some(), "The AssetPair should have fees.")
    }

    serde_json::to_writer_pretty(std::io::stdout(), &asset_pairs)
        .expect("Cannot report the AssetPairs result on the standard output.");
}
