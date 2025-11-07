# Binary Cache Implementation Plan

Implementation plan for replacing JSON cache with memory-mapped binary format.

## Overview

**Goal**: Replace `src/discovery/cache.rs` to use bincode + memmap2 for faster CLI operations.

**Scope**:
- Add new binary cache functions alongside existing JSON cache
- Existing JSON cache functions remain for data_layer use
- Update DiscoveryEngine to use binary cache for CLI operations
- CLI commands work unchanged after migration

**Priorities**:
1. Correctness: Atomic writes, proper error handling
2. Performance: Faster than JSON (measured with benchmarks)
3. Simplicity: Minimal changes to existing code

## Methodology

**TDD Approach**:
- Write tests first for each cache operation
- Verify file format correctness
- Test error recovery paths
- Measure performance improvement

**What to Test**:
- Cache save/load round-trips correctly
- Atomic writes (no partial files)
- Corrupted file handling (skip and continue)
- Missing file handling (cache miss behavior)

**What NOT to Test**:
- bincode/memmap2 library internals
- Filesystem primitives
- DiscoveredProject serialization (already tested)

---

## Step 1: Add Dependencies and Directory Structure

### Goal
Set up build dependencies and cache directory layout without breaking existing functionality.

### Step 1.a: Write Tests
- Test that cache directory is created when it doesn't exist
- Test that config returns correct cache directory path
- Verify directory permissions are correct

### Step 1.b: Implement
- Add bincode and memmap2 to Cargo.toml dependencies section
- Modify DiscoveryConfig to provide cache_dir method that returns PathBuf to cache directory
- Update config validation to ensure cache directory parent is writable

### Success Criteria
- cargo build succeeds with new dependencies
- All existing tests still pass
- Config can return cache directory path
- Cache directory created automatically when needed

---

## Step 2: Implement ProjectIndexEntry and Index Serialization

### Goal
Create lightweight index structure for fast project listing.

### Step 2.a: Write Tests
- Test ProjectIndexEntry serialization round-trip with bincode
- Test building index from Vec of DiscoveredProject
- Test writing and reading index.bin file
- Test atomic write of index file (temp + rename pattern)

### Step 2.b: Implement
- Define ProjectIndexEntry struct in cache.rs with name, project_path, hegel_dir, last_activity
- Implement helper to build index from projects
- Implement write_index function that serializes to bincode and writes atomically
- Implement read_index function that memory-maps and deserializes

### Success Criteria
- ProjectIndexEntry serializes/deserializes correctly
- Index built correctly from project list
- Index file written atomically (verified with crash simulation)
- Index file read successfully with mmap

---

## Step 3: Implement Per-Project File Serialization

### Goal
Write and read individual project binary files.

### Step 3.a: Write Tests
- Test writing single DiscoveredProject to binary file
- Test reading single project from binary file
- Test atomic write for project files
- Test that statistics field is None in cached files
- Test handling of invalid filenames (sanitization)

### Step 3.b: Implement
- Implement write_project function that writes project to name.bin atomically
- Implement read_project function that memory-maps and deserializes
- Add filename sanitization to handle special characters in project names
- Ensure statistics field is cleared before writing to cache

### Success Criteria
- Individual project files written and read correctly
- Atomic writes verified
- Statistics not included in cache files
- Special characters in names handled safely

---

## Step 4: Add save_binary_cache Function

### Goal
Implement new save_binary_cache function alongside existing save_cache for JSON.

### Step 4.a: Write Tests
- Test save_binary_cache writes all project files plus index
- Test partial failure handling (skip failed project, continue)
- Test empty project list handling
- Verify existing save_cache still works unchanged

### Step 4.b: Implement
- Add new save_binary_cache function that takes projects and config
- Iterate projects and write individual bin files
- Build and write index file after all project files written
- Handle serialization errors by logging warning and continuing
- Keep existing save_cache function for JSON (used by data_layer)

### Success Criteria
- save_binary_cache writes correct multi-file structure
- Index written last (after all projects)
- Partial failures don't abort entire operation
- Existing save_cache function unchanged and working

---

## Step 5: Add load_binary_cache Function

### Goal
Implement new load_binary_cache function alongside existing load_cache for JSON.

### Step 5.a: Write Tests
- Test load_binary_cache returns None when index missing
- Test load_binary_cache returns all projects when cache valid
- Test handling of missing project files (skip and log)
- Test handling of corrupted project files (skip and log)
- Test handling of corrupted index file (return error)
- Verify existing load_cache still works unchanged

### Step 5.b: Implement
- Add new load_binary_cache function that takes config
- Check for index.bin, return None if missing
- Read index using mmap and deserialize
- For each index entry, read corresponding project file
- Skip missing or corrupted project files with warning log
- Return error if index itself is corrupted
- Keep existing load_cache function for JSON (used by data_layer)

