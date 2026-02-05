//! Secure communications module (WRAITH-Protocol integration)
//!
//! This module provides the interface for secure end-to-end encrypted communications
//! using the WRAITH-Protocol library.
//!
//! # Architecture
//!
//! The module has three layers:
//!
//! 1. **Identity management**: Cryptographic identity generation,
//!    storage, and retrieval. Uses real WRAITH Ed25519/X25519 key generation.
//!
//! 2. **WRAITH adapter** ([`wraith_adapter`]): Bridges SPECTRE's communication
//!    interface with the real WRAITH protocol stack (`wraith_core::Node`).
//!    Handles type conversion, error mapping, and async operations.
//!
//! 3. **Stub client** ([`WRAITHClient`]): A lightweight stub for testing
//!    environments that do not require real network operations.
//!
//! # Integration Details
//!
//! WRAITH-Protocol provides:
//! - 10+ Gbps throughput with E2EE
//! - Noise_XX mutual authentication
//! - Forward secrecy via key ratcheting
//! - Ed25519 identity + X25519 key exchange

mod identity;
mod peer;
pub mod wraith_adapter;

use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use identity::{ExportFormat, Identity, KeyType};
pub use peer::Peer;

use crate::config::Config;
use crate::error::CommsError;

/// WRAITH protocol version
pub const WRAITH_VERSION: &str = "2.3.7";

/// Load the active identity
pub fn load_identity(config: &Config) -> crate::Result<Identity> {
    Identity::load_active(config)
}

/// Load an identity by name
pub fn load_identity_by_name(name: &str, config: &Config) -> crate::Result<Identity> {
    Identity::load_by_name(name, config)
}

/// Set the active identity
pub fn set_active_identity(id: &str, config: &Config) -> crate::Result<()> {
    Identity::set_active(id, config)
}

/// Get the active identity ID
pub fn get_active_identity_id(config: &Config) -> crate::Result<String> {
    Identity::get_active_id(config)
}

/// List all identities
pub fn list_identities(config: &Config) -> crate::Result<Vec<IdentitySummary>> {
    Identity::list(config)
}

/// Delete an identity by name
pub fn delete_identity(name: &str, config: &Config) -> crate::Result<()> {
    Identity::delete(name, config)
}

/// Compute a fingerprint for a public key
pub fn compute_fingerprint(public_key: &[u8]) -> String {
    use sha2::{Digest, Sha256};

    let hash = Sha256::digest(public_key);
    let hex = hex::encode(&hash[..16]); // Use first 16 bytes

    // Format as colon-separated pairs
    hex.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(":")
        .to_uppercase()
}

/// Resolve a peer by ID or alias
pub fn resolve_peer(identifier: &str, config: &Config) -> crate::Result<Peer> {
    Peer::resolve(identifier, config)
}

/// List all peers
pub fn list_peers(config: &Config) -> crate::Result<Vec<PeerSummary>> {
    Peer::list(config)
}

/// Remove a peer
pub fn remove_peer(id: &str, config: &Config) -> crate::Result<()> {
    Peer::remove(id, config)
}

/// Create a WRAITH client backed by a real WRAITH protocol node.
///
/// This returns a [`wraith_adapter::WraithNode`] that delegates to the real
/// WRAITH protocol stack for cryptographic handshakes, encryption, and transport.
///
/// For testing environments, use [`create_stub_client`] instead.
pub async fn create_client(
    config: &Config,
    identity: &Identity,
) -> crate::Result<wraith_adapter::WraithNode> {
    wraith_adapter::create_wraith_node(config, identity).await
}

/// Create a stub WRAITH client for testing environments.
///
/// The stub client simulates communication behavior without performing real
/// network operations. It is useful for unit and integration tests that test
/// SPECTRE's orchestration logic without requiring a running WRAITH node.
pub async fn create_stub_client(
    config: &Config,
    identity: &Identity,
) -> crate::Result<WRAITHClient> {
    WRAITHClient::new(config, identity).await
}

/// Identity summary for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentitySummary {
    /// Identity ID
    pub id: String,
    /// Identity name
    pub name: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Peer summary for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerSummary {
    /// Peer ID
    pub id: String,
    /// Peer alias
    pub alias: Option<String>,
    /// Trust level
    pub trust: u8,
    /// Last seen timestamp
    pub last_seen: Option<DateTime<Utc>>,
}

