use super::ProjectStatistics;
use serde::{Deserialize, Serialize};

/// Lightweight API response for metrics - contains only summary data, not raw events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetricsSummary {
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

impl From<&ProjectStatistics> for ProjectMetricsSummary {
    fn from(stats: &ProjectStatistics) -> Self {
        Self {
            total_input_tokens: stats.token_metrics.total_input_tokens,
            total_output_tokens: stats.token_metrics.total_output_tokens,
            total_cache_creation_tokens: stats.token_metrics.total_cache_creation_tokens,
            total_cache_read_tokens: stats.token_metrics.total_cache_read_tokens,
            total_events: stats.hook_metrics.total_events,
            bash_command_count: stats.hook_metrics.bash_commands.len(),
            file_modification_count: stats.hook_metrics.file_modifications.len(),
            git_commit_count: stats.git_commits.len(),
            phase_count: stats.phase_metrics.len(),
        }
    }
}
