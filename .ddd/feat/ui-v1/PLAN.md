# UI v1 Implementation Plan

Implementation plan for enhanced multi-level navigation with cross-project aggregates and per-project workflow details.

## Overview

**Goal**: Add two new view levels to hegel-pm web UI: (1) all-projects aggregate dashboard, (2) per-project workflow detail with collapsible workflows and phases.

**Scope**: Lean first-pass implementation focused on core functionality. No pagination, no state persistence, no advanced filtering. See SPEC.md "Out of Scope" section for deferred features.

**Priorities**:
1. Backend API endpoints return correct data structures
2. Frontend displays data with collapsible UI
3. Navigation between views works
4. Tests verify signal reactivity and data transformations

## Methodology

**TDD Discipline**:
- Write tests first (red), then implement (green), then commit
- Backend: Standard unit tests with cargo test
- Frontend: WASM tests with wasm-pack test per TESTING.md
- Test signal updates, not DOM structure
- Commit after every numbered step

**Testing Strategy**:
- Backend: Test data grouping logic, serialization, endpoint responses
- Frontend: Test signal reactivity, collapse/expand state, navigation state
- Skip: Sycamore internals, exact DOM structure, visual styling
- Reference: TESTING.md for WASM test patterns and helpers

**Commit Format**: `feat(ui-v1): complete Step N - description`

---

## Step 1: Backend API Types

### Goal

Create new API response types in dedicated module for UI v1 endpoints.

### Step 1.a: Write Tests

Test file: `src/api_types.rs` with inline test module

Write tests for:
- ProjectInfo struct serializes to JSON with all three fields (project_name, summary, detail)
- ProjectWorkflowDetail contains current_workflow_state and workflows array
- WorkflowSummary contains workflow_id, mode, status enum, phases array
- PhaseSummary contains phase_name, status enum, timestamps, metrics
- AllProjectsAggregate contains total_projects count and aggregate_metrics
- Enum serialization: WorkflowStatus and PhaseStatus serialize to expected string values

### Step 1.b: Implement

Tasks:
1. Create src/api_types.rs file
2. Define AllProjectsAggregate and AggregateMetrics structs
3. Define ProjectInfo struct wrapping ProjectMetricsSummary and ProjectWorkflowDetail
4. Define ProjectWorkflowDetail with current_workflow_state and workflows vec
5. Define WorkflowSummary with workflow_id, mode, status, current_phase, phases, total_metrics
6. Define WorkflowStatus enum (Active, Completed, Aborted)
7. Define PhaseSummary with phase_name, status, timestamps, duration, metrics
8. Define PhaseStatus enum (InProgress, Completed)
9. Define PhaseMetricsSummary with token and event counts
10. Add module to src/lib.rs
11. Import existing ProjectMetricsSummary and WorkflowState from discovery module

### Success Criteria

- src/api_types.rs file exists with all type definitions
- All types derive Debug, Clone, Serialize, Deserialize
- Serialization tests pass
- Enums serialize to correct string values
- cargo test passes
- cargo build passes

---

## Step 2: Workflow Grouping Logic

### Goal

Implement logic to group phase metrics by workflow_id from state transitions.

### Step 2.a: Write Tests

Test file: src/api_types.rs inline tests

Write tests for:
- Empty phase_metrics and state_transitions produce empty workflows vec
- Single workflow with multiple phases groups correctly by workflow_id
- Multiple workflows separate correctly into distinct WorkflowSummary entries
- Phases without end_time are marked InProgress status
- Phases with end_time are marked Completed status
- Workflow with all phases completed has Completed status
- Workflow with any InProgress phase has Active status
- Phase metrics sum correctly to workflow total_metrics
- Current phase identified correctly for active workflows

### Step 2.b: Implement

Tasks:
1. Add helper function in src/api_types.rs: build_workflow_summaries
2. Accept UnifiedMetrics as input
3. Group phase_metrics by workflow_id extracted from state_transitions
4. For each workflow group create WorkflowSummary
5. Determine workflow mode from state_transitions
6. Determine workflow status by checking if any phase has end_time None
7. Identify current_phase for active workflows from latest state transition
8. Build PhaseSummary for each phase in the workflow
9. Sum phase metrics to compute workflow total_metrics
10. Return Vec of WorkflowSummary

### Success Criteria

- build_workflow_summaries function exists and compiles
- All grouping tests pass
- Status determination tests pass
- Metrics aggregation tests pass
- No panics on empty input
- cargo test passes

---

## Step 3: All-Projects Aggregate Endpoint

### Goal

Add GET /api/all-projects endpoint that aggregates metrics across all discovered projects.

### Step 3.a: Write Tests

Test file: src/server_mode.rs inline tests (or new tests/api_tests.rs if server_mode exceeds 200 lines)

Write tests for:
- Endpoint returns AllProjectsAggregate with correct total_projects count
- Aggregate metrics sum correctly across multiple projects
- Empty project list returns zero counts
- Projects without statistics are skipped from aggregation
- Response serializes to valid JSON
- HTTP 200 status code returned

### Step 3.b: Implement

