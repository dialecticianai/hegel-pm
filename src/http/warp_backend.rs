use crate::data_layer::messages::{DataError, DataRequest};
use crate::http::{HttpBackend, ServerConfig};
use async_trait::async_trait;
use std::error::Error;
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, info};
use warp::Filter;

/// Warp HTTP backend implementation
pub struct WarpBackend;

impl WarpBackend {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl HttpBackend for WarpBackend {
    async fn run(
        &self,
        data_tx: mpsc::Sender<DataRequest>,
        config: ServerConfig,
    ) -> Result<(), Box<dyn Error>> {
        info!("🚀 Starting hegel-pm web server with Warp backend...");

        // Clone for project list endpoint
        let data_tx_projects = data_tx.clone();

        // API endpoint for projects list
        let api_projects = warp::path!("api" / "projects")
            .and(warp::any().map(move || data_tx_projects.clone()))
            .and_then(|data_tx: mpsc::Sender<DataRequest>| async move {
                use std::time::Instant;
                let start = Instant::now();

                let (reply_tx, reply_rx) = oneshot::channel();
                data_tx
                    .send(DataRequest::GetProjects { reply: reply_tx })
                    .await
                    .map_err(|e| {
                        error!("Failed to send GetProjects request: {}", e);
                        warp::reject::reject()
                    })?;

                let json_bytes = reply_rx.await.map_err(|e| {
                    error!("Failed to receive GetProjects reply: {}", e);
                    warp::reject::reject()
                })?;

                debug!(
                    "📋 Projects list request completed in {:?}",
                    start.elapsed()
                );

                Ok::<_, warp::Rejection>(
                    warp::http::Response::builder()
                        .status(warp::http::StatusCode::OK)
                        .header("Content-Type", "application/json")
                        .body(json_bytes)
                        .unwrap(),
                )
            });

        // Clone for metrics endpoint
        let data_tx_metrics = data_tx.clone();

        // API endpoint for project metrics
        let api_metrics = warp::path!("api" / "projects" / String / "metrics")
            .and(warp::any().map(move || data_tx_metrics.clone()))
            .and_then(
                |name: String, data_tx: mpsc::Sender<DataRequest>| async move {
                    use std::time::Instant;
                    let start = Instant::now();

                    let (reply_tx, reply_rx) = oneshot::channel();
                    data_tx
                        .send(DataRequest::GetProjectMetrics {
                            name: name.clone(),
                            reply: reply_tx,
                        })
                        .await
                        .map_err(|e| {
                            error!("Failed to send GetProjectMetrics request: {}", e);
                            warp::reject::reject()
                        })?;

                    let result = reply_rx.await.map_err(|e| {
                        error!("Failed to receive GetProjectMetrics reply: {}", e);
                        warp::reject::reject()
                    })?;

                    match result {
                        Ok(json_bytes) => {
                            debug!(
                                "📊 Metrics request for '{}' completed in {:?}",
                                name,
                                start.elapsed()
                            );
                            Ok::<_, warp::Rejection>(
                                warp::http::Response::builder()
                                    .status(warp::http::StatusCode::OK)
                                    .header("Content-Type", "application/json")
                                    .body(json_bytes)
                                    .unwrap(),
                            )
                        }
                        Err(DataError::ProjectNotFound(name)) => {
                            info!("❌ Project not found: {}", name);
                            Ok(warp::http::Response::builder()
                                .status(warp::http::StatusCode::NOT_FOUND)
                                .header("Content-Type", "application/json")
                                .body(
                                    serde_json::to_vec(
                                        &serde_json::json!({"error": "Project not found"}),
                                    )
                                    .unwrap(),
                                )
                                .unwrap())
                        }
                        Err(e) => {
                            error!("❌ Error getting project metrics: {:?}", e);
                            Ok(warp::http::Response::builder()
                                .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                                .header("Content-Type", "application/json")
                                .body(
                                    serde_json::to_vec(
                                        &serde_json::json!({"error": "Internal server error"}),
                                    )
                                    .unwrap(),
                                )
                                .unwrap())
                        }
                    }
                },
            );

        // Clone for all-projects endpoint
        let data_tx_all = data_tx.clone();

        // API endpoint for all-projects aggregate
        let api_all_projects = warp::path!("api" / "all-projects")
            .and(warp::any().map(move || data_tx_all.clone()))
            .and_then(|data_tx: mpsc::Sender<DataRequest>| async move {
                use std::time::Instant;
                let start = Instant::now();

                let (reply_tx, reply_rx) = oneshot::channel();
                data_tx
                    .send(DataRequest::GetAllProjects { reply: reply_tx })
                    .await
                    .map_err(|e| {
                        error!("Failed to send GetAllProjects request: {}", e);
                        warp::reject::reject()
                    })?;

                let json_bytes = reply_rx.await.map_err(|e| {
                    error!("Failed to receive GetAllProjects reply: {}", e);
                    warp::reject::reject()
                })?;

                debug!(
                    "📊 All-projects aggregate request completed in {:?}",
                    start.elapsed()
                );

                Ok::<_, warp::Rejection>(
                    warp::http::Response::builder()
                        .status(warp::http::StatusCode::OK)
                        .header("Content-Type", "application/json")
                        .body(json_bytes)
                        .unwrap(),
                )
            });

        // Serve static files (HTML, WASM, JS)
        let static_files = warp::fs::dir(config.static_dir.clone());

        // Combine routes
        let routes = api_projects
            .or(api_all_projects)
            .or(api_metrics)
            .or(static_files);

        info!("🌐 Server running at {}", config.url());
        info!("📝 Build WASM with: trunk build --release");

        // Open browser
        if config.open_browser {
            if let Err(e) = open::that(&config.url()) {
                error!("⚠️  Failed to open browser: {}", e);
            } else {
                info!("🌍 Opening browser...");
            }
        }

        warp::serve(routes).run((config.host, config.port)).await;

        Ok(())
    }
}
