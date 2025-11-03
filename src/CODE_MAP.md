# src/

Core application modules for Hegel project discovery and visualization.

## Structure

```
src/
├── lib.rs              Library root exposing discovery module
├── main.rs             Binary entry point (mode dispatch)
├── cli.rs              CLI args (--discover, --refresh)
├── discovery_mode.rs   Scan and print project list
├── server_mode.rs      Warp HTTP server + auto-open browser
│
├── discovery/          Project discovery engine and data models
│   └── See discovery/CODE_MAP.md
│
├── client/             Sycamore WASM web UI for dashboard
│   └── See client/CODE_MAP.md
│
└── test_helpers.rs     Shared test utilities (workspaces, fixtures)
```
