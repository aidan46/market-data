#![warn(clippy::pedantic)]
pub mod market_data;

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    /// Constructor function
    #[must_use]
    pub fn new(base_url: String) -> Self {
        println!("Base url = {base_url}");
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
}
