# CLI Discovery Interface
Command-line interface for discovering and inspecting Hegel projects across the filesystem

## Purpose
Provides three CLI subcommands exposing the discovery module: lightweight project listing (list), detailed single-project inspection (show), and aggregate cross-project metrics with sorting (all). Designed for developers, coding agents, and CI/CD pipelines needing fast access to Hegel project state without starting the web server.

## Key API
```
hegel-pm discover list [--json] [--no-cache]
hegel-pm discover show <project-name> [--json] [--no-cache]
hegel-pm discover all [--sort-by <col>] [--benchmark] [--json] [--no-cache]
```

## Core Concepts
- **Cache-first**: All commands use DiscoveryEngine cache by default (`--no-cache` forces refresh)
- **Lazy metrics**: List skips metrics for speed; show/all load UnifiedMetrics on demand
- **Dual output**: Human-readable tables by default, `--json` for machine consumption
- **Sort validation**: Columns validated before sorting (name, path, size, last-activity, tokens, events, phases, load-time)
- **Benchmark mode**: `--benchmark` on all command measures per-project metrics load time

## Gotchas
- List command shows .hegel folder size but doesn't load metrics (O(1) per project)
- All command loads metrics for every project (can be slow for 50+ projects)
- Sort by "load-time" only valid when `--benchmark` is used
- Benchmark times include disk I/O and parsing, not just computation
- Discovery engine prints cache status to stdout (may duplicate with command output)

## Quick Test
`cargo test --bin hegel-pm --features server cli::discover`
