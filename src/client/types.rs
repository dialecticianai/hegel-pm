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

// Client-side types for project-level metrics (mirrors hegel::metrics::UnifiedMetrics)
// Note: Aggregates data across ALL workflows (archived + live), not just current session
// session_id only represents the current/active session, not the aggregated historical data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatistics {
    pub hook_metrics: HookMetrics,
    pub token_metrics: TokenMetrics,
    pub state_transitions: Vec<StateTransitionEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>, // Current session only (not displayed in UI)
    pub phase_metrics: Vec<PhaseMetrics>,
    pub git_commits: Vec<GitCommit>,
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
pub struct StateTransitionEvent {
    pub timestamp: String,
    pub workflow_id: Option<String>,
    pub from_node: String,
    pub to_node: String,
    pub phase: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseMetrics {
    pub phase_name: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_seconds: u64,
    pub token_metrics: TokenMetrics,
    pub bash_commands: Vec<BashCommand>,
    pub file_modifications: Vec<FileModification>,
    pub git_commits: Vec<GitCommit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BashCommand {
    pub command: String,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileModification {
    pub file_path: String,
    pub operation: String,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommit {
    pub hash: String,
    pub timestamp: String,
    pub message: String,
    pub author: String,
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
}
