use hegel_pm::discovery::DiscoveryEngine;
use std::error::Error;
use tracing::info;

/// Run discovery scan and print results to stdout
pub fn run(engine: &DiscoveryEngine, refresh: bool) -> Result<(), Box<dyn Error>> {
    info!("🔍 Scanning for Hegel projects...");
    let projects = engine.get_projects(refresh)?;
    info!("📁 Discovered {} projects:\n", projects.len());

    for project in &projects {
        info!("  • {} ({})", project.name, project.project_path.display());
        if let Some(state) = &project.workflow_state {
            info!("    Mode: {} | Phase: {}", state.mode, state.current_node);
        } else {
            info!("    No active workflow");
        }
    }

    Ok(())
}
