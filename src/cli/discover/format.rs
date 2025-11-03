use std::path::Path;
use std::time::{Duration, SystemTime};

/// Format bytes as human-readable size (KB, MB, GB, TB)
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.1} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Abbreviate path by replacing home directory with ~
pub fn abbreviate_path(path: &Path) -> String {
    if let Some(home) = dirs::home_dir() {
        if let Ok(stripped) = path.strip_prefix(&home) {
            return format!("~/{}", stripped.display());
        }
    }
    path.display().to_string()
}

/// Format SystemTime as human-readable timestamp
pub fn format_timestamp(time: SystemTime) -> String {
    use chrono::{DateTime, Local};
    let datetime: DateTime<Local> = time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Format SystemTime as ISO 8601 for JSON
pub fn format_timestamp_iso(time: SystemTime) -> String {
    use chrono::{DateTime, Utc};
    let datetime: DateTime<Utc> = time.into();
    datetime.to_rfc3339()
}

/// Format duration as milliseconds
pub fn format_duration_ms(duration: Duration) -> String {
    format!("{}ms", duration.as_millis())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1023), "1023 B");
    }

    #[test]
    fn test_format_size_kb() {
        assert_eq!(format_size(1024), "1 KB");
        assert_eq!(format_size(1536), "2 KB");
        assert_eq!(format_size(10240), "10 KB");
    }

    #[test]
    fn test_format_size_mb() {
        assert_eq!(format_size(1_048_576), "1.0 MB");
        assert_eq!(format_size(1_572_864), "1.5 MB");
        assert_eq!(format_size(10_485_760), "10.0 MB");
    }

    #[test]
    fn test_format_size_gb() {
        assert_eq!(format_size(1_073_741_824), "1.0 GB");
        assert_eq!(format_size(1_610_612_736), "1.5 GB");
    }

    #[test]
    fn test_format_size_tb() {
        assert_eq!(format_size(1_099_511_627_776), "1.0 TB");
        assert_eq!(format_size(1_649_267_441_664), "1.5 TB");
    }

    #[test]
    fn test_abbreviate_path_with_home() {
        if let Some(home) = dirs::home_dir() {
            let path = home.join("Code/project");
            assert_eq!(abbreviate_path(&path), "~/Code/project");
        }
    }

    #[test]
    fn test_abbreviate_path_without_home() {
        let path = Path::new("/tmp/project");
        assert_eq!(abbreviate_path(path), "/tmp/project");
    }

    #[test]
    fn test_format_timestamp() {
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);
        let formatted = format_timestamp(time);
        // Just verify it doesn't panic and produces non-empty string
        assert!(!formatted.is_empty());
        assert!(formatted.contains("-")); // Contains date separators
        assert!(formatted.contains(":")); // Contains time separators
    }

    #[test]
    fn test_format_timestamp_iso() {
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);
        let formatted = format_timestamp_iso(time);
        assert!(!formatted.is_empty());
        assert!(formatted.contains("T")); // ISO 8601 contains T
        assert!(formatted.ends_with("Z") || formatted.contains("+")); // Timezone info
    }

    #[test]
    fn test_format_duration_ms() {
        assert_eq!(format_duration_ms(Duration::from_millis(123)), "123ms");
        assert_eq!(format_duration_ms(Duration::from_millis(5)), "5ms");
        assert_eq!(format_duration_ms(Duration::from_secs(1)), "1000ms");
    }
}
