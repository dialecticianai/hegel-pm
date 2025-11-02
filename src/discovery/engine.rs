use anyhow::Result;

use super::{discover_projects, load_cache, save_cache, DiscoveredProject, DiscoveryConfig};

/// Discovery engine that orchestrates project discovery with caching
pub struct DiscoveryEngine {
    config: DiscoveryConfig,
}

impl DiscoveryEngine {
    /// Create a new discovery engine with configuration
    pub fn new(config: DiscoveryConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Get projects, using cache if available or scanning if not
    pub fn get_projects(&self, force_refresh: bool) -> Result<Vec<DiscoveredProject>> {
        if force_refresh {
            // Force refresh bypasses cache
            return self.scan_and_cache();
        }

        // Try to load from cache
        match load_cache(&self.config.cache_location)? {
            Some(projects) => Ok(projects),
            None => {
                // No cache, perform scan
                self.scan_and_cache()
            }
        }
    }

    /// Scan for projects and update cache
    pub fn scan_and_cache(&self) -> Result<Vec<DiscoveredProject>> {
        let projects = discover_projects(&self.config)?;
        save_cache(&projects, &self.config.cache_location)?;
        Ok(projects)
    }

    /// Get configuration
    pub fn config(&self) -> &DiscoveryConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_workspace() -> TempDir {
        let temp = TempDir::new().unwrap();
        let project = temp.path().join("test-project");
        fs::create_dir_all(&project).unwrap();
        fs::create_dir(project.join(".hegel")).unwrap();
        temp
    }

    #[test]
    fn test_engine_creation() {
        let temp = TempDir::new().unwrap();
        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        let engine = DiscoveryEngine::new(config);
        assert!(engine.is_ok());
    }

    #[test]
    fn test_engine_invalid_config() {
        let temp = TempDir::new().unwrap();
        let config = DiscoveryConfig::new(
            vec![], // Empty roots - invalid
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        let engine = DiscoveryEngine::new(config);
        assert!(engine.is_err());
    }

    #[test]
    fn test_get_projects_no_cache() {
        let temp = create_test_workspace();
        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        let engine = DiscoveryEngine::new(config).unwrap();
        let projects = engine.get_projects(false).unwrap();

        assert_eq!(projects.len(), 1);
        // Cache should now exist
        assert!(temp.path().join("cache.json").exists());
    }

    #[test]
    fn test_get_projects_from_cache() {
        let temp = create_test_workspace();
        let cache_file = temp.path().join("cache.json");
        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            cache_file.clone(),
        );

        let engine = DiscoveryEngine::new(config).unwrap();

        // First call creates cache
        let projects1 = engine.get_projects(false).unwrap();
        assert_eq!(projects1.len(), 1);

        // Second call should use cache
        let projects2 = engine.get_projects(false).unwrap();
        assert_eq!(projects2.len(), 1);
    }

    #[test]
    fn test_force_refresh() {
        let temp = create_test_workspace();
        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        let engine = DiscoveryEngine::new(config).unwrap();

        // Initial scan
        let projects1 = engine.get_projects(false).unwrap();
        assert_eq!(projects1.len(), 1);

        // Add another project
        let project2 = temp.path().join("project2");
        fs::create_dir_all(&project2).unwrap();
        fs::create_dir(project2.join(".hegel")).unwrap();

        // Without force refresh, should still return cached 1 project
        let projects2 = engine.get_projects(false).unwrap();
        assert_eq!(projects2.len(), 1);

        // With force refresh, should find 2 projects
        let projects3 = engine.get_projects(true).unwrap();
        assert_eq!(projects3.len(), 2);
    }

    #[test]
    fn test_scan_and_cache() {
        let temp = create_test_workspace();
        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        let engine = DiscoveryEngine::new(config).unwrap();
        let projects = engine.scan_and_cache().unwrap();

        assert_eq!(projects.len(), 1);
        assert!(temp.path().join("cache.json").exists());
    }
}
