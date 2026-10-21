// Unpack the server's §r reference table into a plain object.
export function unpack(p) {
  if (!p || !p['§g']) return p;
  const T = p['§g'], O = T.map(x => Array.isArray(x) ? [] : {});
  const dec = v => (v && typeof v === 'object' && !Array.isArray(v) && typeof v['§r'] === 'number' && Object.keys(v).length === 1) ? O[v['§r']] : v;
  T.forEach((x, i) => {
    if (Array.isArray(x)) x.forEach((y, j) => { O[i][j] = dec(y); });
    else for (const k in x) O[i][k] = dec(x[k]);
  });
  return O[0];
}
