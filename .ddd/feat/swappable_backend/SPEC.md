# Swappable Backend Specification

Decouple data layer from HTTP layer to enable pluggable backends (warp/axum/hyper) and maximize performance through parallel cache architecture.

---

## Overview

**What it does:** Separates heavy I/O operations (JSONL parsing, file reading) from HTTP response handling, enabling compile-time backend selection and eliminating blocking I/O in request handlers.

**Key principles:**
- **Performance-first**: Pre-serialized JSON cache, lock-free reads, parallel cache misses
- **Backend-agnostic**: HTTP layer swappable via feature flags (warp/axum/hyper)
- **Zero request blocking**: All disk I/O happens in worker pool, HTTP handlers only await cached data
- **Compile-time selection**: Single HTTP backend per build (no runtime overhead)

**Scope:** Refactor `src/server_mode.rs` into three layers:
1. **Data layer** (backend-agnostic): Worker pool + shared cache
2. **HTTP abstraction** (trait/module): Common interface for all backends
3. **Backend implementations**: warp, axum (hyper reserved for future)

**Integration context:**
- Integrates with existing `DiscoveryEngine` (src/discovery/engine.rs)
- Replaces current `CacheManager` with worker pool architecture
- Preserves existing API contract (`/api/projects`, `/api/projects/:name/metrics`, `/api/all-projects`)

---

## Data Model

### NEW: Data Layer Types

**Location:** `src/data_layer/messages.rs`

```rust
/// Request sent from HTTP layer to data layer
pub enum DataRequest {
    GetProjects {
        reply: oneshot::Sender<Vec<u8>>,
    },
    GetProjectMetrics {
        name: String,
        reply: oneshot::Sender<Result<Vec<u8>, DataError>>,
    },
    GetAllProjects {
        reply: oneshot::Sender<Vec<u8>>,
    },
    RefreshCache {
        project_name: Option<String>, // None = refresh all
    },
}

/// Errors from data layer operations
pub enum DataError {
    ProjectNotFound(String),
    ParseError(String),
    CacheError(String),
}
```

**Location:** `src/data_layer/cache.rs`

```rust
/// Shared cache storing pre-serialized JSON responses
pub struct ResponseCache {
    /// Map: cache key -> pre-serialized JSON bytes
    cache: Arc<DashMap<String, Arc<Vec<u8>>>>,
    /// Projects metadata (from DiscoveryEngine)
    projects: Arc<RwLock<Vec<ProjectState>>>,
}

/// Cache keys
pub enum CacheKey {
    ProjectList,
    ProjectMetrics(String),
    AllProjectsAggregate,
}
```

**Location:** `src/data_layer/worker.rs`

```rust
/// Worker pool configuration
pub struct WorkerPoolConfig {
    /// Number of worker tasks (default: num_cpus * 2)
    pub worker_count: usize,
    /// Channel buffer size for requests
    pub channel_buffer: usize,
}

/// Worker pool managing parallel cache updates
pub struct WorkerPool {
    request_tx: mpsc::Sender<DataRequest>,
    cache: ResponseCache,
    discovery_engine: Arc<DiscoveryEngine>,
}
```

### NEW: HTTP Backend Abstraction

**Location:** `src/http/mod.rs`

```rust
/// Common interface all HTTP backends must implement
#[async_trait]
pub trait HttpBackend {
    /// Start server with data layer handle
    async fn run(
        &self,
        data_tx: mpsc::Sender<DataRequest>,
        config: ServerConfig,
    ) -> Result<(), Box<dyn Error>>;
}

/// Server configuration (backend-agnostic)
pub struct ServerConfig {
    pub host: [u8; 4],
    pub port: u16,
    pub static_dir: PathBuf,
    pub open_browser: bool,
}
```

### MODIFIED: Existing Types

**Location:** `src/discovery/project.rs::ProjectState`
- **No changes** - continues to store `name`, `hegel_dir`, `workflow_state`, `statistics`

