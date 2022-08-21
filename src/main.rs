use anyhow::Result;
use binance_md::{market_data::Interval, Client};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let base_url = env::var("ENDPOINT")?;
    let client = Client::new(base_url);
    let symbol = "BTCUSDT";
    let limit = Some(1);
    let from_id = None;
    let start_time = None;
    let end_time = None;
    let interval = Interval::Minute(1);
    client.ping().await;
    println!("{:#?}", client.time().await?);
    println!("{:#?}", client.exchange_info(Some(&[symbol])).await?);
    println!("{:#?}", client.depth(symbol, limit).await?);
    println!("{:#?}", client.trades(symbol, limit).await?);
    println!(
        "{:#?}",
        client
            .agg_trades(symbol, from_id, start_time, end_time, limit)
            .await?
    );
    println!(
        "{:#?}",
        client
            .kline(symbol, interval, start_time, end_time, limit)
            .await?
    );
    println!("{:#?}", client.avg_price(symbol).await?);
    println!("{:#?}", client.price_stats_24h(Some(&[symbol])).await?);
    println!("{:#?}", client.symbol_price(Some(&[symbol])).await?);
    Ok(())
}
