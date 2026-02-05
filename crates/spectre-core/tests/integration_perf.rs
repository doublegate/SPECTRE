//! Integration tests -- performance module
#![allow(clippy::disallowed_methods)]

use spectre_core::perf::{ConnectionPool, LruCache, PerfMetrics, PoolConfig};

#[test]
fn test_cache_integration() {
    let mut cache: LruCache<String, Vec<u16>> = LruCache::new(3);

    cache.insert("10.0.0.1".to_string(), vec![22, 80, 443, 8080]);
    cache.insert("10.0.0.2".to_string(), vec![22, 3306]);
    cache.insert("10.0.0.3".to_string(), vec![80]);

    assert_eq!(
        cache.get(&"10.0.0.1".to_string()),
        Some(&vec![22, 80, 443, 8080])
    );
    assert_eq!(cache.get(&"10.0.0.2".to_string()), Some(&vec![22, 3306]));

    // Adding a 4th entry should evict the least recently used
    cache.insert("10.0.0.4".to_string(), vec![5432]);

    // 10.0.0.3 was least recently used (not accessed since insert)
    assert!(cache.get(&"10.0.0.3".to_string()).is_none());
    assert!(cache.get(&"10.0.0.4".to_string()).is_some());

    let stats = cache.stats();
    assert!(stats.accesses > 0);
}

#[test]
fn test_cache_heavy_usage() {
    let mut cache: LruCache<u32, String> = LruCache::new(100);

    for i in 0..100 {
        cache.insert(i, format!("value-{}", i));
    }
    assert_eq!(cache.len(), 100);

    // Access some entries to make them recently used
    for i in (0..50).step_by(2) {
        let _ = cache.get(&i);
    }

    // Add more entries, evicting old ones
    for i in 100..150 {
        cache.insert(i, format!("value-{}", i));
    }

    assert_eq!(cache.len(), 100);

    // Recently accessed even entries should still be present
    assert!(cache.get(&0).is_some());
    assert!(cache.get(&2).is_some());
}

#[tokio::test]
async fn test_metrics_integration() {
    let metrics = PerfMetrics::new();

    for _ in 0..10 {
        metrics
            .record_timing("scan", std::time::Duration::from_millis(100))
            .await;
    }
    for _ in 0..5 {
        metrics
            .record_timing("scan", std::time::Duration::from_millis(200))
            .await;
    }

    metrics
        .record_timing("report", std::time::Duration::from_millis(500))
        .await;

    for _ in 0..42 {
        metrics.increment("hosts_scanned").await;
    }
    for _ in 0..7 {
        metrics.increment("findings_found").await;
    }

    let scan_stats = metrics.timing_stats("scan").await;
    assert!(scan_stats.is_some());
    let stats = scan_stats.unwrap();
    assert_eq!(stats.count, 15);
    assert!(stats.avg >= std::time::Duration::from_millis(100));

    assert_eq!(metrics.counter("hosts_scanned").await, 42);
    assert_eq!(metrics.counter("findings_found").await, 7);
    assert_eq!(metrics.counter("nonexistent").await, 0);
}

#[tokio::test]
async fn test_connection_pool_integration() {
    let config = PoolConfig {
        max_connections: 3,
        min_idle: 0,
        timeout_seconds: 60,
    };
    let pool: ConnectionPool<String> = ConnectionPool::new(config);

    pool.add("conn-1".to_string()).await;
    pool.add("conn-2".to_string()).await;
    pool.add("conn-3".to_string()).await;

    let c1 = pool.acquire().await;
    assert!(c1.is_some());

    let c2 = pool.acquire().await;
    assert!(c2.is_some());

    if let Some(c) = c1 {
        pool.release(c).await;
    }

    let c3 = pool.acquire().await;
    assert!(c3.is_some());
}

#[tokio::test]
async fn test_connection_pool_exhaustion() {
    let config = PoolConfig {
        max_connections: 2,
        min_idle: 0,
        timeout_seconds: 60,
    };
    let pool: ConnectionPool<String> = ConnectionPool::new(config);

    pool.add("conn-1".to_string()).await;
    pool.add("conn-2".to_string()).await;

    let _c1 = pool.acquire().await;
    let _c2 = pool.acquire().await;

    let c3 = pool.acquire().await;
    assert!(c3.is_none());
}

#[tokio::test]
async fn test_metrics_multiple_operations() {
    let metrics = PerfMetrics::new();

    metrics
        .record_timing("discovery", std::time::Duration::from_millis(1000))
        .await;
    metrics.increment("hosts_discovered").await;
    metrics.increment("hosts_discovered").await;
    metrics.increment("hosts_discovered").await;

    metrics
        .record_timing("port_scan", std::time::Duration::from_millis(5000))
        .await;

    metrics
        .record_timing("report_generation", std::time::Duration::from_millis(500))
        .await;

    assert!(metrics.timing_stats("discovery").await.is_some());
    assert!(metrics.timing_stats("port_scan").await.is_some());
    assert!(metrics.timing_stats("report_generation").await.is_some());

    assert_eq!(metrics.counter("hosts_discovered").await, 3);
}
