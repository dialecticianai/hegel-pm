use gloo_net::http::Request;
use sycamore::prelude::*;

use crate::client::types::ProjectStatistics;

#[derive(Props)]
pub struct MetricsViewProps {
    pub selected_project: ReadSignal<Option<String>>,
}

#[component]
pub fn MetricsView(props: MetricsViewProps) -> View {
    let selected_project = props.selected_project;
    let metrics = create_signal(None::<ProjectStatistics>);
    let loading = create_signal(false);
    let error = create_signal(false);

    // Fetch metrics when selected project changes
    create_effect(move || {
        if let Some(name) = selected_project.with(|s| s.clone()) {
            loading.set(true);
            error.set(false);

            let metrics_clone = metrics.clone();
            let loading_clone = loading.clone();
            let error_clone = error.clone();

            sycamore::futures::spawn_local(async move {
                let url = format!("/api/projects/{}/metrics", name);
                match Request::get(&url).send().await {
                    Ok(resp) => {
                        if let Ok(stats) = resp.json::<ProjectStatistics>().await {
                            metrics_clone.set(Some(stats));
                            loading_clone.set(false);
                        } else {
                            error_clone.set(true);
                            loading_clone.set(false);
                        }
                    }
                    Err(_) => {
                        error_clone.set(true);
                        loading_clone.set(false);
                    }
                }
            });
        }
    });

    view! {
        div(class="main-content") {
            h1 {
                "Hegel Metrics Analysis"
                (selected_project.with(|sel| {
                    if let Some(name) = sel {
                        format!(" - {}", name)
                    } else {
                        String::new()
                    }
                }))
            }

            // Status message (always present, content changes)
            p(class="status") {
                (move || {
                    if loading.get() {
                        "Loading metrics..."
                    } else if error.get() {
                        "Error loading metrics"
                    } else if metrics.with(|m| m.is_none()) {
                        "Select a project to view metrics"
                    } else {
                        ""
                    }
                })
            }

            // Metrics sections (always present, content updates reactively)
            div(class="metrics-section") {
                h3 { "Session" }
                p {
                    "ID: "
                    (move || metrics.with(|m| {
                        m.as_ref()
                            .and_then(|s| s.session_id.clone())
                            .unwrap_or_else(|| "N/A".to_string())
                    }))
                }
            }

            div(class="metrics-section") {
                h3 { "Token Usage" }
                div(class="metric-grid") {
                    div(class="metric-item") {
                        div(class="metric-label") { "Input tokens" }
                        div(class="metric-value") {
                            (move || metrics.with(|m| {
                                m.as_ref().map(|s| s.token_metrics.total_input_tokens.to_string()).unwrap_or_else(|| "-".to_string())
                            }))
                        }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Output tokens" }
                        div(class="metric-value") {
                            (move || metrics.with(|m| {
                                m.as_ref().map(|s| s.token_metrics.total_output_tokens.to_string()).unwrap_or_else(|| "-".to_string())
                            }))
                        }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Cache creation" }
                        div(class="metric-value") {
                            (move || metrics.with(|m| {
                                m.as_ref().map(|s| s.token_metrics.total_cache_creation_tokens.to_string()).unwrap_or_else(|| "-".to_string())
                            }))
                        }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Cache reads" }
                        div(class="metric-value") {
                            (move || metrics.with(|m| {
                                m.as_ref().map(|s| s.token_metrics.total_cache_read_tokens.to_string()).unwrap_or_else(|| "-".to_string())
                            }))
                        }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Total tokens" }
                        div(class="metric-value") {
                            (move || metrics.with(|m| {
                                m.as_ref().map(|s| {
                                    (s.token_metrics.total_input_tokens +
                                     s.token_metrics.total_output_tokens +
                                     s.token_metrics.total_cache_creation_tokens +
                                     s.token_metrics.total_cache_read_tokens).to_string()
                                }).unwrap_or_else(|| "-".to_string())
                            }))
                        }
                    }
                }
            }

            div(class="metrics-section") {
                h3 { "Activity" }
                div(class="metric-grid") {
                    div(class="metric-item") {
                        div(class="metric-label") { "Total events" }
                        div(class="metric-value") {
                            (move || metrics.with(|m| {
                                m.as_ref().map(|s| s.hook_metrics.total_events.to_string()).unwrap_or_else(|| "-".to_string())
                            }))
                        }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "Bash commands" }
                        div(class="metric-value") {
                            (move || metrics.with(|m| {
                                m.as_ref().map(|s| s.hook_metrics.bash_count.to_string()).unwrap_or_else(|| "-".to_string())
                            }))
                        }
                    }
                    div(class="metric-item") {
                        div(class="metric-label") { "File modifications" }
                        div(class="metric-value") {
                            (move || metrics.with(|m| {
                                m.as_ref().map(|s| (s.hook_metrics.write_count + s.hook_metrics.edit_count).to_string()).unwrap_or_else(|| "-".to_string())
                            }))
                        }
                    }
                }
            }
        }
    }
}
