use super::format::{abbreviate_path, format_duration_ms, format_size, format_timestamp};
use super::validate_sort_column;
use hegel_pm::discovery::DiscoveryEngine;
use serde::Serialize;
use std::error::Error;
use std::time::Instant;
use tracing::info;

#[derive(Clone)]
struct ProjectRow {
    name: String,
    path: String,
    size: u64,
    last_activity: std::time::SystemTime,
    total_tokens: u64,
    total_events: usize,
    phase_count: usize,
    load_time_ms: Option<u64>,
}

/// Run the all command
pub fn run(
    engine: &DiscoveryEngine,
    sort_by: &str,
    benchmark: bool,
    json: bool,
    no_cache: bool,
) -> Result<(), Box<dyn Error>> {
    // Validate sort column
    validate_sort_column(sort_by, benchmark)?;

    // Load projects
    let mut projects = engine.get_projects(no_cache)?;

    // Load metrics for all projects with optional benchmarking
    let start_all = Instant::now();
    let mut rows: Vec<ProjectRow> = Vec::new();

    for project in &mut projects {
        let start = Instant::now();
        let _ = project.load_statistics(); // Ignore errors
        let load_time = if benchmark {
            Some(start.elapsed().as_millis() as u64)
        } else {
            None
        };

        let (total_tokens, total_events, phase_count) = if let Some(stats) = &project.statistics {
            (
                stats.token_metrics.total_input_tokens + stats.token_metrics.total_output_tokens,
                stats.hook_metrics.total_events as usize,
                stats.phase_metrics.len(),
            )
        } else {
            (0, 0, 0)
        };

        rows.push(ProjectRow {
            name: project.name.clone(),
            path: project.project_path.display().to_string(),
            size: calculate_dir_size(&project.hegel_dir).unwrap_or(0),
            last_activity: project.last_activity,
            total_tokens,
            total_events,
            phase_count,
            load_time_ms: load_time,
        });
    }

    let total_load_time = if benchmark {
        Some(start_all.elapsed().as_millis() as u64)
    } else {
        None
    };

    // Sort rows
    sort_rows(&mut rows, sort_by);

    if json {
        output_json(&rows, sort_by, total_load_time, !no_cache)?;
    } else {
        output_human(&rows, sort_by, total_load_time, !no_cache)?;
    }

    Ok(())
}

fn calculate_dir_size(path: &std::path::Path) -> Result<u64, std::io::Error> {
    let mut total = 0u64;
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            total += metadata.len();
        }
    }
    Ok(total)
}

fn sort_rows(rows: &mut [ProjectRow], sort_by: &str) {
    match sort_by {
        "name" => rows.sort_by(|a, b| a.name.cmp(&b.name)),
        "path" => rows.sort_by(|a, b| a.path.cmp(&b.path)),
        "size" => rows.sort_by(|a, b| b.size.cmp(&a.size)), // Descending
        "last-activity" => rows.sort_by(|a, b| b.last_activity.cmp(&a.last_activity)), // Desc
        "tokens" => rows.sort_by(|a, b| b.total_tokens.cmp(&a.total_tokens)), // Desc
        "events" => rows.sort_by(|a, b| b.total_events.cmp(&a.total_events)), // Desc
        "phases" => rows.sort_by(|a, b| b.phase_count.cmp(&a.phase_count)), // Desc
        "load-time" => rows.sort_by(|a, b| {
            b.load_time_ms
                .unwrap_or(0)
                .cmp(&a.load_time_ms.unwrap_or(0))
        }), // Desc
        _ => {}                                             // Already validated
    }
}

#[derive(Serialize)]
struct AllProjectJson {
    name: String,
    path: String,
    size_bytes: u64,
    last_activity: String,
    total_tokens: u64,
    total_events: usize,
    phase_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_time_ms: Option<u64>,
}

#[derive(Serialize)]
struct AllOutputJson {
    projects: Vec<AllProjectJson>,
    total_count: usize,
    sorted_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_load_time_ms: Option<u64>,
    cache_used: bool,
}

