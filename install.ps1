# install.ps1 - Build ntc from source on Windows and add to PATH
# Requires: Rust (cargo) and Administrator privileges
# Run with: powershell -ExecutionPolicy Bypass -File install.ps1

Write-Host "🔨 Building ntc from source..." -ForegroundColor Cyan

# Check for Administrator privileges
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "❌ Administrator privileges required!" -ForegroundColor Red
    Write-Host "   Please right-click PowerShell and select 'Run as Administrator'" -ForegroundColor Yellow
    exit 1
}

# Check for cargo
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust/Cargo not found. Please install Rust first:" -ForegroundColor Red
    Write-Host "   https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

# Build release binary
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

# Copy to System32 (already in PATH)
Write-Host "📁 Copying ntc.exe to C:\Windows\System32\..." -ForegroundColor Cyan
Copy-Item -Path "target\release\ntc.exe" -Destination "C:\Windows\System32\ntc.exe" -Force

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Failed to copy to System32" -ForegroundColor Red
    exit 1
}

Write-Host ""
& ntc --version
Write-Host ""
Write-Host "✅ ntc installed successfully!" -ForegroundColor Green
Write-Host "   Run 'ntc' from any command prompt to start." -ForegroundColor Cyan