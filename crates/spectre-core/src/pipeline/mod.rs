//! Composable data pipeline module
//!
//! Provides a pipeline framework with typed stages for processing
//! scan data through analysis and output stages.
//!
//! # Pipeline Flow
//!
//! ```text
//! ScanStage -> AnalysisStage -> FilterStage -> OutputStage
//! ```

mod builder;
mod stage;

use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument, warn};

pub use builder::PipelineBuilder;
pub use stage::{PipelineData, Stage, StageConfig, StageType};

/// Pipeline configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Pipeline name
    pub name: String,
    /// Stage configurations
    pub stages: Vec<StageConfig>,
    /// Whether to continue on stage failure
    pub continue_on_error: bool,
}

/// Pipeline execution metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineMetrics {
    /// Total items processed
    pub items_processed: usize,
    /// Total errors encountered
    pub errors: usize,
    /// Stage-level metrics
    pub stage_metrics: Vec<StageMetrics>,
    /// Total pipeline execution time
    pub total_duration: Duration,
    /// Pipeline start time
    pub started_at: DateTime<Utc>,
    /// Pipeline end time
    pub completed_at: Option<DateTime<Utc>>,
}

/// Metrics for a single stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageMetrics {
    /// Stage name
    pub name: String,
    /// Items received
    pub items_in: usize,
    /// Items produced
    pub items_out: usize,
    /// Errors in this stage
    pub errors: usize,
    /// Stage execution time
    pub duration: Duration,
}

/// A composable data pipeline
///
/// Pipelines chain stages together, passing data from one stage
/// to the next. Each stage transforms the data in some way.
pub struct Pipeline {
    /// Pipeline name
    name: String,
    /// Pipeline stages
    stages: Vec<Box<dyn Stage>>,
    /// Whether to continue on stage failure
    continue_on_error: bool,
    /// Execution metrics
    metrics: PipelineMetrics,
}

impl Pipeline {
    /// Create a new pipeline with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stages: Vec::new(),
            continue_on_error: false,
            metrics: PipelineMetrics {
                items_processed: 0,
                errors: 0,
                stage_metrics: Vec::new(),
                total_duration: Duration::ZERO,
                started_at: Utc::now(),
                completed_at: None,
            },
        }
    }

    /// Create a pipeline builder
    pub fn builder(name: &str) -> PipelineBuilder {
        PipelineBuilder::new(name)
    }

    /// Add a stage to the pipeline
    pub fn add_stage(&mut self, stage: Box<dyn Stage>) {
        self.stages.push(stage);
    }

    /// Set whether to continue on stage errors
    pub const fn set_continue_on_error(&mut self, continue_on_error: bool) {
        self.continue_on_error = continue_on_error;
    }

    /// Get the pipeline name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the number of stages
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    /// Execute the pipeline with initial data
    #[instrument(skip(self, data), fields(pipeline = %self.name, stages = self.stages.len()))]
    pub async fn execute(&mut self, data: PipelineData) -> crate::Result<PipelineData> {
        if self.stages.is_empty() {
            return Err(crate::SpectreError::Pipeline(
                crate::error::PipelineError::NoStages,
            ));
        }

        info!(pipeline = %self.name, "Pipeline execution started");
        let pipeline_start = Instant::now();
        self.metrics.started_at = Utc::now();
        self.metrics.stage_metrics.clear();

        let mut current_data = data;

        for (idx, stage) in self.stages.iter().enumerate() {
            let stage_name = stage.name().to_string();
            let stage_start = Instant::now();
            let items_in = current_data.len();

            debug!(
                stage = %stage_name,
                stage_idx = idx,
                items_in = items_in,
                "Executing stage"
            );

            match stage.process(current_data.clone()).await {
                Ok(result) => {
                    let items_out = result.len();
                    let duration = stage_start.elapsed();

                    self.metrics.stage_metrics.push(StageMetrics {
                        name: stage_name.clone(),
                        items_in,
                        items_out,
                        errors: 0,
                        duration,
                    });

                    self.metrics.items_processed += items_out;
                    current_data = result;

                    debug!(
                        stage = %stage_name,
                        items_out = items_out,
                        duration_ms = duration.as_millis(),
                        "Stage complete"
                    );
                },
                Err(e) => {
                    self.metrics.errors += 1;
                    self.metrics.stage_metrics.push(StageMetrics {
                        name: stage_name.clone(),
                        items_in,
                        items_out: 0,
                        errors: 1,
                        duration: stage_start.elapsed(),
                    });

                    if self.continue_on_error {
                        warn!(
                            stage = %stage_name,
                            error = %e,
                            "Stage failed, continuing"
                        );
                        // Continue with the data we had
                    } else {
                        return Err(crate::SpectreError::Pipeline(
                            crate::error::PipelineError::StageError {
                                stage: stage_name,
                                message: e.to_string(),
                            },
                        ));
                    }
                },
            }
        }

        self.metrics.total_duration = pipeline_start.elapsed();
        self.metrics.completed_at = Some(Utc::now());

        info!(
            pipeline = %self.name,
            duration_ms = self.metrics.total_duration.as_millis(),
            items = self.metrics.items_processed,
            errors = self.metrics.errors,
            "Pipeline execution complete"
        );

        Ok(current_data)
    }

    /// Get the pipeline metrics
    pub const fn metrics(&self) -> &PipelineMetrics {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_new() {
        let pipeline = Pipeline::new("test");
        assert_eq!(pipeline.name(), "test");
        assert_eq!(pipeline.stage_count(), 0);
    }

    #[tokio::test]
    async fn test_pipeline_no_stages() {
        let mut pipeline = Pipeline::new("test");
        let result = pipeline.execute(PipelineData::Empty).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pipeline_with_passthrough() {
        let mut pipeline = Pipeline::new("test");
        pipeline.add_stage(Box::new(stage::PassthroughStage::new("passthrough")));

        let data = PipelineData::Text("hello".to_string());
        let result = pipeline.execute(data).await.unwrap();

        assert!(matches!(result, PipelineData::Text(s) if s == "hello"));
    }

    #[tokio::test]
    async fn test_pipeline_metrics() {
        let mut pipeline = Pipeline::new("test");
        pipeline.add_stage(Box::new(stage::PassthroughStage::new("stage-1")));

        let data = PipelineData::Text("test".to_string());
        pipeline.execute(data).await.unwrap();

        let metrics = pipeline.metrics();
        assert_eq!(metrics.stage_metrics.len(), 1);
        assert_eq!(metrics.errors, 0);
        assert!(metrics.completed_at.is_some());
    }

    #[tokio::test]
    async fn test_pipeline_continue_on_error() {
        let mut pipeline = Pipeline::new("test");
        pipeline.set_continue_on_error(true);
        pipeline.add_stage(Box::new(stage::FailingStage::new("fail")));
        pipeline.add_stage(Box::new(stage::PassthroughStage::new("pass")));

        let data = PipelineData::Text("test".to_string());
        let result = pipeline.execute(data).await;
        assert!(result.is_ok());
        assert_eq!(pipeline.metrics().errors, 1);
    }

    #[tokio::test]
    async fn test_pipeline_stop_on_error() {
        let mut pipeline = Pipeline::new("test");
        pipeline.add_stage(Box::new(stage::FailingStage::new("fail")));
        pipeline.add_stage(Box::new(stage::PassthroughStage::new("pass")));

        let data = PipelineData::Text("test".to_string());
        let result = pipeline.execute(data).await;
        assert!(result.is_err());
    }
}
