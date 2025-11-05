use super::cache::{CacheKey, ResponseCache};
use super::messages::{DataError, DataRequest};
use crate::api_types::{
    build_workflow_summaries, AggregateMetrics, AllProjectsAggregate, ProjectInfo,
    ProjectWorkflowDetail,
};
use crate::discovery::{
    DiscoveredProject, DiscoveryEngine, ProjectListItem, ProjectMetricsSummary,
};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// Worker pool configuration
#[derive(Debug, Clone)]
pub struct WorkerPoolConfig {
    /// Number of worker tasks (default: num_cpus * 2)
    pub worker_count: usize,
    /// Channel buffer size for requests
    pub channel_buffer: usize,
}

impl WorkerPoolConfig {
    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.worker_count == 0 {
            return Err("worker_count must be greater than 0".to_string());
        }
        if self.channel_buffer == 0 {
            return Err("channel_buffer must be greater than 0".to_string());
        }
        Ok(())
    }
}

impl Default for WorkerPoolConfig {
    fn default() -> Self {
        Self {
            worker_count: num_cpus::get() * 2,
            channel_buffer: 100,
        }
    }
}

/// Worker pool managing parallel cache updates
pub struct WorkerPool {
    request_rx: mpsc::Receiver<DataRequest>,
    cache: ResponseCache,
    discovery_engine: Arc<DiscoveryEngine>,
    projects: Arc<RwLock<Vec<DiscoveredProject>>>,
}

impl WorkerPool {
    /// Create new worker pool and return channel sender for requests
    pub fn new(
        config: WorkerPoolConfig,
        engine: Arc<DiscoveryEngine>,
    ) -> Result<(Self, mpsc::Sender<DataRequest>), String> {
        config.validate()?;

        let (tx, rx) = mpsc::channel(config.channel_buffer);
        let cache = ResponseCache::new();

        // Load initial projects
        let projects = engine
            .get_projects(false)
            .map_err(|e| format!("Failed to discover projects: {}", e))?;

        info!("📁 Discovered {} projects", projects.len());

        debug!(
            "WorkerPool initialized with {} workers, buffer size {}",
            config.worker_count, config.channel_buffer
        );

        let pool = Self {
            request_rx: rx,
            cache,
            discovery_engine: engine,
            projects: Arc::new(RwLock::new(projects)),
        };

        Ok((pool, tx))
    }

    /// Run worker pool (processes requests until channel closes)
    pub async fn run(mut self) {
        debug!("WorkerPool started, waiting for requests");

        while let Some(request) = self.request_rx.recv().await {
            match request {
                DataRequest::GetProjects { reply } => {
                    self.handle_get_projects(reply).await;
                }
                DataRequest::GetProjectMetrics { name, reply } => {
                    self.handle_get_project_metrics(name, reply).await;
                }
                DataRequest::GetAllProjects { reply } => {
                    self.handle_get_all_projects(reply).await;
                }
                DataRequest::RefreshCache { project_name } => {
                    self.handle_refresh_cache(project_name).await;
                }
            }
        }

        debug!("WorkerPool shutting down (channel closed)");
    }

    /// Handle GetProjects request with cache
    async fn handle_get_projects(&self, reply: tokio::sync::oneshot::Sender<Vec<u8>>) {
        let key = CacheKey::ProjectList;

        // Check cache first
        if let Some(cached) = self.cache.get(&key) {
            debug!("Cache hit for ProjectList");
            let _ = reply.send((*cached).clone());
            return;
        }

        // Cache miss: build project list from discovered projects
        debug!("Cache miss for ProjectList");
        let projects = self.projects.read().unwrap();
        let list_items: Vec<ProjectListItem> = projects
            .iter()
            .map(|p| ProjectListItem {
                name: p.name.clone(),
                workflow_state: p.workflow_state.clone(),
            })
            .collect();

        match serde_json::to_vec(&list_items) {
            Ok(bytes) => {
                self.cache.insert(key, bytes.clone());
                let _ = reply.send(bytes);
            }
            Err(e) => {
                error!("Failed to serialize project list: {}", e);
                let _ = reply.send(b"[]".to_vec());
            }
        }
    }

