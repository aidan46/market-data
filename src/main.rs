use anyhow::Result;
use binance_md::Client;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let client = Client::new()?;
    println!("{:#?}", client.depth("BTCUSDT", Some(2)).await?);
    Ok(())
}
