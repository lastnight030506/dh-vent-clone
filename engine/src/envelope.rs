//! Envelope structures: layer thermal resistance and element areas/openings.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::input::{El as InEl, Project};
use crate::output::{Band, Cons, El, Envelope, Fl, Layer};
use crate::util::{nn, num};

pub fn cons_list(p: &Project) -> BTreeMap<String, Cons> {
    let mut out = BTreeMap::new();
    for c in &p.cons {
        let a_t = nn(c.aT.as_ref(), 8.72);
        let a_n = nn(c.aN.as_ref(), 23.26);
        let mut rl = 0.0;
        let mut dsum = 0.0;
        let mut layers = Vec::new();
        for l in &c.layers {
            let thickness = nn(l.d.as_ref(), 0.0);
            let dm = thickness / 1000.0;
            let lam = num(l.lam.as_ref());
            let s = nn(l.s.as_ref(), 0.0);
            let r = match lam {
                Some(x) if x != 0.0 => dm / x,
                _ => 0.0,
            };
            let d_layer = r * s;
            rl += r;
            dsum += d_layer;
            layers.push(Layer {
                n: l.n.clone().unwrap_or_default(),
                d: l.d.clone().unwrap_or_else(|| Value::String(String::new())),
                lam: l.lam.clone().unwrap_or_else(|| Value::String(String::new())),
                s,
                dm,
                R: r,
                D: d_layer,
            });
        }
        let ro = 1.0 / a_t + rl + 1.0 / a_n;
        out.insert(
            c.id.clone(),
            Cons {
                id: c.id.clone(),
                name: c.name.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| c.id.clone()),
                aT: a_t,
                aN: a_n,
                layers,
                Rl: rl,
                Ro: ro,
                K: 1.0 / ro,
                D: dsum,
            },
        );
    }
    out
}

fn area(e: &InEl) -> f64 {
    nn(e.a.as_ref(), 0.0) * nn(e.b.as_ref(), 0.0) * num(e.n.as_ref()).unwrap_or(1.0)
}

fn cut(els: &[InEl], k: &str, dir: &str) -> f64 {
    els.iter()
        .filter(|e| {
            let opening = (k == "tuong"
                && (e.k.as_deref() == Some("cuaso") || e.k.as_deref() == Some("cuadi")))
                || (k == "mai" && e.k.as_deref() == Some("cuamai"));
            opening && e.dir.as_deref() == Some(dir)
        })
        .map(area)
        .sum()
}

pub fn envelope(p: &Project, cons: &BTreeMap<String, Cons>) -> Envelope {
    let mut list = Vec::new();
    let mut warn = Vec::new();

    for (i, e) in p.els.iter().enumerate() {
        let a = nn(e.a.as_ref(), 0.0);
        let b = nn(e.b.as_ref(), 0.0);
        let n = num(e.n.as_ref()).unwrap_or(1.0);
        let fg = a * b * n;
        let k = e.k.clone().unwrap_or_default();
        let dir = e.dir.clone().unwrap_or_default();
        let con = e
            .con
            .as_ref()
            .and_then(|id| cons.get(id))
            .cloned();
        let mut row = El {
            i,
            k: k.clone(),
            dir,
            a: e.a.clone().unwrap_or_else(|| Value::String(String::new())),
            b: e.b.clone().unwrap_or_else(|| Value::String(String::new())),
            n: e.n.clone().unwrap_or_else(|| Value::String(String::new())),
            Fg: fg,
            con: con.clone(),
            psi: num(e.psi.as_ref()).unwrap_or(1.0),
            inf: num(e.inf.as_ref()).unwrap_or(0.65),
            cr: e.cr.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "kl".into()),
            name: e.name.clone().unwrap_or_default(),
            F: fg,
            cut: None,
        };
        if k == "tuong" || k == "mai" {
            let c = cut(&p.els, &k, &row.dir).max(0.0);
            row.F = (fg - c).max(0.0);
            if k == "tuong" || c > 0.0 {
                row.cut = Some(c);
            }
        }
        if con.is_none() {
            warn.push(format!("Dòng {}: chưa chọn kết cấu.", i + 1));
        }
        list.push(row);
    }

    let a = nn(p.bA.as_ref(), 0.0);
    let b = nn(p.bB.as_ref(), 0.0);
    let rins: f64 = p
        .floorIns
        .iter()
        .map(|l| {
            let d = num(l.d.as_ref()).unwrap_or(0.0);
            let lam = match num(l.lam.as_ref()) {
                Some(x) if x != 0.0 => x,
                _ => 1.0,
            };
            d / 1000.0 / lam
        })
        .sum();
    let on = p.floorOn != Some(false) && a > 0.0 && b > 0.0;
    let f1 = 4.0 * (a + b);
    let bands = if on {
        [
            (1, f1, 2.2),
            (2, (f1 - 48.0).max(0.0), 4.3),
            (3, (f1 - 80.0).max(0.0), 8.6),
            (4, (a * b + 128.0 - 3.0 * f1).max(0.0), 14.2),
        ]
        .into_iter()
        .map(|(i, f, r)| Band {
            i,
            F: f,
            R: r + rins,
            K: 1.0 / (r + rins),
        })
        .collect()
    } else {
        Vec::new()
    };

    Envelope {
        cons: cons.clone(),
        els: list,
        fl: Fl {
            on,
            bands,
            Rins: on.then_some(rins),
        },
        err: Vec::new(),
        warn,
    }
}
