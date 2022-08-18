mod market_data;
mod wallet;
use anyhow::Result;
use std::env;

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Self> {
        let base_url = env::var("ENDPOINT")?;
        println!("Base url = {base_url}");
        Ok(Self {
            base_url,
            client: reqwest::Client::new(),
        })
    }
}
