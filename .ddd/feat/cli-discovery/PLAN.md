# CLI Discovery Implementation Plan

Production implementation of command-line interface for Hegel project discovery.

---

## Overview

**Goal**: Expose existing discovery module functionality through three CLI subcommands: lightweight project listing, detailed single-project view, and aggregate cross-project metrics table.

**Scope**: CLI layer only — reuses all existing discovery logic. No new discovery features, purely interface work.

**Priorities**:
1. Reuse existing `src/discovery` module APIs without duplication
2. Cache-first behavior (fast by default)
3. Clean separation: CLI parsing → discovery API → output formatting
4. Human-friendly defaults, machine-readable JSON optional

**Methodology**: Test-Driven Development with focus on CLI integration, output formatting, and error handling. Skip testing third-party CLI libraries. Test discovery integration, output correctness, and edge cases.

---

## Step 1: CLI Structure and Argument Parsing

### Goal
Refactor existing CLI to use subcommands instead of flags, preserving backward compatibility where sensible.

### Step 1.a: Write Tests
- Test that CLI accepts three subcommands: `list`, `show`, `all`
- Test that global flags are recognized: `--json`, `--no-cache`
- Test that `show` requires project name argument
- Test that `all` accepts optional `--sort-by` and `--benchmark` flags
- Test invalid subcommand returns error with usage help
- Test invalid flags return error with suggestions
- Test `--sort-by` validates column names
- Test backward compatibility: `--discover` still works (deprecated warning)

### Step 1.b: Implement
- Create `src/cli/discover/` directory structure for modular CLI commands
- Refactor `src/cli.rs` to use clap subcommands instead of flat flags
- Define `DiscoverCommand` enum with three variants: List, Show, All in `cli/discover/mod.rs`
- Keep existing clap setup but restructure to support nested commands
- Define global flags for `--json` and rename `--refresh` to `--no-cache` (keep `--refresh` as alias)
- Define subcommand-specific options (project name for show, sort column and benchmark flag for all)
- Create stub modules: `list.rs`, `show.rs`, `all.rs`, `format.rs`
- Update `main.rs` to route to new subcommand structure via `cli::discover`
- Implement validation for sort column names
- Add deprecation warning for old `--discover` flag usage

### Success Criteria
- [ ] All three subcommands parse correctly
- [ ] Global flags recognized across all subcommands
- [ ] Show command requires and parses project name
- [ ] All command parses optional sort-by and benchmark flags
- [ ] Invalid input produces clear error messages
- [ ] Tests validate all parsing scenarios
- [ ] Old `--discover` flag still works with deprecation warning

---

## Step 2: List Command - Lightweight View

### Goal
Implement `discover list` command with cache-aware project listing and folder size calculation.

### Step 2.a: Write Tests
- Test list command loads projects from cache by default
- Test list command with `--no-cache` forces fresh scan
- Test output includes name, path, size, last activity for each project
- Test projects sorted by last activity (most recent first)
- Test folder size calculation is accurate
- Test empty result displays "No Hegel projects found"
- Test JSON output matches schema from spec
- Test cache hit displays confirmation message
- Test discovery config validation failure returns appropriate error

### Step 2.b: Implement
- Enhance existing discovery_mode simple listing into full list command handler
- DiscoveryEngine already initialized in main.rs, reuse that pattern
- Determine cache behavior based on `--no-cache` flag (reuse existing `--refresh` logic)
- Call DiscoveryEngine get_projects method with appropriate refresh flag
- For each project, calculate .hegel folder size using disk usage
- Format human-readable output as aligned table
- Format JSON output matching spec schema
- Add helper for path abbreviation (home directory as tilde)
- Add helper for human-readable size formatting (KB, MB, GB)
- Handle empty results gracefully
- Handle discovery errors with clear messages

### Success Criteria
- [ ] List command loads from cache by default
- [ ] Force refresh bypasses cache when requested
- [ ] Output displays all required fields in clean table format
- [ ] Projects sorted by most recent activity first
- [ ] Folder sizes calculated correctly
- [ ] JSON output valid and matches schema
- [ ] Empty results handled gracefully
- [ ] Errors produce clear, actionable messages

---

## Step 3: Show Command - Single Project Detail

### Goal
Implement `discover show` command for detailed single-project inspection with full metrics.

### Step 3.a: Write Tests
- Test show command finds project by name
- Test show loads full UnifiedMetrics for target project
- Test output displays project metadata and workflow state
- Test output displays all available metrics
- Test project-not-found returns error with suggestions
- Test corrupted state files display error but continue with partial data
- Test metrics loading failure displays warning but shows other data
- Test JSON output matches schema
- Test cache behavior respects `--no-cache` flag

