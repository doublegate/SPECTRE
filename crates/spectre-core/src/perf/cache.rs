//! LRU cache for DNS lookups, service fingerprints, etc.

use std::collections::HashMap;
use std::hash::Hash;

/// A simple LRU (Least Recently Used) cache
#[derive(Debug)]
pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, (V, u64)>,
    access_counter: u64,
}

impl<K: Eq + Hash + Clone, V: Clone> LruCache<K, V> {
    /// Create a new LRU cache with the given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            map: HashMap::with_capacity(capacity),
            access_counter: 0,
        }
    }

    /// Get a value from the cache
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.access_counter += 1;
            if let Some(entry) = self.map.get_mut(key) {
                entry.1 = self.access_counter;
                return Some(&entry.0);
            }
        }
        None
    }

    /// Insert a value into the cache
    pub fn insert(&mut self, key: K, value: V) {
        if self.map.len() >= self.capacity && !self.map.contains_key(&key) {
            self.evict_lru();
        }

        self.access_counter += 1;
        self.map.insert(key, (value, self.access_counter));
    }

    /// Remove a value from the cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|(v, _)| v)
    }

    /// Check if the cache contains a key
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Get the number of items in the cache
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Get the capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.map.clear();
        self.access_counter = 0;
    }

    /// Evict the least recently used item
    fn evict_lru(&mut self) {
        if let Some(lru_key) = self
            .map
            .iter()
            .min_by_key(|(_, (_, counter))| *counter)
            .map(|(k, _)| k.clone())
        {
            self.map.remove(&lru_key);
        }
    }

    /// Get cache hit statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            size: self.map.len(),
            capacity: self.capacity,
            accesses: self.access_counter,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Current number of items
    pub size: usize,
    /// Maximum capacity
    pub capacity: usize,
    /// Total accesses
    pub accesses: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_new() {
        let cache: LruCache<String, String> = LruCache::new(10);
        assert_eq!(cache.capacity(), 10);
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_insert_and_get() {
        let mut cache = LruCache::new(10);
        cache.insert("key1", "value1");
        assert_eq!(cache.get(&"key1"), Some(&"value1"));
    }

    #[test]
    fn test_get_nonexistent() {
        let mut cache: LruCache<&str, &str> = LruCache::new(10);
        assert!(cache.get(&"nonexistent").is_none());
    }

    #[test]
    fn test_contains_key() {
        let mut cache = LruCache::new(10);
        cache.insert("key1", "value1");
        assert!(cache.contains_key(&"key1"));
        assert!(!cache.contains_key(&"key2"));
    }

    #[test]
    fn test_remove() {
        let mut cache = LruCache::new(10);
        cache.insert("key1", "value1");
        let removed = cache.remove(&"key1");
        assert_eq!(removed, Some("value1"));
        assert!(!cache.contains_key(&"key1"));
    }

    #[test]
    fn test_eviction() {
        let mut cache = LruCache::new(3);
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        // Access "a" to make it recently used
        cache.get(&"a");

        // Insert "d" — should evict "b" (least recently used)
        cache.insert("d", 4);

        assert_eq!(cache.len(), 3);
        assert!(cache.contains_key(&"a")); // Recently accessed
        assert!(!cache.contains_key(&"b")); // Evicted
        assert!(cache.contains_key(&"c"));
        assert!(cache.contains_key(&"d"));
    }

    #[test]
    fn test_update_existing() {
        let mut cache = LruCache::new(10);
        cache.insert("key", "v1");
        cache.insert("key", "v2");
        assert_eq!(cache.get(&"key"), Some(&"v2"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut cache = LruCache::new(10);
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = LruCache::new(10);
        cache.insert("a", 1);
        cache.get(&"a");
        cache.get(&"a");

        let stats = cache.stats();
        assert_eq!(stats.size, 1);
        assert_eq!(stats.capacity, 10);
        assert!(stats.accesses > 0);
    }

    #[test]
    fn test_minimum_capacity() {
        let cache: LruCache<&str, &str> = LruCache::new(0);
        assert_eq!(cache.capacity(), 1);
    }

    #[test]
    fn test_string_keys() {
        let mut cache = LruCache::new(10);
        cache.insert("dns:example.com".to_string(), "93.184.216.34".to_string());
        assert!(cache.contains_key(&"dns:example.com".to_string()));
    }
}
