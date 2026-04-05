# brief-sv
A minimal cli app that hosts a single file in your local network ( e.g. WIFI ).

## Installation
``` bash
cargo install --git https://github.com/mtugb/brief-sv
```
## Usage
```bash
brief-sv file.md
```
A QR code will appear in your terminal. Scan it from any device on the same Wi-Fi.

`.md` files are automatically rendered as HTML.

Default port is `7878`. You can change it with `-p` option, but make sure the port is open in your firewall.
## WSL/WSL2 Users
Extra setup is required:
### Step 1
Run `setup-firewall.bat` as Admin in windows side.
The file can be found in my repo.
Or you can also pastes the code below in cmd (admin mode).
```setup-firewall.bat
@echo off
netsh advfirewall firewall delete rule name="brief-sv" >nul 2>&1
netsh advfirewall firewall add rule name="brief-sv" dir=in action=allow protocol=TCP localport=7878
netsh interface portproxy delete v4tov4 listenport=7878 listenaddress=0.0.0.0 >nul 2>&1
for /f %%i in ('wsl hostname -I') do set WSL_IP=%%i
netsh interface portproxy add v4tov4 listenport=7878 listenaddress=0.0.0.0 connectport=7878 connectaddress=%WSL_IP%
echo Done.
pause
```
### Step 2
Run with `--host` option:
```bash
brief-sv file.md --host 192.168.x.x
```
Find your Windows IP by running `ipconfig` in Windows Terminal
and looking for an address starting with `192.168`.
