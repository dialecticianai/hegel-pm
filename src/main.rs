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

    // Serve static HTML (stub UI)
    let routes = warp::path::end()
        .map(|| {
            warp::reply::html(
                r#"
<!DOCTYPE html>
<html>
<head>
    <title>Hegel PM</title>
    <style>
        body { font-family: system-ui; max-width: 1200px; margin: 40px auto; padding: 0 20px; }
        h1 { color: #333; }
        .projects { display: grid; gap: 20px; }
        .project { border: 1px solid #ddd; padding: 20px; border-radius: 8px; }
        .project h2 { margin-top: 0; }
        .meta { color: #666; font-size: 14px; }
    </style>
</head>
<body>
    <h1>Hegel Project Manager</h1>
    <p>Stub UI - Sycamore integration coming soon</p>
    <div class="projects">
        <div class="project">
            <h2>Example Project</h2>
            <div class="meta">Path: /example/path</div>
            <div class="meta">Status: Active</div>
        </div>
    </div>
</body>
</html>
                "#
            )
        });

    println!("🌐 Server running at http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