**Location:** `src/api_types.rs`
- **No changes** - existing response types remain same (`ProjectInfo`, `AllProjectsAggregate`, etc.)

**Location:** `Cargo.toml`
- **MODIFIED**: Add mutually exclusive feature flags

```toml
[features]
default = ["server", "warp-backend"]
server = []
warp-backend = ["warp", "tokio"]
axum-backend = ["axum", "tokio", "tower", "tower-http"]

[dependencies]
# HTTP backends (feature-gated)
warp = { version = "0.3", optional = true }
axum = { version = "0.7", optional = true }
tower = { version = "0.4", optional = true }
tower-http = { version = "0.5", features = ["fs"], optional = true }

# Concurrency (always needed for data layer)
tokio = { version = "1.0", features = ["full"] }
dashmap = "6.0"
async-trait = "0.1"
```

### REMOVED

**Location:** `src/discovery/cache.rs::CacheManager`
- **Replaced by** `src/data_layer/cache.rs::ResponseCache` (different architecture)

---

## Core Operations

### Operation 1: Data Layer Initialization

**Syntax:**
```rust
WorkerPool::new(config: WorkerPoolConfig, engine: Arc<DiscoveryEngine>) -> (WorkerPool, mpsc::Sender<DataRequest>)
```

**Parameters:**
- `config`: Worker count and channel buffer size
- `engine`: Shared discovery engine for accessing project metadata

**Behavior:**
1. Create `ResponseCache` with empty `DashMap`
2. Spawn worker tasks (bounded by `worker_count`)
3. Load projects from `DiscoveryEngine`
4. Pre-populate cache with serialized responses (background task)
5. Return handle (`mpsc::Sender`) for HTTP layer to send requests

**Example:**
```rust
let engine = Arc::new(DiscoveryEngine::new(config)?);
let worker_config = WorkerPoolConfig {
    worker_count: num_cpus::get() * 2,
    channel_buffer: 100,
};
let (pool, data_tx) = WorkerPool::new(worker_config, engine);
tokio::spawn(async move { pool.run().await });
```

**Validation:**
- `worker_count` must be > 0
- `channel_buffer` must be > 0
- Discovery engine must have valid configuration

### Operation 2: Cache Hit Path (Fast)

**Syntax:**
```rust
cache.get(key: &CacheKey) -> Option<Arc<Vec<u8>>>
```

**Parameters:**
- `key`: Cache key identifying the resource

**Behavior:**
1. HTTP handler calls `cache.get()`
2. `DashMap` lock-free read (no blocking)
3. Return cached JSON bytes wrapped in `Arc` (cheap clone)
4. HTTP handler returns bytes directly

**Example:**
```rust
// In HTTP handler
let (tx, rx) = oneshot::channel();
data_tx.send(DataRequest::GetProjects { reply: tx }).await?;
let json_bytes = rx.await?; // Instant if cached
// Return json_bytes as HTTP response
```

**Performance:** <10μs for cache hit (lock-free read)

### Operation 3: Cache Miss Path (Slow - Parallel)

**Syntax:**
```rust
pool.handle_cache_miss(key: CacheKey) -> Result<Arc<Vec<u8>>, DataError>
```

**Parameters:**
- `key`: Cache key for missing resource

**Behavior:**
1. Worker spawns async task for heavy I/O
2. Load project state from `.hegel/` directory
3. Parse JSONL files (`parse_unified_metrics`)
4. Build response struct (`ProjectInfo`, etc.)
5. Serialize to JSON bytes
6. Insert into cache (`Arc::new(bytes)`)
7. Return cached bytes to requester

**Example:**
```rust
// Worker pool processing DataRequest::GetProjectMetrics
match cache.get(&CacheKey::ProjectMetrics(name.clone())) {
    Some(bytes) => reply.send(Ok(bytes)).ok(),
    None => {
        // Spawn worker (doesn't block other requests)
        let bytes = load_and_cache_metrics(&name).await?;
        reply.send(Ok(bytes)).ok()
    }
}
```

