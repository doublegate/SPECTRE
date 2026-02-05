//! Campaign persistence using SQLite

use std::path::{Path, PathBuf};

use rusqlite::{params, Connection};
use tracing::{debug, info, instrument};

use super::{Artifact, ArtifactType, Campaign, CampaignPhase};

/// Campaign persistent storage using SQLite
pub struct CampaignStore {
    conn: Connection,
}

impl CampaignStore {
    /// Open or create a campaign database
    #[instrument(skip_all, fields(path = %path.display()))]
    pub fn open(path: &Path) -> crate::Result<Self> {
        info!("Opening campaign database");

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path).map_err(|e| {
            crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(e.to_string()))
        })?;

        let store = Self { conn };
        store.initialize_schema()?;

        debug!("Campaign database ready");
        Ok(store)
    }

    /// Open an in-memory database (for testing)
    pub fn open_in_memory() -> crate::Result<Self> {
        let conn = Connection::open_in_memory().map_err(|e| {
            crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(e.to_string()))
        })?;

        let store = Self { conn };
        store.initialize_schema()?;
        Ok(store)
    }

    /// Initialize the database schema
    fn initialize_schema(&self) -> crate::Result<()> {
        self.conn
            .execute_batch(
                "
            CREATE TABLE IF NOT EXISTS campaigns (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                phase TEXT NOT NULL DEFAULT 'planning',
                objectives TEXT NOT NULL DEFAULT '[]',
                targets TEXT NOT NULL DEFAULT '[]',
                tags TEXT NOT NULL DEFAULT '[]',
                notes TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                started_at TEXT,
                completed_at TEXT
            );

            CREATE TABLE IF NOT EXISTS artifacts (
                id TEXT PRIMARY KEY,
                campaign_id TEXT NOT NULL,
                artifact_type TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                mime_type TEXT NOT NULL,
                data BLOB NOT NULL,
                size INTEGER NOT NULL,
                hash TEXT NOT NULL,
                tags TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL,
                FOREIGN KEY (campaign_id) REFERENCES campaigns(id)
            );

            CREATE INDEX IF NOT EXISTS idx_artifacts_campaign
                ON artifacts(campaign_id);
        ",
            )
            .map_err(|e| {
                crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                ))
            })?;

        Ok(())
    }

    /// Save a campaign
    pub fn save_campaign(&self, campaign: &Campaign) -> crate::Result<()> {
        let objectives = serde_json::to_string(&campaign.objectives)?;
        let targets = serde_json::to_string(&campaign.targets)?;
        let tags = serde_json::to_string(&campaign.tags)?;

        self.conn
            .execute(
                "INSERT OR REPLACE INTO campaigns
                (id, name, description, phase, objectives, targets, tags, notes,
                 created_at, updated_at, started_at, completed_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                params![
                    campaign.id,
                    campaign.name,
                    campaign.description,
                    campaign.phase.to_string(),
                    objectives,
                    targets,
                    tags,
                    campaign.notes,
                    campaign.created_at.to_rfc3339(),
                    campaign.updated_at.to_rfc3339(),
                    campaign.started_at.map(|t| t.to_rfc3339()),
                    campaign.completed_at.map(|t| t.to_rfc3339()),
                ],
            )
            .map_err(|e| {
                crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                ))
            })?;

        Ok(())
    }

    /// Load a campaign by ID
    pub fn load_campaign(&self, id: &str) -> crate::Result<Campaign> {
        let campaign = self
            .conn
            .query_row(
                "SELECT id, name, description, phase, objectives, targets, tags, notes,
                    created_at, updated_at, started_at, completed_at
                 FROM campaigns WHERE id = ?1",
                params![id],
                |row| {
                    Ok(CampaignRow {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        phase: row.get(3)?,
                        objectives: row.get(4)?,
                        targets: row.get(5)?,
                        tags: row.get(6)?,
                        notes: row.get(7)?,
                        created_at: row.get(8)?,
                        updated_at: row.get(9)?,
                        started_at: row.get(10)?,
                        completed_at: row.get(11)?,
                    })
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => crate::SpectreError::Campaign(
                    crate::error::CampaignError::NotFound(id.to_string()),
                ),
                _ => crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                )),
            })?;

        campaign_from_row(campaign)
    }

    /// List all campaigns
    pub fn list_campaigns(&self) -> crate::Result<Vec<Campaign>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, description, phase, objectives, targets, tags, notes,
                    created_at, updated_at, started_at, completed_at
                 FROM campaigns ORDER BY updated_at DESC",
            )
            .map_err(|e| {
                crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                ))
            })?;

        let rows = stmt
            .query_map([], |row| {
                Ok(CampaignRow {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    phase: row.get(3)?,
                    objectives: row.get(4)?,
                    targets: row.get(5)?,
                    tags: row.get(6)?,
                    notes: row.get(7)?,
                    created_at: row.get(8)?,
                    updated_at: row.get(9)?,
                    started_at: row.get(10)?,
                    completed_at: row.get(11)?,
                })
            })
            .map_err(|e| {
                crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                ))
            })?;

        let mut campaigns = Vec::new();
        for row in rows {
            let row = row.map_err(|e| {
                crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                ))
            })?;
            campaigns.push(campaign_from_row(row)?);
        }

        Ok(campaigns)
    }

    /// Delete a campaign and its artifacts
    pub fn delete_campaign(&self, id: &str) -> crate::Result<()> {
        self.conn
            .execute("DELETE FROM artifacts WHERE campaign_id = ?1", params![id])
            .map_err(|e| {
                crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                ))
            })?;

        let deleted = self
            .conn
            .execute("DELETE FROM campaigns WHERE id = ?1", params![id])
            .map_err(|e| {
                crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                ))
            })?;

        if deleted == 0 {
            return Err(crate::SpectreError::Campaign(
                crate::error::CampaignError::NotFound(id.to_string()),
            ));
        }

        Ok(())
    }

    /// Save an artifact
    pub fn save_artifact(&self, artifact: &Artifact) -> crate::Result<()> {
        let tags = serde_json::to_string(&artifact.tags)?;

        self.conn
            .execute(
                "INSERT OR REPLACE INTO artifacts
                (id, campaign_id, artifact_type, name, description, mime_type,
                 data, size, hash, tags, created_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    artifact.id,
                    artifact.campaign_id,
                    artifact.artifact_type.to_string(),
                    artifact.name,
                    artifact.description,
                    artifact.mime_type,
                    artifact.data,
                    artifact.size as i64,
                    artifact.hash,
                    tags,
                    artifact.created_at.to_rfc3339(),
                ],
            )
            .map_err(|e| {
                crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                ))
            })?;

        Ok(())
    }

    /// List artifacts for a campaign
    pub fn list_artifacts(&self, campaign_id: &str) -> crate::Result<Vec<Artifact>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, campaign_id, artifact_type, name, description, mime_type,
                    data, size, hash, tags, created_at
                 FROM artifacts WHERE campaign_id = ?1 ORDER BY created_at DESC",
            )
            .map_err(|e| {
                crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                ))
            })?;

        let rows = stmt
            .query_map(params![campaign_id], |row| {
                let artifact_type_str: String = row.get(2)?;
                let tags_str: String = row.get(9)?;
                let created_at_str: String = row.get(10)?;

                Ok(ArtifactRow {
                    id: row.get(0)?,
                    campaign_id: row.get(1)?,
                    artifact_type: artifact_type_str,
                    name: row.get(3)?,
                    description: row.get(4)?,
                    mime_type: row.get(5)?,
                    data: row.get(6)?,
                    size: row.get::<_, i64>(7)? as usize,
                    hash: row.get(8)?,
                    tags: tags_str,
                    created_at: created_at_str,
                })
            })
            .map_err(|e| {
                crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                ))
            })?;

        let mut artifacts = Vec::new();
        for row in rows {
            let row = row.map_err(|e| {
                crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                    e.to_string(),
                ))
            })?;
            artifacts.push(artifact_from_row(row)?);
        }

        Ok(artifacts)
    }

    /// Get the database path (returns None for in-memory databases)
    pub fn path(&self) -> Option<PathBuf> {
        self.conn
            .path()
            .filter(|p| !p.is_empty())
            .map(PathBuf::from)
    }
}