### Step 3.b: Implement
- Create show command handler that loads projects via DiscoveryEngine
- Find project by exact name match (case-sensitive)
- Return error with available project names if not found
- Load full metrics by calling load_statistics on project
- Format human-readable output with sections for metadata, state, metrics
- Display workflow state fields if available, "None" otherwise
- Display error field prominently if state corrupted
- Format metrics in readable key-value layout
- Handle metrics loading failures gracefully with warnings
- Format JSON output matching spec schema

### Success Criteria
- [ ] Show command locates project by name correctly
- [ ] Full metrics loaded for target project
- [ ] Output organized into clear sections
- [ ] Workflow state displayed when available
- [ ] Not-found error suggests available projects
- [ ] Corrupted state shows error but displays other data
- [ ] Metrics failures warned but don't abort command
- [ ] JSON output valid and complete

---

## Step 4: All Command - Aggregate Table with Sorting

### Goal
Implement `discover all` command with metrics for all projects and column-based sorting, including optional benchmarking.

### Step 4.a: Write Tests
- Test all command loads metrics for every discovered project
- Test default sort by last-activity (descending)
- Test custom sort by each valid column (name, size, tokens, events, phases)
- Test numeric columns sort descending, text columns ascending
- Test output displays all projects in aligned table
- Test metrics loading failure for one project shows N/A but continues
- Test invalid sort column returns error with valid options
- Test JSON output includes sorted_by field
- Test empty results handled gracefully
- Test `--benchmark` flag adds load time column
- Test benchmark totals displayed at bottom of table
- Test benchmark times accurate (within reasonable margin)
- Test sorting by load-time column works correctly

### Step 4.b: Implement
- Create all command handler that loads projects via DiscoveryEngine
- Load full metrics for all projects (call load_statistics on each)
- When `--benchmark` flag present, measure time for each project's metrics loading
- Collect metrics loading failures but continue processing
- Parse and validate sort-by column name (including "load-time" when benchmarking)
- Implement sorting logic for each column type (numeric descending, text ascending)
- Format table with aligned columns showing all metrics
- When benchmarking, add load-time column with millisecond precision
- Display N/A for projects with failed metrics loads
- Show warnings for any failures
- When benchmarking, display total time at bottom of table
- Format JSON output with sorted_by metadata and timing data when benchmarking
- Handle empty project list gracefully

### Success Criteria
- [ ] All command loads metrics for all projects
- [ ] Default sort by last-activity works correctly
- [ ] Custom sorting works for all valid columns
- [ ] Table output aligned and readable
- [ ] Partial failures show N/A and warn but complete successfully
- [ ] Invalid sort column produces clear error with options
- [ ] JSON output includes sort metadata
- [ ] Zero projects displays appropriate message
- [ ] Benchmark flag adds load-time column correctly
- [ ] Benchmark times measured accurately
- [ ] Total time displayed at bottom when benchmarking
- [ ] Can sort by load-time column when benchmarking

---

## Step 5: Output Formatting Utilities

### Goal
Create reusable formatting helpers for consistent, high-quality output across all commands.

### Step 5.a: Write Tests
- Test human-readable size formatter (bytes to KB/MB/GB)
- Test size formatter handles very large values (TB, PB)
- Test path abbreviation replaces home directory with tilde
- Test timestamp formatting in human-readable mode
- Test ISO 8601 timestamps in JSON mode
- Test table alignment handles varying content lengths
- Test special characters in project names don't break formatting
- Test very long paths truncated appropriately in table view

### Step 5.b: Implement
- Create formatting utilities module
- Implement human-readable size formatter with appropriate unit selection
- Implement path abbreviator using home directory detection
- Implement timestamp formatters (human-readable and ISO 8601)
- Create table formatter that calculates column widths and aligns content
- Add truncation logic for overly long paths in table view
- Add escape handling for special characters if needed
- Create helper for N/A placeholder display

### Success Criteria
- [ ] Size formatter produces readable output for all scales
- [ ] Path abbreviation works cross-platform
- [ ] Timestamp formats correct in both modes
- [ ] Table alignment handles content of varying lengths
- [ ] Long paths truncated cleanly
- [ ] Special characters handled safely
- [ ] Utilities reused across all three commands

---

## Step 6: Error Handling and Edge Cases

### Goal
Comprehensive error handling for all failure modes with clear, actionable error messages.

### Step 6.a: Write Tests
- Test discovery config validation errors display clearly
- Test cache corruption falls back to scan with warning
- Test filesystem permission errors during scan
- Test projects with missing .hegel directories
- Test very large project counts (performance baseline)
- Test metrics parsing errors for individual projects
- Test concurrent access to cache file
- Test network filesystem timeouts (if applicable)

