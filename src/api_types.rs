use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Build workflow summaries from UnifiedMetrics by grouping phase_metrics by workflow_id
pub fn build_workflow_summaries(
    unified_metrics: &hegel::metrics::UnifiedMetrics,
) -> Vec<WorkflowSummary> {
    // Group phases by workflow_id from state_transitions
    let mut workflows: HashMap<String, Vec<&hegel::metrics::PhaseMetrics>> = HashMap::new();

    // Match phases to workflows by finding the state transition with matching phase name and closest preceding timestamp
    for phase in &unified_metrics.phase_metrics {
        // Find the transition that started this phase
        let matching_transition = unified_metrics
            .state_transitions
            .iter()
            .filter(|t| t.to_node == phase.phase_name && t.timestamp <= phase.start_time)
            .max_by_key(|t| &t.timestamp);

        if let Some(transition) = matching_transition {
            if let Some(ref workflow_id) = transition.workflow_id {
                workflows
                    .entry(workflow_id.clone())
                    .or_insert_with(Vec::new)
                    .push(phase);
            }
        }
    }

    // Build WorkflowSummary for each workflow
    let mut workflow_summaries: Vec<WorkflowSummary> = Vec::new();

    for (workflow_id, phases) in workflows {
        // Determine mode from state_transitions
        let mode = unified_metrics
            .state_transitions
            .iter()
            .find(|t| t.workflow_id.as_ref() == Some(&workflow_id))
            .map(|t| t.mode.clone())
            .unwrap_or_else(|| "unknown".to_string());

        // Determine status: Active if any phase has no end_time, otherwise Completed
        let has_active_phase = phases.iter().any(|p| p.end_time.is_none());
        let status = if has_active_phase {
            WorkflowStatus::Active
        } else {
            WorkflowStatus::Completed
        };

        // Find current phase for active workflows
        let current_phase = if has_active_phase {
            phases
                .iter()
                .find(|p| p.end_time.is_none())
                .map(|p| p.phase_name.clone())
        } else {
            None
        };

        // Build phase summaries
        let mut phase_summaries: Vec<PhaseSummary> = Vec::new();
        let mut total_input = 0u64;
        let mut total_output = 0u64;
        let mut total_cache_creation = 0u64;
        let mut total_cache_read = 0u64;
        let mut total_events = 0usize;
        let mut total_bash_commands = 0usize;
        let mut total_file_modifications = 0usize;
        let mut total_git_commits = 0usize;

        for phase in &phases {
            let phase_status = if phase.end_time.is_none() {
                PhaseStatus::InProgress
            } else {
                PhaseStatus::Completed
            };

            let phase_summary = PhaseSummary {
                phase_name: phase.phase_name.clone(),
                status: phase_status,
                start_time: phase.start_time.clone(),
                end_time: phase.end_time.clone(),
                duration_seconds: phase.duration_seconds,
                metrics: PhaseMetricsSummary {
                    total_input_tokens: phase.token_metrics.total_input_tokens,
                    total_output_tokens: phase.token_metrics.total_output_tokens,
                    total_cache_creation_tokens: phase.token_metrics.total_cache_creation_tokens,
                    total_cache_read_tokens: phase.token_metrics.total_cache_read_tokens,
                    total_all_tokens: phase.token_metrics.total_input_tokens
                        + phase.token_metrics.total_output_tokens
                        + phase.token_metrics.total_cache_creation_tokens
                        + phase.token_metrics.total_cache_read_tokens,
                    event_count: phase.bash_commands.len() + phase.file_modifications.len(),
                    bash_command_count: phase.bash_commands.len(),
                    file_modification_count: phase.file_modifications.len(),
                    git_commit_count: phase.git_commits.len(),
                },
            };

            // Accumulate totals
            total_input += phase.token_metrics.total_input_tokens;
            total_output += phase.token_metrics.total_output_tokens;
            total_cache_creation += phase.token_metrics.total_cache_creation_tokens;
            total_cache_read += phase.token_metrics.total_cache_read_tokens;
            total_events += phase.bash_commands.len() + phase.file_modifications.len();
            total_bash_commands += phase.bash_commands.len();
            total_file_modifications += phase.file_modifications.len();
            total_git_commits += phase.git_commits.len();

            phase_summaries.push(phase_summary);
        }

        // Sort phases by start_time
        phase_summaries.sort_by(|a, b| a.start_time.cmp(&b.start_time));

        let workflow_summary = WorkflowSummary {
            workflow_id: workflow_id.clone(),
            mode,
            status,
            current_phase,
            phases: phase_summaries,
            total_metrics: PhaseMetricsSummary {
                total_input_tokens: total_input,
                total_output_tokens: total_output,
                total_cache_creation_tokens: total_cache_creation,
                total_cache_read_tokens: total_cache_read,
                total_all_tokens: total_input
                    + total_output
                    + total_cache_creation
                    + total_cache_read,
                event_count: total_events,
                bash_command_count: total_bash_commands,
                file_modification_count: total_file_modifications,
                git_commit_count: total_git_commits,
            },
        };

        workflow_summaries.push(workflow_summary);
    }

    // Sort workflows by workflow_id (newest first, assuming ISO 8601 timestamps)
    workflow_summaries.sort_by(|a, b| b.workflow_id.cmp(&a.workflow_id));

    workflow_summaries
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_phase_metrics(
        phase_name: &str,
        start_time: &str,
        end_time: Option<&str>,
        duration_seconds: u64,
        input_tokens: u64,
        output_tokens: u64,
        turns: usize,
        workflow_id: Option<String>,
    ) -> hegel::metrics::PhaseMetrics {
        hegel::metrics::PhaseMetrics {
            phase_name: phase_name.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.map(|s| s.to_string()),
            duration_seconds,
            token_metrics: hegel::metrics::TokenMetrics {
                total_input_tokens: input_tokens,
                total_output_tokens: output_tokens,
                total_cache_creation_tokens: 0,
                total_cache_read_tokens: 0,
                assistant_turns: turns,
            },
            bash_commands: vec![],
            file_modifications: vec![],
            git_commits: vec![],
            is_synthetic: false,
            workflow_id,
        }
    }

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

    #[test]
    fn test_build_workflow_summaries_empty() {
        let metrics = hegel::metrics::UnifiedMetrics::default();
        let workflows = build_workflow_summaries(&metrics);
        assert_eq!(workflows.len(), 0);
    }

    #[test]
    fn test_build_workflow_summaries_single_workflow() {
        use hegel::metrics::{PhaseMetrics, StateTransitionEvent, TokenMetrics, UnifiedMetrics};

        let workflow_id = "2025-01-15T10:00:00Z".to_string();

        let mut metrics = UnifiedMetrics::default();

        // Add state transitions
        metrics.state_transitions = vec![
            StateTransitionEvent {
                timestamp: "2025-01-15T10:00:00Z".to_string(),
                workflow_id: Some(workflow_id.clone()),
                from_node: "START".to_string(),
                to_node: "spec".to_string(),
                phase: "spec".to_string(),
                mode: "execution".to_string(),
            },
            StateTransitionEvent {
                timestamp: "2025-01-15T10:30:00Z".to_string(),
                workflow_id: Some(workflow_id.clone()),
                from_node: "spec".to_string(),
                to_node: "plan".to_string(),
                phase: "plan".to_string(),
                mode: "execution".to_string(),
            },
        ];

        // Add phases
        metrics.phase_metrics = vec![
            create_phase_metrics(
                "spec",
                "2025-01-15T10:00:00Z",
                Some("2025-01-15T10:30:00Z"),
                1800,
                1000,
                500,
                5,
                Some("2025-01-15T10:00:00Z".to_string()),
            ),
            create_phase_metrics(
                "plan",
                "2025-01-15T10:30:00Z",
                None,
                0,
                500,
                250,
                3,
                Some("2025-01-15T10:00:00Z".to_string()),
            ),
        ];

        let workflows = build_workflow_summaries(&metrics);

        assert_eq!(workflows.len(), 1);
        let workflow = &workflows[0];
        assert_eq!(workflow.workflow_id, workflow_id);
        assert_eq!(workflow.mode, "execution");
        assert!(matches!(workflow.status, WorkflowStatus::Active));
        assert_eq!(workflow.current_phase, Some("plan".to_string()));
        assert_eq!(workflow.phases.len(), 2);

        // Check phase order (sorted by start_time)
        assert_eq!(workflow.phases[0].phase_name, "spec");
        assert_eq!(workflow.phases[1].phase_name, "plan");

        // Check phase status
        assert!(matches!(workflow.phases[0].status, PhaseStatus::Completed));
        assert!(matches!(workflow.phases[1].status, PhaseStatus::InProgress));

        // Check total metrics (helper sets cache tokens to 0)
        assert_eq!(workflow.total_metrics.total_input_tokens, 1500);
        assert_eq!(workflow.total_metrics.total_output_tokens, 750);
        assert_eq!(workflow.total_metrics.total_all_tokens, 2250);
    }

    #[test]
    fn test_build_workflow_summaries_multiple_workflows() {
        use hegel::metrics::{PhaseMetrics, StateTransitionEvent, TokenMetrics, UnifiedMetrics};

        let workflow1_id = "2025-01-15T10:00:00Z".to_string();
        let workflow2_id = "2025-01-14T10:00:00Z".to_string();

        let mut metrics = UnifiedMetrics::default();

        metrics.state_transitions = vec![
            StateTransitionEvent {
                timestamp: "2025-01-15T10:00:00Z".to_string(),
                workflow_id: Some(workflow1_id.clone()),
                from_node: "START".to_string(),
                to_node: "spec".to_string(),
                phase: "spec".to_string(),
                mode: "execution".to_string(),
            },
            StateTransitionEvent {
                timestamp: "2025-01-14T10:00:00Z".to_string(),
                workflow_id: Some(workflow2_id.clone()),
                from_node: "START".to_string(),
                to_node: "spec".to_string(),
                phase: "spec".to_string(),
                mode: "discovery".to_string(),
            },
        ];

        metrics.phase_metrics = vec![
            create_phase_metrics(
                "spec",
                "2025-01-15T10:00:00Z",
                Some("2025-01-15T10:30:00Z"),
                1800,
                1000,
                500,
                5,
                Some(workflow1_id.clone()),
            ),
            create_phase_metrics(
                "spec",
                "2025-01-14T10:00:00Z",
                Some("2025-01-14T10:30:00Z"),
                1800,
                2000,
                1000,
                10,
                Some(workflow2_id.clone()),
            ),
        ];

        let workflows = build_workflow_summaries(&metrics);

        assert_eq!(workflows.len(), 2);

        // Should be sorted newest first
        assert_eq!(workflows[0].workflow_id, workflow1_id);
        assert_eq!(workflows[1].workflow_id, workflow2_id);

        // Check modes
        assert_eq!(workflows[0].mode, "execution");
        assert_eq!(workflows[1].mode, "discovery");

        // Both completed
        assert!(matches!(workflows[0].status, WorkflowStatus::Completed));
        assert!(matches!(workflows[1].status, WorkflowStatus::Completed));
    }
}
