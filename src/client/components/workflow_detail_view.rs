use gloo_net::http::Request;
use sycamore::prelude::*;
use crate::client::types::{ProjectInfo, WorkflowSummary, PhaseSummary};

#[derive(Props)]
pub struct WorkflowDetailViewProps {
    pub selected_project: ReadSignal<Option<String>>,
    pub is_visible: ReadSignal<bool>,
}

#[component]
pub fn WorkflowDetailView(props: WorkflowDetailViewProps) -> View {
    let selected_project = props.selected_project;
    let is_visible = props.is_visible;

    // Data signals
    let project_info = create_signal(None::<ProjectInfo>);
    let loading = create_signal(false);
    let error = create_signal(false);

    // Fetch project info when selected project changes AND view is visible
    create_effect(move || {
        let should_fetch = is_visible.get() && selected_project.with(|s| s.is_some());

        if should_fetch {
            if let Some(name) = selected_project.with(|s| s.clone()) {
                loading.set(true);
                error.set(false);

                sycamore::futures::spawn_local(async move {
                    let url = format!("/api/projects/{}/metrics", name);
                    web_sys::console::log_1(&format!("Fetching project info from: {}", url).into());

                    match Request::get(&url).send().await {
                        Ok(resp) => {
                            if resp.status() != 200 {
                                web_sys::console::error_1(&format!("HTTP error: {}", resp.status()).into());
                                error.set(true);
                                loading.set(false);
                                return;
                            }

                            match resp.json::<ProjectInfo>().await {
                                Ok(info) => {
                                    web_sys::console::log_1(&"Successfully loaded project info".into());
                                    project_info.set(Some(info));
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
            }
        }
    });

    view! {
        div(class="workflow-detail-view") {
            h1 {
                (move || {
                    selected_project.with(|s| {
                        s.as_ref()
                            .map(|n| format!("Project: {}", n))
                            .unwrap_or_else(|| "Select a project".to_string())
                    })
                })
            }

            p(class="status") {
                (move || {
                    if error.get() {
                        "Failed to load project details"
                    } else if loading.get() {
                        "Loading project details..."
                    } else if project_info.with(|p| p.is_none()) {
                        "Select a project to view details"
                    } else {
                        ""
                    }
                })
            }

            // Summary metrics section
            div(class="summary-section") {
                (move || {
                    let info_clone = project_info.get_clone();
                    if let Some(info) = info_clone {
                        view! {
                            div {
                                h2 { "Summary Metrics" }
                                div(class="metric-grid") {
                                    div(class="metric-item") {
                                        div(class="metric-label") { "Total Tokens" }
                                        div(class="metric-value") { (info.summary.total_all_tokens.to_string()) }
                                    }
                                    div(class="metric-item") {
                                        div(class="metric-label") { "Total Events" }
                                        div(class="metric-value") { (info.summary.total_events.to_string()) }
                                    }
                                    div(class="metric-item") {
                                        div(class="metric-label") { "Workflows" }
                                        div(class="metric-value") { (info.detail.workflows.len().to_string()) }
                                    }
                                }
                            }
                        }
                    } else {
                        view! {}
                    }
                })
            }

            // Workflows section
            div(class="workflows-section") {
                (move || {
                    let info_clone = project_info.get_clone();
                    if let Some(info) = info_clone {
                        let workflows = info.detail.workflows;

                        view! {
                            div {
                                h2 { "Workflows" }
                                Indexed(
                                    list=create_signal(workflows),
                                    view=|workflow| {
                                        view! {
                                            WorkflowItem(workflow=workflow)
                                        }
                                    }
                                )
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

// Separate component for each workflow - owns its own collapse state
#[derive(Props)]
pub struct WorkflowItemProps {
    pub workflow: WorkflowSummary,
}

#[component]
pub fn WorkflowItem(props: WorkflowItemProps) -> View {
    let workflow = props.workflow;

    // Local collapse state for this workflow
    let collapsed = create_signal(true);

    // Clone data for closures
    let mode = workflow.mode.clone();
    let status = workflow.status.clone();
    let workflow_id = workflow.workflow_id.clone();
    let total_tokens = workflow.total_metrics.total_all_tokens;
    let total_events = workflow.total_metrics.event_count;
    let phases_len = workflow.phases.len();
    let phases = workflow.phases.clone();

    view! {
        div(class="workflow-item") {
            div(class="workflow-header", on:click=move |_| {
                collapsed.set(!collapsed.get());
            }) {
                span(class="collapse-icon") {
                    (move || if collapsed.get() { "▶" } else { "▼" })
                }
                span(class="workflow-title") {
                    (format!("{} ({})", mode, status))
                }
                span(class="workflow-id") {
                    (workflow_id.clone())
                }
            }

            // Workflow summary (visible when collapsed)
            div(class="workflow-summary") {
                (move || {
                    if collapsed.get() {
                        view! {
                            p {
                                "Tokens: " (total_tokens.to_string())
                                " | Events: " (total_events.to_string())
                                " | Phases: " (phases_len.to_string())
                            }
                        }
                    } else {
                        view! {}
                    }
                })
            }

            // Phases (visible when expanded)
            div(class="phases-container") {
                (move || {
                    if !collapsed.get() {
                        let phases_clone = phases.clone();
                        view! {
                            div(class="phases-list") {
                                Indexed(
                                    list=create_signal(phases_clone),
                                    view=|phase| {
                                        view! {
                                            PhaseItem(phase=phase)
                                        }
                                    }
                                )
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

// Separate component for each phase - owns its own collapse state
#[derive(Props)]
pub struct PhaseItemProps {
    pub phase: PhaseSummary,
}

#[component]
pub fn PhaseItem(props: PhaseItemProps) -> View {
    let phase = props.phase;

    // Local collapse state for this phase
    let collapsed = create_signal(true);

    // Clone phase data
    let phase_name = phase.phase_name.clone();
    let phase_status = phase.status.clone();
    let duration = phase.duration_seconds;
    let start_time = phase.start_time.clone();
    let end_time = phase.end_time.clone();
    let total_all_tokens = phase.metrics.total_all_tokens;
    let total_input_tokens = phase.metrics.total_input_tokens;
    let total_output_tokens = phase.metrics.total_output_tokens;
    let event_count = phase.metrics.event_count;
    let bash_count = phase.metrics.bash_command_count;
    let file_mod_count = phase.metrics.file_modification_count;

    view! {
        div(class="phase-item") {
            div(class="phase-header", on:click=move |_| {
                collapsed.set(!collapsed.get());
            }) {
                span(class="collapse-icon") {
                    (move || if collapsed.get() { "▶" } else { "▼" })
                }
                span(class="phase-title") {
                    (format!("{} ({})", phase_name, phase_status))
                }
            }

            // Phase details (visible when expanded)
            div(class="phase-details") {
                (move || {
                    if !collapsed.get() {
                        let start = start_time.clone();
                        let end = end_time.clone().unwrap_or_else(|| "In Progress".to_string());

                        view! {
                            div(class="phase-metrics") {
                                p { "Duration: " (duration.to_string()) " seconds" }
                                p { "Start: " (start) }
                                p { "End: " (end) }

                                h4 { "Metrics" }
                                div(class="metric-grid") {
                                    div(class="metric-item") {
                                        div(class="metric-label") { "Total Tokens" }
                                        div(class="metric-value") { (total_all_tokens.to_string()) }
                                    }
                                    div(class="metric-item") {
                                        div(class="metric-label") { "Input Tokens" }
                                        div(class="metric-value") { (total_input_tokens.to_string()) }
                                    }
                                    div(class="metric-item") {
                                        div(class="metric-label") { "Output Tokens" }
                                        div(class="metric-value") { (total_output_tokens.to_string()) }
                                    }
                                    div(class="metric-item") {
                                        div(class="metric-label") { "Events" }
                                        div(class="metric-value") { (event_count.to_string()) }
                                    }
                                    div(class="metric-item") {
                                        div(class="metric-label") { "Bash Commands" }
                                        div(class="metric-value") { (bash_count.to_string()) }
                                    }
                                    div(class="metric-item") {
                                        div(class="metric-label") { "File Modifications" }
                                        div(class="metric-value") { (file_mod_count.to_string()) }
                                    }
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
