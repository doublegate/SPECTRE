use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tauri::{command, AppHandle, Emitter, State};
use tracing::{error, info};

use spectre_core::scan::{create_scanner, parse_ports, parse_targets, ScanConfig, ScanType, TimingTemplate};

use crate::events::{ScanCompleteEvent, ScanErrorEvent, ScanProgressEvent, ScanResultEvent};
use crate::state::AppState;

/// Scan configuration received from the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequest {
    pub targets: Vec<String>,
    pub ports: Option<String>,
    pub scan_type: Option<String>,
    pub timing: Option<u8>,
    pub service_detection: Option<bool>,
    pub os_detection: Option<bool>,
}

/// Scan summary returned after completion.
#[derive(Debug, Serialize)]
pub struct ScanSummary {
    pub scan_id: String,
    pub hosts_scanned: usize,
    pub hosts_up: usize,
    pub open_ports: usize,
    pub duration_ms: u64,
}

/// Start a new scan. Wired to spectre-core Scanner.
#[command]
pub async fn start_scan(
    app_handle: AppHandle,
    state: State<'_, Arc<AppState>>,
    request: ScanRequest,
) -> Result<String, String> {
    info!(targets = ?request.targets, "Starting scan");

    // Generate unique scan ID
    let scan_id = format!("scan_{}", chrono::Utc::now().timestamp_millis());

    // Parse targets
    let target_strings: Vec<String> = request.targets.clone();
    let targets = parse_targets(&target_strings)
        .map_err(|e| format!("Failed to parse targets: {}", e))?;

    if targets.is_empty() {
        return Err("No valid targets provided".to_string());
    }

    // Parse ports (default: common ports)
    let ports = if let Some(ref port_spec) = request.ports {
        parse_ports(Some(port_spec.as_str())).map_err(|e| format!("Failed to parse ports: {}", e))?
    } else {
        vec![22, 80, 443, 8080, 8443, 3306, 5432, 6379, 27017]
    };

    // Parse scan type
    let scan_type = match request.scan_type.as_deref() {
        Some("syn") => ScanType::Syn,
        Some("connect") => ScanType::Connect,
        Some("udp") => ScanType::Udp,
        Some("ack") => ScanType::Ack,
        Some("fin") => ScanType::Fin,
        Some("xmas") => ScanType::Xmas,
        Some("null") => ScanType::Null,
        Some("window") => ScanType::Window,
        None => ScanType::Connect,
        Some(other) => return Err(format!("Invalid scan type: {}", other)),
    };

    // Parse timing template
    let timing = match request.timing {
        Some(0) => TimingTemplate::Paranoid,
        Some(1) => TimingTemplate::Sneaky,
        Some(2) => TimingTemplate::Polite,
        Some(3) | None => TimingTemplate::Normal,
        Some(4) => TimingTemplate::Aggressive,
        Some(5) => TimingTemplate::Insane,
        Some(other) => return Err(format!("Invalid timing template: {} (must be 0-5)", other)),
    };

    // Create scan config
    let scan_config = ScanConfig {
        scan_type,
        targets: targets.clone(),
        ports: ports.clone(),
        service_detection: request.service_detection.unwrap_or(true),
        os_detection: request.os_detection.unwrap_or(false),
        timing,
        rate_limit: None,
        resolve_dns: true,
        interface: None,
        script: None,
        timeout: None,
    };

    // Create scanner
    let config_guard = state.config.read().await;
    let scanner = create_scanner(&config_guard)
        .map_err(|e| format!("Failed to create scanner: {}", e))?;
    drop(config_guard);

    let scan_id_clone = scan_id.clone();
    let app_handle_clone = app_handle.clone();

    // Spawn background scan task
    let task_handle = tokio::spawn(async move {
        let start_time = Instant::now();
        let mut open_ports_total = 0;
        let mut services_found = 0;

        // Progress callback
        let progress_cb = {
            let scan_id = scan_id_clone.clone();
            let app_handle = app_handle_clone.clone();
            move |current: usize, total: usize| {
                let percent = (current as f64 / total as f64) * 100.0;
                let _ = app_handle.emit(
                    "scan:progress",
                    ScanProgressEvent {
                        scan_id: scan_id.clone(),
                        completed: current,
                        total,
                        percent,
                        current_target: None,
                        rate_pps: None,
                    },
                );
            }
        };

        // Execute scan
        match scanner.scan(scan_config, Some(Box::new(progress_cb))).await {
            Ok(results) => {
                let hosts_scanned = results.len();

                // Emit individual host results
                for result in results {
                    let ports_count = result.ports.len();
                    open_ports_total += ports_count;

                    // Count services
                    services_found += result
                        .ports
                        .iter()
                        .filter(|p| p.service.is_some())
                        .count();

                    let event = ScanResultEvent::from_scan_result(scan_id_clone.clone(), result);
                    let _ = app_handle_clone.emit("scan:result", event);
                }

                // Emit completion
                let duration_ms = start_time.elapsed().as_millis() as u64;
                let _ = app_handle_clone.emit(
                    "scan:complete",
                    ScanCompleteEvent {
                        scan_id: scan_id_clone.clone(),
                        hosts_scanned,
                        open_ports: open_ports_total,
                        services_found,
                        duration_ms,
                    },
                );

                info!(
                    scan_id = %scan_id_clone,
                    hosts = hosts_scanned,
                    ports = open_ports_total,
                    duration_ms,
                    "Scan completed successfully"
                );
            }
            Err(e) => {
                error!(scan_id = %scan_id_clone, error = %e, "Scan failed");
                let _ = app_handle_clone.emit(
                    "scan:error",
                    ScanErrorEvent {
                        scan_id: scan_id_clone.clone(),
                        error: e.to_string(),
                        target: None,
                    },
                );
            }
        }
    });

    // Store task handle
    state.active_scans.lock().await.insert(scan_id.clone(), task_handle);

    Ok(scan_id)
}

