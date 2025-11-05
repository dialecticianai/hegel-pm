#![cfg(target_arch = "wasm32")]

use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[path = "common/mod.rs"]
mod common;
use common::*;

// Test for Sidebar: navigation signal updates
#[wasm_bindgen_test]
fn test_sidebar_navigation_signal() {
    #[derive(Clone, Copy, PartialEq)]
    enum View {
        AllProjects,
        ProjectDetail,
    }

    let _ = create_root(|| {
        let current_view = create_signal(View::AllProjects);

        let node = view! {
            div {
                button(id="all-projects", on:click=move |_| current_view.set(View::AllProjects)) { "All Projects" }
                button(id="project", on:click=move |_| current_view.set(View::ProjectDetail)) { "Project" }
                p(id="view-state") {
                    (move || {
                        match current_view.get() {
                            View::AllProjects => "all-projects",
                            View::ProjectDetail => "project-detail",
                        }
                    })
                }
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        let state_elem = query("#view-state");
        assert_text_content!(state_elem, "all-projects");

        // Click project button
        let project_btn = query("#project");
        project_btn
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .click();
        assert_text_content!(state_elem, "project-detail");

        // Click all projects button
        let all_projects_btn = query("#all-projects");
        all_projects_btn
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .click();
        assert_text_content!(state_elem, "all-projects");
    });
}

// Test for Sidebar: selected project signal updates
#[wasm_bindgen_test]
fn test_sidebar_selected_project() {
    let _ = create_root(|| {
        let selected_project = create_signal(None::<String>);

        let select_project = move |name: &str| {
            selected_project.set(Some(name.to_string()));
        };

        let node = view! {
            div {
                button(id="select-foo", on:click=move |_| select_project("foo")) { "Select Foo" }
                button(id="clear", on:click=move |_| selected_project.set(None)) { "Clear" }
                p(id="selected") {
                    (move || {
                        selected_project.with(|s| {
                            s.as_ref().map(|n| n.clone()).unwrap_or_else(|| "none".to_string())
                        })
                    })
                }
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        let selected_elem = query("#selected");
        assert_text_content!(selected_elem, "none");

        // Select a project
        let select_btn = query("#select-foo");
        select_btn
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .click();
        assert_text_content!(selected_elem, "foo");

        // Clear selection
        let clear_btn = query("#clear");
        clear_btn.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
        assert_text_content!(selected_elem, "none");
    });
}

// Test for App: conditional rendering based on view
#[wasm_bindgen_test]
fn test_app_view_routing() {
    #[derive(Clone, Copy, PartialEq)]
    enum View {
        AllProjects,
        ProjectDetail,
    }

    let _ = create_root(|| {
        let current_view = create_signal(View::AllProjects);

        let node = view! {
            div {
                (move || {
                    match current_view.get() {
                        View::AllProjects => view! {
                            p(id="all-projects-view") { "All Projects View" }
                        },
                        View::ProjectDetail => view! {
                            p(id="project-detail-view") { "Project Detail View" }
                        },
                    }
                })
            }
        };

        sycamore::render_in_scope(|| node, &test_container());

        // Initially should show AllProjects view
        let all_projects = query("#all-projects-view");
        assert_text_content!(all_projects, "All Projects View");

        // Switch to ProjectDetail
        current_view.set(View::ProjectDetail);

        // Should now show ProjectDetail view
        let project_detail = query("#project-detail-view");
        assert_text_content!(project_detail, "Project Detail View");
    });
}
