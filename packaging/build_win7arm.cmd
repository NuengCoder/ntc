@echo off
rem build_win7arm.cmd — Win7-compatible ARM64 build.
rem Uses the custom aarch64-win7-windows-msvc target (packaging\targets\):
rem target_vendor = "win7" switches std AND getrandom to the legacy
rem RtlGenRandom backend (advapi32, Win7 SP1+) instead of ProcessPrng
rem (bcryptprimitives.dll, Win10+ only). No RUSTFLAGS cfg is needed.
rem Toolchain: vcvars x64_arm64 pinned to the newest MSVC toolset that
rem really ships the ARM64 set plus rust-lld (see .cargo\config.toml) and
rem -Zbuild-std (custom target
rem ships no prebuilt std; RUSTC_BOOTSTRAP unlocks it on stable).
rem Output: target\win7-arm64\aarch64-win7-windows-msvc\release\ntc.exe
rem Also needs clang.exe on PATH: ring forces clang for every aarch64 +
rem windows target (see packaging\find_clang.cmd).

rem --- Locate Visual Studio (portable: vswhere first, fallback to D:\VS) ---
set "VSROOT="
if exist "%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" (
  for /f "usebackq tokens=*" %%i in (`"%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools -property installationPath 2^>nul`) do set "VSROOT=%%i"
)
if not defined VSROOT if defined VSINSTALLDIR set "VSROOT=%VSINSTALLDIR%"
if not defined VSROOT set "VSROOT=D:\VS"
if not exist "%VSROOT%\VC\Auxiliary\Build\vcvarsall.bat" (
  echo ERROR: vcvarsall.bat not found under "%VSROOT%".
  echo Set VSINSTALLDIR to your Visual Studio install, or install the
  echo "Desktop development with C++" workload with ARM64 build tools.
  exit /b 1
)

rem Pick the newest MSVC toolset that has the whole ARM64 set: cl.exe under
rem HostX64\arm64, lib\arm64\msvcrt.lib AND the C/C++ headers. Installs
rem differ - 14.44 here ships no include dir at all and 14.51 has no ARM64
rem cl.exe (only 29 libs) - so a hardcoded -vcvars_ver either leaves
rem INCLUDE without vcruntime.h or fails the link with LNK1104.
set "MSVCVER="
for /d %%d in ("%VSROOT%\VC\Tools\MSVC\*") do (
  if exist "%%d\include\vcruntime.h" if exist "%%d\bin\HostX64\arm64\cl.exe" if exist "%%d\lib\arm64\msvcrt.lib" set "MSVCVER=%%~nxd"
)
if not defined MSVCVER (
  echo ERROR: no MSVC toolset with the complete ARM64 set under
  echo "%VSROOT%\VC\Tools\MSVC" - install the ARM64 build tools plus the
  echo C++ headers for that toolset.
  exit /b 1
)
echo Using MSVC toolset %MSVCVER%

call "%VSROOT%\VC\Auxiliary\Build\vcvarsall.bat" x64_arm64 -vcvars_ver=%MSVCVER%
if %errorlevel% neq 0 exit /b %errorlevel%

rem ring (russh / ureq / lettre) overrides its compiler to clang for every
rem aarch64 + windows target, and vcvars does not put LLVM on PATH - without
rem this the build dies with "failed to find tool \"clang\"".
call "%~dp0find_clang.cmd"
if errorlevel 1 exit /b 1

rem Compose LIB for the two links sharing this shell (lld ignores LIB
rem entirely — the target link uses build.rs search paths — so order only
rem matters for the x64 host links of build scripts / proc-macros):
rem repo-local libs, x64 CRT, SDK x64, then the vcvars ARM64 dirs.
rem A pinned toolset can ship no x64 libs, so find any installed
rem toolset that has them (newest wins).
set "ARM64LIB=%LIB%"
set "X64MSVCLIB="
for /d %%d in ("%VSROOT%\VC\Tools\MSVC\*") do if exist "%%d\lib\x64\msvcrt.lib" set "X64MSVCLIB=%%d\lib\x64"
if not defined X64MSVCLIB echo WARNING: no MSVC x64 libs found; host links will fail.
set "LIB=%~dp0arm64-libs"
if defined X64MSVCLIB set "LIB=%LIB%;%X64MSVCLIB%"
if defined WindowsSdkDir if defined WindowsSDKVersion set "LIB=%LIB%;%WindowsSdkDir%lib\%WindowsSDKVersion%ucrt\x64;%WindowsSdkDir%lib\%WindowsSDKVersion%um\x64"
set "LIB=%LIB%;%ARM64LIB%"

rem Pin the C compiler to the selected toolset's ARM64 cl: cc would
rem otherwise pick the
rem newest toolset (no ARM64 compiler there) and silently emit x64 objects
rem ("machine type x64 conflicts with arm64" at link).
set "CC_aarch64_win7_windows_msvc=%VCToolsInstallDir%bin\HostX64\ARM64\cl.exe"

rem Nightly toolchain: the custom target JSON plus -Zbuild-std need it
rem (prebuilt std does not exist for this triple). -Zunstable-options
rem lets rustc load the JSON from RUST_TARGET_PATH.
set "RUST_TARGET_PATH=%~dp0targets"
set "RUSTFLAGS=-Zunstable-options"
cargo +nightly build -Zbuild-std=core,std --release --target aarch64-win7-windows-msvc --target-dir target/win7-arm64
if %errorlevel% neq 0 exit /b %errorlevel%
rem Same combase.dll -> ole32.dll post-link fix as build_win7amd.cmd
rem (new SDKs resolve CoTaskMemFree to Win8+ combase.dll). Non-fatal here:
rem the ARM64 import table hasn't been dumpbin-verified on this host yet,
rem so warn instead of failing; verify before shipping Win7-ARM64 binaries.
python3 "%~dp0fix_win7_imports.py" "target\win7-arm64\aarch64-win7-windows-msvc\release\ntc.exe"
if %errorlevel% neq 0 echo WARNING: fix_win7_imports.py found nothing to patch - verify imports with dumpbin before shipping.
echo Build complete: target\win7-arm64\aarch64-win7-windows-msvc\release\ntc.exe
