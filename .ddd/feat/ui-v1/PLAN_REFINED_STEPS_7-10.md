# UI v1 Refined Plan: Steps 7-10

**Purpose**: Refine Steps 7-10 with idiomatic Sycamore patterns for maximum performance

**Key Learning Applied**: From `learnings/LEARNING_SYCAMORE_PRACTICES.md` lines 503-517:
- Use `move` closures in list view functions
- Copy Signal props into closures (Signals are Copy, cheap)
- Avoid HashMap<String, Signal<bool>> - wrong pattern
- Use component-level signals or derive collapse state from data

---

## Step 7: Workflow Detail View Component (REFINED)

### Architecture Decision: Collapse State Management

**Problem**: Original plan used `HashMap<String, Signal<bool>>` for collapse states, but:
1. HashMap stored in Signal is anti-pattern (not fine-grained)
2. Accessing map in closures causes lifetime issues
3. Not idiomatic Sycamore

**Solution**: Use **index-based collapse tracking** with Signal<Vec<bool>>

```rust
// Collapsed workflows: Signal<Vec<bool>> where index = workflow index in array
let workflows_collapsed = create_signal(Vec::<bool>::new());

// Collapsed phases: Signal<Vec<Vec<bool>>> where [workflow_idx][phase_idx]
let phases_collapsed = create_signal(Vec::<Vec<bool>>::new());
```

**Why this works**:
- Signals are Copy - can pass to closures freely
- Vec indices stable for session (no reordering workflows)
- Fine-grained updates: toggling one item doesn't rebuild HashMap
- Simpler than HashMap lookup

**Alternative Pattern**: Make workflow/phase items into sub-components with own state
- Each `WorkflowItem` component has its own `collapsed: Signal<bool>`
- Pro: Maximally isolated state
- Con: More complex, may be overkill for this use case
- Decision: Start with Vec<bool> approach, refactor to components if needed

### Step 7.a: Write Tests

Test file: `tests/client_tests.rs`

Write WASM tests for:
- Component accepts `selected_project: ReadSignal<Option<String>>` as prop
- ProjectInfo signal initializes to None
- Signal updates when project selected
- Collapse state Vec signals initialize correctly (all true = collapsed)
- Expand All sets all workflow collapse states to false
- Collapse All sets all workflow collapse states to true
- Individual workflow toggle updates correct index in Vec
- Individual phase toggle updates correct nested index

### Step 7.b: Implement - Component Structure

**File**: `src/client/components/workflow_detail_view.rs`

```rust
use gloo_net::http::Request;
use sycamore::prelude::*;
use sycamore::reactive::batch;
use crate::client::types::ProjectInfo;

#[derive(Props)]
pub struct WorkflowDetailViewProps {
    pub selected_project: ReadSignal<Option<String>>,
}

#[component]
pub fn WorkflowDetailView(props: WorkflowDetailViewProps) -> View {
    let selected_project = props.selected_project;

    // Data signals
    let project_info = create_signal(None::<ProjectInfo>);
    let loading = create_signal(false);
    let error = create_signal(false);

    // Collapse state signals (index-based)
    let workflows_collapsed = create_signal(Vec::<bool>::new());
    let phases_collapsed = create_signal(Vec::<Vec<bool>>::new());

    // Fetch effect (same as before)
    create_effect(move || {
        if let Some(name) = selected_project.with(|s| s.clone()) {
            // ... fetch logic ...
            // On success, initialize collapse states:
            batch(|| {
                let workflow_count = info.detail.workflows.len();
                workflows_collapsed.set(vec![true; workflow_count]);

                let phase_counts: Vec<Vec<bool>> = info.detail.workflows
                    .iter()
                    .map(|w| vec![true; w.phases.len()])
                    .collect();
                phases_collapsed.set(phase_counts);

                project_info.set(Some(info));
                loading.set(false);
            });
        }
    });

    // Expand/collapse all handlers
    let expand_all = move || {
        batch(|| {
            workflows_collapsed.update(|v| v.iter_mut().for_each(|c| *c = false));
            phases_collapsed.update(|v| {
                v.iter_mut().for_each(|workflow_phases| {
                    workflow_phases.iter_mut().for_each(|c| *c = false);
                });
            });
        });
    };

    let collapse_all = move || {
        workflows_collapsed.update(|v| v.iter_mut().for_each(|c| *c = true));
    };

    view! {
        // ... view implementation ...
    }
}
```

### Step 7.c: Implement - Rendering Workflows

**Pattern**: Use `Indexed` with `move` closure capturing signals

