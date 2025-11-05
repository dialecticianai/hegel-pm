# hegel-pm Architecture

Tech stack and architectural decisions for multi-project Hegel dashboard

---

## Technology Stack

**Language**: Rust (stable, edition 2021)
**Rationale**: Memory safety, zero-cost abstractions, strong type system. Consistency with hegel-cli ecosystem.

**Web Framework**: Sycamore (reactive web framework)
**Rationale**: Rust-native reactive UI, WASM compilation, fine-grained reactivity model suitable for real-time dashboard updates.

**Key Dependencies**:
- **hegel-cli** (as library dependency) - Reuse existing storage, metrics, and state parsing modules
- **serde** + **serde_json** - JSONL parsing (already in hegel-cli)
- **notify** (v6.0) - File watching for live updates (already in hegel-cli)
- **walkdir** (v2.5) - Recursive directory traversal for project discovery (already in hegel-cli)
- **chrono** - Timestamp parsing from workflow_id (already in hegel-cli)
- **anyhow** - Error handling (already in hegel-cli)

**Decisions Made**:
- **HTTP backend**: Swappable architecture (warp or axum via feature flags)
- **Data layer**: Message-passing worker pool with lock-free caching (DashMap)
- **Build system**: Trunk (WASM bundler, generates `static/` artifacts from `index.html`)
- **Deployment**: WASM in browser served by local HTTP server on `localhost:3030`
- **API design**: Lightweight JSON responses (summary counts, not full data arrays)

**Still To Explore**:
- Sycamore routing for multi-view dashboard
- Live state updates
- WASM bundle optimization (code splitting, lazy loading)

---

## Core Architectural Decisions

### Decision 1: Web UI Sidecar to hegel-cli

**Choice**: hegel-pm is a companion tool that depends directly on hegel-cli as a library

**Rationale**:
- **Single source of truth**: All state parsing, metrics logic lives in hegel-cli
- **No duplication**: Reuse proven `storage`, `metrics`, `theme` modules
- **Bundled extension model**: PM is a web UI frontend for CLI's data layer
- **Clear ownership**: CLI owns `.hegel/` format and parsing, PM just visualizes

