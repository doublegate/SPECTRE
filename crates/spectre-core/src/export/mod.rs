//! Export formats module
//!
//! Provides CSV export, custom export templates, export scheduling,
//! and incremental export capabilities.

mod csv;
mod incremental;
mod scheduler;
mod template;

pub use self::csv::CsvExporter;
pub use incremental::{ExportState, IncrementalExporter};
pub use scheduler::ExportScheduler;
pub use template::{ExportTemplate, ExportTemplateEngine};

use serde::{Deserialize, Serialize};

/// Export format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    /// CSV format
    Csv,
    /// JSON format
    Json,
    /// Custom template format
    Template,
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Csv => write!(f, "csv"),
            Self::Json => write!(f, "json"),
            Self::Template => write!(f, "template"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format_display() {
        assert_eq!(ExportFormat::Csv.to_string(), "csv");
        assert_eq!(ExportFormat::Json.to_string(), "json");
        assert_eq!(ExportFormat::Template.to_string(), "template");
    }
}
