# Sycamore Component Patterns

**Purpose**: Component architecture, composition, state management, and lifecycle patterns

**Audience**: AI agents (concise, technical) + human-friendly

---

## Mental Model: Components as Factory Functions

**Key insight**: Sycamore components are functions that create views, NOT persistent objects.

```rust
#[component]
fn Counter(initial: i32) -> View {
    let count = create_signal(initial);
    view! {
        button(on:click=move |_| count += 1) {
            "Count: " (count.get())
        }
    }
}
```

**Component execution**:
1. Function called once per instantiation
2. Creates reactive primitives (signals/memos/effects)
3. Returns `View` (tree of nodes)
4. Reactive primitives outlive function scope (owned by reactive node hierarchy)

**Contrast with class-based frameworks**: No `this`, no instance methods, no lifecycle hooks (except `on_mount`).

---

## Component Definition

### Basic Component (No Props)

```rust
#[component]
fn Hello() -> View {
    view! {
        p { "Hello World!" }
    }
}

// Usage
view! {
    Hello()
}
```

**Behind the scenes**: `#[component]` macro wraps function to work with Sycamore's component system.

### Component with Props

```rust
#[derive(Props)]
struct ButtonProps {
    color: String,
    disabled: bool,
}

#[component]
fn Button(props: ButtonProps) -> View {
    view! {
        button(disabled=props.disabled, style:color=props.color) {
            "Click me"
        }
    }
}

// Usage
view! {
    Button(color="red".to_string(), disabled=false)
}
```

**Props trait**: Derived automatically, generates builder pattern for construction.

**Builder usage** (rarely needed manually):
```rust
let props = ButtonProps::builder()
    .color("blue".to_string())
    .disabled(true)
    .build();
```

---

## The View Type

### Structure

```rust
pub struct View<T = HtmlNode> {
    nodes: SmallVec<[T; 1]>,  // Optimized for single-node views
}
```

**Key facts**:
- View is a **list** of nodes, not a single node
- `SmallVec<[T; 1]>` - stack-allocated for 1 node, heap for multiple (performance optimization)
- Can be empty (`View::new()`), single node, or multiple nodes

### Conversions to View

**Strings/numbers**:
```rust
// All become text nodes
view! {
    p { "Static string" }         // &'static str
    p { (string_var) }            // String
    p { (42) }                    // i32, f64, etc.
}
```

**Signals** (automatic reactivity):
```rust
let count = create_signal(0);
view! {
    p { (count.get()) }  // Updates when count changes
}
```

**Closures** (dynamic views):
```rust
let flag = create_signal(true);
view! {
    p {
        (move || if flag.get() { "On" } else { "Off" })
    }
}
```

**Tuples** (multiple views):
```rust
view! {
    div {
        (view! { p { "First" } }, view! { p { "Second" } })
    }
}
```

**Option/Vec**:
```rust
let maybe_view: Option<View> = Some(view! { p { "Hello" } });
view! {
    div { (View::from(maybe_view)) }
}
```

---

## Props and Children

### Children Pattern

```rust
#[derive(Props)]
struct CardProps {
    title: String,
    children: Children,  // Special prop type
}

#[component]
fn Card(props: CardProps) -> View {
    let children = props.children.call();  // Instantiate children
    view! {
        div(class="card") {
            h2 { (props.title) }
            div(class="card-body") {
                (children)
            }
        }
    }
}

// Usage
view! {
    Card(title="My Card".to_string()) {
        p { "This is the card content." }
        button { "Action" }
    }
}
```

**`Children` type**:
- Wraps `Box<dyn FnOnce() -> View>`
- Lazy - not instantiated until `.call()`
- Created automatically from view! macro content

**When to call**:
- Call `.call()` exactly once (consumes the closure)
- Store result if needed multiple times

---

## State Management

### Local Component State

```rust
#[component]
fn TodoList() -> View {
    let todos = create_signal(vec!["Buy milk".to_string()]);
    let new_todo = create_signal(String::new());

    let add_todo = move |_| {
        todos.update(|list| list.push(new_todo.get_clone()));
        new_todo.set(String::new());
    };

    view! {
        input(bind:value=new_todo)
        button(on:click=add_todo) { "Add" }
        ul {
            Indexed(
                list=todos,
                view=|item| view! { li { (item) } }
            )
        }
    }
}
```

**Scope**: Signals live until component scope disposed (component unmounted).

### Shared State via Context

**Provide context** (parent):
```rust
#[derive(Clone)]
struct AppState {
    user: Signal<Option<String>>,
    theme: Signal<String>,
}

#[component]
fn App() -> View {
    let state = AppState {
        user: create_signal(None),
        theme: create_signal("light".to_string()),
    };

    provide_context(state);

    view! {
        Header()
        Main()
    }
}
```

**Consume context** (child):
```rust
#[component]
fn Header() -> View {
    let state: AppState = use_context();

    view! {
        header {
            "Theme: " (state.theme.get())
        }
    }
}
```

