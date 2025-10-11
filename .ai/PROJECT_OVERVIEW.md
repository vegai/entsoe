# ENTSO-E Rust Library - Project Overview

## Project Purpose

This is a Rust library (`entsoe`) that provides a client for fetching and parsing data from the ENTSO-E (European Network of Transmission System Operators for Electricity) Transparency Platform API.

**Primary Goal:** Fetch electricity prices for the next 24 hours and parse the XML response into Rust structs.

## What is ENTSO-E?

ENTSO-E is the European Network of Transmission System Operators for Electricity. They provide a Transparency Platform that publishes electricity market data across Europe, including:
- Day-ahead electricity prices
- Generation forecasts
- Load data
- Cross-border flows
- And much more

Website: https://entsoe.eu
API Documentation: https://transparency.entsoe.eu/content/static_content/Static%20content/web%20api/Guide.html

## Architecture Overview

The library is organized into several key components:

**Client** - HTTP client wrapper for ENTSO-E API. Handles authentication with API token passed at construction time, constructs proper URLs with query parameters, and makes requests returning raw XML.

**Models** - Data structures representing ENTSO-E concepts. Includes `Price` (single price point with timestamp + value + currency), `PriceDocument` (collection of prices with metadata), `BiddingZone` (European electricity market zones), and common types for timestamps, currencies, and units.

**Parser** - Parses ENTSO-E XML responses using their specific schema. Converts XML to strongly-typed Rust structs and handles errors gracefully for malformed XML or missing fields.

**Error Handling** - Custom error types covering HTTP errors, parsing errors, and validation errors with proper context for debugging.

## ENTSO-E API Basics

### Authentication
- Requires a security token
- Obtained by registering at: https://transparency.entsoe.eu/
- Token is passed to the client constructor: `EntsoeClient::new(token)`
- Token is sent as a query parameter in requests: `securityToken=YOUR_TOKEN`

### Day-Ahead Prices Endpoint
```
GET https://web-api.transparency.entsoe.eu/api
```

**Required Parameters:**
- `documentType=A44` (Price Document)
- `in_Domain` or `out_Domain` - Bidding zone (e.g., "10Y1001A1001A82H" for Germany)
- `periodStart` - Start time in format: YYYYMMDDhhmm (UTC)
- `periodEnd` - End time in format: YYYYMMDDhhmm (UTC)
- `securityToken` - Your API token

### Response Format
XML document following the IEC 62325 standard, containing:
- TimeSeries with price points
- Each point has a position (index) and price value
- Resolution (typically PT60M for hourly prices)
- Currency (EUR, SEK, NOK, etc.)

## Design Principles

1. **Type Safety** - Leverage Rust's type system to prevent invalid states
2. **Error Handling** - Use `Result<T, E>` everywhere, no panics in library code
3. **Async First** - Use `tokio` and `async/await` for HTTP requests
4. **Minimal Dependencies** - Only include what's necessary
5. **Well Tested** - Unit tests, integration tests, and doc tests
6. **Documentation** - Every public API must have doc comments with examples

## Dependencies

- `tokio` - Async runtime
- `reqwest` - HTTP client
- `serde` - Serialization/deserialization
- `quick-xml` - XML parsing
- `chrono` - Date and time handling
- `thiserror` - Error handling
- `url` - URL construction

## Testing Strategy

1. **Unit Tests** - Test parsers with fixture XML files
2. **Integration Tests** - Test full flow (requires API token for live tests)
3. **Doc Tests** - Examples in documentation must compile and run
4. **Mocking** - Mock HTTP responses for reliable tests without API calls

## Notes for AI Assistants

- ENTSO-E uses specific XML schemas - don't guess the structure, refer to actual API responses
- Bidding zones have specific EIC codes (Energy Identification Codes) - these are standardized
- Times are always in UTC
- The API has rate limits - be considerate in tests
- Price values are typically in EUR/MWh but can vary by zone
- Some zones may not have price data available
- Use minimal emojis in documentation and code
- Keep documentation focused and avoid redundant information that can become stale
- Never document directory structures or file trees - they become stale immediately and are easy to discover with `ls` or `find`