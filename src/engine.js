// DH-VENT compute engine – reverse-engineered from the live worker's responses and
// the calculation notes published in the page (QCVN 02:2022/BXD, TCVN 5687:2024,
// Giáo trình Thông gió).
import { CATALOG } from './data/catalog.js';
import { CLIMATE } from './data/climate.js';
import { PSI } from './data/psi.js';

export const DIRS = CATALOG.DIRS;
const DIRNAME = CATALOG.DIRNAME;
const HOURS = CATALOG.HOURS;
const MONTH = ['I', 'II', 'III', 'IV', 'V', 'VI', 'VII', 'VIII', 'IX', 'X', 'XI', 'XII'];

// monthly mean solar declination (degrees) used by the day-length formula
const DECL = [-20.916962574476425, -12.954608093780699, -2.4177348051423553, 9.414893346880085, 18.79191751769616, 23.085911002836543, 21.183693564513824, 13.454959682356424, 2.2168867832133223, -9.59939723422635, -18.91195474122616, -23.049627643930602];

const num = v => (v === '' || v == null || !isFinite(+v)) ? null : +v;
const nn = (v, d = 0) => { const x = num(v); return x == null ? d : x; };
const pctOf = d => ({ B: 10, DB: 10, D: 10, TB: 10, DN: 5, T: 5, N: 0, TN: 0 }[d] ?? 0);
const ang = d => DIRS.indexOf(d) * 45;
function windward(face, wind) {
  if (!DIRS.includes(face) || !DIRS.includes(wind)) return 0;
  const df = Math.abs(((ang(face) - ang(wind)) % 360 + 540) % 360 - 180);
  return df < 1 ? 1 : Math.abs(df - 45) < 1 ? 0.65 : 0;
}
function distKm(a, b) {
  const R = 6371, r = Math.PI / 180;
  const dLat = (b.lat - a.lat) * r, dLon = (b.lon - a.lon) * r;
  const h = Math.sin(dLat / 2) ** 2 + Math.cos(a.lat * r) * Math.cos(b.lat * r) * Math.sin(dLon / 2) ** 2;
  return 2 * R * Math.asin(Math.sqrt(h));
}
const round = (x, d = 0) => Math.round(x * 10 ** d) / 10 ** d;
const fmt = (x, d = 1) => Number(x).toFixed(d);

// Magnus saturation pressure, Pa
const pbh = t => 610.94 * Math.exp(17.625 * t / (t + 243.04));
const moisture = (t, phi) => { const p = (phi / 100) * pbh(t); return 622 * p / (101325 - p); };

function climate(S) {
  const name = S.st || 'Hà Nội';
  const st = CLIMATE.stations[name] || CLIMATE.stations['Hà Nội'];
  const ov = k => num(S[k]);
  const src = {};
  for (const k of ['tNh', 'phiH', 'tNtb', 'tNd', 'phiD', 'vH', 'vD']) src[k] = ov(k) != null;
  for (const k of ['dirH', 'dirD']) src[k] = !!(S[k] && String(S[k]).trim());
  const tNh = ov('tNh') ?? st.tNh, phiH = ov('phiH') ?? st.phiH, tNtb = ov('tNtb') ?? st.tNtb;
  const tNd = ov('tNd') ?? st.tNd, phiD = ov('phiD') ?? st.phiD;
  const vH = ov('vH') ?? st.vH, vD = ov('vD') ?? st.vD;
  const dirH = (S.dirH && String(S.dirH).trim()) || st.dirH, dirD = (S.dirD && String(S.dirD).trim()) || st.dirD;
  const dTh = nn(S.dTh, 1), tTd = nn(S.tTd, 20), labor = S.labor || 'vua';
  const tTh = tNh + dTh;
  const warn = [];
  if (tTh > 34) warn.push(`Nhiệt độ trong nhà mùa hè t_T = ${fmt(tTh, 1)} °C vượt 34 °C (giáo trình Bảng 2.1: t_T không quá 34 °C; vùng có t_N > 33 °C được lấy cao hơn nhưng phải tăng vận tốc gió trong vùng làm việc – mỗi 1 °C tăng 0,5 ÷ 0,8 m/s, không quá 2,5 m/s). Kiểm tra lại ô chênh nhiệt mùa hè.`);
  if (tNd >= tTd) warn.push(`Địa điểm không có mùa lạnh (t_N mùa đông = ${fmt(tNd, 1)} °C ≥ t_T mùa đông = ${fmt(tTd, 1)} °C) – không tính tổn thất nhiệt mùa đông.`);
  return {
    st: name in CLIMATE.stations ? name : 'Hà Nội', reg: st.region, lat: st.lat, lon: st.lon, z: st.z,
    mH: st.mH, mD: st.mD,
    tab: { tNh: st.tNh, phiH: st.phiH, tNtb: st.tNtb, tNd: st.tNd, phiD: st.phiD, vH: st.vH, dirH: st.dirH, vD: st.vD, dirD: st.dirD },
    wH: { ...st.wind.h, dir: dirH, v: vH }, wD: { ...st.wind.d, dir: dirD, v: vD },
    noTavg: false, tNh, phiH, tNtb, tNd, phiD, src,
    dH: moisture(tNh, phiH), dD: moisture(tNd, phiD),
    dTh, tTh, tTd, vH, vD, dirH, dirD, labor, warn,
  };
}

