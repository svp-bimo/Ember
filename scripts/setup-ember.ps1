# Ember CLI setup script for Windows PowerShell
# - Installs ember-cli to Cargo bin
# - Ensures Cargo bin is in PATH (User scope)
# - Adds a persistent PowerShell alias: ember -> ember-cli

$ErrorActionPreference = 'Stop'

function Write-Info($msg) {
    Write-Host "[ember-setup] $msg"
}

# Validate Rust/Cargo
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "cargo was not found. Install Rust from https://rustup.rs first."
}

# Install ember-cli
Write-Info "Installing ember-cli from local workspace..."
$repoRoot = (Resolve-Path "$PSScriptRoot\..").Path
Push-Location $repoRoot
try {
    cargo install --path crates/ember-cli --force | Out-Host
}
finally {
    Pop-Location
}

# Ensure Cargo bin is on PATH (User)
$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if (-not $userPath) { $userPath = "" }
if ($userPath -notlike "*${cargoBin}*") {
    Write-Info "Adding Cargo bin to PATH (User)..."
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$cargoBin", "User")
}
else {
    Write-Info "Cargo bin already on PATH (User)."
}

# Add alias to PowerShell profile
$profilePath = $PROFILE
$aliasLine = "Set-Alias ember ember-cli"
if (-not (Test-Path $profilePath)) {
    Write-Info "Creating PowerShell profile at $profilePath"
    New-Item -ItemType File -Force -Path $profilePath | Out-Null
}

$profileContent = Get-Content $profilePath -Raw
if ($profileContent -notmatch [regex]::Escape($aliasLine)) {
    Write-Info "Adding alias 'ember' -> 'ember-cli' to PowerShell profile."
    Add-Content -Path $profilePath -Value "`n$aliasLine`n"
}
else {
    Write-Info "Alias already present in PowerShell profile."
}

Write-Host ""
Write-Info "Setup complete. Open a NEW PowerShell terminal to use:"
Write-Host "  ember new my-service"
Write-Host "  ember build"
Write-Host "  ember openapi"
