; win7amd.iss — Windows installer for the Windows 7 compatible ntc build.
;
; Same install identity as installer.iss (same AppName, install dir, PATH
; entries) so it installs/upgrades in place with the regular installer.
; Differences:
;   - bundles target\win7-amd64\x86_64-win7-windows-msvc\release\ntc.exe (the
;     official win7-vendor target: std uses GetSystemTimeAsFileTime instead of
;     the Win8+ GetSystemTimePreciseAsFileTime, and getrandom falls back to
;     RtlGenRandom so no bcryptprimitives.dll import). Build it first with:
;     ran win7amd
;   - OutputBaseFilename carries the -win7-x64 suffix.
;   - MinVersion documents the Win7 SP1 floor.
;
; Compile with Inno Setup 6:
;   iscc win7amd.iss                   -> ...-win7-x64.exe       (all users)
;   iscc /DUSERINSTALL win7amd.iss     -> ...-win7-x64-user.exe  (per-user, no UAC)
;
; The shared setup body lives in packaging\ntc_install.issi.

#define AppVer "2.4.8"

#ifdef USERINSTALL
  #define OutputSuffix "-user"
#else
  #define OutputSuffix ""
#endif

#define WinDir     "target\win7-amd64\x86_64-win7-windows-msvc\release"
#define ExeSource  WinDir + "\ntc.exe"
#define DllGlob    WinDir + "\*.dll"
#define DepsGlob   WinDir + "\deps\*.dll"
#define OutputBase "ntc-installer-" + AppVer + "-win7-x64"
#define MinOS "6.1sp1"

#include "packaging\ntc_install.issi"
