# Project Discovery Specification

Automatic discovery of Hegel projects by scanning filesystem for `.hegel/` state directories.

---

## Overview

**What it does:** Recursively walks a configurable root directory to locate all Hegel project state directories, parses their current workflow state, and aggregates basic project statistics for dashboard visualization.

**Key principles:**
- **Zero configuration**: Discovery happens automatically without manual project registration
- **Filesystem as source of truth**: No separate database or config file tracking projects
- **Reuse existing infrastructure**: Leverage hegel-cli's storage and metrics modules for all state parsing
- **Read-only observation**: Never modify discovered `.hegel/` directories
- **On-demand scanning**: Scan runs on first launch (no cache), then only via explicit `hegel-pm scan` command

**Scope:** Single-user local discovery scanning `~/Code` by default. Multi-project aggregation and statistics extraction. Does not include UI rendering or automatic rescanning (separate features).

**Integration context:**
- Depends on `hegel::storage` module for State, WorkflowState parsing
- Depends on `hegel::metrics` module for hooks.jsonl and states.jsonl parsing
- Provides discovered project list to dashboard rendering layer
- Manual rescan triggered via `hegel-pm scan` command

---

## Data Model

### Discovered Project

A discovered project represents a single Hegel project found on the filesystem.

**Structure:**
- Project path (absolute filesystem path to directory containing `.hegel/`)
- Project name (derived from directory name)
- State directory path (absolute path to `.hegel/`)
- Workflow state (parsed from `.hegel/state.json` if exists)
- Last activity timestamp (most recent file modification in `.hegel/`)
- Discovery timestamp (when this project was found)
- Statistics summary (event counts, file modification counts, workflow transitions)

**Example representation:**
```
Project: hegel-cli
Path: /Users/emadum/Code/github.com/dialecticianai/hegel-cli
State Dir: /Users/emadum/Code/github.com/dialecticianai/hegel-cli/.hegel
Workflow: execution (current node: code)
Last Activity: 2025-11-02T16:30:15Z
Events: 1247 hooks, 34 state transitions
Files Modified: 87 unique files
```

### Discovery Configuration

User-configurable settings for discovery behavior.

**Structure:**
- Root directories (list of paths to scan, default: `["~/Code"]`)
- Max depth (how deep to recurse, default: 10 levels)
- Exclusions (directory patterns to skip, default: node_modules, target, .git)
- Cache location (where to persist scan results, default: `~/.config/hegel-pm/cache.json`)

**Example representation:**
```
Root Directories: ["/Users/emadum/Code"]
Max Depth: 10
Exclusions: ["node_modules", "target", ".git", "vendor"]
Cache Location: ~/.config/hegel-pm/cache.json
```

### Project Statistics

Aggregated metrics derived from `.hegel/` JSONL files.

**Structure:**
- Hook event count (total events in hooks.jsonl)
- State transition count (total transitions in states.jsonl)
- Unique files modified (distinct file paths touched)
- Workflow phase distribution (time spent in each phase)
- Session count (distinct session IDs)
- Total events by type (bash commands, file edits, tool uses)

**Derived from:**
- hooks.jsonl parsing (event counts, file modifications)
- states.jsonl parsing (transition history, phase timing)
- state.json parsing (current workflow state)

---

## Core Operations

### Operation: Discover Projects

**Purpose:** Scan configured root directories to find all Hegel projects.

**Behavior:**
- Start from each configured root directory
- Recursively walk directory tree up to max depth
- Skip excluded directories (node_modules, target, etc.)
- Identify directories containing `.hegel/` subdirectory
- For each discovered `.hegel/` directory:
  - Parse state.json if exists (extract workflow state)
  - Scan JSONL files to extract statistics
  - Determine last activity timestamp from file modification times
  - Record discovery timestamp
- Return list of discovered projects sorted by last activity (most recent first)

**Parameters:**
- Root directories (required, list of paths)
- Max depth (optional, default 10)
- Exclusions (optional, default standard ignores)
- Use cache (optional, default true - load from cache if exists, scan if not)

