//! Peer management for WRAITH communications

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};

use super::PeerSummary;
use crate::config::Config;
use crate::error::CommsError;

/// Trusted peer for secure communications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    /// Unique identifier (derived from public key)
    id: String,
    /// Optional alias
    alias: Option<String>,
    /// Public key (base64 encoded)
    public_key: String,
    /// Trust level (0-10)
    trust: u8,
    /// Notes about the peer
    notes: Option<String>,
    /// When the peer was added
    created_at: DateTime<Utc>,
    /// Last communication timestamp
    last_seen: Option<DateTime<Utc>>,
}

impl Peer {
    /// Create a new peer
    #[allow(clippy::needless_pass_by_value)] // Public API: callers always have Vec<u8>
    #[instrument(skip(public_key), fields(alias = ?alias, trust = trust))]
    pub fn new(
        public_key: Vec<u8>,
        alias: Option<String>,
        trust: u8,
        notes: Option<String>,
    ) -> Self {
        info!("Creating new peer");

        // Generate ID from public key
        let id = super::compute_fingerprint(&public_key)
            .replace(':', "")
            .to_lowercase();

        use base64::Engine;
        let public_key_encoded = base64::engine::general_purpose::STANDARD.encode(&public_key);

        debug!(id = %id, "Peer created");

        Self {
            id,
            alias,
            public_key: public_key_encoded,
            trust: trust.min(10),
            notes,
            created_at: Utc::now(),
            last_seen: None,
        }
    }

    /// Get the peer ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the peer alias
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    /// Set the peer alias
    pub fn set_alias(&mut self, alias: Option<String>) {
        self.alias = alias;
    }

    /// Get the trust level
    pub const fn trust(&self) -> u8 {
        self.trust
    }

    /// Set the trust level
    pub fn set_trust(&mut self, trust: u8) {
        self.trust = trust.min(10);
    }

    /// Get the notes
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    /// Set the notes
    pub fn set_notes(&mut self, notes: Option<String>) {
        self.notes = notes;
    }

    /// Get the creation timestamp
    pub const fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Get the last seen timestamp
    pub const fn last_seen(&self) -> Option<&DateTime<Utc>> {
        self.last_seen.as_ref()
    }

    /// Update last seen timestamp
    pub fn touch(&mut self) {
        self.last_seen = Some(Utc::now());
    }

    /// Get the public key fingerprint
    pub fn fingerprint(&self) -> String {
        use base64::Engine;
        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&self.public_key) {
            super::compute_fingerprint(&bytes)
        } else {
            "invalid".to_string()
        }
    }

    /// Get the key type (determined from key length)
    pub fn key_type(&self) -> &str {
        use base64::Engine;
        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&self.public_key) {
            match bytes.len() {
                32 => "Ed25519",
                _ => "Unknown",
            }
        } else {
            "Unknown"
        }
    }

    /// Save peer to storage
    pub fn save(&self, config: &Config) -> crate::Result<()> {
        let peer_dir = get_peer_dir(config)?;
        std::fs::create_dir_all(&peer_dir)?;

        let path = peer_dir.join(format!("{}.json", self.id));
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;

        Ok(())
    }

    /// Resolve a peer by ID or alias
    pub fn resolve(identifier: &str, config: &Config) -> crate::Result<Self> {
        // Try as ID first
        if let Ok(peer) = Self::load_by_id(identifier, config) {
            return Ok(peer);
        }

        // Try as alias
        Self::load_by_alias(identifier, config)
    }

    /// Load a peer by ID
    pub fn load_by_id(id: &str, config: &Config) -> crate::Result<Self> {
        let peer_dir = get_peer_dir(config)?;
        let path = peer_dir.join(format!("{}.json", id));

        let content = std::fs::read_to_string(&path)
            .map_err(|_| crate::SpectreError::Comms(CommsError::PeerNotFound(id.to_string())))?;

        let peer: Self = serde_json::from_str(&content)?;
        Ok(peer)
    }

    /// Load a peer by alias
    pub fn load_by_alias(alias: &str, config: &Config) -> crate::Result<Self> {
        let peers = Self::list(config)?;

        for summary in peers {
            if summary.alias.as_deref() == Some(alias) {
                return Self::load_by_id(&summary.id, config);
            }
        }

        Err(crate::SpectreError::Comms(CommsError::PeerNotFound(
            alias.to_string(),
        )))
    }

    /// List all peers
    pub fn list(config: &Config) -> crate::Result<Vec<PeerSummary>> {
        let peer_dir = get_peer_dir(config)?;

        if !peer_dir.exists() {
            return Ok(Vec::new());
        }

        let mut peers = Vec::new();

        for entry in std::fs::read_dir(peer_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(peer) = serde_json::from_str::<Self>(&content) {
                        peers.push(PeerSummary {
                            id: peer.id,
                            alias: peer.alias,
                            trust: peer.trust,
                            last_seen: peer.last_seen,
                        });
                    }
                }
            }
        }

        Ok(peers)
    }

    /// Remove a peer
    pub fn remove(id: &str, config: &Config) -> crate::Result<()> {
        let peer_dir = get_peer_dir(config)?;
        let path = peer_dir.join(format!("{}.json", id));

        std::fs::remove_file(path)
            .map_err(|_| crate::SpectreError::Comms(CommsError::PeerNotFound(id.to_string())))?;

        Ok(())
    }
}

/// Get the peer storage directory
fn get_peer_dir(config: &Config) -> crate::Result<PathBuf> {
    if let Some(dir) = &config.comms.peer_dir {
        return Ok(dir.clone());
    }

    directories::ProjectDirs::from("com", "spectre", "spectre")
        .map(|dirs| dirs.data_dir().join("peers"))
        .ok_or_else(|| {
            crate::SpectreError::Comms(CommsError::KeyError(
                "Could not determine peer directory".to_string(),
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_creation() {
        let public_key = vec![0u8; 32];
        let peer = Peer::new(
            public_key,
            Some("test_peer".to_string()),
            8,
            Some("Test notes".to_string()),
        );

        assert!(!peer.id().is_empty());
        assert_eq!(peer.alias(), Some("test_peer"));
        assert_eq!(peer.trust(), 8);
        assert_eq!(peer.notes(), Some("Test notes"));
    }

    #[test]
    fn test_peer_trust_clamping() {
        let public_key = vec![0u8; 32];
        let peer = Peer::new(public_key, None, 15, None); // Trust > 10

        assert_eq!(peer.trust(), 10); // Should be clamped
    }

    #[test]
    fn test_peer_fingerprint() {
        let public_key = vec![0u8; 32];
        let peer = Peer::new(public_key, None, 5, None);

        let fingerprint = peer.fingerprint();
        assert!(fingerprint.contains(':'));
    }
}
