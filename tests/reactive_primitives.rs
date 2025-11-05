#![cfg(target_arch = "wasm32")]

use sycamore::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[path = "common/mod.rs"]
mod common;
use common::*;

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
