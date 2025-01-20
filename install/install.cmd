@echo off
REM Check for administrative privileges
openfiles >nul 2>&1

if %errorlevel% NEQ 0 (
    echo This script requires administrative privileges. Please run as administrator.
    exit /b 1
)

REM Download HTTPX from GitHub
echo Downloading HTTPX from GitHub...
powershell -Command "Invoke-WebRequest -Uri 'https://raw.githubusercontent.com/username/repository/branch/path/to/httpx.exe' -OutFile 'httpx.exe'"

REM Install HTTPX
echo Installing HTTPX...
mkdir "C:\Program Files\httpx" 2>nul
copy /y "httpx.exe" "C:\Program Files\HTTPX\httpx.exe"

REM Add HTTPX to system PATH
echo Adding HTTPX to system PATH...
setx PATH "%PATH%;C:\Program Files\HTTPX"

echo Installation complete.
pause