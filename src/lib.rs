#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
pub mod api;
pub(crate) mod utils;
pub(crate) use utils::string_or_float;

pub struct Binance {
    base_url: String,
    client: reqwest::Client,
}

impl Binance {
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
