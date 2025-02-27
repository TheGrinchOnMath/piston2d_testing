use std::fs;
use serde::{Deserialize, Serialize};

#[derive (Serialize, Deserialize)]
pub struct JSON {
    mirrors: Vec<Mirror>,
}

#[derive (Serialize, Deserialize)]
struct Mirror {
    start_pos: Vec<f64>,
    end_pos: Vec<f64>,
    absorption_factor: f64,
}

pub(crate) fn read_json(path: &str) -> JSON {
    let data = fs::read_to_string (path).expect("error reading json");
    serde_json::from_str::<JSON>(data.as_str()).expect("error parsing json")
}