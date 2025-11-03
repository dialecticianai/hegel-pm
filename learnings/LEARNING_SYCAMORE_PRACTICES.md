# Sycamore Performance & Testing

**Purpose**: Performance optimization patterns, testing strategies, common pitfalls

**Audience**: AI agents (concise, technical) + human-friendly

---

## Performance Principles

### Fine-Grained Updates = Inherent Performance

**Key advantage**: Sycamore updates O(dependencies), not O(component tree).

```rust
// When count changes, only the text node updates
// NOT the entire component, NOT sibling elements
let count = create_signal(0);
view! {
    div {
        h1 { "Header" }              // Never re-renders
        p { (count.get()) }          // Updates when count changes
        footer { "Footer" }          // Never re-renders
    }
}
```

**Contrast with VDOM**: React/Vue re-render component → diff → patch. Sycamore: signal → subscribers update directly.

---

## Batching Updates

### `batch()` - Defer Effect Execution

```rust
use sycamore::reactive::batch;

let count = create_signal(0);
let doubled = create_signal(0);

create_effect(move || {
    // Expensive operation
    doubled.set(count.get() * 2);
    expensive_render();
});

// Without batch: effect runs 3 times
count.set(1);
count.set(2);
count.set(3);

// With batch: effect runs once
batch(|| {
    count.set(1);
    count.set(2);
    count.set(3);
});  // Effect runs here, sees count=3
```

