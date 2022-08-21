mod models;

use crate::Client;
use anyhow::{anyhow, Result};
pub use models::*;

impl Client {
    pub async fn ping(&self) {
        let url = format!("{}{}", self.base_url, "/api/v3/ping");
        match self.client.get(url).send().await {
            Ok(..) => println!("Ping successful!"),
            Err(e) => println!("Ping failed: {e}"),
        }
    }

    /// Gets servertime
    /// # Errors
    /// Returns [`Err`] if request is invalid
    pub async fn time(&self) -> Result<ServerTime> {
        let url = format!("{}{}", self.base_url, "/api/v3/time");
        match self
            .client
            .get(url.clone())
            .send()
            .await?
            .json::<ServerTime>()
            .await
        {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// Gets the exchange info
    /// # Errors
    /// Returns [`Err`] if any symbol provided in either symbol does not exist,
    /// the endpoint will throw an error.
    pub async fn exchange_info(&self, symbols: Option<&[&str]>) -> Result<ExchangeInfo> {
        let mut url = format!("{}{}", self.base_url, "/api/v3/exchangeInfo");
        if let Some(symbols) = symbols {
            url = format!("{}{}", url, "?symbols=[");
            for (i, symbol) in symbols.iter().enumerate() {
                url = format!("{}\"{}\"", url, symbol);
                if i != symbols.len() - 1 {
                    url = format!("{},", url);
                }
            }
            url = format!("{}]", url);
        }
        println!("URL = {url}");
        match self
            .client
            .get(url)
            .send()
            .await?
            .json::<ExchangeInfo>()
            .await
        {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// Gets order up to certain limit (default is 100; max 5000)
    /// if limit > 5000, then the response will truncate to 5000
    /// # Errors
    /// Returns [`Err`] if endpoint returns error
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
    /// # Errors
    /// Returns [`Err`] if endpoint returns error
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
    /// `from_id`: if to get aggregate trades from INCLUSIVE
    /// `start_time`: Timestamp in ms to get aggregate trades from INCLUSIVE
    /// `end_time`: Timestamp in ms to get aggregate trades until INCLUSIVE
    /// `limit`: Default 500; max 1000
    /// # Errors
    /// Returns [`Err`] if endpoint returns an error
    pub async fn agg_trades(
        &self,
        symbol: &str,
        from_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<AggTrade>> {
        let mut url = format!("{}{}{}", self.base_url, "/api/v3/aggTrades?symbol=", symbol);
        if let Some(id) = from_id {
            url = format!("{}{}{}", url, "&fromId=", id);
        }
        if let (Some(s_time), Some(e_time)) = (start_time, end_time) {
            url = format!(
                "{}{}{}{}{}",
                url, "&startTime=", s_time, "&endTime=", e_time
            );
        }
        if let Some(limit) = limit {
            url = format!("{}{}{}", url, "&limit=", limit);
        }
        match self
            .client
            .get(url)
            .send()
            .await?
            .json::<Vec<AggTrade>>()
            .await
        {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// Kline/candlestick bars for a symbol.
    /// Klines are uniquely identified by their open time.
    /// `start_time`: Timestamp in ms to get aggregate trades from INCLUSIVE
    /// `end_time`: Timestamp in ms to get aggregate trades until INCLUSIVE
    /// `limit`: Default 500; max 1000
    /// # Errors
    /// Returns [`Err`] if endpoint returns an error
    pub async fn kline(
        &self,
        symbol: &str,
        interval: Interval,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<Kline>> {
        let mut url = format!(
            "{}{}{}{}{}",
            self.base_url, "/api/v3/klines?symbol=", symbol, "&interval=", interval
        );
        if let (Some(s_time), Some(e_time)) = (start_time, end_time) {
            url = format!(
                "{}{}{}{}{}",
                url, "&startTime=", s_time, "&endTime=", e_time
            );
        }
        if let Some(limit) = limit {
            url = format!("{}{}{}", url, "&limit=", limit);
        }
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

    /// Current average price for a symbol.
    /// # Errors
    /// Returns [`Err`] if endpoint returns an error
    pub async fn avg_price(&self, symbol: &str) -> Result<AvgPrice> {
        let url = format!("{}{}{}", self.base_url, "/api/v3/avgPrice?symbol=", symbol);
        match self.client.get(url).send().await?.json::<AvgPrice>().await {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// 24 hour rolling window price change statistics. Careful when accessing this with no symbol.
    /// # Errors
    /// Returns [`Err`] if endpoint returns an error
    pub async fn price_stats_24h(&self, symbols: Option<&[&str]>) -> Result<Vec<PriceStats>> {
        let mut url = format!("{}{}", self.base_url, "/api/v3/ticker/24hr");
        if let Some(symbols) = symbols {
            url = format!("{}{}", url, "?symbols=[");
            for (i, symbol) in symbols.iter().enumerate() {
                url = format!("{}\"{}\"", url, symbol);
                if i != symbols.len() - 1 {
                    url = format!("{},", url);
                }
            }
            url = format!("{}]", url);
        }
        match self
            .client
            .get(url)
            .send()
            .await?
            .json::<Vec<PriceStats>>()
            .await
        {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }
}
