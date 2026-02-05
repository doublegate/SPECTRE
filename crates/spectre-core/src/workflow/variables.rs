//! Variable store for workflow execution
//!
//! Supports variable substitution using `${variable_name}` syntax.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Variable store for workflow execution context
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VariableStore {
    variables: HashMap<String, String>,
}

impl VariableStore {
    /// Create a new empty variable store
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a variable
    pub fn set(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    /// Get a variable value
    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(String::as_str)
    }

    /// Remove a variable
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.variables.remove(key)
    }

    /// Check if a variable exists
    pub fn contains(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    /// Get the number of variables
    pub fn count(&self) -> usize {
        self.variables.len()
    }

    /// Clear all variables
    pub fn clear(&mut self) {
        self.variables.clear();
    }

    /// Perform variable substitution on a string
    ///
    /// Replaces `${var_name}` patterns with variable values.
    /// Unknown variables are left as-is.
    pub fn substitute(&self, input: &str) -> String {
        let mut result = input.to_string();
        for (key, value) in &self.variables {
            let pattern = format!("${{{}}}", key);
            result = result.replace(&pattern, value);
        }
        result
    }

    /// Merge another variable store into this one
    ///
    /// Values from `other` override existing values.
    pub fn merge(&mut self, other: &Self) {
        for (key, value) in &other.variables {
            self.variables.insert(key.clone(), value.clone());
        }
    }

    /// Get all variable names
    pub fn keys(&self) -> Vec<&str> {
        self.variables.keys().map(String::as_str).collect()
    }

    /// Create from a HashMap
    pub const fn from_map(map: HashMap<String, String>) -> Self {
        Self { variables: map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_store_new() {
        let store = VariableStore::new();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let mut store = VariableStore::new();
        store.set("target", "10.0.0.1");
        assert_eq!(store.get("target"), Some("10.0.0.1"));
        assert!(store.get("nonexistent").is_none());
    }

    #[test]
    fn test_remove() {
        let mut store = VariableStore::new();
        store.set("key", "value");
        assert!(store.remove("key").is_some());
        assert!(store.get("key").is_none());
    }

    #[test]
    fn test_contains() {
        let mut store = VariableStore::new();
        store.set("key", "value");
        assert!(store.contains("key"));
        assert!(!store.contains("other"));
    }

    #[test]
    fn test_clear() {
        let mut store = VariableStore::new();
        store.set("a", "1");
        store.set("b", "2");
        store.clear();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_substitute_single() {
        let mut store = VariableStore::new();
        store.set("target", "10.0.0.1");
        let result = store.substitute("Scanning ${target}");
        assert_eq!(result, "Scanning 10.0.0.1");
    }

    #[test]
    fn test_substitute_multiple() {
        let mut store = VariableStore::new();
        store.set("host", "10.0.0.1");
        store.set("port", "80");
        let result = store.substitute("${host}:${port}");
        assert_eq!(result, "10.0.0.1:80");
    }

    #[test]
    fn test_substitute_unknown_variable() {
        let store = VariableStore::new();
        let result = store.substitute("${unknown} stays");
        assert_eq!(result, "${unknown} stays");
    }

    #[test]
    fn test_substitute_no_variables() {
        let store = VariableStore::new();
        let result = store.substitute("plain text");
        assert_eq!(result, "plain text");
    }

    #[test]
    fn test_merge() {
        let mut store1 = VariableStore::new();
        store1.set("a", "1");
        store1.set("b", "2");

        let mut store2 = VariableStore::new();
        store2.set("b", "3");
        store2.set("c", "4");

        store1.merge(&store2);
        assert_eq!(store1.get("a"), Some("1"));
        assert_eq!(store1.get("b"), Some("3")); // Overridden
        assert_eq!(store1.get("c"), Some("4"));
    }

    #[test]
    fn test_keys() {
        let mut store = VariableStore::new();
        store.set("a", "1");
        store.set("b", "2");
        let mut keys = store.keys();
        keys.sort_unstable();
        assert_eq!(keys, vec!["a", "b"]);
    }

    #[test]
    fn test_from_map() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), "value".to_string());
        let store = VariableStore::from_map(map);
        assert_eq!(store.get("key"), Some("value"));
    }

    #[test]
    fn test_serialization() {
        let mut store = VariableStore::new();
        store.set("target", "10.0.0.1");
        let json = serde_json::to_string(&store).unwrap();
        let deserialized: VariableStore = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.get("target"), Some("10.0.0.1"));
    }
}
