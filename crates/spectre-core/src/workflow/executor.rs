//! Workflow executor — async execution engine with step tracking

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use super::dsl::WorkflowDef;
use super::variables::VariableStore;

/// Status of a workflow execution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// Not yet started
    Pending,
    /// Currently executing
    Running,
    /// Completed successfully
    Completed,
    /// Failed with error
    Failed,
    /// Cancelled by user
    Cancelled,
}

/// Status of a single step execution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    /// Not yet executed
    Pending,
    /// Currently executing
    Running,
    /// Completed successfully
    Completed,
    /// Failed
    Failed,
    /// Skipped (condition not met)
    Skipped,
}

/// Result of a single step execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step name
    pub step_name: String,
    /// Step status
    pub status: StepStatus,
    /// Output (captured result)
    pub output: String,
    /// Error message if failed
    pub error: Option<String>,
    /// Execution start time
    pub started_at: Option<DateTime<Utc>>,
    /// Execution end time
    pub completed_at: Option<DateTime<Utc>>,
    /// Retry attempts used
    pub retries_used: u32,
}

/// Workflow execution engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutor {
    /// Workflow definition
    pub workflow: WorkflowDef,
    /// Current status
    pub status: WorkflowStatus,
    /// Variable store
    pub variables: VariableStore,
    /// Results for each step
    pub step_results: Vec<StepResult>,
    /// Current step index
    pub current_step: usize,
    /// Execution start time
    pub started_at: Option<DateTime<Utc>>,
    /// Execution end time
    pub completed_at: Option<DateTime<Utc>>,
}

impl WorkflowExecutor {
    /// Create a new workflow executor
    pub fn new(workflow: WorkflowDef) -> Self {
        let initial_vars = VariableStore::from_map(workflow.variables.clone());
        Self {
            workflow,
            status: WorkflowStatus::Pending,
            variables: initial_vars,
            step_results: Vec::new(),
            current_step: 0,
            started_at: None,
            completed_at: None,
        }
    }

    /// Execute the workflow (simulated — real execution would use actual scan/report engines)
    pub async fn execute(&mut self) -> crate::Result<()> {
        self.status = WorkflowStatus::Running;
        self.started_at = Some(Utc::now());
        self.step_results.clear();

        info!(workflow = %self.workflow.name, "Starting workflow execution");

        let steps = self.workflow.steps.clone();

        for (idx, step) in steps.iter().enumerate() {
            self.current_step = idx;

            // Check condition
            if let Some(ref condition) = step.condition {
                let actual = self
                    .variables
                    .get(&condition.variable)
                    .map(|s| s.to_string());
                if !condition.evaluate(actual.as_deref()) {
                    debug!(step = %step.name, "Step skipped (condition not met)");
                    self.step_results.push(StepResult {
                        step_name: step.name.clone(),
                        status: StepStatus::Skipped,
                        output: String::new(),
                        error: None,
                        started_at: None,
                        completed_at: None,
                        retries_used: 0,
                    });
                    continue;
                }
            }

            // Handle loops
            if let Some(ref loop_spec) = step.loop_spec {
                let source_value = self
                    .variables
                    .get(&loop_spec.over)
                    .unwrap_or("")
                    .to_string();
                let items = loop_spec.parse_items(&source_value);

                let mut loop_outputs = Vec::new();
                for item in &items {
                    self.variables.set(&loop_spec.variable, item);
                    let output = self.execute_single_step(step, idx).await;
                    loop_outputs.push(output);
                }

                let combined = loop_outputs.join(", ");
                if let Some(ref capture) = step.capture {
                    self.variables.set(capture, &combined);
                }

                self.step_results.push(StepResult {
                    step_name: step.name.clone(),
                    status: StepStatus::Completed,
                    output: combined,
                    error: None,
                    started_at: Some(Utc::now()),
                    completed_at: Some(Utc::now()),
                    retries_used: 0,
                });
                continue;
            }

            // Execute step with retry support
            let mut result = StepResult {
                step_name: step.name.clone(),
                status: StepStatus::Running,
                output: String::new(),
                error: None,
                started_at: Some(Utc::now()),
                completed_at: None,
                retries_used: 0,
            };

            let mut success = false;
            let max_attempts = step.retries + 1;

            for attempt in 0..max_attempts {
                result.retries_used = attempt;
                let output = self.execute_single_step(step, idx).await;
                result.output = output;
                // In a real implementation, we would check if the step succeeded
                // and only break on success or continue to retry on failure.
                // For now, simulated steps always succeed.
                success = true;
                if success {
                    break;
                }
            }

            result.completed_at = Some(Utc::now());

            if success {
                result.status = StepStatus::Completed;
                if let Some(ref capture) = step.capture {
                    self.variables.set(capture, &result.output);
                }
            } else {
                result.status = StepStatus::Failed;
                result.error = Some("Step execution failed after retries".to_string());

                if !step.continue_on_error {
                    self.step_results.push(result);
                    self.status = WorkflowStatus::Failed;
                    self.completed_at = Some(Utc::now());
                    return Err(crate::SpectreError::Workflow(
                        crate::error::WorkflowError::StepError {
                            step: step.name.clone(),
                            message: "Failed after retries".to_string(),
                        },
                    ));
                }
                warn!(step = %step.name, "Step failed, continuing");
            }

            self.step_results.push(result);
        }

        self.status = WorkflowStatus::Completed;
        self.completed_at = Some(Utc::now());
        info!(workflow = %self.workflow.name, "Workflow execution completed");

        Ok(())
    }

