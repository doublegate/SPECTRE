#!/usr/bin/env bash
# SPECTRE GUI Development Helper Script
# Starts frontend and backend with automatic display server detection

set -e

# Parse command line arguments
VERBOSE=false
FORCE_BACKEND=""
CHECK_FRONTEND=true

print_usage() {
    cat << EOF
SPECTRE GUI Development Helper Script

Usage: $0 [OPTIONS]

Options:
    --verbose           Enable verbose logging (RUST_LOG=debug)
    --trace             Enable trace logging (RUST_LOG=trace)
    --force-x11         Force X11 backend (override auto-detection)
    --force-wayland     Force Wayland backend (override auto-detection)
    --skip-frontend     Skip frontend check (assume it's running)
    -h, --help          Show this help message

Examples:
    $0                  # Normal mode with auto-detection
    $0 --verbose        # Debug mode
    $0 --force-x11      # Force X11 backend
    $0 --trace          # Maximum verbosity

Display Server Detection:
    The GUI automatically detects Wayland vs X11 and applies appropriate
    workarounds for GPU hardware acceleration. Manual overrides are only
    needed for debugging or if auto-detection fails.

Environment Variables:
    RUST_LOG            Control log verbosity (info, debug, trace)
    GDK_BACKEND         Override display backend (wayland, x11)

For troubleshooting, see: docs/troubleshooting/GUI-DISPLAY-ISSUES.md
EOF
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose)
            VERBOSE=true
            shift
            ;;
        --trace)
            export RUST_LOG=trace
            shift
            ;;
        --force-x11)
            FORCE_BACKEND="x11"
            shift
            ;;
        --force-wayland)
            FORCE_BACKEND="wayland"
            shift
            ;;
        --skip-frontend)
            CHECK_FRONTEND=false
            shift
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo "🚀 Starting SPECTRE GUI Development Environment..."
echo ""

# Check if frontend is already running (unless skipped)
if [ "$CHECK_FRONTEND" = true ]; then
    if ! curl -s http://localhost:1420 > /dev/null 2>&1; then
        echo "⚠️  Frontend not running. Start it with:"
        echo "   cd frontend && pnpm dev"
        echo ""
        echo "Or skip this check with: $0 --skip-frontend"
        echo ""
        exit 1
    fi
    echo "✅ Frontend detected at http://localhost:1420"
else
    echo "⏭️  Skipping frontend check (assuming it's running)"
fi

echo "🔧 Starting Tauri backend..."
echo ""

# Set environment variables based on flags
if [ "$VERBOSE" = true ]; then
    export RUST_LOG=debug
    echo "🐛 Debug logging enabled (RUST_LOG=debug)"
    echo ""
fi

if [ -n "$FORCE_BACKEND" ]; then
    export GDK_BACKEND="$FORCE_BACKEND"
    echo "🔧 Forcing GDK_BACKEND=$FORCE_BACKEND"
    echo ""
fi

# Display current environment (if verbose)
if [ "$VERBOSE" = true ] || [ -n "$RUST_LOG" ]; then
    echo "📋 Environment:"
    echo "   XDG_SESSION_TYPE: ${XDG_SESSION_TYPE:-not set}"
    echo "   DISPLAY: ${DISPLAY:-not set}"
    echo "   WAYLAND_DISPLAY: ${WAYLAND_DISPLAY:-not set}"
    [ -n "$GDK_BACKEND" ] && echo "   GDK_BACKEND: $GDK_BACKEND (override)"
    [ -n "$RUST_LOG" ] && echo "   RUST_LOG: $RUST_LOG"
    echo ""
fi

# Run Tauri dev server
# Note: Display server detection happens automatically in main.rs
cargo tauri dev
