use crate::{
    api::{ServerTime, SystemStatus},
    Binance,
};
use anyhow::{anyhow, Result};

impl Binance {
    pub async fn ping(&self) {
        let url = format!("{}{}", self.base_url, "/api/v3/ping");
        match self.client.get(url).send().await {
            Ok(..) => println!("Ping successful!"),
            Err(e) => println!("Ping failed: {e}"),
        }
    }

    /// Gets `ServerTime`
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

    /// Fetches whether the system is normal or under maintenance.
    /// # Errors
    /// Returns [`Err`] if request is invalid
    pub async fn status(&self) -> Result<SystemStatus> {
        todo!()
    }
}
