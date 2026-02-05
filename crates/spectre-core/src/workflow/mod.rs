//! Workflow automation module
//!
//! Provides a YAML-based workflow DSL, parser, executor with variable
//! substitution, conditionals, loops, templates, and persistence.

mod dsl;
mod executor;
mod parser;
mod persistence;
mod template;
mod variables;

pub use dsl::{Condition, LoopSpec, StepDef, WorkflowDef};
pub use executor::{StepResult, StepStatus, WorkflowExecutor, WorkflowStatus};
pub use parser::WorkflowParser;
pub use persistence::WorkflowStore;
pub use template::WorkflowTemplates;
pub use variables::VariableStore;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_reexports() {
        let vars = VariableStore::new();
        assert_eq!(vars.count(), 0);

        let templates = WorkflowTemplates::new();
        assert!(templates.count() > 0);
    }
}
