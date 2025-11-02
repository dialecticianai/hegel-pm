use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Find all .hegel directories in the given root, respecting exclusions and max depth
pub fn find_hegel_directories(
    root: &PathBuf,
    max_depth: usize,
    exclusions: &[String],
) -> Result<Vec<PathBuf>> {
    let mut found = Vec::new();

    for entry in WalkDir::new(root)
        .max_depth(max_depth)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            // Skip excluded directories
            if let Some(name) = e.file_name().to_str() {
                !exclusions.contains(&name.to_string())
            } else {
                true
            }
        })
    {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                // Log error but continue scanning
                eprintln!("Warning: skipping entry: {}", err);
                continue;
            }
        };

        // Check if this is a .hegel directory
        if entry.file_type().is_dir() && entry.file_name() == ".hegel" {
            // Get the parent directory (the project root)
            if let Some(parent) = entry.path().parent() {
                found.push(parent.to_path_buf());
            }
        }
    }

    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_workspace() -> TempDir {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create nested project structure
        // project1/ with .hegel
        let project1 = root.join("project1");
        fs::create_dir_all(&project1).unwrap();
        fs::create_dir(project1.join(".hegel")).unwrap();

        // nested/project2/ with .hegel
        let project2 = root.join("nested").join("project2");
        fs::create_dir_all(&project2).unwrap();
        fs::create_dir(project2.join(".hegel")).unwrap();

        // deep/nested/project3/ with .hegel
        let project3 = root.join("deep").join("nested").join("project3");
        fs::create_dir_all(&project3).unwrap();
        fs::create_dir(project3.join(".hegel")).unwrap();

        // excluded/node_modules/project4/ with .hegel (should be skipped)
        let project4 = root.join("excluded").join("node_modules").join("project4");
        fs::create_dir_all(&project4).unwrap();
        fs::create_dir(project4.join(".hegel")).unwrap();

        // no_hegel/ without .hegel
        fs::create_dir(root.join("no_hegel")).unwrap();

        temp
    }

    #[test]
    fn test_find_single_project() {
        let temp = TempDir::new().unwrap();
        let project = temp.path().join("test-project");
        fs::create_dir(&project).unwrap();
        fs::create_dir(project.join(".hegel")).unwrap();

        let found = find_hegel_directories(&temp.path().to_path_buf(), 10, &[]).unwrap();

        assert_eq!(found.len(), 1);
        assert_eq!(found[0], project);
    }

    #[test]
    fn test_find_nested_projects() {
        let temp = create_test_workspace();

        let found = find_hegel_directories(&temp.path().to_path_buf(), 10, &[]).unwrap();

        // Should find project1, project2, project3, and project4
        assert_eq!(found.len(), 4);
    }

    #[test]
    fn test_max_depth_limit() {
        let temp = create_test_workspace();

        // Max depth 2 should find project1 (at depth 1)
        // project2 is at nested/project2 (depth 2)
        // project3 is too deep
        let found = find_hegel_directories(&temp.path().to_path_buf(), 3, &[]).unwrap();

        // Should find project1 and project2, not project3
        assert!(found.len() >= 2);
        assert!(found.iter().any(|p| p.ends_with("project1")));
        assert!(found.iter().any(|p| p.ends_with("project2")));
    }

    #[test]
    fn test_exclusions() {
        let temp = create_test_workspace();

        let exclusions = vec!["node_modules".to_string()];
        let found = find_hegel_directories(&temp.path().to_path_buf(), 10, &exclusions).unwrap();

        // Should find project1, project2, project3 (not project4 in node_modules)
        assert_eq!(found.len(), 3);
        assert!(!found
            .iter()
            .any(|p| p.to_string_lossy().contains("project4")));
    }

    #[test]
    fn test_empty_directory() {
        let temp = TempDir::new().unwrap();

        let found = find_hegel_directories(&temp.path().to_path_buf(), 10, &[]).unwrap();

        assert_eq!(found.len(), 0);
    }

    #[test]
    fn test_no_symlinks() {
        let temp = TempDir::new().unwrap();

        // Create a project with .hegel
        let project = temp.path().join("project");
        fs::create_dir(&project).unwrap();
        fs::create_dir(project.join(".hegel")).unwrap();

        // Create a symlink to the project
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let link = temp.path().join("link");
            symlink(&project, &link).ok();
        }

        let found = find_hegel_directories(&temp.path().to_path_buf(), 10, &[]).unwrap();

        // Should only find the original project, not through the symlink
        assert_eq!(found.len(), 1);
        assert_eq!(found[0], project);
    }

    #[test]
    fn test_multiple_exclusions() {
        let temp = TempDir::new().unwrap();

        // Create projects in various excluded directories
        let in_node_modules = temp.path().join("node_modules").join("proj1");
        fs::create_dir_all(&in_node_modules).unwrap();
        fs::create_dir(in_node_modules.join(".hegel")).unwrap();

        let in_target = temp.path().join("target").join("proj2");
        fs::create_dir_all(&in_target).unwrap();
        fs::create_dir(in_target.join(".hegel")).unwrap();

        let in_git = temp.path().join(".git").join("proj3");
        fs::create_dir_all(&in_git).unwrap();
        fs::create_dir(in_git.join(".hegel")).unwrap();

        let valid = temp.path().join("valid");
        fs::create_dir(&valid).unwrap();
        fs::create_dir(valid.join(".hegel")).unwrap();

        let exclusions = vec![
            "node_modules".to_string(),
            "target".to_string(),
            ".git".to_string(),
        ];
        let found = find_hegel_directories(&temp.path().to_path_buf(), 10, &exclusions).unwrap();

        // Should only find the valid project
        assert_eq!(found.len(), 1);
        assert_eq!(found[0], valid);
    }
}