**Validation rules:**
- Root directories must exist and be readable
- Max depth must be positive integer
- Invalid `.hegel/` directories (unreadable or corrupted state.json) are logged but don't fail entire scan
- Empty root directories return empty project list without error

**Error scenarios:**
- **Permission denied on root directory**: Log error, skip that root, continue with others
- **Permission denied on subdirectory**: Log warning, skip subtree, continue scan
- **Corrupted state.json**: Log error with project path, mark project as "invalid state", include in results with error flag
- **Missing state.json**: Valid discovery (project may be newly initialized), include with "no state" indicator
- **Filesystem errors during walk**: Log error, continue scan where possible

**Performance considerations:**
- Persist scan results to cache file (avoid repeated filesystem walks)
- On dashboard launch: Load from cache if exists, otherwise perform initial scan
- Parallel scanning of multiple root directories
- Early termination of deep recursion when no `.hegel/` found in subtree
- Lazy loading of statistics (parse JSONL only when dashboard requests details)

### Operation: Parse Project Statistics

**Purpose:** Extract aggregated metrics from a discovered project's `.hegel/` directory.

**Behavior:**
- Read hooks.jsonl and parse each line as JSON event
- Count total events, group by event type
- Extract unique file paths from file modification events
- Read states.jsonl and parse each transition
- Calculate time spent in each workflow phase
- Count distinct session IDs
- Return statistics summary

**Parameters:**
- Project path (required, absolute path to project root)
- Stat types (optional, which statistics to compute - default: all)

**Validation rules:**
- Project must have `.hegel/` directory
- JSONL files must be valid (one JSON object per line)
- Malformed lines are logged and skipped, not fatal
- Missing JSONL files result in zero counts, not errors

**Error scenarios:**
- **Missing .hegel/ directory**: Return error, project may have been removed
- **Unreadable JSONL files**: Log error, return partial statistics with error flag
- **Malformed JSON lines**: Log warning with line number, skip line, continue parsing
- **Large files**: Stream parse JSONL rather than loading entire file into memory

