# GUI Display Issues Troubleshooting Guide

This guide covers common display server and graphics rendering issues with the SPECTRE GUI application (Tauri 2.10 + WebKitGTK).

## Table of Contents

- [Automatic Detection System](#automatic-detection-system)
- [Common Issues](#common-issues)
  - [Wayland GBM Buffer Errors](#wayland-gbm-buffer-errors)
  - [Blank/White Window](#blankwhite-window)
  - [Hardware Acceleration Disabled](#hardware-acceleration-disabled)
  - [NVIDIA-Specific Issues](#nvidia-specific-issues)
- [Manual Overrides](#manual-overrides)
- [Platform-Specific Notes](#platform-specific-notes)
- [GPU Driver Installation](#gpu-driver-installation)
- [Advanced Debugging](#advanced-debugging)

---

## Automatic Detection System

**As of v0.5.0-alpha.4**, SPECTRE GUI automatically detects your display server (Wayland vs X11) and GPU vendor, then applies appropriate workarounds. **You should not need to set environment variables manually.**

### How It Works

1. **Display Server Detection:**
   - Checks `XDG_SESSION_TYPE` environment variable
   - Falls back to `WAYLAND_DISPLAY` or `DISPLAY` variables
   - Defaults to X11 if unable to detect

2. **GPU Vendor Detection:**
   - Reads PCI vendor ID from `/sys/class/drm/card*/device/vendor` (Linux)
   - Falls back to `lspci` command output
   - Detects NVIDIA, Intel, AMD, or Unknown

3. **Automatic Workarounds:**
   - **NVIDIA + Wayland:** Forces X11 backend (WebKitGTK DMABUF issues)
   - **NVIDIA (any backend):** Disables DMABUF renderer, enables explicit sync workaround
   - **Intel/AMD + X11:** Disables compositing mode for stability
   - **Unknown GPU:** Conservative settings (disable DMABUF)

### Console Output

When launching the GUI, you'll see diagnostic output:

```
🚀 Starting SPECTRE GUI...
🔍 Display Server: X11 (detected: Wayland)
🎨 GPU Vendor: NVIDIA
🔄 Automatic fallback applied for compatibility
🔧 Workarounds: DMABUF renderer disabled, NVIDIA explicit sync disabled
✨ GUI launching...
```

---

## Common Issues

### Wayland GBM Buffer Errors

**Symptoms:**
```
Gdk-Message: Error 71 (Protocol error) dispatching to Wayland display
Failed to create GBM buffer of size 1280x800: Invalid argument
```

**Cause:** WebKitGTK's DMABUF renderer has compatibility issues with certain GPU drivers (especially NVIDIA proprietary drivers) on Wayland.

**Automatic Fix:** The GUI detects this scenario and automatically falls back to X11 backend.

**Manual Override (if needed):**
```bash
GDK_BACKEND=x11 cargo tauri dev
# or
./dev.sh --force-x11
```

**References:**
- [WebKitGTK DMABUF Bug #261874](https://bugs.webkit.org/show_bug.cgi?id=261874)
- [Tauri Issue #13493](https://github.com/tauri-apps/tauri/issues/13493)
- [Ubuntu Bug #2041664](https://bugs.launchpad.net/bugs/2041664)

### Blank/White Window

**Symptoms:**
- Window opens but shows only white/blank content
- No UI elements visible
- No errors in console

**Cause:** DMABUF renderer initialization failure without proper error reporting.

**Automatic Fix:** NVIDIA GPUs automatically have DMABUF renderer disabled.

**Manual Override:**
```bash
WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri dev
```

**Alternative (less aggressive):**
```bash
WEBKIT_DISABLE_COMPOSITING_MODE=1 cargo tauri dev
```

**References:**
- [Tauri Issue #9394](https://github.com/tauri-apps/tauri/issues/9394)
- [Wails Issue #2977](https://github.com/wailsapp/wails/issues/2977)

### Hardware Acceleration Disabled

**Symptoms:**
- Slow rendering performance
- High CPU usage during UI interactions
- Software rendering fallback messages in logs

**Diagnosis:**
```bash
# Check if hardware acceleration is active
glxinfo | grep "direct rendering"
# Should show: direct rendering: Yes

# Check GPU usage while running GUI
nvidia-smi  # for NVIDIA
intel_gpu_top  # for Intel
radeontop  # for AMD
```

**Cause:** GPU driver issues, missing drivers, or incompatible Mesa version.

**Solutions:**

1. **Update GPU Drivers** (see [GPU Driver Installation](#gpu-driver-installation))

2. **Check Mesa Version:**
   ```bash
   glxinfo | grep "OpenGL version"
   # Recommended: Mesa 23.0+ for best WebKitGTK compatibility
   ```

3. **Try Different Backend:**
   ```bash
   # If Wayland fails, try X11
   ./dev.sh --force-x11

   # If X11 fails, try Wayland
   ./dev.sh --force-wayland
   ```

### NVIDIA-Specific Issues

**Common Problems:**
- White screen on Wayland
- GBM buffer creation failures
- Flickering or screen corruption

**Automatic Workarounds Applied:**
```bash
GDK_BACKEND=x11  # Force X11 instead of Wayland
WEBKIT_DISABLE_DMABUF_RENDERER=1  # Disable problematic renderer
__NV_DISABLE_EXPLICIT_SYNC=1  # NVIDIA driver workaround
```

**If Issues Persist:**

1. **Verify NVIDIA Driver Installation:**
   ```bash
   nvidia-smi  # Should show driver version and GPU info
   ```

2. **Install NVIDIA EGL GBM Library** (if available):
   ```bash
   # Debian/Ubuntu
   sudo apt install libnvidia-egl-gbm1

   # Arch Linux
   sudo pacman -S nvidia-utils

   # Fedora
   sudo dnf install nvidia-driver-cuda
   ```

3. **Check for Driver Conflicts:**
   ```bash
   # Ensure nouveau (open-source driver) is disabled
   lsmod | grep nouveau
   # Should return nothing
   ```

**References:**
- [NVIDIA Forum: WebKit/Tauri White Screen](https://forums.developer.nvidia.com/t/webkit-tauri-application-white-screen-on-dgx-spark-gpupermission-issues/348741)
- [Tauri NVIDIA Documentation Issue #9394](https://github.com/tauri-apps/tauri/issues/9394)

---

## Manual Overrides

If automatic detection fails or you want to force specific settings:

### Force Display Backend

```bash
# Force X11 (safest, most compatible)
GDK_BACKEND=x11 cargo tauri dev

# Force Wayland (native, potentially faster)
GDK_BACKEND=wayland cargo tauri dev

# Using dev.sh helper script
./dev.sh --force-x11
./dev.sh --force-wayland
```

### Disable Rendering Features

```bash
# Disable DMABUF renderer (fixes most NVIDIA issues)
WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri dev

# Disable compositing mode (alternative workaround)
WEBKIT_DISABLE_COMPOSITING_MODE=1 cargo tauri dev

# Disable both (most conservative)
WEBKIT_DISABLE_DMABUF_RENDERER=1 WEBKIT_DISABLE_COMPOSITING_MODE=1 cargo tauri dev
```

### NVIDIA-Specific Overrides

```bash
# Standard NVIDIA workaround
__NV_DISABLE_EXPLICIT_SYNC=1 cargo tauri dev

# Full NVIDIA compatibility mode
GDK_BACKEND=x11 \
WEBKIT_DISABLE_DMABUF_RENDERER=1 \
__NV_DISABLE_EXPLICIT_SYNC=1 \
cargo tauri dev
```

### Enable Debug Logging

```bash
# Debug level
RUST_LOG=debug cargo tauri dev

# Trace level (very verbose)
RUST_LOG=trace cargo tauri dev

# Using dev.sh
./dev.sh --verbose  # sets RUST_LOG=debug
./dev.sh --trace    # sets RUST_LOG=trace
```

### Persistent Configuration

Create a `.env` file in `crates/spectre-gui/`:

```bash
# .env
GDK_BACKEND=x11
WEBKIT_DISABLE_DMABUF_RENDERER=1
RUST_LOG=info
```

**Note:** Automatic detection respects user-provided environment variables and won't override them.

---

## Platform-Specific Notes

### Linux

**Supported Configurations:**
- Wayland compositors: Sway, Hyprland, GNOME Wayland, KDE Plasma Wayland
- X11 window managers: i3, bspwm, Openbox, XFCE, GNOME X11, KDE Plasma X11
- XWayland: Supported (X11 apps on Wayland)

**Known Issues:**
- **NVIDIA + Wayland:** Requires X11 fallback (automatic)
- **Old Mesa (<21.0):** May have DMABUF issues, upgrade recommended
- **Hybrid Graphics:** May need `prime-run` or similar GPU switching

**Hybrid Graphics (NVIDIA Optimus):**
```bash
# Run on NVIDIA dGPU
prime-run cargo tauri dev

# Run on Intel iGPU (default)
cargo tauri dev
```

### macOS

**No configuration needed.** macOS doesn't use Wayland/X11, so all display detection is skipped.

**Requirements:**
- macOS 10.15+ (Catalina or later)
- Metal-compatible GPU

### Windows

**No configuration needed.** Windows doesn't use Wayland/X11, so all display detection is skipped.

**Requirements:**
- Windows 10 (1809+) or Windows 11
- DirectX 11+ compatible GPU

---

## GPU Driver Installation

### NVIDIA

**Proprietary Driver (Recommended):**
```bash
# Debian/Ubuntu
sudo apt install nvidia-driver-535  # or latest version

# Arch Linux
sudo pacman -S nvidia nvidia-utils

# Fedora
sudo dnf install akmod-nvidia

# openSUSE
sudo zypper install nvidia-driver-G06
```

**Verify Installation:**
```bash
nvidia-smi
# Should show driver version and GPU info
```

**Nouveau (Open Source - Not Recommended for GUI):**
If you're using nouveau, consider switching to proprietary drivers for better WebKitGTK compatibility.

### Intel

**Mesa Drivers (Usually Pre-installed):**
```bash
# Debian/Ubuntu
sudo apt install mesa-utils mesa-vulkan-drivers intel-media-va-driver

# Arch Linux
sudo pacman -S mesa vulkan-intel intel-media-driver

# Fedora
sudo dnf install mesa-dri-drivers mesa-vulkan-drivers intel-media-driver
```

**Verify Installation:**
```bash
glxinfo | grep "OpenGL renderer"
# Should show Intel GPU name
```

### AMD

**Mesa Drivers (Open Source - Recommended):**
```bash
# Debian/Ubuntu
sudo apt install mesa-utils mesa-vulkan-drivers mesa-vdpau-drivers

# Arch Linux
sudo pacman -S mesa vulkan-radeon libva-mesa-driver

# Fedora
sudo dnf install mesa-dri-drivers mesa-vulkan-drivers mesa-vdpau-drivers
```

**AMDGPU-PRO (Proprietary - Optional):**
Download from [AMD's website](https://www.amd.com/en/support/linux-drivers) if needed for professional workloads.

**Verify Installation:**
```bash
glxinfo | grep "OpenGL renderer"
# Should show AMD GPU name
```

### Verify WebKitGTK Installation

```bash
# Check WebKitGTK version
pkg-config --modversion webkit2gtk-4.1
# Recommended: 2.42.0+ (has DMABUF support)

# Install/upgrade if needed
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev

# Arch Linux
sudo pacman -S webkit2gtk-4.1

# Fedora
sudo dnf install webkit2gtk4.1-devel
```

---

## Advanced Debugging

### Check Display Server

```bash
# Current session type
echo $XDG_SESSION_TYPE
# Output: wayland or x11

# Check available displays
echo $WAYLAND_DISPLAY  # e.g., wayland-0
echo $DISPLAY          # e.g., :0

# List running compositors/window managers
ps aux | grep -E 'sway|hyprland|mutter|kwin|xorg'
```

### Check GPU Information

```bash
# Detailed GPU info
lspci | grep -i vga

# OpenGL information
glxinfo | head -20

# Vulkan information
vulkaninfo | head -20

# PCI vendor ID (used by auto-detection)
cat /sys/class/drm/card0/device/vendor
# 0x10de = NVIDIA
# 0x8086 = Intel
# 0x1002 = AMD
```

### Monitor GPU Usage

```bash
# NVIDIA
nvidia-smi -l 1  # update every second

# Intel
sudo intel_gpu_top

# AMD
radeontop
```

### Check WebKitGTK Processes

```bash
# While GUI is running
ps aux | grep WebKit
# Should show WebKitWebProcess and WebKitNetworkProcess

# Check renderer process arguments
ps aux | grep WebKitWebProcess
# Look for --use-angle or other rendering flags
```

### Capture Debug Logs

```bash
# Full debug output
RUST_LOG=trace \
WEBKIT_DEBUG=all \
GTK_DEBUG=all \
GDK_DEBUG=all \
cargo tauri dev 2>&1 | tee debug.log

# Analyze log for errors
grep -i error debug.log
grep -i warning debug.log
grep -i gbm debug.log
```

### Test Rendering Backends

```bash
# Test software rendering (very slow, but always works)
LIBGL_ALWAYS_SOFTWARE=1 cargo tauri dev

# Test with different Mesa drivers (if multiple installed)
MESA_LOADER_DRIVER_OVERRIDE=iris cargo tauri dev  # Intel
MESA_LOADER_DRIVER_OVERRIDE=radeonsi cargo tauri dev  # AMD
```

### Strace for System Call Analysis

```bash
# Trace display server connections
strace -e trace=connect cargo tauri dev 2>&1 | grep -E 'wayland|x11'

# Trace file access (find missing libraries)
strace -e trace=open,openat cargo tauri dev 2>&1 | grep -E '\.so|gbm|dri'
```

---

## Getting Help

If automatic detection doesn't solve your issue:

1. **Check Logs:**
   ```bash
   ./dev.sh --verbose
   ```

2. **Try Manual Overrides:**
   See [Manual Overrides](#manual-overrides) section

3. **Report Issue:**
   - Include output of `./dev.sh --verbose`
   - Include GPU info: `lspci | grep VGA`
   - Include display server: `echo $XDG_SESSION_TYPE`
   - Include WebKitGTK version: `pkg-config --modversion webkit2gtk-4.1`
   - Include Mesa version: `glxinfo | grep "OpenGL version"`

4. **Workaround While Waiting for Fix:**
   ```bash
   # Most compatible settings
   GDK_BACKEND=x11 \
   WEBKIT_DISABLE_DMABUF_RENDERER=1 \
   cargo tauri dev
   ```

---

## External Resources

### WebKitGTK
- [WebKitGTK Accelerated Rendering Blog](https://blogs.igalia.com/carlosgc/2023/04/03/webkitgtk-accelerated-compositing-rendering/)
- [WebKitGTK Bug Tracker](https://bugs.webkit.org/)
- [Phoronix: WebKitGTK DMA-BUF Support](https://www.phoronix.com/news/WebKitGTK-DMA-BUF-Rendering)

### Tauri
- [Tauri Linux Troubleshooting](https://tauri.app/v2/guides/troubleshooting/linux/)
- [Tauri Issue #13493 (GBM Buffer)](https://github.com/tauri-apps/tauri/issues/13493)
- [Tauri Issue #9394 (NVIDIA Docs)](https://github.com/tauri-apps/tauri/issues/9394)
- [Tauri Issue #8254 (Empty Window)](https://github.com/tauri-apps/tauri/issues/8254)

### Linux Graphics Stack
- [Mesa3D Documentation](https://docs.mesa3d.org/)
- [Wayland Documentation](https://wayland.freedesktop.org/)
- [Arch Wiki: NVIDIA](https://wiki.archlinux.org/title/NVIDIA)
- [Arch Wiki: Intel Graphics](https://wiki.archlinux.org/title/Intel_graphics)
- [Arch Wiki: AMDGPU](https://wiki.archlinux.org/title/AMDGPU)

---

**Last Updated:** 2026-02-06
**SPECTRE Version:** 0.5.0-alpha.4+