/// Send options
#[derive(Debug, Clone, Default)]
pub struct SendOptions {
    /// Request delivery confirmation
    pub confirm_delivery: bool,
    /// Message priority (0-9)
    pub priority: u8,
    /// Message expiration
    pub expires_in: Option<Duration>,
    /// Enable compression
    pub compress: bool,
    /// Use opportunistic encryption
    pub opportunistic: bool,
    /// Custom relay server
    pub relay: Option<String>,
    /// Operation timeout
    pub timeout: Duration,
}

/// Receive options
#[derive(Debug, Clone, Default)]
pub struct ReceiveOptions {
    /// Filter by sender
    pub filter_sender: Option<String>,
    /// Accept from unknown peers
    pub accept_unknown: bool,
    /// Auto-confirm delivery
    pub auto_confirm: bool,
    /// Custom relay server
    pub relay: Option<String>,
    /// Operation timeout
    pub timeout: Option<Duration>,
}

/// Send receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendReceipt {
    /// Message ID
    pub message_id: String,
    /// Whether delivery was confirmed
    pub delivered: bool,
    /// Timestamp
    pub timestamp: Option<DateTime<Utc>>,
}

/// Received message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivedMessage {
    /// Message ID
    pub id: String,
    /// Sender ID
    pub sender: String,
    /// Message data
    pub data: Vec<u8>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Expiration time
    pub expires_at: Option<DateTime<Utc>>,
}

/// Relay connectivity status
#[derive(Debug, Clone)]
pub struct RelayStatus {
    /// Whether connected to relay
    pub connected: bool,
    /// Latency in milliseconds
    pub latency_ms: Option<u64>,
}

/// WRAITH client for secure communications
pub struct WRAITHClient {
    #[allow(dead_code)]
    identity: Identity,
    #[allow(dead_code)]
    relay_url: Option<String>,
    #[allow(dead_code)]
    timeout: Duration,
}

impl WRAITHClient {
    /// Create a new WRAITH client
    pub async fn new(config: &Config, identity: &Identity) -> crate::Result<Self> {
        Ok(Self {
            identity: identity.clone(),
            relay_url: config.comms.relay_url.clone(),
            timeout: Duration::from_secs(config.comms.timeout),
        })
    }

    /// Send a message to a peer
    #[allow(clippy::unused_async)]
    pub async fn send(
        &self,
        _peer: &Peer,
        data: &[u8],
        options: &SendOptions,
        progress: Option<Box<dyn Fn(usize) + Send>>,
    ) -> crate::Result<SendReceipt> {
        // TODO: When WRAITH library is integrated, perform actual send

        // Simulate send progress
        if let Some(callback) = progress {
            for i in (0..=data.len()).step_by(1024) {
                callback(i.min(data.len()));
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }

        // Generate a message ID
        let message_id = format!(
            "{}-{}",
            &self.identity.id()[..8],
            chrono::Utc::now().timestamp_millis()
        );

        Ok(SendReceipt {
            message_id,
            delivered: options.confirm_delivery,
            timestamp: Some(chrono::Utc::now()),
        })
    }

    /// Receive a message
    #[allow(clippy::unused_async)]
    pub async fn receive(&self, options: &ReceiveOptions) -> crate::Result<ReceivedMessage> {
        // TODO: When WRAITH library is integrated, perform actual receive

        // Simulate timeout
        if let Some(timeout) = options.timeout {
            tokio::time::sleep(timeout).await;
            return Err(crate::SpectreError::Comms(CommsError::Timeout));
        }

        // Return stub message
        Ok(ReceivedMessage {
            id: format!("msg-{}", chrono::Utc::now().timestamp_millis()),
            sender: "unknown".to_string(),
            data: b"[stub message]".to_vec(),
            timestamp: chrono::Utc::now(),
            expires_at: None,
        })
    }

    /// Check relay connectivity
    #[allow(clippy::unused_async)]
    pub async fn check_relay_connectivity(&self) -> crate::Result<RelayStatus> {
        // TODO: When WRAITH library is integrated, perform actual connectivity check

        if self.relay_url.is_some() {
            Ok(RelayStatus {
                connected: true,
                latency_ms: Some(50),
            })
        } else {
            Ok(RelayStatus {
                connected: false,
                latency_ms: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_fingerprint() {
        let key = b"test public key data";
        let fingerprint = compute_fingerprint(key);

        // Should be formatted as colon-separated hex
        assert!(fingerprint.contains(':'));
        assert_eq!(fingerprint.len(), 47); // 16 bytes = 32 hex chars + 15 colons
    }

    #[test]
    fn test_send_options_default() {
        let opts = SendOptions::default();
        assert!(!opts.confirm_delivery);
        assert_eq!(opts.priority, 0);
    }
}
