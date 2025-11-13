# Testing Guide - hegel-pm

**Purpose**: Testing strategy and patterns for hegel-pm discovery library and CLI.

**Status**: TDD discipline with ≥80% line coverage enforced by pre-commit hook

---

## Overview

hegel-pm is a pure Rust library and CLI tool for discovering Hegel projects across the filesystem. Testing focuses on:

1. **Discovery engine** - Filesystem walking, project detection, caching
2. **CLI commands** - Argument parsing, output formatting, error handling
3. **Integration** - End-to-end discovery workflows

---

## Running Tests

### Quick Start

```bash
# Recommended: Build + test script
./scripts/test.sh

# Or run tests directly
cargo test

# Run specific module tests
cargo test discovery
cargo test cli

# Run with output
cargo test -- --nocapture

# Run single test
cargo test test_discovery_engine_finds_projects
```

### Coverage

```bash
# Generate coverage report (enforced by pre-commit hook)
./scripts/generate-coverage-report.sh

# Or use cargo-llvm-cov directly
cargo llvm-cov test --html
open target/llvm-cov/html/index.html
```

**Current coverage**: See `COVERAGE_REPORT.md` (auto-generated)
**Target**: ≥80% line coverage (enforced by pre-commit hook)

---

## Testing Philosophy

**TDD discipline**: Code exists because tests drove its implementation.

**Coverage target**: ≥80% lines (enforced by pre-commit hook)

**What to test**:
- ✅ State parsing and serialization (JSONL format correctness)
- ✅ Multi-project discovery and tracking logic
- ✅ Workflow state interpretation
- ✅ Filesystem walking and `.hegel` detection
- ✅ Metrics extraction and aggregation
- ✅ Cache persistence and invalidation
- ✅ CLI argument parsing and output formatting
- ✅ Error handling paths

**What NOT to test**:
- ❌ Third-party library behavior (serde, hegel-cli internals)
- ❌ File system primitives (trust std::fs)
- ❌ External dependencies (walkdir, clap)

**Test organization**: Co-located `#[cfg(test)]` modules in implementation files

---

## Test Structure

### Unit Tests

**Location**: Co-located with implementation code in `#[cfg(test)]` modules

**Pattern**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_config_default() {
        let config = DiscoveryConfig::default();
        assert!(config.roots.len() > 0);
        assert_eq!(config.max_depth, 10);
    }

    #[test]
    fn test_cache_persistence() {
        let cache = Cache::new(temp_path());
        cache.save(&projects).unwrap();

        let loaded = Cache::load(&temp_path()).unwrap();
        assert_eq!(loaded.len(), projects.len());
    }
}
```

### Integration Tests

**Location**: `tests/` directory (if needed for end-to-end scenarios)

**Pattern**:
```rust
// tests/integration_test.rs
use hegel_pm::discovery::{DiscoveryConfig, DiscoveryEngine};

