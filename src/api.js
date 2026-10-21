// HTTP-level contract shared by the Worker and the local dev server.
// The page only ever calls /api/vent/meta, /api/vent/compute and /api/check-code.
import { compute } from './engine.js';

export const META = {
  admin: false,
  mode: 'free',
  app: 'vent',
  contact: '',
  // The duct tabs (tP/t7/t8/t9) are off on the original for this licence and no
  // server output exists for them, so their engine is not reimplemented here.
  cfg: { modes: { tP: 'off', t7: 'off', t8: 'off', t9: 'off' }, tol: 3 },
};

const FULL = { mode: 'free', full: true, teach: false, cfg: META.cfg };

export function handle(path, body) {
  if (path === '/api/vent/meta') return META;
  // No licensing locally: every code is accepted.
  if (path === '/api/check-code') return { status: 'valid', name: 'Local build', app: 'vent' };
  if (path === '/api/vent/compute') {
    try {
      return { ...FULL, res: compute(body && body.input) };
    } catch (e) {
      return { ...FULL, error: String((e && e.message) || e) };
    }
  }
  return null;
}
