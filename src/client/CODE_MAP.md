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
│   └── WorkflowState           # Current node + mode
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

## Next Steps

- Replace hardcoded metrics with live data from `/api/metrics/{project}`
- Add interactive project selection (sidebar → detail view)
- Implement workflow visualization graphs
