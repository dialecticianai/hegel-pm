use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use super::DiscoveredProject;

/// Save discovered projects to cache
pub fn save_cache(projects: &[DiscoveredProject], cache_location: &PathBuf) -> Result<()> {
    // Create parent directory if needed
    if let Some(parent) = cache_location.parent() {
        fs::create_dir_all(parent).context(format!(
            "Failed to create cache directory: {}",
            parent.display()
        ))?;
    }

    // Serialize projects to JSON
    let json = serde_json::to_string_pretty(projects).context("Failed to serialize projects")?;

    // Atomic write: write to temp file, then rename
    let temp_file = cache_location.with_extension("json.tmp");
    fs::write(&temp_file, json).context(format!(
        "Failed to write temp cache file: {}",
        temp_file.display()
    ))?;

    fs::rename(&temp_file, cache_location).context(format!(
        "Failed to rename cache file: {}",
        cache_location.display()
    ))?;

    Ok(())
}

/// Load discovered projects from cache
pub fn load_cache(cache_location: &PathBuf) -> Result<Option<Vec<DiscoveredProject>>> {
    if !cache_location.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(cache_location).context(format!(
        "Failed to read cache file: {}",
        cache_location.display()
    ))?;

    let projects: Vec<DiscoveredProject> =
        serde_json::from_str(&content).context("Failed to parse cache file")?;

    Ok(Some(projects))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;
    use tempfile::TempDir;

    fn create_test_project(name: &str) -> DiscoveredProject {
        let temp = TempDir::new().unwrap();
        DiscoveredProject::new(
            name.to_string(),
            temp.path().to_path_buf(),
            temp.path().join(".hegel"),
            None,
            SystemTime::now(),
            None,
        )
    }

    #[test]
    fn test_save_and_load_cache() {
        let temp = TempDir::new().unwrap();
        let cache_file = temp.path().join("cache.json");

        let projects = vec![
            create_test_project("project1"),
            create_test_project("project2"),
        ];

        // Save cache
        save_cache(&projects, &cache_file).unwrap();

        // Load cache
        let loaded = load_cache(&cache_file).unwrap();
        assert!(loaded.is_some());

        let loaded_projects = loaded.unwrap();
        assert_eq!(loaded_projects.len(), 2);
        assert_eq!(loaded_projects[0].name, "project1");
        assert_eq!(loaded_projects[1].name, "project2");
    }

    #[test]
    fn test_load_nonexistent_cache() {
        let temp = TempDir::new().unwrap();
        let cache_file = temp.path().join("does-not-exist.json");

        let loaded = load_cache(&cache_file).unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_load_corrupted_cache() {
        let temp = TempDir::new().unwrap();
        let cache_file = temp.path().join("cache.json");

        fs::write(&cache_file, "not valid json").unwrap();

        let result = load_cache(&cache_file);
        assert!(result.is_err());
    }

    #[test]
    fn test_save_creates_parent_directory() {
        let temp = TempDir::new().unwrap();
        let cache_file = temp.path().join("nested").join("cache.json");

        let projects = vec![create_test_project("project1")];

        save_cache(&projects, &cache_file).unwrap();

        assert!(cache_file.exists());
    }

    #[test]
    fn test_atomic_write() {
        let temp = TempDir::new().unwrap();
        let cache_file = temp.path().join("cache.json");

        // Write initial cache
        let projects1 = vec![create_test_project("project1")];
        save_cache(&projects1, &cache_file).unwrap();

        // Overwrite with new cache
        let projects2 = vec![
            create_test_project("project1"),
            create_test_project("project2"),
        ];
        save_cache(&projects2, &cache_file).unwrap();

        // Load should get the new cache
        let loaded = load_cache(&cache_file).unwrap().unwrap();
        assert_eq!(loaded.len(), 2);
    }
}
