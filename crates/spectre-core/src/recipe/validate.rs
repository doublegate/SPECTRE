//! Recipe validation

use super::Recipe;

/// Recipe validator
pub struct RecipeValidator;

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the recipe is valid
    pub valid: bool,
    /// Validation errors
    pub errors: Vec<String>,
    /// Validation warnings
    pub warnings: Vec<String>,
}

impl RecipeValidator {
    /// Validate a recipe
    pub fn validate(recipe: &Recipe) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Name must not be empty
        if recipe.name.is_empty() {
            errors.push("Recipe name is required".to_string());
        }

        // Name must be valid identifier (alphanumeric + hyphens)
        if !recipe.name.is_empty()
            && !recipe
                .name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            errors.push(
                "Recipe name must contain only alphanumeric characters, hyphens, and underscores"
                    .to_string(),
            );
        }

        // Version must not be empty
        if recipe.version.is_empty() {
            errors.push("Recipe version is required".to_string());
        }

        // Version should look like semver
        if !recipe.version.is_empty() && !Self::is_valid_semver(&recipe.version) {
            warnings.push(format!(
                "Version '{}' does not follow semantic versioning",
                recipe.version
            ));
        }

        // Must have at least one operation
        if recipe.operations.is_empty() {
            errors.push("Recipe must have at least one operation".to_string());
        }

        // Operations must have names
        for (i, op) in recipe.operations.iter().enumerate() {
            if op.name.is_empty() {
                errors.push(format!("Operation {} has no name", i));
            }
        }

        // Description is recommended
        if recipe.description.is_empty() {
            warnings.push("Recipe should have a description".to_string());
        }

        // Author is recommended
        if recipe.author.is_empty() {
            warnings.push("Recipe should have an author".to_string());
        }

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Check if a string is valid semver (simplified)
    fn is_valid_semver(version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return false;
        }
        parts.iter().all(|p| p.parse::<u32>().is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::RecipeOperation;

    #[test]
    fn test_valid_recipe() {
        let mut recipe = Recipe::new("test-recipe", "1.0.0")
            .with_description("Test")
            .with_author("tester");
        recipe.add_operation(RecipeOperation::new("op1"));

        let result = RecipeValidator::validate(&recipe);
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_empty_name() {
        let mut recipe = Recipe::new("", "1.0.0");
        recipe.add_operation(RecipeOperation::new("op1"));

        let result = RecipeValidator::validate(&recipe);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("name")));
    }

    #[test]
    fn test_empty_version() {
        let mut recipe = Recipe::new("test", "");
        recipe.add_operation(RecipeOperation::new("op1"));

        let result = RecipeValidator::validate(&recipe);
        assert!(!result.valid);
    }

    #[test]
    fn test_invalid_semver() {
        let mut recipe = Recipe::new("test", "abc");
        recipe.add_operation(RecipeOperation::new("op1"));

        let result = RecipeValidator::validate(&recipe);
        assert!(result.warnings.iter().any(|w| w.contains("semantic")));
    }

    #[test]
    fn test_no_operations() {
        let recipe = Recipe::new("test", "1.0.0");
        let result = RecipeValidator::validate(&recipe);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("operation")));
    }

    #[test]
    fn test_empty_operation_name() {
        let mut recipe = Recipe::new("test", "1.0.0");
        recipe.add_operation(RecipeOperation::new(""));

        let result = RecipeValidator::validate(&recipe);
        assert!(!result.valid);
    }

    #[test]
    fn test_missing_description_warning() {
        let mut recipe = Recipe::new("test", "1.0.0");
        recipe.add_operation(RecipeOperation::new("op1"));

        let result = RecipeValidator::validate(&recipe);
        assert!(result.warnings.iter().any(|w| w.contains("description")));
    }

    #[test]
    fn test_invalid_name_chars() {
        let mut recipe = Recipe::new("test recipe!", "1.0.0");
        recipe.add_operation(RecipeOperation::new("op1"));

        let result = RecipeValidator::validate(&recipe);
        assert!(!result.valid);
    }

    #[test]
    fn test_valid_semver_check() {
        assert!(RecipeValidator::is_valid_semver("1.0.0"));
        assert!(RecipeValidator::is_valid_semver("0.1.0"));
        assert!(!RecipeValidator::is_valid_semver("1.0"));
        assert!(!RecipeValidator::is_valid_semver("abc"));
    }
}
