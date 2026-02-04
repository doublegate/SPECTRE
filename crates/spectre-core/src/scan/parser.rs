//! Target and port parsing utilities

use std::net::IpAddr;

use crate::error::ScanError;

use super::types::Target;

/// Well-known ports commonly scanned
pub const COMMON_PORTS: &[u16] = &[
    21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 445, 993, 995, 1723, 3306, 3389, 5432,
    5900, 8080, 8443,
];

/// Top 100 most common ports
pub const TOP_100_PORTS: &[u16] = &[
    7, 9, 13, 21, 22, 23, 25, 26, 37, 53, 79, 80, 81, 88, 106, 110, 111, 113, 119, 135, 139, 143,
    144, 179, 199, 389, 427, 443, 444, 445, 465, 513, 514, 515, 543, 544, 548, 554, 587, 631, 646,
    873, 990, 993, 995, 1025, 1026, 1027, 1028, 1029, 1110, 1433, 1720, 1723, 1755, 1900, 2000,
    2001, 2049, 2121, 2717, 3000, 3128, 3306, 3389, 3986, 4899, 5000, 5009, 5051, 5060, 5101, 5190,
    5357, 5432, 5631, 5666, 5800, 5900, 6000, 6001, 6646, 7070, 8000, 8008, 8009, 8080, 8081, 8443,
    8888, 9100, 9999, 10000, 32768, 49152, 49153, 49154, 49155, 49156, 49157,
];

/// Parse target specifications into Target objects
///
/// Supports:
/// - Single IP addresses: `192.168.1.1`
/// - CIDR notation: `192.168.1.0/24`
/// - IP ranges: `192.168.1.1-10` (TODO)
/// - Hostnames: `example.com`
///
/// # Arguments
///
/// * `specs` - Slice of target specification strings
///
/// # Returns
///
/// Vector of parsed Target objects.
pub fn parse_targets(specs: &[String]) -> crate::Result<Vec<Target>> {
    let mut targets = Vec::new();

    for spec in specs {
        let spec = spec.trim();

        if spec.is_empty() {
            continue;
        }

        // Try parsing as CIDR first (includes simple IPs like 192.168.1.1/32)
        if let Ok(network) = spec.parse::<ipnetwork::IpNetwork>() {
            // If it's a single IP (prefix length = 32 for v4, 128 for v6), store as IP
            if (network.is_ipv4() && network.prefix() == 32)
                || (network.is_ipv6() && network.prefix() == 128)
            {
                targets.push(Target::Ip(network.ip()));
            } else {
                // It's a network range, expand it
                for ip in network.iter() {
                    targets.push(Target::Ip(ip));
                }
            }
            continue;
        }

        // Try parsing as a plain IP address
        if let Ok(ip) = spec.parse::<IpAddr>() {
            targets.push(Target::Ip(ip));
            continue;
        }

        // Check for IP range format (192.168.1.1-10)
        if spec.contains('-') && !spec.contains('/') {
            if let Some(range_targets) = parse_ip_range(spec) {
                targets.extend(range_targets);
                continue;
            }
        }

        // Treat as hostname
        if is_valid_hostname(spec) {
            targets.push(Target::Hostname(spec.to_string()));
        } else {
            return Err(crate::SpectreError::Scan(ScanError::InvalidTarget(
                format!("Invalid target specification: {}", spec),
            )));
        }
    }

    Ok(targets)
}

/// Parse IP range format like "192.168.1.1-10"
fn parse_ip_range(spec: &str) -> Option<Vec<Target>> {
    let parts: Vec<&str> = spec.rsplitn(2, '-').collect();
    if parts.len() != 2 {
        return None;
    }

    let end: u8 = parts[0].parse().ok()?;

    // Get the base IP
    let base_parts: Vec<&str> = parts[1].rsplitn(2, '.').collect();
    if base_parts.len() != 2 {
        return None;
    }

    let start: u8 = base_parts[0].parse().ok()?;
    let base = base_parts[1];

    if end < start {
        return None;
    }

    let mut targets = Vec::new();
    for i in start..=end {
        let ip_str = format!("{}.{}", base, i);
        if let Ok(ip) = ip_str.parse::<IpAddr>() {
            targets.push(Target::Ip(ip));
        }
    }

    Some(targets)
}

/// Check if a string is a valid hostname
fn is_valid_hostname(s: &str) -> bool {
    // Basic hostname validation
    if s.is_empty() || s.len() > 253 {
        return false;
    }

    // Must not start or end with hyphen or dot
    if s.starts_with('-') || s.starts_with('.') || s.ends_with('-') || s.ends_with('.') {
        return false;
    }

    // Check each label
    for label in s.split('.') {
        if label.is_empty() || label.len() > 63 {
            return false;
        }
        if !label.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return false;
        }
    }

    true
}

