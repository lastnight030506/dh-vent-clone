//! Required ventilation airflow and air-change rate.

use serde_json::Value;

use crate::input::Project;
use crate::output::{ClimateOut, Gain, Loss, Solar, VentOut};
use crate::util::{fixed, nn, num};

pub fn ventilation(
    p: &Project,
    c: &ClimateOut,
    gh: &Gain,
    gd: &Gain,
    lh: &Loss,
    ld: &Loss,
    so: &Solar,
) -> VentOut {
    let qth_h = gh.tot + so.tot - lh.tot;
    let qth_d = gd.tot - ld.tot;
    let use_h = qth_h >= qth_d;
    let qth = if use_h { qth_h } else { qth_d };

    let v = p.vent.as_ref();
    let tvlv = num(v.and_then(|x| x.tvlv.as_ref())).unwrap_or(c.tTh);
    let a = nn(v.and_then(|x| x.a.as_ref()), 1.0);
    let h0 = nn(v.and_then(|x| x.h0.as_ref()), 0.0);
    let hv = nn(v.and_then(|x| x.hv.as_ref()), 2.0);
    let t_r = tvlv + a * (h0 - hv);
    let t_v = num(v.and_then(|x| x.tV.as_ref())).unwrap_or(c.tNh);
    let cp = 1.005;

    let mut warn = Vec::new();
    if t_r <= t_v {
        warn.push("Nhiệt độ không khí ra t_R phải lớn hơn nhiệt độ thổi vào t_V – kiểm tra chiều cao h₀ và hệ số a".into());
        return VentOut {
            QthH: qth_h,
            QthD: qth_d,
            Qth: qth,
            useH: use_h,
            tvlv,
            a,
            h0,
            hv,
            tR: t_r,
            tV: t_v,
            Cp: cp,
            warn,
            G: None,
            rho: None,
            L: None,
            V: None,
            n: None,
            nOut: None,
            Lmo: None,
            nOutPlan: None,
        };
    }

    let g = qth * 3.6 / (cp * (t_r - t_v));
    let rho = 353.0 / (273.0 + t_v);
    let l = g / rho;
    let volume = nn(p.bA.as_ref(), 0.0) * nn(p.bB.as_ref(), 0.0) * nn(p.bH.as_ref(), 0.0);
    let n = if volume != 0.0 { l / volume } else { 0.0 };
    let n_out = num(v.and_then(|x| x.nOut.as_ref()));
    if c.tTh > 34.0 {
        warn.push(format!(
            "Nhiệt độ trong nhà mùa hè {} °C > 34 °C – giáo trình yêu cầu không vượt 34 °C (trừ vùng t_N > 33 °C, khi đó tăng vận tốc gió)",
            fixed(c.tTh, 1)
        ));
    }
    let lmo = match n_out {
        Some(x) if x != 0.0 => Some(Value::from(l / x)),
        _ => Some(Value::Null),
    };

    VentOut {
        QthH: qth_h,
        QthD: qth_d,
        Qth: qth,
        useH: use_h,
        tvlv,
        a,
        h0,
        hv,
        tR: t_r,
        tV: t_v,
        Cp: cp,
        warn,
        G: Some(g),
        rho: Some(rho),
        L: Some(l),
        V: Some(volume),
        n: Some(n),
        nOut: n_out,
        Lmo: lmo,
        nOutPlan: None,
    }
}
