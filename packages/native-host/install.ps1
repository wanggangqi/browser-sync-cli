#!/usr/bin/env pwsh
# Install script for Browser Sync Native Messaging Host on Windows

param(
    [string]$ChromePath = "HKCU:\Software\Google\Chrome\NativeMessagingHosts",
    [string]$EdgePath = "HKCU:\Software\Microsoft\Edge\NativeMessagingHosts"
)

$ErrorActionPreference = "Stop"

# Get the directory where this script is located
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$NativeHostExe = Join-Path $ScriptDir "browser-sync-native-host.exe"

# Check if the executable exists
if (-not (Test-Path $NativeHostExe)) {
    # Try to build it
    Write-Host "Building native host..."
    cargo build --release --manifest-path (Join-Path $ScriptDir "Cargo.toml")

    $TargetDir = Join-Path $ScriptDir "target\release"
    $NativeHostExe = Join-Path $TargetDir "browser-sync-native-host.exe"

    if (-not (Test-Path $NativeHostExe)) {
        Write-Error "Failed to find or build native host executable"
        exit 1
    }
}

# Create native messaging host manifest
$HostId = "com.browser_sync.cli"
$ManifestJson = @{
    name = $HostId
    description = "Browser Sync CLI Native Messaging Host"
    path = $NativeHostExe
    type = "stdio"
    allowed_origins = @(
        "chrome-extension://*/",
        "chrome-extension://*/"
    )
} | ConvertTo-Json -Depth 10

# Create manifest directory
$ManifestDir = Join-Path $env:LOCALAPPDATA "browser-sync-cli"
$ManifestFile = Join-Path $ManifestDir "$HostId.json"

Write-Host "Creating manifest directory: $ManifestDir"
New-Item -ItemType Directory -Force -Path $ManifestDir | Out-Null

Write-Host "Writing manifest file: $ManifestFile"
$ManifestJson | Out-File -FilePath $ManifestFile -Encoding utf8

# Register for Chrome
Write-Host "Registering for Chrome..."
if (-not (Test-Path $ChromePath)) {
    New-Item -Path $ChromePath -Force | Out-Null
}
New-ItemProperty -Path $ChromePath -Name $HostId -Value $ManifestFile -Force | Out-Null

# Register for Edge
Write-Host "Registering for Edge..."
if (-not (Test-Path $EdgePath)) {
    New-Item -Path $EdgePath -Force | Out-Null
}
New-ItemProperty -Path $EdgePath -Name $HostId -Value $ManifestFile -Force | Out-Null

Write-Host ""
Write-Host "Installation complete!"
Write-Host "Manifest file: $ManifestFile"
Write-Host "Native host executable: $NativeHostExe"
Write-Host ""
Write-Host "Note: You need to install the Chrome extension separately."
Write-Host "After installing the extension, you need to update the allowed_origins in the manifest file"
Write-Host "to include your extension's ID."