use std::fmt;

/// European electricity bidding zones with their EIC codes.
///
/// Bidding zones are areas within the European electricity market where
/// a single electricity price applies. Each zone has a unique Energy
/// Identification Code (EIC).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BiddingZone {
    DE,
    AT,
    BE,
    DK1,
    DK2,
    FI,
    FR,
    ITNorth,
    NL,
    NO1,
    NO2,
    NO3,
    NO4,
    NO5,
    PL,
    ES,
    SE1,
    SE2,
    SE3,
    SE4,
    CH,
    GB,
}

impl BiddingZone {
    /// Returns the EIC code for this bidding zone.
    ///
    /// # Examples
    ///
    /// ```
    /// use entsoe::BiddingZone;
    ///
    /// assert_eq!(BiddingZone::FI.eic_code(), "10YFI-1--------U");
    /// assert_eq!(BiddingZone::NO2.eic_code(), "10YNO-2--------T");
    /// ```
    pub fn eic_code(&self) -> &'static str {
        match self {
            BiddingZone::DE => "10Y1001A1001A82H",
            BiddingZone::AT => "10YAT-APG------L",
            BiddingZone::BE => "10YBE----------2",
            BiddingZone::DK1 => "10YDK-1--------W",
            BiddingZone::DK2 => "10YDK-2--------M",
            BiddingZone::FI => "10YFI-1--------U",
            BiddingZone::FR => "10YFR-RTE------C",
            BiddingZone::ITNorth => "10Y1001A1001A73I",
            BiddingZone::NL => "10YNL----------L",
            BiddingZone::NO1 => "10YNO-1--------2",
            BiddingZone::NO2 => "10YNO-2--------T",
            BiddingZone::NO3 => "10YNO-3--------J",
            BiddingZone::NO4 => "10YNO-4--------9",
            BiddingZone::NO5 => "10Y1001A1001A48H",
            BiddingZone::PL => "10YPL-AREA-----S",
            BiddingZone::ES => "10YES-REE------0",
            BiddingZone::SE1 => "10Y1001A1001A44P",
            BiddingZone::SE2 => "10Y1001A1001A45N",
            BiddingZone::SE3 => "10Y1001A1001A46L",
            BiddingZone::SE4 => "10Y1001A1001A47J",
            BiddingZone::CH => "10YCH-SWISSGRIDZ",
            BiddingZone::GB => "10YGB----------A",
        }
    }

    /// Parses a bidding zone from a string code.
    ///
    /// Accepts both uppercase and lowercase codes.
    ///
    /// # Examples
    ///
    /// ```
    /// use entsoe::BiddingZone;
    ///
    /// assert_eq!(BiddingZone::from_code("FI"), Some(BiddingZone::FI));
    /// assert_eq!(BiddingZone::from_code("no2"), Some(BiddingZone::NO2));
    /// assert_eq!(BiddingZone::from_code("se3"), Some(BiddingZone::SE3));
    /// assert_eq!(BiddingZone::from_code("INVALID"), None);
    /// ```
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_uppercase().as_str() {
            "DE" => Some(BiddingZone::DE),
            "AT" => Some(BiddingZone::AT),
            "BE" => Some(BiddingZone::BE),
            "DK1" => Some(BiddingZone::DK1),
            "DK2" => Some(BiddingZone::DK2),
            "FI" => Some(BiddingZone::FI),
            "FR" => Some(BiddingZone::FR),
            "IT-NORTH" | "ITNORTH" => Some(BiddingZone::ITNorth),
            "NL" => Some(BiddingZone::NL),
            "NO1" => Some(BiddingZone::NO1),
            "NO2" => Some(BiddingZone::NO2),
            "NO3" => Some(BiddingZone::NO3),
            "NO4" => Some(BiddingZone::NO4),
            "NO5" => Some(BiddingZone::NO5),
            "PL" => Some(BiddingZone::PL),
            "ES" => Some(BiddingZone::ES),
            "SE1" => Some(BiddingZone::SE1),
            "SE2" => Some(BiddingZone::SE2),
            "SE3" => Some(BiddingZone::SE3),
            "SE4" => Some(BiddingZone::SE4),
            "CH" => Some(BiddingZone::CH),
            "GB" => Some(BiddingZone::GB),
            _ => None,
        }
    }

    /// Returns the short code for this bidding zone.
    ///
    /// # Examples
    ///
    /// ```
    /// use entsoe::BiddingZone;
    ///
    /// assert_eq!(BiddingZone::FI.code(), "FI");
    /// assert_eq!(BiddingZone::NO2.code(), "NO2");
    /// ```
    pub fn code(&self) -> &'static str {
        match self {
            BiddingZone::DE => "DE",
            BiddingZone::AT => "AT",
            BiddingZone::BE => "BE",
            BiddingZone::DK1 => "DK1",
            BiddingZone::DK2 => "DK2",
            BiddingZone::FI => "FI",
            BiddingZone::FR => "FR",
            BiddingZone::ITNorth => "IT-North",
            BiddingZone::NL => "NL",
            BiddingZone::NO1 => "NO1",
            BiddingZone::NO2 => "NO2",
            BiddingZone::NO3 => "NO3",
            BiddingZone::NO4 => "NO4",
            BiddingZone::NO5 => "NO5",
            BiddingZone::PL => "PL",
            BiddingZone::ES => "ES",
            BiddingZone::SE1 => "SE1",
            BiddingZone::SE2 => "SE2",
            BiddingZone::SE3 => "SE3",
            BiddingZone::SE4 => "SE4",
            BiddingZone::CH => "CH",
            BiddingZone::GB => "GB",
        }
    }
}

impl fmt::Display for BiddingZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eic_codes() {
        assert_eq!(BiddingZone::FI.eic_code(), "10YFI-1--------U");
        assert_eq!(BiddingZone::NO2.eic_code(), "10YNO-2--------T");
        assert_eq!(BiddingZone::DE.eic_code(), "10Y1001A1001A82H");
    }

    #[test]
    fn test_from_code() {
        assert_eq!(BiddingZone::from_code("FI"), Some(BiddingZone::FI));
        assert_eq!(BiddingZone::from_code("fi"), Some(BiddingZone::FI));
        assert_eq!(BiddingZone::from_code("NO2"), Some(BiddingZone::NO2));
        assert_eq!(BiddingZone::from_code("no2"), Some(BiddingZone::NO2));
        assert_eq!(BiddingZone::from_code("SE3"), Some(BiddingZone::SE3));
        assert_eq!(BiddingZone::from_code("INVALID"), None);
    }

    #[test]
    fn test_code() {
        assert_eq!(BiddingZone::FI.code(), "FI");
        assert_eq!(BiddingZone::NO2.code(), "NO2");
        assert_eq!(BiddingZone::SE3.code(), "SE3");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", BiddingZone::FI), "FI");
        assert_eq!(format!("{}", BiddingZone::NO2), "NO2");
    }
}
