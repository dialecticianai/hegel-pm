use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

pub mod components;
mod types;

use components::{MetricsView, Sidebar};

#[component]
pub fn App() -> View {
    // Shared state: selected project name
    let selected_project = create_signal(None::<String>);

    view! {
        div(class="app-container") {
            Sidebar(selected_project=selected_project)
            MetricsView(selected_project=*selected_project)
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| view! { App {} });
}
