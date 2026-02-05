//! Built-in recipe library with common operations

use super::{Recipe, RecipeOperation};

/// Built-in recipes for common operations
pub struct BuiltinRecipes;

impl BuiltinRecipes {
    /// Get all built-in recipes
    pub fn all() -> Vec<Recipe> {
        vec![
            Self::base64_encode(),
            Self::base64_decode(),
            Self::hex_encode(),
            Self::hex_decode(),
            Self::sha256_hash(),
            Self::md5_hash(),
            Self::url_encode(),
            Self::url_decode(),
        ]
    }

    /// Get a built-in recipe by name
    pub fn get(name: &str) -> Option<Recipe> {
        Self::all().into_iter().find(|r| r.name == name)
    }

    /// Get the count of built-in recipes
    pub const fn count() -> usize {
        8
    }

    /// Base64 encode recipe
    pub fn base64_encode() -> Recipe {
        let mut recipe = Recipe::new("base64-encode", "1.0.0")
            .with_description("Encode data to Base64 format")
            .with_author("SPECTRE")
            .with_tag("encoding")
            .with_tag("base64")
            .with_category("encoding");
        recipe.add_operation(
            RecipeOperation::new("to_base64").with_description("Convert input to Base64 encoding"),
        );
        recipe
    }

    /// Base64 decode recipe
    pub fn base64_decode() -> Recipe {
        let mut recipe = Recipe::new("base64-decode", "1.0.0")
            .with_description("Decode Base64 encoded data")
            .with_author("SPECTRE")
            .with_tag("encoding")
            .with_tag("base64")
            .with_category("encoding");
        recipe.add_operation(
            RecipeOperation::new("from_base64").with_description("Decode Base64 to raw data"),
        );
        recipe
    }

    /// Hex encode recipe
    pub fn hex_encode() -> Recipe {
        let mut recipe = Recipe::new("hex-encode", "1.0.0")
            .with_description("Encode data to hexadecimal")
            .with_author("SPECTRE")
            .with_tag("encoding")
            .with_tag("hex")
            .with_category("encoding");
        recipe.add_operation(RecipeOperation::new("to_hex").with_description("Convert to hex"));
        recipe
    }

    /// Hex decode recipe
    pub fn hex_decode() -> Recipe {
        let mut recipe = Recipe::new("hex-decode", "1.0.0")
            .with_description("Decode hexadecimal data")
            .with_author("SPECTRE")
            .with_tag("encoding")
            .with_tag("hex")
            .with_category("encoding");
        recipe.add_operation(
            RecipeOperation::new("from_hex").with_description("Decode hex to raw data"),
        );
        recipe
    }

    /// SHA-256 hash recipe
    pub fn sha256_hash() -> Recipe {
        let mut recipe = Recipe::new("sha256-hash", "1.0.0")
            .with_description("Compute SHA-256 hash of data")
            .with_author("SPECTRE")
            .with_tag("hash")
            .with_tag("sha256")
            .with_category("hash");
        recipe.add_operation(
            RecipeOperation::new("sha256").with_description("Compute SHA-256 digest"),
        );
        recipe
    }

    /// MD5 hash recipe
    pub fn md5_hash() -> Recipe {
        let mut recipe = Recipe::new("md5-hash", "1.0.0")
            .with_description("Compute MD5 hash of data (for identification, not security)")
            .with_author("SPECTRE")
            .with_tag("hash")
            .with_tag("md5")
            .with_category("hash");
        recipe.add_operation(RecipeOperation::new("md5").with_description("Compute MD5 digest"));
        recipe
    }

    /// URL encode recipe
    pub fn url_encode() -> Recipe {
        let mut recipe = Recipe::new("url-encode", "1.0.0")
            .with_description("URL-encode a string")
            .with_author("SPECTRE")
            .with_tag("encoding")
            .with_tag("url")
            .with_category("encoding");
        recipe.add_operation(
            RecipeOperation::new("url_encode").with_description("URL-encode special characters"),
        );
        recipe
    }

    /// URL decode recipe
    pub fn url_decode() -> Recipe {
        let mut recipe = Recipe::new("url-decode", "1.0.0")
            .with_description("URL-decode a string")
            .with_author("SPECTRE")
            .with_tag("encoding")
            .with_tag("url")
            .with_category("encoding");
        recipe.add_operation(
            RecipeOperation::new("url_decode").with_description("Decode URL-encoded characters"),
        );
        recipe
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_count() {
        assert_eq!(BuiltinRecipes::count(), 8);
        assert_eq!(BuiltinRecipes::all().len(), 8);
    }

    #[test]
    fn test_get_builtin() {
        assert!(BuiltinRecipes::get("base64-encode").is_some());
        assert!(BuiltinRecipes::get("nonexistent").is_none());
    }

    #[test]
    fn test_base64_encode() {
        let recipe = BuiltinRecipes::base64_encode();
        assert_eq!(recipe.name, "base64-encode");
        assert!(recipe.tags.contains(&"base64".to_string()));
        assert_eq!(recipe.operation_count(), 1);
    }

    #[test]
    fn test_sha256_hash() {
        let recipe = BuiltinRecipes::sha256_hash();
        assert_eq!(recipe.category, "hash");
        assert!(recipe.tags.contains(&"sha256".to_string()));
    }

    #[test]
    fn test_all_recipes_valid() {
        for recipe in BuiltinRecipes::all() {
            assert!(!recipe.name.is_empty(), "Recipe name should not be empty");
            assert!(
                !recipe.operations.is_empty(),
                "Recipe {} should have operations",
                recipe.name
            );
            assert!(
                !recipe.version.is_empty(),
                "Recipe {} should have a version",
                recipe.name
            );
        }
    }

    #[test]
    fn test_all_unique_names() {
        let recipes = BuiltinRecipes::all();
        let mut names: Vec<_> = recipes.iter().map(|r| &r.name).collect();
        let original_len = names.len();
        names.sort();
        names.dedup();
        assert_eq!(
            names.len(),
            original_len,
            "All recipe names should be unique"
        );
    }
}
