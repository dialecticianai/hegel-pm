# Swappable Backend Implementation Plan

High-level plan for refactoring server architecture into data layer and pluggable HTTP backends using TDD discipline.

---

## Overview

**Goal**: Decouple heavy I/O from HTTP handlers by introducing a worker pool data layer with pre-serialized cache, then abstract HTTP layer to support compile-time backend selection (warp/axum).

**Scope**: Refactor src/server_mode.rs into three layers: data layer (worker pool + cache), HTTP abstraction (trait), and backend implementations (warp, axum).

**Priorities**:
1. Performance - eliminate blocking I/O in request path
2. Backend agnosticism - swappable via feature flags
3. Backward compatibility - preserve existing API contract
4. Testability - isolate data layer for independent testing

**Methodology**: TDD where tests drive design (data layer interface, HTTP trait), implementation-first where integration is straightforward (backend adapters). Focus on core behavior, defer edge cases.

---

## Step 1: Data Layer Foundation

### Goal
Establish worker pool architecture with message passing and shared cache, testable independently of HTTP layer.

### Step 1.a: Write Tests

Describe test strategy for data layer isolation:
- Test worker pool initialization with valid and invalid configurations
- Test cache key generation for different request types
- Test message round-trip through channels (send request, receive reply)
- Validate worker pool responds to basic data requests with mock discovery engine
- Error cases: channel closed, invalid project names

### Step 1.b: Implement

Create data layer module structure and core types:
- Create src/data_layer directory with mod.rs, messages.rs, cache.rs
- Define DataRequest enum with oneshot reply channels for each endpoint
- Define DataError enum for project not found, parse errors, cache errors
- Define CacheKey enum matching API endpoints (project list, metrics, aggregate)
- Implement ResponseCache struct wrapping DashMap for thread-safe cache
- Add basic WorkerPoolConfig struct with worker count and buffer size
- Stub WorkerPool::new that returns channel sender and spawns placeholder task

### Success Criteria

- Cargo build succeeds with new data_layer module
- Unit tests pass for message type serialization and cache key generation
- Worker pool can be instantiated with valid configuration
- Tests verify channel communication works (send/receive round-trip)
- Invalid configurations (zero workers, zero buffer) return errors

---

## Step 2: Worker Pool Request Handling

### Goal
Implement core worker loop that processes data requests and delegates to cache hit/miss paths.

### Step 2.a: Write Tests

Describe worker pool behavior tests:
- Test worker pool processes GetProjects request and returns cached data
- Test worker pool processes GetProjectMetrics with cache hit (fast path)
- Test worker pool handles cache miss by triggering background load
- Test multiple concurrent requests don't block each other
- Test error propagation when project doesn't exist
- Validate worker pool gracefully handles oneshot receiver drop

### Step 2.b: Implement

Build worker loop and cache interaction logic:
- Implement WorkerPool::run async method with message receive loop
- Handle DataRequest::GetProjects by checking cache, return serialized bytes
- Handle DataRequest::GetProjectMetrics with cache lookup first
- Handle DataRequest::GetAllProjects by aggregating cached or computing fresh
- Implement cache miss path that spawns async task for heavy I/O
- Add placeholder for load_project_metrics function (returns mock JSON for now)
- Wire discovery engine data into cache during initialization

### Success Criteria

