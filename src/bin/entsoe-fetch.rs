use chrono::{Duration, Utc};
use entsoe::{BiddingZone, EntsoeClient};
use rusqlite::{params, Connection};
use std::env;

fn init_database(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS prices (
            timestamp TEXT NOT NULL,
            price TEXT NOT NULL,
            currency TEXT NOT NULL,
            price_area TEXT NOT NULL,
            PRIMARY KEY (timestamp, price_area)
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_timestamp ON prices(timestamp)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_price_area ON prices(price_area)",
        [],
    )?;

    Ok(())
}

fn store_prices(
    conn: &Connection,
    zone: BiddingZone,
    price_doc: &entsoe::PriceDocument,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO prices (timestamp, price, currency, price_area)
         VALUES (?1, ?2, ?3, ?4)",
    )?;

    let zone_code = zone.code();

    for price_point in &price_doc.prices {
        let timestamp = price_point.timestamp.to_rfc3339();
        let price = format!("{:.5}", price_point.price_per_kwh());

        stmt.execute(params![
            timestamp,
            price,
            &price_doc.currency,
            zone_code,
        ])?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <DATABASE_PATH> <BIDDING_ZONE> [HOURS]", args[0]);
        eprintln!();
        eprintln!("Environment variables:");
        eprintln!("  ENTSOE_API_TOKEN    Required: Your ENTSO-E API token");
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  DATABASE_PATH       Path to SQLite database file");
        eprintln!("  BIDDING_ZONE        e.g., FI, NO2, SE3, DE, FR");
        eprintln!("  HOURS               Optional: hours from now (default: 24)");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  ENTSOE_API_TOKEN=your-token {} prices.db FI 48", args[0]);
        eprintln!();
        eprintln!("Note: Writes are idempotent - safe to run multiple times");
        std::process::exit(1);
    }

    let api_token = env::var("ENTSOE_API_TOKEN").map_err(|_| {
        "ENTSOE_API_TOKEN environment variable not set"
    })?;

    let db_path = &args[1];
    let zone_code = &args[2];
    let hours: i64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(24);

    let zone = BiddingZone::from_code(zone_code).ok_or_else(|| {
        format!(
            "Invalid bidding zone: '{}'. Valid zones: FI, NO2, SE3, DE, FR, etc.",
            zone_code
        )
    })?;

    eprintln!("Opening database: {}", db_path);
    let conn = Connection::open(db_path)?;

    eprintln!("Initializing database schema...");
    init_database(&conn)?;

    let client = EntsoeClient::new(api_token);

    let start = Utc::now();
    let end = start + Duration::hours(hours);

    eprintln!("Fetching prices for {} from {} to {}", zone, start, end);

    let price_doc = client.get_day_ahead_prices(zone, start, end).await?;

    eprintln!("Received {} price points", price_doc.prices.len());
    eprintln!("Storing in database...");

    store_prices(&conn, zone, &price_doc)?;

    eprintln!("Done! Stored {} prices for {}", price_doc.prices.len(), zone);

    Ok(())
}
