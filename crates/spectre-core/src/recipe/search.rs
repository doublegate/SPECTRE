//! Recipe search functionality

use super::Recipe;

/// Recipe search engine
pub struct RecipeSearch;

impl RecipeSearch {
    /// Search recipes by name (case-insensitive substring match)
    pub fn by_name<'a>(recipes: &'a [Recipe], query: &str) -> Vec<&'a Recipe> {
        let query_lower = query.to_lowercase();
        recipes
            .iter()
            .filter(|r| r.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Search recipes by tag
    pub fn by_tag<'a>(recipes: &'a [Recipe], tag: &str) -> Vec<&'a Recipe> {
        let tag_lower = tag.to_lowercase();
        recipes
            .iter()
            .filter(|r| r.tags.iter().any(|t| t.to_lowercase() == tag_lower))
            .collect()
    }

    /// Search recipes by category
    pub fn by_category<'a>(recipes: &'a [Recipe], category: &str) -> Vec<&'a Recipe> {
        let cat_lower = category.to_lowercase();
        recipes
            .iter()
            .filter(|r| r.category.to_lowercase() == cat_lower)
            .collect()
    }

    /// Search recipes by operation type
    pub fn by_operation<'a>(recipes: &'a [Recipe], operation: &str) -> Vec<&'a Recipe> {
        let op_lower = operation.to_lowercase();
        recipes
            .iter()
            .filter(|r| {
                r.operations
                    .iter()
                    .any(|o| o.name.to_lowercase().contains(&op_lower))
            })
            .collect()
    }

    /// Full-text search across name, description, tags, and operations
    pub fn full_text<'a>(recipes: &'a [Recipe], query: &str) -> Vec<&'a Recipe> {
        let query_lower = query.to_lowercase();
        recipes
            .iter()
            .filter(|r| {
                r.name.to_lowercase().contains(&query_lower)
                    || r.description.to_lowercase().contains(&query_lower)
                    || r.tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query_lower))
                    || r.operations
                        .iter()
                        .any(|o| o.name.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::RecipeOperation;

    fn make_recipes() -> Vec<Recipe> {
        vec![
            {
                let mut r = Recipe::new("base64-encode", "1.0.0")
                    .with_tag("encoding")
                    .with_category("crypto")
                    .with_description("Encode data to Base64");
                r.add_operation(RecipeOperation::new("to_base64"));
                r
            },
            {
                let mut r = Recipe::new("hex-decode", "1.0.0")
                    .with_tag("encoding")
                    .with_tag("hex")
                    .with_category("crypto")
                    .with_description("Decode hex strings");
                r.add_operation(RecipeOperation::new("from_hex"));
                r
            },
            {
                let mut r = Recipe::new("sha256-hash", "1.0.0")
                    .with_tag("hash")
                    .with_category("hash")
                    .with_description("Compute SHA-256 hash");
                r.add_operation(RecipeOperation::new("sha256"));
                r
            },
        ]
    }

    #[test]
    fn test_search_by_name() {
        let recipes = make_recipes();
        let results = RecipeSearch::by_name(&recipes, "base64");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "base64-encode");
    }

    #[test]
    fn test_search_by_name_case_insensitive() {
        let recipes = make_recipes();
        let results = RecipeSearch::by_name(&recipes, "BASE64");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_by_tag() {
        let recipes = make_recipes();
        let results = RecipeSearch::by_tag(&recipes, "encoding");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_by_category() {
        let recipes = make_recipes();
        let results = RecipeSearch::by_category(&recipes, "crypto");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_by_operation() {
        let recipes = make_recipes();
        let results = RecipeSearch::by_operation(&recipes, "sha256");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_full_text_search() {
        let recipes = make_recipes();
        let results = RecipeSearch::full_text(&recipes, "encode");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_no_results() {
        let recipes = make_recipes();
        let results = RecipeSearch::by_name(&recipes, "nonexistent");
        assert!(results.is_empty());
    }
}
