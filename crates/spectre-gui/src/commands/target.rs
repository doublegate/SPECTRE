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

/// Parse and validate targets using spectre-core's target parsing.
///
/// Supports:
/// - Individual IPs (192.168.1.1)
/// - CIDR ranges (192.168.1.0/24)
/// - Hostnames (example.com)
#[command]
pub async fn parse_targets(input: TargetInput) -> Result<Vec<ParsedTarget>, String> {
    tracing::info!(count = input.targets.len(), "Parsing targets");

    let mut results = Vec::new();

    for target in input.targets {
        let trimmed = target.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Try to parse as CIDR first
        if trimmed.contains('/') {
            match spectre_core::target::expand_cidr(trimmed) {
                Ok(expanded) => {
                    let expanded_ips: Vec<String> = expanded
                        .iter()
                        .filter_map(|t| t.ip().map(|ip| ip.to_string()))
                        .collect();

                    results.push(ParsedTarget {
                        original: trimmed.to_string(),
                        count: expanded_ips.len(),
                        expanded: expanded_ips,
                    });
                },
                Err(e) => {
                    return Err(format!("Invalid CIDR '{}': {}", trimmed, e));
                },
            }
        }
        // Try to parse as individual IP
        else if trimmed.parse::<std::net::IpAddr>().is_ok() {
            results.push(ParsedTarget {
                original: trimmed.to_string(),
                expanded: vec![trimmed.to_string()],
                count: 1,
            });
        }
        // Treat as hostname
        else {
            // For hostnames, we'll add them as-is (resolution happens during scan)
            results.push(ParsedTarget {
                original: trimmed.to_string(),
                expanded: vec![trimmed.to_string()],
                count: 1,
            });
        }
    }

    tracing::info!(
        parsed_count = results.len(),
        total_targets = results.iter().map(|r| r.count).sum::<usize>(),
        "Target parsing complete"
    );

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_targets_single_ip() {
        let input = TargetInput {
            targets: vec!["10.0.0.1".to_string()],
        };
        let result = parse_targets(input).await;
        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse targets");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].original, "10.0.0.1");
        assert_eq!(parsed[0].count, 1);
    }

    #[tokio::test]
    async fn test_parse_targets_cidr() {
        let input = TargetInput {
            targets: vec!["192.168.1.0/30".to_string()],
        };
        let result = parse_targets(input).await;
        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse targets");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].original, "192.168.1.0/30");
        assert_eq!(parsed[0].count, 4);
        assert_eq!(parsed[0].expanded.len(), 4);
    }

    #[tokio::test]
    async fn test_parse_targets_hostname() {
        let input = TargetInput {
            targets: vec!["example.com".to_string()],
        };
        let result = parse_targets(input).await;
        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse targets");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].original, "example.com");
        assert_eq!(parsed[0].count, 1);
    }

    #[tokio::test]
    async fn test_parse_targets_invalid_cidr() {
        let input = TargetInput {
            targets: vec!["192.168.1.0/99".to_string()],
        };
        let result = parse_targets(input).await;
        assert!(result.is_err());
        assert!(result
            .expect_err("Should have failed")
            .contains("Invalid CIDR"));
    }

    #[tokio::test]
    async fn test_parse_targets_mixed() {
        let input = TargetInput {
            targets: vec![
                "10.0.0.1".to_string(),
                "192.168.1.0/30".to_string(),
                "example.com".to_string(),
            ],
        };
        let result = parse_targets(input).await;
        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse targets");
        assert_eq!(parsed.len(), 3);

        // First should be single IP
        assert_eq!(parsed[0].count, 1);

        // Second should be CIDR with 4 IPs
        assert_eq!(parsed[1].count, 4);

        // Third should be hostname
        assert_eq!(parsed[2].count, 1);
        assert_eq!(parsed[2].original, "example.com");
    }

    #[tokio::test]
    async fn test_parse_targets_empty_strings() {
        let input = TargetInput {
            targets: vec!["10.0.0.1".to_string(), "".to_string(), "  ".to_string()],
        };
        let result = parse_targets(input).await;
        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse targets");
        assert_eq!(parsed.len(), 1); // Empty strings should be skipped
    }
}
