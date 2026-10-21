# API and engine contract

The client (`public/index.html`) only talks to three endpoints. Responses are
plain JSON; the client's `DHV.unpack` passes already-plain objects through, so
the local server returns the engine result directly (the live server sends a
`§r`/`§g` packed form instead).

## POST /api/vent/compute

```json
{ "input": { ...project state... }, "code": "optional", "dev": "optional" }
```

Response:

```json
{ "mode": "free", "full": true, "teach": false,
  "cfg": { "modes": { "tP": "off", "t7": "off", "t8": "off", "t9": "off" }, "tol": 3 },
  "res": { "C": {}, "E": {}, "LH": {}, "LD": {}, "GH": {}, "GD": {},
           "SO": {}, "V": {}, "H": {}, "FN": null, "P": {}, "CK": {} } }
```

On a thrown error the server returns the same envelope plus `error` (string)
and no `res`; the client shows a toast.

`GET /api/vent/meta` returns the same envelope, minus `res`/`error`.
`POST /api/check-code` returns `{"status":"valid", ...}` locally; the live app
uses it to unlock a licence.

## Input

The project object is the client's `S`. Fields the engine reads:

| Area | Fields |
| --- | --- |
| Climate | `st`, overrides `tNh phiH tNtb tNd phiD vH vD dirH dirD`, `dTh`, `tTd`, `labor` |
| Envelope | `bA bB bH`, `floorOn`, `floorIns[{d,lam}]`, `roofSummer`, `cons[{id,aT,aN,layers[{d,lam,s}]}]`, `els[{k,dir,a,b,n,con,psi,inf,cr}]` |
| Infiltration | `infL = {h:{cuaso,cuadi,cuamai}, d:{...}}` metres of crack, per season |
| Gains | `light{mode,a,F,e1,e2,kw}`, `phiM`, `motors[{n,kW}]`, `nPeople`, `others[{t,name,Q,G,Qp,eta,C,td,tc,beta}]` |
| Solar | `sol{st,m,slot,t1..t4,q{dir},qday,qmax,psi,z,nuMode,nu,rho}` |
| Flow | `vent{tvlv,a,h0,hv,tV,nOut}` |

Unknown station names fall back to Hà Nội, as on the live server.

## Engine sections

- `C` — resolved climate: `tNh tNd tNtb tTh tTd vH vD dirH dirD`, moisture
  `dH dD`, `warn[]` (t_T > 34 °C, no cold season).
- `E` — envelope rows with net area after openings, `con` (K, D, layers),
  floor bands 1–4, `warn[]` for rows without a structure.
- `LH` / `LD` — losses per season: `rows[]` (`Qk`, `Qb` by orientation),
  `infRows[]` (crack length, `g`, `Q`), `tot`. Winter returns `skip:true` and
  zero totals when `tNd >= tTd`.
- `GH` / `GD` — gains per season: lighting, motors, people, `others`.
- `SO` — solar: station, month, transmittance product, per-direction windows,
  roof absorption (`psi`, `nu`, `Attg`, `QA`), `psiPH` is the interpolated ψ
  even when the user typed one.
- `V` — excess heat, airflow `G`, `L` (m³/h), air changes `n`.
- `H`, `FN`, `P`, `CK` — duct plan/fan/check outputs. Empty stubs; the live
  server has these tabs off for this licence and returns the same shape.

## Parity harness

`test/compare.mjs <fixture> [relTol]` deep-compares `compute(input)` with an
unpacked live response. `test/all.mjs` runs every fixture. Fixtures whose
`.raw.json` is absent can be re-captured with
`DH_CODE=... node test/oracle.mjs <fixture>`; `DH_DEV` overrides the device id.

Recovered formulas, constants and defaults are documented in the engine source
next to their use (R_o/K, floor bands, wind uplift, infiltration table 2.5,
q_n curves, day length, ψ interpolation).
