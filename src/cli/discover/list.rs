use super::format::{abbreviate_path, format_size, format_timestamp, format_timestamp_iso};
use crate::discovery::{DiscoveredProject, DiscoveryEngine};
use serde::Serialize;
use std::error::Error;

/// Run the list command
pub fn run(engine: &DiscoveryEngine, json: bool, no_cache: bool) -> Result<(), Box<dyn Error>> {
    // Load projects (with cache unless no_cache is set)
    let projects = engine.get_projects(no_cache)?;

    if json {
        output_json(&projects, !no_cache)?;
    } else {
        output_human(&projects, !no_cache)?;
    }

    Ok(())
}

/// Calculate directory size (non-recursive)
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

#[derive(Serialize)]
struct ListProjectJson {
    name: String,
    project_path: String,
    hegel_dir: String,
    hegel_size_bytes: u64,
    last_activity: String,
    has_state: bool,
    has_error: bool,
}

#[derive(Serialize)]
struct ListOutputJson {
    projects: Vec<ListProjectJson>,
    total_count: usize,
    cache_used: bool,
}

fn output_json(projects: &[DiscoveredProject], cache_used: bool) -> Result<(), Box<dyn Error>> {
    let json_projects: Vec<ListProjectJson> = projects
        .iter()
        .map(|p| {
            let size = calculate_dir_size(&p.hegel_dir).unwrap_or(0);
            ListProjectJson {
                name: p.name.clone(),
                project_path: p.project_path.display().to_string(),
                hegel_dir: p.hegel_dir.display().to_string(),
                hegel_size_bytes: size,
                last_activity: format_timestamp_iso(p.last_activity),
                has_state: p.has_state(),
                has_error: p.has_error(),
            }
        })
        .collect();

    let output = ListOutputJson {
        projects: json_projects,
        total_count: projects.len(),
        cache_used,
    };

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn output_human(projects: &[DiscoveredProject], _cache_used: bool) -> Result<(), Box<dyn Error>> {
    if projects.is_empty() {
        println!("No Hegel projects found");
        return Ok(());
    }

    // Calculate column widths
    let name_width = projects
        .iter()
        .map(|p| p.name.len())
        .max()
        .unwrap_or(4)
        .max(4);
    let path_width = projects
        .iter()
        .map(|p| abbreviate_path(&p.project_path).len())
        .max()
        .unwrap_or(4)
        .max(4);

    // Print table
    for project in projects {
        let size = calculate_dir_size(&project.hegel_dir).unwrap_or(0);
        let path = abbreviate_path(&project.project_path);
        let timestamp = format_timestamp(project.last_activity);

        println!(
            "{:<name_width$}  {:<path_width$}  {:>8}  {}",
            project.name,
            path,
            format_size(size),
            timestamp,
            name_width = name_width,
            path_width = path_width
        );
    }

    println!("\n{} projects found", projects.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::DiscoveryConfig;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_project(root: &std::path::Path, name: &str) -> std::path::PathBuf {
        let project = root.join(name);
        fs::create_dir_all(&project).unwrap();
        let hegel_dir = project.join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();
        // Add some files to test size calculation
        fs::write(hegel_dir.join("state.json"), b"{}").unwrap();
        fs::write(hegel_dir.join("hooks.jsonl"), b"test data here").unwrap();
        project
    }

    #[test]
    fn test_run_list_command() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1");
        create_test_project(temp.path(), "project2");

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = DiscoveryEngine::new(config).unwrap();

        // Run list command (human output)
        let result = run(&engine, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_list_command_json() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1");

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = DiscoveryEngine::new(config).unwrap();

        // Run list command (JSON output)
        let result = run(&engine, true, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_list_command_empty() {
        let temp = TempDir::new().unwrap();

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = DiscoveryEngine::new(config).unwrap();

        // Run list command with no projects
        let result = run(&engine, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_calculate_dir_size() {
        let temp = TempDir::new().unwrap();
        let test_dir = temp.path().join("test");
        fs::create_dir(&test_dir).unwrap();
        fs::write(test_dir.join("file1.txt"), b"hello").unwrap();
        fs::write(test_dir.join("file2.txt"), b"world!").unwrap();

        let size = calculate_dir_size(&test_dir).unwrap();
        assert_eq!(size, 11); // 5 + 6 bytes
    }

    #[test]
    fn test_calculate_dir_size_empty() {
        let temp = TempDir::new().unwrap();
        let size = calculate_dir_size(temp.path()).unwrap();
        assert_eq!(size, 0);
    }
}
