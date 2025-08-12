@echo off
setlocal EnableExtensions EnableDelayedExpansion
title TinkerVerse Dev

REM Root of the repo (this script lives here)
set "ROOT=%~dp0"
for %%I in ("%ROOT:~0,-1%") do set "ROOT=%%~fI"

REM Paths
set "SERVER=%ROOT%\server"
set "CLIENT=%ROOT%\client2dweb"

if not exist "%SERVER%\Cargo.toml" (
  echo [X] Could not find server\Cargo.toml under "%SERVER%"
  pause
  exit /b 1
)
if not exist "%CLIENT%\index.html" (
  echo [X] Could not find client2dweb\index.html under "%CLIENT%"
  pause
  exit /b 1
)

REM Install cargo-watch once
where cargo-watch >NUL 2>&1
if errorlevel 1 (
  echo [+] Installing cargo-watch (first run only)...
  cargo install cargo-watch
)

REM Start the Rust server (auto-rebuild on save) in a new window
start "TV Server" cmd /k "cd /d %SERVER% && cargo watch -q -x run"

REM Serve the web client
where python >NUL 2>&1
if not errorlevel 1 (
  start "TV Client" cmd /k "cd /d %CLIENT% && python -m http.server 8000"
) else (
  echo [!] Python not found; starting a simple PowerShell static server on :8000
  start "TV Client" powershell -NoLogo -NoProfile -Command ^
    "$h=New-Object Net.HttpListener; $h.Prefixes.Add('http://+:8000/'); $h.Start(); Write-Host 'Serving http://localhost:8000 from %CLIENT%'; Set-Location '%CLIENT%'; while ($true){ $ctx=$h.GetContext(); $p=Join-Path (Get-Location) ($ctx.Request.Url.LocalPath.TrimStart('/')); if(Test-Path $p){ $b=[IO.File]::ReadAllBytes($p) } else { $b=[Text.Encoding]::UTF8.GetBytes('<h1>Not Found</h1>') }; $ctx.Response.OutputStream.Write($b,0,$b.Length); $ctx.Response.Close() }"
)

REM Open the browser
start "" http://localhost:8000

echo [+] Server + client launched. Close windows to stop.
exit /b 0
