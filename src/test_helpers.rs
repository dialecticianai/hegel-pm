use std::fs;
use tempfile::TempDir;

/// Create a test workspace with multiple projects for integration testing
pub fn create_test_workspace() -> TempDir {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_workspace() {
        let temp = create_test_workspace();
        let root = temp.path();

        // Verify project1 exists
        assert!(root.join("project1/.hegel").exists());

        // Verify nested project2 exists
        assert!(root.join("nested/project2/.hegel").exists());

        // Verify deep nested project3 exists
        assert!(root.join("deep/nested/project3/.hegel").exists());

        // Verify excluded project4 exists
        assert!(root.join("excluded/node_modules/project4/.hegel").exists());

        // Verify no_hegel directory exists without .hegel
        assert!(root.join("no_hegel").exists());
        assert!(!root.join("no_hegel/.hegel").exists());
    }
}
