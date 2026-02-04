//! CLI command definitions and execution logic
//!
//! This module defines the command hierarchy for SPECTRE using clap derive macros.

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_complete::Shell;

pub mod chef;
pub mod completions;
pub mod config;
pub mod identity;
pub mod peer;
pub mod receive;
pub mod scan;
pub mod send;
pub mod status;

/// SPECTRE - Security Platform for Encrypted Comms, Testing, Enumeration, Recon
///
/// A unified offensive security toolkit that orchestrates network reconnaissance,
/// data analysis, and secure communications.
#[derive(Parser, Debug)]
#[command(
    name = "spectre",
    author,
    version,
    about = "SPECTRE - Unified Offensive Security Toolkit",
    long_about = "Security Platform for Encrypted Comms, Testing, Enumeration, Recon\n\n\
                  SPECTRE orchestrates three powerful components:\n\
                  • ProRT-IP: Network reconnaissance (10M+ pps scanning)\n\
                  • CyberChef-MCP: Data analysis (463 operations via MCP)\n\
                  • WRAITH-Protocol: Secure communications (10+ Gbps E2EE)",
    propagate_version = true,
    arg_required_else_help = true,
    styles = get_styles()
)]
pub struct Cli {
    /// Increase output verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Write logs to file
    #[arg(long, global = true, value_name = "FILE")]
    pub log_file: Option<PathBuf>,

    /// Configuration file path (overrides discovery)
    #[arg(
        short,
        long,
        global = true,
        value_name = "FILE",
        env = "SPECTRE_CONFIG"
    )]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

/// Top-level command categories
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Network scanning and reconnaissance (ProRT-IP)
    #[command(visible_alias = "s")]
    Scan(scan::ScanArgs),

    /// Data transformation and analysis (CyberChef-MCP)
    #[command(visible_alias = "c")]
    Chef(chef::ChefArgs),

    /// Send secure message or file (WRAITH-Protocol)
    Send(send::SendArgs),

    /// Receive secure message or file (WRAITH-Protocol)
    #[command(visible_alias = "recv")]
    Receive(receive::ReceiveArgs),

    /// Manage cryptographic identities
    #[command(visible_alias = "id")]
    Identity(identity::IdentityArgs),

    /// Manage trusted peers
    Peer(peer::PeerArgs),

    /// Show component status and health
    Status(status::StatusArgs),

    /// Configuration management
    #[command(visible_alias = "cfg")]
    Config(config::ConfigArgs),

    /// Generate shell completions
    Completions(completions::CompletionsArgs),
}

/// Get custom styles for colored output
fn get_styles() -> clap::builder::Styles {
    use clap::builder::styling::*;

    Styles::styled()
        .header(AnsiColor::Green.on_default().bold())
        .usage(AnsiColor::Green.on_default().bold())
        .literal(AnsiColor::Cyan.on_default().bold())
        .placeholder(AnsiColor::Yellow.on_default())
        .error(AnsiColor::Red.on_default().bold())
        .valid(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Red.on_default())
}

/// Execute the parsed CLI command
pub async fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Scan(args) => scan::execute(args, cli.config).await,
        Commands::Chef(args) => chef::execute(args, cli.config).await,
        Commands::Send(args) => send::execute(args, cli.config).await,
        Commands::Receive(args) => receive::execute(args, cli.config).await,
        Commands::Identity(args) => identity::execute(args, cli.config).await,
        Commands::Peer(args) => peer::execute(args, cli.config).await,
        Commands::Status(args) => status::execute(args, cli.config).await,
        Commands::Config(args) => config::execute(args, cli.config).await,
        Commands::Completions(args) => completions::execute(args),
    }
}

/// Generate shell completions for the given shell
pub fn generate_completions(shell: Shell) -> String {
    use clap::CommandFactory;
    use clap_complete::generate;
    use std::io::BufWriter;

    let mut cmd = Cli::command();
    let mut buf = BufWriter::new(Vec::new());
    generate(shell, &mut cmd, "spectre", &mut buf);

    String::from_utf8(buf.into_inner().unwrap_or_default()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_debug() {
        // Ensure CLI struct can be parsed
        Cli::command().debug_assert();
    }

    #[test]
    fn test_help_display() {
        let result = Cli::try_parse_from(["spectre", "--help"]);
        assert!(result.is_err()); // --help causes early exit
        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn test_version_display() {
        let result = Cli::try_parse_from(["spectre", "--version"]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
    }

    #[test]
    fn test_verbose_count() {
        let cli = Cli::try_parse_from(["spectre", "-vvv", "status"]).unwrap();
        assert_eq!(cli.verbose, 3);
    }

    #[test]
    fn test_quiet_flag() {
        let cli = Cli::try_parse_from(["spectre", "-q", "status"]).unwrap();
        assert!(cli.quiet);
    }

    #[test]
    fn test_config_override() {
        let cli = Cli::try_parse_from(["spectre", "-c", "/custom/config.toml", "status"]).unwrap();
        assert_eq!(cli.config, Some(PathBuf::from("/custom/config.toml")));
    }

    #[test]
    fn test_scan_command_parsing() {
        let cli = Cli::try_parse_from(["spectre", "scan", "-S", "-p", "22,80,443", "192.168.1.1"])
            .unwrap();

        if let Commands::Scan(args) = cli.command {
            assert!(args.syn_scan);
            assert_eq!(args.ports, Some("22,80,443".to_string()));
            assert_eq!(args.targets, vec!["192.168.1.1"]);
        } else {
            panic!("Expected Scan command");
        }
    }

    #[test]
    fn test_chef_command_parsing() {
        let cli = Cli::try_parse_from([
            "spectre",
            "chef",
            "From_Base64",
            "--input",
            "SGVsbG8gV29ybGQ=",
        ])
        .unwrap();

        if let Commands::Chef(args) = cli.command {
            assert_eq!(args.operation, Some("From_Base64".to_string()));
            assert_eq!(args.input, Some("SGVsbG8gV29ybGQ=".to_string()));
        } else {
            panic!("Expected Chef command");
        }
    }

    #[test]
    fn test_completions_generation() {
        let bash_completions = generate_completions(Shell::Bash);
        assert!(bash_completions.contains("spectre"));
        assert!(bash_completions.contains("scan"));
        assert!(bash_completions.contains("chef"));
    }

    #[test]
    fn test_alias_resolution() {
        // Test 's' alias for scan
        let cli = Cli::try_parse_from(["spectre", "s", "127.0.0.1"]).unwrap();
        assert!(matches!(cli.command, Commands::Scan(_)));

        // Test 'c' alias for chef
        let cli = Cli::try_parse_from(["spectre", "c", "To_Hex", "-i", "test"]).unwrap();
        assert!(matches!(cli.command, Commands::Chef(_)));
    }
}
