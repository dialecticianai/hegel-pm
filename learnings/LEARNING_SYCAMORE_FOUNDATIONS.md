# Sycamore Reactive Foundations

**Purpose**: Core reactive primitives and mental models for Sycamore 0.9.2

**Audience**: AI agents (concise, technical) + human-friendly

---

## Mental Model: Fine-Grained Reactivity

**Key insight**: No Virtual DOM. Direct, surgical DOM updates via dependency tracking.

**Flow**:
1. Create `Signal<T>` - reactive atom of state
2. Derive computations with `create_memo()` or side effects with `create_effect()`
3. Access signal → automatic tracking in reactive context
4. Update signal → only affected dependents re-execute

**Contrast with VDOM frameworks**:
- VDOM: Component re-renders → diff → patch DOM (coarse-grained)
- Sycamore: Signal changes → only subscribers update (fine-grained)

**Performance characteristic**: Updates are O(dependencies) not O(component tree).

---

## Signals: Reactive Atoms

### Creation & Ownership

```rust
let signal: Signal<i32> = create_signal(123);
```

**Memory model**:
- `Signal<T>` is a **handle/reference**, not the value itself
- Actual value stored in reactive node (managed by `Root`)
- Handle is `Copy` - can move into closures freely
- Cleanup automatic when scope/root disposed

**Key constraint**: Signals tied to reactive node lifecycle. No manual memory management needed.

### Reading Signals

**Four access patterns**:

| Method | When to use | Tracking? | Constraint |
|--------|-------------|-----------|------------|
| `.get()` | `T: Copy`, want tracking | Yes | Type must be `Copy` |
| `.get_clone()` | `T: Clone`, want tracking | Yes | Pays clone cost |
| `.with(|val| ...)` | Avoid clone, want tracking | Yes | Closure borrows `&T` |
| `.with_untracked(|val| ...)` | Need value, **no** tracking | No | Use when breaking reactivity chains |

**Gotcha**: Using `.get()` vs `.get_untracked()` changes reactive graph!

```rust
// Tracked - memo updates when signal changes
let memo = create_memo(move || signal.get() * 2);

// NOT tracked - memo never updates despite accessing signal
let memo = create_memo(move || signal.get_untracked() * 2);
```

**When to use untracked**: Breaking infinite update loops, accessing signals in effects without subscribing.

**Common gotcha - String signals**:
```rust
let text = create_signal(String::from("Hello"));

// ❌ Compile error: String doesn't implement Copy
let value = text.get();

// ✅ Correct: Use get_clone() for non-Copy types
let value = text.get_clone();

// ✅ Or use .with() to avoid cloning if just reading
text.with(|s| println!("{}", s));
```

This applies to any `T: Clone` but not `T: Copy` (Vec, HashMap, String, custom structs, etc.).

### Writing Signals

| Method | Use case | Triggers updates? |
|--------|----------|-------------------|
| `.set(value)` | Replace entire value | Yes |
| `.update(|val| ...)` | Mutate in place (efficient for non-Copy types) | Yes |
| `.replace(value)` | Set new value, return old | Yes |
| `.take()` | Get value, set to `Default` | Yes |
| `.set_fn(|&val| new_val)` | Compute new from old | Yes |
| `.set_silent(value)` | Replace WITHOUT triggering | **No** |
| `.update_silent(|val| ...)` | Mutate WITHOUT triggering | **No** |
| `.replace_silent(value)` | Replace, return old, no trigger | **No** |
| `.take_silent()` | Take to default, no trigger | **No** |

**Silent methods gotcha**: Using `_silent` variants creates **state inconsistencies** - memos/effects won't update!

```rust
let count = create_signal(0);
let doubled = create_memo(move || count.get() * 2);
assert_eq!(doubled.get(), 0);

count.set_silent(5); // Doubled NOT updated!
assert_eq!(doubled.get(), 0); // Stale - still 0, not 10!
```

**When to use silent methods**: Rare - batch initialization before subscribers exist, or manual control flow outside reactivity.