### Step 6.b: Implement
- Add discovery config validation with detailed error messages
- Implement cache corruption detection and fallback logic
- Add filesystem error handling with user-friendly messages
- Handle missing or incomplete .hegel directories gracefully
- Add performance monitoring for large project counts
- Implement per-project error isolation (one failure doesn't abort all)
- Add timeout handling for slow filesystem operations
- Create comprehensive error type enum for CLI errors

### Success Criteria
- [ ] Config validation errors clear and actionable
- [ ] Cache corruption detected and handled gracefully
- [ ] Filesystem errors produce helpful messages
- [ ] Missing directories handled without crashes
- [ ] Large project counts perform acceptably
- [ ] Individual project failures isolated properly
- [ ] All error messages guide user to resolution
- [ ] No panics or unhandled errors in any scenario

---

## Step 7: Integration and Documentation

### Goal
Wire up CLI to main binary, add shell completions, update documentation.

### Step 7.a: Write Tests
- Test CLI integrated into main binary correctly
- Test help text accurate for all commands
- Test examples in help text actually work
- Test shell completions generated correctly (if applicable)
- Test discovery config uses same defaults as server mode
- Test no duplication of discovery logic
- Test CLI and API share same data types where applicable

### Step 7.b: Implement
- Wire discover subcommand into main CLI router
- Generate comprehensive help text with examples
- Add shell completion generation (bash, zsh, fish)
- Update README with CLI discovery examples
- Add section to CLAUDE.md documenting CLI usage
- Verify DiscoveryConfig defaults match server mode
- Ensure API types reused where applicable (no duplication)
- Add usage examples to help output

### Success Criteria
- [ ] Discover subcommand fully integrated
- [ ] Help text comprehensive and accurate
- [ ] Shell completions work for all commands
- [ ] Documentation updated with clear examples
- [ ] Config defaults consistent across CLI and server
- [ ] No duplicated discovery logic exists
- [ ] All examples in docs verified working

---

## Commit Strategy

After each numbered step completes successfully:
- Run full test suite to verify green state
- Commit with conventional format: `feat(cli): complete Step N - <description>`
- Include step number for clear history
- Push after each commit to enable incremental review

Example commits:
- `feat(cli): complete Step 1 - CLI structure and argument parsing`
- `feat(cli): complete Step 2 - list command implementation`
- `feat(cli): complete Step 3 - show command for single project`
- `feat(cli): complete Step 4 - all command with sorting`
- `feat(cli): complete Step 5 - output formatting utilities`
- `feat(cli): complete Step 6 - error handling and edge cases`
- `feat(cli): complete Step 7 - integration and documentation`

---

## Testing Philosophy

**Test coverage target**: ≥80% lines (enforced by pre-commit hook)

**What to test**:
- ✅ CLI argument parsing and validation
- ✅ Discovery integration (correct API usage)
- ✅ Output formatting correctness
- ✅ JSON schema compliance
- ✅ Error handling and edge cases
- ✅ Sorting logic for all columns
- ✅ Cache behavior (hit/miss/fallback)

**What NOT to test**:
- ❌ Discovery module internals (already tested)
- ❌ CLI library behavior (clap/similar)
- ❌ Filesystem primitives (trust std::fs)
- ❌ JSON serialization library (trust serde)

**Test organization**: Co-located `#[cfg(test)]` modules in CLI implementation files

---

## Performance Expectations

Based on spec requirements:
- List command: <500ms with cache hit (100 projects)
- Show command: <200ms with cache hit
- All command: <2s for 50 projects with metrics loading
- Filesystem scan: <10s for typical workspace with exclusions

Performance validation during Step 6 testing.

---

## Integration Points

**Reuses from existing codebase**:
- `src/discovery::DiscoveryEngine` - main discovery orchestration
- `src/discovery::DiscoveryConfig` - configuration with validation
- `src/discovery::DiscoveredProject` - project data model
- `hegel::metrics::UnifiedMetrics` - metrics loading

**New modules created**:
- `src/cli/discover/` - Discovery CLI subcommand module (directory)
  - `mod.rs` - Module root, command dispatch, shared types
  - `list.rs` - List command handler
  - `show.rs` - Show command handler
  - `all.rs` - All command handler with sorting and benchmarking
  - `format.rs` - Output formatting utilities (human-readable and JSON)

**Modified files**:
- `src/cli.rs` - refactor from flags to subcommands
- `src/main.rs` - update dispatch logic for new subcommands
- `src/discovery_mode.rs` - migrate logic into `src/cli/discover/` modules, then delete
- `Cargo.toml` - clap already present, may need feature flags for derive macro

---

## Success Validation

After completing all steps:
- [ ] All three commands work end-to-end
- [ ] Test suite ≥80% coverage
- [ ] No clippy warnings
- [ ] All examples in help text verified
- [ ] Documentation complete and accurate
- [ ] Performance benchmarks within spec requirements
- [ ] Zero panics or unhandled errors in any tested scenario
- [ ] Cache behavior correct in all modes
- [ ] JSON output valid and schema-compliant
- [ ] Error messages clear and actionable
