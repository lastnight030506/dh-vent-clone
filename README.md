# VENT

Desktop app for the Vietnamese industrial-ventilation calculator **VENT**.
Every calculation runs in Rust, locally, with no network and no access code.

## Download and install (Windows)

No coding required. Open the
[latest release](https://github.com/lastnight030506/dh-vent-clone/releases/latest),
download `VENT_1.0.0_x64-setup.exe`, and run it. If Windows SmartScreen appears,
choose **More info → Run anyway** — the installer is not code-signed yet.

Step-by-step guide (Vietnamese and English), including the `.msi` alternative:
[docs/INSTALL-WINDOWS.md](docs/INSTALL-WINDOWS.md).

> The Releases page is reachable only when this repository is public or you are
> a collaborator. To let anyone download the installer, make the repository
> public (Settings → General → Danger Zone → Change visibility).

## What is inside

| Path | Purpose |
| --- | --- |
| `web/` | Single-page UI. PDF libraries and fonts are vendored so exports work offline. |
| `engine/` | Rust calculation engine (climate, envelope, losses, gains, solar, airflow). |
| `src-tauri/` | Tauri 2 shell: one `compute` command, installer metadata and icons. |
| `fixtures/` | 11 input/output pairs captured from the live endpoint. |
| `tools/` | Optional live-capture scripts (need an access code; not used by the app). |
| `docs/CONTRACT.md` | Input/output field contract and module map. |

## Status

- **11/11 oracle fixtures match** at relative tolerance `1e-6`
  (`cargo test -p engine`).
- Verified end to end in the Tauri app: the default project is computed by the
  Rust engine and rendered in the UI (Linux build under Xvfb).
- Tabs 7–10 (`tP` plan, `t7` hydraulics, `t8` fan, `t9` export) are enabled.
  The plan editor, iso/net drawings, checks and the Word/Excel/PDF/DXF/PNG
  exports are client-side and work; the hydraulic numbers do not, because the
  live deployment switches those steps off in its own config (`/api/vent/meta`
  → `modes: tP/t7/t8/t9 = "off"`) and the server returns the segment tree with
  every value `null`. The engine keeps the empty stub (`H`, `P`, `FN`, `CK`).
- No licence logic: `mode` is always `free`, every feature is unlocked.

## Build

Rust 1.77+.

```sh
cargo test -p engine      # parity suite
cargo run -p vent         # desktop app (Linux needs webkit2gtk-4.1 and gtk3)
cargo tauri build         # installer for the current OS
```

Both platforms are built by GitHub Actions on every push to `main`
(`.github/workflows/build.yml`): `vent-linux` (deb, rpm, AppImage) and
`vent-windows` (msi, nsis) artifacts.

Outputs on Linux: `target/release/bundle/deb/VENT_1.0.0_amd64.deb`,
`target/release/bundle/rpm/VENT-1.0.0-1.x86_64.rpm`,
`target/release/bundle/appimage/VENT_1.0.0_amd64.AppImage`
(`cargo tauri build --bundles deb,rpm`). AppImage needs linuxdeploy's bundled
`strip` to understand the distribution's binaries; it works on the Ubuntu CI
runner but fails on Arch (`.relr.dyn` sections).

Outputs on Windows: `target/release/bundle/msi/VENT_1.0.0_x64_en-US.msi` and
`target/release/bundle/nsis/VENT_1.0.0_x64-setup.exe`
(`cargo tauri build --bundles msi,nsis`, needs the Windows WebView2 runtime,
installed by the bundle when missing).

## Publishing a release

Push a tag that matches `version` in `src-tauri/tauri.conf.json` (for example
`v1.0.0`). The build workflow compiles both platforms and attaches the Windows
`.exe`/`.msi` and the Linux `.deb`/`.rpm`/`.AppImage` to a GitHub release named
after the tag. Releases are downloadable by others only if the repository is
public.

## Extending

- **Add a calculation field**: add it to `engine/src/input.rs` (input) and
  `engine/src/output.rs` (result), then compute it in the matching module.
  Unknown input fields are ignored, so old clients keep working.
- **Add a section**: create `engine/src/<name>.rs`, wire it in `engine/src/lib.rs`
  and add the struct to `Output`. The UI reads results by key, so a new key can
  be rendered without touching the engine.
- **Update reference data**: edit `engine/data/{catalog,climate,psi-table}.json`.
  Data is embedded at compile time; no code generation step.
- **Recapture an oracle fixture**: `DH_CODE=... node tools/oracle.mjs <name>`
  writes `fixtures/<name>.{raw,expected}.json`; then run `cargo test -p engine`.

## Data and provenance

- Station normals: QCVN 02:2022/BXD tables A.2/A.3/A.4/A.10/A.16 (152 stations).
- Solar: daily totals per month for the 13 radiation stations (A.18).
- ψ: Bảng 2.10 grid, interpolated bilinearly.
- Formulas and defaults were recovered from the page's calculation notes and by
  probing the live compute endpoint; `engine/src` documents each stage.

Forked and rebranded from **DH-VENT** by Nguyễn Đình Huấn with the author's
permission (recorded here, 2026). Upstream: ndhuan@dut.udn.vn. This fork is not
affiliated with or endorsed by him beyond that permission.
