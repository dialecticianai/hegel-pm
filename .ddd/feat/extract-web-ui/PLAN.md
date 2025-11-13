# Web UI Extraction Plan

Extract web/server functionality from hegel-pm into new hegel-pm-web repository using Perl migration script.

---

## Overview

**Goal**: Split hegel-pm into two repositories - pure CLI/library (hegel-pm) and web dashboard (hegel-pm-web) - using script for mechanical operations and manual edits for code surgery.

**Scope**: Script handles file moves and deletions. Manual edits handle configuration updates, import fixes, and code modifications. Validation that both repos build and all tests pass.

**Priorities**:
1. Zero functionality loss - all tests accounted for
2. Clean separation - no ambiguous file placement
3. Correct dependency direction - hegel-pm-web depends on hegel-pm
4. Script does mechanical work - no complex parsing/editing

**Methodology**: This is a code migration, not feature development. Script automates the tedious file operations, generates checklist for manual edits. No TDD - validate via existing tests in both repositories.

**Division of Labor**:
- **Script**: File/directory moves, deletions, skeleton creation, checklist generation
- **Manual**: Cargo.toml edits, lib.rs/main.rs modifications, import updates, test script fixes

---

## Step 1: Create Migration Script (Mechanical Operations Only)

### Goal
Create Perl oneoff script that handles file moves, deletions, and skeleton creation. Generate checklist for manual edits.

### Script Operations
Location: scripts/oneoffs/20251113-extract-web-ui.pl

The script performs ONLY mechanical operations:
- Create hegel-pm-web directory structure at ../hegel-pm-web
- Move web-specific files/directories from hegel-pm to hegel-pm-web
- Delete moved files from hegel-pm
- Copy scaffold files (LICENSE, scripts)
- Create skeleton Cargo.toml files (with TODO comments)
- Generate MANUAL_EDITS.md checklist in both repos

Support dry-run mode via --dry-run flag to preview changes.

### What Script Does NOT Do
Script will not attempt:
- TOML parsing or modification
- Rust code editing or import rewriting
- Complex text transformations
- Build validation

Those operations require manual edits or happen during validation phase.

### Success Criteria
- Script created at correct location
- Dry-run mode shows planned file operations
- Generates actionable checklist for manual edits
- Script follows oneoff pattern with summary

---

## Step 2: Script Execution and File Moves

### Goal
Execute script to perform all mechanical file operations.

### Execution Sequence
Run script with dry-run first:
- Execute: ./scripts/oneoffs/20251113-extract-web-ui.pl --dry-run
- Review planned operations
- Verify file moves are correct

Execute actual migration:
- Run: ./scripts/oneoffs/20251113-extract-web-ui.pl
- Monitor for errors
- Review generated MANUAL_EDITS.md checklist

### Files Moved by Script
Directories moved to hegel-pm-web/src/:
- http/
- data_layer/
- client/

Files moved to hegel-pm-web/src/:
- server_mode.rs
- benchmark_mode.rs
- api_types.rs

Directories moved to hegel-pm-web/:
- frontends/
- static/
- tests/

Files moved to hegel-pm-web/:
- index.html
- scripts/restart-server.sh

Files copied to hegel-pm-web/:
- LICENSE
- .gitignore (modified for web project)

### Files Deleted from hegel-pm
After moving, script deletes from hegel-pm:
- All moved files and directories listed above
- scripts/restart-server.sh

### Skeleton Files Created
Script creates with TODO markers:
- hegel-pm-web/Cargo.toml (with dependency stubs)
- hegel-pm-web/src/lib.rs (empty with TODO)
- hegel-pm-web/src/main.rs (empty with TODO)
- hegel-pm-web/README.md (stub)
- hegel-pm/MANUAL_EDITS.md (checklist)
- hegel-pm-web/MANUAL_EDITS.md (checklist)

### Success Criteria
- Dry-run completes without errors
- All files moved successfully
- No files lost
- Skeleton files created
- Checklists generated

---

## Step 3: Manual Edit - hegel-pm Cargo.toml

### Goal
Remove all web-specific dependencies and features from hegel-pm.

### Edits Required
Remove these dependency sections:
- warp, axum, tower, tower-http, async-trait
- sycamore, wasm-bindgen, wasm-bindgen-futures, gloo-net, web-sys
- dashmap, reqwest
- tokio (evaluate if CLI needs async - likely not)

Remove features section entirely:
- default, server, warp-backend, axum-backend all go away

Remove bin section required-features constraint.

Change library crate-type from cdylib, rlib to just rlib.

### Success Criteria
- Cargo.toml has minimal dependencies (just hegel, clap, serde, walkdir, dirs, chrono, anyhow)
- No feature flags
- No WASM-specific dependencies
- cargo build succeeds

---

## Step 4: Manual Edit - hegel-pm-web Cargo.toml

### Goal
Create complete Cargo.toml for web repository with all necessary dependencies.

### Structure to Create
Package section:
- name = "hegel-pm-web"
- version = "0.0.1"
- edition = "2021"

Add dependencies:
- hegel = { path = "../hegel-cli" }
- hegel-pm = { path = "../hegel-pm" }
- Copy all web dependencies from original hegel-pm Cargo.toml (warp, axum, sycamore, etc.)

Add features (copy from original):
- default = ["warp-backend"]
- server = []
- warp-backend = ["warp"]
- axum-backend = ["axum", "tower", "tower-http"]

Add bin section:
- name = "hegel-pm-web"
- path = "src/main.rs"
- required-features = ["server"]

