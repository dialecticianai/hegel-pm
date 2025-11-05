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
                (move || {
                    match current_view.get() {
                        ViewEnum::AllProjects => view! {
                            AllProjectsView()
                        },
                        ViewEnum::ProjectDetail => view! {
                            WorkflowDetailView(selected_project=*selected_project)
                        },
                    }
                })
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| view! { App {} });
}
