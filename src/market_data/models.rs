use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug)]
pub struct OrderBook {
    pub symbol: String,
    pub orders: Orders,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orders {
    pub last_update_id: u64,
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub quote_qty: f64,
    pub time: u64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Deserialize)]
pub enum Interval {
    #[serde(rename(serialize = "m"))]
    Minute(u64),
    #[serde(rename(serialize = "h"))]
    Hour(u64),
    #[serde(rename(serialize = "d"))]
    Day(u64),
    #[serde(rename(serialize = "m"))]
    Month(u64),
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Minute(m) => write!(f, "{}m", m),
            Self::Hour(h) => write!(f, "{}h", h),
            Self::Day(d) => write!(f, "{}d", d),
            Self::Month(m) => write!(f, "{}M", m),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    pub open_time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub close_time: i64,
    pub quote_asset_volume: String,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
    pub _ignore: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvgPrice {
    pub mins: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggTrade {
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "a")]
    pub agg_id: u64,
    #[serde(rename = "f")]
    pub first_id: u64,
    #[serde(rename = "l")]
    pub last_id: u64,
    #[serde(rename = "m")]
    pub maker: bool,
    #[serde(rename = "M")]
    pub best_match: bool,
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub qty: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price_change: f64,
    #[serde(with = "string_or_float")]
    pub price_change_percent: f64,
    #[serde(with = "string_or_float")]
    pub weighted_avg_price: f64,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub open_price: f64,
    #[serde(with = "string_or_float")]
    pub high_price: f64,
    #[serde(with = "string_or_float")]
    pub low_price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    #[serde(with = "string_or_float")]
    pub quote_volume: f64,
    #[serde(with = "string_or_float")]
    pub last_qty: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float")]
    pub bid_qty: f64,
    #[serde(with = "string_or_float")]
    pub ask_price: f64,
    #[serde(with = "string_or_float")]
    pub ask_qty: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u16,
    pub limit: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u8,
    pub quote_asset: String,
    pub quote_asset_precision: u8,
    pub base_commission_precision: u8,
    pub quote_commission_precision: u8,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<Filter>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        #[serde(with = "string_or_float")]
        min_price: f64,
        #[serde(with = "string_or_float")]
        max_price: f64,
        #[serde(with = "string_or_float")]
        tick_size: f64,
    },
    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        #[serde(with = "string_or_float")]
        multiplier_up: f64,
        #[serde(with = "string_or_float")]
        multiplier_down: f64,
        avg_price_mins: u16,
    },
    #[serde(rename = "LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    LotSize {
        #[serde(with = "string_or_float")]
        min_qty: f64,
        #[serde(with = "string_or_float")]
        max_qty: f64,
        #[serde(with = "string_or_float")]
        step_size: f64,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional {
        #[serde(with = "string_or_float")]
        min_notional: f64,
        apply_to_market: bool,
        avg_price_mins: u16,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: u64 },
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        #[serde(with = "string_or_float")]
        min_qty: f64,
        #[serde(with = "string_or_float")]
        max_qty: f64,
        #[serde(with = "string_or_float")]
        step_size: f64,
    },
    #[serde(rename = "TRAILING_DELTA")]
    #[serde(rename_all = "camelCase")]
    TrailingDelta {
        min_trailing_above_delta: u16,
        max_trailing_above_delta: u16,
        min_trailing_below_delta: u16,
        max_trailing_below_delta: u16,
    },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrder { max_num_orders: u16 },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: u16 },
}

mod string_or_float {
    use serde::{de, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => {
                if s == "INF" {
                    Ok(f64::INFINITY)
                } else {
                    s.parse().map_err(de::Error::custom)
                }
            }
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}
