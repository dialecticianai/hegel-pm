# src/

Core application modules for Hegel project discovery and visualization.

## Structure

```
src/
├── lib.rs              Library root exposing discovery module
├── main.rs             Binary entry point (mode dispatch)
├── cli.rs              CLI subcommand definitions (discover, deprecated flags)
├── discovery_mode.rs   Legacy discovery output (deprecated, kept for compatibility)
├── server_mode.rs      Warp HTTP server + auto-open browser
│
├── cli/                CLI command implementations
│   └── See cli/CODE_MAP.md
│
├── discovery/          Project discovery engine and data models
│   └── See discovery/CODE_MAP.md
│
├── client/             Sycamore WASM web UI for dashboard
│   └── See client/CODE_MAP.md
│
└── test_helpers.rs     Shared test utilities (workspaces, fixtures)
```
