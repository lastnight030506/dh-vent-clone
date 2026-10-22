//! Embedded reference data: climate normals, solar tables and the ψ grid.
//! Data is parsed once on first use; edit the JSON files in `engine/data/` to update.

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct SolStation {
    pub lat: f64,
    pub lon: f64,
    #[serde(default)]
    pub d: HashMap<String, Vec<Vec<f64>>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Station {
    pub region: String,
    pub lat: f64,
    pub lon: f64,
    pub z: f64,
    pub tNh: f64,
    pub phiH: f64,
    pub tNtb: f64,
    pub tNd: f64,
    pub phiD: f64,
    pub vH: f64,
    pub dirH: String,
    pub vD: f64,
    pub dirD: String,
    pub wind: WindPair,
    pub mH: i64,
    pub mD: i64,
}

#[derive(Debug, Deserialize)]
pub struct WindPair {
    pub h: WindObs,
    pub d: WindObs,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
pub struct WindObs {
    pub st: String,
    pub d: f64,
    pub dir: String,
    pub P: f64,
    pub v: f64,
    pub calm: f64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
pub struct SolarMonth {
    pub qday: f64,
    pub qmax: f64,
    pub N: f64,
}

pub struct Data {
    pub dirs: Vec<String>,
    pub dirname: HashMap<String, String>,
    pub hours: Vec<i64>,
    pub sol: Vec<(String, SolStation)>,
    pub stations: HashMap<String, Station>,
    pub solar_pos: Vec<(String, f64, f64)>,
    pub solar: HashMap<String, HashMap<String, SolarMonth>>,
    pub psi: HashMap<String, HashMap<String, f64>>,
}

#[derive(Deserialize)]
struct CatalogJson {
    #[serde(rename = "DIRS")]
    dirs: Vec<String>,
    #[serde(rename = "DIRNAME")]
    dirname: HashMap<String, String>,
    #[serde(rename = "HOURS")]
    hours: Vec<i64>,
    sol: serde_json::Map<String, Value>,
}

#[derive(Deserialize)]
struct ClimateJson {
    stations: HashMap<String, Station>,
    solar: HashMap<String, HashMap<String, SolarMonth>>,
    #[serde(rename = "solarPos")]
    solar_pos: serde_json::Map<String, Value>,
}

#[derive(Deserialize)]
struct PosJson {
    lat: f64,
    lon: f64,
}

pub fn data() -> &'static Data {
    static DATA: OnceLock<Data> = OnceLock::new();
    DATA.get_or_init(|| {
        let catalog: CatalogJson =
            serde_json::from_str(include_str!("../data/catalog.json")).expect("catalog.json");
        let climate: ClimateJson =
            serde_json::from_str(include_str!("../data/climate.json")).expect("climate.json");
        let psi: HashMap<String, HashMap<String, f64>> =
            serde_json::from_str(include_str!("../data/psi-table.json")).expect("psi-table.json");

        let sol = catalog
            .sol
            .into_iter()
            .map(|(name, value)| {
                let st: SolStation = serde_json::from_value(value).expect("solar station");
                (name, st)
            })
            .collect();

        let solar_pos = climate
            .solar_pos
            .into_iter()
            .map(|(name, value)| {
                let pos: PosJson = serde_json::from_value(value).expect("solar position");
                (name, pos.lat, pos.lon)
            })
            .collect();

        Data {
            dirs: catalog.dirs,
            dirname: catalog.dirname,
            hours: catalog.hours,
            sol,
            stations: climate.stations,
            solar_pos,
            solar: climate.solar,
            psi,
        }
    })
}
