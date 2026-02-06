use serde::{Deserialize, Serialize};
use tauri::command;

/// Identity information.
#[derive(Debug, Serialize)]
pub struct IdentityInfo {
    pub name: String,
    pub fingerprint: String,
    pub created: String,
}

/// Peer information.
#[derive(Debug, Serialize)]
pub struct PeerInfo {
    pub name: String,
    pub fingerprint: String,
    pub status: String,
    pub last_seen: Option<String>,
}

/// Send request from the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendRequest {
    pub peer: String,
    pub data: String,
}

/// Get the current identity. Stub - will be wired in Sprint 5.6.
#[command]
pub async fn get_identity() -> Result<Option<IdentityInfo>, String> {
    Ok(None)
}

/// List trusted peers. Stub.
#[command]
pub async fn list_peers() -> Result<Vec<PeerInfo>, String> {
    Ok(vec![])
}

/// Send data to a peer. Stub.
#[command]
pub async fn send_data(request: SendRequest) -> Result<String, String> {
    tracing::info!(peer = %request.peer, "Send data (stub)");
    Ok(format!("Queued send to {}", request.peer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_identity_stub() {
        let result = get_identity().await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_list_peers_stub() {
        let result = list_peers().await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_send_data_stub() {
        let request = SendRequest {
            peer: "alpha".to_string(),
            data: "hello".to_string(),
        };
        let result = send_data(request).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("alpha"));
    }
}
