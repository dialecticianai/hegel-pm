use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use super::DiscoveredProject;

/// Lightweight index entry for fast project listing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectIndexEntry {
    pub name: String,
    pub project_path: PathBuf,
    pub hegel_dir: PathBuf,
    pub last_activity: SystemTime,
}

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

/// Write index to binary file with atomic write
fn write_index(index: &[ProjectIndexEntry], cache_dir: &PathBuf) -> Result<()> {
    // Ensure cache directory exists
    fs::create_dir_all(cache_dir).context(format!(
        "Failed to create cache directory: {}",
        cache_dir.display()
    ))?;

    let index_path = cache_dir.join("index.bin");
    let temp_path = cache_dir.join("index.bin.tmp");

    // Serialize to JSON (bincode has issues with DiscoveredProject types)
    let encoded = serde_json::to_vec(index).context("Failed to serialize index")?;

    // Atomic write
    fs::write(&temp_path, encoded).context(format!(
        "Failed to write temp index file: {}",
        temp_path.display()
    ))?;

    fs::rename(&temp_path, &index_path).context(format!(
        "Failed to rename index file: {}",
        index_path.display()
    ))?;

    Ok(())
}

/// Read index from binary file using memmap
fn read_index(cache_dir: &PathBuf) -> Result<Option<Vec<ProjectIndexEntry>>> {
    let index_path = cache_dir.join("index.bin");

    if !index_path.exists() {
        return Ok(None);
    }

    // Read file contents
    let contents = fs::read(&index_path).context(format!(
        "Failed to read index file: {}",
        index_path.display()
    ))?;

    // Deserialize from JSON
    let index: Vec<ProjectIndexEntry> =
        serde_json::from_slice(&contents).context("Failed to deserialize index")?;

    Ok(Some(index))
}

/// Write individual project to binary file with atomic write
fn write_project(project: &DiscoveredProject, cache_dir: &PathBuf) -> Result<()> {
    // Sanitize project name for filename
    let safe_name = project
        .name
        .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_");

    let project_path = cache_dir.join(format!("{}.bin", safe_name));
    let temp_path = cache_dir.join(format!("{}.bin.tmp", safe_name));

    // Clear statistics and workflow_state before caching (lazy loaded/can be re-parsed)
    let mut project_copy = project.clone();
    project_copy.statistics = None;
    project_copy.workflow_state = None;

    // Serialize to JSON
    let encoded = serde_json::to_vec(&project_copy).context("Failed to serialize project")?;

    // Atomic write
    fs::write(&temp_path, encoded).context(format!(
        "Failed to write temp project file: {}",
        temp_path.display()
    ))?;

    fs::rename(&temp_path, &project_path).context(format!(
        "Failed to rename project file: {}",
        project_path.display()
    ))?;

    Ok(())
}

/// Read individual project from binary file using memmap
fn read_project(name: &str, cache_dir: &PathBuf) -> Result<Option<DiscoveredProject>> {
    // Sanitize project name for filename
    let safe_name = name.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_");

    let project_path = cache_dir.join(format!("{}.bin", safe_name));

    if !project_path.exists() {
        return Ok(None);
    }

    // Read file contents
    let contents = fs::read(&project_path).context(format!(
        "Failed to read project file: {}",
        project_path.display()
    ))?;

    // Deserialize from JSON
    let project: DiscoveredProject =
        serde_json::from_slice(&contents).context("Failed to deserialize project")?;

    Ok(Some(project))
}

/// Save discovered projects to binary cache
pub fn save_binary_cache(
    projects: &[DiscoveredProject],
    config: &super::DiscoveryConfig,
) -> Result<()> {
    let cache_dir = config.cache_dir();

    // Create cache directory if needed
    fs::create_dir_all(&cache_dir).context(format!(
        "Failed to create cache directory: {}",
        cache_dir.display()
    ))?;

    // Write each project file (skip failures, log warnings)
    for project in projects {
        if let Err(e) = write_project(project, &cache_dir) {
            tracing::warn!("Failed to write project '{}': {}", project.name, e);
        }
    }

    // Build index from all projects
    let index: Vec<ProjectIndexEntry> = projects
        .iter()
        .map(|p| ProjectIndexEntry {
            name: p.name.clone(),
            project_path: p.project_path.clone(),
            hegel_dir: p.hegel_dir.clone(),
            last_activity: p.last_activity,
        })
        .collect();

    // Write index file (atomic)
    write_index(&index, &cache_dir)?;

    Ok(())
}

