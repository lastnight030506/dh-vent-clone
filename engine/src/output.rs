//! Output document consumed by the web UI. Field names match the JavaScript contract.

use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::Value as J;

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Output {
    pub C: ClimateOut,
    pub E: Envelope,
    pub LH: Loss,
    pub LD: Loss,
    pub GH: Gain,
    pub GD: Gain,
    pub SO: Solar,
    pub V: VentOut,
    pub H: NoPlanH,
    pub FN: J,
    pub P: NoPlanP,
    pub CK: BTreeMap<String, J>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct ClimateOut {
    pub st: String,
    pub reg: String,
    pub lat: f64,
    pub lon: f64,
    pub z: f64,
    pub mH: i64,
    pub mD: i64,
    pub tab: Tab,
    pub wH: WindObs,
    pub wD: WindObs,
    pub noTavg: bool,
    pub tNh: f64,
    pub phiH: f64,
    pub tNtb: f64,
    pub tNd: f64,
    pub phiD: f64,
    pub src: Src,
    pub dH: f64,
    pub dD: f64,
    pub dTh: f64,
    pub tTh: f64,
    pub tTd: f64,
    pub vH: f64,
    pub vD: f64,
    pub dirH: String,
    pub dirD: String,
    pub labor: String,
    pub warn: Vec<String>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Tab {
    pub tNh: f64,
    pub phiH: f64,
    pub tNtb: f64,
    pub tNd: f64,
    pub phiD: f64,
    pub vH: f64,
    pub dirH: String,
    pub vD: f64,
    pub dirD: String,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct WindObs {
    pub st: String,
    pub d: f64,
    pub dir: String,
    pub P: f64,
    pub v: f64,
    pub calm: f64,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Src {
    pub tNh: bool,
    pub phiH: bool,
    pub tNtb: bool,
    pub tNd: bool,
    pub phiD: bool,
    pub vH: bool,
    pub vD: bool,
    pub dirH: bool,
    pub dirD: bool,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Envelope {
    pub cons: BTreeMap<String, Cons>,
    pub els: Vec<El>,
    pub fl: Fl,
    pub err: Vec<String>,
    pub warn: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct Cons {
    pub id: String,
    pub name: String,
    pub aT: f64,
    pub aN: f64,
    pub layers: Vec<Layer>,
    pub Rl: f64,
    pub Ro: f64,
    pub K: f64,
    pub D: f64,
}

#[derive(Debug, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct Layer {
    pub n: String,
    pub d: J,
    pub lam: J,
    pub s: f64,
    pub dm: f64,
    pub R: f64,
    pub D: f64,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct El {
    pub i: usize,
    pub k: String,
    pub dir: String,
    pub a: J,
    pub b: J,
    pub n: J,
    pub Fg: f64,
    pub con: Option<Cons>,
    pub psi: f64,
    pub inf: f64,
    pub cr: String,
    pub name: String,
    pub F: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cut: Option<f64>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Fl {
    pub on: bool,
    pub bands: Vec<Band>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Rins: Option<f64>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Band {
    pub i: usize,
    pub F: f64,
    pub R: f64,
    pub K: f64,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Loss {
    pub tT: f64,
    pub tN: f64,
    pub dt: f64,
    pub rows: Vec<LossRow>,
    pub Qk: f64,
    pub Qb: f64,
    pub infRows: Vec<InfRow>,
    pub Qinf: f64,
    pub v: f64,
    pub wd: String,
    pub tot: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<bool>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct LossRow {
    pub k: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
    pub K: f64,
    pub F: f64,
    pub dt: f64,
    pub Q: f64,
    pub pct: Option<f64>,
    pub Qb: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band: Option<usize>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct InfRow {
    pub k: String,
    pub l: f64,
    pub G: f64,
    pub a: f64,
    pub g: f64,
    pub parts: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<bool>,
    pub Q: f64,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Gain {
    pub tT: f64,
    pub Fs: f64,
    pub Nts: f64,
    pub e1: f64,
    pub e2: f64,
    pub Qts: f64,
    pub mot: Vec<Mot>,
    pub Nm: f64,
    pub phi: f64,
    pub Qdc: f64,
    pub nP: f64,
    pub qn: f64,
    pub Qng: f64,
    pub oth: Vec<Oth>,
    pub Qoth: f64,
    pub tot: f64,
}

#[derive(Debug, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct Mot {
    pub name: String,
    pub n: J,
    pub kW: f64,
    pub N: f64,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Oth {
    pub name: String,
    pub t: String,
    pub Q: f64,
    pub f: String,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Solar {
    pub stn: String,
    pub stD: f64,
    pub auto: bool,
    pub s18: String,
    pub s18D: f64,
    pub m: i64,
    pub mon: String,
    pub slot: i64,
    pub hour: i64,
    pub tau: [f64; 4],
    pub tp: f64,
    pub win: Vec<Win>,
    pub Qwin: f64,
    pub Fm: f64,
    pub Km: f64,
    pub rho: f64,
    pub aN: f64,
    pub aT: f64,
    pub qday: f64,
    pub qdayT: f64,
    pub qtb: f64,
    pub qmax: f64,
    pub qmaxT: f64,
    pub N: f64,
    pub warn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tNtb: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub QdT: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Aq: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Attd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub AtN: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Attg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nuD: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nuR: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nuPhi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nuCalc: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nu: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub AtT: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub QA: Option<f64>,
    pub Qroof: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noRoof: Option<bool>,
    pub tot: f64,
    pub psiPH: Option<f64>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Win {
    pub dir: String,
    pub Fcs: f64,
    pub Fcm: f64,
    pub F: f64,
    pub qt: f64,
    pub q: f64,
    pub manual: bool,
    pub Q: f64,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct VentOut {
    pub QthH: f64,
    pub QthD: f64,
    pub Qth: f64,
    pub useH: bool,
    pub tvlv: f64,
    pub a: f64,
    pub h0: f64,
    pub hv: f64,
    pub tR: f64,
    pub tV: f64,
    pub Cp: f64,
    pub warn: Vec<String>,
    pub G: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rho: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub L: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub V: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nOut: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Lmo: Option<J>,
    pub nOutPlan: Option<f64>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct NoPlanH {
    pub tAir: Option<f64>,
    pub rho: Option<f64>,
    pub beta: Option<f64>,
    pub K: Option<f64>,
    pub qOut: Option<f64>,
    pub nOutTot: Option<f64>,
    pub segs: Vec<J>,
    pub err: Vec<String>,
    pub warn: Vec<String>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct NoPlanP {
    pub err: Vec<String>,
    pub warn: Vec<String>,
    pub segs: Vec<J>,
    pub suct: Option<f64>,
    pub label: BTreeMap<String, J>,
    pub geo: Vec<J>,
    pub nOut: i64,
    pub nodeZ: BTreeMap<String, J>,
    pub bends: Bends,
    pub empty: bool,
}

#[derive(Debug, Serialize)]
pub struct Bends {
    pub h: i64,
    pub v: i64,
}
