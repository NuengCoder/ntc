@echo off
rem Cross-compile for Windows ARM64 (MSVC).
rem Uses vswhere/VSINSTALLDIR to locate Visual Studio portably;
rem falls back to the previous hardcoded path. Toolchain: vcvars
rem x64_arm64 pinned to MSVC 14.44 (complete ARM64 set) plus rust-lld
rem (see .cargo\config.toml).
rem Output: target\aarch64-pc-windows-msvc\release\ntc.exe

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

call "%VSROOT%\VC\Auxiliary\Build\vcvarsall.bat" x64_arm64 -vcvars_ver=14.44
if %errorlevel% neq 0 exit /b %errorlevel%

rem Compose LIB for the two links sharing this shell (lld ignores LIB
rem entirely — the target link uses build.rs search paths — so order only
rem matters for the x64 host links of build scripts / proc-macros):
rem repo-local libs, x64 CRT, SDK x64, then the vcvars ARM64 dirs.
rem Without this, host x64 links pick up the ARM64 msvcrt.lib and fail
rem with LNK4272 + unresolved externals (memcpy, __CxxFrameHandler3,
rem mainCRTStartup, ...). The pinned 14.44 toolset ships no x64 libs,
rem so find any installed toolset that has them (newest wins).
set "ARM64LIB=%LIB%"
set "X64MSVCLIB="
for /d %%d in ("%VSROOT%\VC\Tools\MSVC\*") do if exist "%%d\lib\x64\msvcrt.lib" set "X64MSVCLIB=%%d\lib\x64"
if not defined X64MSVCLIB echo WARNING: no MSVC x64 libs found; host links will fail.
set "LIB=%~dp0arm64-libs"
if defined X64MSVCLIB set "LIB=%LIB%;%X64MSVCLIB%"
if defined WindowsSdkDir if defined WindowsSDKVersion set "LIB=%LIB%;%WindowsSdkDir%lib\%WindowsSDKVersion%ucrt\x64;%WindowsSdkDir%lib\%WindowsSDKVersion%um\x64"
set "LIB=%LIB%;%ARM64LIB%"

rem Pin the C compiler to the 14.44 ARM64 cl: cc would otherwise pick the
rem newest toolset (no ARM64 compiler there) and silently emit x64 objects
rem ("machine type x64 conflicts with arm64" at link).
set "CC_aarch64_pc_windows_msvc=%VCToolsInstallDir%bin\HostX64\ARM64\cl.exe"

cargo build --release --target aarch64-pc-windows-msvc
if %errorlevel% neq 0 exit /b %errorlevel%
echo Build complete: target\aarch64-pc-windows-msvc\release\ntc.exe
