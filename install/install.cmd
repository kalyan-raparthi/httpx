REM author: Kalyan Raparthi, kalyan.rapathi@hotmail.com, GitHub: kalyan-raparthi
REM This script installs HTTPX on Windows.

@echo off
REM Check for administrative privileges
openfiles >nul 2>&1

if %errorlevel% NEQ 0 (
    echo This script requires administrative privileges. Please run as administrator.
    exit /b 1
)

REM CREATING HTTPX IN PROGRAMFILES
ECHO CREATING HTTPX IN PROGRAMFILES
cd C:\Program Files &  mkdir HTTPX;

echo '$VERSION V0\n&IP localhost\n&PORT 21' >> C:\Program Files\HTTPX\config.txt

REM Download HTTPX from GitHub
echo DOWNLOADING HTTPX
powershell -Command "Invoke-WebRequest -Uri 'https://github.com/kalyan-raparthi/httpx/raw/refs/heads/main/bin/httpx_win64_0.1.0.exe' -OutFile 'C:\Program Files\HTTPX\httpx.exe'"

REM Install HTTPX
echo INSTALLIN HTTPX


powershell -Command "Invoke-WebRequest -Uri 'https://github.com/kalyan-raparthi/httpx/raw/refs/heads/main/install/win64_installer_0.1.0.exe' -OutFile 'C:\Program Files\HTTPX\win64_installer_0.1.0.exe'"


REM Add HTTPX to system PATH
echo ADDING HTTPX TO PATH 
setx PATH "%PATH%;C:\Program Files\HTTPX"

echo INSTALLATION IS NOW COMPLETE.
echo To use HTTPX, open a new command prompt and type 'httpx --help'
pause
