# Web UI Extraction Specification

Split hegel-pm into two repos: hegel-pm (CLI/lib) and hegel-pm-web (server/UI).

---

## What Moves Where

### hegel-pm (stays, becomes CLI/lib)
**Keep:**
- `src/discovery/` - becomes library export
- `src/cli/` - CLI commands
- Core files: `src/lib.rs`, `src/main.rs` (both modified)

**Remove:**
- `src/http/`, `src/data_layer/`, `src/client/`
- `src/server_mode.rs`, `src/benchmark_mode.rs`, `src/api_types.rs`
- `frontends/`, `static/`, `index.html`
- `tests/` - all web UI tests
- `scripts/restart-server.sh`

### hegel-pm-web (new repo at ../hegel-pm-web)
**Move from hegel-pm:**
- Everything removed above
- Modified `Cargo.toml` (adds `hegel-pm = { path = "../hegel-pm" }`)
- Modified `src/main.rs` (server logic only)
- `scripts/restart-server.sh`, `scripts/test.sh` (web-specific)

---

## Key File Changes

### hegel-pm/Cargo.toml
**Remove dependencies:** warp, axum, tower, sycamore, wasm-bindgen, dashmap, reqwest, tokio (evaluate)
**Remove features:** `server`, `warp-backend`, `axum-backend`
**Remove:** `[[bin]]` required-features
**Change:** `crate-type = ["rlib"]` (no cdylib)

### hegel-pm/src/lib.rs
Export discovery module:
```rust
pub mod discovery;
```

### hegel-pm/src/main.rs
CLI only - remove server mode, benchmark mode.

### hegel-pm-web/Cargo.toml
Add dependency:
```toml
hegel-pm = { path = "../hegel-pm" }
```

---

## Success Criteria

**Baseline (before extraction):**
- Total tests: 139 (93 lib + 45 CLI + 1 doctest)
- Ignored tests: 4
- Integration test files: 5 (async_component_lifecycle, client_tests, collapse_state, navigation, reactive_primitives)

**After extraction:**

**hegel-pm:**
- `cargo build` succeeds
- `cargo test` passes: ~45-50 tests (discovery engine + CLI commands + doctests)
- `hegel-pm discover list` works
- Binary smaller than current (no web dependencies)

**hegel-pm-web:**
- `cargo build --features server` succeeds
- `cargo test --features server` passes: ~93+ tests (all web UI + data layer + HTTP tests)
- Integration tests: 5 test files moved from hegel-pm
- `hegel-pm-web` starts server on localhost:3030
- Browser opens to dashboard
- Uses `hegel_pm::discovery::*` successfully

**Cross-repo verification:**
- No duplicate tests between repos
- Total test count preserved: 139 tests across both repos
- No test functionality lost
