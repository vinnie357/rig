use serde_json::Value;

/// Formats a duration in seconds to human readable format
pub fn format_duration(seconds: u64) -> String {
    match seconds {
        0..=59 => format!("{seconds}s"),
        60..=3599 => format!("{}m {}s", seconds / 60, seconds % 60),
        3600..=86399 => {
            let hours = seconds / 3600;
            let minutes = (seconds % 3600) / 60;
            format!("{hours}h {minutes}m")
        }
        _ => {
            let days = seconds / 86400;
            let hours = (seconds % 86400) / 3600;
            format!("{days}d {hours}h")
        }
    }
}

/// Formats bytes to human readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{bytes} {}", UNITS[unit_index])
    } else {
        format!("{size:.1} {}", UNITS[unit_index])
    }
}

/// Pretty print JSON with proper indentation
pub fn pretty_print_json(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| "Invalid JSON".to_string())
}

/// Creates a simple table row
pub fn format_table_row(columns: &[&str], widths: &[usize]) -> String {
    columns
        .iter()
        .zip(widths.iter())
        .map(|(col, &width)| format!("{col:<width$}"))
        .collect::<Vec<_>>()
        .join(" | ")
}

/// Creates a table separator
pub fn format_table_separator(widths: &[usize]) -> String {
    widths
        .iter()
        .map(|&width| "-".repeat(width))
        .collect::<Vec<_>>()
        .join("-+-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3661), "1h 1m");
        assert_eq!(format_duration(90061), "1d 1h");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
    }
}