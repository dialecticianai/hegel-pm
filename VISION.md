# hegel-pm Vision

Discovery library and CLI for finding and tracking Hegel projects across the filesystem.

---

## Problem Statement

Developers using Hegel methodology across multiple projects need a reliable way to discover, track, and query project state without manual configuration. Each project's `.hegel/` directory contains rich workflow state, but finding all projects and aggregating their data requires custom tooling.

The current reality:
- **Manual tracking**: Developers must remember which directories contain Hegel projects
- **No cross-project queries**: Can't easily answer "show me all projects in CODE phase" or "which project has the most recent activity"
- **Repetitive CLI invocations**: Running `hegel status` in multiple directories to get overall picture
- **State aggregation complexity**: Combining metrics across projects requires custom scripts

For tools like hegel-pm-web (web dashboard) and future ecosystem tools, there's no standardized way to discover and query Hegel project state. The cost: every tool reimplements discovery logic.

---

## Target Users

**Primary**: Tool developers building on top of Hegel ecosystem (e.g., hegel-pm-web dashboard, CI integrations, analysis tools). These tools need reliable project discovery and state extraction.

**Secondary**: CLI users who want cross-project queries ("list all active projects", "show metrics for projects in ~/Code")

**Not for**:
- Individual project workflow management (that's hegel-cli's job)
- Real-time state synchronization (discovery is snapshot-based)
- Projects without `.hegel/` directories

---

## Solution Approach

**Core insight**: Treat project discovery like git repository discovery - walk the filesystem, recognize `.hegel/` markers, extract and cache state.

**Key capabilities**:
- **Auto-discovery**: Recursive filesystem walking to find all `.hegel/` directories in configured roots (default: `~/Code`)
- **State extraction**: Parse `.hegel/state.json`, workflow archives, metrics files via hegel-cli library
- **Smart caching**: Persist discovered projects with timestamps, only re-scan when needed
- **Library API**: Clean Rust API for consumers (DiscoveryEngine, ProjectInfo, etc.)
- **CLI interface**: Commands for listing, showing, and querying discovered projects

**What we're NOT doing**:
- ❌ Not a visualization tool (that's hegel-pm-web)
- ❌ Not modifying workflow state (read-only, hegel-cli manages state)
- ❌ Not a database (cache is lightweight, filesystem is source of truth)
- ❌ Not monitoring for real-time changes (consumers can poll or implement file watching)

---

## Success Criteria

### Qualitative
- [ ] Tool developers can integrate project discovery with <10 lines of code
- [ ] Discovery is fast enough to run on every dashboard load
- [ ] Cache invalidation strategy is obvious and predictable
- [ ] CLI provides useful cross-project queries

### Quantitative
- [ ] Discovers 10+ projects from `~/Code` in <2 seconds (first run)
- [ ] Cached discovery completes in <100ms
- [ ] Handles 100+ projects without performance degradation
- [ ] Zero manual configuration required for default use case
- [ ] Test coverage ≥80%

---

## Guiding Principles

### 1. **Library-first, CLI-second**
The primary interface is the Rust library API. CLI commands are thin wrappers that demonstrate library capabilities.

### 2. **Fast defaults, configurable when needed**
Auto-discover from `~/Code` with sensible exclusions (`node_modules`, `target`). Allow configuration for advanced users.

### 3. **Filesystem is source of truth**
Never cache data that belongs in `.hegel/` directories. Cache discovery results (which projects exist, where they are) but always read fresh state from hegel-cli.

### 4. **Fail gracefully**
If one project's `.hegel/` is corrupted, continue discovering others. Surface errors but don't crash the entire discovery process.

### 5. **Depend on hegel-cli for state parsing**
Don't reimplement JSONL parsing or state interpretation. Use hegel-cli's library API. This ensures consistency across the ecosystem.

---

## Design Philosophy (from Hegel LEXICON)

**Infrastructure compounds**: Build reusable discovery patterns that future tools can leverage.

**No black boxes**: If discovery filters a directory, log why. If state extraction fails, report the error clearly.

**Artifacts disposable, clarity durable**: The value is knowing which projects exist and their current state, not the discovery cache itself.

---

## Non-Goals (This Stage)

- Real-time file watching (consumers can implement if needed)
- State modification (read-only, hegel-cli manages state)
- Remote/cloud project discovery (local filesystem only)
- Git integration (orthogonal concern)
- Project dependency graphing (future feature)

---

**In summary**: hegel-pm makes Hegel projects discoverable. It walks the filesystem, finds `.hegel/` directories, extracts workflow state via hegel-cli, and provides clean library and CLI interfaces for consumers. Success means any tool can reliably find and query Hegel projects without reimplementing discovery logic.