```rust
// In view! macro:
div(class="workflows-section") {
    (move || {
        project_info.with(|info_opt| {
            if let Some(info) = info_opt {
                view! {
                    Indexed(
                        list=create_signal(info.detail.workflows.clone()),
                        view=move |workflow_idx, workflow| {
                            // Copy signals into closure (cheap - signals are Copy)
                            let workflows_collapsed_sig = workflows_collapsed;
                            let phases_collapsed_sig = phases_collapsed;

                            // Clone data needed for view BEFORE view! macro
                            let mode = workflow.mode.clone();
                            let status = workflow.status.clone();
                            let workflow_id = workflow.workflow_id.clone();
                            let total_tokens = workflow.total_metrics.total_all_tokens;
                            let phases = workflow.phases.clone();

                            view! {
                                div(class="workflow-item") {
                                    // Header with toggle
                                    div(class="workflow-header", on:click=move |_| {
                                        workflows_collapsed_sig.update(|v| v[workflow_idx] = !v[workflow_idx]);
                                    }) {
                                        span(class="collapse-icon") {
                                            (move || {
                                                workflows_collapsed_sig.with(|v| {
                                                    if v[workflow_idx] { "▶" } else { "▼" }
                                                })
                                            })
                                        }
                                        span { (format!("{} ({})", mode, status)) }
                                    }

                                    // Conditional phases rendering
                                    (move || {
                                        workflows_collapsed_sig.with(|wc| {
                                            if !wc[workflow_idx] {
                                                // Render phases here
                                                render_phases(workflow_idx, phases.clone(), phases_collapsed_sig)
                                            } else {
                                                view! {}
                                            }
                                        })
                                    })
                                }
                            }
                        }
                    )
                }
            } else {
                view! {}
            }
        })
    })
}
```

**Key patterns**:
1. `move |workflow_idx, workflow|` - Indexed provides index automatically
2. Copy signals at start of closure (cheap, signals are Copy)
3. Clone data before view! macro
4. Access via index: `v[workflow_idx]`

### Step 7.d: Implement - Rendering Phases

**Helper function** (outside component):

```rust
fn render_phases(
    workflow_idx: usize,
    phases: Vec<PhaseSummary>,
    phases_collapsed: Signal<Vec<Vec<bool>>>,
) -> View {
    view! {
        Indexed(
            list=create_signal(phases),
            view=move |phase_idx, phase| {
                let phases_collapsed_sig = phases_collapsed;

                // Clone phase data before view
                let phase_name = phase.phase_name.clone();
                let phase_status = phase.status.clone();
                let duration = phase.duration_seconds;
                let metrics = phase.metrics.clone();

                view! {
                    div(class="phase-item") {
                        div(class="phase-header", on:click=move |_| {
                            phases_collapsed_sig.update(|v| {
                                v[workflow_idx][phase_idx] = !v[workflow_idx][phase_idx];
                            });
                        }) {
                            span(class="collapse-icon") {
                                (move || {
                                    phases_collapsed_sig.with(|v| {
                                        if v[workflow_idx][phase_idx] { "▶" } else { "▼" }
                                    })
                                })
                            }
                            span { (format!("{} ({})", phase_name, phase_status)) }
                        }

                        // Phase details (conditional)
                        (move || {
                            phases_collapsed_sig.with(|pc| {
                                if !pc[workflow_idx][phase_idx] {
                                    view! {
                                        div(class="phase-metrics") {
                                            p { "Duration: " (duration.to_string()) " seconds" }
                                            p { "Total Tokens: " (metrics.total_all_tokens.to_string()) }
                                            // ... more metrics ...
                                        }
                                    }
                                } else {
                                    view! {}
                                }
                            })
                        })
                    }
                }
            }
        )
    }
}
```

### Performance Considerations

**Why Indexed over Keyed**:
- Workflows don't reorder during session
- List size small (<50 workflows typically)
- Simpler than maintaining stable keys
- Per learnings: Indexed fine for <20 items

**Batching**:
- Use `batch()` for expand/collapse all (updates multiple Vec items)
- Use `batch()` for async data load (multiple state updates)

**Cloning**:
- Clone workflow/phase data once per render (when list changes)
- Signals copied freely (Copy trait, just handle)
- No cloning inside reactive closures

### Success Criteria

- Component compiles without E0525 (FnOnce) or E0521 (lifetime) errors
- Collapse states toggle correctly on click
- Expand All / Collapse All work
- No unnecessary clones in hot paths
- trunk build succeeds
- Tests verify signal updates

---

## Step 8: Sidebar Navigation (REFINED)

### Architecture Decision: Navigation State

**Pattern**: Use enum + Signal for type-safe navigation

