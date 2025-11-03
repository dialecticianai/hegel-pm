# Open Questions — Sycamore Best Practices for hegel-pm

**Created**: 2025-11-03
**Purpose**: Bridge Research → Discovery phases, validate theory through practice
**Status**: 8 open, 4 answered, 12 total

---

## Quick Summary

**Study complete**:
- Sycamore 0.9.2 reactive primitives, component patterns, performance/testing
- Three learning docs created (foundations, components, practices)
- Source code analysis complete

**Open questions by category**:
1. Performance & Optimization (3 open, 1 answered)
2. Application Architecture (2 open, 1 answered)
3. Testing Strategy (2 open, 1 answered)
4. Integration Patterns (1 open, 1 answered)

**Primary blockers**: None. All open questions are validation/measurement questions, not blockers.

**Next phase readiness**: ✅ Ready for Discovery. Open questions map cleanly to toy experiments.

---

## 1. Performance & Optimization

### Open Questions

**Q1.1**: What is the actual overhead of context lookup with deep nesting (10+ levels)?
- **Context**: Context API walks parent chain on every `use_context()` call. Source shows no caching/optimization.
- **Theory**: `learnings/LEARNING_SYCAMORE_COMPONENTS.md` - Context API mechanism
- **Hypothesis**: O(depth) linear scan, likely <1µs per lookup even at 10 levels
- **Answer via**: **Toy 1** - Build nested component tree (1, 5, 10, 15, 20 levels), benchmark `use_context()` calls, measure lookup time vs depth
- **Success criteria**: Measure time per lookup, identify if depth matters for hegel-pm (likely max 3-5 levels)

**Q1.2**: When does memoization overhead exceed function re-computation cost?
- **Context**: Memos allocate reactive nodes (~200 bytes) + dependency tracking. Only worth it if computation savings > overhead.
- **Theory**: `learnings/LEARNING_SYCAMORE_PRACTICES.md` - Memoization strategies
- **Hypothesis**: Breakeven at ~1ms computation (100 iterations of simple math vs 1 memo)
- **Answer via**: **Toy 2** - Create test cases with varying computation costs (1µs, 10µs, 100µs, 1ms), compare memo vs function performance with repeated access patterns
- **Success criteria**: Identify cost threshold where memo becomes beneficial, create decision guidelines for hegel-pm

**Q1.3**: What is the performance crossover point for Keyed vs Indexed lists?
- **Context**: `Indexed` re-renders all items on any change. `Keyed` tracks by key, only updates changed items.
- **Theory**: `learnings/LEARNING_SYCAMORE_PRACTICES.md` - List rendering optimization
- **Hypothesis**: Keyed wins at >20 items OR >1 update/sec
- **Answer via**: **Toy 3** - Test lists of 5, 10, 20, 50, 100 items with varying update patterns (add, remove, reorder, modify), measure render time for both approaches
- **Success criteria**: Data-driven decision for hegel-pm project/workflow lists (likely 10-50 workflows max)

### ✅ Answered Questions

**Q1.4**: Should we use `batch()` for multi-signal updates?
- ✅ **ANSWERED**: Yes, essential for async data loading with multiple state signals
  - Source: `learnings/LEARNING_SYCAMORE_PRACTICES.md` - Batching section, practical async example added
  - Validated in: `src/client/components.rs` Sidebar component (should batch `projects.set()` + `loading.set()`)
  - Rationale: Prevents intermediate inconsistent states (e.g., loading=false with stale data)
  - Pattern: Always batch related state updates in async callbacks
- **Applied**: Added practical async example to learning docs based on hegel-pm migration

---

## 2. Application Architecture

### Open Questions

