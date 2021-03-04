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
        .expect("Cannot report the OpenOrders result on the standard output.");
}
