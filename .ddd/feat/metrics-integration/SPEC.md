# Metrics Integration Specification

Expose hegel-cli's workflow metrics through hegel-pm's project discovery system.

---

## Overview

**What it does:** When hegel-pm discovers a project, it can load comprehensive workflow metrics (tokens, phases, git activity, command history) by calling hegel-cli's existing metrics parser.

**Key principles:**
- **Reuse, don't duplicate** — hegel-cli already parses all JSONL files into `UnifiedMetrics`
- **Zero custom types** — hegel-pm uses hegel-cli's types directly via import
- **On-demand loading** — Metrics loaded lazily when requested, not during discovery scan

**Scope:** Replace the stub `ProjectStatistics` implementation with hegel-cli's `UnifiedMetrics` type and implement the TODO that calls the metrics parser.

**Integration context:** hegel-pm already imports hegel-cli's storage types (`State`, `WorkflowState`, `FileStorage`). This extends that pattern to metrics types (`UnifiedMetrics`, `TokenMetrics`, `PhaseMetrics`, etc.).

---

## What We're Building

A discovered project can load its full workflow analysis on demand. This includes:

- **Session info** — which Claude session produced the work
- **Token metrics** — input/output/cache token totals across all assistant turns
- **Phase breakdown** — per-phase durations, tokens, activity (spec took 15min, used 5000 tokens, 12 bash commands)
- **Activity summary** — total events, bash commands, file modifications, state transitions
- **Git commits** — commits made during the workflow with attribution to phases
- **Top activity** — most-used commands and most-edited files

hegel-cli's `parse_unified_metrics()` handles all data extraction and aggregation from the project's `.hegel/` directory.

---

## Data Flow

1. User discovers projects via `DiscoveryEngine`
2. Projects are returned with workflow state but no metrics (fast scan)
3. User requests metrics for a specific project
4. hegel-pm calls `hegel::metrics::parse_unified_metrics(&project.hegel_dir)`
5. Result stored in `project.statistics` field
6. Web UI renders metrics (future: dashboard cards, charts, graphs)

**Performance:** Metrics loading takes ~10-100ms per project. Not done during initial discovery scan to keep that fast (<2s for 10 projects).

---

## Contract

### What hegel-pm provides

A `DiscoveredProject` can load its metrics:

```rust
let mut project = engine.find_project("my-project")?;
project.load_statistics()?;

let stats = project.statistics.unwrap();
// stats is UnifiedMetrics from hegel-cli
// Access: stats.token_metrics, stats.phase_metrics, stats.git_commits, etc.
```

The `statistics` field type changes from custom `ProjectStatistics` struct to `Option<hegel::metrics::UnifiedMetrics>`.

### What hegel-cli provides

The metrics module already exposes:
- `parse_unified_metrics(state_dir)` function
- `UnifiedMetrics` struct with all aggregated data
- Related types: `TokenMetrics`, `PhaseMetrics`, `GitCommit`, `HookMetrics`, etc.

**Requirement:** All metrics types must derive `Serialize` for JSON export to web UI.

---

## Success Criteria

**Functionality:**
- [x] Discovered projects can load metrics on demand
- [x] Metrics data matches `hegel analyze` terminal output exactly
- [x] Empty projects return zero metrics without errors
- [x] Invalid data handled gracefully (error or partial results)
- [x] Metrics serialize to JSON for web UI

**Implementation:**
- [x] `ProjectStatistics` replaced with type alias to `UnifiedMetrics`
- [x] `load_statistics()` calls `parse_unified_metrics()`
- [x] All hegel-cli metrics types derive `Serialize`
- [x] No custom data structures duplicating hegel-cli types
- [x] Comprehensive test coverage

**Performance:**
- [x] Loading metrics for 10 projects completes in <2 seconds
- [x] Metrics cached after first load (no redundant parsing)
- [x] Memory usage bounded (<100MB for 10 projects with metrics)

**Quality:**
- [x] No panics on missing or invalid files
- [x] Error messages include file paths for debugging
- [x] All existing tests still pass in both hegel-cli and hegel-pm
