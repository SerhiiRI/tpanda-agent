; Script generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

#define AppName "Trashpanda Build Agent"
#define AppVersion "0.1.0"
#define AppPublisher "Trashpanda Team"
#define AppURL "http://trashpanda-team.ddns.net"
#define AppExeName "tpanda-agent.exe"
#define AppExeNameService "tpanda-agent"

[Setup]
AppId={{15E20F09-2517-4AC4-B996-3E135AB5C2CA}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
AppPublisherURL={#AppURL}
AppSupportURL={#AppURL}
AppUpdatesURL={#AppURL}
DefaultDirName={autopf}\{#AppName}
DisableProgramGroupPage=yes
OutputDir=target\installer
OutputBaseFilename=Setup agent
SetupIconFile=static\toolbox.ico
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "polish"; MessagesFile: "compiler:Languages\Polish.isl"
Name: "ukrainian"; MessagesFile: "compiler:Languages\Ukrainian.isl"

[Files]
Source: "target\release\tpanda-agent.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "build-paths"; DestDir: "{app}"; Flags: ignoreversion
Source: "build-paths.example"; DestDir: "{app}"; Flags: ignoreversion
Source: "static\shawl-32.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "static\toolbox.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\{#AppExeName}"

[Run]
Filename: {app}\shawl-32.exe; Parameters: "add --name {#AppExeNameService} -- ""{app}\tpanda-agent.exe""" ; Flags: runhidden
Filename: {sys}\sc.exe; Parameters: "start {#AppExeNameService}" ; Flags: runhidden

[UninstallRun]
Filename: {sys}\sc.exe; Parameters: "stop {#AppExeNameService}" ; Flags: runhidden
Filename: {sys}\sc.exe; Parameters: "delete {#AppExeNameService}" ; Flags: runhidden