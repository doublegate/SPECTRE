use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::{command, State};

use crate::state::AppState;

/// Campaign summary for listing - mirrors spectre-core Campaign struct
#[derive(Debug, Serialize)]
pub struct CampaignSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub phase: String,
    pub created: String,
    pub target_count: usize,
    pub objectives: Vec<String>,
}

/// Campaign creation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub description: Option<String>,
    pub objectives: Option<Vec<String>>,
    pub targets: Option<Vec<String>>,
}

/// Export request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub id: String,
    pub path: String,
}

/// Import request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRequest {
    pub path: String,
}

/// Create a new campaign.
#[command]
pub async fn create_campaign(
    state: State<'_, Arc<AppState>>,
    request: CreateCampaignRequest,
) -> Result<String, String> {
    tracing::info!(name = %request.name, "Creating campaign");

    let store_arc = state.get_campaign_store().await?;
    let store_guard = store_arc.lock().await;
    let store = store_guard
        .as_ref()
        .ok_or_else(|| "Campaign store not initialized".to_string())?;

    // Create the campaign
    let mut campaign = spectre_core::campaign::Campaign::new(
        &request.name,
        request.description.as_deref().unwrap_or(""),
    );

    // Add objectives if provided
    if let Some(objectives) = request.objectives {
        for objective in objectives {
            campaign.add_objective(&objective);
        }
    }

    // Add targets if provided
    if let Some(targets) = request.targets {
        for target in targets {
            campaign.add_target(&target);
        }
    }

    // Save to database
    store
        .save_campaign(&campaign)
        .map_err(|e| format!("Failed to save campaign: {}", e))?;

    tracing::info!(id = %campaign.id, "Campaign created successfully");
    Ok(campaign.id)
}

