# Project Discovery Implementation Plan

Step-by-step development plan using TDD discipline to build filesystem-based Hegel project discovery.

---

## Overview

**Goal:** Implement automatic discovery of Hegel projects by scanning configured directories for `.hegel/` state folders, leveraging hegel-cli's existing storage and metrics parsing modules.

**Scope:** Core discovery engine only - no web UI, no CLI commands, no live updates. This plan covers the library code that finds projects and extracts their statistics.

**Priorities:**
1. Correctness: Accurately parse all Hegel state formats using hegel-cli modules
2. Performance: As fast as theoretically possible while keeping code simple and elegant
3. Resilience: Graceful handling of corrupted/missing state files
4. Maintainability: Zero duplication with hegel-cli, refactor CLI when needed

---

## Methodology

**TDD Discipline:**
- Write failing tests first for every function
- Test interfaces and contracts, not internal implementation details
- Cover happy path, error cases, and edge conditions
- Commit immediately after each numbered step completes

**What to test:**
- Project discovery logic (finds correct .hegel directories)
- State parsing delegation to hegel-cli modules
- Statistics aggregation from JSONL files
- Error handling for corrupted/missing files
- Cache persistence and loading
- Configuration validation

**What NOT to test:**
- hegel-cli's internal parsing logic (trust the library)
- Filesystem primitives (trust std::fs)
- Third-party crate behavior (walkdir, serde)

**Refactoring hegel-cli:**
When hegel-cli code is coupled to CLI concerns (terminal output, clap args), we will refactor it to expose library-friendly functions before using from hegel-pm. This keeps both codebases clean.

---

## Step 1: Project Structure and Dependencies

### Goal
Set up hegel-pm as a Rust project with proper dependency on hegel-cli and establish basic module structure.

### Step 1.a: Write Tests
Create initial test module structure that will validate project setup. Write tests that ensure hegel-cli modules are accessible and can be imported. Test that basic types from hegel-cli compile and are usable.

### Step 1.b: Implement
Create Cargo.toml with hegel-cli as path dependency. Set up src directory with main.rs and lib.rs. Create discovery module stub. Import key types from hegel-cli storage and metrics modules to verify dependency works. Ensure project compiles.

### Success Criteria
- [ ] Cargo.toml correctly depends on hegel-cli
- [ ] Can import hegel::storage::State and hegel::metrics types
- [ ] Project compiles without errors
- [ ] Basic test module structure exists
- [ ] Committed with message: feat(discovery): initialize project structure

---

## Step 2: Configuration Model

### Goal
Define and validate discovery configuration with safe defaults and comprehensive validation.

### Step 2.a: Write Tests
Test configuration creation with defaults. Test validation of root directories that exist versus don't exist. Test max depth validation for positive integers only. Test exclusion pattern validation. Test cache location directory writability checking. Test serialization and deserialization of config to JSON.

### Step 2.b: Implement
Create DiscoveryConfig struct with root directories list, max depth, exclusions list, and cache location path. Implement validation function that checks all directories exist and are readable, max depth is positive, cache location parent is writable. Implement Default trait with sensible values: root is home Code directory, max depth ten, standard exclusions for node_modules target git vendor. Add serde derives for cache persistence.

### Success Criteria
- [ ] DiscoveryConfig struct defined with all required fields
- [ ] Validation catches all invalid configurations
- [ ] Default configuration provides sensible starting values
- [ ] Config serializes to and from JSON correctly
- [ ] All tests pass
- [ ] Committed with message: feat(discovery): add configuration model with validation

---

## Step 3: Discovered Project Model

### Goal
Define the data structure representing a discovered Hegel project with all metadata needed for dashboard display.

### Step 3.a: Write Tests
Test project model creation with required fields. Test last activity timestamp calculation from file modification times. Test serialization for cache persistence. Test comparison and sorting by last activity. Test handling of projects with missing state versus valid state versus corrupted state.

### Step 3.b: Implement
Create DiscoveredProject struct with project name, project path, hegel directory path, optional workflow state from hegel-cli State type, last activity timestamp, discovery timestamp, and optional error field for corrupted projects. Implement methods for calculating last activity from filesystem metadata. Add serde derives for caching. Implement Ord trait to sort by last activity descending.

### Success Criteria
- [ ] DiscoveredProject struct defined with all metadata fields
- [ ] Last activity calculation works from file timestamps
- [ ] Projects can be sorted by recency
- [ ] Serialization works for caching
- [ ] Error state properly represented for corrupted projects
- [ ] All tests pass
- [ ] Committed with message: feat(discovery): add discovered project model

