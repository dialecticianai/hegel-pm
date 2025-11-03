use clap::Parser;
use hegel_pm::discovery::{DiscoveryConfig, DiscoveryEngine};

/// Hegel Project Manager - Multi-project workflow orchestration
#[derive(Parser, Debug)]
#[command(name = "hegel-pm")]
#[command(about = "Project manager for Hegel projects with web UI", long_about = None)]
struct Args {
    /// Run discovery scan and print results (don't start server)
    #[arg(long)]
    discover: bool,

    /// Force refresh cache during discovery
    #[arg(long)]
    refresh: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize discovery engine with default config
    let config = DiscoveryConfig::default();
    let engine = DiscoveryEngine::new(config)?;

    if args.discover {
        // Discovery mode: scan and print results
        println!("🔍 Scanning for Hegel projects...");
        let projects = engine.get_projects(args.refresh)?;
        println!("📁 Discovered {} projects:\n", projects.len());

        for project in &projects {
            println!("  • {} ({})", project.name, project.project_path.display());
            if let Some(state) = &project.workflow_state {
                println!("    Mode: {} | Phase: {}", state.mode, state.current_node);
            } else {
                println!("    No active workflow");
            }
        }

        return Ok(());
    }

    // Default mode: start web server
    println!("🚀 Starting hegel-pm web server...");

    // Discover projects
    let projects = engine.get_projects(false)?;
    println!("📁 Discovered {} projects", projects.len());

    // Serve static files (HTML, WASM, JS)
    let static_files = warp::fs::dir("./static");

    let url = "http://localhost:3030";
    println!("🌐 Server running at {}", url);
    println!("📝 Build WASM with: trunk build --release");

    // Open browser
    if let Err(e) = open::that(url) {
        eprintln!("⚠️  Failed to open browser: {}", e);
    } else {
        println!("🌍 Opening browser...");
    }

    warp::serve(static_files)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
