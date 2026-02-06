#!/bin/bash
set -e

echo "=== macOS Platform Test ==="

# Check Xcode Command Line Tools
if ! xcode-select -p &> /dev/null; then
    echo "ERROR: Xcode Command Line Tools not found"
    echo "Install with: xcode-select --install"
    exit 1
fi

# Detect architecture
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    TARGETS=("aarch64-apple-darwin")
    echo "Detected Apple Silicon (ARM64)"
elif [ "$ARCH" = "x86_64" ]; then
    TARGETS=("x86_64-apple-darwin")
    echo "Detected Intel (x86_64)"
else
    echo "Unknown architecture: $ARCH"
    exit 1
fi

# Build frontend
echo "Building frontend..."
cd "$(dirname "$0")/../frontend"
pnpm install --frozen-lockfile
pnpm build

# Build for detected architecture
cd ..
for TARGET in "${TARGETS[@]}"; do
    echo "Building for $TARGET..."
    cargo build -p spectre-gui --target "$TARGET"
done

# Test on current architecture
echo "Running tests..."
cargo test -p spectre-gui

# Try to build DMG (if Tauri CLI available)
if command -v cargo-tauri &> /dev/null; then
    echo "Building DMG..."
    cargo tauri build --bundles dmg
    echo "DMG built successfully!"
else
    echo "Tauri CLI not found. Skipping bundle build."
    echo "Install with: cargo install tauri-cli"
fi

echo "=== macOS test PASSED ==="