function consList(S) {
  const out = {};
  for (const c of S.cons || []) {
    const aT = nn(c.aT, 8.72), aN = nn(c.aN, 23.26);
    let Rl = 0, D = 0;
    const layers = (c.layers || []).map(l => {
      const d = nn(l.d, 0), dm = d / 1000, lam = num(l.lam), s = nn(l.s, 0);
      const R = lam ? dm / lam : 0, Dd = R * s;
      Rl += R; D += Dd;
      return { n: l.n || '', d: l.d ?? '', lam: l.lam ?? '', s: nn(l.s, 0), dm, R, D: Dd };
    });
    const Ro = 1 / aT + Rl + 1 / aN;
    out[c.id] = { id: c.id, name: c.name || c.id, aT, aN, layers, Rl, Ro, K: 1 / Ro, D };
  }
  return out;
}

function envelope(S, C, cons) {
  const els = S.els || [];
  const area = e => nn(e.a, 0) * nn(e.b, 0) * (num(e.n) == null ? 1 : num(e.n));
  const cut = (k, dir) => els.reduce((s, e) => {
    const opening = (k === 'tuong' && (e.k === 'cuaso' || e.k === 'cuadi')) || (k === 'mai' && e.k === 'cuamai');
    return s + (opening && e.dir === dir ? area(e) : 0);
  }, 0);
  const list = [];
  const err = [], warn = [];
  els.forEach((e, i) => {
    const a = nn(e.a, 0), b = nn(e.b, 0), n = num(e.n) == null ? 1 : num(e.n);
    const Fg = a * b * n;
    const con = cons[e.con] || null;
    const row = {
      i, k: e.k, dir: e.dir, a: e.a ?? '', b: e.b ?? '', n: e.n ?? '', Fg, con,
      psi: num(e.psi) == null ? 1 : num(e.psi), inf: num(e.inf) == null ? 0.65 : num(e.inf),
      cr: e.cr || 'kl', name: e.name || '', F: Fg,
    };
    if (e.k === 'tuong' || e.k === 'mai') {
      const c = Math.max(0, cut(e.k, e.dir));
      row.F = Math.max(0, Fg - c);
      if (e.k === 'tuong' || c > 0) row.cut = c;
    }
    if (!con) warn.push(`Dòng ${i + 1}: chưa chọn kết cấu.`);
    list.push(row);
  });

  const a = nn(S.bA, 0), b = nn(S.bB, 0);
  const Rins = (S.floorIns || []).reduce((s, l) => s + (num(l.d) || 0) / 1000 / (num(l.lam) || 1), 0);
  const on = S.floorOn !== false && a > 0 && b > 0;
  const F1 = 4 * (a + b);
  const bands = on ? [
    { i: 1, F: F1, R: 2.2 + Rins }, { i: 2, F: Math.max(0, F1 - 48), R: 4.3 + Rins },
    { i: 3, F: Math.max(0, F1 - 80), R: 8.6 + Rins }, { i: 4, F: Math.max(0, a * b + 128 - 3 * F1), R: 14.2 + Rins },
  ].map(x => ({ ...x, K: 1 / x.R })) : [];
  return { cons, els: list, fl: { on, bands, ...(on ? { Rins } : {}) }, err, warn };
}

