# ENTSO-E Rust Library

A Rust library for fetching and parsing data from the ENTSO-E (European Network of Transmission System Operators for Electricity) Transparency Platform API.

## Features

- Fetch day-ahead electricity prices for European bidding zones
- Parse ENTSO-E XML responses into strongly-typed Rust structs
- Async/await support with Tokio
- Type-safe error handling
- Comprehensive documentation and examples

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
    // Create client with your API token
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

See the [API Reference](.ai/API_REFERENCE.md) for a complete list.

## Examples

Check the `examples/` directory for more usage examples:

```bash
# Fetch day-ahead prices
cargo run --example fetch_prices
```

## Development

### Prerequisites

- Rust 1.70 or later
- An ENTSO-E API token

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Generate documentation
cargo doc --open
```

## Documentation

- **[Project Overview](.ai/PROJECT_OVERVIEW.md)** - Architecture and design
- **[Development Guide](.ai/DEVELOPMENT_GUIDE.md)** - Coding standards and workflows
- **[API Reference](.ai/API_REFERENCE.md)** - ENTSO-E API details
- **[Common Prompts](.ai/PROMPTS.md)** - AI assistant tasks and examples

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