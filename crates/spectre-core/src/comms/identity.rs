//! Identity management for WRAITH communications

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};

use super::IdentitySummary;
use crate::config::Config;
use crate::error::CommsError;

/// Key type for identity generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum KeyType {
    /// Ed25519 (default, recommended)
    #[default]
    Ed25519,
    /// X25519 (for key exchange only)
    X25519,
    /// Kyber (post-quantum)
    Kyber,
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ed25519 => write!(f, "Ed25519"),
            Self::X25519 => write!(f, "X25519"),
            Self::Kyber => write!(f, "Kyber"),
        }
    }
}

/// Export format for identities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExportFormat {
    /// PEM format
    #[default]
    Pem,
    /// JSON format
    Json,
    /// Raw binary
    Raw,
    /// QR code
    Qr,
}

/// Cryptographic identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// Unique identifier (derived from public key)
    id: String,
    /// Optional name/alias
    name: Option<String>,
    /// Key type
    key_type: KeyType,
    /// Public key (base64 encoded)
    public_key: String,
    /// Private key (base64 encoded, encrypted at rest)
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<String>,
    /// Creation timestamp
    created_at: DateTime<Utc>,
    /// Optional comment
    comment: Option<String>,
}

impl Identity {
    /// Generate a new identity
    #[instrument(skip_all, fields(name = name, key_type = ?key_type))]
    pub fn generate(name: &str, key_type: KeyType, comment: Option<&str>) -> crate::Result<Self> {
        info!("Generating new identity");

        // TODO: When WRAITH library is integrated, use real key generation
        // For now, generate stub keys

        use rand::RngCore;
        let mut rng = rand::thread_rng();

        let mut private_key_bytes = vec![0u8; 32];
        rng.fill_bytes(&mut private_key_bytes);

        let mut public_key_bytes = vec![0u8; 32];
        rng.fill_bytes(&mut public_key_bytes);

        use base64::Engine;
        let private_key = base64::engine::general_purpose::STANDARD.encode(&private_key_bytes);
        let public_key = base64::engine::general_purpose::STANDARD.encode(&public_key_bytes);

        // Generate ID from public key
        let id = super::compute_fingerprint(&public_key_bytes)
            .replace(':', "")
            .to_lowercase();

        debug!(id = %id, "Identity generated");

        Ok(Self {
            id,
            name: Some(name.to_string()),
            key_type,
            public_key,
            private_key: Some(private_key),
            created_at: Utc::now(),
            comment: comment.map(String::from),
        })
    }

    /// Get the identity ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the identity name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the identity name
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    /// Get the key type
    pub fn key_type(&self) -> &KeyType {
        &self.key_type
    }

