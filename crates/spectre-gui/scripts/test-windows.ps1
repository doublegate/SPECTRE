#!/usr/bin/env pwsh

Write-Host "=== Windows Platform Test ===" -ForegroundColor Cyan

# Check for Visual C++ Build Tools
$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not (Test-Path $vsWhere)) {
    Write-Host "ERROR: Visual Studio Build Tools not found" -ForegroundColor Red
    Write-Host "Download from: https://visualstudio.microsoft.com/downloads/" -ForegroundColor Yellow
    exit 1
}

# Build frontend
Write-Host "Building frontend..." -ForegroundColor Green
Set-Location (Join-Path $PSScriptRoot "..\frontend")
pnpm install --frozen-lockfile
pnpm build

# Build GUI
Write-Host "Building GUI..." -ForegroundColor Green
Set-Location ..
cargo build -p spectre-gui --target x86_64-pc-windows-msvc

if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Build failed" -ForegroundColor Red
    exit $LASTEXITCODE
}

# Test
Write-Host "Running tests..." -ForegroundColor Green
cargo test -p spectre-gui --target x86_64-pc-windows-msvc

if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Tests failed" -ForegroundColor Red
    exit $LASTEXITCODE
}

# Try to build MSI (if Tauri CLI available)
if (Get-Command cargo-tauri -ErrorAction SilentlyContinue) {
    Write-Host "Building MSI..." -ForegroundColor Green
    cargo tauri build --target x86_64-pc-windows-msvc --bundles msi
    Write-Host "MSI built successfully!" -ForegroundColor Green
} else {
    Write-Host "Tauri CLI not found. Skipping bundle build." -ForegroundColor Yellow
    Write-Host "Install with: cargo install tauri-cli" -ForegroundColor Yellow
}

Write-Host "=== Windows test PASSED ===" -ForegroundColor Green
