//! Pipeline stage definitions and implementations

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Data flowing through the pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineData {
    /// No data
    Empty,
    /// Raw bytes
    Bytes(Vec<u8>),
    /// Text data
    Text(String),
    /// JSON data
    Json(serde_json::Value),
    /// Multiple items
    Items(Vec<Self>),
}

impl PipelineData {
    /// Get the number of items in this data
    pub const fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Bytes(_) | Self::Text(_) | Self::Json(_) => 1,
            Self::Items(items) => items.len(),
        }
    }

    /// Check if the data is empty
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Empty => Vec::new(),
            Self::Bytes(b) => b.clone(),
            Self::Text(t) => t.as_bytes().to_vec(),
            Self::Json(j) => serde_json::to_vec(j).unwrap_or_default(),
            Self::Items(items) => serde_json::to_vec(items).unwrap_or_default(),
        }
    }

    /// Convert to text
    pub fn to_text(&self) -> String {
        match self {
            Self::Empty => String::new(),
            Self::Bytes(b) => String::from_utf8_lossy(b).to_string(),
            Self::Text(t) => t.clone(),
            Self::Json(j) => serde_json::to_string_pretty(j).unwrap_or_default(),
            Self::Items(items) => serde_json::to_string_pretty(items).unwrap_or_default(),
        }
    }
}

/// Stage type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StageType {
    /// Scan stage (produces raw data)
    Scan,
    /// Analysis stage (CyberChef operations)
    Analysis,
    /// Filter stage (removes/selects data)
    Filter,
    /// Transform stage (modifies data format)
    Transform,
    /// Output stage (writes results)
    Output,
}

/// Stage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageConfig {
    /// Stage name
    pub name: String,
    /// Stage type
    pub stage_type: StageType,
    /// Configuration parameters
    pub params: serde_json::Value,
}

/// Trait for pipeline stages
///
/// Each stage takes input data, processes it, and returns output data.
#[async_trait]
pub trait Stage: Send + Sync {
    /// Get the stage name
    fn name(&self) -> &str;

    /// Get the stage type
    fn stage_type(&self) -> StageType;

    /// Process input data and produce output
    async fn process(&self, data: PipelineData) -> crate::Result<PipelineData>;
}

/// A passthrough stage that does not modify data (for testing)
pub struct PassthroughStage {
    name: String,
}

impl PassthroughStage {
    /// Create a new passthrough stage
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Stage for PassthroughStage {
    fn name(&self) -> &str {
        &self.name
    }

    fn stage_type(&self) -> StageType {
        StageType::Transform
    }

    async fn process(&self, data: PipelineData) -> crate::Result<PipelineData> {
        Ok(data)
    }
}

/// A stage that always fails (for testing error handling)
#[doc(hidden)]
pub struct FailingStage {
    name: String,
}

impl FailingStage {
    /// Create a new failing stage
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Stage for FailingStage {
    fn name(&self) -> &str {
        &self.name
    }

    fn stage_type(&self) -> StageType {
        StageType::Transform
    }

    async fn process(&self, _data: PipelineData) -> crate::Result<PipelineData> {
        Err(crate::SpectreError::Pipeline(
            crate::error::PipelineError::StageError {
                stage: self.name.clone(),
                message: "Intentional failure".to_string(),
            },
        ))
    }
}

/// A stage that transforms data to uppercase text
pub struct UppercaseStage {
    name: String,
}

impl UppercaseStage {
    /// Create a new uppercase stage
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Stage for UppercaseStage {
    fn name(&self) -> &str {
        &self.name
    }

    fn stage_type(&self) -> StageType {
        StageType::Transform
    }

    async fn process(&self, data: PipelineData) -> crate::Result<PipelineData> {
        let text = data.to_text().to_uppercase();
        Ok(PipelineData::Text(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_data_empty() {
        let data = PipelineData::Empty;
        assert!(data.is_empty());
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_pipeline_data_text() {
        let data = PipelineData::Text("hello".to_string());
        assert!(!data.is_empty());
        assert_eq!(data.len(), 1);
        assert_eq!(data.to_text(), "hello");
    }

    #[test]
    fn test_pipeline_data_bytes() {
        let data = PipelineData::Bytes(vec![1, 2, 3]);
        assert_eq!(data.to_bytes(), vec![1, 2, 3]);
    }

    #[test]
    fn test_pipeline_data_items() {
        let data = PipelineData::Items(vec![
            PipelineData::Text("a".to_string()),
            PipelineData::Text("b".to_string()),
        ]);
        assert_eq!(data.len(), 2);
    }

    #[tokio::test]
    async fn test_passthrough_stage() {
        let stage = PassthroughStage::new("test");
        assert_eq!(stage.name(), "test");

        let data = PipelineData::Text("hello".to_string());
        let result = stage.process(data).await.unwrap();
        assert!(matches!(result, PipelineData::Text(s) if s == "hello"));
    }

    #[tokio::test]
    async fn test_failing_stage() {
        let stage = FailingStage::new("fail");
        let data = PipelineData::Text("test".to_string());
        let result = stage.process(data).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_uppercase_stage() {
        let stage = UppercaseStage::new("upper");
        let data = PipelineData::Text("hello".to_string());
        let result = stage.process(data).await.unwrap();
        assert!(matches!(result, PipelineData::Text(s) if s == "HELLO"));
    }

    #[test]
    fn test_data_conversions() {
        let text_data = PipelineData::Text("hello".to_string());
        assert_eq!(text_data.to_bytes(), b"hello");

        let bytes_data = PipelineData::Bytes(b"world".to_vec());
        assert_eq!(bytes_data.to_text(), "world");

        let empty = PipelineData::Empty;
        assert!(empty.to_bytes().is_empty());
        assert!(empty.to_text().is_empty());
    }
}
