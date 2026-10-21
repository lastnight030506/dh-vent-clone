# DH-VENT clone

Offline clone of the Vietnamese industrial-ventilation calculator at
`https://dh-tools.huanbachkhoa.workers.dev/vent` (DH Engineering Tools, DH-VENT).

The live page ships the whole UI but keeps every calculation on the server.
This repo reimplements that server side (`src/engine.js`), serves the original
client bytes unchanged, and reproduces the API, so the app runs locally with no
network and no access code.

## Status

- Engine output matches the live endpoint on 11 captured scenarios (843, 673,
  427, … fields each, relative tolerance 1e-6) — see `test/fixtures/`.
- The page loads in Chromium against the local API and renders results
  (checked end to end with `R.V.L = 81743.6596…` for `scenario1`).
- Duct tabs (`tP`, `t7`, `t8`, `t9`) are switched off exactly as on the live
  app for this licence; that engine is not reimplemented.
- Licensing is not reproduced: every access code is accepted locally.

## Run

No dependencies needed:

```sh
node dev.mjs            # http://localhost:8787/vent
```

Deploy to Cloudflare Workers (static page from `public/`, API from `worker.js`):

```sh
npm install && npx wrangler deploy
```

## Layout

```
public/index.html   original client, byte-for-byte
src/engine.js       ventilation engine (climate, envelope, gains, solar, flow)
src/api.js          /api/vent/meta, /api/vent/compute, /api/check-code
worker.js           Worker entry (API only; assets serve the page)
dev.mjs             local Node server with the same API
src/data/*.json     harvested station/climate/ψ tables
src/data/*.js       generated ES modules (scripts/build-data.mjs)
scripts/            data harvest + probe tools (need DH_CODE)
test/               oracle capture, unpacker, deep-diff, fixtures
docs/CONTRACT.md    request/response and engine-field contract
```

```sh
node test/all.mjs                              # all fixtures vs engine
DH_CODE=... node test/oracle.mjs <name>        # re-capture from the live API
```

`test/fixtures/*.expected.json` are unpacked live responses; `*.input.json`
are the inputs sent to the engine.

## Provenance

- Station normals: QCVN 02:2022/BXD tables A.2/A.3/A.4/A.10/A.16 (152 stations).
- Solar: daily totals per month for the 13 radiation stations (A.18).
- ψ: Bảng 2.10 grid, interpolated bilinearly.
- Formulas and defaults were recovered from the page's calculation notes and
  by probing the live compute endpoint; `docs/CONTRACT.md` records the result.

Original app, data and texts © Nguyễn Đình Huấn (ndhuan@dut.udn.vn). This is a
private study copy, not affiliated with or endorsed by the author.
