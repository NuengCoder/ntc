@echo off
rem find_clang.cmd - locate clang.exe for the Windows ARM64 builds.
rem
rem ring 0.17 (pulled in by russh / ureq / lettre) hard-selects clang for
rem every aarch64 + windows target: it compiles both its C sources and its
rem pregenerated win64 ARM64 .S assembly with `clang`, because MSVC's
rem cl.exe / armasm64 cannot assemble perlasm output. cc-rs resolves that
rem compiler by the bare name "clang", so clang.exe has to be on PATH -
rem and vcvars never adds LLVM to PATH. Called from packaging\build_win32.cmd
rem and packaging\build_win7arm.cmd right after vcvars; prepends the
rem directory holding clang.exe to PATH and returns 1 when it is missing.
rem
rem Override with NTC_CLANG=<full path to clang.exe>.
rem Search order: NTC_CLANG, repo-local portable LLVM (packaging\tools\llvm,
rem git-ignored), PATH, Visual Studio "C++ Clang tools for Windows"
rem (%VCINSTALLDIR%Tools\Llvm\<host>\bin), %ProgramFiles%\LLVM\bin.

set "CLANG_EXE="
if defined NTC_CLANG if exist "%NTC_CLANG%" set "CLANG_EXE=%NTC_CLANG%"
if not defined CLANG_EXE if exist "%~dp0tools\llvm\bin\clang.exe" set "CLANG_EXE=%~dp0tools\llvm\bin\clang.exe"
if not defined CLANG_EXE for %%C in (clang.exe) do if not defined CLANG_EXE set "CLANG_EXE=%%~$PATH:C"
if not defined CLANG_EXE if defined VCINSTALLDIR (
  for %%D in ("%VCINSTALLDIR%Tools\Llvm\x64\bin" "%VCINSTALLDIR%Tools\Llvm\ARM64\bin" "%VCINSTALLDIR%Tools\Llvm\bin") do (
    if not defined CLANG_EXE if exist "%%~D\clang.exe" set "CLANG_EXE=%%~D\clang.exe"
  )
)
if not defined CLANG_EXE if exist "%ProgramFiles%\LLVM\bin\clang.exe" set "CLANG_EXE=%ProgramFiles%\LLVM\bin\clang.exe"

if not defined CLANG_EXE (
  echo ERROR: clang.exe not found - it is required for the Windows ARM64 builds.
  echo ring ^(used by russh / ureq / lettre^) forces clang on every
  echo aarch64 + windows target; MSVC cannot assemble its ARM64 asm.
  echo.
  echo Install one of:
  echo   Visual Studio Installer component "C++ Clang tools for Windows"
  echo   winget install LLVM.LLVM
  echo Then re-run this script, or set NTC_CLANG to the full clang.exe path.
  exit /b 1
)

for %%C in ("%CLANG_EXE%") do set "CLANG_DIR=%%~dpC"
set "PATH=%CLANG_DIR%;%PATH%"
echo Using clang: %CLANG_EXE%
exit /b 0
