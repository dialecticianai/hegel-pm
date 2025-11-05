use tokio::sync::oneshot;

/// Request sent from HTTP layer to data layer
#[derive(Debug)]
pub enum DataRequest {
    GetProjects {
        reply: oneshot::Sender<Vec<u8>>,
    },
    GetProjectMetrics {
        name: String,
        reply: oneshot::Sender<Result<Vec<u8>, DataError>>,
    },
    GetAllProjects {
        reply: oneshot::Sender<Vec<u8>>,
    },
    // TODO: Wire up file watching for cache invalidation
    #[allow(dead_code)]
    RefreshCache {
        project_name: Option<String>,
    },
}

/// Errors from data layer operations
#[derive(Debug, Clone)]
pub enum DataError {
    ProjectNotFound(String),
    ParseError(String),
    CacheError(String),
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::ProjectNotFound(name) => write!(f, "Project not found: {}", name),
            DataError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DataError::CacheError(msg) => write!(f, "Cache error: {}", msg),
        }
    }
}

impl std::error::Error for DataError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_error_display() {
        let not_found = DataError::ProjectNotFound("test-project".to_string());
        assert_eq!(not_found.to_string(), "Project not found: test-project");

        let parse_error = DataError::ParseError("invalid JSON".to_string());
        assert_eq!(parse_error.to_string(), "Parse error: invalid JSON");

        let cache_error = DataError::CacheError("write failed".to_string());
        assert_eq!(cache_error.to_string(), "Cache error: write failed");
    }

    #[test]
    fn test_data_error_clone() {
        let error = DataError::ProjectNotFound("test".to_string());
        let cloned = error.clone();
        assert_eq!(error.to_string(), cloned.to_string());
    }
}
