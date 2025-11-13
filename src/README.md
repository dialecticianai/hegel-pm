# src/

Core application modules for Hegel project discovery.

## Structure

```
src/
├── lib.rs              Library root exposing discovery module
├── main.rs             Binary entry point for CLI commands
├── cli.rs              CLI subcommand definitions (discover, hegel)
├── debug.rs            Debug utilities and logging helpers
│
├── cli/                CLI command implementations
│   └── See cli/README.md
│
├── discovery/          Project discovery engine and data models
│   └── See discovery/README.md
│
└── test_helpers.rs     Shared test utilities (workspaces, fixtures)
```
