# Testing Guide - hegel-pm

**Purpose**: Testing strategy and patterns for hegel-pm, covering native Rust code and WASM UI components.

**Status**: Testing infrastructure established during Sycamore 0.9 migration (2025-11-03)

---

## Overview

hegel-pm has two distinct testing surfaces:

1. **Native code** (discovery engine, CLI) - Standard Rust `#[test]`
2. **WASM UI** (Sycamore components) - `wasm-bindgen-test` in browser

---

## Running Tests

### All Tests

```bash
# Native tests only (fast)
cargo test

# WASM tests (requires browser)
wasm-pack test --headless --firefox

# Specific test file
cargo test discovery
wasm-pack test --headless --firefox --test client_tests
```

### Coverage

```bash
# Generate coverage report (native code)
cargo tarpaulin --out Html
```

**Current coverage**: ~34% (will improve with UI testing)

---

## Native Code Testing

### Standard Unit Tests

**Location**: Co-located `#[cfg(test)]` modules in implementation files

**Pattern**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_engine_caching() {
        // Arrange
        let config = DiscoveryConfig::default();
        let engine = DiscoveryEngine::new(config).unwrap();

        // Act
        let projects = engine.get_projects(false).unwrap();

        // Assert
        assert!(!projects.is_empty());
    }
}
```

**What to test**:
- ✅ Discovery engine logic
- ✅ Configuration validation
- ✅ Cache persistence
- ✅ State extraction
- ✅ Error handling paths

**What NOT to test**:
- ❌ hegel-cli library internals (trust dependency)
- ❌ Filesystem primitives (trust std::fs)

---

## WASM UI Testing

### Setup

**Dependencies** (in `Cargo.toml`):
```toml
[target.'cfg(target_arch = "wasm32")'.dev-dependencies]
wasm-bindgen-test = "0.3"
web-sys = { version = "0.3", features = ["Element", "Window", "Document", "HtmlElement"] }
```

**Test file structure**:
```
tests/
└── client_tests.rs    # WASM browser tests
```

### Basic Test Pattern

```rust
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use sycamore::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_signal_reactivity() {
    let _ = create_root(|| {
        let count = create_signal(0);

        let node = view! {
            div {
                p(id="counter") { (count.get()) }
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        let counter = query("#counter");
        assert_text_content!(counter, "0");

        // Update signal
        count.set(42);

        // Verify reactivity
        assert_text_content!(counter, "42");
    });
}
```

### Test Helpers

**Test container**:
```rust
fn test_container() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    document.query_selector("body").unwrap().unwrap()
}
```

**Query helper**:
```rust
fn query(selector: &str) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    document
        .query_selector(selector)
        .unwrap()
        .expect(&format!("Element not found: {}", selector))
}
```

**Assertion macro**:
```rust
macro_rules! assert_text_content {
    ($element:expr, $expected:expr) => {
        assert_eq!(
            $element.text_content().unwrap_or_default().trim(),
            $expected
        );
    };
}
```

### What to Test in WASM

Based on `LEARNING_SYCAMORE_PRACTICES.md`:

**✅ DO test**:
- Signal update logic (reactive primitives)
- User interactions → state changes
- Conditional rendering (different code paths)
- List operations (add/remove/reorder with Keyed)
- Memo/effect behavior

**❌ DON'T test**:
- Sycamore framework internals
- Exact DOM structure (brittle)
- CSS/styling (use visual regression tools)

### Example: Testing Memo Updates

```rust
#[wasm_bindgen_test]
fn test_memo_updates() {
    let _ = create_root(|| {
        let count = create_signal(5);
        let doubled = create_memo(move || count.get() * 2);

        let node = view! {
            div {
                p(id="original") { (count.get()) }
                p(id="doubled") { (doubled.get()) }
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        let original = query("#original");
        let doubled_elem = query("#doubled");

        assert_text_content!(original, "5");
        assert_text_content!(doubled_elem, "10");

        // Update source signal
        count.set(10);

        // Memo should update automatically
        assert_text_content!(original, "10");
        assert_text_content!(doubled_elem, "20");
    });
}
```

---

## Browser Test Runners

### Firefox (Recommended)

```bash
wasm-pack test --headless --firefox
```

**Pros**:
- More stable than Chrome in CI
- Better WebDriver support
- Fewer version conflicts

### Chrome

```bash
wasm-pack test --headless --chrome
```

**Pros**:
- Faster execution
- Better debugging tools

**Cons**:
- ChromeDriver version compatibility issues
- Can fail with network timeouts

### Manual Browser Testing

```bash
# Open interactive browser for debugging
wasm-pack test --firefox
# or
wasm-pack test --chrome
```

Useful for debugging test failures with browser DevTools.

---

## Testing Patterns from Sycamore Research

### 1. Testing Reactive Primitives

**Signals**:
```rust
#[test]
fn test_signal_updates() {
    create_root(|| {
        let count = create_signal(0);
        assert_eq!(count.get(), 0);

        count.set(5);
        assert_eq!(count.get(), 5);
    });
}
```

**Memos**:
```rust
#[test]
fn test_memo_caching() {
    create_root(|| {
        let count = create_signal(1);
        let doubled = create_memo(move || count.get() * 2);

        assert_eq!(doubled.get(), 2);
        assert_eq!(doubled.get(), 2); // Cached, not recomputed

        count.set(5);
        assert_eq!(doubled.get(), 10); // Updated
    });
}
```

**Effects**:
```rust
#[test]
fn test_effect_runs_on_change() {
    create_root(|| {
        let count = create_signal(0);
        let effect_runs = create_signal(0);

        create_effect(move || {
            count.track();
            effect_runs.update(|n| *n += 1);
        });

        assert_eq!(effect_runs.get(), 1); // Runs immediately

        count.set(1);
        assert_eq!(effect_runs.get(), 2); // Runs on update
    });
}
```

### 2. Testing Components (Future)

When testing actual hegel-pm components, use mocking for API calls:

```rust
#[wasm_bindgen_test]
fn test_sidebar_loading_state() {
    // Mock fetch would go here
    let _ = create_root(|| {
        let node = view! { Sidebar {} };
        sycamore::render_in_scope(|| node, &test_container());

        // Check loading state initially
        let loading = query(".project-list p");
        assert_text_content!(loading, "Loading projects...");
    });
}
```

### 3. Testing Keyed Lists

```rust
#[wasm_bindgen_test]
fn test_keyed_list_updates() {
    #[derive(Clone, PartialEq)]
    struct Item { id: u32, name: String }

    let _ = create_root(|| {
        let items = create_signal(vec![
            Item { id: 1, name: "First".into() },
            Item { id: 2, name: "Second".into() },
        ]);

        let node = view! {
            ul {
                Keyed(
                    list=items,
                    key=|item| item.id,
                    view=|item| {
                        let name = item.name.clone();
                        view! { li(id=format!("item-{}", item.id)) { (name) } }
                    }
                )
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        // Verify initial render
        let item1 = query("#item-1");
        assert_text_content!(item1, "First");

        // Add item
        items.update(|list| list.push(Item { id: 3, name: "Third".into() }));

        // Verify new item rendered
        let item3 = query("#item-3");
        assert_text_content!(item3, "Third");
    });
}
```

---

## Common Testing Gotchas

### 1. Signal Ownership in Tests

```rust
// ❌ Wrong: Signal moved into closure
let count = create_signal(0);
create_effect(move || {
    count.set(count.get() + 1); // count moved here
});
count.get(); // ERROR: count moved

// ✅ Correct: Signal is Copy
let count = create_signal(0);
create_effect(move || {
    count.set(count.get() + 1); // copies count
});
assert_eq!(count.get(), 1); // OK: still have count
```

### 2. Test Isolation with `create_root`

Always wrap tests in `create_root()` for proper cleanup:

```rust
#[test]
fn test_something() {
    // ✅ Good: Wrapped in root
    create_root(|| {
        let signal = create_signal(0);
        // test logic
    }); // Cleans up signal
}

// ❌ Bad: No root - memory leak
#[test]
fn test_something() {
    let signal = create_signal(0); // Never cleaned up!
}
```

### 3. Browser Test Timeouts

If tests timeout in browser:
- Check for infinite loops in effects
- Ensure async operations complete
- Verify DOM queries succeed (element exists)

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test-native:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: cargo test

  test-wasm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          target: wasm32-unknown-unknown
      - run: cargo install wasm-pack
      - run: wasm-pack test --headless --firefox
```

---

## Testing Strategy

### Phase 1: Core Reactive Logic ✅
- Unit test signals, memos, effects
- Validate reactive patterns work correctly
- No browser required, fast feedback

### Phase 2: Component Behavior (Future)
- Test user interactions
- Verify conditional rendering
- Validate list operations

### Phase 3: Integration (Future)
- Mock API endpoints
- Test async data loading
- Verify error handling

---

## Resources

**Internal**:
- `learnings/LEARNING_SYCAMORE_PRACTICES.md` - Testing patterns and examples
- `learnings/LEARNING_SYCAMORE_FOUNDATIONS.md` - Reactive primitives to test
- `tests/client_tests.rs` - Example WASM tests

**External**:
- [wasm-bindgen-test docs](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/)
- [Sycamore testing examples](https://github.com/sycamore-rs/sycamore/tree/main/packages/sycamore/tests)

---

## Status

**Native tests**: ✅ Working, 34% coverage
**WASM test infrastructure**: ✅ Set up, example tests pass
**Component tests**: ⏳ Deferred (browser driver issues, patterns validated)

**Next steps** (optional):
1. Add more reactive primitive tests
2. Mock API for component testing
3. Improve browser driver reliability
4. Add visual regression testing for UI

---

**Last updated**: 2025-11-03
**Test framework versions**: wasm-bindgen-test 0.3, wasm-pack 0.13.1