**Implementation**:
- Dependency: `hegel = { path = "../hegel-cli" }` in Cargo.toml
- Import directly: `use hegel::storage::State`, `use hegel::metrics::HookEvent`
- CLI remains standalone: No reverse dependency (CLI doesn't know PM exists)
- Workspace integration: Both live in hegel-workspace as separate repos/submodules

**Tradeoffs**:
- Some unused dependencies: PM pulls in CLI deps (clap, ratatui) but doesn't use them
- Version coupling: PM tied to CLI version
- **Acceptable**: Simplicity and code reuse outweigh minor dependency bloat

**Alternatives rejected**:
- **hegel-core extraction**: Premature abstraction, added complexity
- **Copy code**: DRY violation, format divergence risk
- **Duplicate parsing**: Maintenance burden, bug inconsistencies

### Decision 2: Read-Only Dashboard (No State Mutation)

**Choice**: Dashboard reads `.hegel/` state files but does not write them

**Rationale**:
- **Single source of truth**: Hegel CLI remains authoritative for workflow control
- **Simplicity**: No concurrent write coordination needed
- **Transparency**: State changes always traceable to CLI commands
- **Future extensibility**: Can add write operations later (trigger `hegel next` from UI)

**Tradeoffs**:
- Limited interactivity in V1
- Users must switch to terminal for workflow control
- Future feature: UI buttons for `hegel next`, `hegel restart` require write logic

### Decision 3: Auto-Discovery via Filesystem Walking

**Choice**: Recursively walk configured root directory (`~/Code` default) to find `.hegel/` folders

**Rationale**:
- **Zero configuration**: No manual project registration required
- **Always in sync**: Projects automatically appear/disappear as `.hegel/` dirs are created/removed
- **Matches git mental model**: "Find all git repos" is familiar pattern

**Tradeoffs**:
- Performance cost on large codebases (mitigated by caching, incremental scans)
- May discover abandoned/archived projects (filterable in UI)
- No explicit project ordering (sorted by last activity or alphabetically)

**Alternatives considered**:
- **Manual registration**: Better control but adds friction
- **Config file with project list**: Requires maintenance, gets stale
- **Watch `~/Code` for new `.hegel/` dirs**: Complex, OS-specific

### Decision 4: Live Updates via File Watching

**Choice**: Use `notify` crate to watch all discovered `.hegel/` directories

**Rationale**:
- **Real-time dashboard**: Reflects state changes within 500ms target
- **Efficient**: Event-driven, no polling overhead
- **Reuses hegel-cli dependency**: `notify` already in dependency tree

**Tradeoffs**:
- File descriptor limits on systems with many projects (watch selectively)
- Cross-platform inconsistencies (fsevents vs inotify)
- Complexity vs polling (but better UX)

**Alternatives considered**:
- **Polling every N seconds**: Simpler but wasteful, laggy UX
- **No live updates**: Refresh on demand, poor for active development
- **WebSocket to CLI**: Over-engineered, requires CLI process

### Decision 5: Sycamore Reactive Web Framework

**Choice**: Sycamore for UI rendering

**Rationale**:
- **Rust-native**: No JavaScript, type-safe UI code
- **Fine-grained reactivity**: Efficient updates for live data
- **WASM compilation**: Runs in browser or as desktop app (future: Tauri)

**Tradeoffs**:
- Smaller ecosystem vs React/Vue
- Learning curve for contributors unfamiliar with Rust UI
- WASM bundle size (~500KB typical)

**Alternatives considered**:
- **Yew**: More mature but coarser reactivity model
- **Leptos**: Newer, less stable ecosystem
- **React + Rust backend**: Split-language complexity

### Decision 6: Lightweight API Responses

**Choice**: Server returns aggregate summary counts, not full data arrays

**Rationale**:
- **Performance**: Minimal response sizes for fast transmission
- **Client-side speed**: Instant JSON deserialization (no UI freezing)
- **Bandwidth**: Minimal payload for local HTTP server
- **Caching**: Small responses easy to cache in-memory

**Implementation**:
- `ProjectListItem` type for `/api/projects` (name + workflow_state only, ~60-80% reduction)
- `ProjectMetricsSummary` type with pre-computed aggregates:
  - Counts only (tokens, events, commands, files, phases)
  - `total_all_tokens` computed on backend (eliminates frontend arithmetic)
- `ProjectStatistics` (full UnifiedMetrics) used server-side only
- Response caching: serialized JSON stored per project
- Async cache persistence: background worker with deduplication
- Archive-aware: `parse_unified_metrics(..., include_archives=true)`
  - Merges archived workflows with live data
  - Uses pre-computed totals from archives
  - Fast O(1) access vs re-parsing all JSONL logs

**Tradeoffs**:
- Less detailed data in UI (can't show individual bash commands)
- Future drill-down features require additional endpoints
- **Acceptable**: Dashboard summary view prioritizes speed over detail

**Alternative rejected**:
- **Full data arrays**: Caused browser freezing on large payloads (megabytes)

### Decision 7: Swappable HTTP Backend Architecture

**Choice**: Abstract HTTP layer with trait-based backend selection (warp or axum)

**Rationale**:
- **Flexibility**: Different backends for different deployment scenarios
- **Compile-time selection**: Zero runtime overhead, single backend per build
- **Future-proofing**: Easy to add new backends (hyper, actix-web) without changing data layer
- **Performance isolation**: HTTP layer separate from data layer (worker pool)

**Implementation**:
- `HttpBackend` trait with `run()` method
- Feature flags: `warp-backend` (default), `axum-backend`
- Compile-time mutual exclusion (build error if both enabled)
- Both backends delegate I/O to `data_layer::WorkerPool` via message passing
- `ServerConfig` struct for backend-agnostic configuration

**Data Layer Design**:
- Message-passing worker pool (tokio mpsc channels)
- Pre-serialized JSON caching (DashMap for lock-free reads)
- Parallel cache misses (tokio::spawn for concurrent loading)
- Zero blocking I/O in HTTP handlers (all requests → DataRequest messages)

**Tradeoffs**:
- Additional abstraction layer vs direct warp usage
- Slightly more complex build configuration (feature flags)
- **Acceptable**: Flexibility and testability outweigh minimal complexity

**Alternatives considered**:
- **Warp only**: Simpler but locked into single framework
- **Runtime selection**: Higher complexity, runtime overhead
- **Separate binaries**: Build complexity, code duplication

---

## System Boundaries

### Internal (hegel-pm owns)
- **Project discovery**: Recursive filesystem walking
- **Data layer**: Worker pool with message passing, lock-free response caching
- **HTTP backends**: Swappable warp/axum implementations (trait-based abstraction)
- **Web server**: Local HTTP server for dashboard (pluggable backend)
- **UI rendering**: Sycamore components (project cards, workflow graphs, metrics charts)
- **API layer**: Lightweight response types (`ProjectMetricsSummary`) for metrics endpoint
- **Cache management**: Pre-serialized JSON cache (DashMap), parallel cache misses
- **File watching**: Monitor `.hegel/` directories for changes (future feature)
- **Configuration**: User prefs (root directory, cache location)

### External (dependencies on hegel-cli)
- **State parsing**: Reuse `storage::State`, `storage::WorkflowState` structs
- **JSONL parsing**: Reuse `metrics::hooks`, `metrics::states`, `metrics::transcript` modules
- **Workflow definitions**: Embedded workflows from hegel-cli
- **Theme**: Reuse `theme.rs` for consistent terminal/web colors

### External (integration points)
- **Filesystem**: Read `.hegel/` directories (user owns state)
- **Browser**: Serve dashboard via HTTP on `localhost:PORT`
- **Hegel CLI**: Users control workflows via CLI, PM observes changes

---

## Known Constraints

**Performance**:
- Auto-discover 10+ projects in <2 seconds (target: ~100-200ms per project)
- UI updates within 500ms of `.hegel/` file changes
- Dashboard initial load <1 second for typical workspace

**Memory**:
- <50MB memory footprint for 10 active projects
- Bounded growth: Cache state, discard stale data

**Platform**:
- macOS/Linux initially (file watching differs)
- Windows future (notify crate abstracts platform differences)

**Compatibility**:
- Must parse hegel-cli v0.0.4+ state files
- Graceful degradation for older/invalid state formats
- No breaking changes to `.hegel/` format (PM is read-only)

**Security**:
- Local-only (no network access)
- Read-only filesystem access to `.hegel/` dirs
- No code execution from parsed state

---

## Open Questions (Discovery Phase)

**Resolved**:
- [x] **Web server choice**: Swappable backend (warp default, axum available)
- [x] **Data layer architecture**: Message-passing worker pool with lock-free caching
- [x] **HTTP abstraction**: Trait-based backend selection with compile-time mutual exclusion
- [x] **Project discovery caching**: Implemented persistent cache with atomic writes
- [x] **API optimization**:
  - Lightweight `ProjectListItem` for project list (~60-80% reduction)
  - `ProjectMetricsSummary` with pre-computed aggregates (total_all_tokens)
  - Archive-aware metrics (include_archives=true, uses pre-computed totals)
  - Pre-serialized JSON cache (DashMap) for zero-copy serving
- [x] **Concurrent access**: Lock-free reads, parallel cache misses
- [x] **Component structure**: Refactored to `components/` directory (sidebar.rs, metrics_view.rs)
- [x] **Multi-project commands**: `hegel-pm hegel` xargs-style passthrough for running hegel commands across all projects

**To Investigate**:
- [ ] **Live update mechanism**: File watching vs manual refresh (deferred to future feature)
- [ ] **WASM bundle optimization**: Code splitting, lazy loading for faster initial load?

**No Longer Relevant**:
- ~~**JSONL parsing performance**~~: Solved via archive system (pre-computed totals, no re-parsing)

---

## Non-Functional Requirements

**Reliability**:
- Crashes don't affect `.hegel/` state (read-only)
- Graceful handling of malformed JSONL (log error, continue)
- Auto-recovery from file watcher failures (fallback to polling)

**Performance**:
- Sub-second for common operations (project list, workflow state view)
- Responsive UI updates (<100ms for user interactions)
- Efficient incremental updates (don't re-parse entire JSONL on change)

**Maintainability**:
- <200 lines per implementation module (Hegel standard)
- ≥80% line coverage (TDD discipline)
- Reusable Sycamore components (project card, workflow graph, metrics chart)

**Testability**:
- Mock filesystem for discovery tests
- Fixture `.hegel/` directories for state parsing tests
- Component tests for Sycamore UI logic

**Portability**:
- Minimal platform-specific code (abstract via `notify`, `walkdir`)
- WASM-compatible dependencies (no native-only crates)

---

## Phased Architecture Approach

### Phase 1: Single-User Local Dashboard (Current)
- Read-only state visualization
- Auto-discovery from `~/Code`
- File watching for live updates
- Local HTTP server on `localhost:PORT`

### Phase 2: Interactive Controls (Future)
- Trigger `hegel next`, `hegel restart` from UI
- Write operations to `.hegel/state.json`
- Concurrent access coordination with CLI

### Phase 3: Multi-User Team Dashboard (Commercial)
- User authentication and authorization
- Project ownership and sharing
- Centralized state server (optional)
- Real-time collaboration indicators

**Design principle**: Build data models for Phase 3 (include user_id, project ownership) but only render Phase 1 UI.

---

## Code Reuse Strategy

**From hegel-cli** (canonical implementation):
```rust
// hegel-cli owns these modules, hegel-pm reuses them
use hegel::storage::{State, WorkflowState, SessionMetadata, MetaMode};
use hegel::metrics::{HookEvent, StateTransition, TranscriptMetrics};
use hegel::theme::{HegelTheme, ColorPalette};
```

**New in hegel-pm** (web UI layer):
- Sycamore UI components (project cards, workflow visualizations)
- Web server setup (local HTTP server)
- Project discovery logic (recursive `.hegel/` scanning)
- File watching orchestration (live updates)
- Dashboard layout and routing (multi-view UI)

**Ownership boundary**:
- **hegel-cli owns**: `.hegel/` format, state parsing, JSONL schemas, metrics extraction
- **hegel-pm owns**: Web UI, HTTP server, browser rendering, dashboard layout
- **No shared core**: Direct dependency, not extracted library

---

## Summary

hegel-pm is a **web UI sidecar** to hegel-cli, not a standalone tool. It depends directly on hegel-cli for all state parsing, metrics extraction, and JSONL handling - the CLI owns the data layer, PM owns the visualization layer. This architecture prioritizes **transparency** (read-only, local-first), **code reuse** (no duplication of proven logic), and **future extensibility** (team features in data models, interactive controls later). Core unknowns center on web framework choices, file watching scalability, and Sycamore best practices - all surfaced for Discovery phase investigation.