/// Internal row structure for campaigns
struct CampaignRow {
    id: String,
    name: String,
    description: String,
    phase: String,
    objectives: String,
    targets: String,
    tags: String,
    notes: String,
    created_at: String,
    updated_at: String,
    started_at: Option<String>,
    completed_at: Option<String>,
}

/// Internal row structure for artifacts
struct ArtifactRow {
    id: String,
    campaign_id: String,
    artifact_type: String,
    name: String,
    description: Option<String>,
    mime_type: String,
    data: Vec<u8>,
    size: usize,
    hash: String,
    tags: String,
    created_at: String,
}

/// Convert a database row into a Campaign
fn campaign_from_row(row: CampaignRow) -> crate::Result<Campaign> {
    let phase = match row.phase.as_str() {
        "planning" => CampaignPhase::Planning,
        "recon" => CampaignPhase::Recon,
        "analysis" => CampaignPhase::Analysis,
        "exploitation" => CampaignPhase::Exploitation,
        "reporting" => CampaignPhase::Reporting,
        "complete" => CampaignPhase::Complete,
        "archived" => CampaignPhase::Archived,
        other => {
            return Err(crate::SpectreError::Campaign(
                crate::error::CampaignError::InvalidState(format!("Unknown phase: {}", other)),
            ));
        },
    };

    let objectives: Vec<String> = serde_json::from_str(&row.objectives)?;
    let targets: Vec<String> = serde_json::from_str(&row.targets)?;
    let tags: Vec<String> = serde_json::from_str(&row.tags)?;

    let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
        .map_err(|e| {
            crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(e.to_string()))
        })?
        .with_timezone(&chrono::Utc);

    let updated_at = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
        .map_err(|e| {
            crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(e.to_string()))
        })?
        .with_timezone(&chrono::Utc);

    let started_at = row
        .started_at
        .as_deref()
        .map(|s| {
            chrono::DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|e| {
                    crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                        e.to_string(),
                    ))
                })
        })
        .transpose()?;

    let completed_at = row
        .completed_at
        .as_deref()
        .map(|s| {
            chrono::DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|e| {
                    crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(
                        e.to_string(),
                    ))
                })
        })
        .transpose()?;

    Ok(Campaign {
        id: row.id,
        name: row.name,
        description: row.description,
        phase,
        objectives,
        targets,
        tags,
        notes: row.notes,
        created_at,
        updated_at,
        started_at,
        completed_at,
    })
}

