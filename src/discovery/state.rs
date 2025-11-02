use anyhow::{Context, Result};
use std::path::PathBuf;

use hegel::storage::FileStorage;

use super::{State, WorkflowState};

/// Load workflow state from a .hegel directory
pub fn load_state(hegel_dir: &PathBuf) -> Result<Option<WorkflowState>> {
    let storage = FileStorage::new(hegel_dir).context(format!(
        "Failed to create storage for {}",
        hegel_dir.display()
    ))?;

    let state = storage.load().context("Failed to load state")?;

    Ok(state.workflow_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_hegel_dir_with_state(state_json: &str) -> TempDir {
        let temp = TempDir::new().unwrap();
        let hegel_dir = temp.path().join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();
        fs::write(hegel_dir.join("state.json"), state_json).unwrap();
        temp
    }

    #[test]
    fn test_load_valid_state() {
        let temp = create_hegel_dir_with_state(
            r#"{
                "workflow_state": {
                    "current_node": "spec",
                    "mode": "discovery",
                    "history": ["init", "spec"]
                }
            }"#,
        );

        let hegel_dir = temp.path().join(".hegel");
        let state = load_state(&hegel_dir).unwrap();

        assert!(state.is_some());
        let workflow_state = state.unwrap();
        assert_eq!(workflow_state.current_node, "spec");
        assert_eq!(workflow_state.mode, "discovery");
        assert_eq!(workflow_state.history.len(), 2);
    }

    #[test]
    fn test_load_missing_state() {
        let temp = TempDir::new().unwrap();
        let hegel_dir = temp.path().join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();

        let state = load_state(&hegel_dir).unwrap();

        assert!(state.is_none());
    }

    #[test]
    fn test_load_corrupted_state() {
        let temp = create_hegel_dir_with_state("not valid json");
        let hegel_dir = temp.path().join(".hegel");

        let result = load_state(&hegel_dir);

        // Should fail to load corrupted state
        assert!(result.is_err());
    }

    #[test]
    fn test_load_empty_state() {
        let temp = create_hegel_dir_with_state("{}");
        let hegel_dir = temp.path().join(".hegel");

        let state = load_state(&hegel_dir).unwrap();

        assert!(state.is_none());
    }

    #[test]
    fn test_load_state_with_workflow() {
        let temp = create_hegel_dir_with_state(
            r#"{
                "workflow": {
                    "name": "discovery",
                    "nodes": {}
                },
                "workflow_state": {
                    "current_node": "code",
                    "mode": "discovery",
                    "history": ["spec", "plan", "code"],
                    "workflow_id": "2024-01-01T00:00:00Z"
                }
            }"#,
        );

        let hegel_dir = temp.path().join(".hegel");
        let state = load_state(&hegel_dir).unwrap();

        assert!(state.is_some());
        let workflow_state = state.unwrap();
        assert_eq!(workflow_state.current_node, "code");
        assert!(workflow_state.workflow_id.is_some());
    }
}