**Performance:** ~50-200ms depending on JSONL size (but parallelized across projects)

### Operation 4: Backend Selection (Compile-Time)

**Syntax:**
```bash
cargo build --features warp-backend --no-default-features
cargo build --features axum-backend --no-default-features
```

**Parameters:**
- Feature flag determines which backend module is compiled

**Behavior:**
1. Cargo compiles only selected backend implementation
2. `main.rs` uses `#[cfg(feature = "warp-backend")]` to instantiate backend
3. Backend calls `HttpBackend::run()` with `data_tx` handle
4. Backend registers routes, delegates data fetching to worker pool

**Example:**
```rust
// src/main.rs
#[cfg(feature = "warp-backend")]
use crate::http::warp_backend::WarpBackend;

#[cfg(feature = "axum-backend")]
use crate::http::axum_backend::AxumBackend;

let backend = {
    #[cfg(feature = "warp-backend")]
    { WarpBackend::new() }

    #[cfg(feature = "axum-backend")]
    { AxumBackend::new() }
};

backend.run(data_tx, config).await?;
```

**Validation:**
- Exactly one backend feature must be enabled (enforced by Cargo)
- Backend must implement `HttpBackend` trait

### Operation 5: Cache Invalidation (File Watch)

**Syntax:**
```rust
data_tx.send(DataRequest::RefreshCache { project_name: Some("hegel-cli".into()) }).await
```

**Parameters:**
- `project_name`: Specific project to refresh, or `None` for full refresh

**Behavior:**
1. File watcher detects `.hegel/` changes (existing `notify` integration)
2. Send `RefreshCache` request to worker pool
3. Worker invalidates affected cache entries
4. Next request triggers cache miss → fresh data loaded

**Example:**
```rust
// File watcher callback (future feature)
watcher.on_change(|path| {
    if let Some(project_name) = extract_project_name(path) {
        data_tx.send(DataRequest::RefreshCache {
            project_name: Some(project_name)
        }).await.ok();
    }
});
```

**Validation:**
- Project name must exist in `DiscoveryEngine`
- Invalid project name logs warning, no-ops

---

## Test Scenarios

### Simple: Cache Hit Performance

**Setup:**
```rust
let (pool, data_tx) = WorkerPool::new(config, engine);
// Pre-warm cache
pool.preload_cache().await;
```

**Action:**
```rust
let start = Instant::now();
let (tx, rx) = oneshot::channel();
data_tx.send(DataRequest::GetProjects { reply: tx }).await?;
let bytes = rx.await?;
let elapsed = start.elapsed();
```

**Expected:**
- `elapsed` < 100μs (lock-free read)
- `bytes` is valid JSON
- Multiple concurrent requests don't block each other

### Complex: Parallel Cache Misses

**Setup:**
```rust
let (pool, data_tx) = WorkerPool::new(config, engine);
// Cold cache, 5 projects discovered
```

**Action:**
```rust
// Request metrics for all 5 projects concurrently
let mut handles = vec![];
for project in ["proj1", "proj2", "proj3", "proj4", "proj5"] {
    let tx = data_tx.clone();
    handles.push(tokio::spawn(async move {
        let (reply_tx, reply_rx) = oneshot::channel();
        tx.send(DataRequest::GetProjectMetrics {
            name: project.into(),
            reply: reply_tx,
        }).await.ok();
        reply_rx.await
    }));
}
let results = join_all(handles).await;
```

**Expected:**
- All 5 requests complete in ~200ms (parallel, not sequential)
- No request blocks others
- All results are `Ok(bytes)` with valid JSON
- Subsequent requests return instantly (cached)

### Error: Project Not Found

**Setup:**
```rust
let (pool, data_tx) = WorkerPool::new(config, engine);
```

**Action:**
```rust
let (tx, rx) = oneshot::channel();
data_tx.send(DataRequest::GetProjectMetrics {
    name: "nonexistent-project".into(),
    reply: tx,
}).await?;
let result = rx.await?;
```

