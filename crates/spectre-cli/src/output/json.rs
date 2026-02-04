//! JSON output formatting utilities
//!
//! This module provides utilities for creating formatted JSON output.

use serde::Serialize;
use serde_json::Value;

/// Create a standard success response wrapper
#[allow(dead_code, clippy::disallowed_methods)]
pub fn success_response<T: Serialize>(data: T) -> Value {
    serde_json::json!({
        "success": true,
        "data": data
    })
}

/// Create a standard error response wrapper
#[allow(dead_code, clippy::disallowed_methods)]
pub fn error_response(message: &str, code: Option<&str>) -> Value {
    serde_json::json!({
        "success": false,
        "error": {
            "message": message,
            "code": code
        }
    })
}

/// Create a paginated response wrapper
#[allow(dead_code, clippy::disallowed_methods)]
pub fn paginated_response<T: Serialize>(
    data: T,
    page: usize,
    per_page: usize,
    total: usize,
) -> Value {
    serde_json::json!({
        "success": true,
        "data": data,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total": total,
            "total_pages": total.div_ceil(per_page)
        }
    })
}

/// Pretty print JSON value
#[allow(dead_code)]
pub fn pretty_print(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
}

/// Compact print JSON value
#[allow(dead_code)]
pub fn compact_print(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let response = success_response(vec![1, 2, 3]);
        assert_eq!(response["success"].as_bool(), Some(true));
        assert!(response["data"].is_array());
    }

    #[test]
    fn test_error_response() {
        let response = error_response("Something went wrong", Some("ERR_001"));
        assert_eq!(response["success"].as_bool(), Some(false));
        assert_eq!(response["error"]["message"], "Something went wrong");
        assert_eq!(response["error"]["code"], "ERR_001");
    }

    #[test]
    fn test_paginated_response() {
        let response = paginated_response(vec!["a", "b"], 1, 10, 25);
        assert_eq!(response["success"].as_bool(), Some(true));
        assert_eq!(response["pagination"]["page"], 1);
        assert_eq!(response["pagination"]["total_pages"], 3);
    }

    #[test]
    fn test_pretty_print() {
        #[allow(clippy::disallowed_methods)]
        let value = serde_json::json!({"key": "value"});
        let output = pretty_print(&value);
        assert!(output.contains("key"));
        assert!(output.contains('\n')); // Pretty print has newlines
    }

    #[test]
    fn test_compact_print() {
        #[allow(clippy::disallowed_methods)]
        let value = serde_json::json!({"key": "value"});
        let output = compact_print(&value);
        assert!(output.contains("key"));
        assert!(!output.contains('\n')); // Compact has no newlines
    }
}
