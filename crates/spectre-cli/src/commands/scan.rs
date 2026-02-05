//! Network scanning command (ProRT-IP integration)
//!
//! This module provides the CLI interface for network reconnaissance using ProRT-IP.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, ValueEnum};
use tracing::{debug, info, instrument};

use crate::output::{OutputFormat, OutputWriter};

/// Scan type selection
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum ScanType {
    /// TCP SYN scan (stealth scan, requires root)
    Syn,
    /// TCP Connect scan (full connection)
    #[default]
    Connect,
    /// UDP scan
    Udp,
    /// ACK scan (firewall detection)
    Ack,
    /// FIN scan (stealth)
    Fin,
    /// Xmas scan (stealth)
    Xmas,
    /// Null scan (stealth)
    Null,
    /// Window scan
    Window,
}

impl std::fmt::Display for ScanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syn => write!(f, "SYN"),
            Self::Connect => write!(f, "Connect"),
            Self::Udp => write!(f, "UDP"),
            Self::Ack => write!(f, "ACK"),
            Self::Fin => write!(f, "FIN"),
            Self::Xmas => write!(f, "Xmas"),
            Self::Null => write!(f, "Null"),
            Self::Window => write!(f, "Window"),
        }
    }
}

/// Timing template for scan speed/stealth tradeoff
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum TimingTemplate {
    /// Paranoid (T0) - Very slow, IDS evasion
    #[value(alias = "0")]
    Paranoid,
    /// Sneaky (T1) - Slow, IDS evasion
    #[value(alias = "1")]
    Sneaky,
    /// Polite (T2) - Less bandwidth usage
    #[value(alias = "2")]
    Polite,
    /// Normal (T3) - Default timing
    #[value(alias = "3")]
    #[default]
    Normal,
    /// Aggressive (T4) - Fast scan
    #[value(alias = "4")]
    Aggressive,
    /// Insane (T5) - Very fast, may miss ports
    #[value(alias = "5")]
    Insane,
}

/// Arguments for the scan command
#[derive(Args, Debug)]
#[command(
    about = "Network scanning and reconnaissance",
    long_about = "Perform network scanning using ProRT-IP engine.\n\n\
                  Supports multiple scan types including SYN, Connect, UDP, and more.\n\
                  Results can be output in table or JSON format."
)]
pub struct ScanArgs {
    /// Target hosts (IP, CIDR, hostname, or range)
    #[arg(required = true, value_name = "TARGET")]
    pub targets: Vec<String>,

    /// TCP SYN scan (stealth, requires root)
    #[arg(short = 'S', long = "syn", group = "scan_type_flag")]
    pub syn_scan: bool,

    /// TCP Connect scan (full connection)
    #[arg(short = 'T', long = "connect", group = "scan_type_flag")]
    pub connect_scan: bool,

    /// UDP scan
    #[arg(short = 'U', long = "udp", group = "scan_type_flag")]
    pub udp_scan: bool,

    /// ACK scan (firewall detection)
    #[arg(short = 'A', long = "ack", group = "scan_type_flag")]
    pub ack_scan: bool,

    /// FIN scan (stealth)
    #[arg(short = 'F', long = "fin", group = "scan_type_flag")]
    pub fin_scan: bool,

    /// Xmas scan (stealth)
    #[arg(short = 'X', long = "xmas", group = "scan_type_flag")]
    pub xmas_scan: bool,

    /// Null scan (stealth)
    #[arg(short = 'N', long = "null", group = "scan_type_flag")]
    pub null_scan: bool,

    /// Scan type (alternative to flags)
    #[arg(long, value_enum, group = "scan_type_flag")]
    pub scan_type: Option<ScanType>,

    /// Port specification (e.g., "22,80,443" or "1-1000" or "common")
    #[arg(short = 'p', long, value_name = "PORTS")]
    pub ports: Option<String>,

    /// Service/version detection
    #[arg(long = "service-detection")]
    pub service_detection: bool,

    /// OS detection
    #[arg(short = 'O', long = "os-detection")]
    pub os_detection: bool,

    /// Timing template (0-5, paranoid to insane)
    #[arg(short = 't', long, value_enum, default_value = "normal")]
    pub timing: TimingTemplate,

    /// Maximum packets per second
    #[arg(long, value_name = "RATE")]
    pub rate: Option<u64>,

    /// Output format
    #[arg(short = 'o', long, value_enum, default_value = "table")]
    pub output: OutputFormat,

    /// Output file path
    #[arg(long, value_name = "FILE")]
    pub output_file: Option<PathBuf>,

    /// Include closed ports in output
    #[arg(long)]
    pub show_closed: bool,

    /// Disable DNS resolution
    #[arg(short = 'n', long)]
    pub no_dns: bool,

    /// Network interface to use
    #[arg(short = 'i', long, value_name = "IFACE")]
    pub interface: Option<String>,

    /// Custom Lua script to run
    #[arg(long, value_name = "SCRIPT")]
    pub script: Option<PathBuf>,
}

impl ScanArgs {
    /// Determine the effective scan type from flags
    pub const fn effective_scan_type(&self) -> ScanType {
        if self.syn_scan {
            ScanType::Syn
        } else if self.connect_scan {
            ScanType::Connect
        } else if self.udp_scan {
            ScanType::Udp
        } else if self.ack_scan {
            ScanType::Ack
        } else if self.fin_scan {
            ScanType::Fin
        } else if self.xmas_scan {
            ScanType::Xmas
        } else if self.null_scan {
            ScanType::Null
        } else if let Some(scan_type) = self.scan_type {
            scan_type
        } else {
            ScanType::Connect // Default
        }
    }
}

