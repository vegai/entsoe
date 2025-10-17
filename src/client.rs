use chrono::{DateTime, Utc};
use reqwest::Client;
use url::Url;

use crate::bidding_zone::BiddingZone;
use crate::error::{EntsoeError, Result};
use crate::models::PriceDocument;
use crate::parser::parse_day_ahead_prices;

const API_BASE_URL: &str = "https://web-api.tp.entsoe.eu/api";

/// Client for interacting with the ENTSO-E Transparency Platform API.
pub struct EntsoeClient {
    api_token: String,
    http_client: Client,
}

impl EntsoeClient {
    pub fn new(api_token: impl Into<String>) -> Self {
        Self {
            api_token: api_token.into(),
            http_client: Client::new(),
        }
    }

    /// Fetches day-ahead prices as raw XML bytes. Times must be in UTC.
    ///
    /// # Errors
    ///
    /// Returns error if the HTTP request fails, URL construction fails, or time range is invalid.
    pub async fn fetch_day_ahead_prices(
        &self,
        bidding_zone: BiddingZone,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<bytes::Bytes> {
        if period_start >= period_end {
            return Err(EntsoeError::InvalidTimeRange(
                "period_start must be before period_end".to_string(),
            ));
        }

        let url = self.build_day_ahead_prices_url(bidding_zone, period_start, period_end)?;

        let response = self.http_client.get(url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(EntsoeError::ApiError(format!(
                "API returned status {status}: {body}"
            )));
        }

        let bytes = response.bytes().await?;
        Ok(bytes)
    }

    /// Fetches and parses day-ahead prices. Times must be in UTC.
    ///
    /// # Errors
    ///
    /// Returns error if the HTTP request fails, XML parsing fails, or time range is invalid.
    pub async fn get_day_ahead_prices(
        &self,
        bidding_zone: BiddingZone,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<PriceDocument> {
        let xml = self
            .fetch_day_ahead_prices(bidding_zone, period_start, period_end)
            .await?;
        parse_day_ahead_prices(&xml)
    }

    fn build_day_ahead_prices_url(
        &self,
        bidding_zone: BiddingZone,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Url> {
        let mut url = Url::parse(API_BASE_URL)?;

        {
            let mut query = url.query_pairs_mut();
            query.append_pair("documentType", "A44");
            query.append_pair("in_Domain", bidding_zone.eic_code());
            query.append_pair("out_Domain", bidding_zone.eic_code());
            query.append_pair("periodStart", &format_timestamp(period_start));
            query.append_pair("periodEnd", &format_timestamp(period_end));
            query.append_pair("securityToken", &self.api_token);
        }

        Ok(url)
    }
}

fn format_timestamp(dt: DateTime<Utc>) -> String {
    dt.format("%Y%m%d%H%M").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_format_timestamp() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 14, 30, 0).unwrap();
        assert_eq!(format_timestamp(dt), "202401151430");
    }

    #[test]
    fn test_build_url() {
        let client = EntsoeClient::new("test-token");
        let start = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 16, 0, 0, 0).unwrap();

        let url = client
            .build_day_ahead_prices_url(BiddingZone::DE, start, end)
            .unwrap();

        let url_str = url.as_str();
        assert!(url_str.contains("documentType=A44"));
        assert!(url_str.contains("in_Domain=10Y1001A1001A82H"));
        assert!(url_str.contains("out_Domain=10Y1001A1001A82H"));
        assert!(url_str.contains("periodStart=202401150000"));
        assert!(url_str.contains("periodEnd=202401160000"));
        assert!(url_str.contains("securityToken=test-token"));
    }

    #[test]
    fn test_invalid_time_range() {
        use tokio::runtime::Runtime;

        let rt = Runtime::new().unwrap();
        let client = EntsoeClient::new("test-token");
        let start = Utc.with_ymd_and_hms(2024, 1, 16, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let result = rt.block_on(client.fetch_day_ahead_prices(BiddingZone::DE, start, end));

        assert!(result.is_err());
        match result {
            Err(EntsoeError::InvalidTimeRange(_)) => {}
            _ => panic!("Expected InvalidTimeRange error"),
        }
    }
}
