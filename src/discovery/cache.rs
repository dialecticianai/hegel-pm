//! Project discovery cache implementation
//!
//! Dual-cache strategy:
//! - **Binary cache** (CLI): Multi-file structure at `~/.config/hegel-pm/cache/` with `index.bin` + per-project `.bin` files
//! - **JSON cache** (Server): Single file at `~/.config/hegel-pm/cache.json` for data_layer compatibility
//!
//! Note: "Binary" cache uses JSON serialization (not bincode) due to `InvalidBoolEncoding` errors with `DiscoveredProject`.
//! Multi-file structure enables future incremental updates.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use super::DiscoveredProject;

/// Lightweight index entry for fast project listing without loading full project data
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

/// Save discovered projects to binary cache (multi-file: index.bin + per-project files)
///
/// Index written last to ensure consistency. Project write failures logged as warnings but don't abort.
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
            eprintln!("Failed to write project '{}': {}", project.name, e);
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

    // Write index last to ensure consistency (atomic write)
    write_index(&index, &cache_dir)?;

    Ok(())
}

/// Load discovered projects from binary cache
///
/// Returns `Ok(None)` if cache missing, `Err` if index corrupted. Missing/corrupted project files skipped with warnings.
pub fn load_binary_cache(
    config: &super::DiscoveryConfig,
) -> Result<Option<Vec<DiscoveredProject>>> {
    let cache_dir = config.cache_dir();

    let index = match read_index(&cache_dir)? {
        Some(idx) => idx,
        None => return Ok(None), // Cache miss
    };

    // Skip missing/corrupted project files, continue with valid ones
    let mut projects = Vec::new();
    for entry in index {
        match read_project(&entry.name, &cache_dir) {
            Ok(Some(project)) => projects.push(project),
            Ok(None) => {
                eprintln!("Project file missing for: {}", entry.name);
            }
            Err(e) => {
                eprintln!("Failed to load project '{}': {}", entry.name, e);
            }
        }
    }

    Ok(Some(projects))
}

/// Remove a project from the binary cache (both index and project file)
///
/// Returns `Ok(true)` if project was found and removed, `Ok(false)` if project not in cache.
pub fn remove_from_cache(project_name: &str, config: &super::DiscoveryConfig) -> Result<bool> {
    let cache_dir = config.cache_dir();

    // Load current index
    let mut index = match read_index(&cache_dir)? {
        Some(idx) => idx,
        None => return Ok(false), // No cache, nothing to remove
    };

    // Check if project exists in index
    let original_len = index.len();
    index.retain(|entry| entry.name != project_name);

    if index.len() == original_len {
        // Project not found in index
        return Ok(false);
    }

    // Write updated index (atomic)
    write_index(&index, &cache_dir)?;

    // Delete individual project file (best effort, don't fail if already gone)
    let safe_name =
        project_name.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_");
    let project_path = cache_dir.join(format!("{}.bin", safe_name));
    if project_path.exists() {
        fs::remove_file(&project_path).ok(); // Ignore errors
    }

    Ok(true)
}

/// Refresh all projects in the cache (rediscover and update each one)
///
/// Returns count of successfully refreshed projects.
pub fn refresh_all_projects(config: &super::DiscoveryConfig) -> Result<usize> {
    let cache_dir = config.cache_dir();

    // Load current index
    let index = match read_index(&cache_dir)? {
        Some(idx) => idx,
        None => {
            anyhow::bail!("No cache found. Run 'hegel-pm discover list' first to populate cache.")
        }
    };

    if index.is_empty() {
        return Ok(0);
    }

    let mut refreshed_count = 0;
    let mut errors = Vec::new();

    for entry in &index {
        match refresh_project(&entry.name, config) {
            Ok(_) => refreshed_count += 1,
            Err(e) => errors.push(format!("  - {}: {}", entry.name, e)),
        }
    }

    if !errors.is_empty() {
        eprintln!("\nWarnings during refresh:");
        for error in &errors {
            eprintln!("{}", error);
        }
    }

    Ok(refreshed_count)
}