**Pattern**: Use `.update()` for `Vec`, `HashMap`, `String` to avoid unnecessary clones.

```rust
let list = create_signal(Vec::new());
// Bad: clones entire vec
list.set({
    let mut v = list.get_clone();
    v.push(item);
    v
});

// Good: mutates in place
list.update(|v| v.push(item));
```

**Arithmetic operators** (convenient syntax sugar):

```rust
let mut count = create_signal(0);
count += 1;  // Equivalent to: count.update(|c| *c += 1);
count -= 1;
count *= 2;
count /= 2;
count %= 10;
```

Note: Requires `mut` binding despite signal being `Copy` (Rust syntax requirement).

---

## Memos: Derived State

### `create_memo()` - Cached Derived Values

```rust
let count = create_signal(1);
let doubled = create_memo(move || count.get() * 2);
```

**Characteristics**:
- **Eager**: Runs immediately on creation and whenever dependencies change
- **Cached**: Result stored, not recomputed on every access
- **Tracked**: Accessing memo tracks it as dependency (memo is also a `ReadSignal`)

**Comparison with derived functions** (lazy derivations):

```rust
// Lazy - recomputes every call
let doubled_fn = || count.get() * 2;
let _ = doubled_fn(); // Compute
let _ = doubled_fn(); // Compute again (wasted work)

// Memo - caches result
let doubled_memo = create_memo(move || count.get() * 2);
let _ = doubled_memo.get(); // Cached
let _ = doubled_memo.get(); // Cached (no recomputation)
```

**Trade-off**:
- Memo: Slight overhead (reactive node allocation), but avoids redundant computation
- Function: Zero overhead, but recomputes every time

**Rule of thumb**: Use memos for expensive computations called multiple times. Use functions for trivial derivations (`|| x + 1`).

### `create_selector()` - Value-Equality Optimization

```rust
let x = create_signal(2);
let x_squared = create_selector(move || x.get() * x.get());

create_effect(move || println!("x^2 = {}", x_squared.get()));

x.set(2);   // Triggers effect (4)
x.set(-2);  // Does NOT trigger (still 4, value unchanged)
```

**Difference from `create_memo()`**:
- `create_memo()`: Always notifies dependents when dependencies change (even if output same)
- `create_selector()`: Only notifies if output value changes (uses `PartialEq`)

**When to use**: Expensive downstream effects + output can be equal despite different inputs.

**Variant**: `create_selector_with(f, eq_fn)` for custom equality (e.g., `f64` with epsilon comparison).

---

## Effects: Side Effects from Reactive State

### `create_effect()` - Subscribe to Signal Changes

```rust
let count = create_signal(0);

create_effect(move || {
    println!("Count: {}", count.get());
});
// Prints immediately: "Count: 0"

count.set(1);
// Prints: "Count: 1"
```

**Characteristics**:
- Runs **immediately** on creation
- Re-runs whenever tracked signals change
- Returns `()` (for side effects only)

**Critical rule**: Effects are for **side effects**, not state updates.

**Anti-pattern**:
```rust
// DON'T: Update signals inside effects (can cause loops)
create_effect(move || {
    other_signal.set(count.get() * 2); // Bad - use create_memo instead
});
```

**Correct pattern**:
```rust
// DO: Use memo for derived state
let doubled = create_memo(move || count.get() * 2);

// Effects for actual side effects (logging, DOM manipulation, fetch)
create_effect(move || {
    log(&format!("Value changed: {}", doubled.get()));
});
```

### Infinite Loop Protection

**Built-in safeguard**: Sycamore detects and prevents infinite loops:

```rust
create_effect(move || {
    state.track();
    state.set(0); // Would loop, but Sycamore prevents it
});
```

**How**: Effect queue prevents re-triggering same effect in same update cycle.

### `create_effect_initial()` - Two-Phase Effects

For effects that need different behavior on first run vs subsequent runs:

