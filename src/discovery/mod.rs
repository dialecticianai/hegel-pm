mod api_types;
mod cache;
mod config;
mod discover;
mod engine;
mod project;
mod state;
mod statistics;
mod walker;

pub use api_types::{ProjectListItem, ProjectMetricsSummary};
pub use cache::{load_binary_cache, load_cache, save_binary_cache, save_cache};
pub use config::DiscoveryConfig;
pub use discover::discover_projects;
pub use engine::DiscoveryEngine;
pub use project::DiscoveredProject;
pub use state::load_state;
pub use statistics::ProjectStatistics;
pub use walker::find_hegel_directories;

// Re-export hegel-cli types we depend on
pub use hegel::storage::State;
pub use hegel::storage::WorkflowState;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hegel_cli_imports() {
        // Verify we can access hegel-cli types
        let _state_type_exists = std::mem::size_of::<State>();
        let _workflow_state_type_exists = std::mem::size_of::<WorkflowState>();
    }
}
