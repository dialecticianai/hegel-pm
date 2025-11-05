# UI v1 Specification

Enhanced multi-level navigation for hegel-pm web UI with cross-project aggregates and per-project workflow details.

## Overview

**What it does:** Adds two new view levels to the web UI: (1) an "All Projects" dashboard showing aggregate statistics across all discovered projects, and (2) enhanced per-project detail pages showing collapsible workflow breakdowns with phase-level information similar to `hegel analyze` output.

**Key principles:**
- Progressive disclosure: users navigate from all-projects → single-project (with collapsed workflows shown by default) → expanded workflows → expanded phases
- Progressive loading: workflow details load in background after initial page render to optimize first-paint performance
- Elegant presentation: collapsible UI components with expand/collapse all controls
- Data continuity: leverages existing `/api/projects` and `/api/projects/{name}/metrics` endpoints
- Performance-first: minimal additional backend changes, efficient rendering, lazy data loading

**Scope:** Frontend enhancement to existing Sycamore WASM UI with three distinct view levels: (1) project list (existing), (2) all-projects aggregate view (new), (3) per-project workflow detail view (new).

**Integration context:** Builds on existing web server API endpoints and Sycamore UI components. May require minor backend enhancements to expose workflow-level and phase-level metrics if not already available in current API responses.

## Data Model

### Current State (Existing Code)

**Backend Types** (`src/discovery/api_types.rs`):
- `ProjectListItem` - Contains `name` and `workflow_state` only (lightweight sidebar data)
- `ProjectMetricsSummary` - Contains aggregate token/event counts from `UnifiedMetrics`

**Frontend Types** (`src/client/types.rs`):
- `DiscoveredProject` - Matches `ProjectListItem` (name + workflow_state)
- `ProjectStatistics` - Matches `ProjectMetricsSummary` (summary counts only)
- `PhaseMetrics`, `TokenMetrics`, etc. - Detailed types that match `hegel::metrics` structs but not currently exposed via API

**hegel-cli Types** (`hegel-cli/src/metrics/mod.rs`):
- `UnifiedMetrics` - Contains `hook_metrics`, `token_metrics`, `state_transitions`, `phase_metrics`, `git_commits`
- `PhaseMetrics` - Per-phase breakdown with `phase_name`, `start_time`, `end_time`, `duration_seconds`, `token_metrics`, `bash_commands`, `file_modifications`, `git_commits`
- Supports archived workflows via `parse_unified_metrics(path, include_archives: bool)`

**Current API Endpoints** (`src/server_mode.rs`):
- `GET /api/projects` - Returns `Vec<ProjectListItem>` (name + workflow_state only)
- `GET /api/projects/{name}/metrics` - Returns `ProjectMetricsSummary` (aggregate counts, no phase breakdown)

### Required Changes

**New Backend Types** (create new file `src/api_types.rs`):

```rust
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
```

**Implementation Notes:**
- Build `WorkflowSummary` by grouping `UnifiedMetrics.phase_metrics` by `workflow_id` from `state_transitions`
- `UnifiedMetrics.phase_metrics` already contains all phase data (metrics, timestamps, etc.) - just repackage it
- Determine `WorkflowStatus` by checking if any phase has `end_time: None` (active) vs all phases completed
- `total_metrics` per workflow = sum of metrics across grouped phases
- Archives already included via `parse_unified_metrics(path, include_archives: true)` - no special handling needed

**Modified API Endpoints** (update in `src/server_mode.rs`):

```rust
// GET /api/all-projects - New endpoint for all-projects aggregate view
let api_all_projects = warp::path!("api" / "all-projects")
    .map(move || {
        // Iterate all projects, load statistics if needed
        // Sum up metrics across all projects
        // Return AllProjectsAggregate
    });

// GET /api/projects/{name}/metrics - UPDATE existing endpoint to include workflow detail
let api_metrics = warp::path!("api" / "projects" / String / "metrics")
    .map(move |name: String| {
        // Load UnifiedMetrics with include_archives=true
        // Build ProjectMetricsSummary (existing logic)
        // Group phase_metrics by workflow_id from state_transitions
        // Build WorkflowSummary for each workflow
        // Return ProjectInfo { summary, detail }
    });
```

**Breaking Changes:**
- `/api/projects/{name}/metrics` response structure changes from `ProjectMetricsSummary` to `ProjectInfo`
- `ProjectInfo` contains both `summary` (ProjectMetricsSummary) and `detail` (ProjectWorkflowDetail)
- Frontend must be updated simultaneously with backend

**New Frontend Types** (add to `src/client/types.rs`):

```rust
// Mirror backend types for client-side use
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
    pub summary: ProjectStatistics,
    pub detail: ProjectWorkflowDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWorkflowDetail {
    pub current_workflow_state: Option<WorkflowState>,
    pub workflows: Vec<WorkflowSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSummary {
    pub workflow_id: String,
    pub mode: String,
    pub status: String, // Serialized from backend enum: "Active" | "Completed" | "Aborted"
    pub current_phase: Option<String>,
    pub phases: Vec<PhaseSummary>,
    pub total_metrics: PhaseMetricsSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseSummary {
    pub phase_name: String,
    pub status: String, // Serialized from backend enum: "InProgress" | "Completed"
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_seconds: u64,
    pub metrics: PhaseMetricsSummary,
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
```

