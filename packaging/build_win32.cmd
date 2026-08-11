@echo off
call "D:\VS\VC\Auxiliary\Build\vcvarsall.bat" x64_arm64 -vcvars_ver=14.44.35207
set "LIB=D:\VS\VC\Tools\MSVC\14.51.36231\lib\x64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.26100.0\ucrt\x64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.26100.0\um\x64"
cargo build --release --target aarch64-pc-windows-msvc
if %errorlevel% neq 0 exit /b %errorlevel%
echo Build complete: target\aarch64-pc-windows-msvc\release\ntc.exe