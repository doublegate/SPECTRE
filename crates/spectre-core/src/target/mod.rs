//! Enhanced target management module
//!
//! Provides advanced target handling including:
//! - CIDR range expansion and enumeration
//! - Async DNS resolution
//! - Priority queue for target ordering
//! - Scope enforcement (allowed/blocked networks)
//! - Target deduplication
//! - Target file parsing
//! - Status tracking for each target

mod file;
mod queue;
mod scope;

use std::fmt;
use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use file::parse_target_file;
pub use queue::{TargetIterator, TargetQueue};
pub use scope::ScopeEnforcer;

/// Enhanced target with metadata and status tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedTarget {
    /// The underlying address (IP or hostname)
    pub address: TargetAddress,
    /// Priority (higher = scanned first, default 0)
    pub priority: i32,
    /// Current status
    pub status: TargetStatus,
    /// When this target was added
    pub added_at: DateTime<Utc>,
    /// When scanning started
    pub started_at: Option<DateTime<Utc>>,
    /// When scanning completed
    pub completed_at: Option<DateTime<Utc>>,
    /// Optional notes
    pub notes: Option<String>,
}

/// Target address types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetAddress {
    /// Single IP address
    Ip(IpAddr),
    /// Hostname (unresolved)
    Hostname(String),
}

impl fmt::Display for TargetAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ip(ip) => write!(f, "{}", ip),
            Self::Hostname(host) => write!(f, "{}", host),
        }
    }
}

/// Target processing status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetStatus {
    /// Waiting to be scanned
    Pending,
    /// Currently being scanned
    Active,
    /// Scan complete
    Complete,
    /// Skipped (out of scope, duplicate, etc.)
    Skipped,
    /// Error during scanning
    Error,
}

impl fmt::Display for TargetStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Active => write!(f, "active"),
            Self::Complete => write!(f, "complete"),
            Self::Skipped => write!(f, "skipped"),
            Self::Error => write!(f, "error"),
        }
    }
}

impl EnhancedTarget {
    /// Create a new enhanced target from an IP address
    pub fn from_ip(ip: IpAddr) -> Self {
        Self {
            address: TargetAddress::Ip(ip),
            priority: 0,
            status: TargetStatus::Pending,
            added_at: Utc::now(),
            started_at: None,
            completed_at: None,
            notes: None,
        }
    }

    /// Create a new enhanced target from a hostname
    pub fn from_hostname(hostname: &str) -> Self {
        Self {
            address: TargetAddress::Hostname(hostname.to_string()),
            priority: 0,
            status: TargetStatus::Pending,
            added_at: Utc::now(),
            started_at: None,
            completed_at: None,
            notes: None,
        }
    }

    /// Set the priority for this target
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Mark this target as active (scanning started)
    pub fn mark_active(&mut self) {
        self.status = TargetStatus::Active;
        self.started_at = Some(Utc::now());
    }

    /// Mark this target as complete
    pub fn mark_complete(&mut self) {
        self.status = TargetStatus::Complete;
        self.completed_at = Some(Utc::now());
    }

    /// Mark this target as skipped
    pub fn mark_skipped(&mut self, reason: &str) {
        self.status = TargetStatus::Skipped;
        self.notes = Some(reason.to_string());
    }

    /// Mark this target as errored
    pub fn mark_error(&mut self, error: &str) {
        self.status = TargetStatus::Error;
        self.notes = Some(error.to_string());
        self.completed_at = Some(Utc::now());
    }

    /// Get the IP address, if this is an IP target
    pub fn ip(&self) -> Option<IpAddr> {
        match &self.address {
            TargetAddress::Ip(ip) => Some(*ip),
            TargetAddress::Hostname(_) => None,
        }
    }
}

/// Expand a CIDR range into individual IP addresses
///
/// # Arguments
///
/// * `cidr` - CIDR notation string (e.g., "192.168.1.0/24")
///
/// # Returns
///
/// Vector of enhanced targets, one per IP in the range.
pub fn expand_cidr(cidr: &str) -> crate::Result<Vec<EnhancedTarget>> {
    let network: ipnetwork::IpNetwork = cidr.parse().map_err(|e| {
        crate::SpectreError::Target(crate::error::TargetError::InvalidCidr(format!(
            "{}: {}",
            cidr, e
        )))
    })?;

    let targets: Vec<EnhancedTarget> = network.iter().map(EnhancedTarget::from_ip).collect();

    Ok(targets)
}

