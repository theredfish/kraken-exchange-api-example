/// Tradable asset pair
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPair {
    /// alternate pair name
    pub altname: Option<String>,
    /// WebSocket pair name (if available)
    pub wsname: Option<String>,
    /// asset class of base component
    pub aclass_base: Option<String>,
    /// asset id of base component
    pub base: Option<String>,
    /// asset class of quote component
    pub aclass_quote: Option<String>,
    /// asset id of quote component
    pub quote: Option<String>,
    /// volume lot size
    pub lot: Option<String>,
    /// scaling decimal places for pair
    pub pair_decimals: Option<u32>,
    /// scaling decimal places for volume
    pub lot_decimals: Option<u32>,
    /// amount to multiply lot volume by to get currency volume
    pub lot_multiplier: Option<u32>,
    /// array of leverage amounts available when buying
    pub leverage_buy: Option<Vec<u32>>,
    /// array of leverage amounts available when selling
    pub leverage_sell: Option<Vec<u32>>,
    /// fee schedule array in [volume, percent fee] tuples
    pub fees: Option<Vec<Vec<f64>>>,
    /// maker fee schedule array in [volume, percent fee] tuples (if on maker/taker)
    pub fees_maker: Option<Vec<Vec<f64>>>,
    /// volume discount currency
    pub fee_volume_currency: Option<String>,
    /// margin call level
    pub margin_call: Option<u32>,
    /// stop-out/liquidation margin level
    pub margin_stop: Option<u32>,
    /// minimum order volume for pair
    pub ordermin: Option<String>,
}
