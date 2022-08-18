use crate::Client;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    server_time: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    timezone: String,
    server_time: u64,
}

impl Client {
    pub async fn ping(&self) {
        let url = format!("{}{}", self.base_url, "/api/v3/ping");
        let _ = self.client.get(url).send().await;
    }

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

    pub async fn exchange_info(&self) -> Result<ExchangeInfo> {
        let url = format!("{}{}", self.base_url, "/api/v3/exchangeInfo");
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
