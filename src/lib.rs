//! Rust library for fetching and parsing data from the ENTSO-E Transparency Platform API.
//!
//! Get an API token at <https://transparency.entsoe.eu/>

#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]

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
