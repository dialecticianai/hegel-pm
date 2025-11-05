// Common test utilities for WASM browser tests
// Usage: Include with `mod common; use common::*;` at top of test files

// Helper to create test container
pub fn test_container() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    document.query_selector("body").unwrap().unwrap()
}

// Helper to query elements
pub fn query(selector: &str) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    document
        .query_selector(selector)
        .unwrap()
        .expect(&format!("Element not found: {}", selector))
}

// Helper to assert text content
#[macro_export]
macro_rules! assert_text_content {
    ($element:expr, $expected:expr) => {
        assert_eq!(
            $element.text_content().unwrap_or_default().trim(),
            $expected
        );
    };
}
