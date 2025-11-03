# src/client

Sycamore WASM web UI for hegel-pm dashboard. Renders project list and metrics in browser.

## Structure

```
client/
├── mod.rs                      App entry point, WASM initialization, mounts App to DOM
├── types.rs                    Data models matching server API (DiscoveredProject, WorkflowState, ProjectStatistics)
└── components/                 UI components
    ├── mod.rs                  Re-exports Sidebar and MetricsView
    ├── sidebar.rs              Left panel: project list with workflow states, fetches /api/projects
    └── metrics_view.rs         Main panel: live metrics from /api/projects/{name}/metrics
```

## Key Patterns

**Reactive state**: `create_signal()` for mutable reactive values
**Async data**: `spawn_local()` for fetch operations
**Type safety**: Types match server API via serde
**Lightweight API**: Metrics endpoint returns summary counts (~243 bytes) not full data arrays