**Context API**:
- `provide_context(value)` - Provide value in current scope
- `use_context::<T>()` - Get value from current or ancestor scope (panics if not found)
- `try_use_context::<T>()` - Returns `Option<T>` (safe variant)
- `use_context_or_else(|| default)` - Get or create + provide

**Scope walking**: Context lookup walks up parent chain until found or root reached.

**Type constraint**: `T: Clone + 'static` - context values must be cloneable.

**Pattern**: Store Signals in context (cheap to clone, single source of truth):
```rust
// Good - signals are Copy
#[derive(Clone)]
struct State {
    count: Signal<i32>,
}

// Bad - cloning duplicates state
#[derive(Clone)]
struct State {
    count: i32,  // Each clone has independent count!
}
```

### When to Lift State

**Keep local** when:
- State used only within component
- No sibling communication needed
- Component self-contained

**Lift to context** when:
- Multiple distant components need access
- Cross-cutting concerns (theme, auth, i18n)
- Avoiding prop drilling through many layers

**Anti-pattern**: Over-using context for everything (makes dependencies implicit).

---

## Event Handling

### Basic Events

```rust
#[component]
fn Button() -> View {
    let count = create_signal(0);

    view! {
        button(on:click=move |_event| {
            count += 1;
        }) {
            "Clicked " (count.get()) " times"
        }
    }
}
```

**Event types**: `on:click`, `on:input`, `on:submit`, `on:keydown`, etc.

**Event object**: Closure receives `web_sys::Event` (or specific subtype).

### Capturing Signal Values

**Correct** (signal captured in closure):
```rust
let count = create_signal(0);
view! {
    button(on:click=move |_| count += 1) {
        (count.get())  // Reactive
    }
}
```

Signal is `Copy`, so `move` just copies the handle (not the value).

**Gotcha** - accessing current value in handler:
```rust
// Wrong - reads value at closure creation time (stale)
let count = create_signal(0);
let value = count.get();  // Reads 0
view! {
    button(on:click=move |_| {
        println!("{}", value);  // Always prints 0!
    })
}

// Right - read inside closure
let count = create_signal(0);
view! {
    button(on:click=move |_| {
        println!("{}", count.get());  // Reads current value
    })
}
```

---

## Conditional Rendering

### With Closures

```rust
let show = create_signal(true);
view! {
    (move || {
        if show.get() {
            view! { p { "Visible" } }
        } else {
            view! { p { "Hidden" } }
        }
    })
}
```

**Reactivity**: Closure re-executes when `show` changes, rebuilding view.

### Show/Hide Pattern

```rust
let show = create_signal(false);
view! {
    div(style:display=move || if show.get() { "block" } else { "none" }) {
        p { "Content" }
    }
}
```

**Trade-off**: DOM node stays in tree (just hidden), vs conditional which adds/removes node.

---

## List Rendering

### `Indexed` - By Index

```rust
let items = create_signal(vec!["A".to_string(), "B".to_string()]);

view! {
    ul {
        Indexed(
            list=items,
            view=|item| view! {
                li { (item) }
            }
        )
    }
}
```

**Behavior**: Re-renders on any list change. Simple but potentially inefficient.

### `Keyed` - By Key Function

```rust
#[derive(Clone, PartialEq)]
struct Todo {
    id: u32,
    text: String,
}

let todos = create_signal(vec![
    Todo { id: 1, text: "First".into() },
    Todo { id: 2, text: "Second".into() },
]);

view! {
    ul {
        Keyed(
            list=todos,
            key=|todo| todo.id,  // Stable key
            view=|todo| view! {
                li { (todo.text) }
            }
        )
    }
}
```

**Behavior**: Tracks items by key, only re-renders changed items. More efficient for large/dynamic lists.

**When to use**:
- `Indexed`: Small lists (<10 items), items rarely change
- `Keyed`: Large lists, frequent updates, reorderable items

**Lifetime gotcha with Keyed**:

```rust
// ❌ Lifetime error - trying to borrow from `todo`
Keyed(
    list=todos,
    key=|t| t.id,
    view=|todo| view! {
        div { (todo.text) }  // Borrow doesn't live long enough
    }
)

// ✅ Correct - clone values needed for view
Keyed(
    list=todos,
    key=|t| t.id,
    view=|todo| {
        let text = todo.text.clone();  // Clone before view!
        view! {
            div { (text) }
        }
    }
)
```

The `view` closure parameter is consumed by value, but the view needs owned data. Always clone fields before using in view.

---

## Component Lifecycle

### Mount Hook

```rust
#[component]
fn DataFetcher() -> View {
    let data = create_signal(None);

    on_mount(move || {
        // Runs after component rendered to DOM
        fetch_data().then(move |result| {
            data.set(Some(result));
        });
    });

    view! {
        (move || match data.get() {
            Some(d) => view! { p { (d) } },
            None => view! { p { "Loading..." } },
        })
    }
}
```

**`on_mount(callback)`**:
- Runs **after** view attached to DOM
- Only in browser (no-op in SSR)
- Use for: DOM queries, focus management, third-party lib initialization