    /// Get the creation timestamp
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Get the comment
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    /// Get the public key fingerprint
    pub fn public_key_fingerprint(&self) -> String {
        use base64::Engine;
        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&self.public_key) {
            super::compute_fingerprint(&bytes)
        } else {
            "invalid".to_string()
        }
    }

    /// Get the encoded private key (dangerous!)
    pub fn private_key_encoded(&self) -> crate::Result<String> {
        self.private_key.clone().ok_or_else(|| {
            crate::SpectreError::Comms(CommsError::KeyError("No private key".to_string()))
        })
    }

    /// Convert to JSON value
    pub fn to_json(&self, include_private: bool) -> crate::Result<serde_json::Value> {
        let mut json = serde_json::to_value(self)?;

        if !include_private {
            if let Some(obj) = json.as_object_mut() {
                obj.remove("private_key");
            }
        }

        Ok(json)
    }

    /// Export identity
    pub fn export(&self, include_private: bool, format: ExportFormat) -> crate::Result<Vec<u8>> {
        match format {
            ExportFormat::Json => {
                let json = self.to_json(include_private)?;
                Ok(serde_json::to_vec_pretty(&json)?)
            },
            ExportFormat::Pem => {
                use base64::Engine;
                let public_bytes =
                    base64::engine::general_purpose::STANDARD.decode(&self.public_key)?;

                let mut pem = String::new();
                pem.push_str("-----BEGIN SPECTRE PUBLIC KEY-----\n");
                pem.push_str(&base64::engine::general_purpose::STANDARD.encode(&public_bytes));
                pem.push_str("\n-----END SPECTRE PUBLIC KEY-----\n");

                if include_private {
                    if let Some(private) = &self.private_key {
                        let private_bytes =
                            base64::engine::general_purpose::STANDARD.decode(private)?;
                        pem.push_str("-----BEGIN SPECTRE PRIVATE KEY-----\n");
                        pem.push_str(
                            &base64::engine::general_purpose::STANDARD.encode(&private_bytes),
                        );
                        pem.push_str("\n-----END SPECTRE PRIVATE KEY-----\n");
                    }
                }

                Ok(pem.into_bytes())
            },
            ExportFormat::Raw => {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD
                    .decode(&self.public_key)
                    .map_err(|e| crate::SpectreError::Comms(CommsError::KeyError(e.to_string())))
            },
            ExportFormat::Qr => {
                // QR code generation would require additional dependency
                // For now, return the fingerprint as text
                Ok(self.public_key_fingerprint().into_bytes())
            },
        }
    }

    /// Import an identity from data
    pub fn import(data: &[u8]) -> crate::Result<Self> {
        // Try JSON first
        if let Ok(identity) = serde_json::from_slice::<Identity>(data) {
            return Ok(identity);
        }

        // Try PEM format
        let text = String::from_utf8_lossy(data);
        if text.contains("BEGIN SPECTRE PUBLIC KEY") {
            // Parse PEM format
            // For now, return error - full PEM parsing would be more complex
            return Err(crate::SpectreError::Comms(CommsError::KeyError(
                "PEM import not fully implemented".to_string(),
            )));
        }

        Err(crate::SpectreError::Comms(CommsError::KeyError(
            "Unknown identity format".to_string(),
        )))
    }

    /// Save identity to storage
    pub fn save(&self, config: &Config) -> crate::Result<()> {
        let identity_dir = get_identity_dir(config)?;
        std::fs::create_dir_all(&identity_dir)?;

        let path = identity_dir.join(format!("{}.json", self.id));
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;

        // Set as active if no active identity
        if Self::get_active_id(config).is_err() {
            Self::set_active(&self.id, config)?;
        }

        Ok(())
    }

    /// Load the active identity
    pub fn load_active(config: &Config) -> crate::Result<Self> {
        let active_id = Self::get_active_id(config)?;
        Self::load_by_id(&active_id, config)
    }

    /// Load an identity by ID
    pub fn load_by_id(id: &str, config: &Config) -> crate::Result<Self> {
        let identity_dir = get_identity_dir(config)?;
        let path = identity_dir.join(format!("{}.json", id));

        let content = std::fs::read_to_string(&path).map_err(|_| {
            crate::SpectreError::Comms(CommsError::IdentityNotFound(id.to_string()))
        })?;

        let identity: Identity = serde_json::from_str(&content)?;
        Ok(identity)
    }

    /// Load an identity by name
    pub fn load_by_name(name: &str, config: &Config) -> crate::Result<Self> {
        let identities = Self::list(config)?;

        for summary in identities {
            if summary.name.as_deref() == Some(name) {
                return Self::load_by_id(&summary.id, config);
            }
        }

        Err(crate::SpectreError::Comms(CommsError::IdentityNotFound(
            name.to_string(),
        )))
    }

    /// Get the active identity ID
    pub fn get_active_id(config: &Config) -> crate::Result<String> {
        let identity_dir = get_identity_dir(config)?;
        let active_path = identity_dir.join("active");

        std::fs::read_to_string(&active_path).map_err(|_| {
            crate::SpectreError::Comms(CommsError::IdentityNotFound(
                "No active identity".to_string(),
            ))
        })
    }

    /// Set the active identity
    pub fn set_active(id: &str, config: &Config) -> crate::Result<()> {
        let identity_dir = get_identity_dir(config)?;
        std::fs::create_dir_all(&identity_dir)?;

        let active_path = identity_dir.join("active");
        std::fs::write(active_path, id)?;

        Ok(())
    }

    /// List all identities
    pub fn list(config: &Config) -> crate::Result<Vec<IdentitySummary>> {
        let identity_dir = get_identity_dir(config)?;

        if !identity_dir.exists() {
            return Ok(Vec::new());
        }

        let mut identities = Vec::new();

        for entry in std::fs::read_dir(identity_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(identity) = serde_json::from_str::<Identity>(&content) {
                        identities.push(IdentitySummary {
                            id: identity.id,
                            name: identity.name,
                            created_at: identity.created_at,
                        });
                    }
                }
            }
        }

        Ok(identities)
    }

    /// Delete an identity by name
    pub fn delete(name: &str, config: &Config) -> crate::Result<()> {
        let identity = Self::load_by_name(name, config)?;
        let identity_dir = get_identity_dir(config)?;
        let path = identity_dir.join(format!("{}.json", identity.id));

        std::fs::remove_file(path)?;

        // If this was the active identity, clear active
        if Self::get_active_id(config).ok().as_deref() == Some(&identity.id) {
            let active_path = identity_dir.join("active");
            let _ = std::fs::remove_file(active_path);
        }

        Ok(())
    }
}

/// Get the identity storage directory
fn get_identity_dir(config: &Config) -> crate::Result<PathBuf> {
    if let Some(dir) = &config.comms.identity_dir {
        return Ok(dir.clone());
    }

    directories::ProjectDirs::from("com", "spectre", "spectre")
        .map(|dirs| dirs.data_dir().join("identities"))
        .ok_or_else(|| {
            crate::SpectreError::Comms(CommsError::KeyError(
                "Could not determine identity directory".to_string(),
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_generation() {
        let identity = Identity::generate("test", KeyType::Ed25519, Some("test comment")).unwrap();

        assert!(!identity.id().is_empty());
        assert_eq!(identity.name(), Some("test"));
        assert_eq!(*identity.key_type(), KeyType::Ed25519);
        assert_eq!(identity.comment(), Some("test comment"));
    }

    #[test]
    fn test_key_type_display() {
        assert_eq!(KeyType::Ed25519.to_string(), "Ed25519");
        assert_eq!(KeyType::X25519.to_string(), "X25519");
        assert_eq!(KeyType::Kyber.to_string(), "Kyber");
    }

    #[test]
    fn test_identity_export_json() {
        let identity = Identity::generate("test", KeyType::Ed25519, None).unwrap();
        let exported = identity.export(false, ExportFormat::Json).unwrap();

        let json: serde_json::Value = serde_json::from_slice(&exported).unwrap();
        assert!(json.get("id").is_some());
        assert!(json.get("private_key").is_none()); // Should not include private key
    }

    #[test]
    fn test_identity_export_pem() {
        let identity = Identity::generate("test", KeyType::Ed25519, None).unwrap();
        let exported = identity.export(false, ExportFormat::Pem).unwrap();

        let pem = String::from_utf8(exported).unwrap();
        assert!(pem.contains("BEGIN SPECTRE PUBLIC KEY"));
        assert!(pem.contains("END SPECTRE PUBLIC KEY"));
        assert!(!pem.contains("PRIVATE KEY"));
    }
}