**Performance considerations:**
- Stream-parse large JSONL files (don't load entire file)
- Cache parsed statistics until .hegel/ directory changes detected
- Parallel parsing of multiple JSONL files (hooks, states, etc.)
- Skip parsing if only basic project info needed (state.json only)

### Operation: Validate Discovery Configuration

**Purpose:** Check user-provided discovery configuration for validity.

**Behavior:**
- Verify each root directory exists and is readable
- Check max depth is positive integer
- Validate exclusion patterns are valid directory names
- Return validation result with specific errors for each invalid field

**Parameters:**
- Configuration (required, discovery configuration structure)

**Validation rules:**
- At least one root directory must be provided
- All root directories must exist
- All root directories must be readable
- Max depth must be >= 1
- Cache location directory must be writable

**Error scenarios:**
- **Non-existent root directory**: Return error, specify which path
- **Unreadable root directory**: Return error with permission details
- **Invalid max depth**: Return error, specify valid range
- **Unwritable cache location**: Return error with permission details

---

## Test Scenarios

### Simple: Single Project Discovery

**Setup:**
- Single directory with `.hegel/state.json` containing valid workflow state
- No JSONL files (newly initialized project)

**Expected behavior:**
- Project discovered with correct path and name
- Workflow state parsed correctly from state.json
- Statistics show zero events (no JSONL files)
- Last activity timestamp matches state.json modification time

### Complex: Multiple Nested Projects

**Setup:**
- Root directory contains multiple nested projects at various depths
- Some projects have active workflows, some have no state.json
- Mix of small and large JSONL files (test streaming parser)
- Excluded directories (node_modules, target) contain .hegel directories that should be skipped

**Expected behavior:**
- All non-excluded projects discovered
- Projects sorted by last activity (most recent first)
- Excluded directories skipped (no performance penalty)
- Statistics correctly aggregated for each project
- Large JSONL files parsed without excessive memory usage

### Error: Corrupted State Files

**Setup:**
- Project with malformed state.json (invalid JSON)
- Project with valid state.json but malformed hooks.jsonl (some invalid lines)
- Project with missing .hegel/ directory (removed during scan)

**Expected behavior:**
- Malformed state.json: Project included with "invalid state" flag, error logged
- Malformed hooks.jsonl: Valid lines parsed, invalid lines skipped, warning logged with line numbers
- Missing .hegel/: Project excluded from results, info logged
- Scan completes successfully despite errors, partial results returned

### Error: Permission Issues

**Setup:**
- Root directory with some subdirectories lacking read permissions
- .hegel/ directory with unreadable state.json

**Expected behavior:**
- Unreadable subdirectories skipped with warning logged
- Parent directory scan continues
- Unreadable state.json: Project included with "unreadable state" error flag
- Scan completes, accessible projects returned

### Performance: Large Workspace Scan

**Setup:**
- Root directory with 10,000+ subdirectories
- 20 actual Hegel projects scattered at various depths
- Multiple excluded directories (node_modules, target) with deep nesting

**Expected behavior:**
- Initial scan completes in under 2 seconds
- Cached results loaded in under 10ms on dashboard launch
- Memory usage remains bounded (streaming JSONL parsing)
- Exclusions prevent unnecessary deep recursion

---

## Success Criteria

### Core Functionality
- [ ] Discovers all Hegel projects in configured root directories
- [ ] Correctly parses workflow state from state.json using hegel-cli's State struct
- [ ] Skips excluded directories (node_modules, target, .git, vendor)
- [ ] Returns projects sorted by last activity timestamp
- [ ] Handles missing state.json gracefully (marks project as newly initialized)

### Statistics Extraction
- [ ] Accurately counts hook events from hooks.jsonl using hegel-cli's metrics module
- [ ] Accurately counts state transitions from states.jsonl
- [ ] Extracts unique file paths from file modification events
- [ ] Calculates time spent in each workflow phase
- [ ] Identifies distinct sessions from session IDs

### Error Handling
- [ ] Logs but continues on permission denied for subdirectories
- [ ] Includes projects with corrupted state.json, marks with error flag
- [ ] Skips malformed JSONL lines, logs warnings with line numbers
- [ ] Returns partial results when some roots are inaccessible
- [ ] Never crashes on filesystem errors during scan

### Performance
- [ ] Initial scan of workspace with 10+ projects completes in under 2 seconds
- [ ] Cached results loaded in under 10ms on dashboard launch
- [ ] Memory usage bounded for large JSONL files (streaming parser)
- [ ] Parallel scanning of multiple root directories
- [ ] Early termination of deep recursion when no projects found

### Configuration
- [ ] Validates all configuration parameters before scanning
- [ ] Provides clear error messages for invalid configuration
- [ ] Supports multiple root directories simultaneously
- [ ] Respects max depth setting (prevents infinite recursion)
- [ ] Persists scan results to cache file for subsequent loads

### Integration
- [ ] Uses hegel::storage::State for all state.json parsing
- [ ] Uses hegel::metrics for all JSONL parsing
- [ ] Returns data structures compatible with dashboard rendering layer
- [ ] Supports manual refresh via `hegel-pm scan` command
- [ ] Loads from cache on dashboard launch if cache exists

---

## Non-Functional Requirements

**Reliability:**
- Partial failures (single project errors) never crash entire discovery process
- Filesystem errors are logged with sufficient context for debugging
- State corruption in one project doesn't affect other projects

**Maintainability:**
- All state parsing delegates to hegel-cli modules (no duplication)
- Clear separation between discovery logic and statistics extraction
- Well-structured error types with context information

**Testability:**
- Mock filesystem structure for deterministic test scenarios
- Fixture .hegel/ directories for state parsing tests
- Performance benchmarks for large workspace scenarios

**Security:**
- Read-only filesystem access (never writes to discovered projects)
- No code execution from parsed state (pure data parsing)
- Bounds checking on recursion depth (prevent stack overflow)
