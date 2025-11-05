use gloo_net::http::Request;
use sycamore::prelude::*;

use crate::client::types::{DiscoveredProject, View as ViewEnum};

#[derive(Props)]
pub struct SidebarProps {
    pub current_view: Signal<ViewEnum>,
    pub selected_project: Signal<Option<String>>,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> View {
    let projects = create_signal(Vec::<DiscoveredProject>::new());

    // Fetch projects and populate
    let selected_clone = props.selected_project;
    let view_clone = props.current_view;
    sycamore::futures::spawn_local(async move {
        match Request::get("/api/projects").send().await {
            Ok(resp) => {
                let status = resp.status();
                match resp.json::<Vec<DiscoveredProject>>().await {
                    Ok(projs) => {
                        if let Some(first) = projs.first() {
                            selected_clone.set(Some(first.name.clone()));
                            view_clone.set(ViewEnum::ProjectDetail);
                        }
                        projects.set(projs);
                    }
                    Err(e) => {
                        web_sys::console::error_1(
                            &format!(
                                "Failed to parse /api/projects response (status {}): {:?}",
                                status, e
                            )
                            .into(),
                        );
                    }
                }
            }
            Err(e) => {
                web_sys::console::error_1(
                    &format!("Failed to fetch /api/projects: {:?}", e).into(),
                );
            }
        }
    });

    view! {
        div(class="sidebar") {
            h2 { "Projects" }

            // All Projects link
            div(
                class="nav-item all-projects",
                on:click=move |_| {
                    props.current_view.set(ViewEnum::AllProjects);
                    props.selected_project.set(None);
                }
            ) {
                "All Projects"
            }

            hr {}

            div(class="project-list") {
                Indexed(
                    list=projects,
                    view=move |p| {
                        let name = p.name.clone();
                        let name_for_click = name.clone();
                        let name_for_class = name.clone();
                        let sel_proj = props.selected_project;
                        let current_view = props.current_view;

                        let mode_text = p.workflow_state.as_ref().map(|s| s.mode.clone());
                        let phase_text = p.workflow_state.as_ref().map(|s| s.current_node.clone());

                        view! {
                            div(
                                class=move || {
                                    if sel_proj.with(|sel| sel.as_ref() == Some(&name_for_class)) {
                                        "project-item active"
                                    } else {
                                        "project-item"
                                    }
                                },
                                on:click=move |_| {
                                    current_view.set(ViewEnum::ProjectDetail);
                                    sel_proj.set(Some(name_for_click.clone()));
                                }
                            ) {
                                div(class="project-name") { (name) }
                                div(class="project-state") {
                                    (if let (Some(mode), Some(phase)) = (mode_text, phase_text) {
                                        view! {
                                            span(class="mode") { (mode) }
                                            " • "
                                            span(class="phase") { (phase) }
                                        }
                                    } else {
                                        view! { span(class="inactive") { "No workflow" } }
                                    })
                                }
                            }
                        }
                    }
                )
            }
        }
    }
}
