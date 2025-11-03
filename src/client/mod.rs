use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
fn App() -> View {
    view! {
        div(class="container") {
            h1 { "Hegel Metrics Analysis" }

            div(class="metrics-section") {
                h3 { "Session" }
                p { "ID: Sample session" }
            }

            div(class="metrics-section") {
                h3 { "Token Usage" }
                div(class="metric-grid") {
                    div(class="metric-item") {
                        div(class="metric-label") { "Input tokens" }
                        div(class="metric-value") { "12,345" }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Output tokens" }
                        div(class="metric-value") { "6,789" }
                    }
                }
            }

            div(class="metrics-section") {
                h3 { "Activity" }
                div(class="metric-grid") {
                    div(class="metric-item") {
                        div(class="metric-label") { "Total events" }
                        div(class="metric-value") { "42" }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Bash commands" }
                        div(class="metric-value") { "15" }
                    }
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