#[test]
fn test_discover_and_cache_workflow() {
    let config = DiscoveryConfig::default();
    let engine = DiscoveryEngine::new(config).unwrap();

    // First discovery (fresh scan)
    let projects1 = engine.get_projects(false).unwrap();

    // Second discovery (cached)
    let projects2 = engine.get_projects(false).unwrap();

    assert_eq!(projects1.len(), projects2.len());
}
```

### Test Helpers

**Location**: `src/test_helpers.rs`

Common utilities for creating test fixtures:
```rust
pub fn create_temp_hegel_project(name: &str) -> TempDir {
    let dir = TempDir::new().unwrap();
    let hegel_dir = dir.path().join(".hegel");
    fs::create_dir(&hegel_dir).unwrap();

    // Create minimal state.json
    let state = json!({
        "workflow": "discovery",
        "node": "spec"
    });
    fs::write(hegel_dir.join("state.json"), state.to_string()).unwrap();

    dir
}
```

---

## Testing Discovery Engine

### Core Functionality

**Project detection**:
```rust
#[test]
fn test_finds_hegel_projects() {
    let temp = create_temp_workspace();
    create_hegel_project(&temp, "project1");
    create_hegel_project(&temp, "project2");

    let config = DiscoveryConfig::new(vec![temp.path()], 10, vec![], cache_path());
    let engine = DiscoveryEngine::new(config).unwrap();

    let projects = engine.get_projects(true).unwrap();
    assert_eq!(projects.len(), 2);
}
```

**Exclusion handling**:
```rust
#[test]
fn test_respects_exclusions() {
    let temp = create_temp_workspace();
    create_hegel_project(&temp, "project1");
    create_hegel_project(&temp.path().join("node_modules"), "ignored");

    let config = DiscoveryConfig::new(
        vec![temp.path()],
        10,
        vec!["node_modules".to_string()],
        cache_path()
    );
    let engine = DiscoveryEngine::new(config).unwrap();

    let projects = engine.get_projects(true).unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "project1");
}
```

**Max depth enforcement**:
```rust
#[test]
fn test_respects_max_depth() {
    let temp = create_temp_workspace();
    let deep = temp.path().join("a/b/c/d/e/f");
    fs::create_dir_all(&deep).unwrap();
    create_hegel_project(&deep, "too-deep");

    let config = DiscoveryConfig::new(vec![temp.path()], 3, vec![], cache_path());
    let engine = DiscoveryEngine::new(config).unwrap();

    let projects = engine.get_projects(true).unwrap();
    assert_eq!(projects.len(), 0);
}
```

### Caching Behavior

**Cache persistence**:
```rust
#[test]
fn test_cache_persists_discoveries() {
    let config = DiscoveryConfig::default();
    let engine = DiscoveryEngine::new(config).unwrap();

    // Fresh discovery
    let projects1 = engine.get_projects(true).unwrap();

    // Create new engine, should load from cache
    let engine2 = DiscoveryEngine::new(config).unwrap();
    let projects2 = engine2.get_projects(false).unwrap();

    assert_eq!(projects1.len(), projects2.len());
}
```

**Cache invalidation**:
```rust
#[test]
fn test_cache_invalidation_on_state_change() {
    let temp = create_temp_hegel_project("test");
    let config = DiscoveryConfig::new(vec![temp.path()], 10, vec![], cache_path());
    let engine = DiscoveryEngine::new(config).unwrap();

    // Initial discovery
    let projects1 = engine.get_projects(true).unwrap();

    // Modify state.json (change mtime)
    std::thread::sleep(Duration::from_millis(10));
    modify_state_file(&temp);

    // Should detect change and refresh
    let projects2 = engine.get_projects(false).unwrap();
    assert_ne!(projects1[0].last_modified, projects2[0].last_modified);
}
```

---

## Testing CLI Commands

### Argument Parsing

```rust
#[test]
fn test_cli_discover_list_args() {
    let args = Cli::parse_from(["hegel-pm", "discover", "list"]);
    assert!(matches!(args.command, Command::Discover(DiscoverCmd::List)));
}

#[test]
fn test_cli_json_flag() {
    let args = Cli::parse_from(["hegel-pm", "--json", "discover", "list"]);
    assert!(args.json);
}
```

### Output Formatting

```rust
#[test]
fn test_format_project_list() {
    let projects = vec![
        Project { name: "proj1".into(), workflow: Some("discovery".into()), ... },
        Project { name: "proj2".into(), workflow: None, ... },
    ];

    let output = format_project_list(&projects);
    assert!(output.contains("proj1"));
    assert!(output.contains("discovery"));
    assert!(output.contains("proj2"));
}

#[test]
fn test_json_output() {
    let projects = vec![Project { ... }];
    let json = serde_json::to_string(&projects).unwrap();

    // Verify valid JSON
    let parsed: Vec<Project> = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.len(), projects.len());
}
```

---

## Testing Error Handling

### Graceful Degradation

```rust
#[test]
fn test_corrupted_hegel_dir_doesnt_crash() {
    let temp = create_temp_workspace();
    let hegel_dir = temp.path().join("broken-project/.hegel");
    fs::create_dir_all(&hegel_dir).unwrap();

    // Create invalid state.json
    fs::write(hegel_dir.join("state.json"), "invalid json").unwrap();

    let config = DiscoveryConfig::new(vec![temp.path()], 10, vec![], cache_path());
    let engine = DiscoveryEngine::new(config).unwrap();

    // Should continue discovering other projects
    let result = engine.get_projects(true);
    assert!(result.is_ok());
}
```

### Error Context

```rust
#[test]
fn test_error_includes_file_path() {
    let result = DiscoveryEngine::load_state("/nonexistent/.hegel/state.json");

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("/nonexistent/.hegel/state.json"));
}
```

---

## Common Testing Patterns

### 1. Temporary Directories

Use `tempfile` crate for isolated test environments:

```rust
use tempfile::TempDir;