/// Refresh a single project in the cache (rediscover and update)
///
/// Returns `Ok(true)` if project was found and refreshed, error if not in cache or path invalid.
pub fn refresh_project(project_name: &str, config: &super::DiscoveryConfig) -> Result<bool> {
    let cache_dir = config.cache_dir();

    // Load current index
    let mut index = match read_index(&cache_dir)? {
        Some(idx) => idx,
        None => {
            anyhow::bail!("No cache found. Run 'hegel-pm discover list' first to populate cache.")
        }
    };

    // Find project in index
    let project_entry = index
        .iter()
        .find(|e| e.name == project_name)
        .ok_or_else(|| anyhow::anyhow!("Project '{}' not found in cache", project_name))?;

    let project_path = project_entry.project_path.clone();
    let hegel_dir = project_path.join(".hegel");

    // Verify .hegel directory exists
    if !hegel_dir.exists() {
        anyhow::bail!(
            "Project '{}' not found at cached path: {}\nUse 'hegel-pm remove {}' if you want to stop tracking it.",
            project_name,
            project_path.display(),
            project_name
        );
    }

    // Rediscover the project (same logic as discover_projects but for one project)
    let (workflow_state, error) = match super::load_state(&hegel_dir) {
        Ok(state) => (state, None),
        Err(e) => (None, Some(format!("Failed to load state: {}", e))),
    };

    let last_activity = super::DiscoveredProject::calculate_last_activity(&hegel_dir)
        .unwrap_or(SystemTime::UNIX_EPOCH);

    let refreshed_project = super::DiscoveredProject::new(
        project_name.to_string(),
        project_path.clone(),
        hegel_dir.clone(),
        workflow_state,
        last_activity,
        error,
    );

    // Update index entry with new last_activity
    for entry in index.iter_mut() {
        if entry.name == project_name {
            entry.last_activity = last_activity;
            entry.project_path = project_path.clone();
            entry.hegel_dir = hegel_dir.clone();
            break;
        }
    }

    // Write updated index
    write_index(&index, &cache_dir)?;

    // Write refreshed project file
    write_project(&refreshed_project, &cache_dir)?;

    Ok(true)
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

    #[test]
    fn test_remove_from_cache_existing_project() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![dirs::home_dir().unwrap().join("Code")],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        // Discover and cache projects
        let engine = super::super::DiscoveryEngine::new(config.clone()).unwrap();
        let projects = engine.get_projects(true).unwrap();

        if projects.is_empty() {
            return;
        }

        // Save to cache
        save_binary_cache(&projects, &config).unwrap();

        // Remove first project
        let project_to_remove = &projects[0].name;
        let removed = remove_from_cache(project_to_remove, &config).unwrap();
        assert!(removed);

        // Load cache and verify project is gone
        let loaded = load_binary_cache(&config).unwrap().unwrap();
        assert_eq!(loaded.len(), projects.len() - 1);
        assert!(!loaded.iter().any(|p| p.name == *project_to_remove));

        // Verify project file is deleted
        let cache_dir = config.cache_dir();
        let safe_name =
            project_to_remove.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_");
        assert!(!cache_dir.join(format!("{}.bin", safe_name)).exists());
    }

    #[test]
    fn test_remove_from_cache_nonexistent_project() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![dirs::home_dir().unwrap().join("Code")],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        // Discover and cache projects
        let engine = super::super::DiscoveryEngine::new(config.clone()).unwrap();
        let projects = engine.get_projects(true).unwrap();

        if projects.is_empty() {
            return;
        }

        // Save to cache
        save_binary_cache(&projects, &config).unwrap();

        // Try to remove project that doesn't exist
        let removed = remove_from_cache("nonexistent-project", &config).unwrap();
        assert!(!removed);

        // Load cache and verify nothing changed
        let loaded = load_binary_cache(&config).unwrap().unwrap();
        assert_eq!(loaded.len(), projects.len());
    }

    #[test]
    fn test_remove_from_cache_no_cache() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("config").join("cache.json"),
        );

        // Try to remove from non-existent cache
        let removed = remove_from_cache("some-project", &config).unwrap();
        assert!(!removed);
    }

    #[test]
    fn test_remove_from_cache_with_special_chars() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![dirs::home_dir().unwrap().join("Code")],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        // Discover real project and rename it with special chars for testing
        let engine = super::super::DiscoveryEngine::new(config.clone()).unwrap();
        let projects = engine.get_projects(true).unwrap();

        if projects.is_empty() {
            return;
        }

        let mut project = projects[0].clone();
        project.name = "project/with:special*chars".to_string();

        let projects_with_special = vec![project.clone()];
        save_binary_cache(&projects_with_special, &config).unwrap();

        // Remove project with special characters
        let removed = remove_from_cache(&project.name, &config).unwrap();
        assert!(removed);

        // Verify it's gone
        let loaded = load_binary_cache(&config).unwrap().unwrap();
        assert_eq!(loaded.len(), 0);
    }

    #[test]
    fn test_refresh_project_existing() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![dirs::home_dir().unwrap().join("Code")],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        // Discover and cache projects
        let engine = super::super::DiscoveryEngine::new(config.clone()).unwrap();
        let projects = engine.get_projects(true).unwrap();

        if projects.is_empty() {
            return;
        }

        // Save to cache
        save_binary_cache(&projects, &config).unwrap();

        let project_to_refresh = &projects[0].name;

        // Refresh the project
        let refreshed = refresh_project(project_to_refresh, &config).unwrap();
        assert!(refreshed);

        // Load cache and verify project still exists with updated data
        let loaded = load_binary_cache(&config).unwrap().unwrap();
        assert_eq!(loaded.len(), projects.len());
        assert!(loaded.iter().any(|p| p.name == *project_to_refresh));
    }

    #[test]
    fn test_refresh_project_not_in_cache() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![dirs::home_dir().unwrap().join("Code")],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        // Discover and cache projects
        let engine = super::super::DiscoveryEngine::new(config.clone()).unwrap();
        let projects = engine.get_projects(true).unwrap();

        if projects.is_empty() {
            return;
        }

        // Save to cache
        save_binary_cache(&projects, &config).unwrap();

        // Try to refresh project that doesn't exist in cache
        let result = refresh_project("nonexistent-project", &config);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not found in cache"));
    }

    #[test]
    fn test_refresh_project_no_cache() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![temp.path().to_path_buf()],
            10,
            vec![],
            temp.path().join("config").join("cache.json"),
        );

        // Try to refresh from non-existent cache
        let result = refresh_project("some-project", &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No cache found"));
    }

    #[test]
    fn test_refresh_project_missing_hegel_dir() {
        let temp = TempDir::new().unwrap();
        let config = super::super::DiscoveryConfig::new(
            vec![dirs::home_dir().unwrap().join("Code")],
            10,
            vec![],
            temp.path().join("cache.json"),
        );

        // Discover and cache projects
        let engine = super::super::DiscoveryEngine::new(config.clone()).unwrap();
        let projects = engine.get_projects(true).unwrap();

        if projects.is_empty() {
            return;
        }

        // Create a fake project with non-existent path
        let mut fake_project = projects[0].clone();
        fake_project.name = "fake-project".to_string();
        fake_project.project_path = temp.path().join("nonexistent");
        fake_project.hegel_dir = temp.path().join("nonexistent/.hegel");

        let projects_with_fake = vec![fake_project];
        save_binary_cache(&projects_with_fake, &config).unwrap();

        // Try to refresh project with missing .hegel directory
        let result = refresh_project("fake-project", &config);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("not found at cached path"));
        assert!(err_msg.contains("Use 'hegel-pm remove"));
    }
}