```rust
let initial_val = create_effect_initial(move || {
    // First run: initialization logic
    let setup_value = expensive_setup();

    (
        Box::new(move || {
            // Subsequent runs: update logic
            handle_updates();
        }),
        setup_value // Returned value
    )
});
```

**Use case**: WebSocket connections, event listeners, resource acquisition with cleanup.

---

## Scopes & Roots: Memory Management

### Reactive Root

**Mental model**: The arena allocating all reactive nodes.

```rust
let root = create_root(|| {
    // All signals/memos/effects here belong to this root
    let signal = create_signal(1);
    // ...
});
// Root disposed when dropped (all nodes cleaned up)
```

**Key facts**:
- One `Root` per app (usually leaked: `&'static Root`)
- Root manages `SlotMap<NodeId, ReactiveNode>` - all signals/memos/effects
- Disposing root = freeing all nodes at once

**Ownership trick**: Root is leaked (`Box::leak`) so signals get `'static` lifetime, but can still be manually disposed via `root.dispose()`.

### Child Scopes

```rust
let parent_signal = create_signal(0);

let child_handle = create_child_scope(|| {
    let child_signal = create_signal(10); // Owned by child scope
    // child_signal lifetime tied to child_handle
});

parent_signal.set(5); // Still alive
child_handle.dispose(); // child_signal freed
```

**Use case**: Component cleanup, conditional UI (mount/unmount), resource management.

**Hierarchy**: Child nodes added to parent's `children` vec. Disposing parent → disposes all children recursively.

---

## Dependency Tracking Mechanism

### How Tracking Works

**Tracked context** (inside memo/effect):
1. Root sets `tracker: Some(DependencyTracker)`
2. Signal access (`.get()`, `.with()`) adds signal ID to tracker
3. After closure executes, tracker creates bidirectional links:
   - Signal's `dependents` list ← add memo/effect ID
   - Memo/effect's `dependencies` list ← add signal ID

**Update propagation**:
1. Signal modified → mark signal's dependents as "dirty"
2. Batch update queue runs (topologically sorted to avoid redundant updates)
3. Each dirty node re-executes callback, rebuilds dependencies

**Gotcha**: Dependencies **dynamic** - recreated on every execution. Conditional signal access changes dependency graph!

```rust
let flag = create_signal(true);
let a = create_signal(1);
let b = create_signal(2);

let result = create_memo(move || {
    if flag.get() {
        a.get() // Depends on `a` when flag=true
    } else {
        b.get() // Depends on `b` when flag=false
    }
});
```

When `flag` changes, dependency graph rewires.

### Batching Updates

```rust
use sycamore::reactive::batch;

batch(|| {
    signal1.set(10);
    signal2.set(20);
    signal3.set(30);
});
// Effects run ONCE after batch, not 3 times
```

**Why batch**: Avoid intermediate states, reduce effect executions.

**Default behavior**: Without `batch()`, each `.set()` triggers effects immediately.

**Use case**: Multiple related state updates (e.g., form submission updating several fields).

---

## Common Patterns

### Read-Only Signals

```rust
let signal = create_signal(0);
let read_only: ReadSignal<i32> = *signal; // Deref to ReadSignal

// read_only.set(1); // Compile error!
read_only.get(); // OK
```

**Use case**: Pass signal to child component that shouldn't mutate it.

### Signal Mapping

```rust
let count = create_signal(5);
let doubled = count.map(|&n| n * 2); // Creates memo automatically
```

Shorthand for `create_memo(move || count.get() * 2)`.

### Signal Splitting (Reader/Writer Separation)

```rust
let signal = create_signal(0);
let (read, write) = signal.split();

// `read` is ReadSignal, `write` is Fn(T) -> T
assert_eq!(read.get(), 0);
write(5); // Sets and returns old value
assert_eq!(read.get(), 5);
```

**Use case**: Pass read-only access to child components, keep writer in parent.

### Reducer Pattern

