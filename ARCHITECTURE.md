# hegel-pm Architecture

Tech stack and architectural decisions for Hegel project discovery library and CLI

---

## Technology Stack

**Language**: Rust (stable, edition 2021)
**Rationale**: Memory safety, zero-cost abstractions, strong type system. Consistency with hegel-cli ecosystem.

**Key Dependencies**:
- **hegel** (hegel-cli library) - State parsing, metrics extraction, JSONL handling
- **serde** + **serde_json** - Serialization for cache persistence
- **anyhow** - Error handling
- **clap** - CLI argument parsing
- **walkdir** - Recursive directory traversal
- **chrono** - Timestamp handling

**Decisions Made**:
- **Library-first design**: Primary interface is Rust library API, CLI is thin wrapper
- **Caching strategy**: Persistent binary cache with atomic writes
- **Discovery mechanism**: Filesystem walking with configurable roots and exclusions
- **State extraction**: Delegate to hegel-cli for all `.hegel/` parsing

---

## Core Architectural Decisions

### Decision 1: Library-First Design

**Choice**: Primary interface is Rust library API (`DiscoveryEngine`), CLI commands are thin wrappers

**Rationale**:
- **Reusability**: Consumers like hegel-pm-web can integrate discovery with <10 lines of code
- **Testability**: Library API easier to test than CLI commands
- **Flexibility**: Consumers can extend or customize discovery behavior
- **Clear separation**: Library for programmatic use, CLI for human use

**Implementation**:
```rust
pub struct DiscoveryEngine {
    config: DiscoveryConfig,
    cache: Cache,
}

impl DiscoveryEngine {
    pub fn new(config: DiscoveryConfig) -> Result<Self>;
    pub fn get_projects(&self, refresh: bool) -> Result<Vec<Project>>;
}
```

**Tradeoffs**:
- More upfront API design vs simple CLI-only tool
- **Acceptable**: Library reuse justifies design effort

### Decision 2: Depend on hegel-cli for State Parsing

**Choice**: Use hegel-cli as library dependency for all `.hegel/` data access

**Rationale**:
- **Single source of truth**: hegel-cli owns `.hegel/` format and parsing logic
- **No duplication**: Reuse proven storage, metrics, state modules
- **Consistency**: All tools parse state identically
- **Future-proof**: Format changes in hegel-cli automatically propagate

**Implementation**:
- Dependency: `hegel = { path = "../hegel-cli" }` in Cargo.toml
- Import: `use hegel::storage::State`, `use hegel::metrics::parse_unified_metrics`
- Never parse JSONL directly - always via hegel-cli API

**Tradeoffs**:
- Pulls in hegel-cli dependencies (some unused by discovery)
- Version coupling with hegel-cli
- **Acceptable**: Consistency and code reuse outweigh minor dependency bloat

**Alternatives rejected**:
- **Reimplement parsing**: DRY violation, drift risk
- **Shared core library**: Premature abstraction

### Decision 3: Filesystem Walking for Discovery

**Choice**: Recursively walk configured root directories to find `.hegel/` markers

**Rationale**:
- **Zero configuration**: Auto-discover from `~/Code` (default)
- **Always in sync**: Projects appear/disappear as `.hegel/` dirs are created/removed
- **Familiar pattern**: Matches git repository discovery mental model
- **Configurable**: Users can override roots, max depth, exclusions

**Implementation**:
- `walkdir` crate for cross-platform directory traversal
- Default roots: `~/Code`
- Default exclusions: `node_modules`, `target`, `.git`, `vendor`
- Default max depth: 10 levels
- Stop traversal below `.hegel/` directories (nested projects)

**Tradeoffs**:
- Performance cost on large codebases (mitigated by caching)
- May find abandoned projects (user can filter)
- No explicit project ordering (sorted by last activity or name)

**Alternatives considered**:
- **Manual registration**: Better control but friction
- **Config file with project list**: Requires maintenance, gets stale
- **Watch filesystem for new `.hegel/`**: Complex, OS-specific

### Decision 4: Persistent Binary Cache

**Choice**: Cache discovered projects in `~/.config/hegel-pm/cache.bin` (bincode format)

**Rationale**:
- **Performance**: Subsequent discoveries complete in <100ms (vs ~2s first run)
- **Efficiency**: Skip filesystem walking when cache is fresh
- **Binary format**: Faster serialization/deserialization than JSON
- **Atomic writes**: Write to temp file, rename (no corruption on crash)

**Implementation**:
- Cache structure: `HashMap<String, DiscoveredProject>` with timestamps
- Invalidation: Check mtime of `.hegel/state.json` vs cached timestamp
- Refresh flag: Force re-scan ignoring cache
- Per-project caching: Only re-scan projects with stale cache entries

