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

    println!("Fetching day-ahead prices for Finland...");
    println!("Period: {} to {}", start, end);
    println!();

    let xml_data = client
        .fetch_day_ahead_prices(BiddingZone::FI, start, end)
        .await?;

    println!(
        "✓ Successfully received {} bytes of XML data",
        xml_data.len()
    );
    println!();
    println!("First 500 characters of response:");
    println!("─────────────────────────────────────");
    println!(
        "{}",
        String::from_utf8_lossy(&xml_data[..500.min(xml_data.len())])
    );

    if xml_data.len() > 500 {
        println!("...");
        println!("(truncated {} bytes)", xml_data.len() - 500);
    }

    Ok(())
}
