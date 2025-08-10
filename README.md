# Astral (Phase 0/1 Starter)

This bundle gives you two paths:
- **Works now:** JSON-over-WebSocket server + simple 2D web client.
- **Upgrade path:** FlatBuffers-ready protocol and an alternate server (`net_fb.rs`) + client stub you can enable after running `flatc`.

## Quickstart (JSON path — no FlatBuffers needed)

1. Install Rust: https://rustup.rs
2. Run the server:
   ```bash
   cd server
   cargo run
   ```
   You should see `astral-server listening ws://127.0.0.1:8080`.
3. Open the 2D client:
   - Double-click `client-2d-web/index.html` to open in your browser.
   - Use **WASD/Arrow keys** to thrust. Mouse wheel to zoom.

If you see nothing: make sure the server is running and your firewall permits localhost:8080.

## FlatBuffers path (faster wire format; optional)

1. Install the FlatBuffers compiler (`flatc`):  
   - macOS: `brew install flatbuffers`  
   - Ubuntu/Debian: `sudo apt-get install flatbuffers-compiler`  
   - Windows (Chocolatey): `choco install flatc`

2. Generate Rust code into the server:
   ```bash
   cd astral
   flatc --rust -o server/src protocol/astral.fbs protocol/world.fbs
   ```

3. Switch the server to the FlatBuffers netcode (two options):
   - **Simple:** Edit `server/src/main.rs` and replace
     ```rust
     mod net_json; use net_json as net;
     ```
     with
     ```rust
     mod net_fb; use net_fb as net;
     mod astral_generated; // created by flatc
     mod world_generated;  // created by flatc
     ```
   - Then run `cargo run` again.

4. (Optional) Web client with FlatBuffers:
   - Generate JS or TS from the schemas:
     ```bash
     flatc --js -o client-2d-web/protocol protocol/astral.fbs protocol/world.fbs
     ```
   - Add the FlatBuffers JS runtime to the page (CDN or local copy), then use `client-2d-web/main_fb.js` as a reference.
   - For now, `index.html` + `main.js` (JSON) is the easiest path.

## Project layout

- `protocol/` — FlatBuffers schemas (wire + worldgen)
- `server/` — Rust authoritative fixed-tick sim
  - `net_json.rs` — JSON snapshots for the simple web client (default)
  - `net_fb.rs` — FlatBuffers netcode (enable after running `flatc`)
- `client-2d-web/` — Minimal 2D symbol client (JSON by default)
- `tools/replayer/` — Placeholder for a determinism replayer

Have fun! This is intentionally tiny and deterministic, ready to expand into mining, crafting, combat, and more.
