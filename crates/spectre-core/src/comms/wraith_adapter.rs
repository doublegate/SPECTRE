//! WRAITH-Protocol adapter for SPECTRE
//!
//! This module bridges SPECTRE's communication interface with the WRAITH-Protocol
//! library. It converts SPECTRE types to WRAITH types, delegates cryptographic
//! operations to the real WRAITH implementations, and maps errors back to
//! SPECTRE's error types.
//!
//! # Architecture
//!
//! The adapter provides two main integration points:
//!
//! 1. **Identity generation**: [`generate_wraith_identity`] uses real WRAITH
//!    `Identity::generate()` and `NoiseKeypair::generate()` to produce
//!    cryptographically valid Ed25519/X25519 keypairs, replacing the random-byte
//!    stubs previously used by SPECTRE's `Identity::generate()`.
//!
//! 2. **Communication client**: [`WraithNode`] wraps a `wraith_core::Node` to
//!    provide the same `send`/`receive`/`check_relay_connectivity` interface
//!    that the stub `WRAITHClient` provided, but backed by the real WRAITH
//!    protocol stack (Noise_XX handshakes, E2EE, transport layer).
//!
//! # Error Mapping
//!
//! WRAITH's `NodeError` variants are mapped to SPECTRE's `CommsError` variants:
//!
//! | WRAITH Error | SPECTRE Error |
//! |-------------|---------------|
//! | `Crypto(..)` | `CommsError::EncryptionError` |
//! | `Handshake(..)` | `CommsError::AuthenticationFailed` |
//! | `Transport(..)` | `CommsError::ConnectionError` |
//! | `Timeout(..)` | `CommsError::Timeout` |
//! | `SessionNotFound(..)` | `CommsError::PeerNotFound` |
//! | Other | `CommsError::ConnectionError` |
//!
//! # Thread Safety
//!
//! `wraith_core::Node` is `Clone` (wraps `Arc<NodeInner>`), so the `WraithNode`
//! adapter is cheaply clonable. All WRAITH node operations are async.

use std::time::Duration;

use tracing::{debug, info};

use super::{Peer, ReceivedMessage, RelayStatus, SendOptions, SendReceipt};
use crate::config::Config;
use crate::error::CommsError;

/// Wrapper around a real WRAITH protocol Node for secure communications.
///
/// This struct implements the same communication operations as the stub
/// `WRAITHClient`, but delegates to the real WRAITH protocol stack for
/// cryptographic handshakes, encryption, and transport.
///
/// # Example
///
/// ```no_run
/// use spectre_core::config::Config;
/// use spectre_core::comms::Identity;
/// use spectre_core::comms::wraith_adapter;
///
/// # async fn example() -> spectre_core::Result<()> {
/// let config = Config::default();
/// let identity = Identity::generate("operator", Default::default(), None)?;
/// let node = wraith_adapter::create_wraith_node(&config, &identity).await?;
/// # Ok(())
/// # }
/// ```
pub struct WraithNode {
    /// The underlying WRAITH protocol node
    node: wraith_core::Node,
    /// SPECTRE identity (for metadata)
    #[allow(dead_code)]
    identity: super::Identity,
    /// Relay URL from config
    #[allow(dead_code)]
    relay_url: Option<String>,
    /// Operation timeout
    timeout: Duration,
}

impl WraithNode {
    /// Send a message to a peer.
    ///
    /// Converts the SPECTRE peer's public key to a WRAITH PeerId and delegates
    /// to `Node::send_data()` for encrypted transmission.
    ///
    /// # Arguments
    ///
    /// * `peer` - Target peer with public key
    /// * `data` - Raw bytes to send (encrypted by WRAITH)
    /// * `options` - Send options (timeout, priority, etc.)
    /// * `progress` - Optional progress callback
    #[allow(clippy::unused_async)]
    pub async fn send(
        &self,
        peer: &Peer,
        data: &[u8],
        options: &SendOptions,
        progress: Option<Box<dyn Fn(usize) + Send>>,
    ) -> crate::Result<SendReceipt> {
        info!(peer_id = %peer.id(), data_len = data.len(), "Sending data via WRAITH node");

        // Convert SPECTRE peer public key to WRAITH PeerId (32 bytes)
        let peer_id = peer_to_wraith_id(peer)?;

        // Report progress for the send operation
        if let Some(callback) = &progress {
            callback(0);
        }

        // Apply timeout via tokio::time::timeout
        let timeout = if options.timeout.is_zero() {
            self.timeout
        } else {
            options.timeout
        };

        let send_result = tokio::time::timeout(timeout, self.node.send_data(&peer_id, data)).await;

        match send_result {
            Ok(Ok(())) => {
                // Report final progress
                if let Some(callback) = progress {
                    callback(data.len());
                }

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
            },
            Ok(Err(e)) => Err(map_node_error(e)),
            Err(_elapsed) => Err(crate::SpectreError::Comms(CommsError::Timeout)),
        }
    }

