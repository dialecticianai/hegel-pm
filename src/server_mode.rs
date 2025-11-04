use hegel_pm::discovery::{CacheManager, DiscoveryEngine, ProjectListItem, ProjectMetricsSummary};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use warp::Filter;
use tracing::{debug, error, info};

/// Start web server with project discovery API and static file serving
pub async fn run(engine: &DiscoveryEngine) -> Result<(), Box<dyn Error>> {
    info!("🚀 Starting hegel-pm web server...");
    info!("📍 Cache location: {}", engine.config().cache_location.display());

    // Discover projects
    let projects = engine.get_projects(false)?;
    info!("📁 Discovered {} projects", projects.len());

    // Create cache manager for async, non-blocking saves
    let cache_manager = CacheManager::new(engine.config().cache_location.clone());

    // Response cache: project name -> serialized JSON bytes
    let response_cache: Arc<Mutex<HashMap<String, Vec<u8>>>> = Arc::new(Mutex::new(HashMap::new()));

    // Wrap projects in Arc<Mutex> for shared mutable access
    let projects_arc = Arc::new(Mutex::new(projects));

    // Clone for project list endpoint
    let projects_clone = projects_arc.clone();

    // API endpoint for projects list
    let api_projects = warp::path!("api" / "projects")
        .map(move || {
            use std::time::Instant;
            let start = Instant::now();

            let projects = projects_clone.lock().unwrap();
            // Convert to lightweight ProjectListItem (only name + workflow_state)
            let list_items: Vec<ProjectListItem> = projects
                .iter()
                .map(|p| ProjectListItem {
                    name: p.name.clone(),
                    workflow_state: p.workflow_state.clone(),
                })
                .collect();

            debug!("📋 Projects list request completed in {:?} ({} projects)", start.elapsed(), list_items.len());
            warp::reply::json(&list_items)
        });

    // Clone for metrics endpoint
    let projects_for_metrics = projects_arc.clone();
    let cache_manager_clone = cache_manager.clone();
    let response_cache_clone = response_cache.clone();

    // API endpoint for project metrics
    let api_metrics = warp::path!("api" / "projects" / String / "metrics")
        .map(move |name: String| {
            let cache_mgr = cache_manager_clone.clone();
            let resp_cache = response_cache_clone.clone();
            use std::time::Instant;
            let start = Instant::now();

            // Step 1: Check response cache (instant)
            {
                let cache = resp_cache.lock().unwrap();
                if let Some(cached_json) = cache.get(&name) {
                    debug!("💨 Serving cached response for '{}' in {:?}", name, start.elapsed());
                    return warp::http::Response::builder()
                        .status(warp::http::StatusCode::OK)
                        .header("Content-Type", "application/json")
                        .body(cached_json.clone())
                        .unwrap();
                }
            }

            // Step 1: Check if stats needed and get hegel_dir (brief lock)
            let (needs_loading, hegel_dir) = {
                let projects = projects_for_metrics.lock().unwrap();
                match projects.iter().find(|p| p.name == name) {
                    Some(project) => {
                        if project.has_statistics() {
                            (false, None)
                        } else {
                            (true, Some(project.hegel_dir.clone()))
                        }
                    }
                    None => (false, None)
                }
            }; // Mutex released here

            // Step 2: Load statistics WITHOUT holding mutex (slow I/O)
            let loaded_stats = if needs_loading {
                if let Some(dir) = hegel_dir {
                    let load_start = Instant::now();
                    debug!("⏳ Loading statistics for project: {}", name);
                    match hegel::metrics::parse_unified_metrics(&dir, true) {
                        Ok(stats) => {
                            info!("✅ Statistics loaded in {:?}", load_start.elapsed());
                            Some(stats)
                        }
                        Err(e) => {
                            error!("❌ Failed to load statistics for '{}': {}", name, e);
                            None
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            };

            // Step 3: Update state if we loaded stats (brief lock)
            let needs_caching = if let Some(stats) = loaded_stats {
                let mut projects = projects_for_metrics.lock().unwrap();
                if let Some(project) = projects.iter_mut().find(|p| p.name == name) {
                    project.statistics = Some(stats);
                    true
                } else {
                    false
                }
            } else {
                false
            };

            // Step 4: Get stats (brief lock, clone data)
            let stats_result = {
                let projects = projects_for_metrics.lock().unwrap();
                projects.iter()
                    .find(|p| p.name == name)
                    .map(|project| project.statistics.clone())
            }; // Mutex released BEFORE serialization

            // Step 5: Build response (no mutex, serialize to JSON once and cache)
            let response = match stats_result {
                Some(Some(stats)) => {
                    // Convert to lightweight summary (only counts, no raw data)
                    let summary = ProjectMetricsSummary::from(&stats);
                    match serde_json::to_vec(&summary) {
                        Ok(json_bytes) => {
                            // Cache the serialized response
                            resp_cache.lock().unwrap().insert(name.clone(), json_bytes.clone());
                            debug!("📦 Cached JSON response for '{}' ({} bytes)", name, json_bytes.len());

                            warp::http::Response::builder()
                                .status(warp::http::StatusCode::OK)
                                .header("Content-Type", "application/json")
                                .body(json_bytes)
                                .unwrap()
                        }
                        Err(e) => {
                            error!("❌ JSON serialization failed: {}", e);
                            warp::http::Response::builder()
                                .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                                .header("Content-Type", "application/json")
                                .body(serde_json::to_vec(&serde_json::json!({"error": "Serialization failed"})).unwrap())
                                .unwrap()
                        }
                    }
                }
                Some(None) => warp::http::Response::builder()
                    .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_vec(&serde_json::json!({"error": "Failed to load statistics"})).unwrap())
                    .unwrap(),
                None => {
                    info!("❌ Project not found: {}", name);
                    warp::http::Response::builder()
                        .status(warp::http::StatusCode::NOT_FOUND)
                        .header("Content-Type", "application/json")
                        .body(serde_json::to_vec(&serde_json::json!({"error": "Project not found"})).unwrap())
                        .unwrap()
                }
            };

            debug!("📊 Metrics request for '{}' completed in {:?}", name, start.elapsed());

            // Queue cache save if statistics were loaded (non-blocking)
            if needs_caching {
                let projects = projects_for_metrics.lock().unwrap();
                cache_mgr.queue_save(projects.clone());
                info!("📤 Queued cache save for '{}'", name);
            }

            response
        });

    // Serve static files (HTML, WASM, JS)
    let static_files = warp::fs::dir("./static");

    // Combine routes
    let routes = api_projects.or(api_metrics).or(static_files);

    let url = "http://localhost:3030";
    info!("🌐 Server running at {}", url);
    info!("📝 Build WASM with: trunk build --release");

    // Open browser
    if let Err(e) = open::that(url) {
        error!("⚠️  Failed to open browser: {}", e);
    } else {
        info!("🌍 Opening browser...");
    }

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
