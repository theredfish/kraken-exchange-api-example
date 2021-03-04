use std::collections::HashMap;

/// Open orders
#[derive(Deserialize, Serialize, Debug)]
pub struct OpenOrders {
    /// A list of open orders identified by their ID
    pub open: HashMap<String, OrderInfo>,
}

/// Order information
#[derive(Deserialize, Serialize, Debug)]
pub struct OrderInfo {
    /// Referral order transaction id that created this order
    pub refid: Option<String>,
    /// user reference id
    pub userref: Option<u64>,
    /// status of order:
    pub status: OrderStatus,
    /// unix timestamp of when order was placed
    pub opentm: f64,
    /// unix timestamp of order start time (or 0 if not set)
    pub starttm: f64,
    /// unix timestamp of order end time (or 0 if not set)
    pub expiretm: f64,
    /// order description info
    pub descr: OrderDescription,
    /// volume of order (base currency unless viqc set in oflags)
    pub vol: String,
    /// volume executed (base currency unless viqc set in oflags)
    pub vol_exec: String,
    /// total cost (quote currency unless unless viqc set in oflags)
    pub cost: String,
    /// average price (quote currency unless viqc set in oflags)
    pub price: String,
    /// stop price (quote currency, for trailing stops)
    pub stopprice: Option<String>,
    /// triggered limit price (quote currency, when limit based order type triggered)
    pub limitprice: Option<String>,
    /// comma delimited list of miscellaneous info
    /// * stopped = triggered by stop price
    /// * touched = triggered by touch price
    /// * liquidated = liquidation
    /// * partial = partial fill
    pub misc: String,
    /// comma delimited list of order flags
    /// * viqc = volume in quote currency
    /// * fcib = prefer fee in base currency (default if selling)
    /// * fciq = prefer fee in quote currency (default if buying)
    /// * nompp = no market price protection
    pub oflags: String,
}

/// Order status
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// order pending book entry
    Pending,
    /// open order
    Open,
    /// closed order
    Closed,
    /// order canceled
    Canceled,
    /// order expired
    Expired,
}

/// Order description
#[derive(Deserialize, Serialize, Debug)]
pub struct OrderDescription {
    /// asset pair
    pub pair: String,
    /// type of order (buy/sell)
    pub r#type: String,
    /// order type (See Add standard order)
    pub ordertype: String,
    /// primary price
    pub price: String,
    /// secondary price
    pub price2: String,
    /// amount of leverage
    pub leverage: String,
    /// order description
    pub order: String,
    /// conditional close order description (if conditional close set)
    pub close: String,
}
