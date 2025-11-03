use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredProject {
    pub name: String,
    pub project_path: String,
    pub workflow_state: Option<WorkflowState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub current_node: String,
    pub mode: String,
}
