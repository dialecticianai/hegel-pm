use gloo_net::http::Request;
use sycamore::prelude::*;
use sycamore::reactive::batch;
use crate::client::types::{ProjectInfo, WorkflowSummary, PhaseSummary};

#[derive(Props)]
pub struct WorkflowDetailViewProps {
    pub selected_project: ReadSignal<Option<String>>,
}

#[component]
pub fn WorkflowDetailView(props: WorkflowDetailViewProps) -> View {
    let selected_project = props.selected_project;

    // Data signals
    let project_info = create_signal(None::<ProjectInfo>);
    let loading = create_signal(false);
    let error = create_signal(false);

    // Collapse state signals (index-based)
    let workflows_collapsed = create_signal(Vec::<bool>::new());
    let phases_collapsed = create_signal(Vec::<Vec<bool>>::new());

    // Fetch project info when selected project changes
    create_effect(move || {
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
                            batch(|| {
                                error.set(true);
                                loading.set(false);
                            });
                            return;
                        }

                        match resp.json::<ProjectInfo>().await {
                            Ok(info) => {
                                web_sys::console::log_1(&"Successfully loaded project info".into());

                                // Initialize collapse states based on data
                                let workflow_count = info.detail.workflows.len();
                                let wf_collapsed = vec![true; workflow_count];

                                let phase_counts: Vec<Vec<bool>> = info.detail.workflows
                                    .iter()
                                    .map(|w| vec![true; w.phases.len()])
                                    .collect();

                                batch(|| {
                                    workflows_collapsed.set(wf_collapsed);
                                    phases_collapsed.set(phase_counts);
                                    project_info.set(Some(info));
                                    loading.set(false);
                                });
                            }
                            Err(e) => {
                                web_sys::console::error_1(&format!("JSON parse failed: {:?}", e).into());
                                batch(|| {
                                    error.set(true);
                                    loading.set(false);
                                });
                            }
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Request failed: {:?}", e).into());
                        batch(|| {
                            error.set(true);
                            loading.set(false);
                        });
                    }
                }
            });
        }
    });

    // Expand all workflows and phases
    let expand_all = move || {
        batch(|| {
            workflows_collapsed.update(|v| v.iter_mut().for_each(|c| *c = false));
            phases_collapsed.update(|v| {
                v.iter_mut().for_each(|workflow_phases| {
                    workflow_phases.iter_mut().for_each(|c| *c = false);
                });
            });
        });
    };

    // Collapse all workflows
    let collapse_all = move || {
        workflows_collapsed.update(|v| v.iter_mut().for_each(|c| *c = true));
    };

    view! {
        div(class="workflow-detail-view") {
            h1 {
                "Project Details"
                (selected_project.with(|sel| {
                    if let Some(name) = sel {
                        format!(" - {}", name)
                    } else {
                        String::new()
                    }
                }))
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
                                        div(class="metric-label") { "Bash Commands" }
                                        div(class="metric-value") { (info.summary.bash_command_count.to_string()) }
                                    }
                                    div(class="metric-item") {
                                        div(class="metric-label") { "File Modifications" }
                                        div(class="metric-value") { (info.summary.file_modification_count.to_string()) }
                                    }
                                }
                            }
                        }
                    } else {
                        view! {}
                    }
                })
            }

            // Control buttons
            div(class="workflow-controls") {
                (move || {
                    if project_info.with(|p| p.is_some()) {
                        view! {
                            div {
                                button(class="btn", on:click=move |_| expand_all()) { "Expand All" }
                                button(class="btn", on:click=move |_| collapse_all()) { "Collapse All" }
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
                        // Add index to workflows for tracking
                        let workflows_with_idx: Vec<(usize, WorkflowSummary)> = info.detail.workflows
                            .into_iter()
                            .enumerate()
                            .collect();

                        view! {
                            div {
                                h2 { "Workflows" }
                                Indexed(
                                    list=create_signal(workflows_with_idx),
                                    view=move |(idx, workflow)| {
                                        render_workflow(idx, workflow, workflows_collapsed, phases_collapsed)
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

fn render_workflow(
    workflow_idx: usize,
    workflow: WorkflowSummary,
    workflows_collapsed: Signal<Vec<bool>>,
    phases_collapsed: Signal<Vec<Vec<bool>>>,
) -> View {
    // Clone data needed for view BEFORE view! macro
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
                workflows_collapsed.update(|v| {
                    if workflow_idx < v.len() {
                        v[workflow_idx] = !v[workflow_idx];
                    }
                });
            }) {
                span(class="collapse-icon") {
                    (move || {
                        workflows_collapsed.with(|v| {
                            if workflow_idx < v.len() && v[workflow_idx] {
                                "▶"
                            } else {
                                "▼"
                            }
                        })
                    })
                }
                span(class="workflow-title") {
                    (format!("{} ({})", mode, status))
                }
                span(class="workflow-id") {
                    (workflow_id)
                }
            }

            // Workflow summary metrics (visible when collapsed)
            div(class="workflow-summary") {
                (move || {
                    workflows_collapsed.with(|v| {
                        if workflow_idx < v.len() && v[workflow_idx] {
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
                })
            }

            // Phases (visible when expanded)
            div(class="phases-container") {
                (move || {
                    workflows_collapsed.with(|wc| {
                        if workflow_idx < wc.len() && !wc[workflow_idx] {
                            render_phases(workflow_idx, phases.clone(), phases_collapsed)
                        } else {
                            view! {}
                        }
                    })
                })
            }
        }
    }
}

fn render_phases(
    workflow_idx: usize,
    phases: Vec<PhaseSummary>,
    phases_collapsed: Signal<Vec<Vec<bool>>>,
) -> View {
    // Add index to phases for tracking
    let phases_with_idx: Vec<(usize, PhaseSummary)> = phases
        .into_iter()
        .enumerate()
        .collect();

    view! {
        div(class="phases-list") {
            Indexed(
                list=create_signal(phases_with_idx),
                view=move |(phase_idx, phase)| {
                    render_phase(workflow_idx, phase_idx, phase, phases_collapsed)
                }
            )
        }
    }
}

fn render_phase(
    workflow_idx: usize,
    phase_idx: usize,
    phase: PhaseSummary,
    phases_collapsed: Signal<Vec<Vec<bool>>>,
) -> View {
    // Clone phase data before view
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
                phases_collapsed.update(|v| {
                    if workflow_idx < v.len() && phase_idx < v[workflow_idx].len() {
                        v[workflow_idx][phase_idx] = !v[workflow_idx][phase_idx];
                    }
                });
            }) {
                span(class="collapse-icon") {
                    (move || {
                        phases_collapsed.with(|v| {
                            if workflow_idx < v.len() && phase_idx < v[workflow_idx].len() && v[workflow_idx][phase_idx] {
                                "▶"
                            } else {
                                "▼"
                            }
                        })
                    })
                }
                span(class="phase-title") {
                    (format!("{} ({})", phase_name, phase_status))
                }
            }

            // Phase details (visible when expanded)
            div(class="phase-details") {
                (move || {
                    phases_collapsed.with(|pc| {
                        if workflow_idx < pc.len() && phase_idx < pc[workflow_idx].len() && !pc[workflow_idx][phase_idx] {
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
                })
            }
        }
    }
}