    /// Handle GetProjectMetrics request with cache
    async fn handle_get_project_metrics(
        &self,
        name: String,
        reply: tokio::sync::oneshot::Sender<Result<Vec<u8>, DataError>>,
    ) {
        let key = CacheKey::ProjectMetrics(name.clone());

        // Check cache first
        if let Some(cached) = self.cache.get(&key) {
            debug!("Cache hit for project: {}", name);
            let _ = reply.send(Ok((*cached).clone()));
            return;
        }

        // Cache miss: load metrics from disk
        debug!("Cache miss for project: {}", name);

        // Clone data we need for async work
        let projects = self.projects.clone();
        let cache = self.cache.clone();

        // Spawn async task to handle heavy I/O
        tokio::spawn(async move {
            let result = Self::load_project_metrics(&name, projects).await;
            match result {
                Ok(bytes) => {
                    // Cache the result
                    cache.insert(key, bytes.clone());
                    let _ = reply.send(Ok(bytes));
                }
                Err(e) => {
                    let _ = reply.send(Err(e));
                }
            }
        });
    }

    /// Load project metrics from disk (heavy I/O operation)
    async fn load_project_metrics(
        name: &str,
        projects: Arc<RwLock<Vec<DiscoveredProject>>>,
    ) -> Result<Vec<u8>, DataError> {
        use std::time::Instant;
        let load_start = Instant::now();

        // Find project and check if stats already loaded
        let (hegel_dir, workflow_state, needs_loading) = {
            let projects_read = projects.read().unwrap();
            match projects_read.iter().find(|p| p.name == name) {
                Some(project) => {
                    if project.has_statistics() {
                        // Stats already loaded, just serialize
                        let stats = project.statistics.as_ref().unwrap();
                        let summary = ProjectMetricsSummary::from(stats);
                        let workflows = build_workflow_summaries(stats);
                        let detail = ProjectWorkflowDetail {
                            current_workflow_state: project.workflow_state.clone(),
                            workflows,
                        };
                        let project_info = ProjectInfo {
                            project_name: name.to_string(),
                            summary,
                            detail,
                        };
                        return serde_json::to_vec(&project_info).map_err(|e| {
                            DataError::CacheError(format!("Serialization failed: {}", e))
                        });
                    } else {
                        (
                            project.hegel_dir.clone(),
                            project.workflow_state.clone(),
                            true,
                        )
                    }
                }
                None => return Err(DataError::ProjectNotFound(name.to_string())),
            }
        };

        if !needs_loading {
            return Err(DataError::CacheError("Unexpected state".to_string()));
        }

        // Load statistics (blocking I/O)
        debug!("⏳ Loading statistics for project: {}", name);
        let stats = hegel::metrics::parse_unified_metrics(&hegel_dir, true)
            .map_err(|e| DataError::ParseError(format!("Failed to parse metrics: {}", e)))?;

        info!("✅ Statistics loaded in {:?}", load_start.elapsed());

        // Update project state with loaded stats
        {
            let mut projects_write = projects.write().unwrap();
            if let Some(project) = projects_write.iter_mut().find(|p| p.name == name) {
                project.statistics = Some(stats.clone());
            }
        }

        // Build response
        let summary = ProjectMetricsSummary::from(&stats);
        let workflows = build_workflow_summaries(&stats);
        let detail = ProjectWorkflowDetail {
            current_workflow_state: workflow_state,
            workflows,
        };
        let project_info = ProjectInfo {
            project_name: name.to_string(),
            summary,
            detail,
        };

        serde_json::to_vec(&project_info)
            .map_err(|e| DataError::CacheError(format!("Serialization failed: {}", e)))
    }

    /// Handle GetAllProjects request with cache
    async fn handle_get_all_projects(&self, reply: tokio::sync::oneshot::Sender<Vec<u8>>) {
        let key = CacheKey::AllProjectsAggregate;

        // Check cache first
        if let Some(cached) = self.cache.get(&key) {
            debug!("Cache hit for AllProjectsAggregate");
            let _ = reply.send((*cached).clone());
            return;
        }

        // Cache miss: aggregate metrics from all projects
        debug!("Cache miss for AllProjectsAggregate");
        let projects = self.projects.read().unwrap();

        let mut total_input = 0u64;
        let mut total_output = 0u64;
        let mut total_cache_creation = 0u64;
        let mut total_cache_read = 0u64;
        let mut total_events = 0usize;
        let mut total_bash_commands = 0usize;
        let mut total_file_modifications = 0usize;
        let mut total_git_commits = 0usize;
        let mut total_phases = 0usize;

        // Sum metrics across all projects with statistics
        for project in projects.iter() {
            if let Some(ref stats) = project.statistics {
                total_input += stats.token_metrics.total_input_tokens;
                total_output += stats.token_metrics.total_output_tokens;
                total_cache_creation += stats.token_metrics.total_cache_creation_tokens;
                total_cache_read += stats.token_metrics.total_cache_read_tokens;
                total_events += stats.hook_metrics.total_events;
                total_bash_commands += stats.hook_metrics.bash_commands.len();
                total_file_modifications += stats.hook_metrics.file_modifications.len();
                total_git_commits += stats.git_commits.len();
                total_phases += stats.phase_metrics.len();
            }
        }

        let aggregate = AllProjectsAggregate {
            total_projects: projects.len(),
            aggregate_metrics: AggregateMetrics {
                total_input_tokens: total_input,
                total_output_tokens: total_output,
                total_cache_creation_tokens: total_cache_creation,
                total_cache_read_tokens: total_cache_read,
                total_all_tokens: total_input
                    + total_output
                    + total_cache_creation
                    + total_cache_read,
                total_events,
                bash_command_count: total_bash_commands,
                file_modification_count: total_file_modifications,
                git_commit_count: total_git_commits,
                phase_count: total_phases,
            },
        };

        match serde_json::to_vec(&aggregate) {
            Ok(bytes) => {
                self.cache.insert(key, bytes.clone());
                let _ = reply.send(bytes);
            }
            Err(e) => {
                error!("Failed to serialize aggregate: {}", e);
                let _ = reply.send(b"{}".to_vec());
            }
        }
    }

