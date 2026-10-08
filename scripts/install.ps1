
<#
/************************************************
* File: install.ps1
* Author: Michal Švrček
*
* DevDock Windows installer
*
* ver. 0.4.0
*************************************************/
#>

[CmdletBinding()]
param(
    [string]$BinaryPath
)

$ErrorActionPreference = "Stop"

$ProjectRoot = Split-Path -Parent $PSScriptRoot

if ([string]::IsNullOrWhiteSpace($BinaryPath)) {
    $BinaryPath = Join-Path $ProjectRoot "target\release\dock.exe"
}

if (-not (Test-Path -LiteralPath $BinaryPath -PathType Leaf)) {
    throw "dock.exe not found: $BinaryPath. Run cargo build --release first."
}

$InstallDir = Join-Path $env:LOCALAPPDATA "Programs\DevDock"
$Destination = Join-Path $InstallDir "dock.exe"

Write-Host "DevDock Windows Installer" -ForegroundColor Cyan

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$Source = (Resolve-Path -LiteralPath $BinaryPath).Path

if ($Source -ne $Destination) {
    Copy-Item -LiteralPath $Source -Destination $Destination -Force
}

# Update the persistent user PATH.
$UserPath = [Environment]::GetEnvironmentVariable(
    "Path",
    "User"
)

$PathEntries = @(
    ($UserPath -split ';') | Where-Object {
        -not [string]::IsNullOrWhiteSpace($_)
    }
)

$AlreadyInstalled = $false

foreach ($Entry in $PathEntries) {
    if ($Entry.TrimEnd('\') -ieq $InstallDir.TrimEnd('\')) {
        $AlreadyInstalled = $true
        break
    }
}

if (-not $AlreadyInstalled) {
    $NewUserPath = (@($PathEntries) + $InstallDir) -join ';'

    [Environment]::SetEnvironmentVariable(
        "Path",
        $NewUserPath,
        "User"
    )

    Write-Host "Added DevDock to user PATH."
}

# Update PATH for the installer process too.
$CurrentEntries = $env:Path -split ';'

if (-not ($CurrentEntries | Where-Object {
    $_.TrimEnd('\') -ieq $InstallDir.TrimEnd('\')
})) {
    $env:Path += ";$InstallDir"
}

Write-Host ""
Write-Host "DevDock installed successfully!" -ForegroundColor Green
Write-Host "Location: $Destination"
Write-Host "Version:"
& $Destination --version

Write-Host ""
Write-Host "Open a NEW terminal, then run:"
Write-Host "  dock --version"
Write-Host "  dock list"
Write-Host "  dock rsu"