```rust
#[derive(Clone, Copy, PartialEq)]
enum View {
    AllProjects,
    ProjectDetail,
}

let current_view = create_signal(View::AllProjects);
```

**Why**:
- Type-safe (can't typo view names)
- Signal is Copy (easy to pass around)
- Pattern matching for conditional rendering

### Step 8.a: Write Tests

Test file: `tests/client_tests.rs`

Write WASM tests for:
- Navigation signal initializes to AllProjects
- Clicking "All Projects" sets signal to AllProjects
- Clicking project name sets signal to ProjectDetail and updates selected_project
- Navigation state updates correctly

### Step 8.b: Implement

**File**: `src/client/components/sidebar.rs`

**Modify existing Sidebar component**:
- Accept `current_view: Signal<View>` as prop (or create in parent)
- Add "All Projects" link at top
- Add separator `<hr>` below it
- Wire click handlers to update `current_view` signal

**Pattern**:
```rust
#[derive(Props)]
pub struct SidebarProps {
    pub current_view: Signal<View>,
    pub selected_project: Signal<Option<String>>,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> View {
    let current_view = props.current_view;
    let selected_project = props.selected_project;

    // ... existing fetch logic ...

    view! {
        div(class="sidebar") {
            // All Projects link
            div(
                class="nav-item all-projects",
                on:click=move |_| {
                    current_view.set(View::AllProjects);
                    selected_project.set(None);
                }
            ) {
                "All Projects"
            }

            hr {}

            // Existing project list
            Keyed(
                list=projects,
                key=|p| p.name.clone(),
                view=move |project| {
                    let current_view_sig = current_view;
                    let selected_project_sig = selected_project;
                    let name = project.name.clone();

                    view! {
                        div(
                            class="nav-item project",
                            on:click=move |_| {
                                current_view_sig.set(View::ProjectDetail);
                                selected_project_sig.set(Some(name.clone()));
                            }
                        ) {
                            (project.name)
                        }
                    }
                }
            )
        }
    }
}
```

**Key patterns**:
- Signals copied into `move` closures (cheap)
- String cloned before closure (necessary for owned data)
- Both signals updated in click handler

### Success Criteria

- All Projects link appears at top
- Separator renders correctly
- Clicking updates navigation signal
- Tests verify signal changes
- trunk build succeeds

---

## Step 9: Main App Routing (REFINED)

### Step 9.a: Write Tests

Test file: `tests/client_tests.rs`

Write WASM tests for:
- App component initializes with AllProjects view
- Navigation signal controls which view renders
- Switching navigation updates displayed component

### Step 9.b: Implement

**File**: `src/client/mod.rs`

**Pattern**: Conditional rendering based on enum

```rust
#[component]
fn App() -> View {
    // Define View enum at module level or in types.rs
    let current_view = create_signal(View::AllProjects);
    let selected_project = create_signal(None::<String>);

    view! {
        div(class="app-container") {
            Sidebar(current_view=current_view, selected_project=selected_project)

            div(class="main-content") {
                (move || {
                    match current_view.get() {
                        View::AllProjects => view! {
                            AllProjectsView()
                        },
                        View::ProjectDetail => view! {
                            WorkflowDetailView(selected_project=selected_project.into())
                        },
                    }
                })
            }
        }
    }
}
```

**Key patterns**:
- Pattern matching on Signal<enum>
- `.into()` converts Signal to ReadSignal for props
- Closure re-executes when current_view changes (reactive)

### Success Criteria

- App component compiles
- View switching works correctly
- Tests verify conditional rendering
- trunk build succeeds

---

## Step 10: Integration and Manual Verification (UNCHANGED)

Same as original plan - run tests, build, start server, manual testing.

---

## Summary of Refinements

**Key Changes from Original Plan**:

1. **Collapse State**: HashMap → Vec<bool> (index-based, simpler)
2. **Navigation State**: String → Enum (type-safe)
3. **List Rendering**: Explicit `move` closures with signal copying
4. **Data Cloning**: Clone before view! macro, not inside closures
5. **Batching**: Explicit batch() for multi-signal updates

**Performance Wins**:
- No HashMap allocations/lookups
- Signals copied (cheap) not cloned (free)
- Fine-grained updates (only changed indices)
- Batched multi-updates (no intermediate renders)

**Idiomatic Sycamore**:
- Follows patterns from learnings docs
- Avoids lifetime errors (no borrowed data in closures)
- Uses Indexed appropriately (small, stable lists)
- Type-safe with enums

---

## Next Steps

1. Delete current broken `workflow_detail_view.rs`
2. Implement Step 7 with refined approach
3. Continue with Steps 8-10 as refined
4. Commit after each step completes
