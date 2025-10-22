use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz;
use rusqlite::{Connection, Result as SqliteResult};
use rust_decimal::prelude::*;
use std::env;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Period {
    start: DateTime<Utc>,
    price: Decimal,
}

struct DisplayData {
    periods: Vec<Period>,
}

fn load_prices_from_db(
    conn: &Connection,
    price_area: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> SqliteResult<DisplayData> {
    let mut stmt = conn.prepare(
        "SELECT timestamp, price, currency FROM prices
         WHERE price_area = ?1 AND timestamp >= ?2 AND timestamp < ?3
         ORDER BY timestamp",
    )?;

    let from_str = from.to_rfc3339();
    let to_str = to.to_rfc3339();

    let mut rows = stmt.query([price_area, &from_str, &to_str])?;

    let mut periods = Vec::new();

    while let Some(row) = rows.next()? {
        let timestamp_str: String = row.get(0)?;
        let price_str: String = row.get(1)?;
        let _currency: String = row.get(2)?;

        // Parse timestamp
        let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?
            .with_timezone(&Utc);

        // Parse price (stored as EUR/kWh) and convert to cents/kWh
        let price_kwh = f64::from_str(&price_str).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e))
        })?;

        // Convert to cents/kWh for display (multiply by 100)
        let price_cents = Decimal::from_f64(price_kwh * 100.0).ok_or_else(|| {
            rusqlite::Error::FromSqlConversionFailure(
                1,
                rusqlite::types::Type::Text,
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid price",
                )),
            )
        })?;

        periods.push(Period {
            start: timestamp,
            price: price_cents,
        });
    }

    Ok(DisplayData { periods })
}

fn find_cheapest_consecutive_hours(periods: &[Period], n: usize) -> Option<(usize, Decimal)> {
    if periods.is_empty() || n == 0 {
        return None;
    }

    let actual_n = if n > periods.len() { periods.len() } else { n };
    let mut min_sum: Decimal = periods
        .iter()
        .take(actual_n)
        .map(|period| period.price)
        .sum();
    let mut min_index = 0;
    let mut current_sum = min_sum;

    for i in actual_n..periods.len() {
        current_sum += periods[i].price - periods[i - actual_n].price;

        if current_sum < min_sum {
            min_sum = current_sum;
            min_index = i + 1 - actual_n;
        }
    }

    Some((min_index, min_sum))
}

fn find_expensivest_consecutive_hours(periods: &[Period], n: usize) -> Option<(usize, Decimal)> {
    if periods.is_empty() || n == 0 {
        return None;
    }

    let actual_n = if n > periods.len() { periods.len() } else { n };
    let mut max_sum: Decimal = periods
        .iter()
        .take(actual_n)
        .map(|period| period.price)
        .sum();
    let mut max_index = 0;
    let mut current_sum = max_sum;

    for i in actual_n..periods.len() {
        current_sum += periods[i].price - periods[i - actual_n].price;

        if current_sum > max_sum {
            max_sum = current_sum;
            max_index = i + 1 - actual_n;
        }
    }

    Some((max_index, max_sum))
}

fn render_cheapest(
    periods: &[Period],
    n: usize,
) -> Option<(usize, DateTime<Utc>, DateTime<Utc>, Decimal)> {
    let (index, total_price) = find_cheapest_consecutive_hours(periods, n)?;

    // Divide by n to get average, already in cents
    let n_decimal = Decimal::from_usize(n)?;
    let avg_price = total_price / n_decimal;

    let time_start = periods[index].start;
    let time_end = periods[index].start + Duration::hours(n as i64);

    Some((n, time_start, time_end, avg_price))
}

fn render_expensivest(
    periods: &[Period],
    n: usize,
) -> Option<(usize, DateTime<Utc>, DateTime<Utc>, Decimal)> {
    let (index, total_price) = find_expensivest_consecutive_hours(periods, n)?;

    // Divide by n to get average, already in cents
    let n_decimal = Decimal::from_usize(n)?;
    let avg_price = total_price / n_decimal;

    let time_start = periods[index].start;
    let time_end = periods[index].start + Duration::hours(n as i64);

    Some((n, time_start, time_end, avg_price))
}

fn print_header(s: &str) {
    let sep: String = std::iter::repeat('━').take(s.len()).collect();
    println!("{}\n{}\n", s, sep);
}

fn print_info_header(price_area: &str, now: DateTime<Utc>, timezone: &Tz) {
    let time_format = "%Y-%m-%dT%H:%M:%S %Z";
    println!(
        "{} {}\n",
        price_area,
        now.with_timezone(timezone).format(time_format)
    );
}

fn print_price_md_table(
    prices: Vec<(usize, DateTime<Utc>, DateTime<Utc>, Decimal)>,
    timezone: &Tz,
) {
    let headers = vec!["n", "start", "end", "avg(¢/kWh)"];
    let time_format = "%a %H:%M";

    let mut max_widths = headers.iter().map(|h| h.len()).collect::<Vec<_>>();
    let mut table_data = vec![
        headers
            .iter()
            .map(|&h| h.to_string())
            .collect::<Vec<String>>(),
    ];

    for (n, start, end, price) in prices {
        let row = vec![
            n.to_string(),
            start
                .with_timezone(timezone)
                .format(time_format)
                .to_string(),
            end.with_timezone(timezone).format(time_format).to_string(),
            format!("{:.2}", price),
        ];

        for (i, cell) in row.iter().enumerate() {
            max_widths[i] = std::cmp::max(max_widths[i], cell.len());
        }

        table_data.push(row);
    }

    // Print top border
    println!(
        "┌{}┐",
        max_widths
            .iter()
            .map(|&width| "─".repeat(width + 2))
            .collect::<Vec<_>>()
            .join("┬")
    );

    for (i, row) in table_data.iter().enumerate() {
        // Print row with vertical dividers
        let row_str = row
            .iter()
            .enumerate()
            .map(|(j, cell)| format!(" {:width$} ", cell, width = max_widths[j]))
            .collect::<Vec<_>>()
            .join("│");
        println!("│{}│", row_str);

        // Print header separator after the first row
        if i == 0 {
            println!(
                "├{}┤",
                max_widths
                    .iter()
                    .map(|&width| "─".repeat(width + 2))
                    .collect::<Vec<_>>()
                    .join("┼")
            );
        }
    }

    // Print bottom border
    println!(
        "└{}┘",
        max_widths
            .iter()
            .map(|&width| "─".repeat(width + 2))
            .collect::<Vec<_>>()
            .join("┴")
    );
}

