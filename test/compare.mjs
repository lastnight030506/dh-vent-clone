// Deep-compare engine output with an oracle fixture.
//   node test/compare.mjs scenario1 [relTol]
import fs from 'node:fs';
import path from 'node:path';
import { compute } from '../src/engine.js';

const dir = path.dirname(new URL(import.meta.url).pathname);
const name = process.argv[2] || 'scenario1';
const tol = Number(process.argv[3] || 1e-6);
const input = JSON.parse(fs.readFileSync(path.join(dir, 'fixtures', name + '.input.json'), 'utf8'));
const exp = JSON.parse(fs.readFileSync(path.join(dir, 'fixtures', name + '.expected.json'), 'utf8'));
const got = compute(input);

let bad = 0, checked = 0;
const diffs = [];
function walk(a, b, p) {
  if (typeof a === 'number' && typeof b === 'number') {
    checked++;
    const d = Math.abs(a - b), rel = d / Math.max(1, Math.abs(a), Math.abs(b));
    if (d > 1e-9 && rel > tol) diffs.push(`${p}: expected ${a} got ${b} (Δ${d.toExponential(2)})`);
    return;
  }
  if (a && b && typeof a === 'object' && typeof b === 'object') {
    const keys = new Set([...Object.keys(a), ...Object.keys(b)]);
    for (const k of keys) walk(a[k], b[k], p ? p + '.' + k : k);
    return;
  }
  checked++;
  if (JSON.stringify(a) !== JSON.stringify(b)) diffs.push(`${p}: expected ${JSON.stringify(a)} got ${JSON.stringify(b)}`);
}
walk(exp, got, '');
if (diffs.length) {
  console.log(`MISMATCH ${diffs.length} / ${checked} fields`);
  diffs.slice(0, Number(process.env.SHOW || 40)).forEach(d => console.log(' ', d));
  process.exitCode = 1;
} else {
  console.log(`OK ${name}: ${checked} fields match (tol ${tol})`);
}
