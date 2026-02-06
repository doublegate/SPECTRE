use serde::{Deserialize, Serialize};
use tauri::command;

/// Target parse result.
#[derive(Debug, Serialize)]
pub struct ParsedTarget {
    pub original: String,
    pub expanded: Vec<String>,
    pub count: usize,
}

/// Target validation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetInput {
    pub targets: Vec<String>,
}

/// Parse and validate targets. Stub - will be wired in Sprint 5.3.
#[command]
pub async fn parse_targets(input: TargetInput) -> Result<Vec<ParsedTarget>, String> {
    tracing::info!(count = input.targets.len(), "Parse targets (stub)");
    Ok(input
        .targets
        .into_iter()
        .map(|t| ParsedTarget {
            original: t.clone(),
            expanded: vec![t.clone()],
            count: 1,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_targets_stub() {
        let input = TargetInput {
            targets: vec!["10.0.0.1".to_string(), "192.168.1.0/24".to_string()],
        };
        let result = parse_targets(input).await;
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].original, "10.0.0.1");
    }
}
