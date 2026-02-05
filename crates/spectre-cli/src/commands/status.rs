//! Status command for component health checking
//!
//! This module provides the CLI interface for checking component status and health.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;
use tracing::{debug, info, instrument};

/// Arguments for the status command
#[derive(Args, Debug)]
#[command(
    about = "Show component status and health",
    long_about = "Display the status and health of all SPECTRE components.\n\n\
                  Shows connection status for ProRT-IP, CyberChef-MCP, and WRAITH-Protocol."
)]
pub struct StatusArgs {
    /// Check specific component only
    #[arg(value_enum)]
    pub component: Option<Component>,

    /// Output in JSON format
    #[arg(long)]
    pub json: bool,

    /// Show detailed information
    #[arg(short, long)]
    pub detailed: bool,
}

/// Component to check status for
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Component {
    /// ProRT-IP network scanner
    Prtip,
    /// CyberChef-MCP data transformer
    Chef,
    /// WRAITH-Protocol secure communications
    Wraith,
    /// Configuration status
    Config,
}

/// Status result for a component
#[derive(Debug, serde::Serialize)]
pub struct ComponentStatus {
    pub name: String,
    pub healthy: bool,
    pub status: String,
    pub version: Option<String>,
    pub details: Option<serde_json::Value>,
}

/// Execute the status command
#[instrument(skip(args, config_path))]
pub async fn execute(args: StatusArgs, config_path: Option<PathBuf>) -> Result<()> {
    info!("Checking component status");
    debug!(?config_path, "Configuration path");

    // Load configuration
    let config = spectre_core::config::load_config(config_path.as_deref())
        .context("Failed to load configuration")?;

    let mut statuses = Vec::new();

    // Check specific component or all
    match args.component {
        Some(Component::Prtip) => {
            statuses.push(check_prtip_status(&config, args.detailed).await);
        },
        Some(Component::Chef) => {
            statuses.push(check_chef_status(&config, args.detailed).await);
        },
        Some(Component::Wraith) => {
            statuses.push(check_wraith_status(&config, args.detailed).await);
        },
        Some(Component::Config) => {
            statuses.push(check_config_status(&config, args.detailed).await);
        },
        None => {
            // Check all components
            statuses.push(check_config_status(&config, args.detailed).await);
            statuses.push(check_prtip_status(&config, args.detailed).await);
            statuses.push(check_chef_status(&config, args.detailed).await);
            statuses.push(check_wraith_status(&config, args.detailed).await);
        },
    }

    if args.json {
        let json = serde_json::to_string_pretty(&statuses)?;
        println!("{json}");
    } else {
        print_status_header();

        for status in &statuses {
            print_status(status, args.detailed);
        }

        // Print summary
        let healthy_count = statuses.iter().filter(|s| s.healthy).count();
        let total_count = statuses.len();

        println!();
        if healthy_count == total_count {
            println!(
                "{}",
                format!("All {total_count} components healthy").green()
            );
        } else {
            println!(
                "{}",
                format!("{healthy_count}/{total_count} components healthy").yellow()
            );
        }
    }

    Ok(())
}

/// Print status header
fn print_status_header() {
    println!();
    println!("SPECTRE Component Status");
    println!("========================");
    println!();
}

/// Print individual component status
fn print_status(status: &ComponentStatus, detailed: bool) {
    let indicator = if status.healthy {
        "[OK]".green()
    } else {
        "[!!]".red()
    };

    let name = format!("{:<12}", status.name);
    let status_text = &status.status;

    println!("{indicator} {name} - {status_text}");

    if let Some(version) = &status.version {
        println!("             Version: {version}");
    }

    if detailed {
        if let Some(details) = &status.details {
            if let Some(obj) = details.as_object() {
                for (key, value) in obj {
                    println!("             {key}: {value}");
                }
            }
        }
    }
}

/// Check ProRT-IP status
#[allow(clippy::disallowed_methods)]
async fn check_prtip_status(
    config: &spectre_core::config::Config,
    detailed: bool,
) -> ComponentStatus {
    match spectre_core::scan::check_availability(config).await {
        Ok(info) => ComponentStatus {
            name: "ProRT-IP".to_string(),
            healthy: true,
            status: "Available".to_string(),
            version: Some(info.version),
            details: if detailed {
                Some(serde_json::json!({
                    "scan_types": info.supported_scan_types,
                    "max_rate": info.max_packet_rate,
                    "requires_root": info.requires_root,
                }))
            } else {
                None
            },
        },
        Err(e) => ComponentStatus {
            name: "ProRT-IP".to_string(),
            healthy: false,
            status: format!("Unavailable: {e}"),
            version: None,
            details: None,
        },
    }
}

