@echo off
call "F:\VSBuildTool\VC\Auxiliary\Build\vcvarsall.bat" x64_arm64
cargo build --release --target aarch64-pc-windows-msvc
if %errorlevel% neq 0 exit /b %errorlevel%
echo Build complete: target\aarch64-pc-windows-msvc\release\ntc.exe