```rust
enum Msg {
    Increment,
    Decrement,
}

let (state, dispatch) = create_reducer(0, |&state, msg: Msg| match msg {
    Msg::Increment => state + 1,
    Msg::Decrement => state - 1,
});

dispatch(Msg::Increment);
assert_eq!(state.get(), 1);
```

**Use case**: Complex state machines, Redux-style state management.

### Nightly Feature: Function Call Syntax

With `nightly` feature flag enabled:

```rust
let count = create_signal(0);

// Call to read
let val = count();  // Equivalent to count.get()

// Call with arg to write
let old = count(5); // Equivalent to count.replace(5)
```

**Ergonomics**: More concise, feels like accessing a value directly.

---

## Trait Implementations

Signals forward many standard traits to their inner values:

**Comparison traits** (`PartialEq`, `Eq`, `PartialOrd`, `Ord`):
```rust
let a = create_signal(5);
let b = create_signal(10);
assert!(a < b); // Compares inner values
```

**Hashing** (`Hash`):
```rust
use std::collections::HashSet;
let mut set = HashSet::new();
set.insert(signal); // Can use signals as map/set keys
```

**Formatting** (`Debug`, `Display`):
```rust
let count = create_signal(42);
println!("{}", count);    // Prints: 42
println!("{:?}", count);  // Prints: 42
```

**Serialization** (with `serde` feature):
```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct State {
    count: Signal<i32>, // Can serialize/deserialize signals
}
```

**Default**:
```rust
let signal: Signal<i32> = Signal::default(); // Creates signal with T::default()
```

**Key insight**: Signals are transparent for common operations - they "feel like" the underlying value.

---

## Gotchas & Edge Cases

### 1. **Dispose After Access Panics**

```rust
let signal = create_signal(1);
signal.dispose();
signal.get(); // Panic: "signal was disposed"
```

**Debug mode**: Panic message includes creation location.

**Prevention**: Check `.is_alive()` before accessing disposed signals (rare in practice).

### 2. **Accessing Signal While Updating Panics**

```rust
// Panic: "cannot update signal while reading"
signal.update(|val| {
    signal.set(10); // Can't mutate while borrowed mutably
});
```

**Fix**: Restructure logic to avoid nested updates.

### 3. **Untracked Access Doesn't Subscribe**

Easy to forget `.get_untracked()` means no reactivity:

```rust
let derived = create_memo(move || signal.get_untracked() * 2);
signal.set(10);
// derived still returns old value!
```

**Debugging**: If memo/effect not updating, check for untracked accesses.

### 4. **Dynamic Dependencies Can Surprise**

```rust
let cond = create_signal(true);
let a = create_signal(1);
let b = create_signal(2);

let result = create_memo(move || {
    if cond.get() { a.get() } else { b.get() }
});

// Initially depends on: cond, a
cond.set(false);
// Now depends on: cond, b (NOT a anymore)
```

Changing `a` won't trigger memo until `cond` flips back.

---

## Open Questions (Theory vs Practice)

**Performance**:
- Q: What's the overhead of signal creation vs plain Rust variables? (Benchmark needed)
- Q: How many signals/memos before observable slowdown? (Profiling needed)

**Memory**:
- Q: Does `SlotMap` fragmentation matter for long-running apps? (Load testing)
- Q: When do disposed nodes actually free memory (immediately vs GC-like)?

**Concurrency**:
- Q: Can signals be safely accessed from async tasks? (Feature: `suspense`)
- Q: How does batching interact with async updates?

These require **practical validation** in Discovery/Execution phases.

---

## Sources

- **Local source**: `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sycamore-reactive-0.9.2/src/`
  - `signals.rs` - Signal implementation, tracking, read/write methods
  - `memos.rs` - Memo and selector creation
  - `effects.rs` - Effect creation, infinite loop protection
  - `root.rs` - Root/scope management, dependency tracking mechanism
- **docs.rs**: `https://docs.rs/sycamore/latest/sycamore/reactive/`
  - `fn.create_signal.html` - Usage, reactivity, ownership model

**Created**: 2025-11-03
**Sycamore version**: 0.9.2
