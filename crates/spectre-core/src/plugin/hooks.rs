//! Plugin event hooks — lifecycle callbacks for scan/workflow events
//!
//! Allows plugins to register callbacks that fire at various points
//! in the scan/workflow lifecycle.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::debug;

/// Types of events that plugins can hook into
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HookEvent {
    /// Before a scan starts
    PreScan,
    /// After a scan completes
    PostScan,
    /// When a new host is discovered
    HostDiscovered,
    /// When a new service is identified
    ServiceIdentified,
    /// Before a workflow step executes
    PreWorkflowStep,
    /// After a workflow step completes
    PostWorkflowStep,
    /// Before a report is generated
    PreReport,
    /// After a report is generated
    PostReport,
    /// Before an export operation
    PreExport,
    /// After an export operation
    PostExport,
}

impl std::fmt::Display for HookEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreScan => write!(f, "pre_scan"),
            Self::PostScan => write!(f, "post_scan"),
            Self::HostDiscovered => write!(f, "host_discovered"),
            Self::ServiceIdentified => write!(f, "service_identified"),
            Self::PreWorkflowStep => write!(f, "pre_workflow_step"),
            Self::PostWorkflowStep => write!(f, "post_workflow_step"),
            Self::PreReport => write!(f, "pre_report"),
            Self::PostReport => write!(f, "post_report"),
            Self::PreExport => write!(f, "pre_export"),
            Self::PostExport => write!(f, "post_export"),
        }
    }
}

/// A registered hook callback
#[derive(Debug, Clone)]
pub struct HookRegistration {
    /// Plugin name that registered the hook
    pub plugin_name: String,
    /// Event to hook into
    pub event: HookEvent,
    /// Lua callback function name within the plugin
    pub callback: String,
    /// Priority (lower = higher priority, executed first)
    pub priority: u32,
    /// Whether this hook is currently enabled
    pub enabled: bool,
}

impl HookRegistration {
    /// Create a new hook registration
    pub fn new(plugin_name: &str, event: HookEvent, callback: &str) -> Self {
        Self {
            plugin_name: plugin_name.to_string(),
            event,
            callback: callback.to_string(),
            priority: 100,
            enabled: true,
        }
    }

    /// Set priority
    pub const fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
}

/// Event data passed to hook callbacks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookContext {
    /// Event type
    pub event: HookEvent,
    /// Key-value data associated with the event
    pub data: HashMap<String, String>,
}

impl HookContext {
    /// Create a new hook context
    pub fn new(event: HookEvent) -> Self {
        Self {
            event,
            data: HashMap::new(),
        }
    }

    /// Add a data field
    pub fn with_data(mut self, key: &str, value: &str) -> Self {
        self.data.insert(key.to_string(), value.to_string());
        self
    }

    /// Get a data field
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }
}

/// Hook manager — maintains hook registrations and dispatches events
#[derive(Debug, Clone)]
pub struct HookManager {
    /// All registered hooks, grouped by event
    hooks: HashMap<HookEvent, Vec<HookRegistration>>,
}

impl HookManager {
    /// Create a new hook manager
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
        }
    }

    /// Register a hook
    pub fn register(&mut self, registration: HookRegistration) {
        debug!(
            plugin = %registration.plugin_name,
            event = %registration.event,
            callback = %registration.callback,
            "Registering hook"
        );
        let hooks = self.hooks.entry(registration.event).or_default();
        hooks.push(registration);
        // Sort by priority
        hooks.sort_by_key(|h| h.priority);
    }

    /// Unregister all hooks for a specific plugin
    pub fn unregister_plugin(&mut self, plugin_name: &str) {
        for hooks in self.hooks.values_mut() {
            hooks.retain(|h| h.plugin_name != plugin_name);
        }
    }

    /// Unregister a specific hook
    pub fn unregister(&mut self, plugin_name: &str, event: HookEvent) {
        if let Some(hooks) = self.hooks.get_mut(&event) {
            hooks.retain(|h| h.plugin_name != plugin_name);
        }
    }

    /// Get all hooks for an event (in priority order)
    pub fn get_hooks(&self, event: HookEvent) -> Vec<&HookRegistration> {
        self.hooks
            .get(&event)
            .map(|hooks| hooks.iter().filter(|h| h.enabled).collect())
            .unwrap_or_default()
    }

    /// Get the total number of registered hooks
    pub fn hook_count(&self) -> usize {
        self.hooks.values().map(Vec::len).sum()
    }

    /// Get the number of hooks for a specific event
    pub fn event_hook_count(&self, event: HookEvent) -> usize {
        self.hooks.get(&event).map_or(0, Vec::len)
    }

    /// List all events that have registered hooks
    pub fn active_events(&self) -> Vec<HookEvent> {
        self.hooks
            .iter()
            .filter(|(_, hooks)| !hooks.is_empty())
            .map(|(event, _)| *event)
            .collect()
    }

    /// Enable or disable all hooks for a plugin
    pub fn set_plugin_enabled(&mut self, plugin_name: &str, enabled: bool) {
        for hooks in self.hooks.values_mut() {
            for hook in hooks.iter_mut() {
                if hook.plugin_name == plugin_name {
                    hook.enabled = enabled;
                }
            }
        }
    }

    /// Clear all hooks
    pub fn clear(&mut self) {
        self.hooks.clear();
    }
}

