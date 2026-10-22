# DH-VENT

Windows desktop app for the Vietnamese industrial-ventilation calculator
**DH-VENT** (DH Engineering Tools). The UI is the original web client; every
calculation runs in Rust, locally, with no network and no access code.

## What is inside

| Path | Purpose |
| --- | --- |
| `web/` | Original single-page UI. Licence/access-code screen removed; PDF libraries and fonts are vendored so exports work offline. |
| `engine/` | Rust calculation engine (climate, envelope, losses, gains, solar, airflow). |
| `src-tauri/` | Tauri 2 shell: one `compute` command, Windows installer metadata and icons. |
| `fixtures/` | 11 input/output pairs captured from the live endpoint. |
| `tools/` | Optional live-capture scripts (need an access code; not used by the app). |
| `docs/CONTRACT.md` | Input/output field contract and module map. |

## Status

- **11/11 oracle fixtures match** at relative tolerance `1e-6`
  (`cargo test -p engine`).
- Verified end to end in the Tauri app: the default project is computed by the
  Rust engine and rendered in the UI (Linux build under Xvfb).
- Duct tabs (`tP`, `t7`, `t8`, `t9`) are switched off exactly as in the live app
  for this project; their engine (`H`, `P`, `FN`, `CK`) remains a no-plan stub.
- No licence logic: `mode` is always `free`, every feature is unlocked.

## Build

Rust 1.77+.

```sh
cargo test -p engine      # parity suite
cargo run -p dh-vent      # desktop app (Linux needs webkit2gtk-4.1 and gtk3)
cargo tauri build         # installer for the current OS
```

Windows installers are built by GitHub Actions on every push to `main`
(`.github/workflows/windows.yml`). Download the `dh-vent-windows` artifact from
the run, or build on Windows with:

```sh
cargo install tauri-cli --version "^2" --locked
cargo tauri build
```

Outputs: `target/release/bundle/msi/DH-VENT_1.0.0_x64_en-US.msi` and
`target/release/bundle/nsis/DH-VENT_1.0.0_x64-setup.exe`.

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

Original app, data and texts © Nguyễn Đình Huấn (ndhuan@dut.udn.vn). This is a
private study copy, not affiliated with or endorsed by the author.
