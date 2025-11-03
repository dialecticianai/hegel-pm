use hegel_pm::discovery::DiscoveryEngine;
use std::error::Error;
use std::sync::{Arc, Mutex};
use warp::Filter;

/// Start web server with project discovery API and static file serving
pub async fn run(engine: &DiscoveryEngine) -> Result<(), Box<dyn Error>> {
    println!("🚀 Starting hegel-pm web server...");
    println!("📍 Cache location: {}", engine.config().cache_location.display());

    // Discover projects
    let projects = engine.get_projects(false)?;
    println!("📁 Discovered {} projects", projects.len());

    // Wrap projects in Arc<Mutex> for shared mutable access
    let projects_arc = Arc::new(Mutex::new(projects));

    // Clone for project list endpoint
    let projects_clone = projects_arc.clone();

    // API endpoint for projects list
    let api_projects = warp::path!("api" / "projects")
        .map(move || {
            let projects = projects_clone.lock().unwrap();
            warp::reply::json(&*projects)
        });

    // Clone for metrics endpoint
    let projects_for_metrics = projects_arc.clone();

    // API endpoint for project metrics
    let api_metrics = warp::path!("api" / "projects" / String / "metrics")
        .map(move |name: String| {
            use std::time::Instant;
            let start = Instant::now();

            let mut projects = projects_for_metrics.lock().unwrap();

            // Find project by name and load statistics
            if let Some(project) = projects.iter_mut().find(|p| p.name == name) {
                if !project.has_statistics() {
                    let load_start = Instant::now();
                    println!("⏳ Loading statistics for project: {}", name);
                    let _ = project.load_statistics();
                    println!("✅ Statistics loaded in {:?}", load_start.elapsed());
                }

                let response = match &project.statistics {
                    Some(stats) => warp::reply::with_status(
                        warp::reply::json(stats),
                        warp::http::StatusCode::OK
                    ),
                    None => warp::reply::with_status(
                        warp::reply::json(&serde_json::json!({"error": "Failed to load statistics"})),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR
                    )
                };

                println!("📊 Metrics request for '{}' completed in {:?}", name, start.elapsed());
                response
            } else {
                println!("❌ Project not found: {}", name);
                warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({"error": "Project not found"})),
                    warp::http::StatusCode::NOT_FOUND
                )
            }
        });

    // Serve static files (HTML, WASM, JS)
    let static_files = warp::fs::dir("./static");

    // Combine routes
    let routes = api_projects.or(api_metrics).or(static_files);

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
