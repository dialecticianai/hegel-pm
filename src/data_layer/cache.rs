use dashmap::DashMap;
use std::sync::Arc;

/// Cache key identifying different API endpoints
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CacheKey {
    ProjectList,
    ProjectMetrics(String),
    AllProjectsAggregate,
}

/// Shared cache storing pre-serialized JSON responses
#[derive(Clone)]
pub struct ResponseCache {
    /// Map: cache key -> pre-serialized JSON bytes
    cache: Arc<DashMap<CacheKey, Arc<Vec<u8>>>>,
}

impl ResponseCache {
    /// Create new empty cache
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    /// Get cached response bytes
    pub fn get(&self, key: &CacheKey) -> Option<Arc<Vec<u8>>> {
        self.cache.get(key).map(|entry| entry.value().clone())
    }

    /// Insert pre-serialized response into cache
    pub fn insert(&self, key: CacheKey, bytes: Vec<u8>) {
        self.cache.insert(key, Arc::new(bytes));
    }

    /// Invalidate cache entry
    pub fn invalidate(&self, key: &CacheKey) {
        self.cache.remove(key);
    }

    /// Clear all cache entries
    #[allow(dead_code)]
    pub fn clear(&self) {
        self.cache.clear();
    }

    /// Get cache size
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.cache.len()
    }
}

impl Default for ResponseCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_insert_and_get() {
        let cache = ResponseCache::new();
        let key = CacheKey::ProjectList;
        let data = vec![1, 2, 3, 4, 5];

        cache.insert(key.clone(), data.clone());

        let retrieved = cache.get(&key).expect("Cache entry should exist");
        assert_eq!(*retrieved, data);
    }

    #[test]
    fn test_cache_get_nonexistent() {
        let cache = ResponseCache::new();
        let key = CacheKey::ProjectMetrics("nonexistent".to_string());

        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn test_cache_invalidate() {
        let cache = ResponseCache::new();
        let key = CacheKey::AllProjectsAggregate;
        let data = vec![1, 2, 3];

        cache.insert(key.clone(), data);
        assert!(cache.get(&key).is_some());

        cache.invalidate(&key);
        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn test_cache_multiple_entries() {
        let cache = ResponseCache::new();

        cache.insert(CacheKey::ProjectList, vec![1]);
        cache.insert(CacheKey::ProjectMetrics("proj1".to_string()), vec![2]);
        cache.insert(CacheKey::AllProjectsAggregate, vec![3]);

        assert_eq!(cache.len(), 3);
        assert!(cache.get(&CacheKey::ProjectList).is_some());
        assert!(cache
            .get(&CacheKey::ProjectMetrics("proj1".to_string()))
            .is_some());
        assert!(cache.get(&CacheKey::AllProjectsAggregate).is_some());
    }

    #[test]
    fn test_cache_arc_sharing() {
        let cache = ResponseCache::new();
        let key = CacheKey::ProjectList;
        let data = vec![1, 2, 3, 4, 5];

        cache.insert(key.clone(), data);

        // Get multiple references to same data
        let ref1 = cache.get(&key).unwrap();
        let ref2 = cache.get(&key).unwrap();

        // Both should point to same Arc (cheap clone)
        assert_eq!(ref1, ref2);
        assert_eq!(Arc::strong_count(&ref1), Arc::strong_count(&ref2));
    }

    #[test]
    fn test_cache_key_equality() {
        let key1 = CacheKey::ProjectMetrics("test".to_string());
        let key2 = CacheKey::ProjectMetrics("test".to_string());
        let key3 = CacheKey::ProjectMetrics("other".to_string());

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}
