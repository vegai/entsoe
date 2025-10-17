use rusqlite::{Connection, Result as SqliteResult};
use std::env;
use std::io::{self, Write};

fn export_to_csv(conn: &Connection, price_area: Option<&str>) -> SqliteResult<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "timestamp,price_per_kwh,currency,price_area")
        .expect("Failed to write header");

    if let Some(area) = price_area {
        let mut stmt = conn.prepare(
            "SELECT timestamp, price, currency, price_area FROM prices WHERE price_area = ?1 ORDER BY timestamp"
        )?;
        let mut rows = stmt.query([area])?;

        while let Some(row) = rows.next()? {
            let timestamp: String = row.get(0)?;
            let price: String = row.get(1)?;
            let currency: String = row.get(2)?;
            let price_area: String = row.get(3)?;

            writeln!(
                handle,
                "{},{},{},{}",
                timestamp, price, currency, price_area
            )
            .expect("Failed to write row");
        }
    } else {
        let mut stmt = conn.prepare(
            "SELECT timestamp, price, currency, price_area FROM prices ORDER BY timestamp, price_area"
        )?;
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            let timestamp: String = row.get(0)?;
            let price: String = row.get(1)?;
            let currency: String = row.get(2)?;
            let price_area: String = row.get(3)?;

            writeln!(
                handle,
                "{},{},{},{}",
                timestamp, price, currency, price_area
            )
            .expect("Failed to write row");
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <DATABASE_PATH> [PRICE_AREA]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  DATABASE_PATH       Path to SQLite database file");
        eprintln!("  PRICE_AREA          Optional: filter by price area (e.g., FI, NO2)");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {} prices.db           # Export all prices", args[0]);
        eprintln!("  {} prices.db FI        # Export only FI prices", args[0]);
        eprintln!("  {} prices.db > out.csv # Save to file", args[0]);
        eprintln!();
        eprintln!(
            "Output: CSV to stdout with columns: timestamp,price_per_kwh,currency,price_area"
        );
        std::process::exit(1);
    }

    let db_path = &args[1];
    let price_area = args.get(2).map(|s| s.as_str());

    eprintln!("Reading from database: {}", db_path);
    let conn = Connection::open(db_path)?;

    if let Some(area) = price_area {
        eprintln!("Filtering by price area: {}", area);
    }

    export_to_csv(&conn, price_area)?;

    eprintln!("Export complete");

    Ok(())
}
