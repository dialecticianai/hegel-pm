# src/discovery/

Auto-discovery and state management for Hegel projects across the filesystem.

## Structure

```
discovery/
├── mod.rs              Module exports and public API surface
├── engine.rs           DiscoveryEngine orchestration (caching, background refresh)
├── config.rs           DiscoveryConfig (search roots, exclusions, cache path, validation)
├── walker.rs           Filesystem traversal to locate .hegel/ directories
├── discover.rs         Core discovery logic (scan → load state → construct projects)
├── project.rs          DiscoveredProject model (workflow state, lazy metrics loading)
├── state.rs            Workflow state extraction from .hegel/state.json via hegel-cli FileStorage
├── statistics.rs       Type alias to hegel::metrics::UnifiedMetrics
├── api_types.rs        Lightweight API response types (ProjectMetricsSummary for /api endpoints)
├── cache_manager.rs    Async cache persistence with deduplication and background worker
└── cache.rs            Persistent cache with atomic writes and expiration
```

## Key Patterns

**Abstraction boundary**: All .hegel data access via hegel-cli library (never direct file reads)
**Lazy loading**: Metrics loaded on-demand to keep discovery fast
**Cache invalidation**: Configurable TTL with atomic updates