fn print_graph(periods: &[Period], timezone: &Tz) {
    if periods.is_empty() {
        eprintln!("No data to graph");
        return;
    }

    let start_time = periods[0].start;
    let end_time = periods[periods.len() - 1].start;
    let time_format = "%a %H:%M";

    let start_time_str = start_time.with_timezone(timezone).format(time_format);
    let end_time_str = end_time.with_timezone(timezone).format(time_format);

    // Convert Decimal to f64 for rasciigraph
    let converted_periods: Vec<f64> = periods
        .iter()
        .map(|p| p.price.to_f64().unwrap_or(0.0))
        .collect();

    println!(
        "{}",
        rasciigraph::plot(
            converted_periods,
            rasciigraph::Config::default()
                .with_offset(0)
                .with_height(7)
                .with_width(38)
                .with_caption(format!("¢/kWh ({} - {})", start_time_str, end_time_str))
        )
    );
}

fn parse_timezone(tz_str: &str) -> Result<Tz, String> {
    tz_str.parse::<Tz>().map_err(|_| {
        format!(
            "Invalid timezone: '{}'. Examples: UTC, Europe/Helsinki, Europe/Stockholm",
            tz_str
        )
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <DATABASE_PATH> <PRICE_AREA> [OPTIONS]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  DATABASE_PATH       Path to SQLite database file");
        eprintln!("  PRICE_AREA          Bidding zone (e.g., FI, NO2, SE3)");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --timezone TZ       Display timezone (default: UTC)");
        eprintln!("                      Examples: UTC, Europe/Helsinki, Europe/Stockholm");
        eprintln!("  --hours N           Hours to display from now (default: 24)");
        eprintln!("  --future            Show only future prices (default: show all in range)");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {} prices.db FI", args[0]);
        eprintln!(
            "  {} prices.db FI --timezone Europe/Helsinki --hours 48",
            args[0]
        );
        eprintln!("  {} prices.db NO2 --future", args[0]);
        std::process::exit(1);
    }

    let db_path = &args[1];
    let price_area = &args[2];

    // Parse optional arguments
    let mut timezone: Tz = Tz::UTC;
    let mut hours: i64 = 24;
    let mut future_only = false;

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--timezone" => {
                if i + 1 < args.len() {
                    timezone = parse_timezone(&args[i + 1])?;
                    i += 2;
                } else {
                    return Err("--timezone requires an argument".into());
                }
            }
            "--hours" => {
                if i + 1 < args.len() {
                    hours = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid hours value: '{}'", args[i + 1]))?;
                    i += 2;
                } else {
                    return Err("--hours requires an argument".into());
                }
            }
            "--future" => {
                future_only = true;
                i += 1;
            }
            _ => {
                return Err(format!("Unknown option: '{}'", args[i]).into());
            }
        }
    }

    eprintln!("Reading from database: {}", db_path);
    let conn = Connection::open(db_path)?;

    let now = Utc::now();
    let start_time = if future_only {
        now - Duration::minutes(15)
    } else {
        now - Duration::hours(hours) - Duration::minutes(15)
    };
    let end_time = now + Duration::hours(hours);

    eprintln!(
        "Loading prices for {} from {} to {}",
        price_area, start_time, end_time
    );

    let mut data = load_prices_from_db(&conn, price_area, start_time, end_time)?;

    if data.periods.is_empty() {
        eprintln!(
            "No price data found for {} in the specified time range",
            price_area
        );
        std::process::exit(1);
    }

    // Filter to future only if requested
    // Include periods from 15 minutes ago to capture current period
    if future_only {
        data.periods
            .retain(|p| p.start > now - Duration::minutes(15));
    }

    if data.periods.is_empty() {
        eprintln!("No future prices available for {}", price_area);
        std::process::exit(1);
    }

    eprintln!("Loaded {} price points\n", data.periods.len());

    // Display output
    print_info_header(price_area, now, &timezone);

    // Cheapest consecutive hours
    print_header("Cheapest consecutive n hours & average price");
    let mut cheapest: Vec<(usize, DateTime<Utc>, DateTime<Utc>, Decimal)> = Vec::new();
    for n in [1, 2, 3, 5, 8, 13] {
        if let Some(result) = render_cheapest(&data.periods, n) {
            cheapest.push(result);
        }
    }
    if !cheapest.is_empty() {
        print_price_md_table(cheapest, &timezone);
        println!();
    }

    // Most expensive consecutive hours
    print_header("Priciest consecutive n hours & average price");
    let mut expensivest: Vec<(usize, DateTime<Utc>, DateTime<Utc>, Decimal)> = Vec::new();
    for n in [1, 2, 3, 5, 8, 13] {
        if let Some(result) = render_expensivest(&data.periods, n) {
            expensivest.push(result);
        }
    }
    if !expensivest.is_empty() {
        print_price_md_table(expensivest, &timezone);
        println!();
    }

    // Graph
    print_header("Spot graph");
    print_graph(&data.periods, &timezone);

    Ok(())
}
