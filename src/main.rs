use anyhow::Result;
use binance_md::Client;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let client = Client::new()?;
    let depth = client.depth("BTCUSDT").await?;
    println!("{depth:#?}");
    Ok(())
}