Tasks:
1. Add api_all_projects route handler in src/server_mode.rs
2. Use warp path macro for /api/all-projects
3. Lock projects mutex and iterate all DiscoveredProject entries
4. For each project with statistics, add metrics to running totals
5. Count total projects
6. Build AggregateMetrics with summed token counts, event counts, phase counts
7. Build AllProjectsAggregate with total_projects and aggregate_metrics
8. Return JSON response with warp::reply::json
9. Add route to combined routes in run function

### Success Criteria

- /api/all-projects endpoint defined and registered
- Endpoint returns AllProjectsAggregate JSON
- Aggregation logic sums metrics correctly
- Tests pass for multiple projects and empty state
- Server compiles with cargo build --features server
- Manual curl test returns expected JSON structure

---

## Step 4: Update Project Metrics Endpoint

### Goal

Modify GET /api/projects/{name}/metrics to return ProjectInfo instead of ProjectMetricsSummary.

### Step 4.a: Write Tests

Test file: src/server_mode.rs inline tests

Write tests for:
- Endpoint returns ProjectInfo with project_name field
- ProjectInfo.summary contains existing ProjectMetricsSummary data
- ProjectInfo.detail contains workflow breakdown
- Workflows grouped correctly by workflow_id
- Current workflow state included in detail
- Response serializes to valid JSON
- HTTP 404 for non-existent project
- HTTP 200 for valid project

### Step 4.b: Implement

Tasks:
1. Modify api_metrics endpoint handler in src/server_mode.rs
2. After loading UnifiedMetrics with include_archives true
3. Build ProjectMetricsSummary as before (existing logic)
4. Call build_workflow_summaries from Step 2 with UnifiedMetrics
5. Extract current_workflow_state from project.workflow_state
6. Build ProjectWorkflowDetail with current_workflow_state and workflows
7. Build ProjectInfo with project name, summary, and detail
8. Return ProjectInfo as JSON response
9. Update response cache to handle ProjectInfo type

### Success Criteria

- Endpoint returns ProjectInfo instead of ProjectMetricsSummary
- Existing summary data still present in ProjectInfo.summary
- Workflow detail populated in ProjectInfo.detail
- Tests verify structure and data correctness
- Server compiles and runs
- Manual curl test shows workflow breakdown

---

## Step 5: Frontend Types

### Goal

Add frontend type definitions mirroring backend API types.

### Step 5.a: Write Tests

Test file: tests/client_tests.rs

Write WASM tests for:
- AllProjectsAggregate deserializes from JSON
- ProjectInfo deserializes from JSON
- WorkflowSummary status field contains expected string values
- PhaseSummary status field contains expected string values
- All types have proper Clone and PartialEq traits

### Step 5.b: Implement

Tasks:
1. Add new type definitions to src/client/types.rs
2. Define AllProjectsAggregate struct matching backend
3. Define AggregateMetrics struct matching backend
4. Define ProjectInfo struct with project_name, summary, detail
5. Define ProjectWorkflowDetail struct with current_workflow_state and workflows
6. Define WorkflowSummary with String status field (not enum for WASM compat)
7. Define PhaseSummary with String status field
8. Define PhaseMetricsSummary struct matching backend
9. All types derive Debug, Clone, Serialize, Deserialize, PartialEq

### Success Criteria

- All frontend types defined in src/client/types.rs
- Types match backend API schema
- Status fields use String not enums
- Deserialization tests pass with wasm-pack test
- Frontend compiles with trunk build

---

## Step 6: All-Projects View Component

### Goal

Create all-projects aggregate dashboard component fetching and displaying cross-project metrics.

### Step 6.a: Write Tests

Test file: tests/client_tests.rs

Write WASM tests for:
- Component signal initializes to None
- Signal updates when mock aggregate data loaded
- Loading state signal toggles correctly
- Component renders without panics

### Step 6.b: Implement

Tasks:
1. Create src/client/components/all_projects_view.rs
2. Define AllProjectsView component with no props
3. Create signal for AllProjectsAggregate data
4. Create signal for loading state
5. Use create_effect to fetch from /api/all-projects on mount
6. Parse JSON response into AllProjectsAggregate
7. Update signal with fetched data
8. Build view showing total_projects count
9. Display aggregate token metrics from aggregate_metrics
10. Display aggregate event counts
11. Show loading state while fetching
12. Handle empty state when no projects exist
13. Add component to src/client/components/mod.rs

### Success Criteria

- all_projects_view.rs file exists
- Component compiles and renders
- Fetches data from /api/all-projects endpoint
- Displays aggregate metrics correctly
- Signal tests pass with wasm-pack test
- trunk build succeeds

---

## Step 7: Workflow Detail View Component

### Goal

Create per-project workflow detail component with collapsible workflows and phases.

### Step 7.a: Write Tests

Test file: tests/client_tests.rs

Write WASM tests for:
- Component accepts selected_project signal as prop
- ProjectInfo signal initializes to None
- Signal updates when project selected
- Collapse state signals initialize to all collapsed
- Expand All button sets all collapse signals to false (expanded)
- Collapse All button sets all collapse signals to true (collapsed)
- Individual workflow collapse toggle updates signal correctly
- Individual phase collapse toggle updates signal correctly

