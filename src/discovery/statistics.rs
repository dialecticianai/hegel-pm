// Type alias to hegel-cli's UnifiedMetrics
pub use hegel::metrics::UnifiedMetrics as ProjectStatistics;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_statistics() {
        let stats = ProjectStatistics::default();
        assert_eq!(stats.hook_metrics.total_events, 0);
        assert!(stats.phase_metrics.is_empty());
        assert!(stats.session_id.is_none());
    }

    #[test]
    fn test_serialization() {
        let mut stats = ProjectStatistics::default();
        stats.session_id = Some("test-session".to_string());
        stats.token_metrics.total_input_tokens = 1000;
        stats.hook_metrics.total_events = 100;

        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: ProjectStatistics = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.session_id, Some("test-session".to_string()));
        assert_eq!(deserialized.token_metrics.total_input_tokens, 1000);
        assert_eq!(deserialized.hook_metrics.total_events, 100);
    }
}
