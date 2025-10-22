# ENTSO-E Rust Library

A Rust library for fetching and parsing data from the ENTSO-E (European Network of Transmission System Operators for Electricity) Transparency Platform API.

## Features

- Fetch day-ahead electricity prices for European bidding zones
- Parse ENTSO-E XML responses into Rust structs

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
entsoe = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### Getting an API Token

1. Register at [ENTSO-E Transparency Platform](https://transparency.entsoe.eu/)
2. Navigate to "My Account Settings"
3. Generate a Web API Security Token

### Fetching Day-Ahead Prices

```rust
use entsoe::EntsoeClient;
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = EntsoeClient::new("your-api-token-here");

    // Fetch prices for Germany for the next 24 hours
    let start = Utc::now();
    let end = start + Duration::hours(24);
    
    let prices = client
        .fetch_day_ahead_prices("10Y1001A1001A82H", start, end)
        .await?;

    // Display the prices
    for price in prices.prices {
        println!("{}: {:.2} {}", 
            price.timestamp, 
            price.value, 
            prices.currency
        );
    }

    Ok(())
}
```

## Supported Bidding Zones

The library supports all European bidding zones. Common ones include:

| Zone | Code | EIC Code |
|------|------|----------|
| Germany | DE | `10Y1001A1001A82H` |
| France | FR | `10YFR-RTE------C` |
| Norway NO2 | NO2 | `10YNO-2--------T` |
| Sweden SE3 | SE3 | `10Y1001A1001A46L` |
| Netherlands | NL | `10YNL----------L` |

Use the `BiddingZone` enum to see all supported zones.

## Examples

Check the `examples/` directory for more usage examples:

```bash
# Fetch day-ahead prices
cargo run --example fetch_prices
```

## CLI Tools

Three command-line tools are included for working with electricity prices:

1. **`entsoe-fetch`** - Fetches prices from ENTSO-E API and stores in SQLite
2. **`entsoe-csv`** - Exports prices from SQLite database to CSV
3. **`entsoe-ascii`** - Displays prices as ASCII graphs and analysis tables

```bash
# Build the tools
cargo build --release --bins

# Fetch prices and store in database (fetch all zones!)
export ENTSOE_API_TOKEN="your-token"
target/release/entsoe-fetch prices.db

# Or fetch a specific zone
target/release/entsoe-fetch prices.db FI 48

# Export to CSV
target/release/entsoe-csv prices.db > all_prices.csv
target/release/entsoe-csv prices.db FI > finland_prices.csv

# Display ASCII visualization
target/release/entsoe-ascii prices.db FI --timezone Europe/Helsinki
```

The database-backed approach allows you to:
- Fetch all European zones with a single command
- Fetch data once, export many times
- Query directly with SQL
- Accumulate historical data
- Run exports without API calls

### entsoe-ascii Options

The ASCII visualization tool supports:
- `--timezone TZ` - Display in any IANA timezone (e.g., Europe/Helsinki, Europe/Oslo)
- `--hours N` - Number of hours to display (default: 24)
- `--future` - Show only future prices with historical context

It displays:
- Cheapest consecutive hours for optimal energy consumption
- Most expensive consecutive hours to avoid
- ASCII graph showing price trends
- Detailed price table in compact hourly format

## Development

### Prerequisites

- Rust 1.70 or later
- An ENTSO-E API token


## Important Notes

**Currency**: Day-ahead prices are in EUR/MWh for all zones (ENTSO-E standard). This includes non-Eurozone countries like Sweden, Norway, Denmark, Switzerland, and Poland.

## Resources

- [ENTSO-E Transparency Platform](https://transparency.entsoe.eu/)
- [ENTSO-E API Documentation](https://transparency.entsoe.eu/content/static_content/Static%20content/web%20api/Guide.html)
- [Energy Identification Codes](https://www.entsoe.eu/data/energy-identification-codes-eic/)

## License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0).

See [LICENSE](LICENSE) for details.



## Acknowledgments

This project uses data from the ENTSO-E Transparency Platform. ENTSO-E is the European Network of Transmission System Operators for Electricity.

---

**Note:** This library is not officially affiliated with or endorsed by ENTSO-E.