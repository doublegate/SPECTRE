//! Output formatting for scan results
//!
//! Supports JSON, XML (nmap-compatible), and greppable output formats.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::finding::Host;

/// Output format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    /// JSON output
    Json,
    /// XML output (nmap-compatible)
    Xml,
    /// Greppable output
    Greppable,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Xml => write!(f, "xml"),
            Self::Greppable => write!(f, "greppable"),
        }
    }
}

/// Format hosts as JSON
pub fn format_json(hosts: &[Host], pretty: bool) -> crate::Result<String> {
    if pretty {
        serde_json::to_string_pretty(hosts).map_err(|e| {
            crate::SpectreError::Serialization(format!("JSON formatting error: {}", e))
        })
    } else {
        serde_json::to_string(hosts).map_err(|e| {
            crate::SpectreError::Serialization(format!("JSON formatting error: {}", e))
        })
    }
}

/// Format hosts as nmap-compatible XML
pub fn format_xml(hosts: &[Host]) -> crate::Result<String> {
    use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
    use quick_xml::Writer;
    use std::io::Cursor;

    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

    // XML declaration
    writer
        .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
        .map_err(|e| crate::SpectreError::Serialization(format!("XML write error: {}", e)))?;

    // Root element
    let mut root = BytesStart::new("spectrerun");
    root.push_attribute(("scanner", "spectre"));
    root.push_attribute(("version", env!("CARGO_PKG_VERSION")));
    writer
        .write_event(Event::Start(root))
        .map_err(|e| crate::SpectreError::Serialization(format!("XML write error: {}", e)))?;

    for host in hosts {
        // <host>
        let host_elem = BytesStart::new("host");
        writer
            .write_event(Event::Start(host_elem))
            .map_err(|e| crate::SpectreError::Serialization(format!("XML write error: {}", e)))?;

        // <address addr="..." addrtype="ipv4"/>
        let mut addr = BytesStart::new("address");
        addr.push_attribute(("addr", host.ip.as_str()));
        addr.push_attribute(("addrtype", "ipv4"));
        writer
            .write_event(Event::Empty(addr))
            .map_err(|e| crate::SpectreError::Serialization(format!("XML write error: {}", e)))?;

        // <hostnames>
        if let Some(ref hostname) = host.hostname {
            let hostnames_elem = BytesStart::new("hostnames");
            writer
                .write_event(Event::Start(hostnames_elem))
                .map_err(|e| {
                    crate::SpectreError::Serialization(format!("XML write error: {}", e))
                })?;

            let mut hn = BytesStart::new("hostname");
            hn.push_attribute(("name", hostname.as_str()));
            hn.push_attribute(("type", "PTR"));
            writer.write_event(Event::Empty(hn)).map_err(|e| {
                crate::SpectreError::Serialization(format!("XML write error: {}", e))
            })?;

            writer
                .write_event(Event::End(BytesEnd::new("hostnames")))
                .map_err(|e| {
                    crate::SpectreError::Serialization(format!("XML write error: {}", e))
                })?;
        }

        // <ports>
        let ports_elem = BytesStart::new("ports");
        writer
            .write_event(Event::Start(ports_elem))
            .map_err(|e| crate::SpectreError::Serialization(format!("XML write error: {}", e)))?;

        for service in &host.services {
            // <port protocol="tcp" portid="80">
            let mut port_elem = BytesStart::new("port");
            port_elem.push_attribute(("protocol", service.protocol.as_str()));
            port_elem.push_attribute(("portid", service.port.to_string().as_str()));
            writer.write_event(Event::Start(port_elem)).map_err(|e| {
                crate::SpectreError::Serialization(format!("XML write error: {}", e))
            })?;

            // <state state="open"/>
            let mut state_elem = BytesStart::new("state");
            state_elem.push_attribute(("state", service.state.to_lowercase().as_str()));
            writer.write_event(Event::Empty(state_elem)).map_err(|e| {
                crate::SpectreError::Serialization(format!("XML write error: {}", e))
            })?;

            // <service name="..."/>
            if let Some(ref name) = service.service_name {
                let mut svc = BytesStart::new("service");
                svc.push_attribute(("name", name.as_str()));
                if let Some(ref ver) = service.version {
                    svc.push_attribute(("version", ver.as_str()));
                }
                writer.write_event(Event::Empty(svc)).map_err(|e| {
                    crate::SpectreError::Serialization(format!("XML write error: {}", e))
                })?;
            }

            // </port>
            writer
                .write_event(Event::End(BytesEnd::new("port")))
                .map_err(|e| {
                    crate::SpectreError::Serialization(format!("XML write error: {}", e))
                })?;
        }

        // </ports>
        writer
            .write_event(Event::End(BytesEnd::new("ports")))
            .map_err(|e| crate::SpectreError::Serialization(format!("XML write error: {}", e)))?;

        // <os>
        if let Some(ref os) = host.os {
            let os_elem = BytesStart::new("os");
            writer.write_event(Event::Start(os_elem)).map_err(|e| {
                crate::SpectreError::Serialization(format!("XML write error: {}", e))
            })?;

            let mut osmatch = BytesStart::new("osmatch");
            osmatch.push_attribute(("name", os.as_str()));
            writer.write_event(Event::Empty(osmatch)).map_err(|e| {
                crate::SpectreError::Serialization(format!("XML write error: {}", e))
            })?;

            writer
                .write_event(Event::End(BytesEnd::new("os")))
                .map_err(|e| {
                    crate::SpectreError::Serialization(format!("XML write error: {}", e))
                })?;
        }

        // </host>
        writer
            .write_event(Event::End(BytesEnd::new("host")))
            .map_err(|e| crate::SpectreError::Serialization(format!("XML write error: {}", e)))?;
    }

    // </spectrerun>
    writer
        .write_event(Event::End(BytesEnd::new("spectrerun")))
        .map_err(|e| crate::SpectreError::Serialization(format!("XML write error: {}", e)))?;

    let result = writer.into_inner().into_inner();
    String::from_utf8(result)
        .map_err(|e| crate::SpectreError::Serialization(format!("UTF-8 conversion error: {}", e)))
}

