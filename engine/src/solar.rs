//! Solar heat gain through windows and roof (QCVN 02:2022, Bảng 2.10).

use std::f64::consts::PI;

use crate::data::Data;
use crate::input::Project;
use crate::output::{ClimateOut, Envelope, Solar, Win};
use crate::util::{nearest_pos, nearest_sol, nn, num, psi_at, fixed};

/// Monthly mean solar declination (degrees).
const DECL: [f64; 12] = [
    -20.916962574476425,
    -12.954608093780699,
    -2.4177348051423553,
    9.414893346880085,
    18.79191751769616,
    23.085911002836543,
    21.183693564513824,
    13.454959682356424,
    2.2168867832133223,
    -9.59939723422635,
    -18.91195474122616,
    -23.049627643930602,
];

pub fn solar(p: &Project, c: &ClimateOut, e: &Envelope, d: &Data) -> Solar {
    let sol = p.sol.as_ref();
    let (stn_name, stn_d) = nearest_sol(&d.sol, c.lat, c.lon);
    let stn = d
        .sol
        .iter()
        .find(|(name, _)| name == stn_name)
        .map(|(_, s)| s)
        .expect("solar station");
    let (s18, s18_d) = nearest_pos(&d.solar_pos, c.lat, c.lon);

    let m = nn(sol.and_then(|s| s.m.as_ref()), c.mH as f64) as i64;
    let slot = nn(sol.and_then(|s| s.slot.as_ref()), 7.0) as i64;
    let hour = if slot >= 0 {
        d.hours.get(slot as usize).copied().unwrap_or(13)
    } else {
        13
    };
    let tau = [
        nn(sol.and_then(|s| s.t1.as_ref()), 0.9),
        nn(sol.and_then(|s| s.t2.as_ref()), 0.8),
        nn(sol.and_then(|s| s.t3.as_ref()), 0.75),
        nn(sol.and_then(|s| s.t4.as_ref()), 0.95),
    ];
    let tp: f64 = tau.iter().product();

    let qtab = |dir: &str| -> Vec<f64> {
        if m < 1 {
            return Vec::new();
        }
        stn.d
            .get(dir)
            .and_then(|rows| rows.get((m - 1) as usize))
            .cloned()
            .unwrap_or_default()
    };
    let q_at = |dir: &str| -> f64 {
        if slot < 0 {
            return 0.0;
        }
        qtab(dir).get(slot as usize).copied().unwrap_or(0.0)
    };

    let mut win = Vec::new();
    for dir in &d.dirs {
        let sel: Vec<&crate::output::El> = e
            .els
            .iter()
            .filter(|x| &x.dir == dir && (x.k == "cuaso" || x.k == "cuamai"))
            .collect();
        if sel.is_empty() {
            continue;
        }
        let fcs: f64 = sel.iter().filter(|x| x.k == "cuaso").map(|x| x.F).sum();
        let fcm: f64 = sel.iter().filter(|x| x.k == "cuamai").map(|x| x.F).sum();
        let f = fcs + fcm;
        let man = num(sol.and_then(|s| s.q.get(dir)));
        let qt = man.unwrap_or_else(|| q_at(dir));
        win.push(Win {
            dir: dir.clone(),
            Fcs: fcs,
            Fcm: fcm,
            F: f,
            qt,
            q: qt,
            manual: man.is_some(),
            Q: if man.is_some() { tp * man.unwrap() * f } else { tp * qt * f },
        });
    }
    let qwin: f64 = win.iter().map(|w| w.Q).sum();

    let roofs: Vec<&crate::output::El> = e.els.iter().filter(|x| x.k == "mai").collect();
    let fm: f64 = roofs.iter().map(|x| x.F).sum();
    let rc = roofs.first().and_then(|x| x.con.clone());
    let km = rc.as_ref().map(|x| x.K).unwrap_or(0.0);
    let t_ntb = c.tNtb;
    let rho = nn(sol.and_then(|s| s.rho.as_ref()), 0.65);
    let a_n = rc.as_ref().map(|x| x.aN).unwrap_or(23.26);
    let a_t = rc.as_ref().map(|x| x.aT).unwrap_or(8.72);
    let qday = num(sol.and_then(|s| s.qday.as_ref())).unwrap_or_else(|| {
        d.solar
            .get(&s18)
            .and_then(|months| months.get(&m.to_string()))
            .map(|x| x.qday)
            .unwrap_or(0.0)
    });
    let decl = DECL
        .get((m - 1).max(0) as usize)
        .copied()
        .unwrap_or(0.0)
        * PI
        / 180.0;
    let lat = c.lat * PI / 180.0;
    let n = (2.0 / 15.0)
        * (-lat.tan() * decl.tan()).clamp(-1.0, 1.0).acos()
        * 180.0
        / PI;
    let qtb = qday / 24.0;
    let qmax = num(sol.and_then(|s| s.qmax.as_ref())).unwrap_or(qday * PI / (2.0 * n));
    let auto = sol
        .and_then(|s| s.st.as_deref())
        .map(str::trim)
        .map_or(true, str::is_empty);
    let mon = crate::util::MONTHS
        .get((m - 1).max(0) as usize)
        .copied()
        .unwrap_or("")
        .to_string();

    let mut out = Solar {
        stn: stn_name.to_string(),
        stD: stn_d,
        auto,
        s18: s18.clone(),
        s18D: s18_d,
        m,
        mon,
        slot,
        hour,
        tau,
        tp,
        win,
        Qwin: qwin,
        Fm: fm,
        Km: km,
        rho,
        aN: a_n,
        aT: a_t,
        qday,
        qdayT: qday,
        qtb,
        qmax,
        qmaxT: qmax,
        N: n,
        warn: Vec::new(),
        tNtb: None,
        ttd: None,
        ttg: None,
        QdT: None,
        Aq: None,
        Attd: None,
        AtN: None,
        ratio: None,
        z: None,
        psi: None,
        Attg: None,
        nuD: None,
        nuR: None,
        nuPhi: None,
        nuCalc: None,
        nu: None,
        AtT: None,
        QA: None,
        Qroof: 0.0,
        noRoof: None,
        tot: qwin,
        psiPH: None,
    };

    if roofs.is_empty() {
        out.warn = Vec::new();
        out.Qroof = 0.0;
        out.noRoof = Some(true);
        out.tot = qwin;
        out.psiPH = None;
        return out;
    }

    let rc = rc.expect("roof structure");
    let ttd = rho * qtb / a_n;
    let ttg = t_ntb + ttd;
    let qdt = km * fm * (ttg - c.tTh);
    let aq = qmax - qtb;
    let attd = rho * aq / a_n;
    let atn = c.tNh - t_ntb;
    let ratio = if atn != 0.0 { attd / atn } else { 0.0 };
    let psi_ph = psi_at(ratio, nn(sol.and_then(|s| s.z.as_ref()), 1.0), &d.psi);
    let psi = num(sol.and_then(|s| s.psi.as_ref())).unwrap_or(psi_ph);
    let attg = (attd + atn) * psi;
    let nu_d = rc.D;
    let nu_r = rc.Rl;
    let nu_phi = if nu_d != 0.0 { 0.83 + 3.0 * nu_r / nu_d } else { 0.0 };
    let nu_calc = nu_phi * (nu_d / std::f64::consts::SQRT_2).exp();
    let nu_manual = num(sol.and_then(|s| s.nu.as_ref()));
    let nu = nu_manual.unwrap_or(nu_calc);
    let at_t = if nu != 0.0 { attg / nu } else { 0.0 };
    let qa = a_t * at_t * fm;
    let qroof = qdt + qa;

    let mut warn = Vec::new();
    if s18_d > 150.0 {
        warn.push(format!(
            "Trạm bức xạ gần nhất ({}) cách địa điểm {} km – số liệu chỉ mang tính tham khảo.",
            s18,
            fixed(s18_d, 0)
        ));
    }
    let emit_nu = sol.and_then(|s| s.nuMode.as_deref()) == Some("calc") || nu_manual.is_none();

    out.warn = warn;
    out.tNtb = Some(t_ntb);
    out.ttd = Some(ttd);
    out.ttg = Some(ttg);
    out.QdT = Some(qdt);
    out.Aq = Some(aq);
    out.Attd = Some(attd);
    out.AtN = Some(atn);
    out.ratio = Some(ratio);
    out.z = Some(nn(sol.and_then(|s| s.z.as_ref()), 1.0));
    out.psi = Some(psi);
    out.Attg = Some(attg);
    out.nuD = Some(nu_d);
    out.nuR = Some(nu_r);
    if emit_nu {
        out.nuPhi = Some(nu_phi);
        out.nuCalc = Some(nu_calc);
    }
    out.nu = Some(nu);
    out.AtT = Some(at_t);
    out.QA = Some(qa);
    out.Qroof = qroof;
    out.tot = qwin + qroof;
    out.psiPH = Some(psi_ph);
    out
}
