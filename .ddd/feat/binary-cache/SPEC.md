# Binary Cache Format Specification

Replace JSON-based project cache with memory-mapped binary format for faster CLI operations.

## Overview

**What it does**: Replaces the current single-file JSON cache (`cache.json`) with a multi-file binary cache using `bincode` + `memmap2`. Enables faster CLI operations by avoiding JSON parsing overhead and supports future incremental updates.

**Key principles**:
- Memory-mapped reads for zero-copy access
- Atomic writes for crash safety
- Per-project granularity for future incremental updates
- Backward incompatible (fresh start, no migration)

**Scope**:
- Replaces `src/discovery/cache.rs` implementation only
- CLI-focused optimization (data_layer worker cache unchanged)
- MVP supports full scans only (no incremental updates yet)

**Integration context**:
- Used by `DiscoveryEngine::get_projects()` in `src/discovery/engine.rs`
- CLI commands (`discover list/show/all`) depend on this cache
- Server mode uses data_layer cache (separate, out of scope)

## Data Model

### Modified: `src/discovery/cache.rs`

New cache structure on disk:
```
~/.config/hegel-pm/cache/
  ├── index.bin              # Vec<ProjectIndexEntry>
  ├── hegel-cli.bin          # bincode(DiscoveredProject)
  ├── hegel-pm.bin           # bincode(DiscoveredProject)
  └── my-project.bin         # bincode(DiscoveredProject)
```

### New: `ProjectIndexEntry`

Lightweight index for fast listing without loading full project data:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectIndexEntry {
    pub name: String,
    pub project_path: PathBuf,
    pub hegel_dir: PathBuf,
    pub last_activity: SystemTime,
}
```

**Purpose**: Enable `discover list` to read minimal data without deserializing all projects.

### Modified: `DiscoveredProject` (no changes to struct)

Full project data serialized to per-project `.bin` files. Already has `#[serde(skip_serializing_if = "Option::is_none")]` on `statistics` field, so it's not cached.

**Location**: `src/discovery/project.rs` (unchanged)

### Cache Directory Layout

**index.bin format**:
- Raw bincode serialization of `Vec<ProjectIndexEntry>`
- Memory-mapped for reading
- Written atomically (temp file + rename)

**{project-name}.bin format**:
- Raw bincode serialization of `DiscoveredProject`
- Statistics field is None (lazy loaded)
- Memory-mapped for reading
- Written atomically per project

## Core Operations

### `save_cache(projects: &[DiscoveredProject], config: &DiscoveryConfig) -> Result<()>`

**Behavior**:
1. Create cache directory if needed (`cache_location.parent()/cache/`)
2. For each project:
   - Serialize to bincode
   - Write to `{name}.bin.tmp`
   - Rename to `{name}.bin` (atomic)
3. Build index from all projects
4. Write `index.bin.tmp`
5. Rename to `index.bin` (atomic)

**Modified from current**: Takes `config` instead of `cache_location: &PathBuf` to construct cache dir path.

**Error handling**:
- If any project fails to serialize/write, skip it and continue
- If index write fails, return error (cache incomplete)
- Corrupted files from previous runs are overwritten

**Example**:
```rust
let config = DiscoveryConfig::default();
let projects = discover_projects(&config)?;
save_cache(&projects, &config)?;
// Result: ~/.config/hegel-pm/cache/ contains index.bin + N project files
```

### `load_cache(config: &DiscoveryConfig) -> Result<Option<Vec<DiscoveredProject>>>`

**Behavior**:
1. Check if `cache_dir/index.bin` exists
2. If not, return `Ok(None)` (cache miss)
3. Memory-map `index.bin`
4. Deserialize to `Vec<ProjectIndexEntry>`
5. For each index entry:
   - Memory-map `{name}.bin`
   - Deserialize to `DiscoveredProject`
   - If corrupted, skip and log warning
6. Return `Ok(Some(projects))`

**Modified from current**: Returns full `DiscoveredProject` list (loads all project files), takes `config` instead of path.

**Error handling**:
- Missing project file: Skip and log warning (index might be stale)
- Corrupted project file: Skip and log warning, continues
- Corrupted index file: Return error (requires full rescan)

**Example**:
```rust
let config = DiscoveryConfig::default();
match load_cache(&config)? {
    Some(projects) => println!("Loaded {} projects from cache", projects.len()),
    None => println!("Cache miss, scanning filesystem"),
}
```

### `load_project(name: &str, config: &DiscoveryConfig) -> Result<Option<DiscoveredProject>>`

**New operation** (not in current implementation):

