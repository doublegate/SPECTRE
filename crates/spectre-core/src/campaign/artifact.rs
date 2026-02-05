//! Campaign artifact management
//!
//! Artifacts are outputs produced during a campaign, such as
//! scan results, analysis reports, and screenshots.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of campaign artifacts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactType {
    /// Scan results (JSON)
    ScanResult,
    /// Analysis output
    AnalysisOutput,
    /// Finding report
    FindingReport,
    /// Screenshot
    Screenshot,
    /// Log file
    LogFile,
    /// Configuration snapshot
    ConfigSnapshot,
    /// Custom artifact type
    Custom(String),
}

impl std::fmt::Display for ArtifactType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScanResult => write!(f, "scan_result"),
            Self::AnalysisOutput => write!(f, "analysis_output"),
            Self::FindingReport => write!(f, "finding_report"),
            Self::Screenshot => write!(f, "screenshot"),
            Self::LogFile => write!(f, "log_file"),
            Self::ConfigSnapshot => write!(f, "config_snapshot"),
            Self::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// A campaign artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    /// Unique artifact identifier
    pub id: String,
    /// Campaign this artifact belongs to
    pub campaign_id: String,
    /// Artifact type
    pub artifact_type: ArtifactType,
    /// Human-readable name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// MIME type
    pub mime_type: String,
    /// Artifact data (base64 encoded for binary)
    pub data: Vec<u8>,
    /// Size in bytes
    pub size: usize,
    /// SHA-256 hash of data
    pub hash: String,
    /// When the artifact was created
    pub created_at: DateTime<Utc>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl Artifact {
    /// Create a new artifact
    pub fn new(campaign_id: &str, artifact_type: ArtifactType, name: &str, data: Vec<u8>) -> Self {
        use sha2::{Digest, Sha256};

        let size = data.len();
        let hash = hex::encode(Sha256::digest(&data));

        let mime_type = match &artifact_type {
            ArtifactType::ScanResult | ArtifactType::FindingReport => "application/json",
            ArtifactType::AnalysisOutput | ArtifactType::LogFile => "text/plain",
            ArtifactType::Screenshot => "image/png",
            ArtifactType::ConfigSnapshot => "application/toml",
            ArtifactType::Custom(_) => "application/octet-stream",
        };

        Self {
            id: Uuid::new_v4().to_string(),
            campaign_id: campaign_id.to_string(),
            artifact_type,
            name: name.to_string(),
            description: None,
            mime_type: mime_type.to_string(),
            data,
            size,
            hash,
            created_at: Utc::now(),
            tags: Vec::new(),
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(tag.to_string());
    }

    /// Get the data as a string (if valid UTF-8)
    pub fn as_text(&self) -> Option<String> {
        String::from_utf8(self.data.clone()).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_creation() {
        let artifact = Artifact::new(
            "campaign-123",
            ArtifactType::ScanResult,
            "scan-output.json",
            b"{}".to_vec(),
        );

        assert!(!artifact.id.is_empty());
        assert_eq!(artifact.campaign_id, "campaign-123");
        assert_eq!(artifact.name, "scan-output.json");
        assert_eq!(artifact.size, 2);
        assert!(!artifact.hash.is_empty());
        assert_eq!(artifact.mime_type, "application/json");
    }

    #[test]
    fn test_artifact_with_description() {
        let artifact = Artifact::new(
            "c1",
            ArtifactType::LogFile,
            "scan.log",
            b"log data".to_vec(),
        )
        .with_description("Scan log file");

        assert_eq!(artifact.description.as_deref(), Some("Scan log file"));
    }

    #[test]
    fn test_artifact_tags() {
        let mut artifact = Artifact::new(
            "c1",
            ArtifactType::ScanResult,
            "results.json",
            b"[]".to_vec(),
        );

        artifact.add_tag("scan");
        artifact.add_tag("network");
        assert_eq!(artifact.tags.len(), 2);
    }

    #[test]
    fn test_artifact_as_text() {
        let artifact = Artifact::new(
            "c1",
            ArtifactType::AnalysisOutput,
            "output.txt",
            b"Hello, world!".to_vec(),
        );

        assert_eq!(artifact.as_text().unwrap(), "Hello, world!");
    }

    #[test]
    fn test_artifact_type_display() {
        assert_eq!(ArtifactType::ScanResult.to_string(), "scan_result");
        assert_eq!(
            ArtifactType::Custom("mytype".to_string()).to_string(),
            "custom:mytype"
        );
    }

    #[test]
    fn test_artifact_hash_deterministic() {
        let a1 = Artifact::new("c1", ArtifactType::ScanResult, "a", b"same data".to_vec());
        let a2 = Artifact::new("c2", ArtifactType::LogFile, "b", b"same data".to_vec());
        assert_eq!(a1.hash, a2.hash);
    }
}