---

## Step 4: Basic Filesystem Walking

### Goal
Implement recursive directory traversal that finds all .hegel directories while respecting exclusions and max depth.

### Step 4.a: Write Tests
Test walking single directory with one .hegel folder found. Test walking nested directories up to max depth. Test that directories beyond max depth are not searched. Test that excluded directories are skipped entirely. Test handling of permission denied on subdirectories continues scan. Test empty directories return empty results. Test symlinks are not followed to prevent loops.

### Step 4.b: Implement
Create walker function that takes root directory, max depth, and exclusion list. Use walkdir crate to recursively traverse filesystem. Check depth against max depth and stop when exceeded. Filter out excluded directory names before descending. Detect .hegel directories by checking for subdirectory with that name. Collect all found .hegel parent directories. Handle errors by logging and continuing rather than failing entire scan. Return list of paths containing .hegel directories.

### Success Criteria
- [ ] Finds all .hegel directories in test fixtures
- [ ] Respects max depth limit
- [ ] Skips excluded directories
- [ ] Handles permission errors gracefully
- [ ] Does not follow symlinks
- [ ] All tests pass
- [ ] Committed with message: feat(discovery): implement filesystem walking

---

## Step 5: State Parsing Using hegel-cli

### Goal
Load and parse .hegel state.json files using hegel-cli's storage module, with proper error handling for missing or corrupted files.

### Step 5.a: Write Tests
Test parsing valid state.json file returns populated State struct. Test missing state.json returns None without error. Test corrupted JSON in state.json returns error with context. Test empty state.json file. Test state.json with unexpected schema still deserializes safely. Create fixture .hegel directories with various state scenarios for testing.

### Step 5.b: Implement
**Note:** If hegel-cli's state loading is tightly coupled to CLI directory detection or command-line args, first refactor hegel-cli to expose a simple function that takes a path to .hegel directory and returns Result with State. This keeps both codebases clean.

Create function that takes path to .hegel directory and attempts to load state.json. Use hegel-cli's storage module functions or State deserialization. Handle file not found as valid case returning None. Handle JSON parsing errors as error result with file path context. Return Result containing optional State.

### Success Criteria
- [ ] Successfully parses valid hegel-cli state files
- [ ] Returns None for missing state.json without error
- [ ] Returns descriptive error for corrupted state files
- [ ] Reuses hegel-cli State type directly
- [ ] hegel-cli refactored if needed for library usage
- [ ] All tests pass
- [ ] Committed with message: feat(discovery): integrate hegel-cli state parsing

---

## Step 6: Project Discovery Integration

### Goal
Combine filesystem walking with state parsing to produce complete list of discovered projects.

### Step 6.a: Write Tests
Test discovery finds multiple projects and populates their state. Test discovery handles mix of projects with and without state.json. Test discovery calculates last activity timestamp correctly. Test discovery marks corrupted projects with error flag. Test discovery returns projects sorted by last activity. Test discovery with empty workspace returns empty list. Test parallel processing of multiple root directories.

### Step 6.b: Implement
Create discover function that takes DiscoveryConfig. For each root directory, invoke filesystem walker. For each found .hegel directory, parse state using function from previous step. Calculate last activity from .hegel directory file modification times. Create DiscoveredProject struct with all metadata. Handle state parsing errors by marking project with error but including in results. Collect all discovered projects. Sort by last activity descending. Return sorted list.

### Success Criteria
- [ ] Discovers all projects in test workspace fixtures
- [ ] Correctly populates state for each project
- [ ] Last activity timestamps accurate
- [ ] Corrupted projects included with error markers
- [ ] Results sorted by recency
- [ ] Multiple root directories processed
- [ ] All tests pass
- [ ] Committed with message: feat(discovery): implement project discovery integration

---

## Step 7: Statistics Extraction Using hegel-cli Metrics

### Goal
Extract aggregated statistics from .hegel JSONL files using hegel-cli's metrics parsing modules.

### Step 7.a: Write Tests
Test extracting hook event counts from hooks.jsonl. Test extracting state transitions from states.jsonl. Test extracting unique file paths from file modification events. Test handling missing JSONL files returns zero counts. Test handling corrupted JSONL lines skips bad lines and continues. Test statistics aggregation matches hegel analyze output format. Create fixture .hegel directories with sample JSONL data.

