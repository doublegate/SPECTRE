//! Display Server Detection and Configuration
//!
//! Automatically detects the display server (Wayland vs X11) and configures
//! the appropriate environment variables for optimal WebKitGTK rendering.
//!
//! This module handles the following scenarios:
//! - Wayland session with hardware acceleration (attempts first)
//! - X11 fallback if Wayland fails or has GBM issues
//! - NVIDIA-specific workarounds for DMABUF renderer issues
//! - Intel/AMD GPU optimization
//!
//! ## Environment Variables Configured
//!
//! - `GDK_BACKEND`: Forces GTK to use specific display server (wayland/x11)
//! - `WEBKIT_DISABLE_DMABUF_RENDERER`: Disables problematic DMABUF renderer
//! - `WEBKIT_DISABLE_COMPOSITING_MODE`: Alternative rendering path
//! - `__NV_DISABLE_EXPLICIT_SYNC`: NVIDIA-specific workaround
//!
//! ## References
//!
//! - WebKitGTK DMABUF issues: https://bugs.webkit.org/show_bug.cgi?id=261874
//! - Tauri Wayland support: https://github.com/tauri-apps/tauri/issues/13493
//! - NVIDIA workarounds: https://github.com/tauri-apps/tauri/issues/9394

use std::env;
use std::fs;
use tracing::{debug, info, warn};

/// Display server types detected from environment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayServer {
    /// Wayland compositor detected
    Wayland,
    /// X11 server detected (including XWayland)
    X11,
    /// Unable to determine display server
    Unknown,
}

impl DisplayServer {
    /// Get human-readable name
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Wayland => "Wayland",
            Self::X11 => "X11",
            Self::Unknown => "Unknown",
        }
    }
}

/// GPU vendor types for hardware-specific workarounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVendor {
    /// NVIDIA GPUs (proprietary driver issues with DMABUF)
    Nvidia,
    /// Intel integrated graphics
    Intel,
    /// AMD GPUs (including integrated)
    Amd,
    /// Unknown or unsupported GPU
    Unknown,
}

impl GpuVendor {
    /// Get human-readable name
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Nvidia => "NVIDIA",
            Self::Intel => "Intel",
            Self::Amd => "AMD",
            Self::Unknown => "Unknown",
        }
    }
}

/// Display configuration result
#[derive(Debug)]
pub struct DisplayConfig {
    /// Detected display server
    pub display_server: DisplayServer,
    /// Detected GPU vendor
    pub gpu_vendor: GpuVendor,
    /// Whether user has manually overridden GDK_BACKEND
    pub user_override: bool,
    /// Backend that will be used (after fallback logic)
    pub backend: DisplayServer,
}

/// Detect the active display server from environment variables.
///
/// Checks in order:
/// 1. `XDG_SESSION_TYPE` (most reliable)
/// 2. `WAYLAND_DISPLAY` (Wayland-specific)
/// 3. `DISPLAY` (X11-specific)
pub fn detect_display_server() -> DisplayServer {
    // Check XDG_SESSION_TYPE first (most reliable)
    if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
        match session_type.to_lowercase().as_str() {
            "wayland" => {
                debug!("Detected Wayland from XDG_SESSION_TYPE");
                return DisplayServer::Wayland;
            },
            "x11" => {
                debug!("Detected X11 from XDG_SESSION_TYPE");
                return DisplayServer::X11;
            },
            other => {
                debug!("Unknown XDG_SESSION_TYPE: {}", other);
            },
        }
    }

    // Check Wayland display
    if env::var("WAYLAND_DISPLAY").is_ok() {
        debug!("Detected Wayland from WAYLAND_DISPLAY");
        return DisplayServer::Wayland;
    }

    // Check X11 display
    if env::var("DISPLAY").is_ok() {
        debug!("Detected X11 from DISPLAY");
        return DisplayServer::X11;
    }

    warn!("Unable to detect display server from environment");
    DisplayServer::Unknown
}

/// Detect GPU vendor from system information.
///
/// On Linux, reads from `/sys/class/drm/card*/device/vendor` and lspci.
/// Falls back to Unknown if detection fails.
pub fn detect_gpu_vendor() -> GpuVendor {
    // Try reading PCI vendor ID from sysfs
    if let Some(vendor) = read_pci_vendor() {
        match vendor.as_str() {
            "0x10de" => {
                debug!("Detected NVIDIA GPU from PCI vendor ID");
                return GpuVendor::Nvidia;
            },
            "0x8086" => {
                debug!("Detected Intel GPU from PCI vendor ID");
                return GpuVendor::Intel;
            },
            "0x1002" => {
                debug!("Detected AMD GPU from PCI vendor ID");
                return GpuVendor::Amd;
            },
            other => {
                debug!("Unknown PCI vendor ID: {}", other);
            },
        }
    }

    // Fallback: try to detect from glxinfo or vulkaninfo output
    // This is expensive so we only do it if sysfs failed
    if let Some(vendor) = detect_gpu_from_commands() {
        return vendor;
    }

    warn!("Unable to detect GPU vendor");
    GpuVendor::Unknown
}

