//! Recipe management module
//!
//! Provides recipe storage, import/export, versioning, search,
//! validation, and a built-in recipe library.

mod builtin;
mod format;
mod search;
mod storage;
mod validate;

pub use builtin::BuiltinRecipes;
pub use format::{RecipeBundle, RecipeFormat};
pub use search::RecipeSearch;
pub use storage::RecipeStore;
pub use validate::RecipeValidator;

use serde::{Deserialize, Serialize};

/// A recipe defining a sequence of operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    /// Recipe name
    pub name: String,
    /// Recipe description
    pub description: String,
    /// Recipe version (semver)
    pub version: String,
    /// Author
    pub author: String,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Operations in this recipe
    pub operations: Vec<RecipeOperation>,
    /// Category
    pub category: String,
    /// Changelog entries
    #[serde(default)]
    pub changelog: Vec<ChangelogEntry>,
}

/// A single operation in a recipe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeOperation {
    /// Operation name
    pub name: String,
    /// Operation parameters
    #[serde(default)]
    pub params: std::collections::HashMap<String, serde_json::Value>,
    /// Description of what this operation does
    #[serde(default)]
    pub description: String,
}

/// Changelog entry for recipe versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogEntry {
    /// Version
    pub version: String,
    /// Change description
    pub description: String,
    /// Date of change
    pub date: String,
}

impl Recipe {
    /// Create a new recipe
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            version: version.to_string(),
            author: String::new(),
            tags: Vec::new(),
            operations: Vec::new(),
            category: "general".to_string(),
            changelog: Vec::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Set author
    pub fn with_author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Set category
    pub fn with_category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }

    /// Add an operation
    pub fn add_operation(&mut self, operation: RecipeOperation) {
        self.operations.push(operation);
    }

    /// Get the number of operations
    pub const fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Add a changelog entry
    pub fn add_changelog(&mut self, version: &str, description: &str, date: &str) {
        self.changelog.push(ChangelogEntry {
            version: version.to_string(),
            description: description.to_string(),
            date: date.to_string(),
        });
    }
}

impl RecipeOperation {
    /// Create a new operation
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            params: std::collections::HashMap::new(),
            description: String::new(),
        }
    }

    /// Set a parameter
    pub fn with_param(mut self, key: &str, value: serde_json::Value) -> Self {
        self.params.insert(key.to_string(), value);
        self
    }

    /// Set description
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_new() {
        let recipe = Recipe::new("base64-encode", "1.0.0");
        assert_eq!(recipe.name, "base64-encode");
        assert_eq!(recipe.version, "1.0.0");
        assert_eq!(recipe.operation_count(), 0);
    }

    #[test]
    fn test_recipe_builder() {
        let recipe = Recipe::new("test", "1.0.0")
            .with_description("Test recipe")
            .with_author("SPECTRE")
            .with_tag("encoding")
            .with_category("crypto");

        assert_eq!(recipe.description, "Test recipe");
        assert_eq!(recipe.author, "SPECTRE");
        assert_eq!(recipe.tags, vec!["encoding"]);
        assert_eq!(recipe.category, "crypto");
    }

    #[test]
    fn test_recipe_operations() {
        let mut recipe = Recipe::new("test", "1.0.0");
        recipe.add_operation(RecipeOperation::new("base64_encode"));
        recipe.add_operation(RecipeOperation::new("hex_decode"));
        assert_eq!(recipe.operation_count(), 2);
    }

    #[test]
    fn test_operation_builder() {
        let op = RecipeOperation::new("base64_encode")
            .with_param("padding", serde_json::json!(true))
            .with_description("Encode to Base64");

        assert_eq!(op.name, "base64_encode");
        assert_eq!(op.description, "Encode to Base64");
        assert!(op.params.contains_key("padding"));
    }

    #[test]
    fn test_recipe_changelog() {
        let mut recipe = Recipe::new("test", "1.1.0");
        recipe.add_changelog("1.0.0", "Initial release", "2025-01-01");
        recipe.add_changelog("1.1.0", "Added new operation", "2025-06-01");
        assert_eq!(recipe.changelog.len(), 2);
    }

    #[test]
    fn test_recipe_serialization() {
        let mut recipe = Recipe::new("test", "1.0.0").with_tag("encoding");
        recipe.add_operation(RecipeOperation::new("base64_encode"));

        let json = serde_json::to_string(&recipe).unwrap();
        let deserialized: Recipe = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "test");
        assert_eq!(deserialized.operation_count(), 1);
    }
}
