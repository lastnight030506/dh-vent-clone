// Probe engine defaults and lookup curves that are not in the page.
//   DH_CODE=... node scripts/probe-defaults.mjs
import fs from 'node:fs';
import path from 'node:path';
import { unpack } from '../test/unpack.mjs';

const code = process.env.DH_CODE;
const API = 'https://dh-tools.huanbachkhoa.workers.dev';
const sleep = ms => new Promise(r => setTimeout(r, ms));
async function compute(input) {
  const r = await fetch(API + '/api/vent/compute', {
    method: 'POST', headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ input, code, dev: 'oracledevice0001' }),
  });
  const d = await r.json();
  if (!d.res) throw new Error(JSON.stringify(d));
  return unpack(d.res);
}

const out = {};
// 1. months + qn curves
const months = {};
for (const st of ['Đà Nẵng', 'Hà Nội', 'Cần Thơ']) {
  const R = await compute({ st });
  months[st] = { mH: R.C.mH, mD: R.C.mD };
  await sleep(80);
}
out.months = months;

const qn = {};
for (const labor of ['nghi', 'nhe', 'vua', 'nang']) {
  qn[labor] = {};
  for (const t of [15, 18, 20, 22, 25, 28, 30, 32, 34, 35, 36]) {
    const R = await compute({ st: 'Đà Nẵng', tTd: t, labor });
    qn[labor][t] = R.GD.qn;
    await sleep(80);
  }
}
out.qn = qn;

// 2. defaults: empty light / missing fields / empty sol month
const R0 = await compute({ st: 'Đà Nẵng' });
out.defaults = {
  light: R0.GH.Nts, Fs: R0.GH.Fs, e1: R0.GH.e1, e2: R0.GH.e2, phi: R0.GH.phi,
  solM: R0.SO.m, slot: R0.SO.slot, hour: R0.SO.hour, tau: R0.SO.tau, rho: R0.SO.rho,
  ventA: R0.V.a, h0: R0.V.h0, hv: R0.V.hv, tR: R0.V.tR, tV: R0.V.tV, Cp: R0.V.Cp,
  nOut: R0.V.nOut, nOutPlan: R0.V.nOutPlan,
};
const R1 = await compute({ st: 'Đà Nẵng', light: { mode: 'kw', kw: 10 }, sol: { m: '1' }, vent: {} });
out.lightKw = { Nts: R1.GH.Nts, Qts: R1.GH.Qts };
out.jan = { qday: R1.SO.qday, qmax: R1.SO.qmax, N: R1.SO.N, month: R1.SO.m };

// 3. daylength samples
const dl = [];
for (const [st, m] of [['Hà Nội', '1'], ['Hà Nội', '7'], ['Cần Thơ', '1'], ['Cần Thơ', '7'], ['Đà Nẵng', '4'], ['Đà Nẵng', '10']]) {
  const R = await compute({ st, sol: { m } });
  dl.push({ st, m, lat: R.C.lat, N: R.SO.N, qday: R.SO.qday });
  await sleep(80);
}
out.daylength = dl;

fs.writeFileSync(path.join(path.dirname(new URL(import.meta.url).pathname), '..', 'src', 'data', 'probes.json'), JSON.stringify(out, null, 1));
console.log(JSON.stringify(out, null, 1));
