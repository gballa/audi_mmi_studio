; Audi MMI Studio — NSIS Windows Installer Script
; Architecture: x86_64 / Windows 10 & 11

!define PRODUCT_NAME "Audi MMI Studio"
!define PRODUCT_VERSION "0.1.0"
!define PRODUCT_PUBLISHER "Audi MMI Studio Engineering Team"
!define PRODUCT_WEB_SITE "https://github.com/gballa/audi_mmi_studio"
!define PRODUCT_EXE "mmi-studio-desktop.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
!define PRODUCT_UNINST_ROOT_KEY "HKCU"

SetCompressor /SOLID lzma

Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "..\dist-packages\windows\Audi-MMI-Studio-${PRODUCT_VERSION}-windows-setup.exe"
InstallDir "$LOCALAPPDATA\Programs\AudiMMIStudio"
InstallDirRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "InstallLocation"
RequestExecutionLevel user

Icon "..\src-tauri\icons\icon.ico"
UninstallIcon "..\src-tauri\icons\icon.ico"

Page directory
Page instfiles

UninstPage uninstConfirm
UninstPage instfiles

Section "MainSection" SEC01
    SetOutPath "$INSTDIR"
    SetOverwrite on
    
    File "..\..\..\target\release\${PRODUCT_EXE}"
    File "..\src-tauri\icons\icon.ico"

    ; Create Shortcuts
    CreateDirectory "$SMPROGRAMS\${PRODUCT_NAME}"
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\${PRODUCT_NAME}.lnk" "$INSTDIR\${PRODUCT_EXE}" "" "$INSTDIR\icon.ico" 0
    CreateShortCut "$DESKTOP\${PRODUCT_NAME}.lnk" "$INSTDIR\${PRODUCT_EXE}" "" "$INSTDIR\icon.ico" 0

    ; Write Uninstaller
    WriteUninstaller "$INSTDIR\uninst.exe"
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\Uninstall ${PRODUCT_NAME}.lnk" "$INSTDIR\uninst.exe"

    ; Add/Remove Programs Registry Entries
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayName" "${PRODUCT_NAME}"
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\uninst.exe"
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\icon.ico"
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
SectionEnd

Section "Uninstall"
    Delete "$DESKTOP\${PRODUCT_NAME}.lnk"
    Delete "$SMPROGRAMS\${PRODUCT_NAME}\*.*"
    RMDir "$SMPROGRAMS\${PRODUCT_NAME}"

    Delete "$INSTDIR\${PRODUCT_EXE}"
    Delete "$INSTDIR\icon.ico"
    Delete "$INSTDIR\uninst.exe"
    RMDir "$INSTDIR"

    DeleteRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}"
SectionEnd
