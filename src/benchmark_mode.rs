use anyhow::{Context, Result};
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info};

/// Benchmark results for a single endpoint
#[derive(Debug, Clone, Serialize)]
pub struct EndpointBenchmark {
    /// Endpoint path (e.g., "/api/projects")
    pub path: String,
    /// Average response time in milliseconds
    pub avg_ms: f64,
    /// Number of iterations
    pub iterations: usize,
}

/// Benchmark results for a project-specific endpoint
#[derive(Debug, Clone, Serialize)]
pub struct ProjectBenchmark {
    /// Project name
    pub project_name: String,
    /// Average response time in milliseconds
    pub avg_ms: f64,
    /// Number of iterations
    pub iterations: usize,
}

/// Complete benchmark results
#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkResults {
    /// Backend name (warp or axum)
    pub backend: String,
    /// Endpoint: /api/projects
    pub projects_list: EndpointBenchmark,
    /// Endpoint: /api/all-projects
    pub all_projects: EndpointBenchmark,
    /// Endpoint: /api/projects/:name/metrics (one per discovered project)
    pub project_metrics: Vec<ProjectBenchmark>,
}

/// Wait for server to become ready by polling endpoint
async fn wait_for_server_ready(url: &str, timeout_secs: u64) -> Result<()> {
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    let client = reqwest::Client::new();

    info!("⏳ Waiting for server to become ready at {}", url);

    loop {
        if start.elapsed() > timeout {
            anyhow::bail!(
                "Server did not become ready within {} seconds",
                timeout_secs
            );
        }

        match client.get(url).send().await {
            Ok(response) if response.status().is_success() => {
                debug!("✅ Server ready");
                return Ok(());
            }
            Ok(_) | Err(_) => {
                // Connection refused or non-success status, retry
                sleep(Duration::from_millis(100)).await;
            }
        }
    }
}

/// Run benchmarks for HTTP endpoints
pub async fn run(_iterations: usize, _output_json: bool) -> Result<()> {
    // Wait for server to be ready
    wait_for_server_ready("http://127.0.0.1:3030/api/projects", 10)
        .await
        .context("Failed to wait for server readiness")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_benchmark_creation() {
        let benchmark = EndpointBenchmark {
            path: "/api/projects".to_string(),
            avg_ms: 12.5,
            iterations: 100,
        };

        assert_eq!(benchmark.path, "/api/projects");
        assert_eq!(benchmark.avg_ms, 12.5);
        assert_eq!(benchmark.iterations, 100);
    }

    #[test]
    fn test_project_benchmark_creation() {
        let benchmark = ProjectBenchmark {
            project_name: "hegel-cli".to_string(),
            avg_ms: 23.1,
            iterations: 100,
        };

        assert_eq!(benchmark.project_name, "hegel-cli");
        assert_eq!(benchmark.avg_ms, 23.1);
        assert_eq!(benchmark.iterations, 100);
    }

    #[test]
    fn test_benchmark_results_creation() {
        let results = BenchmarkResults {
            backend: "warp".to_string(),
            projects_list: EndpointBenchmark {
                path: "/api/projects".to_string(),
                avg_ms: 12.5,
                iterations: 100,
            },
            all_projects: EndpointBenchmark {
                path: "/api/all-projects".to_string(),
                avg_ms: 45.3,
                iterations: 100,
            },
            project_metrics: vec![
                ProjectBenchmark {
                    project_name: "hegel-cli".to_string(),
                    avg_ms: 23.1,
                    iterations: 100,
                },
                ProjectBenchmark {
                    project_name: "hegel-pm".to_string(),
                    avg_ms: 18.7,
                    iterations: 100,
                },
            ],
        };

        assert_eq!(results.backend, "warp");
        assert_eq!(results.project_metrics.len(), 2);
    }

    #[test]
    fn test_json_serialization() {
        let results = BenchmarkResults {
            backend: "warp".to_string(),
            projects_list: EndpointBenchmark {
                path: "/api/projects".to_string(),
                avg_ms: 12.5,
                iterations: 100,
            },
            all_projects: EndpointBenchmark {
                path: "/api/all-projects".to_string(),
                avg_ms: 45.3,
                iterations: 100,
            },
            project_metrics: vec![ProjectBenchmark {
                project_name: "hegel-cli".to_string(),
                avg_ms: 23.1,
                iterations: 100,
            }],
        };

        let json = serde_json::to_string(&results).expect("Failed to serialize");
        assert!(json.contains("\"backend\":\"warp\""));
        assert!(json.contains("\"path\":\"/api/projects\""));
        assert!(json.contains("\"project_name\":\"hegel-cli\""));
        assert!(json.contains("\"avg_ms\":12.5"));
    }

    #[tokio::test]
    async fn test_wait_for_server_timeout() {
        // This should timeout quickly since there's no server on port 9999
        let result = wait_for_server_ready("http://127.0.0.1:9999/api/projects", 1).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("did not become ready"));
    }
}
