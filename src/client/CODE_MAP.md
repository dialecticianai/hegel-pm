# src/client

Sycamore WASM web UI for hegel-pm dashboard. Renders project list and metrics in browser.

## Structure

```
client/
├── mod.rs          App entry point, WASM initialization
│   ├── App()                   # Root component (Sidebar + MetricsView)
│   └── main()                  # WASM entrypoint, mounts App to DOM
│
├── types.rs        Data models matching server API
│   ├── DiscoveredProject       # Project with name, path, workflow state
│   ├── WorkflowState           # Current node + mode
│   ├── ProjectStatistics       # Mirrors hegel::metrics::UnifiedMetrics
│   │   ├── hook_metrics        # Tool usage (bash, write, edit counts)
│   │   ├── token_metrics       # Aggregated across all workflows
│   │   ├── state_transitions   # All workflow transitions (archived + live)
│   │   ├── phase_metrics       # Per-phase breakdown with tokens/commands
│   │   └── git_commits         # Commits in session scope
│   ├── StateTransitionEvent    # Workflow state changes
│   ├── PhaseMetrics            # Per-phase aggregation
│   └── GitCommit               # Git commit metadata
│
└── components.rs   UI components
    ├── Sidebar()               # Left panel: project list with workflow states
    │   ├── Fetches /api/projects
    │   ├── Displays loading/error states
    │   └── Shows mode + phase per project
    └── MetricsView()           # Main panel: hardcoded metrics (placeholder)
        └── Token usage, activity, workflow transitions
```

## Key Patterns

**Reactive state**: `create_signal()` for mutable reactive values
**Async data**: `spawn_local()` for fetch operations
**Type safety**: Types mirror `src/discovery/project.rs` via serde

## Key Concepts

**Project-level aggregation**: Metrics aggregate across ALL workflows (archived + current session)
- Token totals include historical archived workflows
- Phase metrics show all completed phases across all sessions
- NOT session-specific - represents entire project history

## Next Steps

- Add workflow visualization graphs (phase timeline, transitions)
- Show breakdown of archived vs live sessions
- Add filtering/drill-down by workflow or time range
