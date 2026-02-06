use serde::{Deserialize, Serialize};
use tauri::command;

/// Campaign summary for listing.
#[derive(Debug, Serialize)]
pub struct CampaignSummary {
    pub name: String,
    pub status: String,
    pub phase: String,
    pub created: String,
    pub target_count: usize,
}

/// Campaign creation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub description: Option<String>,
    pub targets: Option<Vec<String>>,
}

/// Create a new campaign. Stub - will be wired in Sprint 5.3.
#[command]
pub async fn create_campaign(request: CreateCampaignRequest) -> Result<String, String> {
    tracing::info!(name = %request.name, "Create campaign (stub)");
    Ok(format!("Campaign '{}' created", request.name))
}

/// List all campaigns. Stub.
#[command]
pub async fn list_campaigns() -> Result<Vec<CampaignSummary>, String> {
    Ok(vec![])
}

/// Get campaign details. Stub.
#[command]
pub async fn get_campaign(name: String) -> Result<Option<CampaignSummary>, String> {
    tracing::info!(name = %name, "Get campaign (stub)");
    Ok(None)
}

/// Advance campaign to next phase. Stub.
#[command]
pub async fn advance_campaign(name: String) -> Result<String, String> {
    Ok(format!("Campaign '{}' advanced", name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_campaign_stub() {
        let request = CreateCampaignRequest {
            name: "TestOp".to_string(),
            description: Some("Test campaign".to_string()),
            targets: None,
        };
        let result = create_campaign(request).await;
        assert!(result.is_ok());
        assert!(result.expect("Test assertion failed").contains("TestOp"));
    }

    #[tokio::test]
    async fn test_list_campaigns_stub() {
        let result = list_campaigns().await;
        assert!(result.is_ok());
        assert!(result.expect("Test assertion failed").is_empty());
    }

    #[tokio::test]
    async fn test_get_campaign_stub() {
        let result = get_campaign("TestOp".to_string()).await;
        assert!(result.is_ok());
        assert!(result.expect("Test assertion failed").is_none());
    }

    #[tokio::test]
    async fn test_advance_campaign_stub() {
        let result = advance_campaign("TestOp".to_string()).await;
        assert!(result.is_ok());
        assert!(result.expect("Test assertion failed").contains("advanced"));
    }
}