**New Frontend Components** (add to `src/client/components/`):

- `all_projects_view.rs` - All-projects aggregate dashboard component
- `workflow_detail_view.rs` - Per-project workflow detail with collapsible workflows/phases

**Modified Components:**
- `src/client/mod.rs` - Add routing/view switching logic
- `src/client/components/sidebar.rs` - Add "All Projects" link at top with separator below it (above project list)
- `src/client/components/metrics_view.rs` - Replace with workflow detail view (breaking change)

## Core Operations

### Navigation: All-Projects View

**Syntax:** User clicks "All Projects" link at top of sidebar (above project list, separated by horizontal line)

**Behavior:**
- Fetches aggregate data from `/api/all-projects` endpoint
- Displays summary cards with total project count and aggregate token/event metrics
- Summary metrics are summed across all discovered projects

**Validation:**
- Returns empty aggregate when no projects discovered
- API response matches `AllProjectsAggregate` schema

### Navigation: Per-Project Workflow Detail

**Syntax:** User clicks project name from project list or all-projects view

**Behavior:**
- Initial render shows project header and collapsed workflow list (minimal data from existing cache)
- Workflows load in collapsed state by default with summary metrics visible
- Full workflow details (phases, per-phase metrics) load progressively in background
- Once background load completes, workflows become expandable
- Each workflow is collapsible/expandable to reveal phase breakdown
- Within each workflow, phases are listed with individual metrics
- Phases can be collapsed/expanded individually
- "Expand All" / "Collapse All" buttons control all workflows and phases at once

**Validation:**
- Returns empty workflows array when project has no workflows
- API response matches `ProjectInfo` schema
- Workflow status correctly reflects phase completion state

### UI Interaction: Collapse/Expand Controls

**Syntax:** User clicks workflow header, phase header, or global buttons

**Behavior:**
- Clicking workflow header toggles that workflow's phase list
- Clicking phase header toggles that phase's detailed metrics
- "Expand All" button expands all workflows and all phases (overrides any manual collapse state)
- "Collapse All" button collapses all workflows (phases become hidden)
- State persists during session only (resets on page reload)

**Validation:**
- Collapse/expand state updates correctly in component state
- Visual indicators update to reflect state (▼ for expanded, ▶ for collapsed)

## Test Scenarios

**Backend (TDD unit tests with `cargo test`):**
- Grouping `phase_metrics` by `workflow_id` produces correct workflow groupings
- `AllProjectsAggregate` sums metrics correctly across multiple projects
- `ProjectInfo` serializes/deserializes correctly
- `/api/all-projects` endpoint returns valid JSON
- `/api/projects/{name}/metrics` endpoint returns valid `ProjectInfo` JSON

**Frontend (WASM tests with `wasm-pack test --headless --firefox`):**
- All-projects view signal updates correctly when aggregate data loads
- Project detail view signal updates correctly when workflow data loads
- Collapse/expand state signals toggle correctly on user interaction
- "Expand All" button sets all workflow/phase collapse signals to expanded
- "Collapse All" button sets all workflow collapse signals to collapsed
- Navigation signal updates correctly when switching between views
- Test pattern: See `TESTING.md` for WASM test infrastructure and helpers

**Build verification:**
- Backend builds: `cargo build --release --features server`
- Frontend builds: `trunk build --release`
- No compilation errors

## Success Criteria

- `/api/all-projects` endpoint returns `AllProjectsAggregate` with correct schema
- `/api/projects/{name}/metrics` endpoint returns `ProjectInfo` with correct schema
- `ProjectInfo.summary` contains aggregate metrics matching existing `ProjectMetricsSummary`
- `ProjectInfo.detail.workflows` array contains grouped workflows by `workflow_id`
- Each `WorkflowSummary` contains phases grouped correctly from `UnifiedMetrics.phase_metrics`
- Workflow status correctly determined from phase completion state
- Frontend builds successfully without errors
- All backend unit tests pass

## Optional Human Testing

After autonomous implementation is complete, the following can be manually verified:
- All-projects view displays aggregate counts correctly
- Project detail view shows summary metrics and workflow breakdown
- Workflows appear collapsed by default
- Expand/collapse controls are responsive
- Visual indicators (▼/▶) update correctly
- Navigation flows smoothly between views
- Loading states appear during API fetches
- Empty states display appropriately
- UI remains responsive with many projects/workflows

## Out of Scope (Planned for Future)

- **State persistence** - Collapse/expand state will reset on page reload (localStorage persistence planned for later)
- **Pagination** - All workflows displayed without pagination (pagination/virtual scrolling planned for later)
- **Search/filtering** - No search or filter features for workflows (planned for later)
