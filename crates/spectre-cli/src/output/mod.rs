//! Output formatting module
//!
//! This module provides utilities for formatting command output in various formats.

use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};

pub mod json;
pub mod table;

/// Output format selection
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum, PartialEq, Eq)]
pub enum OutputFormat {
    /// Table format (default)
    #[default]
    Table,
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// YAML format
    Yaml,
}

/// Output writer that handles different formats and destinations
pub struct OutputWriter {
    format: OutputFormat,
    output: Box<dyn Write>,
}

impl OutputWriter {
    /// Create a new output writer
    pub fn new(format: OutputFormat, file_path: Option<&Path>) -> Result<Self> {
        let output: Box<dyn Write> = if let Some(path) = file_path {
            Box::new(
                std::fs::File::create(path)
                    .context(format!("Failed to create output file: {:?}", path))?,
            )
        } else {
            Box::new(std::io::stdout())
        };

        Ok(Self { format, output })
    }

    /// Write scan results
    pub fn write_scan_results(&mut self, results: &[spectre_core::scan::ScanResult]) -> Result<()> {
        match self.format {
            OutputFormat::Table => self.write_scan_table(results),
            OutputFormat::Json => self.write_scan_json(results),
            OutputFormat::Csv => self.write_scan_csv(results),
            OutputFormat::Yaml => self.write_scan_yaml(results),
        }
    }

    /// Write operations list
    pub fn write_operations(
        &mut self,
        operations: &[spectre_core::chef::OperationInfo],
    ) -> Result<()> {
        match self.format {
            OutputFormat::Table => self.write_operations_table(operations),
            OutputFormat::Json => self.write_operations_json(operations),
            OutputFormat::Csv => self.write_operations_csv(operations),
            OutputFormat::Yaml => self.write_operations_yaml(operations),
        }
    }

    // Table output methods
    fn write_scan_table(&mut self, results: &[spectre_core::scan::ScanResult]) -> Result<()> {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Host").fg(Color::Cyan),
                Cell::new("Port").fg(Color::Cyan),
                Cell::new("State").fg(Color::Cyan),
                Cell::new("Service").fg(Color::Cyan),
                Cell::new("Version").fg(Color::Cyan),
            ]);

        for result in results {
            for port in &result.ports {
                let state_color = match port.state {
                    spectre_core::scan::PortState::Open => Color::Green,
                    spectre_core::scan::PortState::Closed => Color::Red,
                    spectre_core::scan::PortState::Filtered => Color::Yellow,
                    _ => Color::White,
                };

                table.add_row(vec![
                    Cell::new(&result.host),
                    Cell::new(format!("{}/{}", port.port, port.protocol)),
                    Cell::new(format!("{:?}", port.state)).fg(state_color),
                    Cell::new(port.service.as_deref().unwrap_or("-")),
                    Cell::new(port.version.as_deref().unwrap_or("-")),
                ]);
            }
        }

        writeln!(self.output, "{}", table)?;
        Ok(())
    }

    fn write_operations_table(
        &mut self,
        operations: &[spectre_core::chef::OperationInfo],
    ) -> Result<()> {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Operation").fg(Color::Cyan),
                Cell::new("Category").fg(Color::Cyan),
                Cell::new("Description").fg(Color::Cyan),
            ]);

        for op in operations {
            table.add_row(vec![
                Cell::new(&op.name),
                Cell::new(&op.category),
                Cell::new(&op.description),
            ]);
        }

        writeln!(self.output, "{}", table)?;
        Ok(())
    }

    // JSON output methods
    fn write_scan_json(&mut self, results: &[spectre_core::scan::ScanResult]) -> Result<()> {
        let json = serde_json::to_string_pretty(results)?;
        writeln!(self.output, "{}", json)?;
        Ok(())
    }

    fn write_operations_json(
        &mut self,
        operations: &[spectre_core::chef::OperationInfo],
    ) -> Result<()> {
        let json = serde_json::to_string_pretty(operations)?;
        writeln!(self.output, "{}", json)?;
        Ok(())
    }

    // CSV output methods
    fn write_scan_csv(&mut self, results: &[spectre_core::scan::ScanResult]) -> Result<()> {
        writeln!(self.output, "host,port,protocol,state,service,version")?;

        for result in results {
            for port in &result.ports {
                writeln!(
                    self.output,
                    "{},{},{},{:?},{},{}",
                    result.host,
                    port.port,
                    port.protocol,
                    port.state,
                    port.service.as_deref().unwrap_or(""),
                    port.version.as_deref().unwrap_or("")
                )?;
            }
        }

        Ok(())
    }

    fn write_operations_csv(
        &mut self,
        operations: &[spectre_core::chef::OperationInfo],
    ) -> Result<()> {
        writeln!(self.output, "name,category,description")?;

        for op in operations {
            writeln!(
                self.output,
                "{},{},\"{}\"",
                op.name,
                op.category,
                op.description.replace('"', "\"\"")
            )?;
        }

        Ok(())
    }

    // YAML output methods
    fn write_scan_yaml(&mut self, results: &[spectre_core::scan::ScanResult]) -> Result<()> {
        let yaml = serde_yaml::to_string(results)?;
        writeln!(self.output, "{}", yaml)?;
        Ok(())
    }

    fn write_operations_yaml(
        &mut self,
        operations: &[spectre_core::chef::OperationInfo],
    ) -> Result<()> {
        let yaml = serde_yaml::to_string(operations)?;
        writeln!(self.output, "{}", yaml)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_default() {
        assert_eq!(OutputFormat::default(), OutputFormat::Table);
    }

    #[test]
    fn test_output_writer_creation() {
        let writer = OutputWriter::new(OutputFormat::Json, None);
        assert!(writer.is_ok());
    }
}
