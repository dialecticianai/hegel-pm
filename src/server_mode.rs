use hegel_pm::data_layer::{WorkerPool, WorkerPoolConfig};
use hegel_pm::discovery::DiscoveryEngine;
use hegel_pm::http::{HttpBackend, ServerConfig};
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

/// Start web server with project discovery API and static file serving
pub async fn run(engine: &DiscoveryEngine) -> Result<(), Box<dyn Error>> {
    info!("🚀 Starting hegel-pm web server...");
    info!(
        "📍 Cache location: {}",
        engine.config().cache_location.display()
    );

    // Create worker pool with default configuration
    let config = WorkerPoolConfig::default();

    // Create a new engine with same config (engine is cheap to create)
    let engine_arc = Arc::new(DiscoveryEngine::new(engine.config().clone())?);
    let (pool, data_tx) = WorkerPool::new(config, engine_arc)?;

    // Spawn worker pool in background
    tokio::spawn(async move {
        pool.run().await;
    });

    // Create server configuration
    let server_config = ServerConfig::new([127, 0, 0, 1], 3030, PathBuf::from("./static"), true);

    // Select backend based on feature flags
    #[cfg(feature = "warp-backend")]
    let backend = {
        use hegel_pm::http::warp_backend::WarpBackend;
        WarpBackend::new()
    };

    #[cfg(feature = "axum-backend")]
    let backend = {
        use hegel_pm::http::axum_backend::AxumBackend;
        AxumBackend::new()
    };

    // Run the backend
    backend.run(data_tx, server_config).await?;

    Ok(())
}