**Tradeoffs**:
- Cache can become stale (manual refresh needed)
- Disk space for cache file (minimal, ~KB per project)
- **Acceptable**: Performance gains justify maintenance overhead

**Alternatives considered**:
- **JSON cache**: Human-readable but slower
- **No cache**: Simple but slow for repeated use
- **In-memory only**: Fast but state lost on restart

### Decision 5: CLI Subcommand Structure

**Choice**: Organize CLI as subcommands: `discover`, `hegel` (xargs-style)

**Rationale**:
- **Clarity**: `hegel-pm discover list` is self-documenting
- **Extensibility**: Easy to add new command groups
- **Consistency**: Matches `hegel` CLI patterns (subcommand-based)

**Implementation**:
```bash
hegel-pm discover list              # List all projects
hegel-pm discover show <name>       # Show single project
hegel-pm discover all               # Full table with metrics

hegel-pm hegel status               # Run 'hegel status' on each project
hegel-pm hegel analyze              # Run 'hegel analyze' on each project
```

**Tradeoffs**:
- More verbose than flat commands
- **Acceptable**: Clarity and organization worth extra typing

### Decision 6: Read-Only Discovery

**Choice**: Discovery engine never modifies `.hegel/` directories

**Rationale**:
- **Single source of truth**: hegel-cli owns workflow state
- **Simplicity**: No concurrent write coordination
- **Safety**: Can't corrupt project state
- **Clear responsibility**: Discovery reads, hegel-cli writes

**Tradeoffs**:
- Can't trigger workflows from discovery
- Must use hegel-cli for state changes

---

## System Boundaries

### Internal (hegel-pm owns)
- **Discovery engine**: Filesystem walking, project detection
- **Cache management**: Persistent binary cache with atomic writes
- **CLI commands**: List, show, query operations
- **Library API**: `DiscoveryEngine`, `Project`, `DiscoveryConfig`

### External (dependencies on hegel-cli)
- **State parsing**: `storage::State`, `storage::WorkflowState`
- **Metrics extraction**: `metrics::parse_unified_metrics`
- **JSONL parsing**: All hooks, states, transcripts via hegel-cli

### External (integration points)
- **Filesystem**: Walk directories, read `.hegel/` state files
- **Cache file**: `~/.config/hegel-pm/cache.bin`
- **Consumers**: hegel-pm-web, future tools

---

## Known Constraints

**Performance**:
- Discovers 10+ projects from `~/Code` in <2 seconds (first run)
- Cached discovery completes in <100ms
- Handles 100+ projects without degradation

**Platform**:
- macOS/Linux initially
- Windows future (walkdir abstracts platform differences)

**Compatibility**:
- Must parse hegel-cli v0.0.4+ state files
- Graceful degradation for older/invalid formats

**Security**:
- Local filesystem only (no network)
- Read-only access to `.hegel/` directories
- No code execution from parsed state

---

## Open Questions (Discovery Phase)

**Resolved**:
- [x] **Caching strategy**: Binary cache with per-project invalidation
- [x] **Discovery mechanism**: Filesystem walking with walkdir
- [x] **CLI structure**: Subcommand-based (discover, hegel)

**To Investigate**:
- [ ] **Parallel discovery**: Can filesystem walking be parallelized for large workspaces?
- [ ] **Incremental updates**: File watching integration for real-time discovery?

---

## Non-Functional Requirements

**Reliability**:
- Corrupted cache falls back to fresh scan
- Malformed `.hegel/` in one project doesn't block others
- Atomic cache writes (no corruption on crash)

**Performance**:
- Sub-second for CLI commands (via caching)
- Efficient incremental scans (only re-scan stale projects)

**Maintainability**:
- <200 lines per implementation module (Hegel standard)
- ≥80% line coverage (TDD discipline)
- Clear API boundaries

**Testability**:
- Mock filesystem for discovery tests
- Fixture `.hegel/` directories for state tests
- Binary cache round-trip tests

**Portability**:
- Cross-platform (walkdir abstracts OS differences)
- Minimal platform-specific code

---

## Summary

hegel-pm is a **library-first discovery engine** for finding and tracking Hegel projects. It walks the filesystem to locate `.hegel/` directories, extracts state via hegel-cli, and caches results for fast subsequent access. The architecture prioritizes **reusability** (clean library API for consumers), **performance** (binary caching, efficient scanning), and **consistency** (all state parsing via hegel-cli). Core unknowns center on parallelization and incremental update mechanisms.
