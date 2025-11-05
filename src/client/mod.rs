use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

pub mod components;
mod types;

use components::{AllProjectsView, Sidebar, WorkflowDetailView};
use types::View as ViewEnum;

#[component]
pub fn App() -> View {
    // Shared state: current view and selected project
    let current_view = create_signal(ViewEnum::AllProjects);
    let selected_project = create_signal(None::<String>);

    view! {
        div(class="app-container") {
            Sidebar(current_view=current_view, selected_project=selected_project)

            div(class="main-content") {
                // Keep both views mounted, just toggle visibility with class
                div(class=move || if current_view.get() == ViewEnum::AllProjects { "view-visible" } else { "view-hidden" }) {
                    AllProjectsView()
                }
                div(class=move || if current_view.get() == ViewEnum::ProjectDetail { "view-visible" } else { "view-hidden" }) {
                    WorkflowDetailView(selected_project=*selected_project, is_visible=create_memo(move || current_view.get() == ViewEnum::ProjectDetail))
                }
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| view! { App {} });
}
