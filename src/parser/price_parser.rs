use chrono::{DateTime, Duration, Utc};
use quick_xml::Reader;
use quick_xml::events::Event;

use crate::error::{EntsoeError, Result};
use crate::models::price::{PriceDocument, PricePoint, Resolution};

pub fn parse_day_ahead_prices(xml: &[u8]) -> Result<PriceDocument> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(true);

    let mut currency = None;
    let mut resolution = None;
    let mut period_start = None;
    let mut period_end = None;
    let mut all_points: Vec<(DateTime<Utc>, u32, f64)> = Vec::new();

    let mut in_time_series = false;
    let mut in_period = false;
    let mut in_point = false;
    let mut in_time_interval = false;

    let mut current_period_start = None;
    let mut current_position = None;
    let mut current_price = None;

    let mut current_tag = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = e.name();
                current_tag = String::from_utf8_lossy(name.as_ref()).to_string();

                match name.as_ref() {
                    b"TimeSeries" => in_time_series = true,
                    b"Period" if in_time_series => {
                        in_period = true;
                        current_period_start = None;
                    }
                    b"Point" if in_period => in_point = true,
                    b"timeInterval" if in_period => in_time_interval = true,
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let name = e.name();
                match name.as_ref() {
                    b"TimeSeries" => in_time_series = false,
                    b"Period" => {
                        in_period = false;
                        in_time_interval = false;
                    }
                    b"Point" => {
                        if let (Some(pos), Some(price), Some(start)) =
                            (current_position, current_price, current_period_start)
                        {
                            all_points.push((start, pos, price));
                        }
                        current_position = None;
                        current_price = None;
                        in_point = false;
                    }
                    b"timeInterval" => in_time_interval = false,
                    _ => {}
                }
                current_tag.clear();
            }
            Ok(Event::Text(e)) => {
                let text = std::str::from_utf8(&e)
                    .map_err(|e| EntsoeError::XmlParseError(format!("Invalid UTF-8: {}", e)))?;
                let text = text.trim();

                if text.is_empty() {
                    continue;
                }

                match current_tag.as_str() {
                    "currency_Unit.name" if in_time_series => {
                        if currency.is_none() {
                            currency = Some(text.to_string());
                        }
                    }
                    "resolution" if in_period => {
                        if resolution.is_none() {
                            resolution = Resolution::parse(text);
                        }
                    }
                    "start" if in_time_interval && in_period => {
                        let text_with_seconds =
                            if !text.contains(':') || text.matches(':').count() == 1 {
                                format!("{}:00", text.trim_end_matches('Z')) + "Z"
                            } else {
                                text.to_string()
                            };
                        let dt = DateTime::parse_from_rfc3339(&text_with_seconds)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc));
                        current_period_start = dt;
                        if let Some(dt_val) = dt
                            && (period_start.is_none() || period_start.unwrap() > dt_val)
                        {
                            period_start = Some(dt_val);
                        }
                    }
                    "end" if in_time_interval && in_period => {
                        let text_with_seconds =
                            if !text.contains(':') || text.matches(':').count() == 1 {
                                format!("{}:00", text.trim_end_matches('Z')) + "Z"
                            } else {
                                text.to_string()
                            };
                        let dt = DateTime::parse_from_rfc3339(&text_with_seconds)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc));
                        if let Some(dt_val) = dt
                            && (period_end.is_none() || period_end.unwrap() < dt_val)
                        {
                            period_end = Some(dt_val);
                        }
                    }
                    "position" if in_point => {
                        current_position = text.parse().ok();
                    }
                    "price.amount" if in_point => {
                        current_price = text.parse().ok();
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(EntsoeError::XmlParseError(format!(
                    "XML parsing error: {}",
                    e
                )));
            }
            _ => {}
        }
        buf.clear();
    }

    let currency =
        currency.ok_or_else(|| EntsoeError::MissingField("currency_Unit.name".to_string()))?;
    let resolution =
        resolution.ok_or_else(|| EntsoeError::MissingField("resolution".to_string()))?;
    let period_start =
        period_start.ok_or_else(|| EntsoeError::MissingField("period start".to_string()))?;
    let period_end =
        period_end.ok_or_else(|| EntsoeError::MissingField("period end".to_string()))?;

    if all_points.is_empty() {
        return Err(EntsoeError::XmlParseError(
            "No price points found".to_string(),
        ));
    }

    let mut prices: Vec<PricePoint> = all_points
        .into_iter()
        .map(|(start, position, price)| {
            let offset_minutes = (position as i64 - 1) * resolution.minutes();
            let timestamp = start + Duration::minutes(offset_minutes);
            PricePoint { timestamp, price }
        })
        .collect();

    prices.sort_by_key(|p| p.timestamp);

    Ok(PriceDocument {
        currency,
        resolution,
        period_start,
        period_end,
        prices,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_day_ahead_prices_fixture() {
        let xml = include_bytes!("../../tests/fixtures/day_ahead_prices_fi.xml");
        let result = parse_day_ahead_prices(xml);

        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        let doc = result.unwrap();

        assert_eq!(doc.currency, "EUR");
        assert_eq!(doc.resolution, Resolution::PT15M);
        assert!(!doc.prices.is_empty());

        let first_price = &doc.prices[0];
        assert!(first_price.price > 0.0);
    }

    #[test]
    fn test_parse_empty_xml() {
        let xml = b"<?xml version=\"1.0\"?><root></root>";
        let result = parse_day_ahead_prices(xml);
        assert!(result.is_err());
    }
}
