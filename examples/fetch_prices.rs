use chrono::{Duration, Utc};
use entsoe::{BiddingZone, EntsoeClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = env::var("ENTSOE_API_TOKEN").expect(
        "ENTSOE_API_TOKEN environment variable must be set. \
         Usage: ENTSOE_API_TOKEN=your-token cargo run --example fetch_prices",
    );

    let client = EntsoeClient::new(api_token);

    let start = Utc::now();
    let end = start + Duration::hours(24);

    let price_doc = client
        .get_day_ahead_prices(BiddingZone::FI, start, end)
        .await?;

    println!("{:#?}", price_doc);

    Ok(())
}