function lossSeason(E, C, season, S) {
  const H = season === 'h';
  const tT = H ? C.tTh : C.tTd, tN = H ? C.tNh : C.tNd, dtBase = tT - tN;
  const wd = H ? C.dirH : C.dirD, v = H ? C.vH : C.vD;
  const rows = [];
  const skip = !H && tN >= tT;
  if (!skip) {
    for (const e of E.els) {
      if (H && e.k === 'mai' && !S.roofSummer) continue;
      const dt = e.psi * dtBase;
      const Q = e.con ? e.con.K * e.F * dt : 0;
      const pct = pctOf(e.dir);
      rows.push({ k: e.k, dir: e.dir, K: e.con ? e.con.K : 0, F: e.F, dt, Q, pct, Qb: pct ? Q * pct / 100 : 0, i: e.i });
    }
    if (E.fl.on) for (const b of E.fl.bands) {
      const Q = b.K * b.F * dtBase;
      rows.push({ k: 'nen', band: b.i, K: b.K, F: b.F, dt: dtBase, Q, pct: null, Qb: 0 });
    }
  }
  const Qk = rows.reduce((s, r) => s + r.Q, 0), Qb = rows.reduce((s, r) => s + r.Qb, 0);
  const G25 = { kl: [[1, 3.8], [2, 6.0], [3, 7.4], [4, 8.4], [5, 11.8]], go: [[1, 5.6], [2, 9.1], [3, 11.2], [4, 12.6], [5, 17.5]] };
  const gAt = (mat, vel) => {
    const t = G25[mat] || G25.kl;
    if (vel <= t[0][0]) return t[0][1];
    for (let i = 1; i < t.length; i++) if (vel <= t[i][0]) {
      const [v0, g0] = t[i - 1], [v1, g1] = t[i];
      return g0 + (g1 - g0) * (vel - v0) / (v1 - v0);
    }
    return t[t.length - 1][1];
  };
  const infRows = [];
  if (!skip) {
    const manual = (S.infL || {})[season] || {};
    for (const k of ['cuaso', 'cuadi', 'cuamai']) {
      const sel = E.els.filter(e => e.k === k);
      if (!sel.length) continue;
      let l = sel.reduce((s, e) => s + 2 * (nn(e.a, 0) + nn(e.b, 0)) * (num(e.n) == null ? 1 : num(e.n)) * windward(e.dir, wd), 0);
      const man = num(manual[k]);
      const isManual = man != null;
      if (isManual) l = man;
      const aa = Math.max(...sel.map(e => e.inf));
      const mat = sel.some(e => e.cr === 'go') ? 'go' : 'kl';
      const g = gAt(mat, v);
      const parts = [];
      for (const d of DIRS) { const w = windward(d, wd); if (w && sel.some(e => e.dir === d)) parts.push(`${DIRNAME[d]} ${round(w * 100)} %`); }
      infRows.push({ k, l, G: aa * g * l, a: aa, g, parts, Q: 0.279 * aa * g * l * dtBase, ...(isManual ? { manual: true } : {}) });
    }
  }
  const Qinf = infRows.reduce((s, r) => s + r.Q, 0);
  return {
    tT, tN, dt: dtBase, rows, Qk, Qb, infRows, Qinf, v, wd,
    tot: Qk + Qb + Qinf, ...(skip ? { skip: true } : {}),
  };
}

const QN = { nghi: [145, 115, 3], nhe: [155, 145, 1], vua: [210, 200, 1], nang: [290, 290, 0] };
function gains(S, C) {
  const Fs = nn(S.bA, 0) * nn(S.bB, 0);
  const L = S.light || {};
  const Nts = L.mode === 'kw' ? nn(L.kw, 0) * 1000 : nn(L.a, 20) * (num(L.F) ?? Fs);
  const e1 = nn(L.e1, 0.5), e2 = nn(L.e2, 0.94), Qts = Nts * e1 * e2;
  const mot = (S.motors || []).map(m => ({ name: m.name || '', n: m.n ?? '', kW: num(m.kW) ?? 0, N: nn(m.n, 1) * nn(m.kW, 0) }));
  const Nm = mot.reduce((s, m) => s + m.N, 0);
  const phi = nn(S.phiM, 0.25), Qdc = phi * 1000 * Nm;
  const nP = nn(S.nPeople, 0);
  const mk = tT => {
    const oth = (S.others || []).map(o => {
      let Q = 0;
      if (o.t === 'fuel') Q = 0.279 * nn(o.eta, 0.95) * nn(o.G, 0) * nn(o.Qp, 0);
      else if (o.t === 'prod') Q = 0.279 * nn(o.C, 0) * nn(o.G, 0) * ((num(o.td) ?? 0) - (num(o.tc) ?? tT)) * nn(o.beta, 0.5);
      else Q = nn(o.Q, 0);
      const f = o.t === 'fuel' ? `0,279·${nn(o.eta, 0.95)}·${nn(o.G, 0)}·${nn(o.Qp, 0)}`
        : o.t === 'prod' ? `0,279·${nn(o.C, 0)}·${nn(o.G, 0)}·(${num(o.td) ?? 0} − ${num(o.tc) ?? tT})·${nn(o.beta, 0.5)}`
        : 'nhập trực tiếp';
      return { name: o.name || '', t: o.t || 'W', Q, f };
    });
    const Qoth = oth.reduce((s, o) => s + o.Q, 0);
    const [hi, lo, slope] = QN[C.labor] || QN.vua;
    const qn = Math.max(lo, hi - slope * (tT - 15));
    return { tT, Fs, Nts, e1, e2, Qts, mot, Nm, phi, Qdc, nP, qn, Qng: nP * qn, oth, Qoth, tot: Qts + Qdc + nP * qn + Qoth };
  };
  return { H: mk(C.tTh), D: mk(C.tTd) };
}

