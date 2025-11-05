# src/data_layer/

Message-passing worker pool for async I/O operations and response caching.

## Structure

```
data_layer/
├── mod.rs              Module exports and WorkerPool API
├── worker.rs           WorkerPool implementation (message dispatch, cache orchestration)
├── messages.rs         DataRequest enum and DataError types for worker communication
└── cache.rs            ResponseCache (DashMap-based lock-free cache for pre-serialized JSON)
```

## Key Patterns

**Message passing**: All I/O requests go through DataRequest messages via mpsc channels
**Lock-free reads**: DashMap enables concurrent cache reads without blocking
**Pre-serialized caching**: Cache stores JSON bytes, not deserialized objects
**Parallel cache misses**: Each cache miss spawns tokio task for parallel loading