fn output_json(
    rows: &[ProjectRow],
    sort_by: &str,
    total_load_time: Option<u64>,
    cache_used: bool,
) -> Result<(), Box<dyn Error>> {
    let projects: Vec<AllProjectJson> = rows
        .iter()
        .map(|r| AllProjectJson {
            name: r.name.clone(),
            path: r.path.clone(),
            size_bytes: r.size,
            last_activity: super::format::format_timestamp_iso(r.last_activity),
            total_tokens: r.total_tokens,
            total_events: r.total_events,
            phase_count: r.phase_count,
            load_time_ms: r.load_time_ms,
        })
        .collect();

    let output = AllOutputJson {
        projects,
        total_count: rows.len(),
        sorted_by: sort_by.to_string(),
        total_load_time_ms: total_load_time,
        cache_used,
    };

    info!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn output_human(
    rows: &[ProjectRow],
    sort_by: &str,
    total_load_time: Option<u64>,
    _cache_used: bool,
) -> Result<(), Box<dyn Error>> {
    if rows.is_empty() {
        info!("No Hegel projects found");
        return Ok(());
    }

    // Calculate column widths
    let name_width = rows.iter().map(|r| r.name.len()).max().unwrap_or(4).max(4);
    let path_width = rows
        .iter()
        .map(|r| abbreviate_path(&std::path::PathBuf::from(&r.path)).len())
        .max()
        .unwrap_or(4)
        .max(4);

    // Print header
    if total_load_time.is_some() {
        info!(
            "{:<name_width$}  {:<path_width$}  {:>8}  {:>19}  {:>8}  {:>8}  {:>7}  {:>9}",
            "NAME",
            "PATH",
            "SIZE",
            "LAST ACTIVITY",
            "TOKENS",
            "EVENTS",
            "PHASES",
            "LOAD TIME",
            name_width = name_width,
            path_width = path_width
        );
    } else {
        info!(
            "{:<name_width$}  {:<path_width$}  {:>8}  {:>19}  {:>8}  {:>8}  {:>7}",
            "NAME",
            "PATH",
            "SIZE",
            "LAST ACTIVITY",
            "TOKENS",
            "EVENTS",
            "PHASES",
            name_width = name_width,
            path_width = path_width
        );
    }

    // Print rows
    for row in rows {
        let path_abbrev = abbreviate_path(&std::path::PathBuf::from(&row.path));
        let timestamp = format_timestamp(row.last_activity);

        if let Some(load_ms) = row.load_time_ms {
            info!(
                "{:<name_width$}  {:<path_width$}  {:>8}  {:>19}  {:>8}  {:>8}  {:>7}  {:>9}",
                row.name,
                path_abbrev,
                format_size(row.size),
                timestamp,
                row.total_tokens,
                row.total_events,
                row.phase_count,
                format_duration_ms(std::time::Duration::from_millis(load_ms)),
                name_width = name_width,
                path_width = path_width
            );
        } else {
            info!(
                "{:<name_width$}  {:<path_width$}  {:>8}  {:>19}  {:>8}  {:>8}  {:>7}",
                row.name,
                path_abbrev,
                format_size(row.size),
                timestamp,
                row.total_tokens,
                row.total_events,
                row.phase_count,
                name_width = name_width,
                path_width = path_width
            );
        }
    }

    // Footer
    if sort_by == "last-activity" {
        info!("\n{} projects found", rows.len());
    } else {
        info!("\n{} projects found (sorted by {})", rows.len(), sort_by);
    }

    if let Some(total_ms) = total_load_time {
        info!(
            "Total load time: {}",
            format_duration_ms(std::time::Duration::from_millis(total_ms))
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::VALID_SORT_COLUMNS;
    use super::*;
    use hegel_pm::discovery::DiscoveryConfig;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_project(root: &std::path::Path, name: &str, with_metrics: bool) {
        let project = root.join(name);
        fs::create_dir_all(&project).unwrap();
        let hegel_dir = project.join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();

        if with_metrics {
            fs::write(hegel_dir.join("state.json"), b"{}").unwrap();
            fs::write(hegel_dir.join("hooks.jsonl"), b"test").unwrap();
        }
    }

    #[test]
    fn test_run_all_command() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1", true);
        create_test_project(temp.path(), "project2", true);

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = DiscoveryEngine::new(config).unwrap();

        let result = run(&engine, "last-activity", false, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_all_command_with_benchmark() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1", true);

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = DiscoveryEngine::new(config).unwrap();

        let result = run(&engine, "load-time", true, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_all_command_various_sorts() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1", true);

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = DiscoveryEngine::new(config).unwrap();

        for sort_col in VALID_SORT_COLUMNS {
            let result = run(&engine, sort_col, false, false, false);
            assert!(result.is_ok(), "Failed for sort column: {}", sort_col);
        }
    }

    #[test]
    fn test_run_all_command_invalid_sort() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1", true);

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = DiscoveryEngine::new(config).unwrap();

        let result = run(&engine, "invalid", false, false, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid sort"));
    }

    #[test]
    fn test_sort_rows() {
        let mut rows = vec![
            ProjectRow {
                name: "zzz".to_string(),
                path: "/path/z".to_string(),
                size: 100,
                last_activity: std::time::SystemTime::UNIX_EPOCH,
                total_tokens: 50,
                total_events: 10,
                phase_count: 2,
                load_time_ms: Some(100),
            },
            ProjectRow {
                name: "aaa".to_string(),
                path: "/path/a".to_string(),
                size: 200,
                last_activity: std::time::SystemTime::now(),
                total_tokens: 100,
                total_events: 20,
                phase_count: 5,
                load_time_ms: Some(50),
            },
        ];

        sort_rows(&mut rows, "name");
        assert_eq!(rows[0].name, "aaa");

        sort_rows(&mut rows, "size");
        assert_eq!(rows[0].size, 200); // Descending

        sort_rows(&mut rows, "tokens");
        assert_eq!(rows[0].total_tokens, 100); // Descending
    }
}
