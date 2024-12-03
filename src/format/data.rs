use chrono::{DateTime, Utc};

pub const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

pub fn format_bytes(bytes: u64) -> String {
    let mut size = bytes as f64;
    let mut unit = UNITS[0];

    for &u in UNITS.iter() {
        unit = u;
        if size < 1024.0 {
            break;
        }
        size /= 1024.0;
    }

    format!("{:.2} {}", size, unit)
}

pub fn format_short_date_with_time(datetime: DateTime<Utc>) -> String {
    datetime.format("%Y-%m-%d %H:%M").to_string()
}
