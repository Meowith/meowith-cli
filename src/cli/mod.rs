use meowith_connector::dto::range::Range;

pub mod args;
pub(crate) mod config;
pub mod connector;

///
/// Function that parses string into optional start and optional end range
/// Accepted inputs, (e.g. "10-20", "-20", "10-", "-", "15")
///
pub fn range_from_str(input: &str) -> Option<Range> {
    let parts: Vec<&str> = input.split('-').map(|s| s.trim()).collect();

    if parts.len() > 2 {
        return None;
    }

    let start = match parts.get(0) {
        Some(&"") => None,
        Some(s) => s.parse::<i32>().ok(),
        None => None,
    };

    let end = match parts.get(1) {
        Some(&"") => None,
        Some(s) => s.parse::<i32>().ok(),
        None => None,
    };

    Some(Range { start, end })
}
