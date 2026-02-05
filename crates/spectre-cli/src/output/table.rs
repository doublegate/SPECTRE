//! Table output formatting utilities
//!
//! This module provides utilities for creating formatted tables.

use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};

/// Create a styled table with default settings
#[allow(dead_code)]
pub fn create_table() -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);
    table
}

/// Create a header cell with styling
#[allow(dead_code)]
pub fn header_cell(text: &str) -> Cell {
    Cell::new(text).fg(Color::Cyan)
}

/// Create a cell with success styling (green)
#[allow(dead_code)]
pub fn success_cell(text: &str) -> Cell {
    Cell::new(text).fg(Color::Green)
}

/// Create a cell with error styling (red)
#[allow(dead_code)]
pub fn error_cell(text: &str) -> Cell {
    Cell::new(text).fg(Color::Red)
}

/// Create a cell with warning styling (yellow)
#[allow(dead_code)]
pub fn warning_cell(text: &str) -> Cell {
    Cell::new(text).fg(Color::Yellow)
}

/// Create a cell with info styling (blue)
#[allow(dead_code)]
pub fn info_cell(text: &str) -> Cell {
    Cell::new(text).fg(Color::Blue)
}

/// Create a cell with default styling
#[allow(dead_code)]
pub fn plain_cell(text: &str) -> Cell {
    Cell::new(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table() {
        let table = create_table();
        // Just ensure it can be created without panicking
        let _ = format!("{table}");
    }

    #[test]
    fn test_header_cell() {
        let cell = header_cell("Test");
        // Cells don't implement Display directly, just verify construction
        let _ = cell;
    }
}
