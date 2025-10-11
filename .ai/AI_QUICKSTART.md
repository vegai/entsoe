# AI Assistant Quick Start Guide

This document provides a rapid context loading guide for AI assistants working on the ENTSO-E Rust library project.

## Essential Context

### What is This Project?

A Rust library that fetches and parses electricity price data from the ENTSO-E (European Network of Transmission System Operators for Electricity) API.

**Primary Goal:** Fetch day-ahead electricity prices for the next 24+ hours and parse XML responses into Rust structs.

### Current Status

Project structure is complete. Core functionality needs implementation:
- Client implementation
- XML parser
- Models/types
- Tests

## Key Concepts

### ENTSO-E API Basics

**Base URL:** `https://web-api.transparency.entsoe.eu/api`

**Authentication:** Query parameter `securityToken=YOUR_TOKEN`

The API token is passed to the client constructor:
```rust
let client = EntsoeClient::new("your-token-here");
```

**Day-Ahead Prices Request:**
```
GET /api?documentType=A44
        &in_Domain=10Y1001A1001A82H  (bidding zone EIC code)
        &out_Domain=10Y1001A1001A82H
        &periodStart=202401150000     (YYYYMMDDhhmm UTC)
        &periodEnd=202401160000
        &securityToken=YOUR_TOKEN
```

**Response:** XML document with hourly price points

### Common Bidding Zones

| Zone | EIC Code |
|------|----------|
| Germany | `10Y1001A1001A82H` |
| France | `10YFR-RTE------C` |
| Norway NO2 | `10YNO-2--------T` |
| Sweden SE3 | `10Y1001A1001A46L` |

## Tech Stack

- **Async Runtime:** `tokio`
- **HTTP Client:** `reqwest`
- **XML Parsing:** `quick-xml`
- **Error Handling:** `thiserror`
- **DateTime:** `chrono`
- **Serialization:** `serde`

## Quick Commands

```bash
# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy

# Run example
cargo run --example fetch_prices

# Generate docs
cargo doc --open
```

## Coding Rules

1. **No panics** - Use `Result<T, E>` everywhere in library code
2. **Async first** - All I/O is async with `tokio`
3. **Document everything** - Every public item needs doc comments with examples
4. **Test everything** - Unit tests + integration tests + doc tests
5. **Follow Rust idioms** - Use standard patterns and conventions
6. **Minimal emojis** - Use emojis sparingly or not at all in documentation

## When You Need More Detail

- **Architecture & Design:** Read `PROJECT_OVERVIEW.md`
- **Coding Standards:** Read `DEVELOPMENT_GUIDE.md`
- **API Details:** Read `API_REFERENCE.md`
- **Common Tasks:** Read `PROMPTS.md`

## Typical Workflow

### Starting a Task

1. Read this file for context
2. Understand the specific task
3. Check relevant docs in `.ai/` if needed
4. Implement following the coding rules
5. Add tests
6. Update documentation
7. Run checks: `cargo test && cargo clippy && cargo fmt`

### Example: "Implement the price parser"

1. Create `src/parser/price_parser.rs`
2. Define parsing logic for ENTSO-E XML
3. Handle errors gracefully
4. Add tests with fixture XML in `tests/fixtures/`
5. Document the public API
6. Run `cargo test`

## Common Pitfalls to Avoid

1. Don't use `.unwrap()` or `.expect()` in library code
2. Don't make actual API calls in tests (use fixtures/mocks)
3. Don't forget UTC times (ENTSO-E API requires UTC)
4. Don't guess XML structure (check actual responses)
5. Don't forget to document public APIs

## Example Code Style

```rust
/// Fetches day-ahead electricity prices.
///
/// # Examples
///
/// ```no_run
/// # use entsoe::EntsoeClient;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = EntsoeClient::new("token");
/// let prices = client.fetch_prices("10Y1001A1001A82H").await?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns error if HTTP request fails or XML parsing fails.
pub async fn fetch_prices(&self, zone: &str) -> Result<PriceDocument, EntsoeError> {
    // Implementation
}
```

## Working with the User

- Ask clarifying questions if requirements are unclear
- Suggest alternatives when appropriate
- Explain trade-offs for different approaches
- Point out potential issues before they become problems

## Essential Resources

- [ENTSO-E Transparency Platform](https://transparency.entsoe.eu/)
- [API Documentation](https://transparency.entsoe.eu/content/static_content/Static%20content/web%20api/Guide.html)
- [Rust Async Book](https://rust-lang.github.io/async-book/)

## Pre-Commit Checklist

Before finalizing any code:

- [ ] Code compiles (`cargo build`)
- [ ] Tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Documentation updated
- [ ] Examples work

## Learning the Codebase

**Priority Order:**
1. Read this file (AI_QUICKSTART.md)
2. Skim README.md for user perspective
3. Read PROJECT_OVERVIEW.md for architecture
4. Browse API_REFERENCE.md for ENTSO-E API details
5. Check DEVELOPMENT_GUIDE.md when writing code
6. Use PROMPTS.md for common task patterns

---

**Remember:** All times in UTC, no unwraps in library code, test everything, document everything, minimal emojis.

**Documentation Guidelines:** Never document directory structures or file trees in the docs - they become stale immediately and are trivial to discover with `ls` or `find`.
