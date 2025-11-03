# src/discovery/

Auto-discovery and state management for Hegel projects across the filesystem.

## Structure

```
discovery/
├── mod.rs          Module exports and public API surface
│
├── engine.rs       DiscoveryEngine orchestration (caching, background refresh)
│   ├── new()                       # Create engine with config
│   └── get_projects()              # Fetch with cache-or-scan logic
│
├── config.rs       DiscoveryConfig (search roots, exclusions, cache path)
│   ├── default()                   # ~/Code, depth 10, standard exclusions
│   └── new()                       # Custom configuration
│
├── walker.rs       Filesystem traversal to locate .hegel/ directories
│   └── walk_directories()          # Returns Vec<PathBuf> of project roots
│
├── discover.rs     Core discovery logic (scan → load state → construct projects)
│   └── discover_projects()         # Orchestrates walker + state + project
│
├── project.rs      DiscoveredProject model (workflow state, lazy metrics)
│   ├── load_statistics()           # On-demand hegel-cli metrics loading
│   └── Serializable via serde
│
├── state.rs        Workflow state extraction from .hegel/state.json
│   └── load_workflow_state()       # Uses hegel-cli FileStorage
│
├── statistics.rs   Type alias to hegel::metrics::UnifiedMetrics
│
└── cache.rs        Persistent cache with atomic writes and expiration
    ├── load()                      # Read from cache file
    └── save()                      # Atomic write via temp + rename
```

## Key Patterns

**Abstraction boundary**: All .hegel data access via hegel-cli library (never direct file reads)
**Lazy loading**: Metrics loaded on-demand to keep discovery fast
**Cache invalidation**: Configurable TTL with atomic updates