**Behavior**:
1. Construct path: `cache_dir/{name}.bin`
2. If file doesn't exist, return `Ok(None)`
3. Memory-map file
4. Deserialize to `DiscoveredProject`
5. Return `Ok(Some(project))`

**Error handling**:
- Missing file: Return `Ok(None)`
- Corrupted file: Return error (caller can handle/rescan)

**Example**:
```rust
let config = DiscoveryConfig::default();
if let Some(project) = load_project("hegel-cli", &config)? {
    println!("Found project: {}", project.name);
}
```

**Note**: Not used in MVP, enables future optimizations.

## File Format Details

### index.bin Structure

```
┌─────────────────────────┐
│ bincode header (4 bytes)│  Length of Vec
├─────────────────────────┤
│ ProjectIndexEntry 1     │  Serialized fields
├─────────────────────────┤
│ ProjectIndexEntry 2     │
├─────────────────────────┤
│ ...                     │
└─────────────────────────┘
```

### {project-name}.bin Structure

```
┌─────────────────────────┐
│ DiscoveredProject       │  Full bincode serialization
│  - name                 │
│  - project_path         │
│  - hegel_dir            │
│  - workflow_state       │
│  - last_activity        │
│  - discovered_at        │
│  - error                │
│  - statistics: None     │  Always None in cache
└─────────────────────────┘
```

## Migration Strategy

**No backward compatibility**: Old `cache.json` is ignored. First run with new code will:
1. Not find `index.bin`
2. Return cache miss
3. Do full filesystem scan
4. Write new cache format

**Cleanup** (optional): Delete old `~/.config/hegel-pm/cache.json` manually or via cleanup script.

## Test Scenarios

### 1. Simple: Save and Load Cache

**Setup**:
- Create 2 test projects in temp directory
- Configure DiscoveryEngine to use temp cache dir

**Execute**:
```rust
let projects = discover_projects(&config)?;
save_cache(&projects, &config)?;
let loaded = load_cache(&config)?.unwrap();
```

**Verify**:
- Loaded projects match discovered projects
- `cache/index.bin` exists
- Two `.bin` files exist (one per project)
- All files are valid bincode

### 2. Complex: Load Individual Project

**Setup**:
- Cache with 10 projects exists

**Execute**:
```rust
let project = load_project("hegel-cli", &config)?;
```

**Verify**:
- Returns correct project
- Only reads `hegel-cli.bin`, not other files
- Memory-mapped (no full deserialization until needed)

### 3. Error: Corrupted Index Recovery

**Setup**:
- Valid cache exists
- Corrupt `index.bin` (write random bytes)

**Execute**:
```rust
let result = load_cache(&config);
```

**Verify**:
- Returns error (not Ok(None))
- Error message indicates corrupted cache
- Caller can handle by re-scanning

### 4. Error: Missing Project File

**Setup**:
- Valid `index.bin` with 3 entries
- Delete one `.bin` file

**Execute**:
```rust
let projects = load_cache(&config)?.unwrap();
```

**Verify**:
- Returns Ok with 2 projects (skips missing)
- Logs warning about missing file
- Does not fail entire operation

### 5. Error: Corrupted Project File

**Setup**:
- Valid cache exists
- Corrupt one project `.bin` file

**Execute**:
```rust
let projects = load_cache(&config)?.unwrap();
```

**Verify**:
- Skips corrupted project
- Returns remaining valid projects
- Logs warning

## Success Criteria

**Agent-Verifiable**:

- Tests pass: `cargo test discovery::cache`
- Build succeeds: `cargo build --features server`
- Existing CLI commands work unchanged:
  - `discover list` produces same output
  - `discover show <project>` produces same output
  - `discover all` produces same output
- Cache directory created automatically on first write
- Atomic writes verified (no partial files after crash simulation)
- Performance benchmark: `discover list --no-cache` vs cached (bincode should be faster than JSON)

**Optional Human Testing**:

- Run `discover list` after cache populated, verify instant response
- Check cache directory structure manually
- Delete individual project files, verify graceful degradation

## Out of Scope

**Deferred to future work**:
- Incremental updates (updating single project without full rescan)
- TTL/expiration (cache never expires automatically)
- Migration from old cache.json format
- Optimizing data_layer ResponseCache (keep using JSON for now)
- Filesystem watching for auto-invalidation
- Compression (bincode is already compact)
- Cache size limits or LRU eviction

## Dependencies

**New**:
```toml
[dependencies]
bincode = "1.3"
memmap2 = "0.9"
```

**Note**: bincode maintainer has controversial politics but the code is stable and widely used.