**Q2.1**: How should we structure AppState context for hegel-pm?
- **Context**: Context is type-based (can't have multiple of same type in same scope). Need to decide between one monolithic context vs multiple small contexts.
- **Theory**: `learnings/LEARNING_SYCAMORE_COMPONENTS.md` - Context API patterns
- **Options**:
  - **Option A**: Single `AppState` struct with all global signals as fields
    - Pro: Single lookup, clear ownership
    - Con: Tight coupling, everything updates together
  - **Option B**: Multiple small contexts (`ProjectState`, `ThemeState`, `AuthState`)
    - Pro: Modular, components declare dependencies explicitly
    - Con: Multiple lookups, potential deep nesting, risk of type collisions
- **Answer via**: **Toy 4** - Prototype both approaches with hegel-pm's actual state needs (projects, workflows, metrics, theme), compare ergonomics and component clarity
- **Success criteria**: Choose approach based on: component simplicity, type safety, performance (if Q1.1 shows depth matters)

**Q2.2**: Which components should read context directly vs receive props?
- **Context**: Context makes dependencies implicit. Props make them explicit but can lead to prop drilling.
- **Theory**: `learnings/LEARNING_SYCAMORE_COMPONENTS.md` - State management patterns
- **Trade-off**: Explicitness (props) vs convenience (context)
- **Answer via**: **Toy 4** (same as Q2.1) - Try both patterns in prototype, evaluate readability
- **Success criteria**: Guideline: use context for truly global state (theme, auth), props for component-specific configuration

### ✅ Answered Questions

**Q2.3**: Should components be defined at module level or inline?
- ✅ **ANSWERED**: Module level only, never inline/nested
  - Source: `learnings/LEARNING_SYCAMORE_COMPONENTS.md` - Anti-pattern: nested components
  - Validated in: `src/client/components.rs`, `src/client/mod.rs` - All components at module level ✓
  - Rationale: Component functions must be stable, not recreated on every render
- **Status**: Code review complete - no nested components found in hegel-pm

---

## 3. Testing Strategy

### Open Questions

**Q3.1**: What should we test in hegel-pm UI components?
- **Context**: Limited testing examples found. Need to define scope (what vs what NOT to test).
- **Theory**: `learnings/LEARNING_SYCAMORE_PRACTICES.md` - Testing strategies section
- **Proposed scope**:
  - ✅ Test: Signal update logic, user interactions → state changes, conditional rendering paths
  - ❌ Don't test: Sycamore internals, exact DOM structure, styling
- **Answer via**: **Toy 5** - Write tests for sample hegel-pm component (e.g., project list), validate testing patterns work in practice
- **Success criteria**: Working test suite covering key behaviors, documented testing guidelines

**Q3.2**: How to test async workflows (if we add suspense/resources)?
- **Context**: No testing examples found for suspense/resources in study phase
- **Deferred**: Not needed for current hegel-pm UI (no async data loading yet)
- **Answer via**: When implementing async features, research Sycamore community examples or build experimental toy
- **Next step**: Defer until async features needed

### ✅ Answered Questions

**Q3.3**: What testing framework to use?
- ✅ **ANSWERED**: `wasm-bindgen-test` for component/integration tests, standard `#[test]` for reactive logic
  - Source: `learnings/LEARNING_SYCAMORE_PRACTICES.md` - Testing examples, `tests/web/*.rs` patterns
  - Rationale: `wasm-bindgen-test` integrates with browser environment, supports DOM queries and event triggering
- **Next step**: Set up `wasm-bindgen-test` in hegel-pm if not already configured

---

## 4. Integration Patterns

### Open Questions

**Q4.1**: Are there anti-patterns in current hegel-pm UI code?
- **Context**: Need to review existing UI against learned patterns
- **Theory**: All three learning docs - common pitfalls, anti-patterns sections
- **Check for**:
  - Nested component definitions
  - Excessive `.get()` calls where arithmetic operators could be used
  - Prop drilling where context would be better
  - Using `Indexed` where `Keyed` would perform better
  - Missing cleanup in effects with resources
- **Answer via**: **Discovery Phase Review** - Read `ui/src/app.rs` and `ui/src/components/`, cross-reference against learning docs, document findings
- **Success criteria**: List of specific improvements with references to learning docs

### ✅ Answered Questions

**Q4.2**: Can we use signals in HashMaps/sets as keys?
- ✅ **ANSWERED**: Yes, signals implement `Hash`, `Eq`, `PartialEq`
  - Source: `learnings/LEARNING_SYCAMORE_FOUNDATIONS.md` - Trait implementations section
  - Rationale: Signals forward standard traits to inner values, enabling use in collections
- **Next step**: Can use signal-based keys if needed for state management

---

## Next Steps to Answer These Questions

### Discovery Phase 1: Performance Validation (Answers Q1.1, Q1.2, Q1.3)

**Toy 1 - Context Depth Benchmark**:
1. Create nested component tree with configurable depth (1-20 levels)
2. Provide context at top, consume at bottom
3. Benchmark `use_context()` calls across varying depths
4. Measure: Time per lookup, memory overhead
5. Document: Findings in `learnings/.ddd/2_context_performance.md`

**Toy 2 - Memoization Breakeven Analysis**:
1. Create test computations with varying costs (1µs to 10ms)
2. Compare: Derived function vs `create_memo()` with repeated accesses (1x, 10x, 100x)
3. Measure: Execution time, memory allocation
4. Document: Decision guidelines in `learnings/.ddd/3_memoization_guide.md`

**Toy 3 - List Rendering Performance**:
1. Build test component with configurable list size (5-100 items)
2. Implement both `Indexed` and `Keyed` variants
3. Test operations: Add, remove, reorder, modify
4. Measure: Render time, re-render count
5. Document: Decision matrix for hegel-pm in `learnings/.ddd/4_list_rendering_perf.md`

### Discovery Phase 2: Application Design (Answers Q2.1, Q2.2)

**Toy 4 - AppState Architecture Prototype**:
1. Design `AppState` struct with hegel-pm's actual needs:
   - Projects: `Signal<Vec<Project>>`
   - Current workflow: `Signal<Option<WorkflowId>>`
   - Metrics: `Signal<Metrics>`
   - Theme: `Signal<Theme>`
2. Prototype Option A (monolithic context) vs Option B (multiple contexts)
3. Build sample components consuming state (both approaches)
4. Evaluate: Code clarity, type safety, refactoring ease
5. Document: Architecture decision in `learnings/.ddd/5_appstate_design.md`

### Discovery Phase 3: Testing & Code Review (Answers Q3.1, Q4.1)

**Toy 5 - Test Component Example**:
1. Pick representative hegel-pm component (e.g., project list)
2. Write `wasm-bindgen-test` tests covering:
   - Signal updates
   - User interactions (clicks, inputs)
   - Conditional rendering
3. Validate: Tests run, patterns are clear
4. Document: Testing guidelines in `learnings/.ddd/6_testing_guide.md`

**Code Review Activity**:
1. Read `ui/src/app.rs`, `ui/src/components/*`
2. Cross-reference against learning docs anti-patterns
3. Document findings with specific file:line references
4. Propose improvements with justifications
5. Document: Review in `learnings/.ddd/7_hegel_pm_ui_review.md`

---

## Prioritization

### Must Answer (Blockers for hegel-pm improvements)
- **Q4.1** - Code review (identify actual issues)
- **Q2.1** - AppState structure (foundational architecture decision)

### Should Answer (High value, not blocking)
- **Q3.1** - Testing strategy (improve code quality)
- **Q1.3** - List rendering (likely performance win for project/workflow lists)

### Nice to Answer (Validation, lower impact)
- **Q1.1** - Context depth (likely not a problem, but good to know)
- **Q1.2** - Memo breakeven (helps with future optimizations)
- **Q2.2** - Props vs context (can evolve organically)

### Deferred
- **Q3.2** - Async testing (wait until async features added)

---

## Status: ✅ Ready for Discovery Phase

**All blockers resolved**: No showstoppers. Can proceed to practical validation.

**Clear roadmap**: 5 focused toys + 1 code review activity map to all open questions.

**Expected duration**:
- Phase 1 (Performance): 1-2 sessions (3 toys, likely can run in parallel)
- Phase 2 (Architecture): 1 session (1 prototype toy)
- Phase 3 (Testing + Review): 1 session (1 test example + code review)
- **Total**: 3-4 sessions for complete Discovery phase

**Discovery phase deliverables**:
1. Performance measurement docs (context, memo, lists)
2. AppState architecture decision doc
3. Testing guidelines
4. hegel-pm UI code review with improvement recommendations

**Transition criteria**: After Discovery phase, return to Research mode if major unknowns discovered, otherwise transition to Execution mode (if user wants to apply learnings) or complete research cycle.

---

## Practical Insights (Post-Migration)

**Date**: 2025-11-03
**Activity**: Upgraded hegel-pm from Sycamore 0.8 → 0.9

### New Learnings Added to Docs

1. **`.get_clone()` for non-Copy types** (LEARNING_SYCAMORE_FOUNDATIONS.md)
   - Common gotcha: String, Vec, HashMap need `.get_clone()` not `.get()`
   - Alternative: `.with(|val| ...)` to avoid cloning

2. **Keyed list lifetime pattern** (LEARNING_SYCAMORE_COMPONENTS.md)
   - Must clone fields before using in view closure
   - Pattern: `let name = todo.name.clone(); view! { ... }`

3. **Batching async updates** (LEARNING_SYCAMORE_PRACTICES.md)
   - Essential for preventing UI flicker with multiple state updates
   - Real example from Sidebar component migration

### Migration Validation

✅ **Successful patterns applied**:
- Keyed lists for project rendering (efficient updates)
- Multiple signals for loading/error/data state
- Conditional rendering with proper View returns
- All components at module level (no nesting)

⚠️ **Improvement opportunities identified**:
- Should batch `projects.set()` + `loading.set()` in Sidebar
- Could add memoization for derived project counts/stats

### Status Update

**Migration complete**: ✅ hegel-pm now runs on Sycamore 0.9
**Build status**: ✅ `trunk build --release` succeeds
**Server status**: ✅ Server starts, UI loads

**Remaining work** (if pursuing Discovery phase):
- Performance toys (Q1.1, Q1.2, Q1.3) - optional
- AppState architecture prototype (Q2.1) - deferred
- WASM testing setup refinement (Q3.1) - tested patterns work, tooling needs work