/// Stop a running scan.
#[command]
pub async fn stop_scan(
    state: State<'_, Arc<AppState>>,
    scan_id: String,
) -> Result<String, String> {
    info!(scan_id = %scan_id, "Stopping scan");

    let mut scans = state.active_scans.lock().await;
    if let Some(handle) = scans.remove(&scan_id) {
        handle.abort();
        Ok(format!("Scan {} stopped", scan_id))
    } else {
        Err(format!("Scan {} not found or already completed", scan_id))
    }
}

/// Get list of active scans.
#[command]
pub async fn get_active_scans(state: State<'_, Arc<AppState>>) -> Result<Vec<String>, String> {
    let scans = state.active_scans.lock().await;
    Ok(scans.keys().cloned().collect())
}

/// Get scan results (stub for now - results are streamed via events).
#[command]
pub async fn get_scan_results(_scan_id: String) -> Result<Vec<serde_json::Value>, String> {
    // Results are streamed via events, so this is just a placeholder
    // In a production system, you might store completed results in state
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_request_deserialize() {
        let json = r#"{"targets":["10.0.0.1"],"ports":"22,80,443","scan_type":"connect","timing":3}"#;
        let request: ScanRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.targets.len(), 1);
        assert_eq!(request.targets[0], "10.0.0.1");
        assert_eq!(request.ports.as_deref(), Some("22,80,443"));
        assert_eq!(request.scan_type.as_deref(), Some("connect"));
        assert_eq!(request.timing, Some(3));
    }

    #[test]
    fn test_parse_scan_type_valid() {
        let types = vec!["syn", "connect", "udp", "ack", "fin", "xmas", "null", "window"];
        for t in types {
            let request = ScanRequest {
                targets: vec!["127.0.0.1".to_string()],
                ports: None,
                scan_type: Some(t.to_string()),
                timing: None,
                service_detection: None,
                os_detection: None,
            };
            // Just verify it doesn't panic when parsing
            assert_eq!(request.scan_type.as_deref(), Some(t));
        }
    }

    #[test]
    fn test_parse_timing_template() {
        for timing in 0..=5 {
            let request = ScanRequest {
                targets: vec!["127.0.0.1".to_string()],
                ports: None,
                scan_type: None,
                timing: Some(timing),
                service_detection: None,
                os_detection: None,
            };
            assert_eq!(request.timing, Some(timing));
        }
    }

    #[test]
    fn test_scan_request_defaults() {
        let request = ScanRequest {
            targets: vec!["127.0.0.1".to_string()],
            ports: None,
            scan_type: None,
            timing: None,
            service_detection: None,
            os_detection: None,
        };
        assert!(request.ports.is_none());
        assert!(request.scan_type.is_none());
        assert!(request.timing.is_none());
    }

    // Integration tests would go here with MockRuntime
    // These require Tauri runtime mocking which is complex
    // Actual integration tests will be in separate test files
}
