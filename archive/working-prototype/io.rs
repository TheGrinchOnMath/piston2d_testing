use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct JSON {
    pub(crate) coord_format: String,
    pub(crate) mirrors: Vec<Mirror>,
}

#[derive(Serialize, Deserialize)]
pub struct Mirror {
    pub(crate) start_pos: Vec<f64>,
    pub(crate) end_pos: Vec<f64>,
    pub(crate) absorption_factor: f64,
}

pub(crate) fn read_json(path: &str) -> JSON {
    let data = fs::read_to_string(path).expect("error reading json");
    serde_json::from_str::<JSON>(data.as_str()).expect("error parsing json")
}