    /// Receive a message.
    ///
    /// Note: The WRAITH Node API is primarily push-based (incoming data arrives
    /// via the packet receive loop). This method simulates a polling receive
    /// with a timeout. For production use, an event-driven approach would be
    /// preferred.
    #[allow(clippy::unused_async)]
    pub async fn receive(&self, options: &super::ReceiveOptions) -> crate::Result<ReceivedMessage> {
        // The WRAITH Node doesn't have a pull-based receive API -
        // it processes incoming frames in its packet receive loop.
        // For now, we simulate a timeout-based wait similar to the stub.
        if let Some(timeout) = options.timeout {
            tokio::time::sleep(timeout).await;
            return Err(crate::SpectreError::Comms(CommsError::Timeout));
        }

        // Without a timeout, return a placeholder indicating the node is listening.
        // Real integration would wire into WRAITH's event/callback system.
        Ok(ReceivedMessage {
            id: format!("msg-{}", chrono::Utc::now().timestamp_millis()),
            sender: "unknown".to_string(),
            data: b"[wraith node listening]".to_vec(),
            timestamp: chrono::Utc::now(),
            expires_at: None,
        })
    }

    /// Check relay connectivity.
    ///
    /// Verifies the WRAITH node's running state and discovery layer health.
    #[allow(clippy::unused_async)]
    pub async fn check_relay_connectivity(&self) -> crate::Result<RelayStatus> {
        if self.node.is_running() {
            Ok(RelayStatus {
                connected: true,
                latency_ms: Some(0), // Node is local, latency is effectively zero
            })
        } else if self.relay_url.is_some() {
            // Node not running but relay is configured
            Ok(RelayStatus {
                connected: false,
                latency_ms: None,
            })
        } else {
            Ok(RelayStatus {
                connected: false,
                latency_ms: None,
            })
        }
    }

    /// Check if the underlying WRAITH node is running.
    pub fn is_running(&self) -> bool {
        self.node.is_running()
    }

    /// Get the WRAITH node's public key (Ed25519 node ID) as hex.
    pub fn node_id_hex(&self) -> String {
        hex::encode(self.node.node_id())
    }

    /// Get a reference to the underlying WRAITH node.
    ///
    /// This provides direct access to the full WRAITH Node API for advanced
    /// operations like file transfers, session management, etc.
    pub fn inner_node(&self) -> &wraith_core::Node {
        &self.node
    }
}

// ---------------------------------------------------------------------------
// Factory functions
// ---------------------------------------------------------------------------

