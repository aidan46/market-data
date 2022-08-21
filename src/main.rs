use anyhow::Result;
use binance_md::{market_data::Interval, Client};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let client = Client::new()?;
    println!(
        "{:#?}",
        client
            .kline("BTCUSDT", Interval::Minute(1), None, None, None)
            .await?
    );
    Ok(())
}
