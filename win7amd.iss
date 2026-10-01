; win7amd.iss — Inno Setup script for the Windows 7 compatible ntc build.
; Same install identity as installer.iss (same AppName, install dir, PATH
; entries) so it installs/upgrades in place with the regular installer.
; Differences from installer.iss:
;   - bundles target\win7-amd64\release\ntc.exe (getrandom windows_legacy
;     backend: RtlGenRandom via advapi32, works on Win7 SP1+; the regular
;     build imports ProcessPrng from bcryptprimitives.dll, Win10+ only).
;     Build it first with: ran win7amd
;   - OutputBaseFilename carries the -win7-x64 suffix.
;   - MinVersion documents the Win7 SP1 floor.
; Compile with Inno Setup: iscc win7amd.iss

[Setup]
AppName=ntc
AppVersion=2.4.4
AppPublisher=NuengCoder
AppPublisherURL=https://github.com/NuengCoder/ntc
DefaultDirName={autopf}\ntc
DefaultGroupName=ntc
OutputDir=.\installer
OutputBaseFilename=ntc-installer-2.4.4-win7-x64
SetupIconFile=assets\ntc_image.ico
Compression=lzma
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=6.1sp1
PrivilegesRequired=admin
PrivilegesRequiredOverridesAllowed=dialog

[Files]
Source: "target\win7-amd64\release\ntc.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "target\win7-amd64\release\*.dll"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist
Source: "target\win7-amd64\release\*.dll"; DestDir: "{app}\dll"; Flags: ignoreversion skipifsourcedoesntexist
Source: "target\win7-amd64\release\deps\*.dll"; DestDir: "{app}\dll"; Flags: ignoreversion skipifsourcedoesntexist

[Dirs]
Name: "{app}\dll"

[Icons]
Name: "{group}\ntc"; Filename: "{app}\ntc.exe"; Comment: "Navigate, Toolkit, Center - Interactive shell"
Name: "{group}\Uninstall ntc"; Filename: "{uninstallexe}"
Name: "{autodesktop}\ntc"; Filename: "{app}\ntc.exe"; Comment: "Navigate, Toolkit, Center - Interactive shell"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional icons:"

[Registry]
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; \
    ValueData: "{olddata};{app}"; Check: NeedsAddUserPath(ExpandConstant('{app}'))

Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; ValueName: "Path"; \
    ValueData: "{olddata};{app}"; Check: NeedsAddSystemPath(ExpandConstant('{app}'))

[Run]
Filename: "{app}\ntc.exe"; Parameters: "--version"; Flags: runhidden; Description: "Verify installation";

[Code]
function IsVCRedistInstalled: boolean;
var
  ResultCode: integer;
begin
  Result := Exec('cmd.exe', '/c "where /q vcruntime140.dll"', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
  if not Result then
    Result := FileExists(ExpandConstant('{syswow64}\vcruntime140.dll')) or
              FileExists(ExpandConstant('{sysnative}\vcruntime140.dll')) or
              FileExists(ExpandConstant('{win}\System32\vcruntime140.dll'));
end;

function NeedsAddUserPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKCU, 'Environment', 'Path', OrigPath) then
  begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + UpperCase(Param) + ';', ';' + UpperCase(OrigPath) + ';') = 0;
end;

function NeedsAddSystemPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKLM, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', OrigPath) then
  begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + UpperCase(Param) + ';', ';' + UpperCase(OrigPath) + ';') = 0;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    SaveStringToFile(ExpandConstant('{tmp}\ntc_path_update.txt'), 'PATH updated', False);
  end;
end;
