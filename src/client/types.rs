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

// Lightweight API response for metrics - contains only summary data, not raw events
// This matches src/discovery/api_types.rs::ProjectMetricsSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatistics {
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_cache_creation_tokens: u64,
    pub total_cache_read_tokens: u64,
    pub total_events: usize,
    pub bash_command_count: usize,
    pub file_modification_count: usize,
    pub git_commit_count: usize,
    pub phase_count: usize,
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
pub struct TokenMetrics {
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_cache_creation_tokens: u64,
    pub total_cache_read_tokens: u64,
    pub assistant_turns: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BashCommand {
    pub command: String,
    pub timestamp: Option<String>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileModification {
    pub file_path: String,
    pub tool: String,
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
