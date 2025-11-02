use serde::{Deserialize, Serialize};

/// Statistics for a discovered project
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectStatistics {
    /// Total events in hooks.jsonl
    pub total_events: usize,
    /// Bash command count
    pub bash_commands: usize,
    /// File modifications count
    pub file_modifications: usize,
    /// State transitions count
    pub state_transitions: usize,
}

impl ProjectStatistics {
    /// Create empty statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if statistics have been loaded
    pub fn is_empty(&self) -> bool {
        self.total_events == 0
            && self.bash_commands == 0
            && self.file_modifications == 0
            && self.state_transitions == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_statistics() {
        let stats = ProjectStatistics::new();
        assert!(stats.is_empty());
    }

    #[test]
    fn test_non_empty_statistics() {
        let stats = ProjectStatistics {
            total_events: 10,
            bash_commands: 5,
            file_modifications: 3,
            state_transitions: 2,
        };
        assert!(!stats.is_empty());
    }

    #[test]
    fn test_serialization() {
        let stats = ProjectStatistics {
            total_events: 100,
            bash_commands: 50,
            file_modifications: 30,
            state_transitions: 20,
        };

        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: ProjectStatistics = serde_json::from_str(&json).unwrap();

        assert_eq!(stats.total_events, deserialized.total_events);
        assert_eq!(stats.bash_commands, deserialized.bash_commands);
    }
}
