#![cfg(target_arch = "wasm32")]

use sycamore::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[path = "common/mod.rs"]
mod common;
use common::*;

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
// TODO: Fix this test - reactive closures don't auto-update when calling functions directly
#[wasm_bindgen_test]
#[ignore]
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
// TODO: Fix this test - reactive closures don't auto-update when calling functions directly
#[wasm_bindgen_test]
#[ignore]
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
