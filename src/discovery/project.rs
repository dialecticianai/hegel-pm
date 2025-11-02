use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

use super::{State, WorkflowState};

/// A discovered Hegel project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredProject {
    /// Name of the project (directory name)
    pub name: String,
    /// Path to project root
    pub project_path: PathBuf,
    /// Path to .hegel directory
    pub hegel_dir: PathBuf,
    /// Parsed workflow state (if state.json exists and is valid)
    pub workflow_state: Option<WorkflowState>,
    /// Last activity timestamp (most recent file modification in .hegel/)
    pub last_activity: SystemTime,
    /// When this project was discovered
    pub discovered_at: SystemTime,
    /// Error message if state is corrupted
    pub error: Option<String>,
}

impl DiscoveredProject {
    /// Create a new discovered project
    pub fn new(
        name: String,
        project_path: PathBuf,
        hegel_dir: PathBuf,
        workflow_state: Option<WorkflowState>,
        last_activity: SystemTime,
        error: Option<String>,
    ) -> Self {
        Self {
            name,
            project_path,
            hegel_dir,
            workflow_state,
            last_activity,
            discovered_at: SystemTime::now(),
            error,
        }
    }

    /// Calculate last activity from .hegel directory file modifications
    pub fn calculate_last_activity(hegel_dir: &PathBuf) -> Result<SystemTime> {
        let mut latest = SystemTime::UNIX_EPOCH;

        // Walk .hegel directory and find most recent modification
        for entry in std::fs::read_dir(hegel_dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if let Ok(modified) = metadata.modified() {
                if modified > latest {
                    latest = modified;
                }
            }
        }

        Ok(latest)
    }

    /// Check if project has an error (corrupted state)
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    /// Check if project has workflow state
    pub fn has_state(&self) -> bool {
        self.workflow_state.is_some()
    }
}

impl PartialEq for DiscoveredProject {
    fn eq(&self, other: &Self) -> bool {
        self.project_path == other.project_path
    }
}

impl Eq for DiscoveredProject {}

impl PartialOrd for DiscoveredProject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DiscoveredProject {
    /// Sort by last activity (most recent first)
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse order so most recent comes first
        other.last_activity.cmp(&self.last_activity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use tempfile::TempDir;

    #[test]
    fn test_project_creation() {
        let temp = TempDir::new().unwrap();
        let hegel_dir = temp.path().join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();

        let project = DiscoveredProject::new(
            "test-project".to_string(),
            temp.path().to_path_buf(),
            hegel_dir.clone(),
            None,
            SystemTime::now(),
            None,
        );

        assert_eq!(project.name, "test-project");
        assert_eq!(project.project_path, temp.path());
        assert_eq!(project.hegel_dir, hegel_dir);
        assert!(!project.has_error());
        assert!(!project.has_state());
    }

    #[test]
    fn test_project_with_error() {
        let temp = TempDir::new().unwrap();
        let hegel_dir = temp.path().join(".hegel");

        let project = DiscoveredProject::new(
            "test".to_string(),
            temp.path().to_path_buf(),
            hegel_dir,
            None,
            SystemTime::now(),
            Some("Corrupted state".to_string()),
        );

        assert!(project.has_error());
        assert_eq!(project.error.as_deref(), Some("Corrupted state"));
    }

    #[test]
    fn test_calculate_last_activity() {
        let temp = TempDir::new().unwrap();
        let hegel_dir = temp.path().join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();

        // Create a file
        fs::write(hegel_dir.join("state.json"), b"{}").unwrap();
        thread::sleep(Duration::from_millis(10));

        // Create another file (should be newer)
        fs::write(hegel_dir.join("hooks.jsonl"), b"{}").unwrap();

        let last_activity = DiscoveredProject::calculate_last_activity(&hegel_dir).unwrap();

        // Verify it's recent
        let now = SystemTime::now();
        let elapsed = now.duration_since(last_activity).unwrap();
        assert!(elapsed < Duration::from_secs(5));
    }

    #[test]
    fn test_sorting_by_recency() {
        let temp1 = TempDir::new().unwrap();
        let temp2 = TempDir::new().unwrap();

        let older_time = SystemTime::now() - Duration::from_secs(100);
        let newer_time = SystemTime::now();

        let project1 = DiscoveredProject::new(
            "older".to_string(),
            temp1.path().to_path_buf(),
            temp1.path().join(".hegel"),
            None,
            older_time,
            None,
        );

        let project2 = DiscoveredProject::new(
            "newer".to_string(),
            temp2.path().to_path_buf(),
            temp2.path().join(".hegel"),
            None,
            newer_time,
            None,
        );

        let mut projects = vec![project1.clone(), project2.clone()];
        projects.sort();

        // Most recent should be first
        assert_eq!(projects[0].name, "newer");
        assert_eq!(projects[1].name, "older");
    }

    #[test]
    fn test_serialization() {
        let temp = TempDir::new().unwrap();
        let project = DiscoveredProject::new(
            "test".to_string(),
            temp.path().to_path_buf(),
            temp.path().join(".hegel"),
            None,
            SystemTime::now(),
            None,
        );

        let json = serde_json::to_string(&project).unwrap();
        let deserialized: DiscoveredProject = serde_json::from_str(&json).unwrap();

        assert_eq!(project.name, deserialized.name);
        assert_eq!(project.project_path, deserialized.project_path);
        assert_eq!(project.hegel_dir, deserialized.hegel_dir);
    }
}
