use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

pub mod components;
mod types;

use components::{MetricsView, Sidebar};

#[component]
pub fn App() -> View {
    view! {
        div(class="app-container") {
            Sidebar {}
            MetricsView {}
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| view! { App {} });
}
