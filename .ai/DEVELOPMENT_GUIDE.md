# ENTSO-E Development Guide

## Getting Started

### Prerequisites
- Rust 1.70+ (2021 edition)
- An ENTSO-E API token (get one at https://transparency.entsoe.eu/)
- Basic understanding of async Rust

### Initial Setup

1. Clone the repository
2. Build the project:
   ```bash
   cargo build
   ```
3. Run tests:
   ```bash
   cargo test
   ```

## Coding Standards

### General Principles

1. **Follow Rust idioms** - Use the standard library and common patterns
2. **Prefer immutability** - Only use `mut` when necessary
3. **Explicit over implicit** - Don't rely on type inference where it reduces clarity
4. **Document everything public** - Every public item needs doc comments with examples
5. **Test everything** - Write tests as you write code
6. **Minimal emojis** - Use emojis sparingly in documentation and avoid in code comments

### Naming Conventions

- **Types**: `PascalCase` (e.g., `PriceDocument`, `BiddingZone`)
- **Functions**: `snake_case` (e.g., `fetch_prices`, `parse_xml`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_TIMEOUT`, `API_BASE_URL`)
- **Lifetimes**: Short, descriptive (e.g., `'a`, `'de`)
- **Generics**: Descriptive (e.g., `T`, `E`, or `Client`, `Error` when clearer)

### Error Handling

- Use `thiserror` for defining error types
- Every error should have a descriptive message
- Include context in errors (what was being done when it failed)
- Never use `.unwrap()` or `.expect()` in library code
- Use `?` operator for propagating errors

Example:
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EntsoeError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Failed to parse XML: {0}")]
    ParseError(String),
    
    #[error("Invalid bidding zone: {0}")]
    InvalidBiddingZone(String),
    
    #[error("API authentication failed")]
    AuthenticationError,
}
```

### Async Code

- Use `tokio` as the async runtime
- All I/O operations should be async
- Use `async fn` where possible, `impl Future` when needed
- Don't block the async runtime (no `std::thread::sleep` or blocking I/O)

### Documentation

Every public item must have:
1. A summary line (one sentence)
2. More detailed description if needed
3. Example usage in a `# Examples` section
4. Notes about errors in `# Errors` section
5. Panic conditions in `# Panics` section (if any)

Example:
```rust
/// Fetches day-ahead electricity prices for a specific bidding zone.
///
/// Returns hourly prices for the specified time period. Times must be in UTC.
///
/// # Arguments
///
/// * `zone` - The bidding zone code (e.g., "DE-LU", "NO-2")
/// * `start` - Start of the period (inclusive)
/// * `end` - End of the period (exclusive)
///
/// # Examples
///
/// ```no_run
/// # use entsoe::{EntsoeClient, BiddingZone};
/// # use chrono::Utc;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = EntsoeClient::new("your-token");
/// let start = Utc::now();
/// let end = start + chrono::Duration::hours(24);
/// let prices = client.fetch_day_ahead_prices("DE-LU", start, end).await?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The HTTP request fails
/// - The API returns an error response
/// - The XML response cannot be parsed
/// - The bidding zone is invalid
pub async fn fetch_day_ahead_prices(
    &self,
    zone: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<PriceDocument, EntsoeError> {
    // implementation
}
```

## Testing

### Unit Tests

- Place unit tests in the same file as the code, in a `#[cfg(test)]` module
- Test one behavior per test
- Use descriptive test names: `test_parse_valid_price_document`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bidding_zone_from_str_valid() {
        let zone = BiddingZone::from_str("DE-LU").unwrap();
        assert_eq!(zone.code(), "DE-LU");
    }

    #[test]
    fn test_bidding_zone_from_str_invalid() {
        let result = BiddingZone::from_str("INVALID");
        assert!(result.is_err());
    }
}
```

### Integration Tests

- Place in `tests/` directory
- Test full workflows (fetch + parse)
- Use fixtures for predictable testing
- Mock HTTP responses when possible

### Test Fixtures

- Store sample XML responses in `tests/fixtures/`
- Use real API responses (anonymized if needed)
- Name files descriptively: `day_ahead_prices_de_2024_01_15.xml`

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_parse_valid_price_document

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test integration_tests

# Ignore tests that require API token
cargo test -- --skip requires_api_token
```

## Code Quality Checklist

Before committing:
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Examples compile and work
- [ ] Public APIs have doc comments
- [ ] Error handling is proper (no unwrap/panic)
- [ ] New dependencies are justified

## Common Tasks

### Adding a New Data Type

1. Create model in `src/models/`
2. Create parser in `src/parser/`
3. Add fetch method to client
4. Write unit tests with fixture
5. Add integration test
6. Create example in `examples/`
7. Update documentation

### Adding a New Error Type

1. Add variant to `EntsoeError` in `src/error.rs`
2. Use `#[from]` for automatic conversion when possible
3. Add test for error case
4. Update documentation

### Debugging API Issues

1. Enable debug logging:
   ```bash
   RUST_LOG=debug cargo run --example fetch_prices
   ```
2. Check the raw XML response
3. Verify parameters match API documentation
4. Check bidding zone codes (EIC codes)
5. Ensure times are in UTC and properly formatted

## Dependencies Management

### Adding a Dependency

```bash
cargo add <crate-name>
```

### When to Add a Dependency

Add when:
- It's a well-maintained, popular crate
- It saves significant development time
- It's the standard solution for the problem

Don't add when:
- You can implement it simply yourself
- The crate is unmaintained
- It has excessive transitive dependencies

## Performance Considerations

1. **Avoid unnecessary allocations** - Use references where possible
2. **Parse lazily** - Only parse what's needed
3. **Stream large responses** - Don't load everything into memory
4. **Cache when appropriate** - Reuse clients, connection pools
5. **Profile before optimizing** - Measure, don't guess

## Git Workflow

### Commit Messages

Follow conventional commits:
```
<type>: <description>

[optional body]

[optional footer]
```

Types:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation only
- `test:` - Adding tests
- `refactor:` - Code change that neither fixes a bug nor adds a feature
- `perf:` - Performance improvement
- `chore:` - Maintenance tasks

Examples:
```
feat: add support for day-ahead price fetching

fix: handle missing currency field in XML response

docs: update README with installation instructions
```

## Troubleshooting

### "API returns 401 Unauthorized"
- Verify the API token is correct
- Check that token is valid on ENTSO-E website
- Ensure token is passed correctly to client constructor

### "Cannot parse XML"
- Save the raw XML response to a file and inspect it
- Compare against known good responses in `tests/fixtures/`
- Check API documentation for schema changes

### "No data available for bidding zone"
- Verify the bidding zone code (EIC code) is correct
- Check that data is available for that zone and time period
- Some zones don't publish certain data types

## Resources

- [ENTSO-E Transparency Platform](https://transparency.entsoe.eu/)
- [API Guide](https://transparency.entsoe.eu/content/static_content/Static%20content/web%20api/Guide.html)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)