    /// Handle RefreshCache request (invalidates cache entries)
    async fn handle_refresh_cache(&self, project_name: Option<String>) {
        match project_name {
            Some(name) => {
                debug!("Invalidating cache for project: {}", name);
                self.cache.invalidate(&CacheKey::ProjectMetrics(name));
            }
            None => {
                debug!("Invalidating all cache entries");
                self.cache.invalidate(&CacheKey::ProjectList);
                self.cache.invalidate(&CacheKey::AllProjectsAggregate);
                // Note: Can't efficiently invalidate all project metrics without iteration
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::DiscoveryConfig;

    #[test]
    fn test_worker_pool_config_default() {
        let config = WorkerPoolConfig::default();
        assert!(config.worker_count > 0);
        assert_eq!(config.channel_buffer, 100);
    }

    #[test]
    fn test_worker_pool_config_validation() {
        let valid = WorkerPoolConfig {
            worker_count: 4,
            channel_buffer: 50,
        };
        assert!(valid.validate().is_ok());

        let invalid_workers = WorkerPoolConfig {
            worker_count: 0,
            channel_buffer: 50,
        };
        assert!(invalid_workers.validate().is_err());

        let invalid_buffer = WorkerPoolConfig {
            worker_count: 4,
            channel_buffer: 0,
        };
        assert!(invalid_buffer.validate().is_err());
    }

    #[tokio::test]
    async fn test_worker_pool_initialization() {
        let config = DiscoveryConfig::default();
        let engine = Arc::new(DiscoveryEngine::new(config).unwrap());

        let worker_config = WorkerPoolConfig {
            worker_count: 2,
            channel_buffer: 10,
        };

        let result = WorkerPool::new(worker_config, engine);
        assert!(result.is_ok());

        let (pool, tx) = result.unwrap();
        drop(pool); // Don't start the worker loop
        drop(tx); // Close channel
    }

    #[tokio::test]
    async fn test_worker_pool_channel_communication() {
        let config = DiscoveryConfig::default();
        let engine = Arc::new(DiscoveryEngine::new(config).unwrap());

        let worker_config = WorkerPoolConfig {
            worker_count: 2,
            channel_buffer: 10,
        };

        let (pool, tx) = WorkerPool::new(worker_config, engine).unwrap();

        // Spawn worker loop
        tokio::spawn(async move {
            pool.run().await;
        });

        // Send GetProjects request
        let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
        tx.send(DataRequest::GetProjects { reply: reply_tx })
            .await
            .unwrap();

        // Receive response
        let response = reply_rx.await.unwrap();
        // Should be valid JSON array (may or may not be empty depending on discovered projects)
        assert!(response.starts_with(b"["));
        assert!(response.ends_with(b"]"));
    }

    #[tokio::test]
    async fn test_worker_pool_handles_dropped_receiver() {
        let config = DiscoveryConfig::default();
        let engine = Arc::new(DiscoveryEngine::new(config).unwrap());

        let worker_config = WorkerPoolConfig {
            worker_count: 2,
            channel_buffer: 10,
        };

        let (pool, tx) = WorkerPool::new(worker_config, engine).unwrap();

        tokio::spawn(async move {
            pool.run().await;
        });

        // Send request but drop receiver immediately
        let (reply_tx, _reply_rx) = tokio::sync::oneshot::channel();
        tx.send(DataRequest::GetProjects { reply: reply_tx })
            .await
            .unwrap();

        // Worker should not panic (send failure is silently ignored)
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    #[tokio::test]
    async fn test_cache_hit_fast_path() {
        let config = DiscoveryConfig::default();
        let engine = Arc::new(DiscoveryEngine::new(config).unwrap());

        let worker_config = WorkerPoolConfig {
            worker_count: 2,
            channel_buffer: 10,
        };

        let (pool, tx) = WorkerPool::new(worker_config, engine).unwrap();

        tokio::spawn(async move {
            pool.run().await;
        });

        // First request (cache miss)
        let (reply_tx1, reply_rx1) = tokio::sync::oneshot::channel();
        tx.send(DataRequest::GetProjects { reply: reply_tx1 })
            .await
            .unwrap();
        let response1 = reply_rx1.await.unwrap();

        // Second request (cache hit - should be instant)
        let (reply_tx2, reply_rx2) = tokio::sync::oneshot::channel();
        tx.send(DataRequest::GetProjects { reply: reply_tx2 })
            .await
            .unwrap();
        let response2 = reply_rx2.await.unwrap();

        // Both responses should be identical (cache hit)
        assert_eq!(response1, response2);
        // Should be valid JSON array
        assert!(response1.starts_with(b"["));
        assert!(response1.ends_with(b"]"));
    }

    #[tokio::test]
    async fn test_project_metrics_cache_miss() {
        let config = DiscoveryConfig::default();
        let engine = Arc::new(DiscoveryEngine::new(config).unwrap());

        let worker_config = WorkerPoolConfig {
            worker_count: 2,
            channel_buffer: 10,
        };

        let (pool, tx) = WorkerPool::new(worker_config, engine).unwrap();

        tokio::spawn(async move {
            pool.run().await;
        });

        // Request nonexistent project
        let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
        tx.send(DataRequest::GetProjectMetrics {
            name: "nonexistent".to_string(),
            reply: reply_tx,
        })
        .await
        .unwrap();

        let result = reply_rx.await.unwrap();
        assert!(result.is_err());
        match result {
            Err(DataError::ProjectNotFound(name)) => {
                assert_eq!(name, "nonexistent");
            }
            _ => panic!("Expected ProjectNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_concurrent_requests_dont_block() {
        let config = DiscoveryConfig::default();
        let engine = Arc::new(DiscoveryEngine::new(config).unwrap());

        let worker_config = WorkerPoolConfig {
            worker_count: 2,
            channel_buffer: 10,
        };

        let (pool, tx) = WorkerPool::new(worker_config, engine).unwrap();

        tokio::spawn(async move {
            pool.run().await;
        });

        // Send multiple concurrent requests
        let mut handles = vec![];
        for i in 0..5 {
            let tx_clone = tx.clone();
            let handle = tokio::spawn(async move {
                let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
                tx_clone
                    .send(DataRequest::GetProjectMetrics {
                        name: format!("project-{}", i),
                        reply: reply_tx,
                    })
                    .await
                    .ok();
                reply_rx.await
            });
            handles.push(handle);
        }

        // All requests should complete
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
            assert!(result.unwrap().is_err()); // All are ProjectNotFound
        }
    }

    #[tokio::test]
    async fn test_refresh_cache_invalidates() {
        let config = DiscoveryConfig::default();
        let engine = Arc::new(DiscoveryEngine::new(config).unwrap());

        let worker_config = WorkerPoolConfig {
            worker_count: 2,
            channel_buffer: 10,
        };

        let (pool, tx) = WorkerPool::new(worker_config, engine).unwrap();

        tokio::spawn(async move {
            pool.run().await;
        });

        // Populate cache
        let (reply_tx1, reply_rx1) = tokio::sync::oneshot::channel();
        tx.send(DataRequest::GetProjects { reply: reply_tx1 })
            .await
            .unwrap();
        let _ = reply_rx1.await.unwrap();

        // Invalidate cache
        tx.send(DataRequest::RefreshCache { project_name: None })
            .await
            .unwrap();

        // Give worker time to process
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Next request should be cache miss (but will reload and cache again)
        let (reply_tx2, reply_rx2) = tokio::sync::oneshot::channel();
        tx.send(DataRequest::GetProjects { reply: reply_tx2 })
            .await
            .unwrap();
        let response = reply_rx2.await.unwrap();

        // Should be valid JSON array
        assert!(response.starts_with(b"["));
        assert!(response.ends_with(b"]"));
    }
}
