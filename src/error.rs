use thiserror::Error;

#[derive(Debug, Error)]
pub enum EntsoeError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Failed to build URL: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Invalid bidding zone code: {0}")]
    InvalidBiddingZone(String),

    #[error("Invalid time range: {0}")]
    InvalidTimeRange(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Failed to parse XML: {0}")]
    XmlParseError(String),

    #[error("Missing required field: {0}")]
    MissingField(String),
}

pub type Result<T> = std::result::Result<T, EntsoeError>;
