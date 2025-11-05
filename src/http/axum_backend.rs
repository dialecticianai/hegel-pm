use crate::data_layer::messages::{DataError, DataRequest};
use crate::http::{HttpBackend, ServerConfig};
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::error::Error;
use std::net::Ipv4Addr;
use tokio::sync::{mpsc, oneshot};
use tower_http::services::ServeDir;
use tracing::{debug, error, info};

/// Axum HTTP backend implementation
pub struct AxumBackend;

impl AxumBackend {
    pub fn new() -> Self {
        Self
    }
}

/// Shared state for Axum handlers
#[derive(Clone)]
struct AppState {
    data_tx: mpsc::Sender<DataRequest>,
}

#[async_trait]
impl HttpBackend for AxumBackend {
    async fn run(
        &self,
        data_tx: mpsc::Sender<DataRequest>,
        config: ServerConfig,
    ) -> Result<(), Box<dyn Error>> {
        info!("🚀 Starting hegel-pm web server with Axum backend...");

        let state = AppState { data_tx };

        // Build router with API routes
        let app = Router::new()
            .route("/api/projects", get(get_projects))
            .route("/api/projects/:name/metrics", get(get_project_metrics))
            .route("/api/all-projects", get(get_all_projects))
            .nest_service("/", ServeDir::new(config.static_dir.clone()))
            .with_state(state);

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

        // Start server
        let addr = (
            Ipv4Addr::new(
                config.host[0],
                config.host[1],
                config.host[2],
                config.host[3],
            ),
            config.port,
        );
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

/// GET /api/projects - List all projects
async fn get_projects(State(state): State<AppState>) -> impl IntoResponse {
    use std::time::Instant;
    let start = Instant::now();

    let (reply_tx, reply_rx) = oneshot::channel();
    if let Err(e) = state
        .data_tx
        .send(DataRequest::GetProjects { reply: reply_tx })
        .await
    {
        error!("Failed to send GetProjects request: {}", e);
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&serde_json::json!({"error": "Internal server error"})).unwrap(),
            ))
            .unwrap();
    }

    match reply_rx.await {
        Ok(json_bytes) => {
            debug!(
                "📋 Projects list request completed in {:?}",
                start.elapsed()
            );
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(json_bytes))
                .unwrap()
        }
        Err(e) => {
            error!("Failed to receive GetProjects reply: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({"error": "Internal server error"}))
                        .unwrap(),
                ))
                .unwrap()
        }
    }
}

/// GET /api/projects/:name/metrics - Get metrics for a specific project
async fn get_project_metrics(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    use std::time::Instant;
    let start = Instant::now();

    let (reply_tx, reply_rx) = oneshot::channel();
    if let Err(e) = state
        .data_tx
        .send(DataRequest::GetProjectMetrics {
            name: name.clone(),
            reply: reply_tx,
        })
        .await
    {
        error!("Failed to send GetProjectMetrics request: {}", e);
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&serde_json::json!({"error": "Internal server error"})).unwrap(),
            ))
            .unwrap();
    }

    match reply_rx.await {
        Ok(Ok(json_bytes)) => {
            debug!(
                "📊 Metrics request for '{}' completed in {:?}",
                name,
                start.elapsed()
            );
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(json_bytes))
                .unwrap()
        }
        Ok(Err(DataError::ProjectNotFound(name))) => {
            info!("❌ Project not found: {}", name);
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({"error": "Project not found"})).unwrap(),
                ))
                .unwrap()
        }
        Ok(Err(e)) => {
            error!("❌ Error getting project metrics: {:?}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({"error": "Internal server error"}))
                        .unwrap(),
                ))
                .unwrap()
        }
        Err(e) => {
            error!("Failed to receive GetProjectMetrics reply: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({"error": "Internal server error"}))
                        .unwrap(),
                ))
                .unwrap()
        }
    }
}

/// GET /api/all-projects - Get aggregate metrics for all projects
async fn get_all_projects(State(state): State<AppState>) -> impl IntoResponse {
    use std::time::Instant;
    let start = Instant::now();

    let (reply_tx, reply_rx) = oneshot::channel();
    if let Err(e) = state
        .data_tx
        .send(DataRequest::GetAllProjects { reply: reply_tx })
        .await
    {
        error!("Failed to send GetAllProjects request: {}", e);
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&serde_json::json!({"error": "Internal server error"})).unwrap(),
            ))
            .unwrap();
    }

    match reply_rx.await {
        Ok(json_bytes) => {
            debug!(
                "📊 All-projects aggregate request completed in {:?}",
                start.elapsed()
            );
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(json_bytes))
                .unwrap()
        }
        Err(e) => {
            error!("Failed to receive GetAllProjects reply: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({"error": "Internal server error"}))
                        .unwrap(),
                ))
                .unwrap()
        }
    }
}
