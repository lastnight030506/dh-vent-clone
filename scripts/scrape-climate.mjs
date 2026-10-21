// Harvest site data that the page does not ship: per-station climate (QCVN 02:2022 A.2/A.3/A.4/A.10/A.16)
// and the A.18 daily solar total per month.
//   DH_CODE=... node scripts/scrape-climate.mjs
import fs from 'node:fs';
import path from 'node:path';
import { unpack } from '../test/unpack.mjs';

const root = path.join(path.dirname(new URL(import.meta.url).pathname), '..');
const html = fs.readFileSync(path.join(root, 'index.html'), 'utf8');
const m = html.match(/const M = (\{.*?\});\n/s);
if (!m) throw new Error('M catalog not found');
const M = JSON.parse(m[1]);

const API = process.env.DH_API || 'https://dh-tools.huanbachkhoa.workers.dev';
const code = process.env.DH_CODE;
if (!code) { console.error('set DH_CODE'); process.exit(1); }

const sleep = ms => new Promise(r => setTimeout(r, ms));
async function compute(input) {
  const r = await fetch(API + '/api/vent/compute', {
    method: 'POST', headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ input, code, dev: process.env.DH_DEV || 'oracledevice0001' }),
  });
  const d = JSON.parse(await r.text());
  if (d.error) throw new Error(d.error);
  return unpack(d.res);
}

const outFile = path.join(root, 'src', 'data', 'climate.json');
fs.mkdirSync(path.dirname(outFile), { recursive: true });
const out = fs.existsSync(outFile) ? JSON.parse(fs.readFileSync(outFile, 'utf8')) : { stations: {}, solar: {} };

for (const [name, region] of M.data.clim) {
  if (out.stations[name]) continue;
  const R = await compute({ st: name, sol: { m: '7' } });
  const C = R.C;
  out.stations[name] = {
    region, lat: C.lat, lon: C.lon, z: C.z,
    tNh: C.tab.tNh, phiH: C.tab.phiH, tNtb: C.tab.tNtb, tNd: C.tab.tNd, phiD: C.tab.phiD,
    vH: C.tab.vH, dirH: C.tab.dirH, vD: C.tab.vD, dirD: C.tab.dirD,
    wind: { h: C.wH, d: C.wD },
    s18: R.SO.s18, s18D: R.SO.s18D,
  };
  fs.writeFileSync(outFile, JSON.stringify(out, null, 1));
  process.stdout.write('.');
  await sleep(120);
}
console.log('\nstations:', Object.keys(out.stations).length);

// A.18 station list, then 12 months each
const s18set = [...new Set(Object.values(out.stations).map(s => s.s18).filter(Boolean))];
const solar = {};
for (const st of s18set) {
  solar[st] = {};
  for (let mo = 1; mo <= 12; mo++) {
    const R = await compute({ st, sol: { m: String(mo) } });
    solar[st][mo] = { qday: R.SO.qday, qmax: R.SO.qmax, N: R.SO.N };
    await sleep(120);
  }
  process.stdout.write('s');
}
out.solar = solar;
fs.writeFileSync(outFile, JSON.stringify(out, null, 1));
console.log('\nsolar stations:', Object.keys(solar).length);
