//! Recipe import/export format support (JSON, YAML, TOML)

use serde::{Deserialize, Serialize};

use super::Recipe;

/// Supported recipe formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipeFormat {
    /// JSON format
    Json,
    /// YAML format
    Yaml,
    /// TOML format
    Toml,
}

/// A portable recipe bundle (for sharing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeBundle {
    /// Bundle format version
    pub format_version: String,
    /// Recipes in this bundle
    pub recipes: Vec<Recipe>,
    /// Bundle metadata
    pub metadata: BundleMetadata,
}

/// Metadata for a recipe bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleMetadata {
    /// Bundle creator
    pub created_by: String,
    /// Creation date
    pub created_at: String,
    /// Bundle description
    pub description: String,
}

impl RecipeBundle {
    /// Create a new recipe bundle
    pub fn new(description: &str) -> Self {
        Self {
            format_version: "1.0.0".to_string(),
            recipes: Vec::new(),
            metadata: BundleMetadata {
                created_by: "SPECTRE".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                description: description.to_string(),
            },
        }
    }

    /// Add a recipe to the bundle
    pub fn add(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    /// Get the number of recipes
    pub fn count(&self) -> usize {
        self.recipes.len()
    }
}

impl RecipeFormat {
    /// Export a recipe to string in this format
    pub fn export(&self, recipe: &Recipe) -> crate::Result<String> {
        match self {
            Self::Json => serde_json::to_string_pretty(recipe).map_err(|e| {
                crate::SpectreError::Recipe(crate::error::RecipeError::InvalidFormat(format!(
                    "JSON export error: {}",
                    e
                )))
            }),
            Self::Yaml => serde_yaml::to_string(recipe).map_err(|e| {
                crate::SpectreError::Recipe(crate::error::RecipeError::InvalidFormat(format!(
                    "YAML export error: {}",
                    e
                )))
            }),
            Self::Toml => toml::to_string_pretty(recipe).map_err(|e| {
                crate::SpectreError::Recipe(crate::error::RecipeError::InvalidFormat(format!(
                    "TOML export error: {}",
                    e
                )))
            }),
        }
    }

    /// Import a recipe from string in this format
    pub fn import(&self, content: &str) -> crate::Result<Recipe> {
        match self {
            Self::Json => serde_json::from_str(content).map_err(|e| {
                crate::SpectreError::Recipe(crate::error::RecipeError::InvalidFormat(format!(
                    "JSON import error: {}",
                    e
                )))
            }),
            Self::Yaml => serde_yaml::from_str(content).map_err(|e| {
                crate::SpectreError::Recipe(crate::error::RecipeError::InvalidFormat(format!(
                    "YAML import error: {}",
                    e
                )))
            }),
            Self::Toml => toml::from_str(content).map_err(|e| {
                crate::SpectreError::Recipe(crate::error::RecipeError::InvalidFormat(format!(
                    "TOML import error: {}",
                    e
                )))
            }),
        }
    }

    /// Export a bundle
    pub fn export_bundle(&self, bundle: &RecipeBundle) -> crate::Result<String> {
        match self {
            Self::Json => serde_json::to_string_pretty(bundle).map_err(|e| {
                crate::SpectreError::Recipe(crate::error::RecipeError::InvalidFormat(format!(
                    "Bundle export error: {}",
                    e
                )))
            }),
            Self::Yaml => serde_yaml::to_string(bundle).map_err(|e| {
                crate::SpectreError::Recipe(crate::error::RecipeError::InvalidFormat(format!(
                    "Bundle export error: {}",
                    e
                )))
            }),
            Self::Toml => Err(crate::SpectreError::Recipe(
                crate::error::RecipeError::InvalidFormat(
                    "TOML does not support complex nested structures for bundles".to_string(),
                ),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::RecipeOperation;

    fn make_recipe() -> Recipe {
        let mut recipe = Recipe::new("test", "1.0.0")
            .with_description("Test")
            .with_author("tester");
        recipe.add_operation(RecipeOperation::new("base64_encode"));
        recipe
    }

    #[test]
    fn test_export_json() {
        let recipe = make_recipe();
        let json = RecipeFormat::Json.export(&recipe).unwrap();
        assert!(json.contains("test"));
    }

    #[test]
    fn test_export_yaml() {
        let recipe = make_recipe();
        let yaml = RecipeFormat::Yaml.export(&recipe).unwrap();
        assert!(yaml.contains("test"));
    }

    #[test]
    fn test_export_toml() {
        let recipe = make_recipe();
        let toml_str = RecipeFormat::Toml.export(&recipe).unwrap();
        assert!(toml_str.contains("test"));
    }

    #[test]
    fn test_import_json() {
        let recipe = make_recipe();
        let json = RecipeFormat::Json.export(&recipe).unwrap();
        let imported = RecipeFormat::Json.import(&json).unwrap();
        assert_eq!(imported.name, "test");
    }

    #[test]
    fn test_import_yaml() {
        let recipe = make_recipe();
        let yaml = RecipeFormat::Yaml.export(&recipe).unwrap();
        let imported = RecipeFormat::Yaml.import(&yaml).unwrap();
        assert_eq!(imported.name, "test");
    }

    #[test]
    fn test_import_invalid() {
        assert!(RecipeFormat::Json.import("invalid json").is_err());
    }

    #[test]
    fn test_recipe_bundle() {
        let mut bundle = RecipeBundle::new("Test bundle");
        bundle.add(make_recipe());
        assert_eq!(bundle.count(), 1);
    }

    #[test]
    fn test_bundle_export_json() {
        let mut bundle = RecipeBundle::new("Test");
        bundle.add(make_recipe());

        let json = RecipeFormat::Json.export_bundle(&bundle).unwrap();
        assert!(json.contains("format_version"));
        assert!(json.contains("test"));
    }

    #[test]
    fn test_bundle_export_yaml() {
        let mut bundle = RecipeBundle::new("Test");
        bundle.add(make_recipe());

        let yaml = RecipeFormat::Yaml.export_bundle(&bundle).unwrap();
        assert!(yaml.contains("format_version"));
    }
}
