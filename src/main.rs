use anyhow::Result;
use binance_md::Client;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let client = Client::new()?;
    println!(
        "{:#?}",
        client
            .agg_trades("BTCUSDT", None, None, None, Some(1))
            .await?
    );
    Ok(())
}
