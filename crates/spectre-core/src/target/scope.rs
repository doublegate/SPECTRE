//! Scope enforcement for target management
//!
//! Ensures targets are within the authorized scope of operations.
//! Supports both allow-lists and block-lists of networks.

use std::net::IpAddr;

use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};

use super::TargetAddress;

/// Scope enforcer for validating targets against authorized networks
///
/// # Example
///
/// ```
/// use spectre_core::target::{ScopeEnforcer, TargetAddress};
///
/// let mut scope = ScopeEnforcer::new();
/// scope.allow("192.168.0.0/16".parse().unwrap());
/// scope.block("192.168.1.0/24".parse().unwrap());
///
/// let addr = TargetAddress::Ip("192.168.2.1".parse().unwrap());
/// assert!(scope.is_in_scope(&addr));
///
/// let blocked = TargetAddress::Ip("192.168.1.100".parse().unwrap());
/// assert!(!scope.is_in_scope(&blocked));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeEnforcer {
    /// Allowed networks (if empty, all are allowed)
    allowed: Vec<IpNetwork>,
    /// Blocked networks (always enforced)
    blocked: Vec<IpNetwork>,
    /// Whether to allow hostnames (unresolved)
    allow_hostnames: bool,
}

impl ScopeEnforcer {
    /// Create a new scope enforcer with no restrictions
    pub const fn new() -> Self {
        Self {
            allowed: Vec::new(),
            blocked: Vec::new(),
            allow_hostnames: true,
        }
    }

    /// Add an allowed network
    pub fn allow(&mut self, network: IpNetwork) {
        self.allowed.push(network);
    }

    /// Add a blocked network
    pub fn block(&mut self, network: IpNetwork) {
        self.blocked.push(network);
    }

    /// Set whether hostnames are allowed
    pub const fn set_allow_hostnames(&mut self, allow: bool) {
        self.allow_hostnames = allow;
    }

    /// Check if a target address is within scope
    pub fn is_in_scope(&self, target: &TargetAddress) -> bool {
        match target {
            TargetAddress::Ip(ip) => self.is_ip_in_scope(*ip),
            TargetAddress::Hostname(_) => self.allow_hostnames,
        }
    }

    /// Check if an IP address is within scope
    pub fn is_ip_in_scope(&self, ip: IpAddr) -> bool {
        // Check blocked list first
        for blocked in &self.blocked {
            if blocked.contains(ip) {
                return false;
            }
        }

        // If allow list is empty, all non-blocked IPs are in scope
        if self.allowed.is_empty() {
            return true;
        }

        // Check against allow list
        for allowed in &self.allowed {
            if allowed.contains(ip) {
                return true;
            }
        }

        false
    }

    /// Get the number of allowed networks
    pub const fn allowed_count(&self) -> usize {
        self.allowed.len()
    }

    /// Get the number of blocked networks
    pub const fn blocked_count(&self) -> usize {
        self.blocked.len()
    }

    /// Check if scope enforcement is active (has any rules)
    pub const fn is_active(&self) -> bool {
        !self.allowed.is_empty() || !self.blocked.is_empty()
    }
}

