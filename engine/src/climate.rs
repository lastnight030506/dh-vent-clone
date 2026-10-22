//! Climate station selection and design indoor/outdoor temperatures.

use crate::data::Data;
use crate::input::Project;
use crate::output::{ClimateOut, Src, Tab, WindObs};
use crate::util::{fixed, moisture, nn, num};

pub fn climate(p: &Project, d: &Data) -> ClimateOut {
    let name = p
        .st
        .clone()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Hà Nội".into());
    let fallback = d.stations.get("Hà Nội").expect("Hà Nội station");
    let st = d.stations.get(&name).unwrap_or(fallback);

    let ov = |v: Option<&serde_json::Value>| num(v);
    let src = Src {
        tNh: ov(p.tNh.as_ref()).is_some(),
        phiH: ov(p.phiH.as_ref()).is_some(),
        tNtb: ov(p.tNtb.as_ref()).is_some(),
        tNd: ov(p.tNd.as_ref()).is_some(),
        phiD: ov(p.phiD.as_ref()).is_some(),
        vH: ov(p.vH.as_ref()).is_some(),
        vD: ov(p.vD.as_ref()).is_some(),
        dirH: p.dirH.as_deref().map(str::trim).is_some_and(|s| !s.is_empty()),
        dirD: p.dirD.as_deref().map(str::trim).is_some_and(|s| !s.is_empty()),
    };

    let t_nh = ov(p.tNh.as_ref()).unwrap_or(st.tNh);
    let phi_h = ov(p.phiH.as_ref()).unwrap_or(st.phiH);
    let t_ntb = ov(p.tNtb.as_ref()).unwrap_or(st.tNtb);
    let t_nd = ov(p.tNd.as_ref()).unwrap_or(st.tNd);
    let phi_d = ov(p.phiD.as_ref()).unwrap_or(st.phiD);
    let v_h = ov(p.vH.as_ref()).unwrap_or(st.vH);
    let v_d = ov(p.vD.as_ref()).unwrap_or(st.vD);
    let dir_h = p
        .dirH
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(&st.dirH)
        .to_string();
    let dir_d = p
        .dirD
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(&st.dirD)
        .to_string();

    let d_th = nn(p.dTh.as_ref(), 1.0);
    let t_td = nn(p.tTd.as_ref(), 20.0);
    let labor = p
        .labor
        .clone()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "vua".into());
    let t_th = t_nh + d_th;

    let mut warn = Vec::new();
    if t_th > 34.0 {
        warn.push(format!(
            "Nhiệt độ trong nhà mùa hè t_T = {} °C vượt 34 °C (giáo trình Bảng 2.1: t_T không quá 34 °C; vùng có t_N > 33 °C được lấy cao hơn nhưng phải tăng vận tốc gió trong vùng làm việc – mỗi 1 °C tăng 0,5 ÷ 0,8 m/s, không quá 2,5 m/s). Kiểm tra lại ô chênh nhiệt mùa hè.",
            fixed(t_th, 1)
        ));
    }
    if t_nd >= t_td {
        warn.push(format!(
            "Địa điểm không có mùa lạnh (t_N mùa đông = {} °C ≥ t_T mùa đông = {} °C) – không tính tổn thất nhiệt mùa đông.",
            fixed(t_nd, 1),
            fixed(t_td, 1)
        ));
    }

    ClimateOut {
        st: if d.stations.contains_key(&name) {
            name
        } else {
            "Hà Nội".into()
        },
        reg: st.region.clone(),
        lat: st.lat,
        lon: st.lon,
        z: st.z,
        mH: st.mH,
        mD: st.mD,
        tab: Tab {
            tNh: st.tNh,
            phiH: st.phiH,
            tNtb: st.tNtb,
            tNd: st.tNd,
            phiD: st.phiD,
            vH: st.vH,
            dirH: st.dirH.clone(),
            vD: st.vD,
            dirD: st.dirD.clone(),
        },
        wH: WindObs {
            st: st.wind.h.st.clone(),
            d: st.wind.h.d,
            dir: dir_h.clone(),
            P: st.wind.h.P,
            v: v_h,
            calm: st.wind.h.calm,
        },
        wD: WindObs {
            st: st.wind.d.st.clone(),
            d: st.wind.d.d,
            dir: dir_d.clone(),
            P: st.wind.d.P,
            v: v_d,
            calm: st.wind.d.calm,
        },
        noTavg: false,
        tNh: t_nh,
        phiH: phi_h,
        tNtb: t_ntb,
        tNd: t_nd,
        phiD: phi_d,
        src,
        dH: moisture(t_nh, phi_h),
        dD: moisture(t_nd, phi_d),
        dTh: d_th,
        tTh: t_th,
        tTd: t_td,
        vH: v_h,
        vD: v_d,
        dirH: dir_h,
        dirD: dir_d,
        labor,
        warn,
    }
}
