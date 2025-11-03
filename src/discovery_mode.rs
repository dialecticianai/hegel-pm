use hegel_pm::discovery::DiscoveryEngine;
use std::error::Error;

/// Run discovery scan and print results to stdout
pub fn run(engine: &DiscoveryEngine, refresh: bool) -> Result<(), Box<dyn Error>> {
    println!("🔍 Scanning for Hegel projects...");
    let projects = engine.get_projects(refresh)?;
    println!("📁 Discovered {} projects:\n", projects.len());

    for project in &projects {
        println!("  • {} ({})", project.name, project.project_path.display());
        if let Some(state) = &project.workflow_state {
            println!("    Mode: {} | Phase: {}", state.mode, state.current_node);
        } else {
            println!("    No active workflow");
        }
    }

    Ok(())
}