- Worker pool processes requests without panicking
- Tests show cache hits return in under 100 microseconds
- Tests show multiple concurrent requests complete in parallel
- Cache miss spawns separate task (doesn't block worker loop)
- Error responses correctly propagate through oneshot channels
- Worker pool continues running after individual request errors

---

## Step 3: Cache Population and Heavy I/O

### Goal
Integrate real JSONL parsing and project metrics loading into cache miss path, pre-serialize responses.

### Step 3.a: Write Tests

Describe cache loading test strategy:
- Test cache preload on startup populates all discovered projects
- Test cache miss triggers parse_unified_metrics call with correct hegel_dir
- Test serialized JSON bytes match existing API response schema
- Test cache stores Arc-wrapped bytes for cheap cloning
- Test cache invalidation removes specific project entries
- Integration test with real .hegel directory fixtures validates end-to-end

### Step 3.b: Implement

Connect data layer to existing hegel metrics parsing:
- Implement load_project_metrics using hegel::metrics::parse_unified_metrics
- Build ProjectInfo response struct from UnifiedMetrics (existing logic from server_mode.rs)
- Serialize response to JSON bytes using serde_json::to_vec
- Wrap bytes in Arc and insert into DashMap cache
- Implement preload_cache method that iterates discovered projects
- Add RefreshCache request handling that invalidates and reloads specific entries
- Update worker pool initialization to trigger preload in background task

### Success Criteria

- Cache preload completes for test workspace in under one second
- Loaded metrics match existing API schema byte-for-byte
- Cache entries are Arc-wrapped for zero-copy cloning
- Integration test using test fixtures validates correct JSON output
- RefreshCache invalidates and reloads specified projects
- Tests confirm heavy I/O happens outside worker loop (parallel execution)

---

## Step 4: HTTP Backend Abstraction

### Goal
Define trait and configuration types that all HTTP backends must implement, enabling compile-time backend selection.

### Step 4.a: Write Tests

Describe backend abstraction test strategy:
- Test ServerConfig validates required fields (host, port, static dir)
- Test HttpBackend trait requires run method with correct signature
- Mock backend implementation passes data requests through to worker pool
- Test feature flag mutual exclusion (cannot enable both warp and axum)
- Validate backend-agnostic integration tests can run against any implementation

### Step 4.b: Implement

Create HTTP abstraction layer:
- Create src/http directory with mod.rs
- Define HttpBackend trait with async run method taking data channel and config
- Define ServerConfig struct with host, port, static_dir, open_browser fields
- Add async-trait dependency for trait async methods
- Update Cargo.toml with mutually exclusive feature flags for warp-backend and axum-backend
- Make warp dependency optional, gated behind warp-backend feature
- Create http/warp_backend.rs and http/axum_backend.rs stub files with feature gates

### Success Criteria

- HttpBackend trait compiles with correct async signature
- ServerConfig validates at construction time
- Cargo build with warp-backend feature succeeds
- Cargo build with axum-backend feature succeeds
- Cargo build with both features fails with conflict error
- Build with no backend feature fails (requires at least one)

---

## Step 5: Warp Backend Migration

### Goal
Refactor existing server_mode.rs warp code into warp_backend.rs implementing HttpBackend trait, preserve all behavior.

### Step 5.a: Write Tests

Describe warp backend integration tests:
- Test warp backend serves GET /api/projects and returns valid JSON
- Test warp backend serves GET /api/projects/:name/metrics with project name
- Test warp backend serves GET /api/all-projects aggregate endpoint
- Test warp backend serves static files from configured directory
- Test warp backend returns 404 for nonexistent project metrics
- Test warp backend delegates data fetching to worker pool (no direct I/O)
- Validate response headers include correct Content-Type

### Step 5.b: Implement

Move warp implementation into backend module:
- Copy existing warp route definitions from server_mode.rs to warp_backend.rs
- Replace direct discovery engine calls with DataRequest message sends
- Update each route handler to await oneshot reply from worker pool
- Preserve existing response format (warp::reply::json for cached bytes)
- Keep static file serving logic unchanged
- Implement HttpBackend trait for WarpBackend struct
- Update server_mode.rs to instantiate WarpBackend and call trait method
- Remove old inline warp code from server_mode.rs

### Success Criteria

- Warp backend builds with warp-backend feature flag
- All three API endpoints return identical JSON to previous implementation
- Integration tests using curl or reqwest pass against warp backend
- Static files served correctly from configured directory
- Server startup logs show correct URL and opens browser
- No blocking I/O calls in warp route handlers (only channel send/await)

---

## Step 6: Axum Backend Implementation

### Goal
Implement axum backend matching warp behavior, validate both backends serve identical responses.

### Step 6.a: Write Tests

Describe axum backend test strategy:
- Test axum backend serves same three API endpoints as warp
- Test axum response JSON byte-for-byte matches warp responses
- Test axum static file serving works with tower-http
- Test axum error handling returns same status codes as warp
- Backend-agnostic integration test suite runs against both backends
- Performance comparison shows similar latency for cache hits

### Step 6.b: Implement

Create axum backend from scratch matching warp contract:
- Add axum, tower, and tower-http dependencies with axum-backend feature gate
- Implement AxumBackend struct with HttpBackend trait
- Define axum routes for /api/projects, /api/projects/:name/metrics, /api/all-projects
- Use tower-http ServeDir for static file serving
- Extract path parameters and send DataRequest messages to worker pool
- Await oneshot replies and construct axum Json responses
- Handle errors by returning appropriate StatusCode responses
- Update main.rs to select AxumBackend when axum-backend feature enabled

### Success Criteria

- Axum backend builds with axum-backend feature flag
- Axum serves identical JSON responses to warp (validated by integration tests)
- Static files accessible through axum backend
- Error responses (404 for missing projects) match warp behavior
- Integration test suite passes for both warp and axum backends
- Binary size comparison shows different HTTP dependencies loaded

---

## Step 7: Main Orchestration and Cleanup

### Goal
Wire backend selection in main.rs, remove deprecated code, update documentation.

### Step 7.a: Write Tests

Describe orchestration test strategy:
- Test main.rs selects correct backend based on feature flag
- Test worker pool and backend start successfully together
- Test server responds to requests end-to-end (data layer + HTTP)
- Test build script or CI validates mutual exclusivity of features
- Smoke test that deprecated CacheManager code is removed

### Step 7.b: Implement

Final integration and cleanup:
- Update main.rs with conditional compilation selecting backend implementation
- Instantiate WorkerPool before backend, pass channel sender to backend
- Remove old CacheManager from discovery module
- Remove inline warp code from old server_mode.rs
- Update ARCHITECTURE.md to document new layered design
- Update TESTING.md with data layer test patterns
- Add allow dead code annotations with TODO comments for RefreshCache file watching hooks
- Update README or CLAUDE.md with feature flag build instructions

### Success Criteria

- Server starts successfully with warp-backend feature
- Server starts successfully with axum-backend feature
- All tests pass for both backend configurations
- Build with both features simultaneously fails as expected
- Old CacheManager code removed from codebase
- Documentation reflects new architecture
- Allow dead code warnings suppressed for unimplemented file watching integration

---

## Out of Scope

Explicitly not implementing in this plan:
- File watching integration for cache invalidation (architecture ready, implementation deferred)
- Custom hyper backend (abstraction supports it, not building now)
- Performance benchmarking with criterion (validate manually first)
- Graceful shutdown coordination
- Metrics and observability instrumentation
- Dynamic worker pool resizing
- Persistent disk cache