#[test]
fn test_with_temp_dir() {
    let temp = TempDir::new().unwrap();

    // Create test files in temp.path()

    // temp is automatically cleaned up when dropped
}
```

### 2. Fixture Data

Create reusable test fixtures:

```rust
fn create_test_project(dir: &Path, name: &str, workflow: &str) -> PathBuf {
    let project_dir = dir.join(name);
    let hegel_dir = project_dir.join(".hegel");
    fs::create_dir_all(&hegel_dir).unwrap();

    let state = json!({
        "workflow": workflow,
        "node": "spec",
        "history": []
    });
    fs::write(hegel_dir.join("state.json"), state.to_string()).unwrap();

    project_dir
}
```

### 3. Mock hegel-cli Data

Test metrics extraction without full hegel-cli setup:

```rust
fn create_mock_metrics(hegel_dir: &Path) -> std::io::Result<()> {
    let hooks = vec![
        json!({"event": "tool_use", "timestamp": "2025-01-01T00:00:00Z"}),
        json!({"event": "file_write", "timestamp": "2025-01-01T00:01:00Z"}),
    ];

    let mut file = fs::File::create(hegel_dir.join("hooks.jsonl"))?;
    for hook in hooks {
        writeln!(file, "{}", hook.to_string())?;
    }
    Ok(())
}
```

---

## Performance Testing

### Benchmark-style Tests

```rust
#[test]
fn test_discovery_performance() {
    let temp = create_large_workspace(100); // 100 projects

    let config = DiscoveryConfig::new(vec![temp.path()], 10, vec![], cache_path());
    let engine = DiscoveryEngine::new(config).unwrap();

    let start = Instant::now();
    let projects = engine.get_projects(true).unwrap();
    let duration = start.elapsed();

    assert_eq!(projects.len(), 100);
    assert!(duration < Duration::from_secs(5), "Discovery took {:?}", duration);
}
```

### Cache Performance

```rust
#[test]
fn test_cached_discovery_is_fast() {
    let config = DiscoveryConfig::default();
    let engine = DiscoveryEngine::new(config).unwrap();

    // Prime cache
    engine.get_projects(true).unwrap();

    // Measure cached access
    let start = Instant::now();
    let projects = engine.get_projects(false).unwrap();
    let duration = start.elapsed();

    assert!(duration < Duration::from_millis(100), "Cached discovery took {:?}", duration);
}
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: ./scripts/test.sh

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: cargo install cargo-llvm-cov
      - run: cargo llvm-cov test --lcov --output-path lcov.info
      - uses: codecov/codecov-action@v3
        with:
          files: lcov.info
```

---

## Pre-commit Hook

Coverage is enforced automatically:

```bash
# Installed by .git/hooks/pre-commit
# Runs on every commit:
1. rustfmt on staged .rs files
2. Coverage report generation
3. Fails commit if coverage < 80%
```

**Manual coverage check**:
```bash
./scripts/generate-coverage-report.sh
cat COVERAGE_REPORT.md  # View results
```

---

## Debugging Tests

### Print Output

```rust
#[test]
fn test_with_debug_output() {
    let projects = engine.get_projects(true).unwrap();

    // Print during test
    println!("Found {} projects", projects.len());
    for p in &projects {
        println!("  - {}: {:?}", p.name, p.workflow);
    }

    assert!(!projects.is_empty());
}

// Run with: cargo test -- --nocapture
```

### Conditional Logging

```rust
#[test]
fn test_with_logging() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }

    // Use log macros
    log::debug!("Starting discovery...");

    let projects = engine.get_projects(true).unwrap();
    log::info!("Found {} projects", projects.len());
}

// Run with: RUST_LOG=debug cargo test
```

---

## Test Organization Checklist

- [ ] Each module has `#[cfg(test)]` mod tests
- [ ] Test names clearly describe what they test
- [ ] Tests are independent (no shared state)
- [ ] Temporary files cleaned up (use TempDir)
- [ ] Error cases tested, not just happy path
- [ ] Edge cases covered (empty lists, max depth, etc.)
- [ ] Coverage ≥80% (enforced by pre-commit)

---

## Resources

**Internal**:
- `src/test_helpers.rs` - Shared test utilities
- `COVERAGE_REPORT.md` - Auto-generated coverage report
- `scripts/test.sh` - Build + test script
- `scripts/generate-coverage-report.sh` - Coverage generation

**External**:
- [Rust testing book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)

---

**Last updated**: 2025-11-13
**Test coverage**: See `COVERAGE_REPORT.md` (≥80% target)
