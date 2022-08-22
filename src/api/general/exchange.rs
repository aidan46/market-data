use crate::{api::ExchangeInfo, Binance};
use anyhow::{anyhow, Result};

impl Binance {
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
}
