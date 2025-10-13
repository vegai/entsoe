//! ENTSO-E Rust Library
//!
//! A Rust library for fetching and parsing data from the ENTSO-E (European Network
//! of Transmission System Operators for Electricity) Transparency Platform API.
//!
//! # Overview
//!
//! This library provides a client for interacting with the ENTSO-E API to fetch
//! electricity market data, starting with day-ahead prices. Perhaps other API endpoints
//! later.
//!
//! # Features
//!
//! - Fetch day-ahead electricity prices for European bidding zones
//! - Async/await support with Tokio
//!
//! # Examples
//!
//! ```no_run
//! use entsoe::{EntsoeClient, BiddingZone};
//! use chrono::{Utc, Duration};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = EntsoeClient::new("your-api-token");
//! let start = Utc::now();
//! let end = start + Duration::hours(24);
//!
//! let xml_data = client.fetch_day_ahead_prices(BiddingZone::FI, start, end).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Getting an API Token
//!
//! 1. Register at <https://transparency.entsoe.eu/>
//! 2. Navigate to "My Account Settings"
//! 3. Generate a Web API Security Token

pub mod bidding_zone;
pub mod client;
pub mod error;
pub mod models;
pub mod parser;

pub use bidding_zone::BiddingZone;
pub use client::EntsoeClient;
pub use error::{EntsoeError, Result};
pub use models::{PriceDocument, PricePoint, Resolution};
pub use parser::parse_day_ahead_prices;
