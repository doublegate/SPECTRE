//! Plugin registry — discovery, versioning, dependency resolution
//!
//! Manages installed plugins: listing, lookup, dependency ordering,
//! and version compatibility checking.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use super::Plugin;

/// Entry in the plugin registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// Path to the plugin directory
    pub path: PathBuf,
    /// Whether the plugin is enabled
    pub enabled: bool,
    /// Tags
    pub tags: Vec<String>,
    /// Dependency names
    pub dependencies: Vec<String>,
}

impl RegistryEntry {
    /// Create a registry entry from a plugin
    pub fn from_plugin(plugin: &Plugin) -> Self {
        Self {
            name: plugin.manifest.name.clone(),
            version: plugin.manifest.version.clone(),
            description: plugin.manifest.description.clone(),
            author: plugin.manifest.author.clone(),
            path: plugin.path.clone(),
            enabled: true,
            tags: plugin.manifest.tags.clone(),
            dependencies: plugin.manifest.dependencies.clone(),
        }
    }
}

/// Plugin registry for managing installed plugins
#[derive(Debug, Clone)]
pub struct PluginRegistry {
    /// Registered plugins by name
    entries: HashMap<String, RegistryEntry>,
    /// Plugin directories to scan
    plugin_dirs: Vec<PathBuf>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            plugin_dirs: Vec::new(),
        }
    }

    /// Add a plugin directory to scan
    pub fn add_plugin_dir(&mut self, dir: PathBuf) {
        self.plugin_dirs.push(dir);
    }

    /// Scan plugin directories and populate the registry
    pub fn scan(&mut self) -> crate::Result<usize> {
        let mut discovered = 0;

        let dirs = self.plugin_dirs.clone();
        for dir in &dirs {
            if !dir.exists() {
                debug!(path = %dir.display(), "Plugin directory does not exist, skipping");
                continue;
            }

            let plugins = super::discover_plugins(dir)?;
            for plugin in &plugins {
                let entry = RegistryEntry::from_plugin(plugin);
                info!(name = %entry.name, version = %entry.version, "Registered plugin");
                self.entries.insert(entry.name.clone(), entry);
                discovered += 1;
            }
        }

        Ok(discovered)
    }

    /// Register a plugin manually
    pub fn register(&mut self, entry: RegistryEntry) {
        debug!(name = %entry.name, "Manually registering plugin");
        self.entries.insert(entry.name.clone(), entry);
    }

    /// Unregister a plugin by name
    pub fn unregister(&mut self, name: &str) -> bool {
        self.entries.remove(name).is_some()
    }

    /// Get a plugin entry by name
    pub fn get(&self, name: &str) -> Option<&RegistryEntry> {
        self.entries.get(name)
    }

    /// List all registered plugins
    pub fn list(&self) -> Vec<&RegistryEntry> {
        let mut entries: Vec<_> = self.entries.values().collect();
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        entries
    }

    /// List enabled plugins only
    pub fn list_enabled(&self) -> Vec<&RegistryEntry> {
        let mut entries: Vec<_> = self.entries.values().filter(|e| e.enabled).collect();
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        entries
    }

    /// Get the number of registered plugins
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Enable or disable a plugin
    pub fn set_enabled(&mut self, name: &str, enabled: bool) -> bool {
        if let Some(entry) = self.entries.get_mut(name) {
            entry.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// Search for plugins by tag
    pub fn search_by_tag(&self, tag: &str) -> Vec<&RegistryEntry> {
        self.entries
            .values()
            .filter(|e| e.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
            .collect()
    }

    /// Search for plugins by name substring
    pub fn search_by_name(&self, query: &str) -> Vec<&RegistryEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .values()
            .filter(|e| e.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Resolve plugin load order based on dependencies
    ///
    /// Returns plugins in dependency order (dependencies first).
    /// Returns an error if there are circular dependencies or missing dependencies.
    pub fn resolve_load_order(&self) -> crate::Result<Vec<&str>> {
        let enabled: HashMap<&str, &RegistryEntry> = self
            .entries
            .values()
            .filter(|e| e.enabled)
            .map(|e| (e.name.as_str(), e))
            .collect();

        let mut visited: HashMap<&str, bool> = HashMap::new();
        let mut order: Vec<&str> = Vec::new();

        for name in enabled.keys() {
            Self::dfs_resolve(name, &enabled, &mut visited, &mut order)?;
        }

        Ok(order)
    }

    /// Depth-first dependency resolution
    fn dfs_resolve<'a>(
        name: &'a str,
        plugins: &HashMap<&'a str, &'a RegistryEntry>,
        visited: &mut HashMap<&'a str, bool>,
        order: &mut Vec<&'a str>,
    ) -> crate::Result<()> {
        if let Some(&in_progress) = visited.get(name) {
            if in_progress {
                return Err(crate::SpectreError::Plugin(
                    crate::error::PluginError::RuntimeError(format!(
                        "Circular dependency detected involving: {}",
                        name
                    )),
                ));
            }
            // Already fully visited
            return Ok(());
        }

        // Mark as in-progress
        visited.insert(name, true);

        if let Some(entry) = plugins.get(name) {
            for dep in &entry.dependencies {
                if !plugins.contains_key(dep.as_str()) {
                    warn!(plugin = %name, dependency = %dep, "Missing plugin dependency");
                    // Not a hard error -- dependency may be optional
                    continue;
                }
                Self::dfs_resolve(dep.as_str(), plugins, visited, order)?;
            }
        }

        // Mark as fully visited
        visited.insert(name, false);
        order.push(name);

        Ok(())
    }

    /// Check if all dependencies for a plugin are satisfied
    pub fn check_dependencies(&self, name: &str) -> Vec<String> {
        let mut missing = Vec::new();
        if let Some(entry) = self.entries.get(name) {
            for dep in &entry.dependencies {
                if !self.entries.contains_key(dep) {
                    missing.push(dep.clone());
                }
            }
        }
        missing
    }

    /// Compare two semver-like version strings
    ///
    /// Returns Ordering::Less, Equal, or Greater.
    pub fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
        let parse = |v: &str| -> Vec<u64> {
            v.split('.')
                .map(|s| s.parse::<u64>().unwrap_or(0))
                .collect()
        };
        let va = parse(a);
        let vb = parse(b);

        let max_len = va.len().max(vb.len());
        for i in 0..max_len {
            let pa = va.get(i).copied().unwrap_or(0);
            let pb = vb.get(i).copied().unwrap_or(0);
            match pa.cmp(&pb) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }
        std::cmp::Ordering::Equal
    }

    /// Save registry to a JSON file
    pub fn save(&self, path: &Path) -> crate::Result<()> {
        let entries: Vec<&RegistryEntry> = self.entries.values().collect();
        let json = serde_json::to_string_pretty(&entries).map_err(|e| {
            crate::SpectreError::Plugin(crate::error::PluginError::RuntimeError(format!(
                "Registry serialization error: {}",
                e
            )))
        })?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load registry from a JSON file
    pub fn load(path: &Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let entries: Vec<RegistryEntry> = serde_json::from_str(&content).map_err(|e| {
            crate::SpectreError::Plugin(crate::error::PluginError::RuntimeError(format!(
                "Registry deserialization error: {}",
                e
            )))
        })?;

        let mut registry = Self::new();
        for entry in entries {
            registry.entries.insert(entry.name.clone(), entry);
        }
        Ok(registry)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(name: &str, version: &str, deps: Vec<&str>) -> RegistryEntry {
        RegistryEntry {
            name: name.to_string(),
            version: version.to_string(),
            description: format!("Plugin {}", name),
            author: "Test".to_string(),
            path: PathBuf::from(format!("/plugins/{}", name)),
            enabled: true,
            tags: vec!["test".to_string()],
            dependencies: deps.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn test_registry_new() {
        let registry = PluginRegistry::new();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_register_and_get() {
        let mut registry = PluginRegistry::new();
        registry.register(make_entry("test-plugin", "1.0.0", vec![]));
        assert_eq!(registry.count(), 1);
        assert!(registry.get("test-plugin").is_some());
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_unregister() {
        let mut registry = PluginRegistry::new();
        registry.register(make_entry("test-plugin", "1.0.0", vec![]));
        assert!(registry.unregister("test-plugin"));
        assert!(!registry.unregister("test-plugin"));
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_list() {
        let mut registry = PluginRegistry::new();
        registry.register(make_entry("bravo", "1.0.0", vec![]));
        registry.register(make_entry("alpha", "1.0.0", vec![]));
        let list = registry.list();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name, "alpha");
        assert_eq!(list[1].name, "bravo");
    }

    #[test]
    fn test_set_enabled() {
        let mut registry = PluginRegistry::new();
        registry.register(make_entry("p1", "1.0.0", vec![]));
        assert!(registry.set_enabled("p1", false));
        assert!(registry.list_enabled().is_empty());
        assert!(registry.set_enabled("p1", true));
        assert_eq!(registry.list_enabled().len(), 1);
    }

    #[test]
    fn test_search_by_tag() {
        let mut registry = PluginRegistry::new();
        registry.register(make_entry("p1", "1.0.0", vec![]));
        let results = registry.search_by_tag("test");
        assert_eq!(results.len(), 1);
        let results = registry.search_by_tag("nonexistent");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_by_name() {
        let mut registry = PluginRegistry::new();
        registry.register(make_entry("scan-helper", "1.0.0", vec![]));
        registry.register(make_entry("report-gen", "1.0.0", vec![]));
        let results = registry.search_by_name("scan");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "scan-helper");
    }

    #[test]
    fn test_resolve_load_order_no_deps() {
        let mut registry = PluginRegistry::new();
        registry.register(make_entry("a", "1.0.0", vec![]));
        registry.register(make_entry("b", "1.0.0", vec![]));
        let order = registry.resolve_load_order().unwrap();
        assert_eq!(order.len(), 2);
    }

    #[test]
    fn test_resolve_load_order_with_deps() {
        let mut registry = PluginRegistry::new();
        registry.register(make_entry("base", "1.0.0", vec![]));
        registry.register(make_entry("derived", "1.0.0", vec!["base"]));
        let order = registry.resolve_load_order().unwrap();
        let base_pos = order.iter().position(|&n| n == "base").unwrap();
        let derived_pos = order.iter().position(|&n| n == "derived").unwrap();
        assert!(base_pos < derived_pos);
    }

    #[test]
    fn test_resolve_load_order_circular() {
        let mut registry = PluginRegistry::new();
        registry.register(make_entry("a", "1.0.0", vec!["b"]));
        registry.register(make_entry("b", "1.0.0", vec!["a"]));
        let result = registry.resolve_load_order();
        assert!(result.is_err());
    }

    #[test]
    fn test_check_dependencies_satisfied() {
        let mut registry = PluginRegistry::new();
        registry.register(make_entry("base", "1.0.0", vec![]));
        registry.register(make_entry("derived", "1.0.0", vec!["base"]));
        let missing = registry.check_dependencies("derived");
        assert!(missing.is_empty());
    }

    #[test]
    fn test_check_dependencies_missing() {
        let mut registry = PluginRegistry::new();
        registry.register(make_entry("derived", "1.0.0", vec!["base"]));
        let missing = registry.check_dependencies("derived");
        assert_eq!(missing, vec!["base".to_string()]);
    }

    #[test]
    fn test_compare_versions() {
        use std::cmp::Ordering;
        assert_eq!(
            PluginRegistry::compare_versions("1.0.0", "1.0.0"),
            Ordering::Equal
        );
        assert_eq!(
            PluginRegistry::compare_versions("1.0.0", "2.0.0"),
            Ordering::Less
        );
        assert_eq!(
            PluginRegistry::compare_versions("2.0.0", "1.0.0"),
            Ordering::Greater
        );
        assert_eq!(
            PluginRegistry::compare_versions("1.2.0", "1.10.0"),
            Ordering::Less
        );
        assert_eq!(
            PluginRegistry::compare_versions("1.0", "1.0.0"),
            Ordering::Equal
        );
    }

    #[test]
    fn test_save_load_registry() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("registry.json");

        let mut registry = PluginRegistry::new();
        registry.register(make_entry("p1", "1.0.0", vec![]));
        registry.register(make_entry("p2", "2.0.0", vec!["p1"]));
        registry.save(&path).unwrap();

        let loaded = PluginRegistry::load(&path).unwrap();
        assert_eq!(loaded.count(), 2);
        assert!(loaded.get("p1").is_some());
        assert!(loaded.get("p2").is_some());
    }

    #[test]
    fn test_scan_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let mut registry = PluginRegistry::new();
        registry.add_plugin_dir(dir.path().to_path_buf());
        let count = registry.scan().unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_scan_nonexistent_dir() {
        let mut registry = PluginRegistry::new();
        registry.add_plugin_dir(PathBuf::from("/nonexistent/plugin/dir"));
        let count = registry.scan().unwrap();
        assert_eq!(count, 0);
    }
}
