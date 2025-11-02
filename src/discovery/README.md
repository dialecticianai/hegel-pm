# discovery
Automatic discovery of Hegel projects by scanning filesystem for `.hegel/` state directories.

## Purpose
Provides zero-configuration project discovery for hegel-pm dashboard. Recursively scans configured root directories, parses workflow state using hegel-cli's storage module, and caches results for fast subsequent loads. Designed as read-only observer with graceful degradation for corrupted state.

## Key API
```rust
DiscoveryEngine::new(config) -> Result<DiscoveryEngine>
engine.get_projects(force_refresh: bool) -> Result<Vec<DiscoveredProject>>
engine.scan_and_cache() -> Result<Vec<DiscoveredProject>>
```

## Core Concepts
- **DiscoveryConfig**: Root directories, max depth, exclusions, cache location with validation
- **DiscoveredProject**: Project metadata including name, paths, workflow state, last activity, optional error
- **Cache-or-scan pattern**: Load from cache if exists, scan filesystem if not
- **Lazy statistics**: ProjectStatistics loaded on-demand (placeholder for future metrics extraction)
- **Atomic cache writes**: Temp file + rename for consistency
- **hegel-cli integration**: Delegates all state.json parsing to `hegel::storage::FileStorage`

## Gotchas
- Statistics extraction is TODO placeholder (marked for future UI implementation)
- Cache never expires automatically (manual `force_refresh` or `hegel-pm scan` command required)
- Max depth counts from root (not from where .hegel found), typical value is 10
- Corrupted state marked with error flag but included in results (not filtered out)
- Symlinks not followed to prevent infinite loops
- Exclusions match directory names exactly (not glob patterns)

## Quick Test
`cargo test --lib discovery`