/// Read PCI vendor ID from sysfs (Linux only)
#[cfg(target_os = "linux")]
fn read_pci_vendor() -> Option<String> {
    // Try to find the primary GPU card
    for card_num in 0..4 {
        let vendor_path = format!("/sys/class/drm/card{}/device/vendor", card_num);
        if let Ok(vendor) = fs::read_to_string(&vendor_path) {
            return Some(vendor.trim().to_string());
        }
    }
    None
}

#[cfg(not(target_os = "linux"))]
fn read_pci_vendor() -> Option<String> {
    None
}

/// Detect GPU vendor from command output (fallback method)
fn detect_gpu_from_commands() -> Option<GpuVendor> {
    use std::process::Command;

    // Try lspci first
    if let Ok(output) = Command::new("lspci").output() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            let lower = stdout.to_lowercase();
            if lower.contains("nvidia") {
                debug!("Detected NVIDIA GPU from lspci");
                return Some(GpuVendor::Nvidia);
            }
            if lower.contains("intel") && lower.contains("vga") {
                debug!("Detected Intel GPU from lspci");
                return Some(GpuVendor::Intel);
            }
            if lower.contains("amd") || lower.contains("radeon") {
                debug!("Detected AMD GPU from lspci");
                return Some(GpuVendor::Amd);
            }
        }
    }

    None
}

/// Configure display server and apply workarounds.
///
/// This is the main entry point called from `main.rs`. It:
/// 1. Detects display server and GPU vendor
/// 2. Checks for user overrides
/// 3. Applies appropriate workarounds
/// 4. Sets environment variables
///
/// Returns configuration details for logging.
pub fn configure_display_server() -> DisplayConfig {
    let display_server = detect_display_server();
    let gpu_vendor = detect_gpu_vendor();

    // Check if user has manually set GDK_BACKEND (respect their choice)
    let user_override = env::var("GDK_BACKEND").is_ok();

    if user_override {
        info!("GDK_BACKEND already set by user, respecting override");
        let backend = env::var("GDK_BACKEND")
            .ok()
            .and_then(|s| match s.as_str() {
                "wayland" => Some(DisplayServer::Wayland),
                "x11" => Some(DisplayServer::X11),
                _ => None,
            })
            .unwrap_or(display_server);

        // Still apply GPU-specific workarounds even with user override
        apply_gpu_workarounds(gpu_vendor, backend);

        return DisplayConfig {
            display_server,
            gpu_vendor,
            user_override: true,
            backend,
        };
    }

    // Automatic configuration based on detection
    let backend = determine_backend(display_server, gpu_vendor);

    // Set GDK_BACKEND if needed
    if backend != display_server {
        info!("Setting GDK_BACKEND={}", backend.as_str().to_lowercase());
        env::set_var("GDK_BACKEND", backend.as_str().to_lowercase());
    }

    // Apply GPU-specific workarounds
    apply_gpu_workarounds(gpu_vendor, backend);

    DisplayConfig {
        display_server,
        gpu_vendor,
        user_override: false,
        backend,
    }
}

/// Determine the best backend to use based on display server and GPU.
///
/// NVIDIA GPUs have known issues with Wayland + DMABUF renderer,
/// so we automatically fall back to X11 for better stability.
fn determine_backend(display_server: DisplayServer, gpu_vendor: GpuVendor) -> DisplayServer {
    match (display_server, gpu_vendor) {
        // NVIDIA + Wayland = known issues, force X11
        (DisplayServer::Wayland, GpuVendor::Nvidia) => {
            warn!("NVIDIA GPU detected on Wayland - forcing X11 backend for stability");
            warn!("WebKitGTK has known issues with NVIDIA + Wayland DMABUF renderer");
            warn!("See: https://bugs.webkit.org/show_bug.cgi?id=261874");
            DisplayServer::X11
        },

        // Unknown display server, try X11 as safest fallback
        (DisplayServer::Unknown, _) => {
            warn!("Unable to detect display server, defaulting to X11");
            DisplayServer::X11
        },

        // All other combinations: use detected display server
        _ => display_server,
    }
}