### Success Criteria
- load_binary_cache returns correct data from binary cache
- Missing project files logged but don't fail operation
- Corrupted project files skipped gracefully
- Corrupted index returns clear error
- Cache miss returns None correctly
- Existing load_cache function unchanged and working

---

## Step 6: Update DiscoveryEngine to Use Binary Cache

### Goal
Switch DiscoveryEngine to use binary cache functions for CLI operations.

### Step 6.a: Write Tests
- Test DiscoveryEngine.get_projects uses binary cache when available
- Test fallback to scan when binary cache missing
- Test --no-cache flag bypasses binary cache
- Verify data_layer can still use JSON cache independently

### Step 6.b: Implement
- Update DiscoveryEngine.get_projects to call load_binary_cache
- Update DiscoveryEngine.scan_and_cache to call save_binary_cache
- Ensure force_refresh bypasses binary cache correctly
- Keep JSON cache functions available for data_layer worker pool

### Success Criteria
- CLI operations use binary cache by default
- Force refresh bypasses binary cache
- Data_layer can still access JSON cache
- No breaking changes to public API

---

## Step 7: Verify CLI Commands Work Unchanged

### Goal
Ensure all existing CLI commands function identically with new cache format.

### Step 6.a: Write Tests
- Integration test: discover list with binary cache
- Integration test: discover show with binary cache
- Integration test: discover all with binary cache
- Test cache refresh (--no-cache flag still works)

### Step 6.b: Implement
- No implementation needed if previous steps correct
- Fix any integration issues discovered
- Ensure logging shows cache hits/misses clearly

### Success Criteria
- discover list produces identical output
- discover show produces identical output
- discover all produces identical output
- --no-cache flag bypasses binary cache correctly
- Cache status visible in logs

---

## Step 8: Performance Benchmarking

### Goal
Verify binary cache is faster than JSON cache was.

### Step 7.a: Write Tests
- Benchmark test: time to load index only (list operation)
- Benchmark test: time to load all projects (show/all operation)
- Comparison with previous JSON performance if data available

### Step 7.b: Implement
- Add benchmark tests using cargo bench or manual timing
- Measure cache read times for various project counts
- Document performance improvements in commit message

### Success Criteria
- Binary cache faster than JSON for listing
- Binary cache faster than JSON for full load
- Performance gains documented
- No performance regressions

---

## Step 9: Documentation and Cleanup

### Goal
Update documentation and remove obsolete code.

### Step 8.a: Write Tests
- No new tests, verify all existing tests pass

### Step 8.b: Implement
- Update cache.rs module documentation
- Add comments explaining index structure
- Note migration strategy (no backward compatibility)
- Update CLAUDE.md if cache behavior mentioned

### Success Criteria
- Module documentation accurate
- Code comments explain key decisions
- README or CLAUDE.md updated if needed
- No dead code remaining

---

## Commit Strategy

Logical groupings to minimize repo noise:

**Commit 1: Binary cache implementation** (Steps 1-5)
- `feat(cache): add binary cache with memmap and bincode`
- Includes: dependencies, ProjectIndexEntry, read/write functions, save_binary_cache, load_binary_cache
- All tests passing for new functions
- Existing JSON cache functions unchanged

**Commit 2: CLI integration** (Step 6)
- `feat(cache): use binary cache in DiscoveryEngine for CLI operations`
- Updates DiscoveryEngine to call binary cache functions
- CLI commands work with new cache
- Data_layer still uses JSON cache

**Commit 3: Testing and benchmarks** (Steps 7-8)
- `test(cache): add integration tests and performance benchmarks`
- Integration tests for all CLI commands
- Performance benchmarks showing improvement
- All existing tests still passing

**Commit 4: Documentation** (Step 9)
- `docs(cache): update cache documentation for binary format`
- Module documentation
- CLAUDE.md updates if needed
- Code comments

All commits include standard footer with Claude Code attribution.

---

## Risk Mitigation

**Corrupted cache files**: Auto-recovery by skipping corrupted files and continuing. User can always force rescan with --no-cache.

**Cache directory permissions**: Validated during config initialization, fails early with clear error.

**Cross-platform compatibility**: bincode and memmap2 are cross-platform. SystemTime serialization tested on all platforms.

**Performance regression**: Benchmarks verify improvement. If slower, revert and investigate.

---

## Out of Scope

Deferred to future work:
- Incremental cache updates (single project refresh)
- Cache size limits or eviction
- TTL/expiration mechanisms
- Migration tool for old cache.json
- Filesystem watching for auto-invalidation
- Optimizing data_layer ResponseCache
