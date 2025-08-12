# TinkerVerse — Dev Loop

This repo includes a simple **Windows** one-click dev runner and a **GitHub Actions CI** workflow.

## One-click local dev (Windows)

1) Double-click `dev.bat`
2) It will:
   - Auto-install `cargo-watch` if needed
   - Start the Rust server with auto-rebuild on file save
   - Serve the web client at `http://localhost:8000`
   - Open your browser

> Tip: Hard-refresh the client after changes (Ctrl+Shift+R).

### Manual commands
```bat
cd server
cargo watch -q -x run

cd ..\client2dweb
python -m http.server 8000
```

## CI (GitHub Actions)

The workflow at `.github/workflows/ci.yml` builds the server on every push/PR
using a clean Windows image and uploads dev artifacts.

## Suggested PR flow

```bat
git switch -c fix/short-description
REM (commit your changes)
git push -u origin fix/short-description
```

Open a Pull Request on GitHub → CI will compile and attach artifacts.

## Client WebSocket URL

The client typically connects to: `ws://127.0.0.1:8080` during local dev.
If you host the server elsewhere, update the client accordingly.
