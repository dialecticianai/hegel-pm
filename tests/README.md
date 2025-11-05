# tests/

WASM browser tests for Sycamore UI components. Tests run with wasm-bindgen-test in headless browser.

## Structure

```
tests/
└── client_tests.rs             WASM tests for signal reactivity, collapse state, navigation routing
```

## Test Coverage

**Signal reactivity**: Verify signals update correctly on user interactions
**Collapse state**: Test workflow/phase expand/collapse functionality
**Navigation**: Verify View enum routing and project selection
**Component behavior**: Test sidebar navigation, view switching

## Running Tests

```bash
# Run WASM tests in headless browser
wasm-pack test --headless --firefox

# Or use the test script
./scripts/test.sh --exclude backend
```

## Test Patterns

Tests follow Sycamore WASM testing patterns from `learnings/LEARNING_SYCAMORE_PRACTICES.md`:
- Wrap in `create_root()` for scope management
- Use `sycamore::render_in_scope()` to render to test container
- Query DOM elements with `query()` helper
- Assert text content with `assert_text_content!()` macro
- Test signal updates, not exact DOM structure
