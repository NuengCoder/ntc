[Setup]
AppName=ntc
AppVersion=1.5.0
AppPublisher=NuengCoder
AppPublisherURL=https://github.com/NuengCoder/ntc
DefaultDirName={autopf}\ntc
DefaultGroupName=ntc
OutputDir=.\installer
OutputBaseFilename=ntc-installer-1.5.0
SetupIconFile=assets\ntc_image.ico
Compression=lzma
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=admin
PrivilegesRequiredOverridesAllowed=dialog

[Files]
Source: "target\release\ntc.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\ntc"; Filename: "{app}\ntc.exe"
Name: "{group}\Uninstall ntc"; Filename: "{uninstallexe}"

[Registry]
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; \
    ValueData: "{olddata};{app}"; Check: NeedsAddUserPath(ExpandConstant('{app}'))

Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; ValueName: "Path"; \
    ValueData: "{olddata};{app}"; Check: NeedsAddSystemPath(ExpandConstant('{app}'))

[Run]
Filename: "{app}\ntc.exe"; Parameters: "--version"; Flags: runhidden; Description: "Verify installation";

[Code]
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