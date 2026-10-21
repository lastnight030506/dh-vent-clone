// Run every fixture through test/compare.mjs.
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';

const dir = new URL('./fixtures/', import.meta.url).pathname;
const names = fs.readdirSync(dir)
  .filter(f => f.endsWith('.input.json'))
  .map(f => f.slice(0, -'.input.json'.length));

let failed = 0;
for (const name of names) {
  if (spawnSync(process.execPath, ['test/compare.mjs', name], { stdio: 'inherit' }).status) failed++;
}
process.exit(failed ? 1 : 0);