### Step 7.b: Implement
**Note:** If hegel-cli's metrics extraction is coupled to CLI output formatting or command args, first refactor hegel-cli to expose library functions that parse JSONL and return structured data. The hegel analyze command should then use these same library functions.

Create ProjectStatistics struct matching data from hegel analyze output. Create function that takes path to .hegel directory and returns statistics. Use hegel-cli metrics module to parse hooks.jsonl for event counts. Use hegel-cli metrics module to parse states.jsonl for transitions. Aggregate data into ProjectStatistics. Handle missing files as zero counts. Handle malformed JSONL lines by logging warning and skipping. Return Result with statistics.

### Success Criteria
- [ ] Statistics match hegel analyze output for same project
- [ ] Reuses hegel-cli metrics parsing completely
- [ ] Handles missing JSONL files gracefully
- [ ] Skips corrupted JSONL lines with warnings
- [ ] Returns structured data matching dashboard needs
- [ ] hegel-cli refactored if needed for library usage
- [ ] All tests pass
- [ ] Committed with message: feat(discovery): add statistics extraction

---

## Step 8: Lazy Statistics Loading

### Goal
Optimize discovery by only parsing JSONL when statistics are actually requested, not during initial scan.

### Step 8.a: Write Tests
Test initial discovery completes quickly without loading statistics. Test statistics can be loaded on demand for specific project. Test statistics loading caches results to avoid re-parsing. Test loading statistics for multiple projects in parallel. Test statistics loading handles errors independently per project.

### Step 8.b: Implement
Modify DiscoveredProject to have optional statistics field initially None. Create separate function to load statistics for a project on demand. Cache loaded statistics in the project struct. Modify discovery process to skip statistics extraction during scan. Add method to batch load statistics for multiple projects efficiently. Handle statistics loading errors without affecting other projects.

### Success Criteria
- [ ] Initial discovery much faster without statistics parsing
- [ ] Statistics loaded correctly on demand
- [ ] Statistics cached after first load
- [ ] Batch loading supported for efficiency
- [ ] Errors isolated per project
- [ ] All tests pass
- [ ] Committed with message: feat(discovery): implement lazy statistics loading

---

## Step 9: Cache Persistence

### Goal
Persist discovered projects to cache file and load from cache on subsequent runs.

### Step 9.a: Write Tests
Test saving discovered projects to cache file creates valid JSON. Test loading from cache returns same projects as original scan. Test cache file missing triggers fresh scan. Test corrupted cache file triggers fresh scan with warning. Test cache respects configured cache location. Test cache handles concurrent access safely. Test cache invalidation when configuration changes.

### Step 9.b: Implement
Create cache module with save and load functions. Save discovered projects list as JSON to configured cache location using atomic write pattern. Load cache file and deserialize to project list. Handle missing cache by returning None to trigger scan. Handle corrupted cache by logging warning and returning None. Include configuration hash in cache to detect config changes. Use file locking if needed for concurrent safety.

### Success Criteria
- [ ] Cache saves successfully to configured location
- [ ] Cache loads correctly on subsequent runs
- [ ] Missing cache triggers fresh scan
- [ ] Corrupted cache handled gracefully
- [ ] Cache invalidates when config changes
- [ ] Concurrent access safe
- [ ] All tests pass
- [ ] Committed with message: feat(discovery): add cache persistence

---

## Step 10: Discovery Orchestration

### Goal
Tie together discovery, caching, and statistics into high-level API that dashboard will use.

### Step 10.a: Write Tests
Test API with no cache performs full scan and saves cache. Test API with valid cache loads from cache. Test API with force refresh bypasses cache and rescans. Test API handles configuration changes by invalidating cache. Test API returns results with statistics when requested. Test API handles all error scenarios gracefully.

### Step 10.b: Implement
Create DiscoveryEngine struct that owns configuration and cache. Implement method to get projects that checks cache first, loads if valid, scans if not. Add force refresh parameter to bypass cache. Integrate statistics loading based on caller request. Handle all error paths with clear context. Provide clean API for dashboard to consume.

### Success Criteria
- [ ] Single clean API for dashboard usage
- [ ] Cache-or-scan logic works correctly
- [ ] Force refresh option available
- [ ] Statistics loading integrated
- [ ] All error scenarios handled
- [ ] API documented for consumers
- [ ] All tests pass
- [ ] Committed with message: feat(discovery): implement discovery orchestration

---

