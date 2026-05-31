@echo off
REM install.bat - Build ntc from source on Windows and add to PATH
REM Requires: Rust (cargo) and Administrator privileges

echo 🔨 Building ntc from source...

:: Check for Administrator privileges
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Administrator privileges required!
    echo    Please right-click and select "Run as Administrator"
    pause
    exit /b 1
)

:: Check for cargo
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ Rust/Cargo not found. Please install Rust first:
    echo    https://rustup.rs/
    pause
    exit /b 1
)

:: Build release binary
cargo build --release

if %errorlevel% neq 0 (
    echo ❌ Build failed!
    pause
    exit /b 1
)

:: Copy to System32 (already in PATH)
echo 📁 Copying ntc.exe to C:\Windows\System32\...
copy /Y target\release\ntc.exe C:\Windows\System32\ntc.exe >nul

if %errorlevel% neq 0 (
    echo ❌ Failed to copy to System32
    pause
    exit /b 1
)

echo.
ntc.exe --version
echo.
echo ✅ ntc installed successfully!
echo    Run 'ntc' from any command prompt to start.
echo.
pause