//! Performance optimization module
//!
//! Provides caching layers, connection pooling patterns,
//! and performance metrics collection.

mod cache;
mod metrics;
mod pool;

pub use cache::LruCache;
pub use metrics::{PerfMetrics, PerfTimer};
pub use pool::{ConnectionPool, PoolConfig};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_reexports() {
        let cache: LruCache<String, String> = LruCache::new(10);
        assert_eq!(cache.len(), 0);
    }
}
