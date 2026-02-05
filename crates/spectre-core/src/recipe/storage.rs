//! Recipe storage — file-based storage with index

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use tracing::{debug, info, warn};

use super::Recipe;

/// File-based recipe store
#[derive(Debug, Clone)]
pub struct RecipeStore {
    store_dir: PathBuf,
    index: HashMap<String, RecipeIndexEntry>,
}

/// Index entry for a stored recipe
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RecipeIndexEntry {
    name: String,
    version: String,
    category: String,
    tags: Vec<String>,
    file_name: String,
}

impl RecipeStore {
    /// Create a new recipe store
    pub fn new(store_dir: &Path) -> Self {
        let mut store = Self {
            store_dir: store_dir.to_path_buf(),
            index: HashMap::new(),
        };
        let _ = store.rebuild_index();
        store
    }

    /// Save a recipe
    pub fn save(&mut self, recipe: &Recipe) -> crate::Result<()> {
        std::fs::create_dir_all(&self.store_dir)?;

        let file_name = format!("{}.json", recipe.name);
        let path = self.store_dir.join(&file_name);

        let json = serde_json::to_string_pretty(recipe).map_err(|e| {
            crate::SpectreError::Recipe(crate::error::RecipeError::StorageError(format!(
                "Serialization error: {}",
                e
            )))
        })?;

        std::fs::write(&path, json)?;

        self.index.insert(
            recipe.name.clone(),
            RecipeIndexEntry {
                name: recipe.name.clone(),
                version: recipe.version.clone(),
                category: recipe.category.clone(),
                tags: recipe.tags.clone(),
                file_name,
            },
        );

        debug!(name = %recipe.name, "Recipe saved");
        Ok(())
    }

    /// Load a recipe by name
    pub fn load(&self, name: &str) -> crate::Result<Recipe> {
        let entry = self.index.get(name).ok_or_else(|| {
            crate::SpectreError::Recipe(crate::error::RecipeError::NotFound(name.to_string()))
        })?;

        let path = self.store_dir.join(&entry.file_name);
        let content = std::fs::read_to_string(&path)?;

        serde_json::from_str(&content).map_err(|e| {
            crate::SpectreError::Recipe(crate::error::RecipeError::StorageError(format!(
                "Deserialization error: {}",
                e
            )))
        })
    }

    /// Delete a recipe
    pub fn delete(&mut self, name: &str) -> crate::Result<bool> {
        if let Some(entry) = self.index.remove(name) {
            let path = self.store_dir.join(&entry.file_name);
            if path.exists() {
                std::fs::remove_file(&path)?;
            }
            debug!(name = %name, "Recipe deleted");
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// List all recipe names
    pub fn list(&self) -> Vec<&str> {
        let mut names: Vec<_> = self.index.keys().map(String::as_str).collect();
        names.sort_unstable();
        names
    }

    /// Get recipe count
    pub fn count(&self) -> usize {
        self.index.len()
    }

    /// Search recipes by category
    pub fn by_category(&self, category: &str) -> Vec<&str> {
        self.index
            .values()
            .filter(|e| e.category == category)
            .map(|e| e.name.as_str())
            .collect()
    }

    /// Search recipes by tag
    pub fn by_tag(&self, tag: &str) -> Vec<&str> {
        self.index
            .values()
            .filter(|e| e.tags.iter().any(|t| t == tag))
            .map(|e| e.name.as_str())
            .collect()
    }

    /// Rebuild the index by scanning the store directory
    fn rebuild_index(&mut self) -> crate::Result<()> {
        self.index.clear();

        if !self.store_dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(&self.store_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                match std::fs::read_to_string(&path) {
                    Ok(content) => match serde_json::from_str::<Recipe>(&content) {
                        Ok(recipe) => {
                            let file_name = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("")
                                .to_string();
                            self.index.insert(
                                recipe.name.clone(),
                                RecipeIndexEntry {
                                    name: recipe.name.clone(),
                                    version: recipe.version.clone(),
                                    category: recipe.category.clone(),
                                    tags: recipe.tags.clone(),
                                    file_name,
                                },
                            );
                        },
                        Err(e) => {
                            warn!(path = %path.display(), error = %e, "Failed to parse recipe");
                        },
                    },
                    Err(e) => warn!(path = %path.display(), error = %e, "Failed to read recipe"),
                }
            }
        }

        info!(count = self.index.len(), "Recipe index rebuilt");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::RecipeOperation;

    fn make_recipe(name: &str) -> Recipe {
        let mut recipe = Recipe::new(name, "1.0.0")
            .with_description("Test recipe")
            .with_tag("test")
            .with_category("encoding");
        recipe.add_operation(RecipeOperation::new("base64_encode"));
        recipe
    }

    #[test]
    fn test_recipe_store_new() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecipeStore::new(dir.path());
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = RecipeStore::new(dir.path());

        let recipe = make_recipe("test-recipe");
        store.save(&recipe).unwrap();

        let loaded = store.load("test-recipe").unwrap();
        assert_eq!(loaded.name, "test-recipe");
        assert_eq!(loaded.operation_count(), 1);
    }

    #[test]
    fn test_load_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecipeStore::new(dir.path());
        assert!(store.load("nonexistent").is_err());
    }

    #[test]
    fn test_delete() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = RecipeStore::new(dir.path());

        store.save(&make_recipe("test")).unwrap();
        assert!(store.delete("test").unwrap());
        assert!(!store.delete("test").unwrap());
    }

    #[test]
    fn test_list() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = RecipeStore::new(dir.path());

        store.save(&make_recipe("b-recipe")).unwrap();
        store.save(&make_recipe("a-recipe")).unwrap();

        let list = store.list();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], "a-recipe");
    }

    #[test]
    fn test_by_category() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = RecipeStore::new(dir.path());

        store
            .save(&make_recipe("test").with_category("encoding"))
            .unwrap();
        store
            .save(&make_recipe("test2").with_category("crypto"))
            .unwrap();

        assert_eq!(store.by_category("encoding").len(), 1);
        assert_eq!(store.by_category("crypto").len(), 1);
    }

    #[test]
    fn test_by_tag() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = RecipeStore::new(dir.path());

        store.save(&make_recipe("test")).unwrap();
        assert_eq!(store.by_tag("test").len(), 1);
        assert_eq!(store.by_tag("nonexistent").len(), 0);
    }

    #[test]
    fn test_rebuild_index() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = RecipeStore::new(dir.path());

        store.save(&make_recipe("test")).unwrap();

        // Create a new store to trigger index rebuild
        let store2 = RecipeStore::new(dir.path());
        assert_eq!(store2.count(), 1);
    }
}