/// Apply GPU vendor-specific workarounds.
///
/// NVIDIA GPUs require disabling the DMABUF renderer and enabling
/// explicit sync workaround. Intel/AMD GPUs work better with default settings.
fn apply_gpu_workarounds(gpu_vendor: GpuVendor, backend: DisplayServer) {
    match gpu_vendor {
        GpuVendor::Nvidia => {
            // NVIDIA-specific workarounds
            info!("Applying NVIDIA GPU workarounds...");

            // Disable DMABUF renderer (causes GBM buffer creation failures)
            // See: https://github.com/tauri-apps/tauri/issues/13493
            if env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_err() {
                env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
                debug!("Set WEBKIT_DISABLE_DMABUF_RENDERER=1");
            }

            // Disable explicit sync (NVIDIA-specific issue)
            // See: https://github.com/tauri-apps/tauri/issues/9394
            if env::var("__NV_DISABLE_EXPLICIT_SYNC").is_err() {
                env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");
                debug!("Set __NV_DISABLE_EXPLICIT_SYNC=1");
            }

            // If using X11 backend, ensure we're not trying to use Wayland protocols
            if backend == DisplayServer::X11 && env::var("GDK_BACKEND").is_err() {
                env::set_var("GDK_BACKEND", "x11");
                debug!("Explicitly set GDK_BACKEND=x11 for NVIDIA");
            }
        },

        GpuVendor::Intel | GpuVendor::Amd => {
            // Intel/AMD generally work well with default settings
            // Only disable DMABUF if we're on X11 (Wayland should work)
            if backend == DisplayServer::X11 {
                // Try to use hardware acceleration, but disable compositing mode
                // as a safer alternative to DMABUF
                if env::var("WEBKIT_DISABLE_COMPOSITING_MODE").is_err() {
                    env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
                    debug!("Set WEBKIT_DISABLE_COMPOSITING_MODE=1 for X11 backend");
                }
            }
        },

        GpuVendor::Unknown => {
            // Conservative approach for unknown GPUs
            warn!("Unknown GPU vendor - applying conservative workarounds");
            if env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_err() {
                env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
                debug!("Set WEBKIT_DISABLE_DMABUF_RENDERER=1 (conservative)");
            }
        },
    }
}

/// Print display configuration to console.
///
/// Called from main.rs to inform user about detected configuration.
pub fn print_display_config(config: &DisplayConfig) {
    println!("🚀 Starting SPECTRE GUI...");
    println!(
        "🔍 Display Server: {} (detected: {})",
        config.backend.as_str(),
        config.display_server.as_str()
    );
    println!("🎨 GPU Vendor: {}", config.gpu_vendor.as_str());

    if config.user_override {
        println!(
            "⚙️  User override: GDK_BACKEND={}",
            env::var("GDK_BACKEND").unwrap_or_default()
        );
    }

    if config.backend != config.display_server {
        println!("🔄 Automatic fallback applied for compatibility");
    }

    // Print active workarounds
    let mut workarounds = Vec::new();
    if env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_ok() {
        workarounds.push("DMABUF renderer disabled");
    }
    if env::var("WEBKIT_DISABLE_COMPOSITING_MODE").is_ok() {
        workarounds.push("Compositing mode disabled");
    }
    if env::var("__NV_DISABLE_EXPLICIT_SYNC").is_ok() {
        workarounds.push("NVIDIA explicit sync disabled");
    }

    if !workarounds.is_empty() {
        println!("🔧 Workarounds: {}", workarounds.join(", "));
    }

    println!("✨ GUI launching...");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_server_as_str() {
        assert_eq!(DisplayServer::Wayland.as_str(), "Wayland");
        assert_eq!(DisplayServer::X11.as_str(), "X11");
        assert_eq!(DisplayServer::Unknown.as_str(), "Unknown");
    }

    #[test]
    fn test_gpu_vendor_as_str() {
        assert_eq!(GpuVendor::Nvidia.as_str(), "NVIDIA");
        assert_eq!(GpuVendor::Intel.as_str(), "Intel");
        assert_eq!(GpuVendor::Amd.as_str(), "AMD");
        assert_eq!(GpuVendor::Unknown.as_str(), "Unknown");
    }

    #[test]
    fn test_detect_display_server_unknown() {
        // Clear environment variables
        env::remove_var("XDG_SESSION_TYPE");
        env::remove_var("WAYLAND_DISPLAY");
        env::remove_var("DISPLAY");

        let result = detect_display_server();
        assert_eq!(result, DisplayServer::Unknown);
    }

    #[test]
    fn test_determine_backend_nvidia_wayland() {
        let backend = determine_backend(DisplayServer::Wayland, GpuVendor::Nvidia);
        // NVIDIA + Wayland should force X11
        assert_eq!(backend, DisplayServer::X11);
    }

    #[test]
    fn test_determine_backend_intel_wayland() {
        let backend = determine_backend(DisplayServer::Wayland, GpuVendor::Intel);
        // Intel + Wayland should stay on Wayland
        assert_eq!(backend, DisplayServer::Wayland);
    }

    #[test]
    fn test_determine_backend_unknown_display() {
        let backend = determine_backend(DisplayServer::Unknown, GpuVendor::Intel);
        // Unknown display should default to X11
        assert_eq!(backend, DisplayServer::X11);
    }
}
