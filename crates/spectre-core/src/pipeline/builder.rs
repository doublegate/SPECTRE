//! Pipeline builder for ergonomic pipeline construction

use super::stage::{PassthroughStage, Stage, UppercaseStage};
use super::Pipeline;

/// Builder for constructing pipelines
pub struct PipelineBuilder {
    name: String,
    stages: Vec<Box<dyn Stage>>,
    continue_on_error: bool,
}

impl PipelineBuilder {
    /// Create a new pipeline builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stages: Vec::new(),
            continue_on_error: false,
        }
    }

    /// Add a custom stage
    pub fn stage(mut self, stage: Box<dyn Stage>) -> Self {
        self.stages.push(stage);
        self
    }

    /// Add a passthrough stage
    pub fn passthrough(mut self, name: &str) -> Self {
        self.stages.push(Box::new(PassthroughStage::new(name)));
        self
    }

    /// Add an uppercase transform stage
    pub fn uppercase(mut self, name: &str) -> Self {
        self.stages.push(Box::new(UppercaseStage::new(name)));
        self
    }

    /// Set whether to continue on stage errors
    pub fn continue_on_error(mut self, yes: bool) -> Self {
        self.continue_on_error = yes;
        self
    }

    /// Build the pipeline
    pub fn build(self) -> Pipeline {
        let mut pipeline = Pipeline::new(&self.name);
        pipeline.set_continue_on_error(self.continue_on_error);
        for stage in self.stages {
            pipeline.add_stage(stage);
        }
        pipeline
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::stage::PipelineData;

    #[tokio::test]
    async fn test_builder_basic() {
        let mut pipeline = PipelineBuilder::new("test").passthrough("stage-1").build();

        assert_eq!(pipeline.name(), "test");
        assert_eq!(pipeline.stage_count(), 1);

        let result = pipeline
            .execute(PipelineData::Text("hello".to_string()))
            .await
            .unwrap();
        assert!(matches!(result, PipelineData::Text(s) if s == "hello"));
    }

    #[tokio::test]
    async fn test_builder_chain() {
        let mut pipeline = PipelineBuilder::new("chain")
            .passthrough("input")
            .uppercase("upper")
            .build();

        let result = pipeline
            .execute(PipelineData::Text("hello".to_string()))
            .await
            .unwrap();
        assert!(matches!(result, PipelineData::Text(s) if s == "HELLO"));
    }

    #[tokio::test]
    async fn test_builder_continue_on_error() {
        use crate::pipeline::stage::FailingStage;

        let mut pipeline = PipelineBuilder::new("resilient")
            .stage(Box::new(FailingStage::new("fail")))
            .passthrough("pass")
            .continue_on_error(true)
            .build();

        let result = pipeline
            .execute(PipelineData::Text("test".to_string()))
            .await;
        assert!(result.is_ok());
    }
}
