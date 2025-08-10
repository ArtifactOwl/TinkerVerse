# TinkerVerse — MVP with Mining + Fabricator (JSON path)

Brand rename ✅, plus a playable loop:
- **Mining**: resource asteroid (C / H₂O / N) with proximity extraction
- **Inventory** per ship (C, H₂O, N) and **Food (kg)**
- **Fabricator** demo: synth **1 kg Food** from C + N + H₂O
- **HUD**: Mine / Stop / Print Food; live inventory
- 2D client: **Y-up**, smooth camera, **WASD/Arrows**

## Quickstart
Server:
```bash
cd server
cargo run
```
Client (new terminal):
```bash
cd client-2d-web
python -m http.server 5500
```
Open http://localhost:5500 and fly near the **orange asteroid** → click **Mine**. Watch C/H₂O/N rise, then click **Print Food**.

## Repo rename notes
- Package: `tinkerverse-server`
- Web title and logs use "TinkerVerse"
- Protocol files kept as `protocol/astral.fbs` and `protocol/world.fbs` for now. (We can migrate names later when we switch to FlatBuffers on the wire.)

## Make a PR with this update
From your repo root:
```bash
git checkout -b feature/tinkerverse-mining-fab
# copy these files over the repo root (preserve .git/)
git add -A
git commit -m "TinkerVerse: mining + inventory + fabricator + HUD; brand rename"
git push -u origin feature/tinkerverse-mining-fab
```
Then open a Pull Request from `feature/tinkerverse-mining-fab` → `main`.