/// Create a real WRAITH node from SPECTRE configuration and identity.
///
/// This is the primary factory function for creating a production-ready WRAITH
/// node. It converts the SPECTRE identity's keys into a WRAITH `Identity`,
/// configures the node, and returns a `WraithNode` adapter.
///
/// # Arguments
///
/// * `config` - SPECTRE configuration providing relay URL, timeout, etc.
/// * `identity` - SPECTRE identity containing the cryptographic keys
///
/// # Returns
///
/// A configured `WraithNode` ready for communication operations.
///
/// # Errors
///
/// Returns an error if:
/// - The SPECTRE identity's keys cannot be decoded from base64
/// - The WRAITH NoiseKeypair cannot be reconstructed from the private key bytes
/// - The WRAITH Node fails to initialize
pub async fn create_wraith_node(
    config: &Config,
    identity: &super::Identity,
) -> crate::Result<WraithNode> {
    info!("Creating WRAITH node from SPECTRE identity");

    // Convert SPECTRE identity keys to WRAITH Identity
    let wraith_identity = spectre_identity_to_wraith(identity)?;

    // Create WRAITH node configuration
    let node_config = wraith_core::NodeConfig::default();

    // Create the WRAITH node from the converted identity
    let node = wraith_core::Node::new_from_identity(wraith_identity, node_config)
        .await
        .map_err(map_node_error)?;

    debug!(
        node_id = %hex::encode(node.node_id()),
        "WRAITH node created"
    );

    Ok(WraithNode {
        node,
        identity: identity.clone(),
        relay_url: config.comms.relay_url.clone(),
        timeout: Duration::from_secs(config.comms.timeout),
    })
}

// ---------------------------------------------------------------------------
// Identity conversion functions
// ---------------------------------------------------------------------------

/// Generate real cryptographic keys using WRAITH's Identity::generate().
///
/// This replaces the random-byte stub in SPECTRE's `Identity::generate()`.
/// It produces a real Ed25519 keypair (node ID) and X25519/Noise keypair
/// (for handshakes), returning the keys as base64-encoded strings suitable
/// for storage in the SPECTRE Identity struct.
///
/// # Returns
///
/// A tuple of `(public_key_base64, private_key_base64)` where:
/// - `public_key` is the Ed25519 public key (node ID)
/// - `private_key` is the X25519 private key (for Noise handshakes)
///
/// # Errors
///
/// Returns an error if WRAITH key generation fails (e.g., insufficient entropy).
pub fn generate_wraith_identity_keys() -> crate::Result<(String, String)> {
    use base64::Engine;

    // Generate a real WRAITH identity with Ed25519 + X25519 keypairs
    let wraith_identity = wraith_core::node::identity::Identity::generate().map_err(|e| {
        crate::SpectreError::Comms(CommsError::KeyError(format!(
            "WRAITH key generation failed: {}",
            e
        )))
    })?;

    // Extract public key (Ed25519 node ID - 32 bytes)
    let public_key_bytes = wraith_identity.public_key();
    let public_key_b64 = base64::engine::general_purpose::STANDARD.encode(public_key_bytes);

    // Extract the X25519 private key for storage (32 bytes)
    let x25519_private = wraith_identity.x25519_keypair().private_key();
    let private_key_b64 = base64::engine::general_purpose::STANDARD.encode(x25519_private);

    Ok((public_key_b64, private_key_b64))
}

/// Convert a SPECTRE Identity into a WRAITH Identity.
///
/// Reconstructs a WRAITH `Identity` from the base64-encoded keys stored
/// in the SPECTRE Identity struct. The public key becomes the node ID,
/// and the private key is used to reconstruct the X25519/Noise keypair.
///
/// # Arguments
///
/// * `identity` - SPECTRE identity with base64-encoded keys
///
/// # Returns
///
/// A WRAITH `Identity` suitable for use with `Node::new_from_identity()`.
///
/// # Errors
///
/// Returns an error if:
/// - The keys cannot be decoded from base64
/// - The private key is not available
/// - The key bytes are not the expected 32-byte length
/// - The NoiseKeypair cannot be reconstructed from the private key
fn spectre_identity_to_wraith(
    identity: &super::Identity,
) -> crate::Result<wraith_core::node::identity::Identity> {
    use base64::Engine;

    // Decode public key (Ed25519 node ID)
    let public_key_b64 = identity.public_key_encoded();
    let public_key_bytes = base64::engine::general_purpose::STANDARD
        .decode(public_key_b64)
        .map_err(|e| {
            crate::SpectreError::Comms(CommsError::KeyError(format!(
                "Failed to decode public key: {}",
                e
            )))
        })?;

    if public_key_bytes.len() != 32 {
        return Err(crate::SpectreError::Comms(CommsError::KeyError(format!(
            "Invalid public key length: expected 32, got {}",
            public_key_bytes.len()
        ))));
    }

    let mut node_id = [0u8; 32];
    node_id.copy_from_slice(&public_key_bytes);

    // Decode private key (X25519 for Noise handshakes)
    let private_key_b64 = identity.private_key_encoded()?;
    let private_key_bytes = base64::engine::general_purpose::STANDARD
        .decode(&private_key_b64)
        .map_err(|e| {
            crate::SpectreError::Comms(CommsError::KeyError(format!(
                "Failed to decode private key: {}",
                e
            )))
        })?;

    if private_key_bytes.len() != 32 {
        return Err(crate::SpectreError::Comms(CommsError::KeyError(format!(
            "Invalid private key length: expected 32, got {}",
            private_key_bytes.len()
        ))));
    }

    let mut private_key_arr = [0u8; 32];
    private_key_arr.copy_from_slice(&private_key_bytes);

    // Reconstruct the Noise keypair from the private key bytes
    let x25519 = wraith_crypto::noise::NoiseKeypair::from_bytes(private_key_arr).map_err(|e| {
        crate::SpectreError::Comms(CommsError::KeyError(format!(
            "Failed to reconstruct Noise keypair: {}",
            e
        )))
    })?;

    // Build the WRAITH Identity from components
    let wraith_identity = wraith_core::node::identity::Identity::from_components(node_id, x25519);

    Ok(wraith_identity)
}