## Step 11: Performance Validation

### Goal
Verify discovery performance is optimal while maintaining code simplicity and elegance.

### Step 11.a: Write Tests
Create performance benchmark fixture with realistic workspace structure. Measure initial scan time. Measure cache load time. Measure memory usage stays bounded during scan. Test parallel directory scanning improves performance. Test exclusions prevent unnecessary traversal overhead.

### Step 11.b: Implement
Add benchmark harness using Criterion or manual timing. Create realistic test workspace structure. Run benchmarks to establish baseline. Profile and optimize hot paths if optimizations maintain code elegance. Ensure parallel scanning configured correctly. Verify exclusions short-circuit traversal.

### Success Criteria
- [ ] Initial scan time measured and reasonable
- [ ] Cache load time measured and fast
- [ ] Memory usage bounded and reasonable
- [ ] Parallel scanning functional
- [ ] Exclusions provide measurable speedup
- [ ] Performance benchmarks documented
- [ ] Committed with message: test(discovery): add performance validation

---

## Step 12: Error Handling Review

### Goal
Comprehensive review of all error paths with specific focus on partial failures and error context.

### Step 12.a: Write Tests
Test every error scenario produces clear error message with context. Test partial failures in multi-project scan continue and report results. Test filesystem permission errors logged with specific paths. Test corrupted file errors include file path and line number where applicable. Test error types are specific not generic. Test errors serialize for potential remote display.

### Step 12.b: Implement
Review all error handling code. Ensure errors include file paths and line numbers where relevant. Use anyhow context to add information at each layer. Ensure partial failures collect errors but continue processing. Create specific error types for different failure modes. Add error serialization if needed for API transport. Document error handling strategy.

### Success Criteria
- [ ] All errors include helpful context
- [ ] Partial failures handled correctly
- [ ] Error types are specific and actionable
- [ ] Errors ready for dashboard display
- [ ] Error handling documented
- [ ] All tests pass
- [ ] Committed with message: refactor(discovery): improve error handling and context

---

## Step 13: Integration Testing

### Goal
End-to-end testing of complete discovery workflow with realistic workspace fixtures.

### Step 13.a: Write Tests
Create integration test workspace with multiple nested projects. Include projects with various states: active workflows, completed workflows, no state, corrupted state. Test full discovery cycle including cache save and load. Test configuration changes invalidate cache. Test statistics extraction for all projects. Test error scenarios with realistic corruption. Verify output matches expectations for sample workspace.

### Step 13.b: Implement
Build comprehensive test fixtures in tests directory. Create workspace structure matching real hegel project layouts. Include real hegel-cli state files and JSONL data. Run full discovery workflow as integration test. Verify all data fields populated correctly. Test cache round-trip. Clean up test artifacts properly.

### Success Criteria
- [ ] Integration tests cover full workflow
- [ ] Test fixtures realistic and comprehensive
- [ ] All discovery features tested end-to-end
- [ ] Cache persistence tested in integration
- [ ] Error scenarios validated with real data
- [ ] All tests pass
- [ ] Committed with message: test(discovery): add integration tests

---

## Step 14: Documentation

### Goal
Document the discovery API and internal architecture for future maintainers and dashboard integration.

### Step 14.a: Write Tests
Not applicable for documentation step.

### Step 14.b: Implement
Write module-level documentation for discovery module. Document DiscoveryEngine API with usage examples. Document configuration options and defaults. Document error types and when they occur. Document cache format and location. Document performance characteristics. Document integration points with hegel-cli. Add inline documentation for all public functions and types.

### Success Criteria
- [ ] All public APIs documented
- [ ] Usage examples included
- [ ] Integration points with hegel-cli explained
- [ ] Performance characteristics documented
- [ ] Cache behavior documented
- [ ] Error handling strategy documented
- [ ] Committed with message: docs(discovery): document discovery API and architecture

---

## Summary

This plan builds project discovery incrementally with TDD discipline, leveraging hegel-cli's proven state and metrics parsing while keeping codebases clean through refactoring when needed. Each step is independently testable and committable, enabling clear progress tracking and easy rollback if needed.

Key architectural decisions:
- Direct dependency on hegel-cli as library
- Refactor hegel-cli when needed to expose clean library functions
- Lazy loading of statistics for performance
- Persistent cache with automatic invalidation
- Graceful degradation for corrupted or missing data
- Clear error context throughout

The result will be a robust discovery engine ready for dashboard integration.
