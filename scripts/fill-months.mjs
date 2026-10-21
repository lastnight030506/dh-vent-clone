import fs from 'node:fs'; import path from 'node:path'; import { unpack } from '../test/unpack.mjs';
const root = path.join(path.dirname(new URL(import.meta.url).pathname), '..');
const f = path.join(root,'src','data','climate.json');
const C = JSON.parse(fs.readFileSync(f,'utf8'));
const code = process.env.DH_CODE;
for (const [st,s] of Object.entries(C.stations)) {
  if (s.mH) continue;
  const r = await fetch('https://dh-tools.huanbachkhoa.workers.dev/api/vent/compute',{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify({input:{st},code,dev:'oracledevice0001'})});
  const R = unpack((await r.json()).res);
  s.mH = R.C.mH; s.mD = R.C.mD; delete s.s18; delete s.s18D;
  fs.writeFileSync(f, JSON.stringify(C,null,1)); process.stdout.write('.');
  await new Promise(r=>setTimeout(r,70));
}
console.log('\nmonths done', Object.keys(C.stations).length);
