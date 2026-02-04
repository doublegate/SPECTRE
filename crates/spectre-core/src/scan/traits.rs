//! Scanner trait definitions

use async_trait::async_trait;

use super::types::{ScanConfig, ScanResult};

/// Progress callback type for scan progress reporting
pub type ProgressCallback = Box<dyn Fn(usize, usize) + Send + Sync>;

/// Scanner trait defining the interface for network scanners
///
/// This trait abstracts the scanning functionality, allowing different
/// implementations (ProRT-IP, nmap wrapper, etc.) to be used interchangeably.
#[async_trait]
pub trait Scanner: Send + Sync {
    /// Perform a scan with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Scan configuration specifying targets, ports, and options
    /// * `progress_callback` - Optional callback called for each target scanned
    ///
    /// # Returns
    ///
    /// Vector of scan results, one per target host.
    async fn scan(
        &self,
        config: ScanConfig,
        progress_callback: Option<ProgressCallback>,
    ) -> crate::Result<Vec<ScanResult>>;

    /// Get the network interface this scanner uses
    fn interface(&self) -> Option<&str>;

    /// Check if this scanner requires root/admin privileges
    fn requires_root(&self) -> bool;
}

#[cfg(test)]
mod tests {
    // The Scanner trait is tested through the StubScanner implementation
    // in the parent module.
}