function nearest(list, lat, lon) {
  let best = null;
  for (const [name, s] of Object.entries(list)) {
    const d = distKm({ lat, lon }, s);
    if (!best || d < best.d) best = { name, d, s };
  }
  return best;
}

function solar(S, C, E) {
  const sol = S.sol || {};
  const stn = nearest(CATALOG.sol, C.lat, C.lon);
  const s18 = nearest(CLIMATE.solarPos, C.lat, C.lon);
  const m = nn(sol.m, C.mH) | 0;
  const slot = nn(sol.slot, 7) | 0, hour = HOURS[slot] ?? 13;
  const tau = [nn(sol.t1, 0.9), nn(sol.t2, 0.8), nn(sol.t3, 0.75), nn(sol.t4, 0.95)];
  const tp = tau.reduce((a, b) => a * b, 1);
  const qtab = t => (stn.s.d?.[t] || [])[m - 1] || [];
  const win = [];
  for (const d of DIRS) {
    const sel = E.els.filter(e => e.dir === d && (e.k === 'cuaso' || e.k === 'cuamai'));
    if (!sel.length) continue;
    const Fcs = sel.filter(e => e.k === 'cuaso').reduce((s, e) => s + e.F, 0);
    const Fcm = sel.filter(e => e.k === 'cuamai').reduce((s, e) => s + e.F, 0);
    const F = Fcs + Fcm;
    const man = num((sol.q || {})[d]);
    const qt = man ?? (qtab(d)[slot] ?? 0);
    win.push({ dir: d, Fcs, Fcm, F, qt, q: qt, manual: man != null, Q: man != null ? tp * man * F : tp * qt * F });
  }
  const Qwin = win.reduce((s, w) => s + w.Q, 0);
  const roofs = E.els.filter(e => e.k === 'mai');
  const Fm = roofs.reduce((s, e) => s + e.F, 0);
  const rc = roofs.length ? roofs[0].con : null;
  const Km = rc ? rc.K : 0;
  const tNtb = C.tNtb, rho = nn(sol.rho, 0.65), aN = rc ? rc.aN : 23.26, aT = rc ? rc.aT : 8.72;
  const qday = num(sol.qday) ?? (CLIMATE.solar[s18.name]?.[m]?.qday ?? 0);
  const decl = DECL[m - 1] * Math.PI / 180, lat = C.lat * Math.PI / 180;
  const N = (2 / 15) * Math.acos(Math.max(-1, Math.min(1, -Math.tan(lat) * Math.tan(decl)))) * 180 / Math.PI;
  const qtb = qday / 24;
  const qmax = num(sol.qmax) ?? qday * Math.PI / (2 * N);
  const base = {
    stn: stn.name, stD: stn.d, auto: !(sol.st && String(sol.st).trim()), s18: s18.name, s18D: s18.d,
    m, mon: MONTH[m - 1], slot, hour, tau, tp, win, Qwin, Fm, Km, rho, aN, aT, qday, qdayT: qday, qtb, qmax, qmaxT: qmax, N,
  };
  if (!roofs.length) return { ...base, warn: [], Qroof: 0, noRoof: true, tot: Qwin, psiPH: null };
  const ttd = rho * qtb / aN, ttg = tNtb + ttd;
  const QdT = Km * Fm * (ttg - C.tTh);
  const Aq = qmax - qtb, Attd = rho * Aq / aN, AtN = C.tNh - tNtb, ratio = AtN ? Attd / AtN : 0;
  const psiPH = psiAt(ratio, nn(sol.z, 1));
  const psi = num(sol.psi) ?? psiPH;
  const Attg = (Attd + AtN) * psi;
  const nuD = rc.D, nuR = rc.Rl, nuPhi = nuD ? 0.83 + 3 * nuR / nuD : 0;
  const nuCalc = nuPhi * Math.exp(nuD / Math.SQRT2);
  const nuManual = num(sol.nu);
  const nu = nuManual ?? nuCalc;
  const AtT = nu ? Attg / nu : 0;
  const QA = aT * AtT * Fm;
  const Qroof = QdT + QA;
  const warn = [];
  if (s18.d > 150) warn.push(`Trạm bức xạ gần nhất (${s18.name}) cách địa điểm ${fmt(s18.d, 0)} km – số liệu chỉ mang tính tham khảo.`);
  const nuFields = sol.nuMode === 'calc' || nuManual == null ? { nuPhi, nuCalc } : {};
  return { ...base, warn, tNtb, ttd, ttg, QdT, Aq, Attd, AtN, ratio, z: nn(sol.z, 1), psi, Attg, nuD, nuR, ...nuFields, nu, AtT, QA, Qroof, tot: Qwin + Qroof, psiPH };
}

