use chrono::{Duration, Utc};
use entsoe::{BiddingZone, EntsoeClient};
use rusqlite::{Connection, params};
use std::env;

fn init_database(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Create prices table with CHECK constraints for data validation:
    // - timestamp: RFC3339 format (YYYY-MM-DDTHH:MM:SS...)
    // - price: Must be numeric (can be negative - electricity prices can go negative)
    // - currency: Must be 3 uppercase letters (e.g., EUR, SEK)
    // - price_area: Must be 2-8 characters (e.g., FI, NO2, IT-North)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS prices (
            timestamp TEXT NOT NULL CHECK(timestamp GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]T[0-9][0-9]:[0-9][0-9]:[0-9][0-9]*'),
            price TEXT NOT NULL CHECK(price GLOB '*[0-9]*' AND typeof(CAST(price AS REAL)) = 'real'),
            currency TEXT NOT NULL CHECK(length(currency) = 3 AND currency = upper(currency)),
            price_area TEXT NOT NULL CHECK(length(price_area) >= 2 AND length(price_area) <= 8),
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

        stmt.execute(params![timestamp, price, &price_doc.currency, zone_code,])?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <DATABASE_PATH> [BIDDING_ZONE] [HOURS]", args[0]);
        eprintln!();
        eprintln!("Environment variables:");
        eprintln!("  ENTSOE_API_TOKEN    Required: Your ENTSO-E API token");
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  DATABASE_PATH       Path to SQLite database file");
        eprintln!(
            "  BIDDING_ZONE        Optional: specific zone (e.g., FI, NO2) or omit for all zones"
        );
        eprintln!("  HOURS               Optional: hours from now (default: 24)");
        eprintln!();
        eprintln!("Examples:");
        eprintln!(
            "  ENTSOE_API_TOKEN=your-token {} prices.db           # Fetch all zones",
            args[0]
        );
        eprintln!(
            "  ENTSOE_API_TOKEN=your-token {} prices.db FI        # Fetch only FI",
            args[0]
        );
        eprintln!(
            "  ENTSOE_API_TOKEN=your-token {} prices.db FI 48     # Fetch FI, 48 hours",
            args[0]
        );
        eprintln!();
        eprintln!("Note: Writes are idempotent - safe to run multiple times");
        std::process::exit(1);
    }

    let api_token = env::var("ENTSOE_API_TOKEN")
        .map_err(|_| "ENTSOE_API_TOKEN environment variable not set")?;

    let db_path = &args[1];

    let zones: Vec<BiddingZone> = if let Some(zone_code) = args.get(2) {
        let zone = BiddingZone::from_code(zone_code).ok_or_else(|| {
            format!(
                "Invalid bidding zone: '{}'. Valid zones: FI, NO2, SE3, DE, FR, etc.",
                zone_code
            )
        })?;
        vec![zone]
    } else {
        BiddingZone::all_zones()
    };

    let hours: i64 = if zones.len() == 1 {
        args.get(3).and_then(|s| s.parse().ok()).unwrap_or(24)
    } else {
        args.get(2).and_then(|s| s.parse().ok()).unwrap_or(24)
    };

    eprintln!("Opening database: {}", db_path);
    let conn = Connection::open(db_path)?;

    eprintln!("Initializing database schema...");
    init_database(&conn)?;

    let client = EntsoeClient::new(api_token);

    let start = Utc::now();
    let end = start + Duration::hours(hours);

    if zones.len() == 1 {
        eprintln!("Fetching prices for {} from {} to {}", zones[0], start, end);
    } else {
        eprintln!(
            "Fetching prices for {} zones from {} to {}",
            zones.len(),
            start,
            end
        );
    }

    let mut total_prices = 0;
    let mut successful_zones = 0;
    let mut failed_zones = Vec::new();

    for zone in zones {
        eprint!("  Fetching {}... ", zone);

        match client.get_day_ahead_prices(zone, start, end).await {
            Ok(price_doc) => match store_prices(&conn, zone, &price_doc) {
                Ok(_) => {
                    eprintln!("✓ {} prices", price_doc.prices.len());
                    total_prices += price_doc.prices.len();
                    successful_zones += 1;
                }
                Err(e) => {
                    eprintln!("✗ Database error: {}", e);
                    failed_zones.push((zone, format!("Database error: {}", e)));
                }
            },
            Err(e) => {
                eprintln!("✗ {}", e);
                failed_zones.push((zone, e.to_string()));
            }
        }
    }

    eprintln!();
    eprintln!("Summary:");
    eprintln!("  Successful: {} zones", successful_zones);
    eprintln!("  Total prices stored: {}", total_prices);

    if !failed_zones.is_empty() {
        eprintln!("  Failed: {} zones", failed_zones.len());
        for (zone, error) in &failed_zones {
            eprintln!("    {}: {}", zone, error);
        }
    }

    Ok(())
}
