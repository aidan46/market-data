use crate::{
    api::{AvgPrice, BookTicker, Interval, LiveTicker, PriceStats},
    Binance,
};
use anyhow::{anyhow, Result};

impl Binance {
    /// Gets the live ticker price
    /// # Errors
    /// Returns [`Err`] when binance api returns an error
    pub async fn get_live_ticker_price(&self, symbols: Option<&[&str]>) -> Result<Vec<LiveTicker>> {
        let mut url = format!("{}{}", self.base_url, "/api/v3/ticker/price");
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
            .json::<Vec<LiveTicker>>()
            .await
        {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// Current average price for a symbol.
    /// # Errors
    /// Returns [`Err`] if endpoint returns an error
    pub async fn get_average_price(&self, symbol: &str) -> Result<AvgPrice> {
        let url = format!("{}{}{}", self.base_url, "/api/v3/avgPrice?symbol=", symbol);
        match self.client.get(url).send().await?.json::<AvgPrice>().await {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// Best price/qty on the order book for a symbol or symbols.
    /// # Errors
    /// Returns [`Err`] if endpoint returns an error
    pub async fn get_best_order_book_price(
        &self,
        symbols: Option<&[&str]>,
    ) -> Result<Vec<BookTicker>> {
        let mut url = format!("{}{}", self.base_url, "/api/v3/ticker/bookTicker");
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
            .json::<Vec<BookTicker>>()
            .await
        {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// 24 hour rolling window price change statistics. Careful when accessing this with no symbol.
    /// # Errors
    /// Returns [`Err`] if endpoint returns an error
    pub async fn get_24h_price_stats(&self, symbols: Option<&[&str]>) -> Result<Vec<PriceStats>> {
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

    /// Gets the price change data within a requested window of time.
    /// # Errors
    /// Returns [`Err`] if endpoint returns an error
    pub async fn get_rolling_window_price_stats(
        &self,
        _symbols: &[&str],
        _window_size: Interval,
    ) -> Result<Vec<PriceStats>> {
        todo!()
    }
}
