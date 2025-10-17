use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct PricePoint {
    pub timestamp: DateTime<Utc>,
    pub price: f64,
}

impl PricePoint {
    /// Returns the price in EUR/kWh (converted from EUR/MWh).
    ///
    /// The ENTSO-E API returns prices in EUR/MWh, this method converts to EUR/kWh
    /// by dividing by 1000.
    #[must_use]
    pub fn price_per_kwh(&self) -> f64 {
        self.price / 1000.0
    }
}

/// Document containing electricity price data from ENTSO-E.
///
/// Day-ahead prices are in EUR/MWh for all zones (ENTSO-E standard).
/// Use `PricePoint::price_per_kwh()` to convert to EUR/kWh.
#[derive(Debug, Clone, PartialEq)]
pub struct PriceDocument {
    pub currency: String,
    pub resolution: Resolution,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub prices: Vec<PricePoint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    PT15M,
    PT60M,
}

impl Resolution {
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "PT15M" => Some(Resolution::PT15M),
            "PT60M" => Some(Resolution::PT60M),
            _ => None,
        }
    }

    #[must_use]
    pub fn minutes(&self) -> i64 {
        match self {
            Resolution::PT15M => 15,
            Resolution::PT60M => 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolution_parse() {
        assert_eq!(Resolution::parse("PT15M"), Some(Resolution::PT15M));
        assert_eq!(Resolution::parse("PT60M"), Some(Resolution::PT60M));
        assert_eq!(Resolution::parse("INVALID"), None);
    }

    #[test]
    fn test_resolution_minutes() {
        assert_eq!(Resolution::PT15M.minutes(), 15);
        assert_eq!(Resolution::PT60M.minutes(), 60);
    }

    #[test]
    fn test_price_per_kwh() {
        use chrono::Utc;
        let price_point = PricePoint {
            timestamp: Utc::now(),
            price: 50.0,
        };
        assert!((price_point.price_per_kwh() - 0.05).abs() < 0.000001);
    }
}
