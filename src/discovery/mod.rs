mod config;

pub use config::DiscoveryConfig;

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
