use anyhow::Result;
use binance_md::Client;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let base_url = env::var("ENDPOINT")?;
    let client = Client::new(base_url);
    println!(
        "{:#?}",
        client.exchange_info(Some(&["BTCUSDT", "BNBBTC"])).await?
    );
    Ok(())
}