/// Check CyberChef-MCP status
#[allow(clippy::disallowed_methods)]
async fn check_chef_status(
    config: &spectre_core::config::Config,
    detailed: bool,
) -> ComponentStatus {
    match spectre_core::chef::create_chef_client(config).await {
        Ok(client) => match client.health_check().await {
            Ok(health) => ComponentStatus {
                name: "CyberChef".to_string(),
                healthy: health.healthy,
                status: if health.healthy {
                    "Connected".to_string()
                } else {
                    "Unhealthy".to_string()
                },
                version: Some(health.mcp_version),
                details: if detailed {
                    Some(serde_json::json!({
                        "container": health.container_status,
                        "operations": health.operation_count,
                        "mcp_endpoint": config.chef.mcp_endpoint(),
                    }))
                } else {
                    None
                },
            },
            Err(e) => ComponentStatus {
                name: "CyberChef".to_string(),
                healthy: false,
                status: format!("Health check failed: {e}"),
                version: None,
                details: None,
            },
        },
        Err(e) => ComponentStatus {
            name: "CyberChef".to_string(),
            healthy: false,
            status: format!("Not connected: {e}"),
            version: None,
            details: if detailed {
                Some(serde_json::json!({
                    "hint": "Run 'spectre chef setup --start' to start container"
                }))
            } else {
                None
            },
        },
    }
}

/// Check WRAITH-Protocol status
#[allow(clippy::disallowed_methods)]
async fn check_wraith_status(
    config: &spectre_core::config::Config,
    detailed: bool,
) -> ComponentStatus {
    // Check for identity
    let identity = spectre_core::comms::load_identity(config);

    match identity {
        Ok(id) => {
            // Try to create client and check connectivity
            match spectre_core::comms::create_client(config, &id).await {
                Ok(client) => match client.check_relay_connectivity().await {
                    Ok(relay_status) => ComponentStatus {
                        name: "WRAITH".to_string(),
                        healthy: true,
                        status: "Ready".to_string(),
                        version: Some(spectre_core::comms::WRAITH_VERSION.to_string()),
                        details: if detailed {
                            Some(serde_json::json!({
                                "identity": id.id(),
                                "relay_connected": relay_status.connected,
                                "relay_latency_ms": relay_status.latency_ms,
                            }))
                        } else {
                            None
                        },
                    },
                    Err(_) => ComponentStatus {
                        name: "WRAITH".to_string(),
                        healthy: true,
                        status: "Identity ready (relay offline)".to_string(),
                        version: Some(spectre_core::comms::WRAITH_VERSION.to_string()),
                        details: if detailed {
                            Some(serde_json::json!({
                                "identity": id.id(),
                                "relay_connected": false,
                            }))
                        } else {
                            None
                        },
                    },
                },
                Err(e) => ComponentStatus {
                    name: "WRAITH".to_string(),
                    healthy: false,
                    status: format!("Client error: {e}"),
                    version: Some(spectre_core::comms::WRAITH_VERSION.to_string()),
                    details: None,
                },
            }
        },
        Err(_) => ComponentStatus {
            name: "WRAITH".to_string(),
            healthy: false,
            status: "No identity".to_string(),
            version: Some(spectre_core::comms::WRAITH_VERSION.to_string()),
            details: if detailed {
                Some(serde_json::json!({
                    "hint": "Run 'spectre identity init' to create identity"
                }))
            } else {
                None
            },
        },
    }
}

/// Check configuration status
#[allow(clippy::disallowed_methods)]
async fn check_config_status(
    config: &spectre_core::config::Config,
    detailed: bool,
) -> ComponentStatus {
    let config_path = config.loaded_from().map(|p| p.display().to_string());

    ComponentStatus {
        name: "Config".to_string(),
        healthy: true,
        status: "Loaded".to_string(),
        version: None,
        details: if detailed {
            Some(serde_json::json!({
                "path": config_path,
                "scan_interface": config.scan.interface(),
                "chef_endpoint": config.chef.mcp_endpoint(),
                "wraith_relay": config.comms.relay_url(),
            }))
        } else {
            None
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::disallowed_methods)]
    fn test_component_status_serialization() {
        let status = ComponentStatus {
            name: "Test".to_string(),
            healthy: true,
            status: "OK".to_string(),
            version: Some("1.0.0".to_string()),
            details: Some(serde_json::json!({"key": "value"})),
        };

        let json = serde_json::to_string(&status).expect("serialization should succeed");
        assert!(json.contains("Test"));
        assert!(json.contains("1.0.0"));
    }
}