Add lib section:
- crate-type = ["cdylib", "rlib"]

### Success Criteria
- Valid TOML structure
- All dependencies present
- cargo build --features server succeeds

---

## Step 5: Manual Edit - hegel-pm src/lib.rs and src/main.rs

### Goal
Simplify hegel-pm to be CLI/library only.

### src/lib.rs Edits
Remove module declarations for:
- mod http
- mod data_layer
- mod client
- mod server_mode
- mod benchmark_mode
- mod api_types

Keep:
- pub mod discovery
- mod test_helpers (if used)
- mod debug

### src/main.rs Edits
Remove server/benchmark mode logic entirely.

Keep only:
- CLI argument parsing for discover and hegel commands
- Dispatch to discovery engine
- Dispatch to hegel command passthrough

Strip to minimal CLI-only binary.

### Success Criteria
- cargo build succeeds
- hegel-pm binary runs discovery commands
- No references to web modules

---

## Step 6: Manual Edit - hegel-pm-web src/lib.rs and src/main.rs

### Goal
Create web-specific library and binary entry points.

### src/lib.rs Creation
Declare modules for moved code:
- pub mod http
- pub mod data_layer
- pub mod client
- pub mod server_mode
- pub mod benchmark_mode
- pub mod api_types

Note: src/debug.rs and src/test_helpers.rs remain in hegel-pm, so don't declare them here.

### src/main.rs Creation
Copy original hegel-pm main.rs server/benchmark logic.

Remove CLI discover command logic.

Entry point decides: server mode vs benchmark mode.

### Import Fixes
All files in hegel-pm-web that import discovery need updates:
- Change crate::discovery::* to hegel_pm::discovery::*
- Update in: data_layer/worker.rs, http backends, etc.

Use find/replace or manual editing.

### Success Criteria
- cargo build --features server succeeds
- hegel-pm-web binary starts server
- All imports resolve correctly

---

## Step 7: Manual Edit - Build Scripts

### Goal
Update test.sh in both repos to handle new structure.

### hegel-pm scripts/test.sh
Remove:
- Trunk build commands
- Frontend build logic
- Server-specific test flags

Keep:
- cargo build for library
- cargo test for CLI and discovery tests

### hegel-pm-web scripts/test.sh
Copy from hegel-pm (if needed), or create new:
- Trunk build for frontend
- cargo build --features server for backend
- cargo test --features server

### hegel-pm-web scripts/restart-server.sh
Should already be moved by script - verify it works.

### Success Criteria
- ./scripts/test.sh runs in both repos
- No broken script dependencies

---

## Step 8: Validation and Testing

### Goal
Verify both repositories build and all tests pass after all manual edits complete.

### hegel-pm Validation
Run in migrated hegel-pm directory:
- cargo build - must succeed with no web dependencies
- cargo test - must pass ~45-50 tests (discovery + CLI + doctests)
- hegel-pm discover list - CLI commands work
- Binary size reduced (no WASM/web libs)

### hegel-pm-web Validation
Run in new hegel-pm-web directory:
- cargo build --features server - must succeed
- cargo test --features server - must pass ~93 tests (web UI + data layer + HTTP)
- Integration tests run (5 test files: async_component_lifecycle, client_tests, collapse_state, navigation, reactive_primitives)
- ./scripts/test.sh - full build and test cycle works
- hegel-pm-web starts server on localhost:3030

### Cross-Repository Validation
Verify integration:
- hegel-pm-web successfully imports hegel_pm::discovery
- Total test count preserved: 139 tests across both repos (93 + 45 + 1)
- 4 ignored tests accounted for
- No duplicate tests between repos
- Both repos can be developed independently

### Success Criteria
- Both repos build without errors
- All 139 tests pass (split across repos as expected)
- No test functionality lost
- Library dependency works correctly
- Baseline test counts match: hegel-pm ~45-50, hegel-pm-web ~93, doctest 1

---

## Commit Strategy

**Single commit per repo after all steps complete**

This is a structural reorganization, not incremental development. Make one atomic commit in each repository once validation passes.

**Commit in hegel-pm:**
- Type: refactor(extraction)
- Subject: extract web UI to separate hegel-pm-web repository
- Body:
  - Document what was removed (HTTP server, data layer, frontends, web UI)
  - Document what remains (CLI, discovery library)
  - Note simplified dependencies and binary
  - Reference hegel-pm-web as new home for web functionality
- Include: Cargo.toml changes, lib.rs/main.rs simplification, deleted files, updated test.sh

**Commit in hegel-pm-web (initial):**
- Type: feat(init)
- Subject: initialize hegel-pm-web repository from hegel-pm extraction
- Body:
  - Document what was moved (server, data layer, frontends, tests)
  - Note dependency on hegel-pm library for discovery engine
  - Explain purpose as web dashboard for Hegel projects
- Include: all moved files, new Cargo.toml, lib.rs/main.rs, updated imports, build scripts

Do not commit the migration script itself - it's a throwaway oneoff tool.

---

## Summary

This plan splits hegel-pm into two repositories using a pragmatic hybrid approach: Perl script handles mechanical file operations (moves, deletions, scaffolding), then manual edits handle code surgery (Cargo.toml, imports, main.rs logic). Script generates MANUAL_EDITS.md checklists to guide the manual work. Validation ensures both repositories build and all 139 tests pass. No functionality lost - just reorganized into cleaner boundaries with hegel-pm as CLI/library and hegel-pm-web as standalone web dashboard.
