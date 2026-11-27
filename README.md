# Realms of Will — Infinite 3D Isometric Kingdom Engine & Asset Manager

An open-world, zero-install browser kingdom engine built with **Three.js Orthographic Isometric 3D**, **GPU Instancing (`THREE.InstancedMesh`)**, a **WebAssembly (`sim_core.wasm`) SoA Physics + Spatial Hash Core**, and **Simulated Quantum Annealing (SQA)** terrain/settlement placement.

## Live Entry Points
* **`index.html` / `open_world.html`** — Infinite streaming 3D open world (smooth heightfield terrain, cascading waterfalls, Quaternius medieval castles & villages, animated dragons, 3-tier crowd LOD, up to 5,000 physics items & 2,500 spatial-hashed NPCs).
* **`realm_prototype.html`** — Isometric Kingdom Courtyard with playable realistic heroes (`King`, `Knight`, `Ranger`, `Adventurer`, `Peasant`) and bone-attached weapons.
* **`asset_inspector.html`** — 3D Entity Curator for browsing, animating, and marking Keep/Drop across **3,177 CC0 3D assets**.
* **`sim_core.wasm` & `rust_engine/`** — Zero-GC linear-memory physics, value-noise terrain, and $O(1)$ spatial-hash crowd steering.

## Using This Repo as a GitHub Asset CDN
All 3D models live under `assets_3d/` and are indexed in `assets_3d/catalog.json`:
```
https://raw.githubusercontent.com/prashanth-karanam/realms-of-will/main/assets_3d/<path>
```
Or serve directly via GitHub Pages once enabled!

## Asset Licenses (CC0 1.0 Public Domain)
All 3D assets inside `assets_3d/` are sourced from free **CC0 1.0 Public Domain** creators:
* **Quaternius** (`assets_3d/quaternius/`) — Characters, Fantasy Races, Modular Outfits, Medieval Village & Props MegaKits, RTS Castles, Animals, Nature
* **Poly Pizza CC0** (`assets_3d/poly_pizza/`) — Dragons, Kings, Knights, Warriors, Orcs, Wizards, Giants
* **KayKit** (`assets_3d/kaykit/`) — Resource Bits, Weapons, Forest, Dungeon, Medieval Hexagon & Builder Packs
* **Kenney** (`assets_3d/kenney/`) — Castle Kit, Fantasy Town Kit, Nature Kit, Graveyard & Pirate Kits