impl Default for HookManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_event_display() {
        assert_eq!(HookEvent::PreScan.to_string(), "pre_scan");
        assert_eq!(HookEvent::PostScan.to_string(), "post_scan");
        assert_eq!(HookEvent::HostDiscovered.to_string(), "host_discovered");
        assert_eq!(
            HookEvent::ServiceIdentified.to_string(),
            "service_identified"
        );
    }

    #[test]
    fn test_hook_registration_new() {
        let reg = HookRegistration::new("my-plugin", HookEvent::PreScan, "on_pre_scan");
        assert_eq!(reg.plugin_name, "my-plugin");
        assert_eq!(reg.event, HookEvent::PreScan);
        assert_eq!(reg.callback, "on_pre_scan");
        assert_eq!(reg.priority, 100);
        assert!(reg.enabled);
    }

    #[test]
    fn test_hook_registration_with_priority() {
        let reg =
            HookRegistration::new("my-plugin", HookEvent::PreScan, "on_pre_scan").with_priority(10);
        assert_eq!(reg.priority, 10);
    }

    #[test]
    fn test_hook_context() {
        let ctx = HookContext::new(HookEvent::PostScan)
            .with_data("target", "10.0.0.1")
            .with_data("port_count", "42");

        assert_eq!(ctx.event, HookEvent::PostScan);
        assert_eq!(ctx.get("target"), Some("10.0.0.1"));
        assert_eq!(ctx.get("port_count"), Some("42"));
        assert!(ctx.get("nonexistent").is_none());
    }

    #[test]
    fn test_hook_manager_register() {
        let mut manager = HookManager::new();
        manager.register(HookRegistration::new("plugin-a", HookEvent::PreScan, "cb"));
        assert_eq!(manager.hook_count(), 1);
        assert_eq!(manager.event_hook_count(HookEvent::PreScan), 1);
        assert_eq!(manager.event_hook_count(HookEvent::PostScan), 0);
    }

    #[test]
    fn test_hook_manager_get_hooks_priority_order() {
        let mut manager = HookManager::new();
        manager.register(
            HookRegistration::new("plugin-a", HookEvent::PreScan, "cb_a").with_priority(50),
        );
        manager.register(
            HookRegistration::new("plugin-b", HookEvent::PreScan, "cb_b").with_priority(10),
        );
        manager.register(
            HookRegistration::new("plugin-c", HookEvent::PreScan, "cb_c").with_priority(100),
        );

        let hooks = manager.get_hooks(HookEvent::PreScan);
        assert_eq!(hooks.len(), 3);
        assert_eq!(hooks[0].plugin_name, "plugin-b");
        assert_eq!(hooks[1].plugin_name, "plugin-a");
        assert_eq!(hooks[2].plugin_name, "plugin-c");
    }

    #[test]
    fn test_hook_manager_unregister_plugin() {
        let mut manager = HookManager::new();
        manager.register(HookRegistration::new("plugin-a", HookEvent::PreScan, "cb1"));
        manager.register(HookRegistration::new(
            "plugin-a",
            HookEvent::PostScan,
            "cb2",
        ));
        manager.register(HookRegistration::new("plugin-b", HookEvent::PreScan, "cb3"));

        manager.unregister_plugin("plugin-a");
        assert_eq!(manager.hook_count(), 1);
        assert_eq!(manager.event_hook_count(HookEvent::PreScan), 1);
        assert_eq!(manager.event_hook_count(HookEvent::PostScan), 0);
    }

    #[test]
    fn test_hook_manager_unregister_specific() {
        let mut manager = HookManager::new();
        manager.register(HookRegistration::new("plugin-a", HookEvent::PreScan, "cb1"));
        manager.register(HookRegistration::new(
            "plugin-a",
            HookEvent::PostScan,
            "cb2",
        ));

        manager.unregister("plugin-a", HookEvent::PreScan);
        assert_eq!(manager.hook_count(), 1);
        assert_eq!(manager.event_hook_count(HookEvent::PostScan), 1);
    }

    #[test]
    fn test_hook_manager_active_events() {
        let mut manager = HookManager::new();
        manager.register(HookRegistration::new("p", HookEvent::PreScan, "cb"));
        manager.register(HookRegistration::new("p", HookEvent::PostScan, "cb"));

        let events = manager.active_events();
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn test_hook_manager_set_plugin_enabled() {
        let mut manager = HookManager::new();
        manager.register(HookRegistration::new("plugin-a", HookEvent::PreScan, "cb"));

        manager.set_plugin_enabled("plugin-a", false);
        let hooks = manager.get_hooks(HookEvent::PreScan);
        assert!(hooks.is_empty()); // disabled hooks not returned

        manager.set_plugin_enabled("plugin-a", true);
        let hooks = manager.get_hooks(HookEvent::PreScan);
        assert_eq!(hooks.len(), 1);
    }

    #[test]
    fn test_hook_manager_clear() {
        let mut manager = HookManager::new();
        manager.register(HookRegistration::new("p", HookEvent::PreScan, "cb"));
        manager.clear();
        assert_eq!(manager.hook_count(), 0);
    }

    #[test]
    fn test_hook_context_serialization() {
        let ctx = HookContext::new(HookEvent::PostScan).with_data("host", "10.0.0.1");
        let json = serde_json::to_string(&ctx).unwrap();
        let deserialized: HookContext = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.event, HookEvent::PostScan);
        assert_eq!(deserialized.get("host"), Some("10.0.0.1"));
    }
}