/// Format hosts as greppable output
///
/// Format: `Host: <ip> (<hostname>) Ports: <port>/<state>/<protocol>//<service>/<version>/`
pub fn format_greppable(hosts: &[Host]) -> String {
    let mut lines = Vec::new();

    for host in hosts {
        let hostname = host.hostname.as_deref().unwrap_or("");

        let ports: Vec<String> = host
            .services
            .iter()
            .map(|s| {
                let service_name = s.service_name.as_deref().unwrap_or("");
                let version = s.version.as_deref().unwrap_or("");
                format!(
                    "{}/{}/{}//{}/{}/",
                    s.port,
                    s.state.to_lowercase(),
                    s.protocol,
                    service_name,
                    version
                )
            })
            .collect();

        lines.push(format!(
            "Host: {} ({}) Ports: {}",
            host.ip,
            hostname,
            ports.join(", ")
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::finding::Service;

    fn make_host() -> Host {
        Host {
            ip: "10.0.0.1".to_string(),
            hostname: Some("web.example.com".to_string()),
            os: Some("Linux".to_string()),
            os_version: Some("5.15".to_string()),
            services: vec![
                Service {
                    port: 22,
                    protocol: "tcp".to_string(),
                    state: "Open".to_string(),
                    service_name: Some("ssh".to_string()),
                    version: Some("OpenSSH 8.9".to_string()),
                    banner: None,
                },
                Service {
                    port: 80,
                    protocol: "tcp".to_string(),
                    state: "Open".to_string(),
                    service_name: Some("http".to_string()),
                    version: Some("nginx 1.24".to_string()),
                    banner: None,
                },
            ],
            scan_time_ms: 150,
        }
    }

    #[test]
    fn test_format_json() {
        let hosts = vec![make_host()];
        let json = format_json(&hosts, true).unwrap();
        assert!(json.contains("10.0.0.1"));
        assert!(json.contains("ssh"));
    }

    #[test]
    fn test_format_json_compact() {
        let hosts = vec![make_host()];
        let json = format_json(&hosts, false).unwrap();
        assert!(!json.contains('\n'));
    }

    #[test]
    fn test_format_xml() {
        let hosts = vec![make_host()];
        let xml = format_xml(&hosts).unwrap();
        assert!(xml.contains("<spectrerun"));
        assert!(xml.contains("10.0.0.1"));
        assert!(xml.contains("portid=\"22\""));
        assert!(xml.contains("state=\"open\""));
        assert!(xml.contains("<hostname"));
        assert!(xml.contains("<osmatch"));
    }

    #[test]
    fn test_format_greppable() {
        let hosts = vec![make_host()];
        let grep = format_greppable(&hosts);
        assert!(grep.contains("Host: 10.0.0.1"));
        assert!(grep.contains("web.example.com"));
        assert!(grep.contains("22/open/tcp//ssh/"));
    }

    #[test]
    fn test_output_format_display() {
        assert_eq!(OutputFormat::Json.to_string(), "json");
        assert_eq!(OutputFormat::Xml.to_string(), "xml");
        assert_eq!(OutputFormat::Greppable.to_string(), "greppable");
    }

    #[test]
    fn test_format_xml_no_hostname() {
        let mut host = make_host();
        host.hostname = None;
        let hosts = vec![host];
        let xml = format_xml(&hosts).unwrap();
        assert!(!xml.contains("<hostnames"));
    }

    #[test]
    fn test_format_greppable_no_hostname() {
        let mut host = make_host();
        host.hostname = None;
        let hosts = vec![host];
        let grep = format_greppable(&hosts);
        assert!(grep.contains("Host: 10.0.0.1 ()"));
    }
}
