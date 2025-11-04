# src/cli

CLI command implementations for discovering and inspecting Hegel projects.

## Structure

```
cli/
├── discover/
│   ├── mod.rs       Command dispatch, sort column validation
│   ├── list.rs      Lightweight project listing (name, path, size, timestamp)
│   ├── show.rs      Single project detail view (workflow state, metrics)
│   ├── all.rs       Aggregate table with sorting and optional benchmarking
│   └── format.rs    Output formatting utilities (sizes, timestamps, paths, durations)
└── hegel.rs         Run hegel commands across all projects (xargs-style passthrough)
```

## Key Patterns

**Cache-first**: All commands use cached discovery results unless `--no-cache` is passed
**Dual output**: Human-readable by default, `--json` for machine consumption
**Lazy metrics**: List command skips metrics loading, show/all load on demand
**Sort validation**: Central validation ensures column names are valid before sorting