/// Convert a database row into an Artifact
fn artifact_from_row(row: ArtifactRow) -> crate::Result<Artifact> {
    let artifact_type = match row.artifact_type.as_str() {
        "scan_result" => ArtifactType::ScanResult,
        "analysis_output" => ArtifactType::AnalysisOutput,
        "finding_report" => ArtifactType::FindingReport,
        "screenshot" => ArtifactType::Screenshot,
        "log_file" => ArtifactType::LogFile,
        "config_snapshot" => ArtifactType::ConfigSnapshot,
        other => {
            if let Some(name) = other.strip_prefix("custom:") {
                ArtifactType::Custom(name.to_string())
            } else {
                ArtifactType::Custom(other.to_string())
            }
        },
    };

    let tags: Vec<String> = serde_json::from_str(&row.tags)?;

    let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
        .map_err(|e| {
            crate::SpectreError::Campaign(crate::error::CampaignError::DatabaseError(e.to_string()))
        })?
        .with_timezone(&chrono::Utc);

    Ok(Artifact {
        id: row.id,
        campaign_id: row.campaign_id,
        artifact_type,
        name: row.name,
        description: row.description,
        mime_type: row.mime_type,
        data: row.data,
        size: row.size,
        hash: row.hash,
        tags,
        created_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_create() {
        let store = CampaignStore::open_in_memory().unwrap();
        assert!(store.path().is_none()); // in-memory has no path
    }

    #[test]
    fn test_save_and_load_campaign() {
        let store = CampaignStore::open_in_memory().unwrap();

        let mut campaign = Campaign::new("test", "test campaign");
        campaign.add_objective("Find vulns");
        campaign.add_target("10.0.0.0/24");
        campaign.add_tag("test");

        store.save_campaign(&campaign).unwrap();

        let loaded = store.load_campaign(&campaign.id).unwrap();
        assert_eq!(loaded.name, "test");
        assert_eq!(loaded.objectives.len(), 1);
        assert_eq!(loaded.targets.len(), 1);
        assert_eq!(loaded.tags.len(), 1);
    }

    #[test]
    fn test_list_campaigns() {
        let store = CampaignStore::open_in_memory().unwrap();

        store
            .save_campaign(&Campaign::new("campaign-1", "first"))
            .unwrap();
        store
            .save_campaign(&Campaign::new("campaign-2", "second"))
            .unwrap();

        let campaigns = store.list_campaigns().unwrap();
        assert_eq!(campaigns.len(), 2);
    }

    #[test]
    fn test_delete_campaign() {
        let store = CampaignStore::open_in_memory().unwrap();

        let campaign = Campaign::new("to-delete", "will be deleted");
        let id = campaign.id.clone();
        store.save_campaign(&campaign).unwrap();

        store.delete_campaign(&id).unwrap();
        let result = store.load_campaign(&id);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_nonexistent() {
        let store = CampaignStore::open_in_memory().unwrap();
        let result = store.delete_campaign("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_save_and_load_artifact() {
        let store = CampaignStore::open_in_memory().unwrap();

        let campaign = Campaign::new("test", "test");
        store.save_campaign(&campaign).unwrap();

        let artifact = Artifact::new(
            &campaign.id,
            ArtifactType::ScanResult,
            "results.json",
            b"{\"hosts\": []}".to_vec(),
        );

        store.save_artifact(&artifact).unwrap();

        let artifacts = store.list_artifacts(&campaign.id).unwrap();
        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].name, "results.json");
        assert_eq!(artifacts[0].artifact_type, ArtifactType::ScanResult);
    }

    #[test]
    fn test_campaign_update() {
        let store = CampaignStore::open_in_memory().unwrap();

        let mut campaign = Campaign::new("test", "test");
        store.save_campaign(&campaign).unwrap();

        campaign.advance_phase().unwrap(); // Planning -> Recon
        store.save_campaign(&campaign).unwrap();

        let loaded = store.load_campaign(&campaign.id).unwrap();
        assert_eq!(loaded.phase, CampaignPhase::Recon);
    }

    #[test]
    fn test_cascade_delete_artifacts() {
        let store = CampaignStore::open_in_memory().unwrap();

        let campaign = Campaign::new("test", "test");
        store.save_campaign(&campaign).unwrap();

        let artifact = Artifact::new(
            &campaign.id,
            ArtifactType::LogFile,
            "log.txt",
            b"log data".to_vec(),
        );
        store.save_artifact(&artifact).unwrap();

        store.delete_campaign(&campaign.id).unwrap();

        let artifacts = store.list_artifacts(&campaign.id).unwrap();
        assert!(artifacts.is_empty());
    }

    #[test]
    fn test_load_nonexistent_campaign() {
        let store = CampaignStore::open_in_memory().unwrap();
        let result = store.load_campaign("does-not-exist");
        assert!(result.is_err());
    }
}