**Expected:**
- `result` is `Err(DataError::ProjectNotFound("nonexistent-project"))`
- HTTP backend returns 404 status
- Error does not crash worker pool

### Integration: Backend Swap (warp → axum)

**Setup:**
```bash
# Build with warp
cargo build --release --features warp-backend --no-default-features
./target/release/hegel-pm
```

**Action:**
```bash
curl http://localhost:3030/api/projects
# Observe response JSON

# Rebuild with axum
cargo clean
cargo build --release --features axum-backend --no-default-features
./target/release/hegel-pm
```

**Expected:**
- Both builds serve identical JSON responses
- API contract unchanged (`/api/projects` returns same structure)
- Performance characteristics similar (both use same data layer)
- Binary size differs (different HTTP dependencies)

---

## Success Criteria

**Agent-Verifiable:**

- All existing tests pass: `cargo test --features warp-backend`
- Axum backend tests pass: `cargo test --features axum-backend`
- Build succeeds with warp: `cargo build --features warp-backend --no-default-features`
- Build succeeds with axum: `cargo build --features axum-backend --no-default-features`
- Build fails with both features: `cargo build --features warp-backend,axum-backend` (mutual exclusion)
- Cache hit benchmark: <100μs for `/api/projects` (cached)
- Parallel cache miss: 5 projects load in <500ms total (not 5x sequential time)
- Data layer tests pass independently: `cargo test --lib data_layer`
- HTTP backend tests are backend-agnostic: same test suite runs for warp and axum
- Response JSON schema unchanged: `curl` output matches existing format
- No blocking I/O in HTTP handlers: profiling shows no `parse_unified_metrics` calls in request path

**Optional Human Testing:**

- UI remains responsive during cache refresh
- Browser DevTools shows <50ms API response times (cached)
- Switching backends doesn't change user-visible behavior

---

## Out of Scope

**Deferred to future work:**
- Custom hyper-based backend (architecture supports it, implementation not included)
- File watching for cache invalidation (architecture allows it, not implemented in this feature)
- Websocket support for real-time updates
- Graceful shutdown coordination between data layer and HTTP layer
- Metrics/observability for worker pool performance
- Dynamic worker pool resizing based on load
- Persistent disk cache (currently in-memory only)
- HTTP/2 or HTTP/3 support (backend-dependent)
- TLS/HTTPS support (local-only for now)

**Explicitly not included:**
- Changes to API response schemas (existing types preserved)
- UI modifications (frontend unchanged)
- Discovery engine refactoring (reused as-is)
- Breaking changes to `.hegel/` state format

---

## Implementation Notes

**Module structure:**
```
src/
├── data_layer/
│   ├── mod.rs           # Public API: WorkerPool::new()
│   ├── cache.rs         # ResponseCache with DashMap
│   ├── messages.rs      # DataRequest/DataError enums
│   └── worker.rs        # Worker pool impl, cache miss handling
├── http/
│   ├── mod.rs           # HttpBackend trait, ServerConfig
│   ├── warp_backend.rs  # #[cfg(feature = "warp-backend")]
│   └── axum_backend.rs  # #[cfg(feature = "axum-backend")]
├── server_mode.rs       # MODIFIED: Orchestration only (no HTTP details)
└── main.rs              # MODIFIED: Backend selection via cfg
```

**Migration path:**
1. Extract data layer (no HTTP dependencies)
2. Define `HttpBackend` trait
3. Refactor existing warp code into `warp_backend.rs`
4. Implement `axum_backend.rs` matching warp behavior
5. Update `main.rs` for backend selection
6. Delete old `CacheManager` code
7. Update tests to be backend-agnostic

**Testing approach:**
- Data layer: Unit tests with mock `DiscoveryEngine`
- HTTP backends: Integration tests using `reqwest` (backend-agnostic assertions)
- Performance: Benchmark cache hit/miss paths with `criterion`
