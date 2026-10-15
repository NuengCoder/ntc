@echo off
rem build_win7amd.cmd — Win7-compatible AMD64 build.
rem Uses the official x86_64-win7-windows-msvc target (vendor="win7"):
rem std uses GetSystemTimeAsFileTime instead of GetSystemTimePreciseAsFileTime
rem (Win8+ only, Kernel32.dll loader error on Win7) and getrandom defaults to
rem the legacy RtlGenRandom backend (advapi32, Win7 SP1+) instead of ProcessPrng
rem (bcryptprimitives.dll, Win10+ only).
rem The --cfg getrandom_backend="windows_legacy" is kept as belt-and-braces for
rem older getrandom copies in the tree that predate the win7 vendor detection.
rem Needs nightly + -Zbuild-std (Tier 3 target ships no prebuilt std).
rem Output: target\win7-amd64\x86_64-win7-windows-msvc\release\ntc.exe
rem Post-link, packaging\fix_win7_imports.py rewrites the combase.dll import
rem (new SDKs resolve Ole32.lib's CoTaskMemFree there; Win8+) to ole32.dll,
rem which exports the identical function on Win7 SP1 through Win11.
rem NOTE: like all VS2015+ binaries this still needs the Universal CRT on
rem Win7 (KB2999226, usually already installed via Windows Update).
rem NOTE: the quotes on the set line below must stay raw - do NOT escape
rem them. This file exists because passing quotes through `ran` (TOML ->
rem ntc -> cmd /C) gets them backslash-escaped by Rust argv quoting, which
rem cmd.exe keeps literally and rustc then rejects.
set RUSTFLAGS=--cfg getrandom_backend="windows_legacy"
cargo +nightly build -Zbuild-std=core,std --release --target x86_64-win7-windows-msvc --target-dir target/win7-amd64
if %errorlevel% neq 0 exit /b %errorlevel%
python3 "%~dp0fix_win7_imports.py" "target\win7-amd64\x86_64-win7-windows-msvc\release\ntc.exe"
if %errorlevel% neq 0 exit /b %errorlevel%
echo Build complete: target\win7-amd64\x86_64-win7-windows-msvc\release\ntc.exe