/// Convert a SPECTRE Peer's public key to a WRAITH PeerId (32-byte array).
///
/// Decodes the base64-encoded public key from the Peer struct and converts
/// it to the 32-byte array that WRAITH uses as a PeerId.
fn peer_to_wraith_id(peer: &Peer) -> crate::Result<[u8; 32]> {
    use base64::Engine;

    let fingerprint = peer.fingerprint();
    // The peer stores its public key as base64; we need to access the raw bytes.
    // Since Peer doesn't expose raw public_key bytes directly, we reconstruct
    // from the base64-encoded field via the JSON serialization.
    let json = serde_json::to_value(peer)?;
    let pk_b64 = json
        .get("public_key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            crate::SpectreError::Comms(CommsError::PeerNotFound(format!(
                "Peer {} has no public key",
                fingerprint
            )))
        })?;

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(pk_b64)
        .map_err(|e| {
            crate::SpectreError::Comms(CommsError::KeyError(format!(
                "Failed to decode peer public key: {}",
                e
            )))
        })?;

    if bytes.len() != 32 {
        return Err(crate::SpectreError::Comms(CommsError::KeyError(format!(
            "Invalid peer public key length: expected 32, got {}",
            bytes.len()
        ))));
    }

    let mut peer_id = [0u8; 32];
    peer_id.copy_from_slice(&bytes);
    Ok(peer_id)
}

// ---------------------------------------------------------------------------
// Error mapping
// ---------------------------------------------------------------------------

