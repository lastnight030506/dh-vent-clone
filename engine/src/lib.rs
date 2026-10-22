//! DH-VENT calculation engine.
//!
//! The public entry point is [`compute_value`], which takes the project JSON sent by the
//! web UI and returns the result document. The modules mirror the calculation stages:
//! climate → envelope → losses → gains → solar → ventilation.

mod climate;
mod data;
mod envelope;
mod gains;
mod input;
mod losses;
mod output;
mod solar;
mod util;
mod vent;

use std::collections::BTreeMap;

use serde_json::Value;

pub use input::Project;
pub use output::Output;

use crate::output::{Bends, NoPlanH, NoPlanP};

pub fn compute(p: &Project) -> Output {
    let d = data::data();
    let c = climate::climate(p, d);
    let cons = envelope::cons_list(p);
    let e = envelope::envelope(p, &cons);
    let lh = losses::loss_season(p, &c, &e, 'h', d);
    let ld = losses::loss_season(p, &c, &e, 'd', d);
    let g = gains::gains(p, &c);
    let so = solar::solar(p, &c, &e, d);
    let v = vent::ventilation(p, &c, &g.h, &g.d, &lh, &ld, &so);

    Output {
        C: c,
        E: e,
        LH: lh,
        LD: ld,
        GH: g.h,
        GD: g.d,
        SO: so,
        V: v,
        H: NoPlanH {
            tAir: None,
            rho: None,
            beta: None,
            K: None,
            qOut: None,
            nOutTot: None,
            segs: Vec::new(),
            err: vec!["Phương án TG: Chưa vạch tuyến ống".into()],
            warn: Vec::new(),
        },
        FN: Value::Null,
        P: NoPlanP {
            err: vec!["Chưa vạch tuyến ống".into()],
            warn: Vec::new(),
            segs: Vec::new(),
            suct: None,
            label: BTreeMap::new(),
            geo: Vec::new(),
            nOut: 0,
            nodeZ: BTreeMap::new(),
            bends: Bends { h: 0, v: 0 },
            empty: true,
        },
        CK: BTreeMap::new(),
    }
}

/// Compute from raw JSON. Used by the desktop shell and the parity tests.
pub fn compute_value(input: Value) -> Result<Value, String> {
    let p: Project = if input.is_null() {
        Project::default()
    } else {
        serde_json::from_value(input).map_err(|e| format!("invalid input: {e}"))?
    };
    serde_json::to_value(compute(&p)).map_err(|e| format!("serialize error: {e}"))
}
