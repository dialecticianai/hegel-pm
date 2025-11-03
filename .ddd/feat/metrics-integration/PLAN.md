# Metrics Integration Implementation Plan

Step-by-step plan to integrate hegel-cli's metrics library into hegel-pm's project discovery.

---

## Overview

**Goal:** Enable discovered projects to load comprehensive workflow metrics by calling hegel-cli's existing parser.

**Scope:** Four-step implementation:
0. Extract test workspace helper to shared test_helpers module
1. Add Serialize to hegel-cli's metrics types
2. Replace ProjectStatistics with type alias
3. Implement load_statistics with integration tests

**Methodology:** TDD throughout. Each step has tests first, then implementation, then commit.

---

## Step 0: Extract Test Workspace Helper

**Goal:** Create reusable test infrastructure for setting up complete test projects with workflow data.

**Context:** The walker module has create_test_workspace helper that builds multi-project directory structure. We need this plus helpers for creating workflow artifacts (hooks, states, transcripts).

### Step 0.a: Write Tests

Test that test helpers work correctly:
- Test helper creates valid project structure
- Test helper can create projects with state.json
- Test helper can create projects with hooks.jsonl
- Test helper can create projects with complete workflow data

### Step 0.b: Implement

Create src/test_helpers.rs module. Extract create_test_workspace from walker tests into shared module. Add helpers for creating workflow artifacts:
- Helper to create state.json with workflow state
- Helper to create hooks.jsonl with sample events
- Helper to create states.jsonl with transitions
- Helper to create complete project with all artifacts

Make helpers available to all test modules via cfg(test) attribute.

### Success Criteria
- [x] test_helpers module exists with cfg(test) attribute
- [x] create_test_workspace extracted and working
- [x] Workflow artifact helpers implemented
- [x] Walker tests use new shared helpers
- [x] All existing tests still pass

---

## Step 1: Add Serialization to hegel-cli Metrics Types

**Goal:** Make all metrics types JSON-serializable for hegel-pm's web UI.

**Context:** Work happens in hegel-cli repository. hegel-pm depends on hegel-cli via path dependency, so changes immediately available.

### Step 1.a: Write Tests

Test that metrics types serialize to JSON without errors. Create a test that constructs each metrics type with sample data and calls serde_json to_string. Verify the JSON output contains expected fields.

Test cases:
- TokenMetrics with all token counts populated
- PhaseMetrics with complete phase data
- UnifiedMetrics with full workflow data
- Empty metrics (zero counts) serialize correctly

### Step 1.b: Implement

Add Serialize and Deserialize derives to these types in hegel-cli:
- UnifiedMetrics
- TokenMetrics
- PhaseMetrics
- HookMetrics
- GitCommit
- BashCommand
- FileModification
- StateTransitionEvent

Run existing hegel-cli test suite to ensure no regressions from adding derives.

### Success Criteria
- [x] All metrics types derive Serialize and Deserialize
- [x] New serialization tests pass
- [x] All existing hegel-cli tests still pass
- [x] No behavior changes to metrics parsing logic

---

## Step 2: Replace ProjectStatistics with Type Alias

**Goal:** Eliminate custom ProjectStatistics struct and use hegel-cli's UnifiedMetrics directly.

**Context:** Work happens in hegel-pm repository. Changes span statistics module and discovery module.

### Step 2.a: Write Tests

Update existing ProjectStatistics tests to work with UnifiedMetrics type instead. Tests should verify:
- Empty statistics represented by UnifiedMetrics with zero counts
- Statistics serialization to JSON works
- is_empty check works on UnifiedMetrics (may need helper)

### Step 2.b: Implement

Replace ProjectStatistics struct definition with type alias to UnifiedMetrics. Update all imports and usages throughout hegel-pm codebase. Update module exports to re-export UnifiedMetrics as ProjectStatistics.

Delete custom ProjectStatistics implementation since all logic now comes from hegel-cli.

### Success Criteria
- [x] ProjectStatistics is type alias not custom struct
- [x] All imports updated to use hegel metrics types
- [x] Tests pass with new type
- [x] No duplication of hegel-cli types

---

## Step 3: Implement load_statistics

**Goal:** Replace TODO stub with actual metrics loading call.

### Step 3.a: Write Tests

Test loading statistics for different project scenarios:

Empty project test: Create temporary directory with empty .hegel directory. Call load_statistics. Verify it returns UnifiedMetrics with zero counts and no errors.

Full workflow test: Set up test project with complete workflow artifacts using test helpers. Call load_statistics. Verify returned metrics contain expected session ID, phase count, token totals, and activity counts.

Error handling test: Create project with invalid or missing data. Verify load_statistics either returns graceful error or empty metrics depending on failure mode.

Integration test: Use real DiscoveredProject from discovery engine. Verify load_statistics integrates correctly with project discovery flow.

### Step 3.b: Implement

Replace TODO implementation in DiscoveredProject load_statistics method with single line calling hegel metrics parse_unified_metrics, passing the hegel_dir path. Store result in statistics field.

Handle errors appropriately: propagate parse errors up to caller rather than silently swallowing them.

### Success Criteria
- [x] load_statistics calls parse_unified_metrics
- [x] Result cached in project statistics field
- [x] Empty project returns zero metrics
- [x] Full workflow returns complete data
- [x] Errors propagated with context
- [x] Integration test passes

---

## Testing Strategy

**Unit tests:** Each metrics type serialization, ProjectStatistics type alias functionality, load_statistics error cases.

**Integration tests:** Full discovery-to-metrics flow using test helpers from hegel-pm test_helpers module.

**Performance validation:** Loading stats for ten projects completes under two seconds as specified.

**Error coverage:** Missing files, corrupted data, permission denied all handled gracefully.

---

## Commit Strategy

Four commits following conventional format:

First commit in hegel-pm:
- refactor(test): extract test workspace helpers to shared module
- Includes test_helpers module creation and walker test migration

Second commit in hegel-cli:
- feat(metrics): add Serialize derives for web UI integration
- Includes derive additions and serialization tests

Third commit in hegel-pm:
- refactor(discovery): replace ProjectStatistics with UnifiedMetrics alias
- Includes type alias change and updated tests

Fourth commit in hegel-pm:
- feat(discovery): implement metrics loading via hegel-cli parser
- Includes load_statistics implementation and integration tests

---

## Risk Mitigation

**Breaking changes:** Type alias maintains API compatibility. Existing code using statistics field continues working.

**hegel-cli coupling:** Acceptable since hegel-pm explicitly depends on hegel-cli library.

**Serialization overhead:** Serialize derives are zero-cost at runtime, only impact compile time.

**Test coverage:** Comprehensive tests prevent regressions when extending metrics in future.

---

## Success Definition

Feature complete when:
- All three steps committed and pushed
- All tests passing in both repositories
- Can load metrics for any discovered project
- Metrics data matches hegel analyze output
- Ready for web UI integration in future work
