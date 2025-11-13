use anyhow::Result;
use std::time::SystemTime;

use super::{find_hegel_directories, load_state, DiscoveredProject, DiscoveryConfig};

/// Discover all Hegel projects based on configuration
pub fn discover_projects(config: &DiscoveryConfig) -> Result<Vec<DiscoveredProject>> {
    let mut all_projects = Vec::new();

    // Scan each root directory
    for root in &config.root_directories {
        let hegel_dirs = find_hegel_directories(root, config.max_depth, &config.exclusions)?;

        for project_path in hegel_dirs {
            let hegel_dir = project_path.join(".hegel");

            // Extract project name from directory
            let name = project_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            // Try to load state
            let (workflow_state, error) = match load_state(&hegel_dir) {
                Ok(state) => (state, None),
                Err(e) => (None, Some(format!("Failed to load state: {}", e))),
            };

            // Calculate last activity
            let last_activity = DiscoveredProject::calculate_last_activity(&hegel_dir)
                .unwrap_or(SystemTime::UNIX_EPOCH);

            let project = DiscoveredProject::new(
                name,
                project_path,
                hegel_dir,
                workflow_state,
                last_activity,
                error,
            );

            all_projects.push(project);
        }
    }

    // Sort by last activity (most recent first)
    all_projects.sort();

    Ok(all_projects)
}

#[cfg(test)]
mod tests {
    use super::*;
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
                r#"{
                    "workflow": {
                        "current_node": "code",
                        "mode": "discovery",
                        "history": ["spec", "code"]
                    }
                }"#,
            )
            .unwrap();
        }
    }

    #[test]
    fn test_discover_single_project() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1", true);

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        let projects = discover_projects(&config).unwrap();

        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "project1");
        assert!(projects[0].has_state());
        assert!(!projects[0].has_error());
    }

    #[test]
    fn test_discover_multiple_projects() {
        let temp = TempDir::new().unwrap();
        create_test_project(temp.path(), "project1", true);
        create_test_project(temp.path(), "project2", false);
        create_test_project(temp.path(), "project3", true);

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        let projects = discover_projects(&config).unwrap();

        assert_eq!(projects.len(), 3);
    }

    #[test]
    fn test_discover_with_corrupted_state() {
        let temp = TempDir::new().unwrap();
        let project = temp.path().join("bad-project");
        fs::create_dir_all(&project).unwrap();
        let hegel_dir = project.join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();
        fs::write(hegel_dir.join("state.json"), "not valid json").unwrap();

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        let projects = discover_projects(&config).unwrap();

        assert_eq!(projects.len(), 1);
        assert!(projects[0].has_error());
        assert!(!projects[0].has_state());
    }

    #[test]
    fn test_discover_empty_workspace() {
        let temp = TempDir::new().unwrap();

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        let projects = discover_projects(&config).unwrap();

        assert_eq!(projects.len(), 0);
    }

    #[test]
    fn test_discover_sorted_by_recency() {
        let temp = TempDir::new().unwrap();

        // Create projects with different modification times
        create_test_project(temp.path(), "project1", true);
        std::thread::sleep(std::time::Duration::from_millis(10));
        create_test_project(temp.path(), "project2", true);

        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        let projects = discover_projects(&config).unwrap();

        assert_eq!(projects.len(), 2);
        // Most recent should be first
        assert_eq!(projects[0].name, "project2");
    }

    #[test]
    fn test_discover_multiple_roots() {
        let temp1 = TempDir::new().unwrap();
        let temp2 = TempDir::new().unwrap();

        create_test_project(temp1.path(), "project1", true);
        create_test_project(temp2.path(), "project2", true);

        let config = DiscoveryConfig::new(
            vec![temp1.path().to_path_buf(), temp2.path().to_path_buf()],
            10,
            vec![],
            temp1.path().join("cache.json"),
        );

        let projects = discover_projects(&config).unwrap();

        assert_eq!(projects.len(), 2);
    }
}
