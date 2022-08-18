use crate::Client;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    last_update_id: u64,
    bids: Vec<Order>,
    asks: Vec<Order>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    price: String,
    qty: String,
}

impl Client {
    pub async fn depth(&self, symbol: &str) -> Result<OrderBook> {
        let url = format!("{}{}{}", self.base_url, "/api/v3/depth?symbol=", symbol);
        match self.client.get(url).send().await?.json::<OrderBook>().await {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }
}
