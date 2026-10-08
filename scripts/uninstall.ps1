
<#
/************************************************
* File: uninstall.ps1
* Author: Michal Švrček
*
* DevDock Windows uninstaller
*
* ver. 0.4.0
*************************************************/
#>

$ErrorActionPreference = "Stop"

$InstallDir = Join-Path $env:LOCALAPPDATA "Programs\DevDock"
$BinaryPath = Join-Path $InstallDir "dock.exe"

Write-Host "DevDock Windows Uninstaller" -ForegroundColor Cyan

if (Test-Path -LiteralPath $BinaryPath) {
    Remove-Item -LiteralPath $BinaryPath -Force
}

if ((Test-Path -LiteralPath $InstallDir) -and
    -not (Get-ChildItem -LiteralPath $InstallDir -Force)) {
    Remove-Item -LiteralPath $InstallDir -Force
}

$UserPath = [Environment]::GetEnvironmentVariable(
    "Path",
    "User"
)

$UpdatedEntries = @(
    ($UserPath -split ';') | Where-Object {
        -not [string]::IsNullOrWhiteSpace($_) -and
        $_.TrimEnd('\') -ine $InstallDir.TrimEnd('\')
    }
)

[Environment]::SetEnvironmentVariable(
    "Path",
    ($UpdatedEntries -join ';'),
    "User"
)

Write-Host ""
Write-Host "DevDock uninstalled successfully." -ForegroundColor Green
Write-Host "Your sessions were preserved in:"
Write-Host "  $env:APPDATA\DevDock\sessions"
Write-Host ""
Write-Host "Open a new terminal to refresh PATH."
