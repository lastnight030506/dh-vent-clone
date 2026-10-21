// Local replica of the Worker: same API, page served at / and /vent.
import http from 'node:http';
import fs from 'node:fs';
import { handle } from './src/api.js';

const page = fs.readFileSync(new URL('./public/index.html', import.meta.url));
const json = (res, data, status = 200) => {
  res.writeHead(status, { 'content-type': 'application/json; charset=utf-8', 'cache-control': 'no-store' });
  res.end(JSON.stringify(data));
};

const port = Number(process.env.PORT) || 8787;
http.createServer(async (req, res) => {
  const { pathname } = new URL(req.url, 'http://localhost');
  if (!pathname.startsWith('/api/')) {
    res.writeHead(200, { 'content-type': 'text/html; charset=utf-8' });
    return res.end(page);
  }
  let body = null;
  if (req.method === 'POST') {
    let raw = '';
    for await (const chunk of req) raw += chunk;
    try { body = JSON.parse(raw); } catch {}
  }
  const out = handle(pathname, body);
  if (out) json(res, out); else json(res, { error: 'not found' }, 404);
}).listen(port, () => console.log(`DH-VENT clone: http://localhost:${port}/vent`));
