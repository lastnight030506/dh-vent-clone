# Engine contract

The UI is unchanged: it calls `APP.compute(input)` and expects the same response
envelope the web API used to return. In the desktop app that call is a Tauri
command, not HTTP.

## Command

```js
window.__TAURI__.core.invoke('compute', { input })
```

`input` is the UI's project object (`S`). Response:

```json
{ "mode": "free", "full": true, "teach": false,
  "cfg": { "modes": { "tP": "off", "t7": "off", "t8": "off", "t9": "off" }, "tol": 3 },
  "res": { "C": {}, "E": {}, "LH": {}, "LD": {}, "GH": {}, "GD": {},
           "SO": {}, "V": {}, "H": {}, "FN": null, "P": {}, "CK": {} } }
```

`engine::compute_value` returns `res` or an `Err(String)`; the shell surfaces the
error as a rejected promise.

## Input

The project object is serialised as-is. Fields the engine reads:

| Area | Fields |
| --- | --- |
| Climate | `st`, overrides `tNh phiH tNtb tNd phiD vH vD dirH dirD`, `dTh`, `tTd`, `labor` |
| Envelope | `bA bB bH`, `floorOn`, `floorIns[{d,lam}]`, `roofSummer`, `cons[{id,aT,aN,layers[{d,lam,s}]}]`, `els[{k,dir,a,b,n,con,psi,inf,cr}]` |
| Infiltration | `infL = {h:{cuaso,cuadi,cuamai}, d:{...}}` metres of crack, per season |
| Gains | `light{mode,a,F,e1,e2,kw}`, `phiM`, `motors[{n,kW}]`, `nPeople`, `others[{t,name,Q,G,Qp,eta,C,td,tc,beta}]` |
| Solar | `sol{st,m,slot,t1..t4,q{dir},qday,qmax,psi,z,nuMode,nu,rho}` |
| Flow | `vent{tvlv,a,h0,hv,tV,nOut}` |

Numeric cells may be numbers, numeric strings, or `""` (empty). Unknown station
names and an empty `st` fall back to Hà Nội. Unknown fields are ignored.

## Output sections

- `C` — resolved climate: `tNh tNd tNtb tTh tTd vH vD dirH dirD`, moisture
  `dH dD`, `warn[]` (t_T > 34 °C, no cold season).
- `E` — envelope rows with net area after openings, `con` (K, D, layers),
  floor bands 1–4, `warn[]` for rows without a structure.
- `LH` / `LD` — losses per season: `rows[]` (`Qk`, `Qb` by orientation),
  `infRows[]` (crack length, `g`, `Q`), `tot`. Winter returns `skip:true` and
  zero totals when `tNd >= tTd`.
- `GH` / `GD` — gains per season: lighting, motors, people, `others`.
- `SO` — solar: station, month, transmittance product, per-direction windows,
  roof absorption (`psi`, `nu`, `Attg`, `QA`). `psiPH` is the interpolated ψ
  even when the user typed one.
- `V` — excess heat, airflow `G`, `L` (m³/h), air changes `n`.
- `H`, `FN`, `P`, `CK` — duct plan/fan/check outputs. Empty stubs; those tabs
  are off in the live app for this project and return the same shape.

Fields that the JavaScript engine omitted (`undefined`) are omitted here; fields
that were explicitly `null` remain `null`. Parity tests compare key sets exactly.

## Module map

| File | Responsibility |
| --- | --- |
| `engine/src/climate.rs` | station lookup, overrides, design temperatures, warnings |
| `engine/src/envelope.rs` | layer resistance `Ro/K/D`, element net areas, floor bands |
| `engine/src/losses.rs` | transmission + infiltration losses for one season |
| `engine/src/gains.rs` | lighting/motors/people/other gains for both seasons |
| `engine/src/solar.rs` | solar stations, day length, ψ/ν, roof and window gains |
| `engine/src/vent.rs` | excess heat, airflow, air changes |
| `engine/src/util.rs` | JS-compatible numeric parsing, windward, ψ interpolation |
| `engine/src/data.rs` | embedded JSON tables, parsed once |
| `engine/src/input.rs` / `output.rs` | serde contract |

## Parity harness

`cargo test -p engine` runs `engine/tests/parity.rs` over every
`fixtures/*.input.json`, comparing with `*.expected.json` at
`max(1e-9 absolute, 1e-6 relative)` and exact key sets.

Fixtures whose `.raw.json` is absent can be re-captured with
`DH_CODE=... node tools/oracle.mjs <fixture>`; `DH_DEV` overrides the device id.
