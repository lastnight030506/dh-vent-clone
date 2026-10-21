// Probe the server's Bảng 2.10 ψ table (ratio × z) for the clone's solar interpolation.
//   DH_CODE=... node scripts/probe-psi.mjs
import fs from 'node:fs';
import path from 'node:path';
import { unpack } from '../test/unpack.mjs';

const code = process.env.DH_CODE;
const API = 'https://dh-tools.huanbachkhoa.workers.dev';
const sleep = ms => new Promise(r => setTimeout(r, ms));

async function so(input) {
  const r = await fetch(API + '/api/vent/compute', {
    method: 'POST', headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ input, code, dev: 'oracledevice0001' }),
  });
  return unpack((await r.json()).res).SO;
}

// Đà Nẵng, July: Aq = 556.1276, aN = 23.26, AtN = tNh - tNtb = 5
const Aq = 556.1276, aN = 23.26, AtN = 5;
const table = {};
for (const ratio of [1, 2, 3, 5]) {
  table[ratio] = {};
  const rho = ratio * aN * AtN / Aq;
  for (const z of [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]) {
    const S = await so({ st: 'Đà Nẵng', tNh: 34.4,
      cons: [{ id: 'mai', name: 'Mái', aT: 8.72, aN: 23.26, layers: [{ n: 'Tôn', d: 5, lam: 58, s: 126.2 }] }],
      els: [{ k: 'mai', dir: 'N', a: 10, b: 10, n: 1, con: 'mai' }],
      sol: { m: '7', rho, z, nuMode: 'input', nu: '4' } });
    table[ratio][z] = S.psi;
    await sleep(100);
  }
  process.stdout.write('.');
}
fs.writeFileSync(path.join(path.dirname(new URL(import.meta.url).pathname), '..', 'src', 'data', 'psi-table.json'), JSON.stringify(table, null, 1));
console.log('\n', JSON.stringify(table));
