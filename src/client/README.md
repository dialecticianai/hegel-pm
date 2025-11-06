# src/client

Sycamore WASM web UI for hegel-pm dashboard. Multi-view interface with all-projects aggregate and per-project workflow detail.

## Structure

```
client/
├── mod.rs                      App entry point with view routing, WASM initialization
├── types.rs                    Data models and View enum for navigation (AllProjects | ProjectDetail)
│
└── components/                 UI components
    ├── mod.rs                  Re-exports all view components
    ├── sidebar.rs              Left panel: project list with "All Projects" link, navigation control
    ├── all_projects_view.rs    Dashboard showing aggregate metrics across all projects
    └── workflow_detail_view.rs Per-project view with summary metrics + collapsible workflow/phase breakdowns
```

## Key Patterns

**View routing**: Pattern matching on `Signal<View>` enum for type-safe navigation
**Reactive state**: `create_signal()` for mutable reactive values, `Signal<Vec<bool>>` for index-based collapse state
**Async data**: `spawn_local()` for fetch operations, `batch()` for multi-signal updates
**Type safety**: Types match server API via serde (ProjectInfo, WorkflowSummary, PhaseSummary)
**Idiomatic Sycamore**: Signals are Copy, data cloned before view! macros, tuples for indices through Indexed
