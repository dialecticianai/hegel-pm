mod all;
mod format;
mod list;
mod show;

use crate::cli::DiscoverCommand;
use crate::discovery::DiscoveryEngine;
use std::error::Error;

/// Run a discover subcommand
pub fn run(
    engine: &DiscoveryEngine,
    subcommand: &DiscoverCommand,
    json: bool,
    no_cache: bool,
) -> Result<(), Box<dyn Error>> {
    match subcommand {
        DiscoverCommand::List => list::run(engine, json, no_cache),
        DiscoverCommand::Show { project_name } => show::run(engine, project_name, json, no_cache),
        DiscoverCommand::All { sort_by, benchmark } => {
            all::run(engine, sort_by, *benchmark, json, no_cache)
        }
    }
}

/// Valid sort column names
pub const VALID_SORT_COLUMNS: &[&str] = &[
    "name",
    "path",
    "size",
    "last-activity",
    "tokens",
    "events",
    "phases",
];

/// Valid sort column names when benchmarking is enabled
pub const VALID_SORT_COLUMNS_WITH_BENCHMARK: &[&str] = &[
    "name",
    "path",
    "size",
    "last-activity",
    "tokens",
    "events",
    "phases",
    "load-time",
];

/// Validate sort column name
pub fn validate_sort_column(column: &str, benchmark: bool) -> Result<(), Box<dyn Error>> {
    let valid_columns = if benchmark {
        VALID_SORT_COLUMNS_WITH_BENCHMARK
    } else {
        VALID_SORT_COLUMNS
    };

    if valid_columns.contains(&column) {
        Ok(())
    } else {
        Err(format!(
            "Invalid sort column '{}'\n\nValid columns: {}",
            column,
            valid_columns.join(", ")
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_sort_column_valid() {
        assert!(validate_sort_column("name", false).is_ok());
        assert!(validate_sort_column("tokens", false).is_ok());
        assert!(validate_sort_column("last-activity", false).is_ok());
    }

    #[test]
    fn test_validate_sort_column_invalid() {
        let result = validate_sort_column("invalid", false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid sort"));
    }

    #[test]
    fn test_validate_sort_column_load_time_without_benchmark() {
        let result = validate_sort_column("load-time", false);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sort_column_load_time_with_benchmark() {
        assert!(validate_sort_column("load-time", true).is_ok());
    }
}
