//! User-defined scan profiles combining scan type, timing, ports, and flags

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::scan::{ScanType, TimingTemplate};

/// A user-defined scan profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProfile {
    /// Profile name
    pub name: String,
    /// Profile description
    pub description: String,
    /// Scan type
    pub scan_type: ScanType,
    /// Timing template
    pub timing: TimingTemplate,
    /// Ports to scan
    pub ports: Vec<u16>,
    /// Enable service detection
    pub service_detection: bool,
    /// Enable OS detection
    pub os_detection: bool,
    /// Rate limit (packets per second)
    pub rate_limit: Option<u64>,
    /// Custom flags/options
    pub flags: HashMap<String, String>,
}

impl ScanProfile {
    /// Create a new scan profile
    pub fn new(name: &str, scan_type: ScanType) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            scan_type,
            timing: TimingTemplate::Normal,
            ports: Vec::new(),
            service_detection: false,
            os_detection: false,
            rate_limit: None,
            flags: HashMap::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Set timing template
    pub const fn with_timing(mut self, timing: TimingTemplate) -> Self {
        self.timing = timing;
        self
    }

    /// Set ports
    pub fn with_ports(mut self, ports: Vec<u16>) -> Self {
        self.ports = ports;
        self
    }

    /// Enable service detection
    pub const fn with_service_detection(mut self) -> Self {
        self.service_detection = true;
        self
    }

    /// Enable OS detection
    pub const fn with_os_detection(mut self) -> Self {
        self.os_detection = true;
        self
    }

    /// Set rate limit
    pub const fn with_rate_limit(mut self, rate: u64) -> Self {
        self.rate_limit = Some(rate);
        self
    }

    /// Set a custom flag
    pub fn with_flag(mut self, key: &str, value: &str) -> Self {
        self.flags.insert(key.to_string(), value.to_string());
        self
    }
}

/// Store for managing scan profiles
#[derive(Debug, Clone, Default)]
pub struct ScanProfileStore {
    profiles: HashMap<String, ScanProfile>,
}

impl ScanProfileStore {
    /// Create a new profile store
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a profile
    pub fn add(&mut self, profile: ScanProfile) {
        debug!(name = %profile.name, "Storing scan profile");
        self.profiles.insert(profile.name.clone(), profile);
    }

    /// Get a profile by name
    pub fn get(&self, name: &str) -> Option<&ScanProfile> {
        self.profiles.get(name)
    }

    /// Remove a profile
    pub fn remove(&mut self, name: &str) -> Option<ScanProfile> {
        self.profiles.remove(name)
    }

    /// List all profiles
    pub fn list(&self) -> Vec<&ScanProfile> {
        let mut profiles: Vec<_> = self.profiles.values().collect();
        profiles.sort_by(|a, b| a.name.cmp(&b.name));
        profiles
    }

    /// Get profile count
    pub fn count(&self) -> usize {
        self.profiles.len()
    }

    /// Serialize profiles to JSON
    pub fn to_json(&self) -> crate::Result<String> {
        let profiles: Vec<&ScanProfile> = self.profiles.values().collect();
        serde_json::to_string_pretty(&profiles).map_err(|e| {
            crate::SpectreError::Serialization(format!("Profile serialization error: {}", e))
        })
    }

    /// Load profiles from JSON
    pub fn from_json(json: &str) -> crate::Result<Self> {
        let profiles: Vec<ScanProfile> = serde_json::from_str(json).map_err(|e| {
            crate::SpectreError::Serialization(format!("Profile deserialization error: {}", e))
        })?;
        let mut store = Self::new();
        for profile in profiles {
            store.add(profile);
        }
        Ok(store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_new() {
        let profile = ScanProfile::new("default", ScanType::Connect);
        assert_eq!(profile.name, "default");
        assert!(matches!(profile.scan_type, ScanType::Connect));
        assert!(matches!(profile.timing, TimingTemplate::Normal));
    }

    #[test]
    fn test_profile_builder() {
        let profile = ScanProfile::new("custom", ScanType::Syn)
            .with_description("Custom profile")
            .with_timing(TimingTemplate::Aggressive)
            .with_ports(vec![22, 80, 443])
            .with_service_detection()
            .with_os_detection()
            .with_rate_limit(1000)
            .with_flag("retry", "3");

        assert_eq!(profile.description, "Custom profile");
        assert!(profile.service_detection);
        assert!(profile.os_detection);
        assert_eq!(profile.rate_limit, Some(1000));
        assert_eq!(profile.flags.get("retry"), Some(&"3".to_string()));
    }

    #[test]
    fn test_profile_store() {
        let mut store = ScanProfileStore::new();
        store.add(ScanProfile::new("fast", ScanType::Connect));
        store.add(ScanProfile::new("stealth", ScanType::Syn));

        assert_eq!(store.count(), 2);
        assert!(store.get("fast").is_some());
        assert!(store.get("nonexistent").is_none());
    }

    #[test]
    fn test_profile_store_remove() {
        let mut store = ScanProfileStore::new();
        store.add(ScanProfile::new("test", ScanType::Connect));
        let removed = store.remove("test");
        assert!(removed.is_some());
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_profile_store_list() {
        let mut store = ScanProfileStore::new();
        store.add(ScanProfile::new("b-profile", ScanType::Connect));
        store.add(ScanProfile::new("a-profile", ScanType::Syn));

        let list = store.list();
        assert_eq!(list[0].name, "a-profile");
        assert_eq!(list[1].name, "b-profile");
    }

    #[test]
    fn test_profile_store_json_roundtrip() {
        let mut store = ScanProfileStore::new();
        store.add(
            ScanProfile::new("test", ScanType::Connect)
                .with_ports(vec![80, 443])
                .with_description("Test profile"),
        );

        let json = store.to_json().unwrap();
        let loaded = ScanProfileStore::from_json(&json).unwrap();
        assert_eq!(loaded.count(), 1);
        assert!(loaded.get("test").is_some());
    }

    #[test]
    fn test_profile_serialization() {
        let profile = ScanProfile::new("test", ScanType::Syn).with_ports(vec![22]);
        let json = serde_json::to_string(&profile).unwrap();
        let deserialized: ScanProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "test");
        assert_eq!(deserialized.ports, vec![22]);
    }
}