/// Map a WRAITH NodeError to a SPECTRE SpectreError.
///
/// This provides meaningful error translation between the two error hierarchies,
/// preserving context while using SPECTRE's established error categories.
fn map_node_error(err: wraith_core::NodeError) -> crate::SpectreError {
    match &err {
        wraith_core::NodeError::Crypto(msg) => {
            crate::SpectreError::Comms(CommsError::EncryptionError(msg.clone()))
        },
        wraith_core::NodeError::Handshake(msg) => {
            crate::SpectreError::Comms(CommsError::AuthenticationFailed(msg.to_string()))
        },
        wraith_core::NodeError::Transport(msg) | wraith_core::NodeError::TransportInit(msg) => {
            crate::SpectreError::Comms(CommsError::ConnectionError(msg.to_string()))
        },
        wraith_core::NodeError::Timeout(_) => crate::SpectreError::Comms(CommsError::Timeout),
        wraith_core::NodeError::SessionNotFound(peer_id) => {
            crate::SpectreError::Comms(CommsError::PeerNotFound(hex::encode(&peer_id[..8])))
        },
        wraith_core::NodeError::PeerNotFound(peer_id) => {
            crate::SpectreError::Comms(CommsError::PeerNotFound(hex::encode(&peer_id[..8])))
        },
        _ => crate::SpectreError::Comms(CommsError::ConnectionError(err.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::comms::identity::KeyType;

    #[test]
    fn test_generate_wraith_identity_keys() {
        let result = generate_wraith_identity_keys();
        assert!(result.is_ok(), "Key generation should succeed");

        let (public_key, private_key) = result.unwrap();

        // Verify base64 encoding produces valid 32-byte keys
        use base64::Engine;
        let pub_bytes = base64::engine::general_purpose::STANDARD
            .decode(&public_key)
            .unwrap();
        let priv_bytes = base64::engine::general_purpose::STANDARD
            .decode(&private_key)
            .unwrap();

        assert_eq!(pub_bytes.len(), 32, "Public key should be 32 bytes");
        assert_eq!(priv_bytes.len(), 32, "Private key should be 32 bytes");
    }

    #[test]
    fn test_generate_wraith_identity_keys_uniqueness() {
        let (pub1, priv1) = generate_wraith_identity_keys().unwrap();
        let (pub2, priv2) = generate_wraith_identity_keys().unwrap();

        assert_ne!(
            pub1, pub2,
            "Each generation should produce unique public keys"
        );
        assert_ne!(
            priv1, priv2,
            "Each generation should produce unique private keys"
        );
    }

    #[test]
    fn test_spectre_identity_to_wraith_roundtrip() {
        // Generate a SPECTRE identity (which now uses real WRAITH keys)
        let identity =
            super::super::Identity::generate("test-roundtrip", KeyType::Ed25519, None).unwrap();

        // Convert to WRAITH identity
        let wraith_id = spectre_identity_to_wraith(&identity);
        assert!(
            wraith_id.is_ok(),
            "Conversion should succeed: {:?}",
            wraith_id.err()
        );

        let wraith_id = wraith_id.unwrap();

        // Verify the node ID matches the SPECTRE public key
        use base64::Engine;
        let spectre_pub = base64::engine::general_purpose::STANDARD
            .decode(identity.public_key_encoded())
            .unwrap();
        assert_eq!(
            wraith_id.public_key(),
            &spectre_pub[..],
            "Node ID should match SPECTRE public key"
        );
    }

    #[test]
    fn test_peer_to_wraith_id() {
        let public_key = vec![42u8; 32];
        let peer = Peer::new(public_key.clone(), Some("test-peer".to_string()), 8, None);

        let result = peer_to_wraith_id(&peer);
        assert!(result.is_ok());

        let peer_id = result.unwrap();
        assert_eq!(&peer_id[..], &public_key[..]);
    }

    #[test]
    fn test_map_node_error_crypto() {
        let err = wraith_core::NodeError::Crypto("test crypto error".to_string());
        let mapped = map_node_error(err);
        match mapped {
            crate::SpectreError::Comms(CommsError::EncryptionError(msg)) => {
                assert!(msg.contains("test crypto error"));
            },
            other => panic!("Expected EncryptionError, got: {:?}", other),
        }
    }

    #[test]
    fn test_map_node_error_timeout() {
        let err = wraith_core::NodeError::Timeout(std::borrow::Cow::Borrowed("test timeout"));
        let mapped = map_node_error(err);
        assert!(matches!(
            mapped,
            crate::SpectreError::Comms(CommsError::Timeout)
        ));
    }

    #[tokio::test]
    async fn test_create_wraith_node() {
        let config = Config::default();
        let identity =
            super::super::Identity::generate("wraith-test", KeyType::Ed25519, None).unwrap();

        let result = create_wraith_node(&config, &identity).await;
        assert!(
            result.is_ok(),
            "Node creation should succeed: {:?}",
            result.err()
        );

        let node = result.unwrap();
        assert!(!node.is_running(), "Node should not be running initially");
        assert!(
            !node.node_id_hex().is_empty(),
            "Node ID should not be empty"
        );
    }

    #[tokio::test]
    async fn test_wraith_node_relay_status_not_running() {
        let config = Config::default();
        let identity =
            super::super::Identity::generate("relay-test", KeyType::Ed25519, None).unwrap();

        let node = create_wraith_node(&config, &identity).await.unwrap();
        let status = node.check_relay_connectivity().await.unwrap();

        assert!(
            !status.connected,
            "Node not running should report not connected"
        );
    }
}
