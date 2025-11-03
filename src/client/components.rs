use sycamore::prelude::*;

#[component]
pub fn Sidebar() -> View {
    view! {
        div(class="sidebar") {
            h2 { "Projects" }
            p { "Loading projects..." }
        }
    }
}

#[component]
pub fn MetricsView() -> View {
    view! {
        div(class="main-content") {
            h1 { "Hegel Metrics Analysis" }

            div(class="metrics-section") {
                h3 { "Session" }
                p { "ID: 88f74756-ad2c-4789-b0c0-f370e0419d3a" }
            }

            div(class="metrics-section") {
                h3 { "Token Usage" }
                div(class="metric-grid") {
                    div(class="metric-item") {
                        div(class="metric-label") { "Input tokens" }
                        div(class="metric-value") { "77,521" }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Output tokens" }
                        div(class="metric-value") { "87,556" }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Cache creation" }
                        div(class="metric-value") { "2,344,565" }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Cache reads" }
                        div(class="metric-value") { "101,224,969" }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Assistant turns" }
                        div(class="metric-value") { "1,121" }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Total" }
                        div(class="metric-value") { "103,734,611" }
                    }
                }
            }

            div(class="metrics-section") {
                h3 { "Activity" }
                div(class="metric-grid") {
                    div(class="metric-item") {
                        div(class="metric-label") { "Total events" }
                        div(class="metric-value") { "307" }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Bash commands" }
                        div(class="metric-value") { "58" }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "File modifications" }
                        div(class="metric-value") { "31" }
                    }
                }
            }

            div(class="metrics-section") {
                h3 { "Top Bash Commands" }
                ul(class="top-list") {
                    li { span(class="command-text") { "cargo test 2>&1 | grep \"test result:\" | tail -2" } span(class="command-count") { "2x" } }
                    li { span(class="command-text") { "git status" } span(class="command-count") { "2x" } }
                    li { span(class="command-text") { "cargo test --quiet" } span(class="command-count") { "2x" } }
                    li { span(class="command-text") { "hegel guides" } span(class="command-count") { "2x" } }
                }
            }

            div(class="metrics-section") {
                h3 { "Workflow Transitions" }
                div(class="metric-item") {
                    div(class="metric-label") { "Total transitions" }
                    div(class="metric-value") { "379" }
                }
                div(class="metric-item") {
                    div(class="metric-label") { "Mode" }
                    div(class="metric-value") { "execution" }
                }
            }
        }
    }
}
