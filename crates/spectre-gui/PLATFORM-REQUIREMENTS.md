# Platform Requirements for SPECTRE GUI

This document outlines the system requirements and dependencies needed to build and run the SPECTRE GUI application on different platforms.

## Linux

### System Requirements
- **Minimum OS:** Ubuntu 22.04, Fedora 36, Arch Linux (current), or equivalent
- **Kernel:** Linux 6.2+ (recommended for optimal ProRT-IP performance)
- **Architecture:** x86_64 (AMD64)

### System Dependencies

The SPECTRE GUI requires the following system libraries:

- **webkit2gtk-4.1** or later - WebView rendering engine (Tauri 2.0 requirement)
- **GTK 3.24** or later - GUI toolkit
- **libayatana-appindicator3** - System tray support
- **librsvg2** - SVG rendering
- **libpcap** - Packet capture (for ProRT-IP network scanning)

### Installation Commands

#### Debian/Ubuntu
```bash
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libpcap-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev
```

#### Fedora/RHEL/CentOS
```bash
sudo dnf install -y \
    webkit2gtk4.1-devel \
    gtk3-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    libpcap-devel \
    gcc \
    gcc-c++ \
    make \
    openssl-devel
```

#### Arch Linux
```bash
sudo pacman -S --needed \
    webkit2gtk-4.1 \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    libpcap \
    base-devel \
    openssl
```

### Testing Your Installation

Run the provided test script:
```bash
cd crates/spectre-gui
./scripts/test-linux.sh
```

---

## macOS

### System Requirements
- **Minimum OS:** macOS 10.15 (Catalina) or later
- **Recommended:** macOS 12 (Monterey) or later
- **Architecture:** Intel (x86_64) or Apple Silicon (ARM64)

### System Dependencies

- **Xcode Command Line Tools** - Required for compilation
- **libpcap** - Packet capture (usually pre-installed, Homebrew version recommended)

### Installation Commands

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install libpcap via Homebrew (recommended)
brew install libpcap
```

### Architecture Support

The SPECTRE GUI supports both Intel and Apple Silicon:
- **Intel (x86_64):** Fully supported on Intel Macs
- **Apple Silicon (ARM64):** Fully supported on M1/M2/M3 Macs
- **Universal Binary:** Can be built to support both architectures in a single .app

### Testing Your Installation

Run the provided test script:
```bash
cd crates/spectre-gui
./scripts/test-macos.sh
```

---

## Windows

### System Requirements
- **Minimum OS:** Windows 10 version 1809 (October 2018 Update) or later
- **Recommended:** Windows 11
- **Architecture:** x86_64 (64-bit)

### System Dependencies

- **Microsoft Edge WebView2 Runtime** - Bundled with installer (automatically installed)
- **Visual C++ Redistributable** - Included in installer
- **Microsoft Visual C++ Build Tools** - Required for building from source

### Development Requirements

If building from source:

1. **Visual Studio Build Tools**
   - Download from: https://visualstudio.microsoft.com/downloads/
   - Install "Desktop development with C++" workload
   - Alternative: Full Visual Studio 2019/2022 Community Edition

2. **Rust** (via rustup)
   - Download from: https://rustup.rs/
   - Ensure MSVC toolchain is installed: `rustup default stable-msvc`

### Optional: ProRT-IP Network Scanning

For full network scanning capabilities with ProRT-IP:

- **Npcap 1.70 or later**
  - Download: https://npcap.com/#download
  - Required for raw packet access and advanced network scanning
  - Note: Requires administrator privileges for installation and operation

### Runtime Requirements

End users only need:
- Windows 10 1809+ or Windows 11
- WebView2 Runtime (bundled with installer)

### Testing Your Installation

Run the provided test script in PowerShell:
```powershell
cd crates/spectre-gui
.\scripts\test-windows.ps1
```

---

## Additional Notes

### For End Users

If you're just running the pre-built installers:

- **Linux:** AppImage requires no dependencies (self-contained). DEB/RPM packages will automatically install dependencies.
- **macOS:** DMG installer includes all required components.
- **Windows:** MSI installer includes WebView2 and Visual C++ Redistributable.

### For Developers

If you're building from source:

1. Install Rust via [rustup](https://rustup.rs/)
2. Install Node.js 22+ and pnpm 9
3. Follow platform-specific dependency installation above
4. Run the test scripts to verify your environment

### CI/CD

The GitHub Actions workflows automatically handle all platform-specific dependencies and build configurations. See `.github/workflows/gui.yml` and `.github/workflows/release.yml` for reference.

### Security Note

ProRT-IP network scanning functionality requires elevated privileges:
- **Linux:** Run with `sudo` or grant `CAP_NET_RAW` capability
- **macOS:** Run with `sudo` or grant admin privileges
- **Windows:** Run as Administrator and install Npcap

---

## Troubleshooting

### Linux: "webkit2gtk-4.1 not found"
Ensure you're running Ubuntu 22.04+ or equivalent. Older versions (like Ubuntu 20.04) ship webkit2gtk-4.0, which is incompatible with Tauri 2.0.

### macOS: "xcode-select: error: tool 'xcodebuild' requires Xcode"
Run `xcode-select --install` to install Command Line Tools without full Xcode.

### Windows: "error: linker `link.exe` not found"
Install Visual Studio Build Tools with "Desktop development with C++" workload.

### Network scanning fails
- Ensure libpcap (Linux/macOS) or Npcap (Windows) is installed
- Run with elevated privileges (sudo/Administrator)
- Check firewall settings

---

## Support

For issues or questions:
- GitHub Issues: https://github.com/doublegate/SPECTRE/issues
- Documentation: See `docs/` directory
- Security: See `SECURITY.md`
