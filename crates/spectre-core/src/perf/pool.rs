//! Connection pooling patterns

use std::collections::VecDeque;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::debug;

/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Maximum number of connections
    pub max_connections: usize,
    /// Minimum idle connections to maintain
    pub min_idle: usize,
    /// Connection timeout in seconds
    pub timeout_seconds: u64,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_idle: 2,
            timeout_seconds: 30,
        }
    }
}

/// A generic connection pool
///
/// Manages a pool of reusable connections/resources.
#[derive(Debug)]
pub struct ConnectionPool<T> {
    config: PoolConfig,
    available: Arc<Mutex<VecDeque<T>>>,
    in_use: Arc<Mutex<usize>>,
}

impl<T: Send + 'static> ConnectionPool<T> {
    /// Create a new connection pool
    pub fn new(config: PoolConfig) -> Self {
        Self {
            config,
            available: Arc::new(Mutex::new(VecDeque::new())),
            in_use: Arc::new(Mutex::new(0)),
        }
    }

    /// Add a connection to the pool
    pub async fn add(&self, conn: T) {
        let mut available = self.available.lock().await;
        available.push_back(conn);
        debug!(available = available.len(), "Connection added to pool");
    }

    /// Acquire a connection from the pool
    pub async fn acquire(&self) -> Option<T> {
        let mut available = self.available.lock().await;
        if let Some(conn) = available.pop_front() {
            let mut in_use = self.in_use.lock().await;
            *in_use += 1;
            debug!(
                available = available.len(),
                in_use = *in_use,
                "Connection acquired"
            );
            Some(conn)
        } else {
            None
        }
    }

    /// Release a connection back to the pool
    pub async fn release(&self, conn: T) {
        let mut available = self.available.lock().await;
        let mut in_use = self.in_use.lock().await;

        if *in_use > 0 {
            *in_use -= 1;
        }

        if available.len() < self.config.max_connections {
            available.push_back(conn);
        }
        // If pool is full, drop the connection

        debug!(
            available = available.len(),
            in_use = *in_use,
            "Connection released"
        );
    }

    /// Get the number of available connections
    pub async fn available_count(&self) -> usize {
        self.available.lock().await.len()
    }

    /// Get the number of in-use connections
    pub async fn in_use_count(&self) -> usize {
        *self.in_use.lock().await
    }

    /// Get the pool configuration
    pub fn config(&self) -> &PoolConfig {
        &self.config
    }

    /// Drain all connections from the pool
    pub async fn drain(&self) -> Vec<T> {
        let mut available = self.available.lock().await;
        available.drain(..).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_config_default() {
        let config = PoolConfig::default();
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_idle, 2);
        assert_eq!(config.timeout_seconds, 30);
    }

    #[tokio::test]
    async fn test_pool_add_and_acquire() {
        let pool = ConnectionPool::new(PoolConfig::default());
        pool.add("conn-1".to_string()).await;
        pool.add("conn-2".to_string()).await;

        assert_eq!(pool.available_count().await, 2);

        let conn = pool.acquire().await;
        assert!(conn.is_some());
        assert_eq!(conn.unwrap(), "conn-1");
        assert_eq!(pool.available_count().await, 1);
        assert_eq!(pool.in_use_count().await, 1);
    }

    #[tokio::test]
    async fn test_pool_acquire_empty() {
        let pool: ConnectionPool<String> = ConnectionPool::new(PoolConfig::default());
        let conn = pool.acquire().await;
        assert!(conn.is_none());
    }

    #[tokio::test]
    async fn test_pool_release() {
        let pool = ConnectionPool::new(PoolConfig::default());
        pool.add("conn-1".to_string()).await;

        let conn = pool.acquire().await.unwrap();
        assert_eq!(pool.available_count().await, 0);

        pool.release(conn).await;
        assert_eq!(pool.available_count().await, 1);
        assert_eq!(pool.in_use_count().await, 0);
    }

    #[tokio::test]
    async fn test_pool_drain() {
        let pool = ConnectionPool::new(PoolConfig::default());
        pool.add(1).await;
        pool.add(2).await;
        pool.add(3).await;

        let drained = pool.drain().await;
        assert_eq!(drained.len(), 3);
        assert_eq!(pool.available_count().await, 0);
    }

    #[test]
    fn test_pool_config_serialization() {
        let config = PoolConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: PoolConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.max_connections, 10);
    }
}