**How it works**:
1. `start_batch()` - Set batching flag
2. Signal updates queue to `node_update_queue` (don't run yet)
3. `end_batch()` - Process queue, run effects once

**When to use**:
- Multiple related state updates (form submission, bulk data import)
- Avoid intermediate inconsistent states
- Reduce redundant effect executions

**Practical example - Async data loading**:

```rust
let data = create_signal(Vec::new());
let loading = create_signal(true);
let error = create_signal(false);

spawn_local(async move {
    match fetch_data().await {
        Ok(result) => {
            // ✅ Good: Batch related updates
            batch(|| {
                data.set(result);
                loading.set(false);
            });
        }
        Err(_) => {
            // ✅ Good: Batch error state too
            batch(|| {
                error.set(true);
                loading.set(false);
            });
        }
    }
});
```

Without batching, UI might briefly show "loading=false" with stale data, causing flicker.

---

## Memoization Strategy

### Memo vs Derived Function

```rust
let expensive_data = create_signal(vec![1, 2, 3, /* ...1000 items */]);

// Bad: Recomputes every access
let sum_fn = || expensive_data.with(|data| data.iter().sum::<i32>());
view! {
    p { (sum_fn()) }     // Computes
    p { (sum_fn()) }     // Computes again (wasted!)
}

// Good: Computes once, caches result
let sum_memo = create_memo(move || expensive_data.with(|data| data.iter().sum::<i32>()));
view! {
    p { (sum_memo.get()) }  // Cached
    p { (sum_memo.get()) }  // Cached
}
```

**Rule of thumb**:
- **Memo**: Expensive computation (>1ms), accessed multiple times
- **Function**: Trivial computation (<1µs), accessed once

**Overhead**: Memo allocates reactive node (~200 bytes), adds dependency tracking. Only worth it if savings > overhead.

### `create_selector()` - Equality-Based Updates

```rust
let x = create_signal(2);

// create_memo: Always notifies dependents when x changes
let squared_memo = create_memo(move || x.get() * x.get());

// create_selector: Only notifies if output changes
let squared_selector = create_selector(move || x.get() * x.get());

create_effect(move || {
    println!("Value: {}", squared_selector.get());
});

x.set(2);   // Effect runs (4 != previous)
x.set(-2);  // Effect does NOT run (4 == 4, despite x changing!)
```

**Use case**: Expensive downstream effects + output can be equal despite input changes.

**Examples**:
- Filtering/searching (different queries → same result set)
- Rounding/bucketing (41.2 → 40, 41.8 → 40)
- Validation status (multiple invalid states → all show "Invalid")

---

## Avoiding Unnecessary Reactivity

### Untracked Access

```rust
let trigger = create_signal(());
let config = create_signal(Configuration::default());

// Bad: Effect re-runs when config changes (unnecessary)
create_effect(move || {
    trigger.track();
    let cfg = config.get();  // Tracked!
    perform_action_with_config(cfg);
});

// Good: Only re-run when trigger changes
create_effect(move || {
    trigger.track();
    let cfg = config.get_untracked();  // Not tracked
    perform_action_with_config(cfg);
});
```

**Pattern**: Trigger signal for explicit control, untracked access for "props".

### `untrack()` Scope

```rust
use sycamore::reactive::untrack;

let a = create_signal(1);
let b = create_signal(2);

let result = create_memo(move || {
    let x = a.get();  // Tracked
    let y = untrack(|| b.get());  // Not tracked
    x + y
});

a.set(10);  // Memo updates
b.set(20);  // Memo does NOT update
```

**Use case**: Break circular dependencies, read "configuration" signals without subscribing.

---

## List Rendering Optimization

### Keyed vs Indexed

**`Indexed`**: Dumb, re-renders everything on any change.
```rust
let items = create_signal(vec![1, 2, 3]);

Indexed(
    list=items,
    view=|item| view! { li { (item) } }
)

// Adding 1 item → all 4 items re-render
```

**`Keyed`**: Smart, tracks by key, only updates changed items.
```rust
#[derive(Clone, PartialEq)]
struct Item { id: u32, text: String }

let items = create_signal(vec![
    Item { id: 1, text: "A".into() },
    Item { id: 2, text: "B".into() },
]);

Keyed(
    list=items,
    key=|item| item.id,  // Stable unique key
    view=|item| view! { li { (item.text) } }
)

// Adding 1 item → only new item renders
// Reordering → DOM nodes moved, not recreated
```

**Performance crossover** (rough heuristic):
- <5 items: `Indexed` fine
- 5-20 items: Depends on update frequency
- >20 items or frequent updates: Use `Keyed`

**Key function requirements**:
- Unique per item
- Stable across updates (DON'T use index, DO use database ID)

---

## Common Pitfalls

### 1. **Clone-Heavy Props**

```rust
// Anti-pattern: Clone entire Vec on every access
#[derive(Clone)]
struct Props {
    items: Vec<String>,  // Cloned when passed to component!
}

// Better: Wrap in Signal (Signal is Copy, cheap)
#[derive(Clone)]
struct Props {
    items: Signal<Vec<String>>,
}
```

### 2. **Accidental Tracking in Setup**

```rust
#[component]
fn MyComponent() -> View {
    let external = use_context::<Signal<i32>>();

    // Bug: This makes component re-execute on signal change?
    // NO! Component body is untracked (wrapped in component_scope)
    let initial = external.get();  // Safe, reads once

    view! {
        // Tracking happens HERE
        p { (move || external.get()) }
    }
}
```

Component bodies already untracked - no accidental reactivity.

### 3. **Forgetting `.call()` on Children**

```rust
#[derive(Props)]
struct CardProps {
    children: Children,
}

#[component]
fn Card(props: CardProps) -> View {
    // Bug: children is a closure, not a view
    view! {
        div { (props.children) }  // Type error!
    }
}

// Fix: Call to instantiate
#[component]
fn Card(props: CardProps) -> View {
    let children = props.children.call();
    view! {
        div { (children) }
    }
}
```

### 4. **Multiple `.call()` on Children**

```rust
// Bug: Children is FnOnce, can only call once
let children = props.children;
children.call();  // OK
children.call();  // Panic or compile error!

// Fix: Store result
let children = props.children.call();
// Use `children` multiple times
```

---

## Testing Strategies

### Unit Testing Reactive Logic

```rust
#[cfg(test)]
mod tests {
    use sycamore::reactive::*;

    #[test]
    fn signal_updates_memo() {
        create_root(|| {
            let count = create_signal(0);
            let doubled = create_memo(move || count.get() * 2);

            assert_eq!(doubled.get(), 0);

            count.set(5);
            assert_eq!(doubled.get(), 10);
        });
    }

    #[test]
    fn effect_runs_on_change() {
        create_root(|| {
            let count = create_signal(0);
            let effect_runs = create_signal(0);

            create_effect(move || {
                count.track();
                effect_runs.update(|n| *n += 1);
            });

            assert_eq!(effect_runs.get(), 1);  // Runs immediately

            count.set(1);
            assert_eq!(effect_runs.get(), 2);  // Runs on update
        });
    }
}
```

**Pattern**: Wrap in `create_root()` for scope management, use regular Rust assertions.

### Component Testing (WASM)

```rust
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn counter_increments() {
    let _ = create_root(|| {
        let count = create_signal(0);

        let node = view! {
            button(on:click=move |_| count += 1) {
                (count.get())
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        let button = query("button");
        assert_text_content!(button, "0");

        button.click();  // Trigger event
        assert_text_content!(button, "1");
    });
}

fn test_container() -> web_sys::Element {
    document()
        .query_selector("test-container")
        .unwrap()
        .unwrap()
}

fn query(selector: &str) -> web_sys::Element {
    document()
        .query_selector(selector)
        .unwrap()
        .unwrap()
}
```

**Setup**:
1. Use `wasm-bindgen-test` crate
2. Render to test container
3. Query DOM, assert state
4. Trigger events, assert updates

**Test macros** (from Sycamore test utils):
```rust
assert_text_content!(element, "expected text");
```

### Testing Strategy

**What to test**:
- Signal update logic (pure reactive code)
- Component behavior (user interactions → state changes)
- Conditional rendering (different code paths)
- List operations (add/remove/reorder)

**What NOT to test**:
- Sycamore internals (trust the library)
- Exact DOM structure (brittle, implementation detail)
- Styling/CSS (visual regression tools better)

**Mocking**: For context, provide test values:
```rust
#[wasm_bindgen_test]
fn uses_context() {
    create_root(|| {
        provide_context(MockAppState::default());

        // Component under test uses context
        let node = view! { MyComponent() };
        sycamore::render_in_scope(|| node, &test_container());

        // Assert behavior
    });
}
```

---

## Performance Debugging

### Identifying Unnecessary Updates

```rust
let count = create_signal(0);

create_effect(move || {
    console_log!("Effect running, count = {}", count.get());
});

// If effect logs more than expected, check:
// 1. Signal accessed in tracked scope?
// 2. Multiple signals triggering same effect?
// 3. Forgot untracked access?
```

### Profiling Signal Access

```rust
// Temporarily log all accesses
let value = create_signal(42);
create_effect(move || {
    console_log!("Signal accessed at: {}", js_sys::Date::now());
    value.track();
});

// Helps identify hot paths
```

---

## Open Questions (Practical Validation Needed)

**Performance**:
- Q: Actual memo allocation cost vs benefit crossover?
- Q: Batch overhead - when does it hurt vs help?
- Q: Keyed vs Indexed breakeven point (measure with real data)?

**Testing**:
- Q: Best practices for testing async workflows (suspense, resources)?
- Q: Integration testing strategy (multi-component interactions)?

---

## Sources

- **Local source**: `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`
  - `sycamore-reactive-0.9.2/src/root.rs` - batch() implementation
  - `sycamore-reactive-0.9.2/src/memos.rs` - create_selector vs create_memo
  - `sycamore-0.9.2/tests/web/*.rs` - Test patterns and examples
- **Insights**: From STUDY phase guidance + source code analysis

**Created**: 2025-11-03
**Sycamore version**: 0.9.2