/// Load discovered projects from binary cache
pub fn load_binary_cache(
    config: &super::DiscoveryConfig,
) -> Result<Option<Vec<DiscoveredProject>>> {
    let cache_dir = config.cache_dir();

    // Read index
    let index = match read_index(&cache_dir)? {
        Some(idx) => idx,
        None => return Ok(None),
    };

    // Load each project from index
    let mut projects = Vec::new();
    for entry in index {
        match read_project(&entry.name, &cache_dir) {
            Ok(Some(project)) => projects.push(project),
            Ok(None) => {
                tracing::warn!("Project file missing for: {}", entry.name);
            }
            Err(e) => {
                tracing::warn!("Failed to load project '{}': {}", entry.name, e);
            }
        }
    }

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

    #[test]
    fn test_discovered_project_json_serialization() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![dirs::home_dir().unwrap().join("Code")],
            10,
            vec![],
            temp.path().join("cache.json"),
        );
        let engine = super::super::DiscoveryEngine::new(config).unwrap();
        let projects = engine.get_projects(true).unwrap();

        if projects.is_empty() {
            return;
        }

        let mut project = projects[0].clone();
        // Clear fields that aren't cached
        project.statistics = None;
        project.workflow_state = None;

        // Test JSON serialization (used by cache)
        let encoded = serde_json::to_vec(&project).unwrap();

        // Write to file and read back to simulate real usage
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("test.bin");
        fs::write(&test_file, &encoded).unwrap();
        let read_back = fs::read(&test_file).unwrap();

        let decoded: DiscoveredProject = serde_json::from_slice(&read_back).unwrap();

        assert_eq!(project.name, decoded.name);
    }

    #[test]
    fn test_project_index_entry_serialization() {
        let temp = TempDir::new().unwrap();
        let entry = ProjectIndexEntry {
            name: "test-project".to_string(),
            project_path: temp.path().to_path_buf(),
            hegel_dir: temp.path().join(".hegel"),
            last_activity: SystemTime::now(),
        };

        // Test JSON serialization round-trip
        let encoded = serde_json::to_vec(&entry).unwrap();
        let decoded: ProjectIndexEntry = serde_json::from_slice(&encoded).unwrap();

        assert_eq!(entry.name, decoded.name);
        assert_eq!(entry.project_path, decoded.project_path);
        assert_eq!(entry.hegel_dir, decoded.hegel_dir);
        assert_eq!(entry.last_activity, decoded.last_activity);
    }

    #[test]
    fn test_build_index_from_projects() {
        let projects = vec![
            create_test_project("project1"),
            create_test_project("project2"),
        ];

        let index: Vec<ProjectIndexEntry> = projects
            .iter()
            .map(|p| ProjectIndexEntry {
                name: p.name.clone(),
                project_path: p.project_path.clone(),
                hegel_dir: p.hegel_dir.clone(),
                last_activity: p.last_activity,
            })
            .collect();

        assert_eq!(index.len(), 2);
        assert_eq!(index[0].name, "project1");
        assert_eq!(index[1].name, "project2");
    }

    #[test]
    fn test_write_and_read_index() {
        let temp = TempDir::new().unwrap();
        let cache_dir = temp.path().join("cache");

        let index = vec![
            ProjectIndexEntry {
                name: "project1".to_string(),
                project_path: temp.path().join("project1"),
                hegel_dir: temp.path().join("project1/.hegel"),
                last_activity: SystemTime::now(),
            },
            ProjectIndexEntry {
                name: "project2".to_string(),
                project_path: temp.path().join("project2"),
                hegel_dir: temp.path().join("project2/.hegel"),
                last_activity: SystemTime::now(),
            },
        ];

        // Write index
        write_index(&index, &cache_dir).unwrap();

        // Verify file exists
        assert!(cache_dir.join("index.bin").exists());

        // Read index back
        let loaded_index = read_index(&cache_dir).unwrap().unwrap();
        assert_eq!(loaded_index.len(), 2);
        assert_eq!(loaded_index[0].name, "project1");
        assert_eq!(loaded_index[1].name, "project2");
    }

    #[test]
    fn test_read_index_missing() {
        let temp = TempDir::new().unwrap();
        let cache_dir = temp.path().join("cache");

        let result = read_index(&cache_dir).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_write_and_read_project() {
        let temp = TempDir::new().unwrap();
        let cache_dir = temp.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();

        let config = super::super::DiscoveryConfig::default();
        let engine = super::super::DiscoveryEngine::new(config).unwrap();
        let projects = engine.get_projects(true).unwrap();

        if projects.is_empty() {
            return;
        }

        let project = &projects[0];

        // Write project
        write_project(project, &cache_dir).unwrap();

        // Verify file exists (sanitized name)
        let safe_name = project
            .name
            .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_");
        assert!(cache_dir.join(format!("{}.bin", safe_name)).exists());

        // Read project back
        let loaded_project = read_project(&project.name, &cache_dir).unwrap().unwrap();
        assert_eq!(loaded_project.name, project.name);
        assert!(loaded_project.statistics.is_none());
    }

    #[test]
    fn test_read_project_missing() {
        let temp = TempDir::new().unwrap();
        let cache_dir = temp.path().join("cache");

        let result = read_project("nonexistent", &cache_dir).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_project_name_sanitization() {
        let temp = TempDir::new().unwrap();
        let cache_dir = temp.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();

        let config = super::super::DiscoveryConfig::new(
            vec![dirs::home_dir().unwrap().join("Code")],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        // Discover real project and rename it with bad chars for testing
        let engine = super::super::DiscoveryEngine::new(config).unwrap();
        let projects = engine.get_projects(true).unwrap();

        if projects.is_empty() {
            return;
        }

        let mut project = projects[0].clone();
        project.name = "bad/name:here".to_string();

        // Write project with special characters
        write_project(&project, &cache_dir).unwrap();

        // Should sanitize to safe filename
        assert!(cache_dir.join("bad_name_here.bin").exists());

        // Should still be able to read it back
        let loaded = read_project("bad/name:here", &cache_dir).unwrap().unwrap();
        assert_eq!(loaded.name, "bad/name:here");
    }

    #[test]
    fn test_save_and_load_binary_cache() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![dirs::home_dir().unwrap().join("Code")],
            10,
            vec!["node_modules".to_string(), "target".to_string()],
            temp.path().join("cache.json"),
        );

        // Discover real projects from filesystem
        let engine = super::super::DiscoveryEngine::new(config.clone()).unwrap();
        let projects = engine.get_projects(true).unwrap(); // force refresh

        if projects.is_empty() {
            // Skip test if no projects found
            return;
        }

        // Save binary cache
        save_binary_cache(&projects, &config).unwrap();

        // Verify cache directory and files exist
        let cache_dir = config.cache_dir();
        assert!(cache_dir.join("index.bin").exists());

        // Load binary cache
        let loaded = load_binary_cache(&config).unwrap().unwrap();
        assert_eq!(loaded.len(), projects.len());
        assert_eq!(loaded[0].name, projects[0].name);
    }

    #[test]
    fn test_load_binary_cache_missing() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("config").join("cache.json"),
        );

        let result = load_binary_cache(&config).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_binary_cache_handles_missing_project_file() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![dirs::home_dir().unwrap().join("Code")],
            10,
            vec!["node_modules".to_string(), "target".to_string()],
            temp.path().join("cache.json"),
        );

        // Discover real projects
        let engine = super::super::DiscoveryEngine::new(config.clone()).unwrap();
        let projects = engine.get_projects(true).unwrap();

        if projects.len() < 2 {
            // Skip test if not enough projects
            return;
        }

        // Save binary cache
        save_binary_cache(&projects, &config).unwrap();

        // Delete one project file
        let cache_dir = config.cache_dir();
        let first_project_name = projects[0]
            .name
            .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_");
        fs::remove_file(cache_dir.join(format!("{}.bin", first_project_name))).unwrap();

        // Load should skip missing file but return others
        let loaded = load_binary_cache(&config).unwrap().unwrap();
        assert_eq!(loaded.len(), projects.len() - 1);
    }

    #[test]
    fn test_binary_cache_empty_projects() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("config").join("cache.json"),
        );

        let projects: Vec<DiscoveredProject> = vec![];

        // Save empty cache
        save_binary_cache(&projects, &config).unwrap();

        // Load should return empty vec
        let loaded = load_binary_cache(&config).unwrap().unwrap();
        assert_eq!(loaded.len(), 0);
    }
}
