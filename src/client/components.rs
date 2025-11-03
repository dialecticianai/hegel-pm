use gloo_net::http::Request;
use sycamore::prelude::*;

use super::types::DiscoveredProject;

#[component]
pub fn Sidebar() -> View {
    let projects = create_signal(Vec::<DiscoveredProject>::new());
    let loading = create_signal(true);
    let error = create_signal(false);

    // Fetch projects
    sycamore::futures::spawn_local(async move {
        match Request::get("/api/projects").send().await {
            Ok(resp) => {
                if let Ok(projs) = resp.json::<Vec<DiscoveredProject>>().await {
                    projects.set(projs);
                    loading.set(false);
                } else {
                    error.set(true);
                    loading.set(false);
                }
            }
            Err(_) => {
                error.set(true);
                loading.set(false);
            }
        }
    });

    view! {
        div(class="sidebar") {
            h2 { "Projects" }
            div(class="project-list") {
                (if loading.get() {
                    view! { p { "Loading projects..." } }
                } else if error.get() {
                    view! { p { "Error loading projects" } }
                } else {
                    view! {
                        Keyed(
                            list=projects,
                            key=|p| p.name.clone(),
                            view=|p| {
                                let name = p.name.clone();
                                let state_view = if let Some(s) = p.workflow_state.clone() {
                                    let mode = s.mode;
                                    let phase = s.current_node;
                                    view! {
                                        span(class="mode") { (mode) }
                                        " • "
                                        span(class="phase") { (phase) }
                                    }
                                } else {
                                    view! { span(class="inactive") { "No workflow" } }
                                };

                                view! {
                                    div(class="project-item") {
                                        div(class="project-name") { (name) }
                                        div(class="project-state") {
                                            (state_view)
                                        }
                                    }
                                }
                            }
                        )
                    }
                })
            }
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
