use serde::{Deserialize, Serialize};

use crate::discovery::{ProjectMetricsSummary, WorkflowState};

/// All-projects aggregate view (new endpoint)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllProjectsAggregate {
    pub total_projects: usize,
    pub aggregate_metrics: AggregateMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateMetrics {
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_cache_creation_tokens: u64,
    pub total_cache_read_tokens: u64,
    pub total_all_tokens: u64,
    pub total_events: usize,
    pub bash_command_count: usize,
    pub file_modification_count: usize,
    pub git_commit_count: usize,
    pub phase_count: usize,
}

/// Combined project info (summary + workflow detail)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub project_name: String,
    pub summary: ProjectMetricsSummary,
    pub detail: ProjectWorkflowDetail,
}

/// Per-project workflow detail view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWorkflowDetail {
    pub current_workflow_state: Option<WorkflowState>,
    pub workflows: Vec<WorkflowSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSummary {
    pub workflow_id: String,
    pub mode: String,
    pub status: WorkflowStatus,
    pub current_phase: Option<String>,
    pub phases: Vec<PhaseSummary>,
    pub total_metrics: PhaseMetricsSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Active,
    Completed,
    Aborted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseSummary {
    pub phase_name: String,
    pub status: PhaseStatus,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_seconds: u64,
    pub metrics: PhaseMetricsSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhaseStatus {
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseMetricsSummary {
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_cache_creation_tokens: u64,
    pub total_cache_read_tokens: u64,
    pub total_all_tokens: u64,
    pub event_count: usize,
    pub bash_command_count: usize,
    pub file_modification_count: usize,
    pub git_commit_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_info_serialization() {
        let summary = ProjectMetricsSummary {
            total_input_tokens: 1000,
            total_output_tokens: 500,
            total_cache_creation_tokens: 100,
            total_cache_read_tokens: 200,
            total_all_tokens: 1800,
            total_events: 50,
            bash_command_count: 10,
            file_modification_count: 5,
            git_commit_count: 2,
            phase_count: 3,
        };

        let detail = ProjectWorkflowDetail {
            current_workflow_state: None,
            workflows: vec![],
        };

        let project_info = ProjectInfo {
            project_name: "test-project".to_string(),
            summary,
            detail,
        };

        let json = serde_json::to_string(&project_info).unwrap();
        assert!(json.contains("test-project"));
        assert!(json.contains("1000"));
        assert!(json.contains("workflows"));

        let deserialized: ProjectInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.project_name, "test-project");
        assert_eq!(deserialized.summary.total_input_tokens, 1000);
    }

    #[test]
    fn test_workflow_status_serialization() {
        let active = WorkflowStatus::Active;
        let completed = WorkflowStatus::Completed;
        let aborted = WorkflowStatus::Aborted;

        let active_json = serde_json::to_string(&active).unwrap();
        let completed_json = serde_json::to_string(&completed).unwrap();
        let aborted_json = serde_json::to_string(&aborted).unwrap();

        assert_eq!(active_json, "\"Active\"");
        assert_eq!(completed_json, "\"Completed\"");
        assert_eq!(aborted_json, "\"Aborted\"");

        let active_de: WorkflowStatus = serde_json::from_str(&active_json).unwrap();
        matches!(active_de, WorkflowStatus::Active);
    }

    #[test]
    fn test_phase_status_serialization() {
        let in_progress = PhaseStatus::InProgress;
        let completed = PhaseStatus::Completed;

        let in_progress_json = serde_json::to_string(&in_progress).unwrap();
        let completed_json = serde_json::to_string(&completed).unwrap();

        assert_eq!(in_progress_json, "\"InProgress\"");
        assert_eq!(completed_json, "\"Completed\"");

        let in_progress_de: PhaseStatus = serde_json::from_str(&in_progress_json).unwrap();
        matches!(in_progress_de, PhaseStatus::InProgress);
    }

    #[test]
    fn test_all_projects_aggregate_serialization() {
        let aggregate = AllProjectsAggregate {
            total_projects: 5,
            aggregate_metrics: AggregateMetrics {
                total_input_tokens: 5000,
                total_output_tokens: 2500,
                total_cache_creation_tokens: 500,
                total_cache_read_tokens: 1000,
                total_all_tokens: 9000,
                total_events: 250,
                bash_command_count: 50,
                file_modification_count: 25,
                git_commit_count: 10,
                phase_count: 15,
            },
        };

        let json = serde_json::to_string(&aggregate).unwrap();
        assert!(json.contains("5"));
        assert!(json.contains("5000"));

        let deserialized: AllProjectsAggregate = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.total_projects, 5);
        assert_eq!(deserialized.aggregate_metrics.total_input_tokens, 5000);
    }
}
