use hegel_pm::discovery::DiscoveryEngine;
use std::error::Error;
use warp::Filter;

/// Start web server with project discovery API and static file serving
pub async fn run(engine: &DiscoveryEngine) -> Result<(), Box<dyn Error>> {
    println!("🚀 Starting hegel-pm web server...");

    // Discover projects
    let projects = engine.get_projects(false)?;
    println!("📁 Discovered {} projects", projects.len());

    // Clone projects for API endpoint
    let projects_clone = projects.clone();

    // API endpoint for projects
    let api_projects = warp::path!("api" / "projects")
        .map(move || warp::reply::json(&projects_clone));

    // Serve static files (HTML, WASM, JS)
    let static_files = warp::fs::dir("./static");

    // Combine routes
    let routes = api_projects.or(static_files);

    let url = "http://localhost:3030";
    println!("🌐 Server running at {}", url);
    println!("📝 Build WASM with: trunk build --release");

    // Open browser
    if let Err(e) = open::that(url) {
        eprintln!("⚠️  Failed to open browser: {}", e);
    } else {
        println!("🌍 Opening browser...");
    }

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