/// List all campaigns.
#[command]
pub async fn list_campaigns(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<CampaignSummary>, String> {
    tracing::debug!("Listing campaigns");

    let store_arc = state.get_campaign_store().await?;
    let store_guard = store_arc.lock().await;
    let store = store_guard
        .as_ref()
        .ok_or_else(|| "Campaign store not initialized".to_string())?;

    let campaigns = store
        .list_campaigns()
        .map_err(|e| format!("Failed to list campaigns: {}", e))?;

    let summaries: Vec<CampaignSummary> = campaigns
        .into_iter()
        .map(|c| {
            let is_complete = c.is_complete();
            let is_active = c.is_active();
            let phase_str = c.phase.to_string();
            let created_str = c.created_at.to_rfc3339();
            let target_count = c.targets.len();

            CampaignSummary {
                id: c.id,
                name: c.name,
                description: c.description,
                status: if is_complete {
                    "complete".to_string()
                } else if is_active {
                    "active".to_string()
                } else {
                    "planning".to_string()
                },
                phase: phase_str,
                created: created_str,
                target_count,
                objectives: c.objectives,
            }
        })
        .collect();

    Ok(summaries)
}

/// Get campaign details by ID.
#[command]
pub async fn get_campaign(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> Result<Option<CampaignSummary>, String> {
    tracing::debug!(id = %id, "Getting campaign");

    let store_arc = state.get_campaign_store().await?;
    let store_guard = store_arc.lock().await;
    let store = store_guard
        .as_ref()
        .ok_or_else(|| "Campaign store not initialized".to_string())?;

    match store.load_campaign(&id) {
        Ok(c) => {
            let is_complete = c.is_complete();
            let is_active = c.is_active();
            let phase_str = c.phase.to_string();
            let created_str = c.created_at.to_rfc3339();
            let target_count = c.targets.len();

            Ok(Some(CampaignSummary {
                id: c.id,
                name: c.name,
                description: c.description,
                status: if is_complete {
                    "complete".to_string()
                } else if is_active {
                    "active".to_string()
                } else {
                    "planning".to_string()
                },
                phase: phase_str,
                created: created_str,
                target_count,
                objectives: c.objectives,
            }))
        },
        Err(spectre_core::SpectreError::Campaign(
            spectre_core::error::CampaignError::NotFound(_),
        )) => Ok(None),
        Err(e) => Err(format!("Failed to get campaign: {}", e)),
    }
}

/// Advance campaign to next phase.
#[command]
pub async fn advance_campaign(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> Result<String, String> {
    tracing::info!(id = %id, "Advancing campaign");

    let store_arc = state.get_campaign_store().await?;
    let store_guard = store_arc.lock().await;
    let store = store_guard
        .as_ref()
        .ok_or_else(|| "Campaign store not initialized".to_string())?;

    // Load the campaign
    let mut campaign = store
        .load_campaign(&id)
        .map_err(|e| format!("Failed to load campaign: {}", e))?;

    // Advance to next phase
    campaign
        .advance_phase()
        .map_err(|e| format!("Failed to advance phase: {}", e))?;

    // Save back to database
    store
        .save_campaign(&campaign)
        .map_err(|e| format!("Failed to save campaign: {}", e))?;

    tracing::info!(phase = %campaign.phase, "Campaign advanced successfully");
    Ok(campaign.phase.to_string())
}

/// Export campaign to JSON file.
#[command]
pub async fn export_campaign(
    state: State<'_, Arc<AppState>>,
    request: ExportRequest,
) -> Result<(), String> {
    tracing::info!(id = %request.id, path = %request.path, "Exporting campaign");

    let store_arc = state.get_campaign_store().await?;
    let store_guard = store_arc.lock().await;
    let store = store_guard
        .as_ref()
        .ok_or_else(|| "Campaign store not initialized".to_string())?;

    // Load the campaign
    let campaign = store
        .load_campaign(&request.id)
        .map_err(|e| format!("Failed to load campaign: {}", e))?;

    // Convert to JSON
    let json = campaign
        .to_json()
        .map_err(|e| format!("Failed to serialize campaign: {}", e))?;

    // Write to file
    std::fs::write(&request.path, json)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    tracing::info!("Campaign exported successfully");
    Ok(())
}

/// Import campaign from JSON file.
#[command]
pub async fn import_campaign(
    state: State<'_, Arc<AppState>>,
    request: ImportRequest,
) -> Result<String, String> {
    tracing::info!(path = %request.path, "Importing campaign");

    let store_arc = state.get_campaign_store().await?;
    let store_guard = store_arc.lock().await;
    let store = store_guard
        .as_ref()
        .ok_or_else(|| "Campaign store not initialized".to_string())?;

    // Read the file
    let json = std::fs::read_to_string(&request.path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse the campaign
    let campaign = spectre_core::campaign::Campaign::from_json(&json)
        .map_err(|e| format!("Failed to parse campaign: {}", e))?;

    // Save to database
    let id = campaign.id.clone();
    store
        .save_campaign(&campaign)
        .map_err(|e| format!("Failed to save campaign: {}", e))?;

    tracing::info!(id = %id, "Campaign imported successfully");
    Ok(id)
}

/// Archive campaign (mark as archived).
#[command]
pub async fn archive_campaign(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> Result<(), String> {
    tracing::info!(id = %id, "Archiving campaign");

    let store_arc = state.get_campaign_store().await?;
    let store_guard = store_arc.lock().await;
    let store = store_guard
        .as_ref()
        .ok_or_else(|| "Campaign store not initialized".to_string())?;

    // Load the campaign
    let mut campaign = store
        .load_campaign(&id)
        .map_err(|e| format!("Failed to load campaign: {}", e))?;

    // Set to archived phase
    campaign.set_phase(spectre_core::campaign::CampaignPhase::Archived);

    // Save back to database
    store
        .save_campaign(&campaign)
        .map_err(|e| format!("Failed to save campaign: {}", e))?;

    tracing::info!("Campaign archived successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectre_core::campaign::{Campaign, CampaignStore};
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_create_campaign_success() {
        let store = CampaignStore::open_in_memory().unwrap();

        let mut campaign = Campaign::new("TestOp", "Test campaign");
        campaign.add_objective("Find vulns");
        campaign.add_target("10.0.0.0/24");

        store.save_campaign(&campaign).unwrap();

        let loaded = store.load_campaign(&campaign.id).unwrap();
        assert_eq!(loaded.name, "TestOp");
        assert_eq!(loaded.objectives.len(), 1);
        assert_eq!(loaded.targets.len(), 1);
    }

    #[tokio::test]
    async fn test_list_campaigns() {
        let store = CampaignStore::open_in_memory().unwrap();

        let campaign = Campaign::new("TestOp1", "First");
        store.save_campaign(&campaign).unwrap();

        let campaigns = store.list_campaigns().unwrap();
        assert_eq!(campaigns.len(), 1);
        assert_eq!(campaigns[0].name, "TestOp1");
    }

    #[tokio::test]
    async fn test_get_campaign_not_found() {
        let store = CampaignStore::open_in_memory().unwrap();
        let result = store.load_campaign("nonexistent");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_advance_campaign() {
        let store = CampaignStore::open_in_memory().unwrap();

        let mut campaign = Campaign::new("TestOp", "Test");
        store.save_campaign(&campaign).unwrap();

        campaign.advance_phase().unwrap();
        store.save_campaign(&campaign).unwrap();

        let loaded = store.load_campaign(&campaign.id).unwrap();
        assert_eq!(loaded.phase.to_string(), "recon");
    }

    #[tokio::test]
    async fn test_export_import_campaign() {
        let temp_dir = TempDir::new().unwrap();
        let export_path = temp_dir.path().join("campaign.json");

        let mut campaign = Campaign::new("ExportTest", "Export test");
        campaign.add_objective("Test objective");
        campaign.add_target("192.168.1.0/24");

        // Export
        let json = campaign.to_json().unwrap();
        std::fs::write(&export_path, json).unwrap();

        // Verify file exists
        assert!(export_path.exists());

        // Import
        let json = std::fs::read_to_string(&export_path).unwrap();
        let imported = Campaign::from_json(&json).unwrap();
        assert_eq!(imported.id, campaign.id);
        assert_eq!(imported.name, campaign.name);
    }

    #[tokio::test]
    async fn test_archive_campaign() {
        let store = CampaignStore::open_in_memory().unwrap();

        let mut campaign = Campaign::new("ArchiveTest", "Archive test");
        store.save_campaign(&campaign).unwrap();

        campaign.set_phase(spectre_core::campaign::CampaignPhase::Archived);
        store.save_campaign(&campaign).unwrap();

        let loaded = store.load_campaign(&campaign.id).unwrap();
        assert_eq!(loaded.phase.to_string(), "archived");
    }
}
