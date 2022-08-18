use crate::Client;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    status: u8,
    msg: String,
}

impl Client {
    pub async fn system_status(&self) -> Result<Status> {
        let url = format!("{}{}", &self.base_url, "/sapi/v1/system/status");
        match self.client.get(url).send().await?.json::<Status>().await {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        }
    }
}
