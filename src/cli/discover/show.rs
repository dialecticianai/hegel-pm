use super::format::{format_size, format_timestamp, format_timestamp_iso};
use hegel_pm::discovery::{DiscoveredProject, DiscoveryEngine};
use serde::Serialize;
use std::error::Error;

/// Run the show command
pub fn run(
    engine: &DiscoveryEngine,
    project_name: &str,
    json: bool,
    no_cache: bool,
) -> Result<(), Box<dyn Error>> {
    // Load projects
    let mut projects = engine.get_projects(no_cache)?;

    // Collect available names first for error message
    let available_names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();

    // Find project by name
    let project = projects
        .iter_mut()
        .find(|p| p.name == project_name)
        .ok_or_else(|| {
            format!(
                "Project '{}' not found\n\nAvailable projects:\n{}",
                project_name,
                available_names
                    .iter()
                    .map(|n| format!("  - {}", n))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })?;

    // Load metrics
    let _ = project.load_statistics(); // Ignore errors, will show N/A

    if json {
        output_json(project)?;
    } else {
        output_human(project)?;
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

#[derive(Serialize)]
struct WorkflowStateJson {
    mode: String,
    current_node: String,
    history: Vec<String>,
}

#[derive(Serialize)]
struct MetricsJson {
    total_input_tokens: u64,
    total_output_tokens: u64,
    total_events: usize,
    phase_count: usize,
}

#[derive(Serialize)]
struct ShowProjectJson {
    name: String,
    project_path: String,
    hegel_dir: String,
    hegel_size_bytes: u64,
    last_activity: String,
    workflow_state: Option<WorkflowStateJson>,
    metrics: Option<MetricsJson>,
    error: Option<String>,
}

fn output_json(project: &DiscoveredProject) -> Result<(), Box<dyn Error>> {
    let size = calculate_dir_size(&project.hegel_dir).unwrap_or(0);

    let workflow_state = project.workflow_state.as_ref().map(|ws| WorkflowStateJson {
        mode: ws.mode.clone(),
        current_node: ws.current_node.clone(),
        history: ws.history.clone(),
    });

    let metrics = project.statistics.as_ref().map(|stats| MetricsJson {
        total_input_tokens: stats.token_metrics.total_input_tokens,
        total_output_tokens: stats.token_metrics.total_output_tokens,
        total_events: stats.hook_metrics.total_events as usize,
        phase_count: stats.phase_metrics.len(),
    });

    let output = ShowProjectJson {
        name: project.name.clone(),
        project_path: project.project_path.display().to_string(),
        hegel_dir: project.hegel_dir.display().to_string(),
        hegel_size_bytes: size,
        last_activity: format_timestamp_iso(project.last_activity),
        workflow_state,
        metrics,
        error: project.error.clone(),
    };

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn output_human(project: &DiscoveredProject) -> Result<(), Box<dyn Error>> {
    let size = calculate_dir_size(&project.hegel_dir).unwrap_or(0);

    println!("Project: {}", project.name);
    println!("Path: {}", project.project_path.display());
    println!(".hegel size: {}", format_size(size));
    println!(
        "Last activity: {}\n",
        format_timestamp(project.last_activity)
    );

    // Workflow state
    if let Some(error) = &project.error {
        println!("Workflow State: Error loading state");
        println!("  Error: {}\n", error);
    } else if let Some(state) = &project.workflow_state {
        println!("Workflow State:");
        println!("  Mode: {}", state.mode);
        println!("  Current node: {}", state.current_node);
        println!("  History: {}\n", state.history.join(" → "));
    } else {
        println!("Workflow State: None\n");
    }

    // Metrics
    if let Some(stats) = &project.statistics {
        println!("Metrics:");
        println!(
            "  Total tokens: {} (input: {}, output: {})",
            stats.token_metrics.total_input_tokens + stats.token_metrics.total_output_tokens,
            stats.token_metrics.total_input_tokens,
            stats.token_metrics.total_output_tokens
        );
        println!("  Total events: {}", stats.hook_metrics.total_events);
        println!("  Phase count: {}", stats.phase_metrics.len());
    } else {
        println!("Metrics: No metrics available");
    }

    // Status
    let status = if project.has_error() {
        "Error (corrupted state)"
    } else if project.has_state() {
        "Active"
    } else {
        "Inactive"
    };
    println!("\nStatus: {}", status);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hegel_pm::discovery::DiscoveryConfig;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_project(root: &std::path::Path, name: &str, with_state: bool) {
        let project = root.join(name);
        fs::create_dir_all(&project).unwrap();
        let hegel_dir = project.join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();

        if with_state {
            fs::write(
                hegel_dir.join("state.json"),
                r#"{"workflow_state":{"current_node":"code","mode":"discovery","history":["spec","code"]}}"#,
            )
            .unwrap();
        }
    }

    #[test]
    fn test_run_show_command() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1", true);

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = DiscoveryEngine::new(config).unwrap();

        let result = run(&engine, "project1", false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_show_command_json() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1", true);

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = DiscoveryEngine::new(config).unwrap();

        let result = run(&engine, "project1", true, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_show_command_not_found() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1", true);

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = DiscoveryEngine::new(config).unwrap();

        let result = run(&engine, "nonexistent", false, false);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("not found"));
        assert!(err_msg.contains("Available projects"));
    }

    #[test]
    fn test_run_show_command_no_state() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1", false);

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = DiscoveryEngine::new(config).unwrap();

        let result = run(&engine, "project1", false, false);
        assert!(result.is_ok());
    }
}
