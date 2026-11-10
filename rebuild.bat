@echo off
chcp 65001 >nul
setlocal
rem Rebuild pot from sources and restart it.
rem Usage: rebuild.bat        - full build (exe + installers)
rem        rebuild.bat nobundle - fast build (exe only, no installers)

set PATH=%USERPROFILE%\.cargo\bin;%PATH%
cd /d "%~dp0"

set BUNDLE_ARG=
if /i "%~1"=="nobundle" set BUNDLE_ARG=--no-bundle

echo [1/4] Closing pot...
taskkill /F /IM pot.exe >nul 2>&1

where pnpm >nul 2>&1
if errorlevel 1 (
    echo ERROR: pnpm not found in PATH
    pause
    exit /b 1
)
where cargo >nul 2>&1
if errorlevel 1 (
    echo ERROR: cargo not found. Install rustup or add .cargo\bin to PATH
    pause
    exit /b 1
)

echo [2/4] Building...
set TAURI_ARGS=
if defined BUNDLE_ARG set TAURI_ARGS=--bundles none
call pnpm tauri build %TAURI_ARGS%
if errorlevel 1 (
    echo.
    echo BUILD FAILED. Check free disk space and messages above.
    pause
    exit /b 1
)

echo [3/4] Artifacts:
dir /b "src-tauri\target\release\pot.exe"
if not defined BUNDLE_ARG dir /b "src-tauri\target\release\bundle\nsis\pot_*_x64-setup.exe"

echo [4/4] Starting fresh build...
start "" "src-tauri\target\release\pot.exe"

echo Done.
pause
