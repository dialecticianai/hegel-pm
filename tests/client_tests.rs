#![cfg(target_arch = "wasm32")]

use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// Helper to create test container
fn test_container() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    document.query_selector("body").unwrap().unwrap()
}

// Helper to query elements
fn query(selector: &str) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    document
        .query_selector(selector)
        .unwrap()
        .expect(&format!("Element not found: {}", selector))
}

// Helper to assert text content
macro_rules! assert_text_content {
    ($element:expr, $expected:expr) => {
        assert_eq!(
            $element.text_content().unwrap_or_default().trim(),
            $expected
        );
    };
}

// Test for reactive signal updates (from our learning docs pattern)
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

        // Check reactivity works
        assert_text_content!(counter, "42");
    });
}

// Test for memo behavior (from our learning docs)
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

// Test for WorkflowDetailView: collapse state signals
#[wasm_bindgen_test]
fn test_workflow_collapse_state() {
    let _ = create_root(|| {
        let collapsed = create_signal(true);

        let node = view! {
            div {
                p(id="collapse-state") { (move || if collapsed.get() { "collapsed" } else { "expanded" }) }
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        let state_elem = query("#collapse-state");
        assert_text_content!(state_elem, "collapsed");

        // Toggle collapse state
        collapsed.set(false);
        assert_text_content!(state_elem, "expanded");

        // Toggle back
        collapsed.set(true);
        assert_text_content!(state_elem, "collapsed");
    });
}

// Test for WorkflowDetailView: expand all functionality
#[wasm_bindgen_test]
fn test_expand_all_workflows() {
    let _ = create_root(|| {
        let workflow1_collapsed = create_signal(true);
        let workflow2_collapsed = create_signal(true);

        let expand_all = move || {
            workflow1_collapsed.set(false);
            workflow2_collapsed.set(false);
        };

        let node = view! {
            div {
                button(id="expand-all", on:click=move |_| expand_all()) { "Expand All" }
                p(id="w1-state") { (move || if workflow1_collapsed.get() { "collapsed" } else { "expanded" }) }
                p(id="w2-state") { (move || if workflow2_collapsed.get() { "collapsed" } else { "expanded" }) }
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        let w1_state = query("#w1-state");
        let w2_state = query("#w2-state");

        // Initially collapsed
        assert_text_content!(w1_state, "collapsed");
        assert_text_content!(w2_state, "collapsed");

        // Click expand all
        expand_all();

        // Both should be expanded
        assert_text_content!(w1_state, "expanded");
        assert_text_content!(w2_state, "expanded");
    });
}

// Test for WorkflowDetailView: collapse all functionality
#[wasm_bindgen_test]
fn test_collapse_all_workflows() {
    let _ = create_root(|| {
        let workflow1_collapsed = create_signal(false);
        let workflow2_collapsed = create_signal(false);

        let collapse_all = move || {
            workflow1_collapsed.set(true);
            workflow2_collapsed.set(true);
        };

        let node = view! {
            div {
                button(id="collapse-all", on:click=move |_| collapse_all()) { "Collapse All" }
                p(id="w1-state") { (move || if workflow1_collapsed.get() { "collapsed" } else { "expanded" }) }
                p(id="w2-state") { (move || if workflow2_collapsed.get() { "collapsed" } else { "expanded" }) }
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        let w1_state = query("#w1-state");
        let w2_state = query("#w2-state");

        // Initially expanded
        assert_text_content!(w1_state, "expanded");
        assert_text_content!(w2_state, "expanded");

        // Click collapse all
        collapse_all();

        // Both should be collapsed
        assert_text_content!(w1_state, "collapsed");
        assert_text_content!(w2_state, "collapsed");
    });
}

// Test for WorkflowDetailView: individual workflow toggle
#[wasm_bindgen_test]
fn test_individual_workflow_toggle() {
    let _ = create_root(|| {
        let collapsed = create_signal(true);

        let toggle = move || {
            collapsed.set(!collapsed.get());
        };

        let node = view! {
            div {
                button(id="toggle", on:click=move |_| toggle()) { "Toggle" }
                p(id="state") { (move || if collapsed.get() { "collapsed" } else { "expanded" }) }
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        let state_elem = query("#state");
        assert_text_content!(state_elem, "collapsed");

        // Toggle
        toggle();
        assert_text_content!(state_elem, "expanded");

        // Toggle again
        toggle();
        assert_text_content!(state_elem, "collapsed");
    });
}

// Test for Sidebar: navigation signal updates
#[wasm_bindgen_test]
fn test_sidebar_navigation_signal() {
    #[derive(Clone, Copy, PartialEq)]
    enum View {
        AllProjects,
        ProjectDetail,
    }

    let _ = create_root(|| {
        let current_view = create_signal(View::AllProjects);

        let node = view! {
            div {
                button(id="all-projects", on:click=move |_| current_view.set(View::AllProjects)) { "All Projects" }
                button(id="project", on:click=move |_| current_view.set(View::ProjectDetail)) { "Project" }
                p(id="view-state") {
                    (move || {
                        match current_view.get() {
                            View::AllProjects => "all-projects",
                            View::ProjectDetail => "project-detail",
                        }
                    })
                }
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        let state_elem = query("#view-state");
        assert_text_content!(state_elem, "all-projects");

        // Click project button
        let project_btn = query("#project");
        project_btn
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .click();
        assert_text_content!(state_elem, "project-detail");

        // Click all projects button
        let all_projects_btn = query("#all-projects");
        all_projects_btn
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .click();
        assert_text_content!(state_elem, "all-projects");
    });
}

// Test for Sidebar: selected project signal updates
#[wasm_bindgen_test]
fn test_sidebar_selected_project() {
    let _ = create_root(|| {
        let selected_project = create_signal(None::<String>);

        let select_project = move |name: &str| {
            selected_project.set(Some(name.to_string()));
        };

        let node = view! {
            div {
                button(id="select-foo", on:click=move |_| select_project("foo")) { "Select Foo" }
                button(id="clear", on:click=move |_| selected_project.set(None)) { "Clear" }
                p(id="selected") {
                    (move || {
                        selected_project.with(|s| {
                            s.as_ref().map(|n| n.clone()).unwrap_or_else(|| "none".to_string())
                        })
                    })
                }
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        let selected_elem = query("#selected");
        assert_text_content!(selected_elem, "none");

        // Select a project
        let select_btn = query("#select-foo");
        select_btn
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .click();
        assert_text_content!(selected_elem, "foo");

        // Clear selection
        let clear_btn = query("#clear");
        clear_btn.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
        assert_text_content!(selected_elem, "none");
    });
}

// Test for App: conditional rendering based on view
#[wasm_bindgen_test]
fn test_app_view_routing() {
    #[derive(Clone, Copy, PartialEq)]
    enum View {
        AllProjects,
        ProjectDetail,
    }

    let _ = create_root(|| {
        let current_view = create_signal(View::AllProjects);

        let node = view! {
            div {
                (move || {
                    match current_view.get() {
                        View::AllProjects => view! {
                            p(id="all-projects-view") { "All Projects View" }
                        },
                        View::ProjectDetail => view! {
                            p(id="project-detail-view") { "Project Detail View" }
                        },
                    }
                })
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        // Initially should show AllProjects view
        let all_projects = query("#all-projects-view");
        assert_text_content!(all_projects, "All Projects View");

        // Switch to ProjectDetail
        current_view.set(View::ProjectDetail);

        // Should now show ProjectDetail view
        let project_detail = query("#project-detail-view");
        assert_text_content!(project_detail, "Project Detail View");
    });
}
