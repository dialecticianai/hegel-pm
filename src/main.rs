use hegel_pm::discovery::{DiscoveryConfig, DiscoveryEngine};
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting hegel-pm web server...");

    // Initialize discovery engine with default config
    let config = DiscoveryConfig::default();
    let engine = DiscoveryEngine::new(config)?;

    // Discover projects
    let projects = engine.get_projects(false)?;
    println!("📁 Discovered {} projects", projects.len());

    // Serve static files (HTML, WASM, JS)
    let static_files = warp::fs::dir("./static");

    println!("🌐 Server running at http://localhost:3030");
    println!("📝 Build WASM with: trunk build --release");

    warp::serve(static_files)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
