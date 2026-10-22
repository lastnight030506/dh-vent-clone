//! Small numeric helpers mirroring the JavaScript reference engine.

use serde_json::Value;
use std::collections::HashMap;

pub const MONTHS: [&str; 12] = [
    "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII",
];

/// JS `num`: empty/non-finite values become `None`.
pub fn num(v: Option<&Value>) -> Option<f64> {
    let x = match v? {
        Value::Number(n) => n.as_f64()?,
        Value::String(s) => {
            let s = s.trim();
            if s.is_empty() {
                return None;
            }
            s.parse::<f64>().ok()?
        }
        Value::Bool(b) => {
            if *b {
                1.0
            } else {
                0.0
            }
        }
        _ => return None,
    };
    x.is_finite().then_some(x)
}

/// JS `nn`: numeric value or default.
pub fn nn(v: Option<&Value>, default: f64) -> f64 {
    num(v).unwrap_or(default)
}

/// JS `Number(x).toFixed(digits)` for the values used in warnings.
pub fn fixed(x: f64, digits: usize) -> String {
    format!("{x:.digits$}")
}

/// JS number-to-string for the formula captions (`0.95`, `50`, `34.4`).
pub fn jsnum(x: f64) -> String {
    if x == 0.0 {
        "0".into()
    } else if x.fract() == 0.0 && x.abs() < 1e21 {
        format!("{}", x as i64)
    } else {
        format!("{x}")
    }
}

pub fn round(x: f64, digits: i32) -> f64 {
    let f = 10f64.powi(digits);
    (x * f).round() / f
}

pub fn pct_of(dir: &str) -> f64 {
    match dir {
        "B" | "DB" | "D" | "TB" => 10.0,
        "DN" | "T" => 5.0,
        _ => 0.0,
    }
}

fn angle(dir: &str, dirs: &[String]) -> f64 {
    dirs.iter()
        .position(|d| d == dir)
        .map(|i| i as f64 * 45.0)
        .unwrap_or(-45.0)
}

pub fn windward(face: &str, wind: &str, dirs: &[String]) -> f64 {
    if !dirs.iter().any(|d| d == face) || !dirs.iter().any(|d| d == wind) {
        return 0.0;
    }
    let df = (((angle(face, dirs) - angle(wind, dirs)) % 360.0 + 540.0) % 360.0 - 180.0).abs();
    if df < 1.0 {
        1.0
    } else if (df - 45.0).abs() < 1.0 {
        0.65
    } else {
        0.0
    }
}

pub fn dist_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371.0;
    let r = std::f64::consts::PI / 180.0;
    let dlat = (lat2 - lat1) * r;
    let dlon = (lon2 - lon1) * r;
    let h = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * R * h.sqrt().asin()
}

pub fn nearest_sol<'a>(list: &'a [(String, crate::data::SolStation)], lat: f64, lon: f64) -> (&'a str, f64) {
    let mut best: Option<(&str, f64)> = None;
    for (name, s) in list {
        let d = dist_km(lat, lon, s.lat, s.lon);
        if best.map_or(true, |(_, bd)| d < bd) {
            best = Some((name, d));
        }
    }
    best.expect("solar station list is not empty")
}

pub fn nearest_pos(list: &[(String, f64, f64)], lat: f64, lon: f64) -> (String, f64) {
    let mut best: Option<(String, f64)> = None;
    for (name, la, lo) in list {
        let d = dist_km(lat, lon, *la, *lo);
        if best.as_ref().map_or(true, |(_, bd)| d < *bd) {
            best = Some((name.clone(), d));
        }
    }
    best.expect("radiation station list is not empty")
}

/// Magnus saturation vapour pressure, Pa.
pub fn pbh(t: f64) -> f64 {
    610.94 * (17.625 * t / (t + 243.04)).exp()
}

/// Moisture content g/kg.
pub fn moisture(t: f64, phi: f64) -> f64 {
    let p = (phi / 100.0) * pbh(t);
    622.0 * p / (101325.0 - p)
}

/// Bilinear interpolation of the Bảng 2.10 ψ grid.
pub fn psi_at(ratio: f64, z: f64, psi: &HashMap<String, HashMap<String, f64>>) -> f64 {
    let knots = [1.0, 2.0, 3.0, 5.0];
    let r = ratio.clamp(knots[0], knots[knots.len() - 1]);
    let mut i = 0;
    while i < knots.len() - 2 && r > knots[i + 1] {
        i += 1;
    }
    let (r0, r1) = (knots[i], knots[i + 1]);
    let zz = z.clamp(1.0, 10.0);
    let z0 = zz.floor() as i32;
    let z1 = (z0 + 1).min(10);
    let fz = zz - z0 as f64;
    let v = |k: i32, z: i32| -> f64 {
        psi.get(&k.to_string())
            .and_then(|row| row.get(&z.to_string()))
            .copied()
            .unwrap_or(0.0)
    };
    let a = v(r0 as i32, z0) + (v(r1 as i32, z0) - v(r0 as i32, z0)) * (r - r0) / (r1 - r0);
    let b = v(r0 as i32, z1) + (v(r1 as i32, z1) - v(r0 as i32, z1)) * (r - r0) / (r1 - r0);
    a + (b - a) * fz
}