/// Resolve a hostname to IP addresses asynchronously
///
/// # Arguments
///
/// * `hostname` - The hostname to resolve
///
/// # Returns
///
/// Vector of resolved IP addresses.
pub async fn resolve_hostname(hostname: &str) -> crate::Result<Vec<IpAddr>> {
    use tokio::net::lookup_host;

    let addrs: Vec<IpAddr> = lookup_host(format!("{}:0", hostname))
        .await
        .map_err(|e| {
            crate::SpectreError::Target(crate::error::TargetError::DnsResolution {
                host: hostname.to_string(),
                message: e.to_string(),
            })
        })?
        .map(|addr| addr.ip())
        .collect();

    if addrs.is_empty() {
        return Err(crate::SpectreError::Target(
            crate::error::TargetError::DnsResolution {
                host: hostname.to_string(),
                message: "No addresses resolved".to_string(),
            },
        ));
    }

    Ok(addrs)
}

/// Deduplicate a list of targets by address
pub fn deduplicate(targets: &mut Vec<EnhancedTarget>) {
    let mut seen = std::collections::HashSet::new();
    targets.retain(|t| seen.insert(t.address.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_target_from_ip() {
        let target = EnhancedTarget::from_ip("192.168.1.1".parse().unwrap());
        assert_eq!(target.address.to_string(), "192.168.1.1");
        assert_eq!(target.status, TargetStatus::Pending);
        assert_eq!(target.priority, 0);
    }

    #[test]
    fn test_enhanced_target_from_hostname() {
        let target = EnhancedTarget::from_hostname("example.com");
        assert_eq!(target.address.to_string(), "example.com");
        assert_eq!(target.status, TargetStatus::Pending);
    }

    #[test]
    fn test_target_with_priority() {
        let target = EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()).with_priority(5);
        assert_eq!(target.priority, 5);
    }

    #[test]
    fn test_target_status_transitions() {
        let mut target = EnhancedTarget::from_ip("10.0.0.1".parse().unwrap());
        assert_eq!(target.status, TargetStatus::Pending);

        target.mark_active();
        assert_eq!(target.status, TargetStatus::Active);
        assert!(target.started_at.is_some());

        target.mark_complete();
        assert_eq!(target.status, TargetStatus::Complete);
        assert!(target.completed_at.is_some());
    }

    #[test]
    fn test_target_mark_skipped() {
        let mut target = EnhancedTarget::from_ip("10.0.0.1".parse().unwrap());
        target.mark_skipped("out of scope");
        assert_eq!(target.status, TargetStatus::Skipped);
        assert_eq!(target.notes.as_deref(), Some("out of scope"));
    }

    #[test]
    fn test_target_mark_error() {
        let mut target = EnhancedTarget::from_ip("10.0.0.1".parse().unwrap());
        target.mark_error("connection refused");
        assert_eq!(target.status, TargetStatus::Error);
        assert_eq!(target.notes.as_deref(), Some("connection refused"));
    }

    #[test]
    fn test_expand_cidr() {
        let targets = expand_cidr("192.168.1.0/30").unwrap();
        assert_eq!(targets.len(), 4);
    }

    #[test]
    fn test_expand_cidr_invalid() {
        let result = expand_cidr("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_deduplicate() {
        let mut targets = vec![
            EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()),
            EnhancedTarget::from_ip("10.0.0.2".parse().unwrap()),
            EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()),
            EnhancedTarget::from_ip("10.0.0.3".parse().unwrap()),
        ];
        deduplicate(&mut targets);
        assert_eq!(targets.len(), 3);
    }

    #[test]
    fn test_target_status_display() {
        assert_eq!(TargetStatus::Pending.to_string(), "pending");
        assert_eq!(TargetStatus::Active.to_string(), "active");
        assert_eq!(TargetStatus::Complete.to_string(), "complete");
        assert_eq!(TargetStatus::Skipped.to_string(), "skipped");
        assert_eq!(TargetStatus::Error.to_string(), "error");
    }

    #[test]
    fn test_target_address_display() {
        let ip_addr = TargetAddress::Ip("192.168.1.1".parse().unwrap());
        assert_eq!(ip_addr.to_string(), "192.168.1.1");

        let hostname = TargetAddress::Hostname("example.com".to_string());
        assert_eq!(hostname.to_string(), "example.com");
    }

    #[tokio::test]
    async fn test_resolve_hostname_localhost() {
        let result = resolve_hostname("localhost").await;
        // localhost should always resolve
        assert!(result.is_ok());
        let addrs = result.unwrap();
        assert!(!addrs.is_empty());
    }

    #[tokio::test]
    async fn test_resolve_hostname_invalid() {
        let result = resolve_hostname("this-host-definitely-does-not-exist.invalid").await;
        assert!(result.is_err());
    }
}