/// Execute the scan command
#[instrument(skip(args, config_path), fields(targets = ?args.targets, scan_type = %args.effective_scan_type()))]
pub async fn execute(args: ScanArgs, config_path: Option<PathBuf>) -> Result<()> {
    info!("Starting network scan");
    debug!(?config_path, "Configuration path");

    // Load configuration
    let config = spectre_core::config::load_config(config_path.as_deref())
        .context("Failed to load configuration")?;

    // Parse targets
    let targets = spectre_core::scan::parse_targets(&args.targets)
        .context("Failed to parse target specifications")?;

    debug!(target_count = targets.len(), "Parsed targets");

    // Parse ports
    let ports = spectre_core::scan::parse_ports(args.ports.as_deref())
        .context("Failed to parse port specification")?;

    debug!(port_count = ports.len(), "Parsed ports");

    // Build scan configuration
    let scan_config = spectre_core::scan::ScanConfig {
        scan_type: map_scan_type(args.effective_scan_type()),
        targets,
        ports,
        service_detection: args.service_detection,
        os_detection: args.os_detection,
        timing: map_timing(args.timing),
        rate_limit: args.rate,
        resolve_dns: !args.no_dns,
        interface: args.interface,
        script: args.script,
        ..Default::default()
    };

    // Create scanner
    let scanner = spectre_core::scan::create_scanner(&config)?;

    // Execute scan with progress indication
    let total_targets = scan_config.targets.len() as u64;
    let progress = indicatif::ProgressBar::new(total_targets);
    progress.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} hosts ({eta})")?
            .progress_chars("#>-"),
    );

    let progress_clone = progress.clone();
    let results = scanner
        .scan(
            scan_config,
            Some(Box::new(move |current, _total| {
                progress_clone.set_position(current as u64);
            })),
        )
        .await
        .context("Scan execution failed")?;

    progress.finish_with_message("Scan complete");

    // Filter results if needed
    let results: Vec<_> = if args.show_closed {
        results
    } else {
        results
            .into_iter()
            .filter(|r| !r.ports.is_empty())
            .collect()
    };

    // Create output writer
    let mut writer = OutputWriter::new(args.output, args.output_file.as_deref())?;

    // Write results
    writer.write_scan_results(&results)?;

    info!(hosts_scanned = results.len(), "Scan completed successfully");

    Ok(())
}

/// Map CLI scan type to core scan type
const fn map_scan_type(scan_type: ScanType) -> spectre_core::scan::ScanType {
    match scan_type {
        ScanType::Syn => spectre_core::scan::ScanType::Syn,
        ScanType::Connect => spectre_core::scan::ScanType::Connect,
        ScanType::Udp => spectre_core::scan::ScanType::Udp,
        ScanType::Ack => spectre_core::scan::ScanType::Ack,
        ScanType::Fin => spectre_core::scan::ScanType::Fin,
        ScanType::Xmas => spectre_core::scan::ScanType::Xmas,
        ScanType::Null => spectre_core::scan::ScanType::Null,
        ScanType::Window => spectre_core::scan::ScanType::Window,
    }
}

/// Map CLI timing template to core timing
const fn map_timing(timing: TimingTemplate) -> spectre_core::scan::TimingTemplate {
    match timing {
        TimingTemplate::Paranoid => spectre_core::scan::TimingTemplate::Paranoid,
        TimingTemplate::Sneaky => spectre_core::scan::TimingTemplate::Sneaky,
        TimingTemplate::Polite => spectre_core::scan::TimingTemplate::Polite,
        TimingTemplate::Normal => spectre_core::scan::TimingTemplate::Normal,
        TimingTemplate::Aggressive => spectre_core::scan::TimingTemplate::Aggressive,
        TimingTemplate::Insane => spectre_core::scan::TimingTemplate::Insane,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_type_from_flags() {
        let args = ScanArgs {
            targets: vec!["127.0.0.1".to_string()],
            syn_scan: true,
            connect_scan: false,
            udp_scan: false,
            ack_scan: false,
            fin_scan: false,
            xmas_scan: false,
            null_scan: false,
            scan_type: None,
            ports: None,
            service_detection: false,
            os_detection: false,
            timing: TimingTemplate::Normal,
            rate: None,
            output: OutputFormat::Table,
            output_file: None,
            show_closed: false,
            no_dns: false,
            interface: None,
            script: None,
        };

        assert!(matches!(args.effective_scan_type(), ScanType::Syn));
    }

    #[test]
    fn test_default_scan_type() {
        let args = ScanArgs {
            targets: vec!["127.0.0.1".to_string()],
            syn_scan: false,
            connect_scan: false,
            udp_scan: false,
            ack_scan: false,
            fin_scan: false,
            xmas_scan: false,
            null_scan: false,
            scan_type: None,
            ports: None,
            service_detection: false,
            os_detection: false,
            timing: TimingTemplate::Normal,
            rate: None,
            output: OutputFormat::Table,
            output_file: None,
            show_closed: false,
            no_dns: false,
            interface: None,
            script: None,
        };

        assert!(matches!(args.effective_scan_type(), ScanType::Connect));
    }

    #[test]
    fn test_scan_type_display() {
        assert_eq!(ScanType::Syn.to_string(), "SYN");
        assert_eq!(ScanType::Connect.to_string(), "Connect");
        assert_eq!(ScanType::Udp.to_string(), "UDP");
    }
}
