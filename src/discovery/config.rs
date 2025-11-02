use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for project discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Root directories to scan for Hegel projects
    pub root_directories: Vec<PathBuf>,
    /// Maximum recursion depth
    pub max_depth: usize,
    /// Directory names to exclude from scanning
    pub exclusions: Vec<String>,
    /// Cache file location
    pub cache_location: PathBuf,
}

impl DiscoveryConfig {
    /// Create a new configuration with custom values
    pub fn new(
        root_directories: Vec<PathBuf>,
        max_depth: usize,
        exclusions: Vec<String>,
        cache_location: PathBuf,
    ) -> Self {
        Self {
            root_directories,
            max_depth,
            exclusions,
            cache_location,
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // At least one root directory required
        if self.root_directories.is_empty() {
            bail!("At least one root directory must be provided");
        }

        // Check all root directories exist and are readable
        for root in &self.root_directories {
            if !root.exists() {
                bail!("Root directory does not exist: {}", root.display());
            }

            if !root.is_dir() {
                bail!("Root path is not a directory: {}", root.display());
            }

            // Try to read the directory to verify permissions
            std::fs::read_dir(root).context(format!(
                "Root directory is not readable: {}",
                root.display()
            ))?;
        }

        // Max depth must be at least 1
        if self.max_depth < 1 {
            bail!("Max depth must be at least 1, got {}", self.max_depth);
        }

        // Verify cache location parent directory is writable
        if let Some(parent) = self.cache_location.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).context(format!(
                    "Cannot create cache directory: {}",
                    parent.display()
                ))?;
            }

            // Test writability by creating a temp file
            let test_file = parent.join(".hegel-pm-write-test");
            std::fs::write(&test_file, b"test")
                .context(format!("Cache location not writable: {}", parent.display()))?;
            std::fs::remove_file(test_file).ok();
        }

        Ok(())
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let code_dir = home.join("Code");

        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| home.join(".config"))
            .join("hegel-pm");

        Self {
            root_directories: vec![code_dir],
            max_depth: 10,
            exclusions: vec![
                "node_modules".to_string(),
                "target".to_string(),
                ".git".to_string(),
                "vendor".to_string(),
            ],
            cache_location: config_dir.join("cache.json"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = DiscoveryConfig::default();
        assert!(!config.root_directories.is_empty());
        assert_eq!(config.max_depth, 10);
        assert_eq!(config.exclusions.len(), 4);
        assert!(config.exclusions.contains(&"node_modules".to_string()));
    }

    #[test]
    fn test_config_creation() {
        let temp = TempDir::new().unwrap();
        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            5,
            vec!["test".to_string()],
            temp.path().join("cache.json"),
        );

        assert_eq!(config.root_directories.len(), 1);
        assert_eq!(config.max_depth, 5);
        assert_eq!(config.exclusions.len(), 1);
    }

    #[test]
    fn test_validation_empty_roots() {
        let temp = TempDir::new().unwrap();
        let config = DiscoveryConfig::new(vec![], 10, vec![], temp.path().join("cache.json"));

        let result = config.validate();
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("at least one root") || err_msg.contains("At least one root"),
            "Expected error about root directory, got: {}",
            err_msg
        );
    }

    #[test]
    fn test_validation_nonexistent_root() {
        let temp = TempDir::new().unwrap();
        let nonexistent = temp.path().join("does-not-exist");
        let config = DiscoveryConfig::new(
            vec![nonexistent],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_validation_invalid_max_depth() {
        let temp = TempDir::new().unwrap();
        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            0,
            vec![],
            temp.path().join("cache.json"),
        );

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least 1"));
    }

    #[test]
    fn test_validation_success() {
        let temp = TempDir::new().unwrap();
        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec!["node_modules".to_string()],
            temp.path().join("cache.json"),
        );

        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_serialization() {
        let temp = TempDir::new().unwrap();
        let config = DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec!["test".to_string()],
            temp.path().join("cache.json"),
        );

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: DiscoveryConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.root_directories, deserialized.root_directories);
        assert_eq!(config.max_depth, deserialized.max_depth);
        assert_eq!(config.exclusions, deserialized.exclusions);
    }
}
