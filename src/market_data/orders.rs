use std::fmt::Display;

use crate::Client;
use anyhow::{anyhow, Result};
use serde::Deserialize;

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
    pub price: String,
    pub qty: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
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
    pub price: String,
}

impl Client {
    /// Gets order up to certain limit (default is 100; max 5000)
    /// if limit > 5000, then the response will truncate to 5000
    pub async fn depth(&self, symbol: &str, limit: Option<u64>) -> Result<OrderBook> {
        let url = match limit {
            Some(limit) => format!(
                "{}{}{}{}{}",
                self.base_url, "/api/v3/depth?symbol=", symbol, "&limit=", limit
            ),
            None => format!("{}{}{}", self.base_url, "/api/v3/depth?symbol=", symbol),
        };
        match self.client.get(url).send().await?.json::<Orders>().await {
            Ok(orders) => Ok(OrderBook {
                symbol: symbol.to_owned(),
                orders,
            }),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// Get recent trades up to limit (default is 500, max 1000)
    pub async fn trades(&self, symbol: &str, limit: Option<u64>) -> Result<Vec<Trade>> {
        let url = match limit {
            Some(limit) => format!(
                "{}{}{}{}{}",
                self.base_url, "/api/v3/trades?symbol=", symbol, "&limit=", limit
            ),
            None => format!("{}{}{}", self.base_url, "/api/v3/trades?symbol=", symbol),
        };
        match self
            .client
            .get(url)
            .send()
            .await?
            .json::<Vec<Trade>>()
            .await
        {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// Get compressed, aggregate trades.
    /// Trades that fill at the time, from the same order,
    /// with the same price will have the quantity aggregated.
    pub async fn agg_trades(&self, symbol: &str) -> Result<Vec<Trade>> {
        let url = format!("{}{}{}", self.base_url, "/api/v3/trades?symbol=", symbol);
        match self
            .client
            .get(url)
            .send()
            .await?
            .json::<Vec<Trade>>()
            .await
        {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// Kline/candlestick bars for a symbol.
    /// Klines are uniquely identified by their open time.
    pub async fn kline(&self, symbol: &str, interval: Interval) -> Result<Vec<Kline>> {
        let url = format!(
            "{}{}{}{}{}",
            self.base_url, "/api/v3/klines?symbol=", symbol, "&interval=", interval
        );
        match self
            .client
            .get(url)
            .send()
            .await?
            .json::<Vec<Kline>>()
            .await
        {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub async fn avg_price(&self, symbol: &str) -> Result<AvgPrice> {
        let url = format!("{}{}{}", self.base_url, "/api/v3/avgPrice?symbol=", symbol);
        match self.client.get(url).send().await?.json::<AvgPrice>().await {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }
}
