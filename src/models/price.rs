use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct PricePoint {
    pub timestamp: DateTime<Utc>,
    pub price: f64,
}

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
}
