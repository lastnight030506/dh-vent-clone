//! Transmission and infiltration heat losses for one season.

use crate::data::Data;
use crate::input::Project;
use crate::output::{ClimateOut, Envelope, InfRow, Loss, LossRow};
use crate::util::{nn, num, pct_of, round, windward};

const KL: [[f64; 2]; 5] = [[1.0, 3.8], [2.0, 6.0], [3.0, 7.4], [4.0, 8.4], [5.0, 11.8]];
const GO: [[f64; 2]; 5] = [[1.0, 5.6], [2.0, 9.1], [3.0, 11.2], [4.0, 12.6], [5.0, 17.5]];

fn g_at(mat: &str, vel: f64) -> f64 {
    let table = if mat == "go" { &GO } else { &KL };
    if vel <= table[0][0] {
        return table[0][1];
    }
    for i in 1..table.len() {
        if vel <= table[i][0] {
            let [v0, g0] = table[i - 1];
            let [v1, g1] = table[i];
            return g0 + (g1 - g0) * (vel - v0) / (v1 - v0);
        }
    }
    table[table.len() - 1][1]
}

pub fn loss_season(p: &Project, c: &ClimateOut, e: &Envelope, season: char, d: &Data) -> Loss {
    let is_summer = season == 'h';
    let t_t = if is_summer { c.tTh } else { c.tTd };
    let t_n = if is_summer { c.tNh } else { c.tNd };
    let dt_base = t_t - t_n;
    let wd = if is_summer { &c.dirH } else { &c.dirD };
    let v = if is_summer { c.vH } else { c.vD };
    let skip = !is_summer && t_n >= t_t;

    let mut rows = Vec::new();
    if !skip {
        for el in &e.els {
            if is_summer && el.k == "mai" && p.roofSummer != Some(true) {
                continue;
            }
            let dt = el.psi * dt_base;
            let heat = el.con.as_ref().map(|con| con.K * el.F * dt).unwrap_or(0.0);
            let pct = pct_of(&el.dir);
            rows.push(LossRow {
                k: el.k.clone(),
                dir: Some(el.dir.clone()),
                K: el.con.as_ref().map(|con| con.K).unwrap_or(0.0),
                F: el.F,
                dt,
                Q: heat,
                pct: Some(pct),
                Qb: if pct != 0.0 { heat * pct / 100.0 } else { 0.0 },
                i: Some(el.i),
                band: None,
            });
        }
        if e.fl.on {
            for b in &e.fl.bands {
                let heat = b.K * b.F * dt_base;
                rows.push(LossRow {
                    k: "nen".into(),
                    dir: None,
                    K: b.K,
                    F: b.F,
                    dt: dt_base,
                    Q: heat,
                    pct: None,
                    Qb: 0.0,
                    i: None,
                    band: Some(b.i),
                });
            }
        }
    }
    let qk: f64 = rows.iter().map(|r| r.Q).sum();
    let qb: f64 = rows.iter().map(|r| r.Qb).sum();

    let mut inf_rows = Vec::new();
    if !skip {
        let manual = if is_summer { &p.infL.h } else { &p.infL.d };
        for k in ["cuaso", "cuadi", "cuamai"] {
            let sel: Vec<&crate::output::El> = e.els.iter().filter(|x| x.k == k).collect();
            if sel.is_empty() {
                continue;
            }
            let mut length: f64 = sel
                .iter()
                .map(|x| {
                    let n = num(Some(&x.n)).unwrap_or(1.0);
                    2.0 * (nn(Some(&x.a), 0.0) + nn(Some(&x.b), 0.0)) * n
                        * windward(&x.dir, wd, &d.dirs)
                })
                .sum();
            let man = num(manual.get(k));
            let is_manual = man.is_some();
            if let Some(m) = man {
                length = m;
            }
            let aa = sel.iter().map(|x| x.inf).fold(f64::NEG_INFINITY, f64::max);
            let mat = if sel.iter().any(|x| x.cr == "go") { "go" } else { "kl" };
            let g = g_at(mat, v);
            let mut parts = Vec::new();
            for dir in &d.dirs {
                let w = windward(dir, wd, &d.dirs);
                if w != 0.0 && sel.iter().any(|x| &x.dir == dir) {
                    parts.push(format!(
                        "{} {} %",
                        d.dirname.get(dir).map(String::as_str).unwrap_or(dir),
                        round(w * 100.0, 0)
                    ));
                }
            }
            inf_rows.push(InfRow {
                k: k.into(),
                l: length,
                G: aa * g * length,
                a: aa,
                g,
                parts,
                manual: is_manual.then_some(true),
                Q: 0.279 * aa * g * length * dt_base,
            });
        }
    }
    let qinf: f64 = inf_rows.iter().map(|r| r.Q).sum();

    Loss {
        tT: t_t,
        tN: t_n,
        dt: dt_base,
        rows,
        Qk: qk,
        Qb: qb,
        infRows: inf_rows,
        Qinf: qinf,
        v,
        wd: wd.clone(),
        tot: qk + qb + qinf,
        skip: skip.then_some(true),
    }
}
