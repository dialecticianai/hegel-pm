# Manual Edits Required for hegel-pm

After running the migration script, complete these manual edits:

## 1. Edit Cargo.toml

Remove these dependencies:
- [ ] warp, axum, tower, tower-http, async-trait
- [ ] sycamore, wasm-bindgen, wasm-bindgen-futures, gloo-net, web-sys, console_error_panic_hook
- [ ] dashmap, reqwest
- [ ] tokio (evaluate - keep only if CLI needs async)

Remove sections:
- [ ] Remove [features] section entirely
- [ ] Remove [[bin]] required-features constraint
- [ ] Change [lib] crate-type from ["cdylib", "rlib"] to ["rlib"]

## 2. Edit src/lib.rs

Remove module declarations:
- [ ] Remove: mod http;
- [ ] Remove: mod data_layer;
- [ ] Remove: mod client;
- [ ] Remove: mod server_mode;
- [ ] Remove: mod benchmark_mode;
- [ ] Remove: mod api_types;

Keep:
- [ ] Verify: pub mod discovery;
- [ ] Verify: mod cli; (if present)
- [ ] Verify: mod debug; (if present)
- [ ] Verify: mod test_helpers; (if present)

## 3. Edit src/main.rs

Remove server/benchmark logic:
- [ ] Remove server mode launch code
- [ ] Remove benchmark mode code
- [ ] Keep only CLI command dispatch (discover, hegel)

## 4. Edit scripts/test.sh

Remove frontend build:
- [ ] Remove trunk build commands
- [ ] Remove frontend-specific logic
- [ ] Keep cargo build and cargo test for CLI/lib

## 5. Verify

- [ ] Run: cargo build
- [ ] Run: cargo test
- [ ] Verify: hegel-pm discover list works
- [ ] Check test count: ~45-50 tests pass
