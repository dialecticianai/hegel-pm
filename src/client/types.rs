use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredProject {
    pub name: String,
    pub project_path: String,
    pub workflow_state: Option<WorkflowState>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowState {
    pub current_node: String,
    pub mode: String,
}

// Client-side types for metrics (mirrors hegel::metrics::UnifiedMetrics)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatistics {
    pub session_id: Option<String>,
    pub token_metrics: TokenMetrics,
    pub hook_metrics: HookMetrics,
    pub state_metrics: StateMetrics,
    pub phase_metrics: Vec<PhaseMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetrics {
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_cache_creation_tokens: u64,
    pub total_cache_read_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookMetrics {
    pub total_events: usize,
    pub tool_use_count: usize,
    pub bash_count: usize,
    pub write_count: usize,
    pub edit_count: usize,
    pub top_bash_commands: Vec<(String, usize)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMetrics {
    pub transition_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseMetrics {
    pub phase_name: String,
    pub visit_count: usize,
}