/// Parse port specifications into port numbers
///
/// Supports:
/// - Single ports: `80`
/// - Port ranges: `1-1000`
/// - Port lists: `22,80,443`
/// - Named sets: `common`, `all`, `top100`
///
/// # Arguments
///
/// * `spec` - Optional port specification string
///
/// # Returns
///
/// Vector of port numbers to scan.
pub fn parse_ports(spec: Option<&str>) -> crate::Result<Vec<u16>> {
    let spec = spec.unwrap_or("common");

    match spec.to_lowercase().as_str() {
        "common" => return Ok(COMMON_PORTS.to_vec()),
        "top100" => return Ok(TOP_100_PORTS.to_vec()),
        "all" => return Ok((1..=65535).collect()),
        _ => {},
    }

    let mut ports = Vec::new();

    for part in spec.split(',') {
        let part = part.trim();

        if part.contains('-') {
            // Port range
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                return Err(crate::SpectreError::Scan(ScanError::InvalidPort(format!(
                    "Invalid port range: {}",
                    part
                ))));
            }

            let start: u16 = range_parts[0].parse().map_err(|_| {
                crate::SpectreError::Scan(ScanError::InvalidPort(format!(
                    "Invalid port number: {}",
                    range_parts[0]
                )))
            })?;

            let end: u16 = range_parts[1].parse().map_err(|_| {
                crate::SpectreError::Scan(ScanError::InvalidPort(format!(
                    "Invalid port number: {}",
                    range_parts[1]
                )))
            })?;

            if end < start {
                return Err(crate::SpectreError::Scan(ScanError::InvalidPort(format!(
                    "Invalid port range: {} > {}",
                    start, end
                ))));
            }

            for port in start..=end {
                if !ports.contains(&port) {
                    ports.push(port);
                }
            }
        } else {
            // Single port
            let port: u16 = part.parse().map_err(|_| {
                crate::SpectreError::Scan(ScanError::InvalidPort(format!(
                    "Invalid port number: {}",
                    part
                )))
            })?;

            if !ports.contains(&port) {
                ports.push(port);
            }
        }
    }

    if ports.is_empty() {
        return Err(crate::SpectreError::Scan(ScanError::InvalidPort(
            "No ports specified".to_string(),
        )));
    }

    Ok(ports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_ip() {
        let targets = parse_targets(&["192.168.1.1".to_string()]).unwrap();
        assert_eq!(targets.len(), 1);
        assert!(matches!(&targets[0], Target::Ip(ip) if ip.to_string() == "192.168.1.1"));
    }

    #[test]
    fn test_parse_cidr() {
        let targets = parse_targets(&["192.168.1.0/30".to_string()]).unwrap();
        assert_eq!(targets.len(), 4); // /30 has 4 IPs
    }

    #[test]
    fn test_parse_ip_range() {
        let targets = parse_targets(&["192.168.1.1-5".to_string()]).unwrap();
        assert_eq!(targets.len(), 5);
    }

    #[test]
    fn test_parse_hostname() {
        let targets = parse_targets(&["example.com".to_string()]).unwrap();
        assert_eq!(targets.len(), 1);
        assert!(matches!(&targets[0], Target::Hostname(h) if h == "example.com"));
    }

    #[test]
    fn test_parse_invalid_target() {
        let result = parse_targets(&["invalid..hostname".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_single_port() {
        let ports = parse_ports(Some("80")).unwrap();
        assert_eq!(ports, vec![80]);
    }

    #[test]
    fn test_parse_port_range() {
        let ports = parse_ports(Some("1-5")).unwrap();
        assert_eq!(ports, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_port_list() {
        let ports = parse_ports(Some("22,80,443")).unwrap();
        assert_eq!(ports, vec![22, 80, 443]);
    }

    #[test]
    fn test_parse_common_ports() {
        let ports = parse_ports(Some("common")).unwrap();
        assert_eq!(ports, COMMON_PORTS.to_vec());
    }

    #[test]
    fn test_parse_all_ports() {
        let ports = parse_ports(Some("all")).unwrap();
        assert_eq!(ports.len(), 65535);
    }

    #[test]
    fn test_parse_invalid_port() {
        let result = parse_ports(Some("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_port_range() {
        let result = parse_ports(Some("100-50"));
        assert!(result.is_err());
    }

    #[test]
    fn test_hostname_validation() {
        assert!(is_valid_hostname("example.com"));
        assert!(is_valid_hostname("sub.example.com"));
        assert!(is_valid_hostname("example-site.com"));
        assert!(!is_valid_hostname(""));
        assert!(!is_valid_hostname("-example.com"));
        assert!(!is_valid_hostname("example..com"));
    }
}
