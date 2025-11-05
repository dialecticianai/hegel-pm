use crate::data_layer::messages::DataRequest;
use async_trait::async_trait;
use std::error::Error;
use std::path::PathBuf;
use tokio::sync::mpsc;

/// Common interface all HTTP backends must implement
#[async_trait]
pub trait HttpBackend {
    /// Start server with data layer handle
    async fn run(
        &self,
        data_tx: mpsc::Sender<DataRequest>,
        config: ServerConfig,
    ) -> Result<(), Box<dyn Error>>;
}

/// Server configuration (backend-agnostic)
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: [u8; 4],
    pub port: u16,
    pub static_dir: PathBuf,
    pub open_browser: bool,
}

impl ServerConfig {
    /// Create new server configuration with validation
    pub fn new(host: [u8; 4], port: u16, static_dir: PathBuf, open_browser: bool) -> Self {
        Self {
            host,
            port,
            static_dir,
            open_browser,
        }
    }

    /// Get server URL as string
    pub fn url(&self) -> String {
        format!(
            "http://{}.{}.{}.{}:{}",
            self.host[0], self.host[1], self.host[2], self.host[3], self.port
        )
    }
}

// Compile-time mutual exclusion check
#[cfg(all(feature = "warp-backend", feature = "axum-backend"))]
compile_error!("Cannot enable both warp-backend and axum-backend features simultaneously");

// Conditional compilation for backend implementations
#[cfg(feature = "warp-backend")]
pub mod warp_backend;

#[cfg(feature = "axum-backend")]
pub mod axum_backend;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_creation() {
        let config = ServerConfig::new([127, 0, 0, 1], 3030, PathBuf::from("./static"), true);
        assert_eq!(config.host, [127, 0, 0, 1]);
        assert_eq!(config.port, 3030);
        assert_eq!(config.static_dir, PathBuf::from("./static"));
        assert_eq!(config.open_browser, true);
    }

    #[test]
    fn test_server_config_url() {
        let config = ServerConfig::new([127, 0, 0, 1], 3030, PathBuf::from("./static"), true);
        assert_eq!(config.url(), "http://127.0.0.1:3030");
    }

    #[test]
    fn test_server_config_different_port() {
        let config = ServerConfig::new([0, 0, 0, 0], 8080, PathBuf::from("./public"), false);
        assert_eq!(config.url(), "http://0.0.0.0:8080");
        assert_eq!(config.open_browser, false);
    }
}