    /// Execute a single step (simulated)
    async fn execute_single_step(&self, step: &super::dsl::StepDef, _idx: usize) -> String {
        // Substitute variables in parameters
        let action = self.variables.substitute(&step.action);

        // In a real implementation, this would dispatch to scan/report/export engines
        // For now, return a simulated output
        debug!(step = %step.name, action = %action, "Executing step");

        format!("{}:completed", step.name)
    }

    /// Get the number of completed steps
    pub fn completed_steps(&self) -> usize {
        self.step_results
            .iter()
            .filter(|r| r.status == StepStatus::Completed)
            .count()
    }

    /// Get the number of failed steps
    pub fn failed_steps(&self) -> usize {
        self.step_results
            .iter()
            .filter(|r| r.status == StepStatus::Failed)
            .count()
    }

    /// Get the number of skipped steps
    pub fn skipped_steps(&self) -> usize {
        self.step_results
            .iter()
            .filter(|r| r.status == StepStatus::Skipped)
            .count()
    }

    /// Get progress as fraction (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        let total = self.workflow.steps.len();
        if total == 0 {
            return 0.0;
        }
        self.step_results.len() as f64 / total as f64
    }

    /// Cancel the workflow
    pub fn cancel(&mut self) {
        self.status = WorkflowStatus::Cancelled;
        self.completed_at = Some(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::dsl::{Condition, LoopSpec, StepDef};

    fn make_workflow() -> WorkflowDef {
        let mut wf = WorkflowDef::new("test-workflow");
        wf.add_step(StepDef::new("step-1", "scan"));
        wf.add_step(StepDef::new("step-2", "report"));
        wf
    }

    #[tokio::test]
    async fn test_executor_basic() {
        let wf = make_workflow();
        let mut executor = WorkflowExecutor::new(wf);
        assert!(matches!(executor.status, WorkflowStatus::Pending));

        executor.execute().await.unwrap();
        assert!(matches!(executor.status, WorkflowStatus::Completed));
        assert_eq!(executor.completed_steps(), 2);
    }

    #[tokio::test]
    async fn test_executor_with_condition_skip() {
        let mut wf = WorkflowDef::new("test");
        wf.add_step(StepDef::new("step-1", "scan").with_capture("result"));
        wf.add_step(
            StepDef::new("step-2", "report")
                .with_condition(Condition::equals("nonexistent", "value")),
        );

        let mut executor = WorkflowExecutor::new(wf);
        executor.execute().await.unwrap();

        assert_eq!(executor.completed_steps(), 1);
        assert_eq!(executor.skipped_steps(), 1);
    }

    #[tokio::test]
    async fn test_executor_with_loop() {
        let mut wf = WorkflowDef::new("test");
        wf.set_variable("targets", "10.0.0.1,10.0.0.2,10.0.0.3");
        wf.add_step(
            StepDef::new("scan-each", "scan").with_loop(LoopSpec::new("targets", "target")),
        );

        let mut executor = WorkflowExecutor::new(wf);
        executor.execute().await.unwrap();
        assert_eq!(executor.completed_steps(), 1);
    }

    #[tokio::test]
    async fn test_executor_variable_capture() {
        let mut wf = WorkflowDef::new("test");
        wf.add_step(StepDef::new("step-1", "scan").with_capture("scan_output"));

        let mut executor = WorkflowExecutor::new(wf);
        executor.execute().await.unwrap();

        assert!(executor.variables.get("scan_output").is_some());
    }

    #[tokio::test]
    async fn test_executor_progress() {
        let wf = make_workflow();
        let mut executor = WorkflowExecutor::new(wf);

        assert!((executor.progress() - 0.0).abs() < f64::EPSILON);
        executor.execute().await.unwrap();
        assert!((executor.progress() - 1.0).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_executor_cancel() {
        let wf = make_workflow();
        let mut executor = WorkflowExecutor::new(wf);
        executor.cancel();
        assert!(matches!(executor.status, WorkflowStatus::Cancelled));
    }

    #[test]
    fn test_step_status_values() {
        assert!(matches!(StepStatus::Pending, StepStatus::Pending));
        assert!(matches!(StepStatus::Running, StepStatus::Running));
        assert!(matches!(StepStatus::Completed, StepStatus::Completed));
        assert!(matches!(StepStatus::Failed, StepStatus::Failed));
        assert!(matches!(StepStatus::Skipped, StepStatus::Skipped));
    }

    #[test]
    fn test_workflow_status_values() {
        assert!(matches!(WorkflowStatus::Pending, WorkflowStatus::Pending));
        assert!(matches!(WorkflowStatus::Running, WorkflowStatus::Running));
    }

    #[tokio::test]
    async fn test_executor_empty_workflow() {
        let wf = WorkflowDef::new("empty");
        let mut executor = WorkflowExecutor::new(wf);
        executor.execute().await.unwrap();
        assert!(matches!(executor.status, WorkflowStatus::Completed));
        assert_eq!(executor.completed_steps(), 0);
    }
}
