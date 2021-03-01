use serde::de::{Deserialize, Deserializer, Error, MapAccess, Visitor};

use std::fmt;
#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct AssetPair {
    pub altname: Option<String>,
    pub wsname: Option<String>,
    pub aclass_base: Option<String>,
    pub base: Option<String>,
    pub aclass_quote: Option<String>,
    pub quote: Option<String>,
    pub lot: Option<String>,
    pub pair_decimals: Option<i64>,
    pub lot_decimals: Option<i64>,
    pub lot_multiplier: Option<i64>,
    pub leverage_buy: Option<Vec<i64>>,
    pub leverage_sell: Option<Vec<i64>>,
    pub fees: Option<Vec<Vec<f64>>>,
    pub fees_maker: Option<Vec<Vec<f64>>>,
    pub fee_volume_currency: Option<String>,
    pub margin_call: Option<i64>,
    pub margin_stop: Option<i64>,
    pub ordermin: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct UnknownAssetPair {
    pub asset_pair: AssetPair,
}

struct UnknownAssetPairVisitor;

/// A structure to deserialize an http response for the asset pair endpoint.
/// Since the asset pair key (e.g XBTUSD -> XXBTZUSD) is dynamic we don't know
/// the data structure for the different asset pairs. The BDD tests will handle that
/// case by deserializing the json dynamically into an AssetPair structure.
#[derive(Debug, Deserialize)]
pub struct AssetPairResponse {
    pub error: Vec<serde_json::Value>,
    pub result: UnknownAssetPair,
}

impl<'de> Visitor<'de> for UnknownAssetPairVisitor {
    type Value = UnknownAssetPair;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "A data structure representing an AssetPair with an unknown asset pair id as key",
        )
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        if let Some((_dynamic_key, asset_pair)) = access.next_entry::<String, AssetPair>()? {
            Ok(UnknownAssetPair { asset_pair })
        } else {
            Err(M::Error::custom("Cannot deserialize the data. A string is expected for the key and an AssetPair is expected for the value."))
        }
    }
}

impl<'de> Deserialize<'de> for UnknownAssetPair {
    fn deserialize<D>(deserializer: D) -> Result<UnknownAssetPair, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(UnknownAssetPairVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_deserialize_an_asset_pair_with_dynamic_key_id() {
        let json = r#"{"XXBTZUSD":{"leverage_buy":[2,3,4,5],"leverage_sell":[2,3,4,5]}}"#;
        let asset_pair: UnknownAssetPair = serde_json::from_str(json)
            .expect("Should deserialize the string to an UnknownAssetPair");
        let expected_asset_pair = UnknownAssetPair {
            asset_pair: AssetPair {
                leverage_buy: Some(vec![2, 3, 4, 5]),
                leverage_sell: Some(vec![2, 3, 4, 5]),
                ..Default::default()
            },
        };

        assert_eq!(asset_pair, expected_asset_pair);
    }
}
