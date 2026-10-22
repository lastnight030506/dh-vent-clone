//! Internal heat gains for summer and winter.

use serde_json::Value;

use crate::input::{Light, Project};
use crate::output::{ClimateOut, Gain, Mot, Oth};
use crate::util::{jsnum, nn, num};

pub struct Gains {
    pub h: Gain,
    pub d: Gain,
}

fn light_field<'a>(l: Option<&'a Light>, f: impl Fn(&'a Light) -> Option<&'a Value>) -> Option<&'a Value> {
    l.and_then(f)
}

pub fn gains(p: &Project, c: &ClimateOut) -> Gains {
    let fs = nn(p.bA.as_ref(), 0.0) * nn(p.bB.as_ref(), 0.0);
    let l = p.light.as_ref();
    let nts = if l.and_then(|x| x.mode.as_deref()) == Some("kw") {
        nn(light_field(l, |x| x.kw.as_ref()), 0.0) * 1000.0
    } else {
        nn(light_field(l, |x| x.a.as_ref()), 20.0)
            * num(light_field(l, |x| x.F.as_ref())).unwrap_or(fs)
    };
    let e1 = nn(light_field(l, |x| x.e1.as_ref()), 0.5);
    let e2 = nn(light_field(l, |x| x.e2.as_ref()), 0.94);
    let qts = nts * e1 * e2;

    let mot: Vec<Mot> = p
        .motors
        .iter()
        .map(|m| {
            let n = m.n.clone().unwrap_or_else(|| Value::String(String::new()));
            let kw = num(m.kW.as_ref()).unwrap_or(0.0);
            Mot {
                name: m.name.clone().unwrap_or_default(),
                n,
                kW: kw,
                N: nn(m.n.as_ref(), 1.0) * nn(m.kW.as_ref(), 0.0),
            }
        })
        .collect();
    let nm: f64 = mot.iter().map(|m| m.N).sum();
    let phi = nn(p.phiM.as_ref(), 0.25);
    let qdc = phi * 1000.0 * nm;
    let n_people = nn(p.nPeople.as_ref(), 0.0);

    let make = |t_t: f64| -> Gain {
        let oth: Vec<Oth> = p
            .others
            .iter()
            .map(|o| {
                let kind = o
                    .t
                    .clone()
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "W".into());
                let q = match kind.as_str() {
                    "fuel" => {
                        0.279 * nn(o.eta.as_ref(), 0.95) * nn(o.G.as_ref(), 0.0) * nn(o.Qp.as_ref(), 0.0)
                    }
                    "prod" => {
                        let td = num(o.td.as_ref()).unwrap_or(0.0);
                        let tc = num(o.tc.as_ref()).unwrap_or(t_t);
                        0.279
                            * nn(o.C.as_ref(), 0.0)
                            * nn(o.G.as_ref(), 0.0)
                            * (td - tc)
                            * nn(o.beta.as_ref(), 0.5)
                    }
                    _ => nn(o.Q.as_ref(), 0.0),
                };
                let f = match kind.as_str() {
                    "fuel" => format!(
                        "0,279·{}·{}·{}",
                        jsnum(nn(o.eta.as_ref(), 0.95)),
                        jsnum(nn(o.G.as_ref(), 0.0)),
                        jsnum(nn(o.Qp.as_ref(), 0.0))
                    ),
                    "prod" => format!(
                        "0,279·{}·{}·({} − {})·{}",
                        jsnum(nn(o.C.as_ref(), 0.0)),
                        jsnum(nn(o.G.as_ref(), 0.0)),
                        jsnum(num(o.td.as_ref()).unwrap_or(0.0)),
                        jsnum(num(o.tc.as_ref()).unwrap_or(t_t)),
                        jsnum(nn(o.beta.as_ref(), 0.5))
                    ),
                    _ => "nhập trực tiếp".into(),
                };
                Oth {
                    name: o.name.clone().unwrap_or_default(),
                    t: kind,
                    Q: q,
                    f,
                }
            })
            .collect();
        let q_oth: f64 = oth.iter().map(|o| o.Q).sum();
        let (hi, lo, slope) = match c.labor.as_str() {
            "nghi" => (145.0, 115.0, 3.0),
            "nhe" => (155.0, 145.0, 1.0),
            "nang" => (290.0, 290.0, 0.0),
            _ => (210.0, 200.0, 1.0),
        };
        let qn = (hi - slope * (t_t - 15.0)).max(lo);
        Gain {
            tT: t_t,
            Fs: fs,
            Nts: nts,
            e1,
            e2,
            Qts: qts,
            mot: mot.clone(),
            Nm: nm,
            phi,
            Qdc: qdc,
            nP: n_people,
            qn,
            Qng: n_people * qn,
            oth,
            Qoth: q_oth,
            tot: qts + qdc + n_people * qn + q_oth,
        }
    };

    Gains {
        h: make(c.tTh),
        d: make(c.tTd),
    }
}
