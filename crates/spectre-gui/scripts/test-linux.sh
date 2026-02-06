#!/bin/bash
set -e

echo "=== Linux Platform Test ==="

# Check system dependencies
echo "Checking webkit2gtk-4.1..."
if ! pkg-config --exists webkit2gtk-4.1; then
    echo "ERROR: webkit2gtk-4.1 not found"
    echo "Install with: sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev"
    exit 1
fi

echo "Checking GTK 3..."
if ! pkg-config --exists gtk+-3.0; then
    echo "ERROR: GTK 3 not found"
    echo "Install with: sudo apt-get install libgtk-3-dev"
    exit 1
fi

# Build frontend
echo "Building frontend..."
cd "$(dirname "$0")/../frontend"
pnpm install --frozen-lockfile
pnpm build

# Build GUI
echo "Building GUI..."
cd ..
cargo build -p spectre-gui

# Test
echo "Running tests..."
cargo test -p spectre-gui

# Try to build bundle (if Tauri CLI available)
if command -v cargo-tauri &> /dev/null; then
    echo "Building AppImage..."
    cargo tauri build --target x86_64-unknown-linux-gnu --bundles appimage
    echo "AppImage built successfully!"
else
    echo "Tauri CLI not found. Skipping bundle build."
    echo "Install with: cargo install tauri-cli"
fi

echo "=== Linux test PASSED ==="
