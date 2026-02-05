//! Target file parsing
//!
//! Parse target files containing one target per line, supporting:
//! - IP addresses
//! - CIDR ranges
//! - Hostnames
//! - Comments (lines starting with #)
//! - Empty lines (ignored)

use std::path::Path;

use super::EnhancedTarget;
use crate::error::TargetError;

/// Parse a target file into a list of enhanced targets
///
/// # File Format
///
/// One target per line. Supports:
/// - IP addresses: `192.168.1.1`
/// - CIDR ranges: `10.0.0.0/24` (expanded to individual IPs)
/// - Hostnames: `example.com`
/// - Comments: `# This is a comment`
/// - Inline comments: `192.168.1.1 # web server`
/// - Empty lines (ignored)
///
/// # Arguments
///
/// * `path` - Path to the target file
///
/// # Returns
///
/// Vector of parsed enhanced targets.
pub fn parse_target_file(path: &Path) -> crate::Result<Vec<EnhancedTarget>> {
    let content = std::fs::read_to_string(path).map_err(|e| {
        crate::SpectreError::Target(TargetError::FileParseError(format!(
            "Failed to read target file '{}': {}",
            path.display(),
            e
        )))
    })?;

    parse_target_string(&content)
}

/// Parse target specifications from a string (one per line)
pub fn parse_target_string(content: &str) -> crate::Result<Vec<EnhancedTarget>> {
    let mut targets = Vec::new();

    for (line_no, line) in content.lines().enumerate() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Handle inline comments
        let spec = if let Some(idx) = line.find('#') {
            line[..idx].trim()
        } else {
            line
        };

        if spec.is_empty() {
            continue;
        }

        // Try parsing as CIDR
        if spec.contains('/') {
            match super::expand_cidr(spec) {
                Ok(cidr_targets) => {
                    targets.extend(cidr_targets);
                    continue;
                },
                Err(_) => {
                    return Err(crate::SpectreError::Target(TargetError::FileParseError(
                        format!("Invalid CIDR on line {}: {}", line_no + 1, spec),
                    )));
                },
            }
        }

        // Try parsing as IP address
        if let Ok(ip) = spec.parse::<std::net::IpAddr>() {
            targets.push(EnhancedTarget::from_ip(ip));
            continue;
        }

        // Treat as hostname if it looks valid
        if is_valid_target_hostname(spec) {
            targets.push(EnhancedTarget::from_hostname(spec));
            continue;
        }

        return Err(crate::SpectreError::Target(TargetError::FileParseError(
            format!("Invalid target on line {}: {}", line_no + 1, spec),
        )));
    }

    Ok(targets)
}

/// Basic hostname validation for target files
fn is_valid_target_hostname(s: &str) -> bool {
    if s.is_empty() || s.len() > 253 {
        return false;
    }

    if s.starts_with('-') || s.starts_with('.') || s.ends_with('-') || s.ends_with('.') {
        return false;
    }

    for label in s.split('.') {
        if label.is_empty() || label.len() > 63 {
            return false;
        }
        if !label.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return false;
        }
    }

    // Must have at least one dot (or be a simple hostname)
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_target_string_ips() {
        let content = "192.168.1.1\n192.168.1.2\n192.168.1.3\n";
        let targets = parse_target_string(content).unwrap();
        assert_eq!(targets.len(), 3);
    }

    #[test]
    fn test_parse_target_string_with_comments() {
        let content = "# Network A targets\n192.168.1.1\n# Network B\n10.0.0.1\n";
        let targets = parse_target_string(content).unwrap();
        assert_eq!(targets.len(), 2);
    }

    #[test]
    fn test_parse_target_string_inline_comments() {
        let content = "192.168.1.1 # web server\n192.168.1.2 # db server\n";
        let targets = parse_target_string(content).unwrap();
        assert_eq!(targets.len(), 2);
    }

    #[test]
    fn test_parse_target_string_empty_lines() {
        let content = "\n192.168.1.1\n\n\n192.168.1.2\n\n";
        let targets = parse_target_string(content).unwrap();
        assert_eq!(targets.len(), 2);
    }

    #[test]
    fn test_parse_target_string_cidr() {
        let content = "192.168.1.0/30\n";
        let targets = parse_target_string(content).unwrap();
        assert_eq!(targets.len(), 4);
    }

    #[test]
    fn test_parse_target_string_hostnames() {
        let content = "example.com\ntest.local\n";
        let targets = parse_target_string(content).unwrap();
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[0].address.to_string(), "example.com");
    }

    #[test]
    fn test_parse_target_string_mixed() {
        let content = "# Targets\n192.168.1.1\n10.0.0.0/30\nexample.com\n";
        let targets = parse_target_string(content).unwrap();
        // 1 IP + 4 from CIDR + 1 hostname = 6
        assert_eq!(targets.len(), 6);
    }

    #[test]
    fn test_parse_target_string_invalid() {
        let content = "invalid..hostname\n";
        let result = parse_target_string(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_target_file_not_found() {
        let result = parse_target_file(Path::new("/nonexistent/file.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_target_string_whitespace() {
        let content = "  192.168.1.1  \n  10.0.0.1  \n";
        let targets = parse_target_string(content).unwrap();
        assert_eq!(targets.len(), 2);
    }

    #[test]
    fn test_hostname_validation() {
        assert!(is_valid_target_hostname("example.com"));
        assert!(is_valid_target_hostname("sub.example.com"));
        assert!(is_valid_target_hostname("my-host"));
        assert!(!is_valid_target_hostname(""));
        assert!(!is_valid_target_hostname("-bad.com"));
        assert!(!is_valid_target_hostname("bad..com"));
    }
}