**Timing**: Queued as microtask, executes after current render.

### Cleanup

```rust
#[component]
fn Timer() -> View {
    on_mount(|| {
        let interval_id = set_interval(|| {
            console_log!("Tick");
        }, 1000);

        on_cleanup(move || {
            clear_interval(interval_id);  // Clean up on unmount
        });
    });

    view! {
        p { "Timer running..." }
    }
}
```

**`on_cleanup(callback)`**:
- Registered via `sycamore_reactive::on_cleanup`
- Runs when scope disposed (component unmounted, memo/effect re-executed)
- Use for: Timers, event listeners, subscriptions, manual resource cleanup

**Automatic cleanup**: Signals, memos, effects cleaned up automatically by scope system.

---

## Component Scope

### Untracked Execution

```rust
#[component]
fn MyComponent() -> View {
    // Component body runs in untracked scope
    let external_signal = use_context::<Signal<i32>>();

    // Accessing signal here does NOT make component reactive
    let initial = external_signal.get();  // Read once at creation

    view! {
        // Reactivity happens inside view! closures
        p { (move || external_signal.get()) }
    }
}
```

**Key insight**: `component_scope(f)` wraps function in `untrack(f)`.

**Why**: Prevents component from re-executing when signals accessed during setup. Reactivity explicit via closures in view!.

---

## SSR vs Client Rendering

### Conditional Compilation

```rust
#[component]
fn DataComponent() -> View {
    if is_ssr!() {
        // Server-side: fetch synchronously, embed in HTML
        let data = fetch_data_sync();
        view! { p { (data) } }
    } else {
        // Client-side: fetch asynchronously
        let data = create_signal(None);
        on_mount(move || {
            fetch_data().then(move |d| data.set(Some(d)));
        });
        view! { (move || render_data(data.get())) }
    }
}
```

**Macros**:
- `is_ssr!()` - Returns `true` on server, `false` on client
- `is_not_ssr!()` - Inverse

**Node types**:
- `wasm32` target: `DomNode` (or `HydrateNode` with `hydrate` feature)
- Non-wasm32: `SsrNode`
- Type alias: `HtmlNode` selects appropriate type

### Client-Only Effects

```rust
create_client_effect(move || {
    // Only runs on client, not during SSR
    console_log!("User interacted");
});
```

Equivalent to:
```rust
if is_not_ssr!() {
    create_effect(move || { /* ... */ });
}
```

---

## Patterns & Best Practices

### Derived Props

```rust
#[derive(Props)]
struct UserCardProps {
    user: Signal<User>,
}

#[component]
fn UserCard(props: UserCardProps) -> View {
    // Derive specific fields as memos
    let name = create_memo(move || props.user.with(|u| u.name.clone()));
    let avatar_url = create_memo(move || props.user.with(|u| u.avatar_url.clone()));

    view! {
        div {
            img(src=avatar_url.get())
            p { (name.get()) }
        }
    }
}
```

**Why**: Memoizes derived values, avoids recomputing on every access.

### Composition Over Props Sprawl

```rust
// Bad - too many props
#[derive(Props)]
struct FormProps {
    title: String,
    submit_text: String,
    cancel_text: String,
    on_submit: Box<dyn Fn()>,
    // ... 10 more props
}

// Good - compose smaller components
#[component]
fn Form(props: FormProps) -> View {
    view! {
        FormHeader(title=props.title)
        FormBody { (props.children.call()) }
        FormFooter(on_submit=props.on_submit)
    }
}
```

### Avoid Nested Components

```rust
// Anti-pattern - inner component recreated on every render
#[component]
fn Outer() -> View {
    let state = create_signal(0);

    #[component]
    fn Inner() -> View {  // DON'T - closure captures happen unexpectedly
        view! { p { "Inner" } }
    }

    view! { Inner() }
}

// Correct - define components at module level
#[component]
fn Inner() -> View {
    view! { p { "Inner" } }
}

#[component]
fn Outer() -> View {
    view! { Inner() }
}
```

**Why**: Component functions should be stable, not recreated on every render.

---

## Open Questions (Theory vs Practice)

**Performance**:
- Q: What's the overhead of component instantiation vs inline view! blocks?
- Q: How many components before perf degrades?

**Context**:
- Q: Performance impact of deep context lookups (10+ levels)?
- Q: Best practices for organizing context (one big context vs many small ones)?

**Lists**:
- Q: Crossover point where Keyed beats Indexed (list size, update frequency)?

These require practical validation.

---

## Sources

- **Local source**: `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`
  - `sycamore-core-0.9.2/src/component.rs` - Component trait, Props, Children
  - `sycamore-reactive-0.9.2/src/context.rs` - Context API implementation
  - `sycamore-web-0.9.2/src/lib.rs` - SSR detection, on_mount
  - `sycamore-web-0.9.2/src/view.rs` - View type, conversions
- **docs.rs**: `https://docs.rs/sycamore/latest/sycamore/`

**Created**: 2025-11-03
**Sycamore version**: 0.9.2
