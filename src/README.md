# src/

Core application modules for Hegel project discovery and visualization.

## Structure

```
src/
├── lib.rs              Library root exposing discovery, data_layer, http modules
├── main.rs             Binary entry point (mode dispatch, cfg-guarded for native only)
├── cli.rs              CLI subcommand definitions (discover, deprecated flags)
├── api_types.rs        Shared API response types (ProjectInfo, AggregateMetrics, workflow summaries)
├── discovery_mode.rs   Legacy discovery output (deprecated, kept for compatibility)
├── server_mode.rs      Server orchestration (worker pool + backend selection)
│
├── cli/                CLI command implementations
│   └── See cli/README.md
│
├── discovery/          Project discovery engine and data models
│   └── See discovery/README.md
│
├── data_layer/         Message-passing worker pool for async I/O and caching
│   └── See data_layer/README.md
│
├── http/               HTTP backend abstraction and implementations (warp, axum)
│   └── See http/README.md
│
├── client/             Sycamore WASM web UI for dashboard
│   └── See client/README.md
│
└── test_helpers.rs     Shared test utilities (workspaces, fixtures)
```