impl Default for ScopeEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_no_restrictions() {
        let scope = ScopeEnforcer::new();
        assert!(scope.is_in_scope(&TargetAddress::Ip("10.0.0.1".parse().unwrap())));
        assert!(scope.is_in_scope(&TargetAddress::Ip("192.168.1.1".parse().unwrap())));
        assert!(scope.is_in_scope(&TargetAddress::Hostname("example.com".to_string())));
    }

    #[test]
    fn test_scope_allow_list() {
        let mut scope = ScopeEnforcer::new();
        scope.allow("192.168.0.0/16".parse().unwrap());

        assert!(scope.is_in_scope(&TargetAddress::Ip("192.168.1.1".parse().unwrap())));
        assert!(scope.is_in_scope(&TargetAddress::Ip("192.168.255.255".parse().unwrap())));
        assert!(!scope.is_in_scope(&TargetAddress::Ip("10.0.0.1".parse().unwrap())));
    }

    #[test]
    fn test_scope_block_list() {
        let mut scope = ScopeEnforcer::new();
        scope.block("192.168.1.0/24".parse().unwrap());

        assert!(!scope.is_in_scope(&TargetAddress::Ip("192.168.1.1".parse().unwrap())));
        assert!(scope.is_in_scope(&TargetAddress::Ip("192.168.2.1".parse().unwrap())));
        assert!(scope.is_in_scope(&TargetAddress::Ip("10.0.0.1".parse().unwrap())));
    }

    #[test]
    fn test_scope_block_overrides_allow() {
        let mut scope = ScopeEnforcer::new();
        scope.allow("192.168.0.0/16".parse().unwrap());
        scope.block("192.168.1.0/24".parse().unwrap());

        assert!(!scope.is_in_scope(&TargetAddress::Ip("192.168.1.100".parse().unwrap())));
        assert!(scope.is_in_scope(&TargetAddress::Ip("192.168.2.1".parse().unwrap())));
    }

    #[test]
    fn test_scope_hostname_control() {
        let mut scope = ScopeEnforcer::new();
        assert!(scope.is_in_scope(&TargetAddress::Hostname("example.com".to_string())));

        scope.set_allow_hostnames(false);
        assert!(!scope.is_in_scope(&TargetAddress::Hostname("example.com".to_string())));
    }

    #[test]
    fn test_scope_counts() {
        let mut scope = ScopeEnforcer::new();
        assert_eq!(scope.allowed_count(), 0);
        assert_eq!(scope.blocked_count(), 0);
        assert!(!scope.is_active());

        scope.allow("10.0.0.0/8".parse().unwrap());
        scope.block("10.0.1.0/24".parse().unwrap());

        assert_eq!(scope.allowed_count(), 1);
        assert_eq!(scope.blocked_count(), 1);
        assert!(scope.is_active());
    }

    #[test]
    fn test_scope_multiple_allow_ranges() {
        let mut scope = ScopeEnforcer::new();
        scope.allow("10.0.0.0/8".parse().unwrap());
        scope.allow("172.16.0.0/12".parse().unwrap());

        assert!(scope.is_in_scope(&TargetAddress::Ip("10.1.2.3".parse().unwrap())));
        assert!(scope.is_in_scope(&TargetAddress::Ip("172.16.5.5".parse().unwrap())));
        assert!(!scope.is_in_scope(&TargetAddress::Ip("8.8.8.8".parse().unwrap())));
    }

    mod proptest_tests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn blocked_ip_never_in_scope(a in 0u8..=255, b in 0u8..=255) {
                let mut scope = ScopeEnforcer::new();
                scope.allow("10.0.0.0/8".parse().unwrap());
                scope.block("10.1.0.0/16".parse().unwrap());
                let ip_str = format!("10.1.{a}.{b}");
                let ip: std::net::IpAddr = ip_str.parse().unwrap();
                prop_assert!(!scope.is_ip_in_scope(ip));
            }

            #[test]
            fn allowed_non_blocked_ip_in_scope(a in 0u8..=255, b in 0u8..=255) {
                let mut scope = ScopeEnforcer::new();
                scope.allow("10.0.0.0/8".parse().unwrap());
                scope.block("10.1.0.0/16".parse().unwrap());
                // 10.2.x.x should always be in scope
                let ip_str = format!("10.2.{a}.{b}");
                let ip: std::net::IpAddr = ip_str.parse().unwrap();
                prop_assert!(scope.is_ip_in_scope(ip));
            }

            #[test]
            fn empty_scope_allows_all(a in 0u8..=255, b in 0u8..=255, c in 0u8..=255, d in 0u8..=255) {
                let scope = ScopeEnforcer::new();
                let ip_str = format!("{a}.{b}.{c}.{d}");
                let ip: std::net::IpAddr = ip_str.parse().unwrap();
                prop_assert!(scope.is_ip_in_scope(ip));
            }
        }
    }
}