### Step 7.b: Implement

Tasks:
1. Create src/client/components/workflow_detail_view.rs
2. Define WorkflowDetailView component accepting selected_project ReadSignal
3. Create signal for ProjectInfo data
4. Create map of workflow collapse states (workflow_id -> bool signal)
5. Create nested map of phase collapse states (workflow_id -> phase_name -> bool signal)
6. Use create_effect to fetch /api/projects/{name}/metrics when selected_project changes
7. Parse JSON response into ProjectInfo
8. Update ProjectInfo signal with fetched data
9. Initialize all collapse state signals to true (collapsed)
10. Build view showing project_name at top
11. Display summary metrics from ProjectInfo.summary
12. Render Expand All and Collapse All buttons
13. Render each workflow as collapsible section
14. Show workflow summary metrics when collapsed
15. Show phase breakdown when expanded
16. Render each phase as collapsible subsection
17. Show phase metrics when phase expanded
18. Wire up collapse/expand button click handlers
19. Replace existing metrics_view.rs component with workflow_detail_view.rs in mod.rs

### Success Criteria

- workflow_detail_view.rs file exists
- Component compiles and renders
- Fetches ProjectInfo from endpoint
- Displays summary and detail sections
- Collapse/expand state signals work correctly
- Button handlers update state appropriately
- Tests verify signal reactivity
- trunk build succeeds

---

## Step 8: Sidebar Navigation

### Goal

Update sidebar to add All Projects link at top with separator.

### Step 8.a: Write Tests

Test file: tests/client_tests.rs

Write WASM tests for:
- Sidebar component renders without selected project
- Clicking All Projects updates navigation signal
- Clicking project name updates selected_project signal
- Navigation state updates correctly

### Step 8.b: Implement

Tasks:
1. Modify src/client/components/sidebar.rs
2. Add navigation signal for current view (AllProjects vs ProjectDetail)
3. Add All Projects clickable element at top of sidebar
4. Add horizontal separator below All Projects link
5. Wire click handler to update navigation signal to AllProjects
6. Update project click handlers to set navigation to ProjectDetail
7. Add CSS classes for separator and navigation items
8. Ensure selected_project signal still works for project clicks

### Success Criteria

- All Projects link appears at top of sidebar
- Separator displays between All Projects and project list
- Clicking All Projects updates navigation state
- Clicking projects still updates selected_project
- Tests verify navigation signal updates
- trunk build succeeds

---

## Step 9: Main App Routing

### Goal

Add view routing logic to main App component to switch between all-projects and project-detail views.

### Step 9.a: Write Tests

Test file: tests/client_tests.rs

Write WASM tests for:
- App component initializes with default view
- Navigation signal controls which view displays
- Switching navigation updates displayed component

### Step 9.b: Implement

Tasks:
1. Modify src/client/mod.rs App component
2. Create navigation signal (AllProjects or ProjectDetail enum or string)
3. Update Sidebar to accept navigation signal
4. Add conditional rendering based on navigation signal value
5. Show AllProjectsView when navigation is AllProjects
6. Show WorkflowDetailView when navigation is ProjectDetail
7. Pass selected_project signal to WorkflowDetailView
8. Default to AllProjects view on initial load

### Success Criteria

- App component compiles
- Navigation signal controls view rendering
- AllProjectsView displays when All Projects selected
- WorkflowDetailView displays when project selected
- Navigation flows work correctly
- Tests verify view switching
- trunk build succeeds

---

## Step 10: Integration and Manual Verification

### Goal

Verify end-to-end functionality with running server and browser testing.

### Step 10.a: Write Tests

No new tests - run all existing tests to verify integration.

Run:
- cargo test (backend unit tests)
- wasm-pack test --headless --firefox (frontend WASM tests)
- cargo build --release --features server (backend build)
- trunk build --release (frontend build)

### Step 10.b: Implement

Tasks:
1. Run test suite and fix any failures
2. Start server with cargo run --features server --release
3. Open browser to localhost:3030
4. Verify All Projects view displays aggregate metrics
5. Verify clicking project shows workflow detail
6. Verify workflows display in collapsed state
7. Verify clicking workflow expands phases
8. Verify Expand All button expands all workflows and phases
9. Verify Collapse All button collapses all workflows
10. Verify navigation back to All Projects works
11. Fix any discovered issues

### Success Criteria

- All backend unit tests pass
- All frontend WASM tests pass
- Backend builds without errors
- Frontend builds without errors
- Server starts successfully
- All Projects view displays correctly in browser
- Project detail view displays correctly in browser
- Collapse/expand controls function
- Navigation between views works
- No console errors in browser

---

## Final Commit and Summary

After Step 10 completion:
- Final commit: `feat(ui-v1): complete implementation - multi-level navigation with workflow detail`
- Verify all 10 step commits are in git log
- Verify all tests pass
- Ready for human testing per SPEC.md "Optional Human Testing" section
