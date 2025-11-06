use crate::discovery::DiscoveryEngine;
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

/// Benchmark a single endpoint with multiple iterations
async fn benchmark_endpoint(url: &str, iterations: usize) -> Result<EndpointBenchmark> {
    let client = reqwest::Client::new();

    // Warmup request
    debug!("🔥 Warmup request to {}", url);
    client.get(url).send().await?.error_for_status()?;

    // Timed iterations
    let mut total_ms = 0.0;
    for i in 0..iterations {
        let start = std::time::Instant::now();
        client.get(url).send().await?.error_for_status()?;
        let elapsed = start.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
        total_ms += elapsed;

        if (i + 1) % 10 == 0 {
            debug!("  Completed {} iterations", i + 1);
        }
    }

    let avg_ms = total_ms / iterations as f64;
    debug!("✅ Average: {:.2}ms over {} iterations", avg_ms, iterations);

    Ok(EndpointBenchmark {
        path: url.to_string(),
        avg_ms,
        iterations,
    })
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

/// Benchmark project-specific metrics endpoints
async fn benchmark_project_metrics(
    base_url: &str,
    project_names: &[String],
    iterations: usize,
) -> Result<Vec<ProjectBenchmark>> {
    let mut results = Vec::new();

    for project_name in project_names {
        info!("📊 Benchmarking /api/projects/{}/metrics...", project_name);
        let url = format!("{}/api/projects/{}/metrics", base_url, project_name);

        let endpoint_result = benchmark_endpoint(&url, iterations)
            .await
            .with_context(|| format!("Failed to benchmark project {}", project_name))?;

        results.push(ProjectBenchmark {
            project_name: project_name.clone(),
            avg_ms: endpoint_result.avg_ms,
            iterations: endpoint_result.iterations,
        });
    }

    Ok(results)
}

/// Run benchmarks for HTTP endpoints
pub async fn run(engine: &DiscoveryEngine, iterations: usize, _output_json: bool) -> Result<()> {
    let base_url = "http://127.0.0.1:3030";

    // Wait for server to be ready
    info!("🚀 Starting HTTP endpoint benchmarks");
    wait_for_server_ready(&format!("{}/api/projects", base_url), 10)
        .await
        .context("Failed to wait for server readiness")?;

    // Get discovered projects
    let projects = engine.get_projects(false)?;
    let project_names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
    info!("📋 Found {} projects to benchmark", project_names.len());

    // Benchmark /api/projects endpoint
    info!("📊 Benchmarking /api/projects...");
    let _projects_list = benchmark_endpoint(&format!("{}/api/projects", base_url), iterations)
        .await
        .context("Failed to benchmark /api/projects")?;

    // Benchmark /api/all-projects endpoint
    info!("📊 Benchmarking /api/all-projects...");
    let _all_projects = benchmark_endpoint(&format!("{}/api/all-projects", base_url), iterations)
        .await
        .context("Failed to benchmark /api/all-projects")?;

    // Benchmark per-project metrics
    let _project_metrics = benchmark_project_metrics(base_url, &project_names, iterations)
        .await
        .context("Failed to benchmark project metrics")?;

    info!("✅ Benchmarks complete");

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

    #[tokio::test]
    async fn test_benchmark_endpoint_error_on_bad_url() {
        // Should fail when server not running
        let result = benchmark_endpoint("http://127.0.0.1:9999/api/projects", 5).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_benchmark_project_metrics_error() {
        // Should fail when server not running
        let project_names = vec!["test-project".to_string()];
        let result = benchmark_project_metrics("http://127.0.0.1:9999", &project_names, 5).await;
        assert!(result.is_err());
    }
}
