; installer.iss — Windows installer for the regular (Win10+) ntc build.
;
; Compile with Inno Setup 6:
;   iscc installer.iss                    -> ntc-installer-<ver>.exe
;                                            all users, requires admin
;   iscc /DUSERINSTALL installer.iss      -> ntc-installer-<ver>-user.exe
;                                            per-user, no UAC
;
; The per-user variants exist for standard user accounts: a setup whose
; manifest asks for elevation cannot start without an admin credential (Windows
; cannot create its temp directory), while the `-user` build installs into
; %LOCALAPPDATA%\Programs\ntc with a per-user PATH entry.
;
; The shared setup body lives in packaging\ntc_install.issi.

#define AppVer "2.4.8"

#ifdef USERINSTALL
  #define OutputSuffix "-user"
#else
  #define OutputSuffix ""
#endif

#define ExeSource  "target\release\ntc.exe"
#define DllGlob    "target\release\*.dll"
#define DepsGlob   "target\release\deps\*.dll"
#define OutputBase "ntc-installer-" + AppVer
#define MinOS ""

#include "packaging\ntc_install.issi"
