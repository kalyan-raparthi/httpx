[![Powered by Rust](https://img.shields.io/badge/powered_by-Rust-orange.svg)](https://www.rust-lang.org/)
## HTTPX - RUST http_server
Supports HTTP/1.1

### Upcoming Features
- Daemon Generation
- User and Log Management

## Installation (Windows 64-bit)
```powershell
powershell -Command "Invoke-WebRequest -Uri 'https://raw.githubusercontent.com/kalyan-raparthi/httpx/refs/heads/main/install/install.cmd' -OutFile 'C:\Program Files\install.cmd'";
Start-Process -FilePath 'C:\Program Files\install.cmd';
```

## File Structure
```
/rust
    └── /kit
            ├── core.rs
            ├── mod
            └── net_ctl

/install
    ├── install.cmd
    └── win64_installer_0.1.0.exe

/bin
    ├── _win64_0.1.0.exe
    └── install.rs
```

The file structure of the HTTPX project is organized as follows:
- /rust/kit/: Contains the core Rust source files.
    - `core.rs`: Core functionalities of the HTTP server.
    - `mod`: Module definitions.
    - `net_ctl`: Network control utilities.

- `/install/`: Contains installation scripts and executables.
    - `install.cmd`: Command script for installation.
    - `win64_installer_0.1.0.exe`: Windows 64-bit installer executable.

- `/bin/`: Contains compiled binaries and additional scripts.
    - `httpx/_win64_0.1.0.exe`: Compiled HTTPX executable for Windows 64-bit.
    - `install.rs`: Rust script for installation.

## 
<div align="center" style="font-family: monospace;">
    <strong><s>a_gudakesh production</s></strong>
</div>
