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
- **Web server**: `warp` (async HTTP server, serves static files + JSON API)
- **Build system**: Trunk (WASM bundler, generates `static/` artifacts from `index.html`)
- **Deployment**: WASM in browser served by local warp server on `localhost:3030`
- **API design**: Lightweight JSON responses (summary counts, not full data arrays)

**Still To Explore**:
- Sycamore routing for multi-view dashboard
- Live state updates (deferred to future feature)

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
- **Performance**: ~243 byte response vs megabytes of raw event data
- **Client-side speed**: Instant JSON deserialization (no UI freezing)
- **Bandwidth**: Minimal payload for local HTTP server
- **Caching**: Small responses easy to cache in-memory

**Implementation**:
- `ProjectMetricsSummary` type with counts only (tokens, events, commands, files)
- `ProjectStatistics` (full data) used server-side only
- Response caching: serialized JSON stored per project
- Async cache persistence: background worker with deduplication

**Tradeoffs**:
- Less detailed data in UI (can't show individual bash commands)
- Future drill-down features require additional endpoints
- **Acceptable**: Dashboard summary view prioritizes speed over detail

**Alternative rejected**:
- **Full data arrays**: Caused browser freezing on large payloads (megabytes)

---

## System Boundaries

### Internal (hegel-pm owns)
- **Project discovery**: Recursive filesystem walking
- **Web server**: Local HTTP server for dashboard (warp)
- **UI rendering**: Sycamore components (project cards, workflow graphs, metrics charts)
- **API layer**: Lightweight response types (`ProjectMetricsSummary`) for metrics endpoint
- **Cache management**: Async cache persistence with background worker and deduplication
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
- [x] **Web server choice**: `warp` selected for async HTTP + static file serving
- [x] **Project discovery caching**: Implemented persistent cache with atomic writes
- [x] **API optimization**: Lightweight summary responses (ProjectMetricsSummary ~243 bytes)
- [x] **Concurrent access**: Async cache manager with mutex-free serialization
- [x] **Component structure**: Refactored to `components/` directory (sidebar.rs, metrics_view.rs)

**To Investigate**:
- [ ] **JSONL parsing performance**: Large `hooks.jsonl` files (>10K events) - streaming vs full read?
- [ ] **Live update mechanism**: File watching vs manual refresh (deferred to future feature)
- [ ] **WASM bundle optimization**: Code splitting, lazy loading for faster initial load?

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
