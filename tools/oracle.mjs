// Query the upstream calculator API for a fixture input and store the raw response.
//   DH_CODE=... node tools/oracle.mjs scenario1
import fs from 'node:fs';
import path from 'node:path';
import { unpack } from './unpack.mjs';

const API = process.env.DH_API || 'https://dh-tools.huanbachkhoa.workers.dev';
const code = process.env.DH_CODE;
if (!code) { console.error('set DH_CODE (access code)'); process.exit(1); }

const name = process.argv[2] || 'scenario1';
const dir = path.dirname(new URL(import.meta.url).pathname);
const inputFile = path.join(dir, '..', 'fixtures', name + '.input.json');
const input = JSON.parse(fs.readFileSync(inputFile, 'utf8'));

const res = await fetch(API + '/api/vent/compute', {
  method: 'POST',
  headers: { 'content-type': 'application/json' },
  body: JSON.stringify({ input, code, dev: process.env.DH_DEV || 'oracledevice0001' }),
});
const text = await res.text();
fs.writeFileSync(path.join(dir, '..', 'fixtures', name + '.raw.json'), text);
const data = JSON.parse(text);
fs.writeFileSync(path.join(dir, '..', 'fixtures', name + '.expected.json'), JSON.stringify(unpack(data.res), null, 1));
console.log('http', res.status, 'bytes', text.length, 'mode', data.mode, 'code', data.code && data.code.status);
console.log('top-level keys:', Object.keys(unpack(data.res) || {}).join(', '));
