@echo off
rem Win7-compatible AMD64 build.
rem Uses the getrandom windows_legacy backend (RtlGenRandom via advapi32,
rem works on Win7+) instead of the default ProcessPrng backend
rem (bcryptprimitives.dll, Win10+ only).
rem NOTE: the quotes on the set line below must stay raw - do NOT escape
rem them. This file exists because passing quotes through `ran` (TOML ->
rem ntc -> cmd /C) gets them backslash-escaped by Rust argv quoting, which
rem cmd.exe keeps literally and rustc then rejects.
set RUSTFLAGS=--cfg getrandom_backend="windows_legacy"
cargo build --release --target-dir target/win7-amd64
if %errorlevel% neq 0 exit /b %errorlevel%
echo Build complete: target\win7-amd64\release\ntc.exe
