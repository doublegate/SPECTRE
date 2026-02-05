//! Campaign management CLI commands

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};
use colored::Colorize;
use spectre_core::campaign::{Campaign, CampaignPhase, CampaignStore};

/// Campaign management arguments
#[derive(Args, Debug)]
pub struct CampaignArgs {
    #[command(subcommand)]
    pub command: CampaignCommands,
}

/// Campaign subcommands
#[derive(Subcommand, Debug)]
pub enum CampaignCommands {
    /// Create a new campaign
    Create {
        /// Campaign name/codename
        name: String,
        /// Campaign description
        #[arg(short, long, default_value = "")]
        description: String,
    },

    /// Show campaign status
    Status {
        /// Campaign ID
        id: String,
    },

    /// List all campaigns
    List {
        /// Filter by phase
        #[arg(short, long)]
        phase: Option<String>,
    },

    /// Advance campaign to next phase
    Advance {
        /// Campaign ID
        id: String,
    },

    /// Export campaign
    Export {
        /// Campaign ID
        id: String,
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Export format (json, yaml)
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Import a campaign
    Import {
        /// Path to campaign file
        file: PathBuf,
    },

    /// Archive a completed campaign
    Archive {
        /// Campaign ID
        id: String,
    },
}

/// Execute campaign commands
pub async fn execute(args: CampaignArgs, config_path: Option<PathBuf>) -> Result<()> {
    let db_path = get_db_path(&config_path);
    let store = CampaignStore::open(&db_path)?;

    match args.command {
        CampaignCommands::Create { name, description } => {
            let campaign = Campaign::new(&name, &description);
            let id = campaign.id.clone();
            store.save_campaign(&campaign)?;

            println!("{} Campaign created", "OK".green().bold());
            println!("  ID:   {}", id);
            println!("  Name: {}", name);
            println!("  Phase: {}", campaign.phase);
        },

        CampaignCommands::Status { id } => {
            let campaign = store.load_campaign(&id)?;

            println!("{} {}", "Campaign:".cyan().bold(), campaign.name);
            println!("  ID:          {}", campaign.id);
            println!("  Phase:       {}", campaign.phase);
            println!("  Description: {}", campaign.description);
            println!(
                "  Created:     {}",
                campaign.created_at.format("%Y-%m-%d %H:%M")
            );

            if !campaign.objectives.is_empty() {
                println!("  Objectives:");
                for obj in &campaign.objectives {
                    println!("    - {}", obj);
                }
            }

            if !campaign.targets.is_empty() {
                println!("  Targets:");
                for target in &campaign.targets {
                    println!("    - {}", target);
                }
            }

            if !campaign.tags.is_empty() {
                println!("  Tags: {}", campaign.tags.join(", "));
            }
        },

        CampaignCommands::List { phase } => {
            let campaigns = store.list_campaigns()?;

            let filtered: Vec<&Campaign> = if let Some(ref phase_filter) = phase {
                campaigns
                    .iter()
                    .filter(|c| c.phase.to_string() == *phase_filter)
                    .collect()
            } else {
                campaigns.iter().collect()
            };

            if filtered.is_empty() {
                println!("{}", "No campaigns found".yellow());
                return Ok(());
            }

            println!(
                "{:<38} {:<25} {:<15} {:<20}",
                "ID".bold(),
                "Name".bold(),
                "Phase".bold(),
                "Updated".bold()
            );
            println!("{}", "-".repeat(98));

            for campaign in &filtered {
                println!(
                    "{:<38} {:<25} {:<15} {:<20}",
                    campaign.id,
                    campaign.name,
                    campaign.phase,
                    campaign.updated_at.format("%Y-%m-%d %H:%M"),
                );
            }

            println!("\n{} campaign(s)", filtered.len());
        },

        CampaignCommands::Advance { id } => {
            let mut campaign = store.load_campaign(&id)?;
            let old_phase = campaign.phase;
            campaign.advance_phase()?;
            store.save_campaign(&campaign)?;

            println!(
                "{} Campaign advanced: {} -> {}",
                "OK".green().bold(),
                old_phase,
                campaign.phase
            );
        },

        CampaignCommands::Export { id, output, format } => {
            let campaign = store.load_campaign(&id)?;

            let content = match format.as_str() {
                "yaml" | "yml" => campaign.to_yaml()?,
                _ => campaign.to_json()?,
            };

            if let Some(path) = output {
                std::fs::write(&path, &content)?;
                println!(
                    "{} Campaign exported to {}",
                    "OK".green().bold(),
                    path.display()
                );
            } else {
                println!("{}", content);
            }
        },

        CampaignCommands::Import { file } => {
            let content = std::fs::read_to_string(&file)?;

            let campaign = if file
                .extension()
                .is_some_and(|ext| ext == "yaml" || ext == "yml")
            {
                Campaign::from_yaml(&content)?
            } else {
                Campaign::from_json(&content)?
            };

            let name = campaign.name.clone();
            let id = campaign.id.clone();
            store.save_campaign(&campaign)?;

            println!("{} Campaign imported", "OK".green().bold());
            println!("  ID:   {}", id);
            println!("  Name: {}", name);
        },

        CampaignCommands::Archive { id } => {
            let mut campaign = store.load_campaign(&id)?;
            campaign.set_phase(CampaignPhase::Archived);
            store.save_campaign(&campaign)?;

            println!(
                "{} Campaign '{}' archived",
                "OK".green().bold(),
                campaign.name
            );
        },
    }

    Ok(())
}

/// Get the database path
fn get_db_path(config_path: &Option<PathBuf>) -> PathBuf {
    // Check if config specifies a data directory
    if let Some(ref config) = config_path {
        if let Some(parent) = config.parent() {
            return parent.join("campaigns.db");
        }
    }

    // Use default data directory
    directories::ProjectDirs::from("com", "spectre", "spectre")
        .map(|dirs: directories::ProjectDirs| dirs.data_dir().join("campaigns.db"))
        .unwrap_or_else(|| PathBuf::from("campaigns.db"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    // Re-import for parsing tests
    #[derive(Parser, Debug)]
    struct TestCli {
        #[command(subcommand)]
        command: CampaignCommands,
    }

    #[test]
    fn test_create_command() {
        let cli = TestCli::try_parse_from(["test", "create", "NIGHTFALL"]).unwrap();
        assert!(
            matches!(cli.command, CampaignCommands::Create { name, .. } if name == "NIGHTFALL")
        );
    }

    #[test]
    fn test_list_command() {
        let cli = TestCli::try_parse_from(["test", "list"]).unwrap();
        assert!(matches!(cli.command, CampaignCommands::List { .. }));
    }

    #[test]
    fn test_status_command() {
        let cli = TestCli::try_parse_from(["test", "status", "some-id"]).unwrap();
        assert!(matches!(cli.command, CampaignCommands::Status { id } if id == "some-id"));
    }
}
