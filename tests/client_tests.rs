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