function psiAt(ratio, z) {
  const knots = [1, 2, 3, 5];
  const r = Math.max(knots[0], Math.min(knots[knots.length - 1], ratio));
  let i = 0;
  while (i < knots.length - 2 && r > knots[i + 1]) i++;
  const r0 = knots[i], r1 = knots[i + 1];
  const zz = Math.max(1, Math.min(10, z));
  const z0 = Math.floor(zz), z1 = Math.min(10, z0 + 1), fz = zz - z0;
  const v = (k, zz2) => PSI[String(k)][String(zz2)];
  const a = v(r0, z0) + (v(r1, z0) - v(r0, z0)) * (r - r0) / (r1 - r0);
  const b = v(r0, z1) + (v(r1, z1) - v(r0, z1)) * (r - r0) / (r1 - r0);
  return a + (b - a) * fz;
}

function ventilation(S, C, GH, GD, LH, LD, SO) {
  const QthH = GH.tot + SO.tot - LH.tot;
  const QthD = GD.tot - LD.tot;
  const useH = QthH >= QthD;
  const Qth = useH ? QthH : QthD;
  const v = S.vent || {};
  const tvlv = num(v.tvlv) ?? C.tTh;
  const a = nn(v.a, 1), h0 = nn(v.h0, 0), hv = nn(v.hv, 2);
  const tR = tvlv + a * (h0 - hv);
  const tV = num(v.tV) ?? C.tNh;
  const Cp = 1.005;
  const warn = [];
  if (tR <= tV) {
    warn.push('Nhiệt độ không khí ra t_R phải lớn hơn nhiệt độ thổi vào t_V – kiểm tra chiều cao h₀ và hệ số a');
    return { QthH, QthD, Qth, useH, tvlv, a, h0, hv, tR, tV, Cp, warn, G: null, nOutPlan: null };
  }
  const G = Qth * 3.6 / (Cp * (tR - tV));
  const rho = 353 / (273 + tV);
  const L = G / rho;
  const V = nn(S.bA, 0) * nn(S.bB, 0) * nn(S.bH, 0);
  const n = V ? L / V : 0;
  const nOut = num(v.nOut);
  if (C.tTh > 34) warn.push(`Nhiệt độ trong nhà mùa hè ${fmt(C.tTh, 1)} °C > 34 °C – giáo trình yêu cầu không vượt 34 °C (trừ vùng t_N > 33 °C, khi đó tăng vận tốc gió)`);
  return { QthH, QthD, Qth, useH, tvlv, a, h0, hv, tR, tV, Cp, warn, G, rho, L, V, n, nOut, Lmo: nOut ? L / nOut : null, nOutPlan: null };
}

const NO_PLAN_H = { tAir: null, rho: null, beta: null, K: null, qOut: null, nOutTot: null, segs: [], err: ['Phương án TG: Chưa vạch tuyến ống'], warn: [] };
const NO_PLAN_P = { err: ['Chưa vạch tuyến ống'], warn: [], segs: [], suct: null, label: {}, geo: [], nOut: 0, nodeZ: {}, bends: { h: 0, v: 0 }, empty: true };

export function compute(input) {
  const S = input || {};
  const C = climate(S);
  const cons = consList(S);
  const E = envelope(S, C, cons);
  const LH = lossSeason(E, C, 'h', S);
  const LD = lossSeason(E, C, 'd', S);
  const { H: GH, D: GD } = gains(S, C);
  const SO = solar(S, C, E);
  const V = ventilation(S, C, GH, GD, LH, LD, SO);
  return { C, E, LH, LD, GH, GD, SO, V, H: { ...NO_PLAN_H }, FN: null, P: { ...NO_PLAN_P }, CK: {} };
}

export { psiAt, moisture, windward };
