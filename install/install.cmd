@echo off
REM Check for administrative privileges
openfiles >nul 2>&1

if %errorlevel% NEQ 0 (
    echo This script requires administrative privileges. Please run as administrator.
    exit /b 1
)

REM Deleting HTTPX folder if it already exists
if exist "C:\Program Files\HTTPX" (
    echo Deleting existing HTTPX folder...
    rmdir /s /q "C:\Program Files\HTTPX"
)

echo Creating HTTPX folder in Program Files
mkdir "C:\Program Files\HTTPX"

echo Adding config file
rem Adding config.txt
echo "$VERSION V0\n&IP localhost\n&PORT 21" > "C:\Program Files\HTTPX\config.txt"

REM echo Downloading HTTPX...
REM powershell -Command "Invoke-WebRequest -Uri 'https://github.com/kalyan-raparthi/httpx/raw/refs/heads/main/bin/httpx_win64_0.1.0.exe' -OutFile 'C:\Program Files\HTTPX\httpx.exe'"

if %errorlevel% NEQ 0 (
    echo Download failed. Please check your network connection.
    exit /b 1
)

echo Adding HTTPX to PATH
setx PATH "%PATH%;C:\Program Files\HTTPX"

echo Installation is now complete.
echo To use HTTPX, open a new command prompt and type 'httpx --help'
pause
