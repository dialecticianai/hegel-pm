# Research Plan - Sycamore Best Practices

**Goal**: Build foundational knowledge of Sycamore reactive patterns and best practices for hegel-pm UI development

**Success criteria**:
- [ ] Priorities 0-2 complete with learning docs
- [ ] Key patterns validated with test examples in our codebase
- [ ] Identified specific patterns to apply/improve in hegel-pm UI
- [ ] Open questions catalogued for practical validation

---

## Documentation Strategy

**Primary sources** (in priority order):
1. **Local source code**: `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sycamore-*/src/`
2. **docs.rs via w3m**: `https://docs.rs/sycamore/latest/sycamore/`
3. **GitHub examples**: `https://raw.githubusercontent.com/sycamore-rs/sycamore/main/examples/`

**Key local source modules discovered**:
- `sycamore-0.9.2/src/lib.rs` - Main entry point, feature flags, prelude
- `sycamore-reactive-0.9.2/src/` - Core reactive primitives:
  - `signals.rs` - Signal implementation
  - `memos.rs` - Memoized computations
  - `effects.rs` - Side effects
  - `context.rs` - Context API
  - `root.rs` - Reactive scopes/roots
- `sycamore-web-0.9.2/src/` - Web-specific rendering

---

## Priority 0: Foundational Reactive Primitives (MUST KNOW)

**Target**: 0.5-1 session

### Topics:
1. **Signal mechanics**
   - Creation: `create_signal()`
   - Reading vs tracking
   - Mutation patterns
   - Memory/cleanup model
   - Source: `sycamore-reactive-0.9.2/src/signals.rs`
   - Docs: `https://docs.rs/sycamore/latest/sycamore/reactive/struct.Signal.html`

2. **Memo vs Effect**
   - `create_memo()` - Derived reactive values
   - `create_effect()` - Side effects from signals
   - When to use which
   - Source: `sycamore-reactive-0.9.2/src/memos.rs`, `effects.rs`
   - Docs: `https://docs.rs/sycamore/latest/sycamore/reactive/fn.create_memo.html`

3. **Reactive scope/root model**
   - `create_root()` vs `create_child_scope()`
   - Ownership and cleanup
   - When scopes are necessary
   - Source: `sycamore-reactive-0.9.2/src/root.rs`
   - Docs: `https://docs.rs/sycamore/latest/sycamore/reactive/fn.create_root.html`

4. **View DSL basics**
   - `view!` macro usage
   - Component definition with `#[component]`
   - Props pattern
   - Source: `sycamore-0.9.2/src/lib.rs` (re-exports from sycamore_macro)
   - Docs: `https://docs.rs/sycamore/latest/sycamore/macro.view.html`

### Deliverable:
- `LEARNING_SYCAMORE_FOUNDATIONS.md` - Synthesized understanding of reactive primitives

---

## Priority 1: Component Patterns (CORE)

**Target**: 0.5 session

### Topics:
1. **Component composition**
   - Props macro usage and patterns
   - Children handling
   - Component lifecycle (mount, cleanup)
   - Source: `sycamore-0.9.2/src/prelude` imports
   - Docs: `https://docs.rs/sycamore/latest/sycamore/macro.Props.html`

2. **State management patterns**
   - Local component state
   - Shared state via Context API
   - `create_context()` / `use_context()`
   - When to lift state vs keep local
   - Source: `sycamore-reactive-0.9.2/src/context.rs`
   - Docs: `https://docs.rs/sycamore/latest/sycamore/reactive/fn.create_context.html`

3. **Event handling**
   - `on:click`, `on:input` patterns
   - Closure captures and Signal updates
   - Avoiding unnecessary re-renders
   - Examples: Check GitHub examples/counter, examples/todomvc

4. **Conditional rendering and lists**
   - `Keyed` vs `Indexed` components
   - Efficient list updates
   - Show/hide patterns
   - Source: Look for `Keyed`/`Indexed` in sycamore-web
   - Docs: `https://docs.rs/sycamore/latest/sycamore/web/struct.Keyed.html`

### Deliverable:
- `LEARNING_SYCAMORE_COMPONENTS.md` - Component patterns and best practices
- Small test component in hegel-pm demonstrating patterns

---

## Priority 2: Performance & Testing (PRACTICAL)

**Target**: 0.5 session

### Topics:
1. **Performance considerations**
   - Minimizing reactivity overhead
   - Batch updates with `batch()`
   - Selector patterns for derived state
   - When to use `create_selector()` vs `create_memo()`
   - Source: `sycamore-reactive-0.9.2/src/` (batch, selector functions)
   - Docs: `https://docs.rs/sycamore/latest/sycamore/reactive/fn.batch.html`

2. **Testing strategies**
   - Unit testing reactive components
   - Testing signal updates
   - Mocking/isolation patterns
   - Check: `sycamore-0.9.2/tests/` directory for examples
   - May need to fetch from GitHub: `https://raw.githubusercontent.com/sycamore-rs/sycamore/main/packages/sycamore/tests/`

3. **Common pitfalls**
   - Signal ownership issues
   - Memory leaks from uncleaned effects
   - Infinite update loops
   - Clone vs reference patterns
   - Source: Read through source comments and docs warnings

### Deliverable:
- `LEARNING_SYCAMORE_PRACTICES.md` - Performance tips, testing approach, common mistakes
- Test examples demonstrating testing patterns

---

## Priority 3: Advanced Features (DEFER until needed)

**Not blocking for initial research cycle**

- Async resources and suspense (feature flag: `suspense`)
- Server-side rendering (SSR)
- Routing patterns
- Complex animation/transitions
- WASM interop patterns

These will be studied on-demand when implementing specific features.

---

## Application to hegel-pm

**During ASSESS phase**, specifically identify:
1. Current reactive patterns in our UI that could be improved
2. State management approach (local vs context)
3. Component structure opportunities
4. Testing gaps

**Files to review against learned patterns**:
- `ui/src/app.rs` - Main app structure
- `ui/src/components/` - Existing components
- Any signal/state usage patterns

---

## Time Budget

- **Priority 0**: 1 session (foundational, most critical)
- **Priority 1**: 0.5 session (practical patterns)
- **Priority 2**: 0.5 session (optimization & testing)
- **Total**: ~2 sessions

**Exit criteria**: Can confidently refactor/extend hegel-pm UI with Sycamore best practices, know where to look for answers, understand testing approach.

---

## Notes

- Sycamore v0.9.2 is current version in use
- Inspired by SolidJS - similar fine-grained reactivity model
- No VDOM - direct DOM updates via reactivity
- Type-checked view DSL at compile time
- Feature flags affect API surface (we use: `web`, `wasm-bindgen-interning`)
