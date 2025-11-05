use gloo_net::http::Request;
use sycamore::prelude::*;
use crate::client::types::AllProjectsAggregate;

#[component]
pub fn AllProjectsView() -> View {
    let aggregate_data = create_signal(None::<AllProjectsAggregate>);
    let loading = create_signal(true);
    let error = create_signal(false);

    // Fetch aggregate data on mount
    create_effect(move || {
        sycamore::futures::spawn_local(async move {
            match Request::get("/api/all-projects").send().await {
                Ok(resp) => {
                    if resp.status() != 200 {
                        web_sys::console::error_1(&format!("HTTP error: {}", resp.status()).into());
                        error.set(true);
                        loading.set(false);
                        return;
                    }

                    match resp.json::<AllProjectsAggregate>().await {
                        Ok(data) => {
                            aggregate_data.set(Some(data));
                            loading.set(false);
                        }
                        Err(e) => {
                            web_sys::console::error_1(&format!("JSON parse failed: {:?}", e).into());
                            error.set(true);
                            loading.set(false);
                        }
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Request failed: {:?}", e).into());
                    error.set(true);
                    loading.set(false);
                }
            }
        });
    });

    view! {
        div(class="all-projects-view") {
            h1 { "All Projects" }

            p(class="status") {
                (move || {
                    if error.get() {
                        "Failed to load aggregate metrics"
                    } else if loading.get() {
                        "Loading aggregate metrics..."
                    } else {
                        ""
                    }
                })
            }

            div(class="aggregate-stats") {
                (move || {
                    // Clone aggregate data before building view to avoid lifetime issues
                    let data_clone = aggregate_data.get_clone();
                    if let Some(aggregate) = data_clone {
                        view! {
                    div(class="aggregate-stats") {
                        div(class="stat-card") {
                            h3 { "Total Projects" }
                            p(class="stat-value") { (aggregate.total_projects.to_string()) }
                        }

                        div(class="stat-card") {
                            h3 { "Total Tokens" }
                            p(class="stat-value") { (format_large_number(aggregate.aggregate_metrics.total_all_tokens)) }
                        }

                        div(class="stat-card") {
                            h3 { "Total Events" }
                            p(class="stat-value") { (aggregate.aggregate_metrics.total_events.to_string()) }
                        }

                        div(class="stat-card") {
                            h3 { "Bash Commands" }
                            p(class="stat-value") { (aggregate.aggregate_metrics.bash_command_count.to_string()) }
                        }

                        div(class="stat-card") {
                            h3 { "File Modifications" }
                            p(class="stat-value") { (aggregate.aggregate_metrics.file_modification_count.to_string()) }
                        }

                        div(class="stat-card") {
                            h3 { "Git Commits" }
                            p(class="stat-value") { (aggregate.aggregate_metrics.git_commit_count.to_string()) }
                        }

                        div(class="stat-card") {
                            h3 { "Total Phases" }
                            p(class="stat-value") { (aggregate.aggregate_metrics.phase_count.to_string()) }
                        }
                    }

                    div(class="token-breakdown") {
                        h2 { "Token Breakdown" }
                        div(class="breakdown-stats") {
                            p { "Input: " (format_large_number(aggregate.aggregate_metrics.total_input_tokens)) }
                            p { "Output: " (format_large_number(aggregate.aggregate_metrics.total_output_tokens)) }
                            p { "Cache Creation: " (format_large_number(aggregate.aggregate_metrics.total_cache_creation_tokens)) }
                            p { "Cache Read: " (format_large_number(aggregate.aggregate_metrics.total_cache_read_tokens)) }
                        }
                    }
                        }
                    } else {
                        view! {}
                    }
                })
            }
        }
    }
}

fn format_large_number(num: u64) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}
