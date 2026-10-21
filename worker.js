// Cloudflare Worker entry. Static files in ./public are served by the assets
// binding (see wrangler.toml); the worker handles only the JSON API.
import { handle } from './src/api.js';

const json = (data, status = 200) => new Response(JSON.stringify(data), {
  status,
  headers: { 'content-type': 'application/json; charset=utf-8', 'cache-control': 'no-store' },
});

export default {
  async fetch(req) {
    const { pathname } = new URL(req.url);
    let body = null;
    if (req.method === 'POST') body = await req.json().catch(() => null);
    const out = handle(pathname, body);
    return out ? json(out) : json({ error: 'not found' }, 404);
  },
};
