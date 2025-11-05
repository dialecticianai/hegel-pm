#![cfg(target_arch = "wasm32")]

use sycamore::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[path = "common/mod.rs"]
mod common;
use common::*;

// This test reproduces the "signal was disposed" panic when a component
// with an async effect is unmounted before the async operation completes
//
// This mimics the real scenario:
// 1. AllProjectsView mounts, starts async fetch
// 2. Sidebar auto-selects first project, switches view immediately
// 3. AllProjectsView unmounts (signals disposed)
// 4. AllProjectsView's async callback fires, tries to update disposed signals → PANIC
#[wasm_bindgen_test]
fn test_async_effect_with_immediate_unmount() {
    let _ = create_root(|| {
        // Simulate the App's current_view signal
        let show_view1 = create_signal(true);

        // Simplified: just track if we can still update signals
        // In real app, signals are created in component scope and disposed on unmount
        let render_view = move || {
            if show_view1.get() {
                // View 1: creates signals in component scope, starts async work
                let data = create_signal(None::<String>);
                let loading = create_signal(true);

                // This effect runs immediately on mount
                create_effect(move || {
                    sycamore::futures::spawn_local(async move {
                        // Simulate slow async operation
                        // In real app, this is the fetch to /api/all-projects
                        gloo_net::http::Request::get("/api/all-projects")
                            .send()
                            .await
                            .ok();

                        // When this line runs, if component was unmounted,
                        // signals are disposed → panic: "signal was disposed"
                        data.set(Some("loaded".to_string()));
                        loading.set(false);
                    });
                });

                view! {
                    div(id="view1") { "View 1" }
                }
            } else {
                view! {
                    div(id="view2") { "View 2" }
                }
            }
        };

        let node = view! {
            div {
                (render_view)
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        // View1 is showing, async fetch started
        let _view1 = query("#view1");

        // Immediately switch views (before async completes)
        // This mimics Sidebar auto-selecting and switching view on load
        show_view1.set(false);

        // View2 now showing, View1 unmounted, signals disposed
        let _view2 = query("#view2");

        // The async callback will eventually fire and try to update disposed signals
        // Test framework might not catch the async panic, but this reproduces the scenario
    });
}
