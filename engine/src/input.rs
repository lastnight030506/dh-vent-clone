//! Input document (`S`) as sent by the web UI.
//!
//! Scalar cells are kept as JSON values because the UI uses `""` for empty cells.
//! Unknown fields are ignored so new UI fields do not break older engines.

use serde::Deserialize;
use serde_json::Value;

pub type J = Value;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct Project {
    pub st: Option<String>,
    pub tNh: Option<J>,
    pub phiH: Option<J>,
    pub tNtb: Option<J>,
    pub tNd: Option<J>,
    pub phiD: Option<J>,
    pub dTh: Option<J>,
    pub tTd: Option<J>,
    pub labor: Option<String>,
    pub vH: Option<J>,
    pub dirH: Option<String>,
    pub vD: Option<J>,
    pub dirD: Option<String>,
    pub bA: Option<J>,
    pub bB: Option<J>,
    pub bH: Option<J>,
    pub floorOn: Option<bool>,
    pub floorIns: Vec<FloorLayer>,
    pub roofSummer: Option<bool>,
    pub cons: Vec<Con>,
    pub els: Vec<El>,
    pub infL: InfL,
    pub light: Option<Light>,
    pub phiM: Option<J>,
    pub motors: Vec<Motor>,
    pub nPeople: Option<J>,
    pub others: Vec<Other>,
    pub sol: Option<Sol>,
    pub vent: Option<Vent>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct Con {
    pub id: String,
    pub name: Option<String>,
    pub aT: Option<J>,
    pub aN: Option<J>,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct Layer {
    pub n: Option<String>,
    pub d: Option<J>,
    pub lam: Option<J>,
    pub s: Option<J>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct FloorLayer {
    pub d: Option<J>,
    pub lam: Option<J>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct El {
    pub k: Option<String>,
    pub dir: Option<String>,
    pub a: Option<J>,
    pub b: Option<J>,
    pub n: Option<J>,
    pub con: Option<String>,
    pub psi: Option<J>,
    pub inf: Option<J>,
    pub cr: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct InfL {
    pub h: std::collections::HashMap<String, J>,
    pub d: std::collections::HashMap<String, J>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct Light {
    pub mode: Option<String>,
    pub a: Option<J>,
    pub F: Option<J>,
    pub e1: Option<J>,
    pub e2: Option<J>,
    pub kw: Option<J>,
}

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct Motor {
    pub name: Option<String>,
    pub n: Option<J>,
    pub kW: Option<J>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct Other {
    pub name: Option<String>,
    pub t: Option<String>,
    pub Q: Option<J>,
    pub G: Option<J>,
    pub Qp: Option<J>,
    pub eta: Option<J>,
    pub C: Option<J>,
    pub td: Option<J>,
    pub tc: Option<J>,
    pub beta: Option<J>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct Sol {
    pub st: Option<String>,
    pub m: Option<J>,
    pub slot: Option<J>,
    pub t1: Option<J>,
    pub t2: Option<J>,
    pub t3: Option<J>,
    pub t4: Option<J>,
    pub q: std::collections::HashMap<String, J>,
    pub rho: Option<J>,
    pub z: Option<J>,
    pub nuMode: Option<String>,
    pub nu: Option<J>,
    pub qday: Option<J>,
    pub qmax: Option<J>,
    pub psi: Option<J>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct Vent {
    pub a: Option<J>,
    pub h0: Option<J>,
    pub hv: Option<J>,
    pub tV: Option<J>,
    pub tvlv: Option<J>,
    pub nOut: Option<J>,
}
